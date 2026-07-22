# Fairness audit: ISA-level disassembly of every dispatch cell

**Date:** 2026-07-22
**Status:** audit evidence, part of the composition-matrix machinery build (topic
`202607221700_topic.bench-composition-matrix-synthesis.md`, the fairness-gate mandate). Records the
disassembly confirmation that every dispatch cell's ISA-level codegen matches its label, and that no cell
gets an accidental advantage. Growing document: extended as new dispatch cells land. No benches run.

## Method

A dedicated `mock/benches/disasm-probe/` cdylib exports each interpreter as a non-inlined
`#[no_mangle] extern "C"` symbol (`di_switch`, `di_fntable`, ...), built with the exact variant profile
(`opt-level=3`, `lto="fat"`, `codegen-units=1`) so the codegen matches what a real variant would measure.
`llvm-objdump -d` disassembles the dylib; each symbol (and each internal preserve-none handler) is
classified by the instructions that distinguish the dispatch shapes on aarch64:

- `br xN` (register-indirect branch) is the jump-table tail or a guaranteed tail call.
- `blr xN` (register-indirect call) is a function-pointer-table indirect call.
- `stp x19..x28` (store-pair of a callee-saved register) is a callee-saved spill; a preserve-none handler
  must have none across its dispatch.

The probe is an audit tool, not a bench; it is never timed.

## Per-cell ISA confirmation

Measured on the built `libdisasm_probe.dylib` (M1, nightly-2026-05-28, variant profile):

| cell | label | ISA evidence | verdict |
|---|---|---|---|
| switch (`interpret`) | jump table | `di_switch`: one `br xN` (indirect jump through the table), zero `blr` | CONFIRMED |
| flat switch (`interpret_predecoded`) | jump table | `di_pre_switch`: one `br xN`, zero `blr` | CONFIRMED |
| fntable (`interpret_fntable`) | indirect call, survives LTO | `di_fntable`: one `blr xN`, zero `br` | CONFIRMED (not devirtualized) |
| bit-tree (`interpret_bittree`) | balanced comparison tree | `di_bittree`: zero `br`, zero `blr` (pure compare + conditional branch) | CONFIRMED |
| threaded wire (`interp_threaded`) | preserve-none context-threaded | all 17 handlers `h_const..h_input`: `spills=0`, one tail `br` each | CONFIRMED, no ABI fallback |
| threaded flat (`threaded_flat`) | preserve-none context-threaded | all 17 handlers: `spills=0`, one tail `br` each | CONFIRMED, no ABI fallback |
| threaded direct (`threaded_direct`) | preserve-none, handler ptr per node | all 17 handlers: `spills=0`, one tail `br` each | CONFIRMED, no ABI fallback |

The single most important confirmation: `rust_preserve_none_cc` is an incomplete nightly feature, and the
worry (topic Part 0) was that it might silently fall back to the standard ABI on some handler, in which case
the threaded cells would be paying callee-saved spills and measuring something that is not preserve-none.
The disassembly shows **zero callee-saved spills across all ~51 handlers of all three threaded families**,
every one ending in exactly one tail `br`. The mechanism is real everywhere; the threaded cells are the
shape they claim to be.

## No accidental advantage (the check-vs-no-check class)

The confound both prior audits found (threaded used unchecked raw pointers while switch/fntable used checked
slice indexing) is closed at the source by the fidelity gate: every dispatch shape reaches operands and
stores results through the one shared `access::rload` / `access::rstore` (unchecked, sound post-validation).
So the operand-access code is identical across cells and the only thing the dispatch axis varies is
dispatch. This is source-verified (one primitive, used by every interpreter) and consistent with the
disassembly (the handlers' operand loads are plain `ldr`, no bounds-check compare-and-branch in any cell).

The `stp x19..x28` counts on the `di_*` WRAPPER symbols (3 to 6 each) are the wrapper's own frame doing the
`Vec` allocation, wire parse, and post-pass checksum, not the dispatch loop, and are identical in kind
across every wrapper; they are excluded from the dispatch comparison (the wrapper is audit scaffolding, not
a measured cell).

## Cells whose fairness is not a dispatch-label question

- Stages (`optimize`, `fusion`, `liveness`): program-to-program or execution-shape transforms, not dispatch
  shapes. Their fairness is semantic preservation, verified by cross-validation on the live-out (sink) fold
  across all six profiles and many seeds, plus the shared access primitive in their interpreters.
- Vertical/SoA SIMD: a genuinely different evaluation shape (one dispatch over W inputs). Its advantage
  (dispatch amortised over W) is the real property under test, not an artefact; cross-validated per lane.
- regcache: uses the shared access primitive; its per-operand compare (cache hit test) is a real cost of
  the technique, faithfully represented.

## Not yet built / flagged

- perfect-hash dispatch: degenerate to the fn-pointer table for the dense contiguous opcode set (0..16); a
  representative perfect-hash cell needs a sparse opcode design decision (panel / op). A degenerate cell
  would misrepresent the technique, so it is not built.
- Later dispatch cells (Zig computed-goto, copy-and-patch stencils, trace dispatch) get the same ISA
  confirmation appended here as they land.

## Reproduce

```
cd mock/benches/disasm-probe && cargo build --release
llvm-objdump -d target/release/libdisasm_probe.dylib | \
  awk '/^[0-9a-f]+ <.*>:/{n=$0;s=0;t=0} /\tstp\t(x19|x20|x21|x22|x23|x24|x25|x26|x27|x28)/{s++} /\tbr\tx/{t++} /\tret/{if(t>0)printf "spills=%d %s\n",s,n}'
```
