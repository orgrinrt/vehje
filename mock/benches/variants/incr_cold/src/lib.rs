// C4a incremental compilation, COLD path: rebuild all N modules from nothing.
// One edit is applied (input-derived, from a pre-generated valid-program edit
// table) so the module set matches the warm variant's; cold compiles every
// module, warm reloads via the cache. Both fold the same result and
// cross-validate. Module set and edit table are built once per subprocess
// (OnceLock, not const-foldable); the timed region allocates nothing.
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use std::sync::OnceLock;
use vehje_bench_carrier::incr::{compile_module, edited_table, gen_modules};

const SEED: u64 = 0x30d0_0001;
static MODS: OnceLock<Vec<Vec<u8>>> = OnceLock::new();
static EDITS: OnceLock<Vec<(usize, Vec<u8>)>> = OnceLock::new();

#[bench_variant("incr_cold", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let mods = MODS.get_or_init(|| gen_modules(N, SEED));
    let nmod = mods.len();
    let edits = EDITS.get_or_init(|| edited_table(SEED, nmod));
    let mut scratch = vec![0u64; 32];
    timed! { run {
        let (edited, ebytes) = &edits[input[0] as usize];
        let mut fold = 0u64;
        for i in 0..nmod {
            let cs = if i == *edited {
                compile_module(ebytes, &mut scratch)
            } else {
                compile_module(&mods[i], &mut scratch)
            };
            fold = fold.rotate_left(5) ^ cs;
        }
        output.copy_from_slice(&fold.to_le_bytes());
    } }
}
