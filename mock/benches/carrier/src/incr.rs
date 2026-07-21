//! Incremental content-addressed compilation: cold rebuild vs warm reload.
//!
//! The old warm-load bench "measured" incrementality with an `if (i == 1234)`
//! branch that recompiled one hardcoded module and skipped the rest, so it
//! measured a predicted branch, not a cache. This module implements the real
//! mechanism: every module is content-hashed, a real cache is consulted, and
//! only a genuine miss triggers a recompile.
//!
//! A "module" is a small IR program (carrier wire bytes); "compiling" it is
//! interpreting it to a checksum, a stand-in for real lowering work that the
//! Zig runtime will replace without changing this bench's shape. The cold path
//! compiles all N modules from an empty cache. The warm path holds the previous
//! build's cache and reloads after a single edit: it hashes all N modules,
//! hits the cache on the N-1 unchanged ones, misses on the edited one, and
//! recompiles only it. Both paths fold the same final module set, so they
//! cross-validate byte-exact: the warm path is faster because it did less work,
//! not because it computed something different.

use crate::gen::{generate, GenParams};
use crate::ir::{encode, Decoded, REC24};
use crate::interp::interpret;

/// Generate `n` small module programs (fixed ~24-node IR each), one per module
/// index, as carrier wire bytes. Built once outside any timed region.
pub fn gen_modules(n: usize, seed: u64) -> Vec<Vec<u8>> {
    (0..n)
        .map(|i| {
            let mut p = GenParams::default_point();
            p.node_count = 24;
            p.seed = seed ^ (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
            encode(&generate(&p), &REC24)
        })
        .collect()
}

/// Fast word-at-a-time content hash of a module's wire bytes, the cache key.
/// Real content-addressed systems use a fast hash (xxhash / blake3), not a
/// byte-serial FNV whose mul-dependency chain would make hashing cost as much
/// as the compile it is meant to cheaply gate. This reads 8 bytes per step and
/// mixes with a rotate, so hashing is realistically cheaper than compiling.
#[inline]
pub fn content_hash(bytes: &[u8]) -> u64 {
    let mut h = 0x9e37_79b9_7f4a_7c15u64 ^ (bytes.len() as u64);
    let mut chunks = bytes.chunks_exact(8);
    for c in &mut chunks {
        let w = u64::from_le_bytes(c.try_into().unwrap());
        h = (h ^ w).rotate_left(27).wrapping_mul(0xff51_afd7_ed55_8ccd);
    }
    let mut tail = 0u64;
    for (i, &b) in chunks.remainder().iter().enumerate() {
        tail |= (b as u64) << (i * 8);
    }
    (h ^ tail).rotate_left(31).wrapping_mul(0xc4ce_b9fe_1a85_ec53)
}

/// How many passes a "compile" runs. A real front-end parses, resolves, checks,
/// infers, lowers, and optimizes, each a walk over the program, so compilation
/// costs many times more than hashing the source. This models that: a compile
/// is `COMPILE_PASSES` interpreter walks, which keeps compile cost well above
/// the single content-hash pass, the regime where incremental caching pays off.
pub const COMPILE_PASSES: usize = 8;

/// "Compile" a module: parse once, then run `COMPILE_PASSES` interpreter walks
/// (standing in for the front-end's passes) folded into one checksum. The point
/// is that this is several times more expensive than [`content_hash`], so
/// skipping it on a cache hit is the win the warm path measures. The Zig runtime
/// replaces the pass body later without changing the bench's shape.
#[inline]
pub fn compile_module(bytes: &[u8], scratch: &mut [u64]) -> u64 {
    let d = Decoded::parse(bytes, REC24).expect("module parses");
    let mut cs = 0u64;
    for p in 0..COMPILE_PASSES {
        cs = cs.rotate_left(9) ^ interpret(&d, 0x1234_5678 ^ p as u64, scratch);
    }
    cs
}

/// A tiny open-addressed hash cache from content hash to compiled checksum. Its
/// buffers are caller-owned so the timed region allocates nothing.
pub struct Cache<'a> {
    pub keys: &'a mut [u64],
    pub vals: &'a mut [u64],
    pub mask: usize,
}

/// Read-only lookup against pre-populated cache buffers, so a warm reload can
/// consult the previous build's cache through shared references without forming
/// a `&mut`. `mask` is `capacity - 1` (capacity a power of two).
#[inline]
pub fn lookup(keys: &[u64], vals: &[u64], mask: usize, key: u64) -> Option<u64> {
    let mut h = (key.wrapping_mul(0x9e37_79b9_7f4a_7c15) as usize) & mask;
    loop {
        let k = keys[h];
        if k == Cache::EMPTY {
            return None;
        }
        if k == key {
            return Some(vals[h]);
        }
        h = (h + 1) & mask;
    }
}

impl<'a> Cache<'a> {
    pub const EMPTY: u64 = u64::MAX;

    /// Clear all slots (untimed setup between cold runs).
    pub fn clear(&mut self) {
        for k in self.keys.iter_mut() {
            *k = Self::EMPTY;
        }
    }

    #[inline]
    pub fn get(&self, key: u64) -> Option<u64> {
        let mut h = (key.wrapping_mul(0x9e37_79b9_7f4a_7c15) as usize) & self.mask;
        loop {
            let k = self.keys[h];
            if k == Self::EMPTY {
                return None;
            }
            if k == key {
                return Some(self.vals[h]);
            }
            h = (h + 1) & self.mask;
        }
    }

    #[inline]
    pub fn insert(&mut self, key: u64, val: u64) {
        let mut h = (key.wrapping_mul(0x9e37_79b9_7f4a_7c15) as usize) & self.mask;
        loop {
            let k = self.keys[h];
            if k == Self::EMPTY || k == key {
                self.keys[h] = key;
                self.vals[h] = val;
                return;
            }
            h = (h + 1) & self.mask;
        }
    }
}

/// The result of an input-derived edit: which module changed, and its new
/// (valid, different) wire bytes. The edit regenerates module `perturb % nmod`
/// with a perturbed seed, so it stays a well-formed program (a byte-level splat
/// would corrupt operand indices and crash the interpreter) while its content
/// hash and compiled checksum both change, forcing a genuine cache miss.
pub fn edited_module(base_seed: u64, nmod: usize, perturb: u8) -> (usize, Vec<u8>) {
    let idx = (perturb as usize) % nmod.max(1);
    let mut p = GenParams::default_point();
    p.node_count = 24;
    p.seed = base_seed
        ^ (idx as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (((perturb as u64) << 32) | 0xed17_ed17);
    (idx, encode(&generate(&p), &REC24))
}

/// Pre-generate the edited module for every perturb value 0..256, so the timed
/// region indexes an owned edit without allocating.
pub fn edited_table(base_seed: u64, nmod: usize) -> Vec<(usize, Vec<u8>)> {
    (0..256u16).map(|p| edited_module(base_seed, nmod, p as u8)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cold_eq_warm_fold() {
        // Cold (compile all from empty) and warm (cache hit on N-1, recompile
        // the edited one) must fold to the identical result for the same edited
        // module set, so the harness variants cross-validate.
        let n = 256usize;
        let modules = gen_modules(n, 0xabc);
        let cap = (n * 4).next_power_of_two();
        let mut scratch = vec![0u64; 64];
        let mut ekeys = vec![Cache::EMPTY; cap];
        let mut evals = vec![0u64; cap];
        let mut ebuf = Vec::new();

        let _ = &mut ekeys; // silence unused-mut on the first clone path
        for perturb in [1u8, 42, 200] {
            let (edited, ebytes) = edited_module(0xabc, n, perturb);
            ebuf.clear();
            ebuf.extend_from_slice(&ebytes);

            // cold: compile everything (with the edit applied), no cache.
            let mut cold_fold = 0u64;
            for i in 0..n {
                let cs = if i == edited {
                    compile_module(&ebuf, &mut scratch)
                } else {
                    compile_module(&modules[i], &mut scratch)
                };
                cold_fold = cold_fold.rotate_left(5) ^ cs;
            }

            // warm: pre-populate the cache with the PREVIOUS build (all modules
            // unedited), then reload after the edit.
            for k in ekeys.iter_mut() {
                *k = Cache::EMPTY;
            }
            {
                let mut warm = Cache { keys: &mut ekeys, vals: &mut evals, mask: cap - 1 };
                for i in 0..n {
                    let cs = compile_module(&modules[i], &mut scratch);
                    warm.insert(content_hash(&modules[i]), cs);
                }
            }
            // timed-shape reload: hash all, hit N-1, miss + recompile the edit.
            let mut warm_fold = 0u64;
            for i in 0..n {
                if i == edited {
                    let key = content_hash(&ebuf);
                    let cs = match lookup(&ekeys, &evals, cap - 1, key) {
                        Some(v) => v,
                        None => compile_module(&ebuf, &mut scratch),
                    };
                    warm_fold = warm_fold.rotate_left(5) ^ cs;
                } else {
                    let key = content_hash(&modules[i]);
                    let cs = lookup(&ekeys, &evals, cap - 1, key).expect("unedited module must hit");
                    warm_fold = warm_fold.rotate_left(5) ^ cs;
                }
            }
            assert_eq!(cold_fold, warm_fold, "cold and warm disagree at perturb {perturb}");
        }
    }
}
