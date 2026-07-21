// C4a incremental compilation, WARM path: reload after one edit via a real
// content-addressed cache. The honest fix for the old `if (i == 1234)` no-op:
// the warm path hashes ALL N modules, consults a real cache, HITS on the N-1
// unchanged ones, MISSES on the single input-edited one (a valid regenerated
// program with a different content hash), and recompiles only it. The cache is
// pre-populated once per subprocess (the previous build) outside the timed
// region; the timed region does N hashes + N-1 lookups + 1 recompile and
// allocates nothing. Folds identically to incr_cold, so the two cross-validate.
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use std::sync::OnceLock;
use vehje_bench_carrier::incr::{compile_module, content_hash, edited_table, gen_modules, lookup, Cache};

const SEED: u64 = 0x30d0_0001;
static MODS: OnceLock<Vec<Vec<u8>>> = OnceLock::new();
static EDITS: OnceLock<Vec<(usize, Vec<u8>)>> = OnceLock::new();
static CACHE: OnceLock<(Vec<u64>, Vec<u64>, usize)> = OnceLock::new();

#[bench_variant("incr_warm", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let mods = MODS.get_or_init(|| gen_modules(N, SEED));
    let nmod = mods.len();
    let edits = EDITS.get_or_init(|| edited_table(SEED, nmod));
    let (keys, vals, mask) = CACHE.get_or_init(|| {
        let cap = (nmod * 4).next_power_of_two();
        let mut keys = vec![Cache::EMPTY; cap];
        let mut vals = vec![0u64; cap];
        let mut scratch = vec![0u64; 32];
        {
            let mut c = Cache { keys: &mut keys, vals: &mut vals, mask: cap - 1 };
            for m in mods.iter() {
                c.insert(content_hash(m), compile_module(m, &mut scratch));
            }
        }
        (keys, vals, cap - 1)
    });
    let mut scratch = vec![0u64; 32];
    timed! { run {
        let (edited, ebytes) = &edits[input[0] as usize];
        let mut fold = 0u64;
        for i in 0..nmod {
            let cs = if i == *edited {
                let key = content_hash(ebytes);
                lookup(keys, vals, *mask, key).unwrap_or_else(|| compile_module(ebytes, &mut scratch))
            } else {
                let key = content_hash(&mods[i]);
                lookup(keys, vals, *mask, key).unwrap_or_else(|| compile_module(&mods[i], &mut scratch))
            };
            fold = fold.rotate_left(5) ^ cs;
        }
        output.copy_from_slice(&fold.to_le_bytes());
    } }
}
