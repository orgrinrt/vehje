//! Branch tier: interp_dispatch. A tiny bytecode interpreter that dispatches
//! the branch program per input byte via a switch, modelling the interpreter
//! dispatch overhead copy-and-patch removes.
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

// opcodes for the per-byte branch program
const LOADB: u8 = 0; const ANDI1: u8 = 1; const BRZ: u8 = 2; const ADDB: u8 = 3;
const MUL3: u8 = 4; const JMP: u8 = 5; const XORSHL: u8 = 6; const HALT: u8 = 7;
// program: LOADB; ANDI1; BRZ taken(->4); [else] XORSHL; JMP end; [taken] ADDB; MUL3; HALT
// encode as (op, arg) pairs. taken block starts at index 5, end at 8.
static PROG: &[(u8, u8)] = &[
    (LOADB,0),(ANDI1,0),(BRZ,5),(XORSHL,0),(JMP,8),(ADDB,0),(MUL3,0),(HALT,0),(HALT,0)
];
#[bench_variant("bt_interp_dispatch", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! { run {
        let mut acc: u64 = 0;
        for &b in input.iter() {
            // interpret PROG for this byte: reg r, cond c
            let mut pc: usize = 0; let mut r: u64 = 0; let mut c: u64 = 0;
            loop {
                let (op, arg) = PROG[pc];
                match op {
                    LOADB => { r = b as u64; pc += 1; }
                    ANDI1 => { c = r & 1; pc += 1; }
                    BRZ => { if c == 0 { pc = arg as usize; } else { pc += 1; } }
                    ADDB => { r = acc.wrapping_add(b as u64); pc += 1; }
                    MUL3 => { acc = r.wrapping_mul(3); pc += 1; }
                    XORSHL => { acc ^= (b as u64) << 1; pc += 1; }
                    JMP => { pc = arg as usize; }
                    _ => break, // HALT
                }
            }
        }
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}
