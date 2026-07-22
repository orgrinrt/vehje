//! Sharded interner merge: does parallel sharded interning beat single-threaded,
//! given the merge tail?
//!
//! Interning maps each token's bytes to a stable dense id, deduping. It runs on
//! every identifier/literal during lex/parse, so it is a compile hot path. The
//! named lever is to shard it: each thread interns its slice of the token stream
//! into a private table, then a merge folds the shard tables into one global
//! table (resolving cross-shard duplicates and remapping local ids to global).
//! The parallel intern is S times cheaper if balanced, but the merge is a
//! sequential tail over the distinct strings each shard found. This bench
//! measures whether shard-plus-merge beats single-threaded, and how the answer
//! depends on the dedup rate (high dedup shrinks the tail; mostly-unique tokens
//! make the merge cost approach the intern cost).
//!
//! Interning is real: each token owns a byte copy in the pool, so the interner
//! hashes and byte-compares to discover duplicates (not a pointer or offset
//! compare). Single-threaded and sharded produce the same equivalence classes
//! (same bytes iff same id); the canonical first-occurrence signature is folded
//! for cross-validation.

use crate::gen::Rng;

/// A token corpus: a byte pool plus tokens as (offset, len) slices, each token
/// owning its own byte copy so interning must hash and compare bytes.
pub struct Corpus {
    pub pool: Vec<u8>,
    pub toks: Vec<(u32, u32)>,
}

/// Generate `n` tokens drawn from a `vocab`-string vocabulary, with the top
/// `common` strings taking ~80% of the draws (the parse reality: identifiers and
/// keywords repeat, literals vary). Each token appends its own byte copy.
pub fn gen_corpus(n: usize, vocab: usize, common: usize, seed: u64) -> Corpus {
    let mut rng = Rng::new(seed);
    // build the vocabulary: `vocab` distinct 3..11-byte strings.
    let mut vpool: Vec<u8> = Vec::new();
    let mut voff: Vec<(u32, u32)> = Vec::with_capacity(vocab);
    for v in 0..vocab {
        let len = 3 + (rng.next_u64() as usize % 9);
        let off = vpool.len() as u32;
        for i in 0..len {
            // encode the vocab index into the bytes so distinct entries differ.
            vpool.push((b'a' + ((v ^ (i * 31)) % 26) as u8) as u8);
        }
        vpool.push((b'0' + (v % 10) as u8) as u8); // distinctness sentinel
        voff.push((off, len as u32 + 1));
    }
    let mut pool: Vec<u8> = Vec::with_capacity(n * 8);
    let mut toks: Vec<(u32, u32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let r = rng.next_u64();
        let v = if (r & 0xFF) < 205 && common > 0 {
            (rng.next_u64() as usize) % common // ~80% common
        } else {
            (rng.next_u64() as usize) % vocab
        };
        let (vo, vl) = voff[v];
        let off = pool.len() as u32;
        pool.extend_from_slice(&vpool[vo as usize..(vo + vl) as usize]);
        toks.push((off, vl));
    }
    Corpus { pool, toks }
}

#[inline]
fn hash_bytes(b: &[u8]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &x in b {
        h ^= x as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// An open-addressed interner: hash -> id, storing each id's (offset,len) so a
/// hash collision falls back to a byte compare. Caller-owned buffers.
pub struct Interner<'a> {
    pub keys: &'a mut [u64],   // stored hash per slot, 0 = empty
    pub slot_id: &'a mut [u32], // id per slot
    pub id_off: &'a mut [u32], // (offset,len) per id, for byte compare
    pub id_len: &'a mut [u32],
    pub mask: usize,
    pub next_id: u32,
}

impl<'a> Interner<'a> {
    pub fn clear(&mut self) {
        for k in self.keys.iter_mut() {
            *k = 0;
        }
        self.next_id = 0;
    }

    /// Intern the bytes `pool[off..off+len]`; return its id, inserting if new.
    #[inline]
    pub fn intern(&mut self, pool: &[u8], off: u32, len: u32) -> u32 {
        let s = &pool[off as usize..(off + len) as usize];
        let h = hash_bytes(s) | 1; // never 0 (0 marks empty)
        let mut slot = (h as usize) & self.mask;
        loop {
            let k = self.keys[slot];
            if k == 0 {
                let id = self.next_id;
                self.keys[slot] = h;
                self.slot_id[slot] = id;
                self.id_off[id as usize] = off;
                self.id_len[id as usize] = len;
                self.next_id += 1;
                return id;
            }
            if k == h {
                let id = self.slot_id[slot];
                let o = self.id_off[id as usize];
                let l = self.id_len[id as usize];
                if l == len && pool[o as usize..(o + l) as usize] == *s {
                    return id;
                }
            }
            slot = (slot + 1) & self.mask;
        }
    }
}

/// Single-threaded: intern every token into one table. `ids[i]` = token i's id.
/// Returns the number of distinct strings.
pub fn intern_single(c: &Corpus, it: &mut Interner, ids: &mut [u32]) -> u32 {
    it.clear();
    for (i, &(off, len)) in c.toks.iter().enumerate() {
        ids[i] = it.intern(&c.pool, off, len);
    }
    it.next_id
}

/// Fold a canonical first-occurrence signature: for each token, the smallest
/// token index sharing its id. Independent of the id numbering, so single and
/// sharded agree iff their equivalence classes agree.
pub fn canonical_checksum(ids: &[u32], ndistinct: u32) -> u64 {
    let mut first = vec![u32::MAX; ndistinct as usize];
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for (i, &id) in ids.iter().enumerate() {
        if first[id as usize] == u32::MAX {
            first[id as usize] = i as u32;
        }
        h = (h ^ first[id as usize] as u64).rotate_left(7).wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// One shard's local interning result: its distinct strings (as offsets into
/// the shared pool, in local-id order) and its tokens' local ids.
pub struct ShardResult {
    pub lo: usize,
    pub hi: usize,
    pub ndistinct: u32,
    pub id_off: Vec<u32>,
    pub id_len: Vec<u32>,
    pub local_ids: Vec<u32>,
}

/// Intern one shard's token range `[lo, hi)` into a private table (the parallel
/// step). Allocates the shard's own table and result buffers, which is the real
/// cost of sharding (each thread needs a private table).
pub fn intern_shard(c: &Corpus, lo: usize, hi: usize) -> ShardResult {
    let n = hi - lo;
    let cap = (n * 2).next_power_of_two().max(16);
    let mut keys = vec![0u64; cap];
    let mut slot_id = vec![0u32; cap];
    let mut id_off = vec![0u32; n];
    let mut id_len = vec![0u32; n];
    let mut local_ids = vec![0u32; n];
    let nd = {
        let mut it = Interner {
            keys: &mut keys,
            slot_id: &mut slot_id,
            id_off: &mut id_off,
            id_len: &mut id_len,
            mask: cap - 1,
            next_id: 0,
        };
        for (k, i) in (lo..hi).enumerate() {
            let (o, l) = c.toks[i];
            local_ids[k] = it.intern(&c.pool, o, l);
        }
        it.next_id
    };
    id_off.truncate(nd as usize);
    id_len.truncate(nd as usize);
    ShardResult { lo, hi, ndistinct: nd, id_off, id_len, local_ids }
}

/// Merge the shard tables into one global interning (the sequential tail):
/// re-intern each shard's distinct strings into the global table (deduping
/// cross-shard), build the local->global remap, and rewrite every token's id.
/// Returns the global distinct count. `out_ids` is the final per-token id array.
pub fn merge_shards(
    pool: &[u8],
    shards: &[ShardResult],
    global: &mut Interner,
    out_ids: &mut [u32],
    remap_scratch: &mut Vec<u32>,
) -> u32 {
    global.clear();
    for sh in shards {
        remap_scratch.clear();
        remap_scratch.resize(sh.ndistinct as usize, 0);
        for lid in 0..sh.ndistinct as usize {
            remap_scratch[lid] = global.intern(pool, sh.id_off[lid], sh.id_len[lid]);
        }
        for (k, i) in (sh.lo..sh.hi).enumerate() {
            out_ids[i] = remap_scratch[sh.local_ids[k] as usize];
        }
    }
    global.next_id
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_interner(cap: usize, nids: usize) -> (Vec<u64>, Vec<u32>, Vec<u32>, Vec<u32>) {
        (vec![0u64; cap], vec![0u32; cap], vec![0u32; nids], vec![0u32; nids])
    }

    #[test]
    fn interning_dedups_correctly() {
        let c = gen_corpus(2000, 256, 32, 0x5eed);
        let cap = 8192;
        let (mut k, mut si, mut io, mut il) = mk_interner(cap, c.toks.len());
        let mut it = Interner { keys: &mut k, slot_id: &mut si, id_off: &mut io, id_len: &mut il, mask: cap - 1, next_id: 0 };
        let mut ids = vec![0u32; c.toks.len()];
        let nd = intern_single(&c, &mut it, &mut ids);
        // same string iff same id (checked against the raw bytes).
        for i in 0..c.toks.len().min(400) {
            for j in (i + 1)..c.toks.len().min(400) {
                let (oi, li) = c.toks[i];
                let (oj, lj) = c.toks[j];
                let bi = &c.pool[oi as usize..(oi + li) as usize];
                let bj = &c.pool[oj as usize..(oj + lj) as usize];
                assert_eq!(ids[i] == ids[j], bi == bj, "dedup wrong at ({i},{j})");
            }
        }
        assert!(nd <= 256, "distinct exceeds vocab");
        assert!(nd > 32, "distinct below the common set");
    }

    #[test]
    fn single_eq_sharded() {
        let c = gen_corpus(4000, 256, 32, 0x99);
        let cap = 8192;
        let n = c.toks.len();
        // single
        let (mut k, mut si, mut io, mut il) = mk_interner(cap, n);
        let mut it = Interner { keys: &mut k, slot_id: &mut si, id_off: &mut io, id_len: &mut il, mask: cap - 1, next_id: 0 };
        let mut ids_s = vec![0u32; n];
        let nd_s = intern_single(&c, &mut it, &mut ids_s);
        let ck_s = canonical_checksum(&ids_s, nd_s);
        // sharded (4 shards) + merge
        let s = 4;
        let shards: Vec<ShardResult> = (0..s).map(|si| intern_shard(&c, si * n / s, (si + 1) * n / s)).collect();
        let (mut gk, mut gsi, mut gio, mut gil) = mk_interner(cap, n);
        let mut git = Interner { keys: &mut gk, slot_id: &mut gsi, id_off: &mut gio, id_len: &mut gil, mask: cap - 1, next_id: 0 };
        let mut ids_m = vec![0u32; n];
        let mut remap = Vec::new();
        let nd_m = merge_shards(&c.pool, &shards, &mut git, &mut ids_m, &mut remap);
        assert_eq!(nd_s, nd_m, "distinct count differs (single {nd_s} vs sharded {nd_m})");
        assert_eq!(ck_s, canonical_checksum(&ids_m, nd_m), "canonical signature differs");
    }

}
