# Fairness audit and built machinery: the runtime C ABI batched-execute benches

**Date:** 2026-07-23
**What this is:** the pre-run record of what the arc has BUILT and the fairness evidence for it, the
step-6 artefact the expert panel reviews before any timing run (op's gate: build all, fairness-audit,
panel-review the built machinery, then run). It mirrors the interpreter arc's
`202607221230_fairness-audit-disassembly.md`. No timing numbers appear here; none have been taken.

## The built machinery

One shared runtime cdylib plus one shared Zig object expose every entry point over the unmodified carrier
interpreter bodies; N thin host bench families dlopen them and drive per-batch crossings in the timed region.

### The runtime objects

`mock/benches/carrier-runtime/` is a standalone-workspace cdylib. It exports, all wrapping the carrier's own
`interpret_predecoded` (scalar) and `interpret_vertical::<8>` (SoA) bodies with zero new payload logic:

| Symbol | Shape | Payload |
|---|---|---|
| `cr_init` / `cr_free` | handle lifecycle (parse + predecode once, reuse scratch) | n/a |
| `cr_execute1` | W=1 anchor, one record per call | scalar |
| `cr_null_entry` | empty payload, fold W seeds only | none (crossing floor) |
| `cr_execute_scalar_runtime_w` / `cr_execute_soa_runtime_w` | runtime batch width, internal loop | scalar / SoA-8 |
| `cr_execute_{scalar,soa}_w{1,2,4,8,16,32,64,128}` | per-W monomorphised (width baked into symbol) | scalar / SoA-8 |
| `cr_execute_{scalar,soa}_dispatch` | one symbol, `match W` to const bodies | scalar / SoA-8 |
| `cr_execute_sink_batched` / `cr_execute_sink_per_record` | results out via the `#[repr(C)]` reserve/commit sink | scalar |

`mock/benches/carrier-zig/interp.zig` exports the same scalar surface with a `zr_` prefix
(`zr_init`/`zr_free`/`zr_execute1`/`zr_null_entry`/`zr_execute_scalar_runtime_w`/`_w{1..128}`/`_dispatch`),
folding the identical keep-alive, so a Zig cell is a byte-exact cross-language comparator.

The ABI batch width `W` (records per call, swept 1..256, Wmax 128) is kept distinct from the SIMD lane width
(fixed 8): a `W`-record batch is interpreted in `W/8` SoA-8 passes plus a scalar remainder. This separates the
two 1/W knees (crossing amortisation over batch W; the fixed SoA-8 vectorisation win) and is why a runtime-W
entry can drive the SoA payload at all.

### The host bench families (`carrier/src/bench/boundary/`)

Seven of the nine planned individual benches are built, each a `bench_matrix!` family in the `warm` regime whose
cell crosses into a resolved function pointer. Shared machinery (the runtime-handle state, the column-crossing
loop, seed marshalling, the dylib-path resolution) lives in `common.rs`.

| Bench | Family | Axis isolated | Cells |
|---|---|---|---|
| 1 | `abi_cross_scalar` | crossing amortisation over W (C_cross) | inproc_direct (rung a) / inproc_fnptr (rung b) / ffi_batched_scalar / null_entry (rung c) |
| 3 | `abi_soa_win` | the vectorisation win across the boundary (headline) | scalar_payload / soa_payload / null_entry |
| 4 | `abi_entry_form` | the entry-form (op's vehicle) | runtime_w / scalar_anchor / dispatch_table / per_w_set / null_entry |
| 5 | `abi_sink` | the output-sink shape | null_sink / batched_sink / per_record_sink / batched_sink_decode |
| 6 | `abi_lifecycle` | session vs call-scoped runtime | held_handle / fresh_per_column / fresh_per_batch / null_entry |
| 7 | `abi_residency` | input buffer residency | reused_buffer / fresh_buffer / null_entry |
| 8 | `abi_zig_entry` | the entry-form on the real target language | zig_runtime_w / zig_anchor / zig_dispatch / zig_per_w_set / zig_null |

The measurement backbone is `pass(N, W) = (N/W) * C_cross + N * I_payload`, total record count `N = 256` held
fixed and `W` swept as the size axis, so `k = N/W` crossings per pass with payload work constant across the
sweep (which keeps calibration reps comparable across W and isolates C_cross in the k-slope). The C_cross fit is
an analysis-time step over the raw `(W, time)` points with `k` as the x-axis, not `W`.

## Cross-validation status (byte-exact, correctness before any timing)

Every real crossing cell is proven to compute byte-exactly what the in-process payload computes; floors are
exempt. All green via `cargo test` (correctness, allowed before the run gate):

- bench 1: `ffi_batched_scalar` crossing == in-process scalar, 4 profiles x 5 W.
- bench 3: `soa_payload` crossing == in-process SoA-8, 4 profiles x 5 W (gated on the `vertical` feature).
- bench 4: `scalar_anchor` == in-process; `per_w_set` == `runtime_w` at the same effective width, 3 profiles.
- bench 5: the sink arena == in-process per-record checksums, both sink shapes, 3 profiles x 4 W. Plus a
  carrier-runtime unit test that batched and per-record sinks write identical committed contents.
- bench 8: the Zig batched entry == the Rust runtime entry, 4 profiles x 5 W (both objects opened, one program).

The scalar and SoA payloads fold differently (per-record versus all-lanes, matching the carrier's own vertical
bench), so cross-payload agreement is not expected; each payload cross-validates against its own replica, and
the SoA lane-l == scalar seed-l fidelity is a carrier-runtime unit test.

## ISA-shape gate (bench 0): the entry forms are the shapes they claim

Disassembly of the release runtime cdylib (`otool -tV`, aarch64) confirms each entry is the shape its label
claims, so a spurious delta between two cells that compiled to the same code cannot be mistaken for a real one:

| Entry | Claim | Evidence |
|---|---|---|
| `cr_execute_soa_runtime_w` | real NEON vectorisation | 437 NEON-shaped lines (`v*.2d`/`ld1`/`fmla`/`.16b`) |
| `cr_execute_scalar_runtime_w` | scalar, no accidental auto-vectorisation | 0 NEON lines, a 38-line tight GPR loop |
| `cr_execute_scalar_dispatch` | one symbol, branch on W to const bodies | 77 branch-shaped lines (`cmp`/`b.`/`cbz`) over a 382-line body |
| `cr_execute_scalar_w64` | per-W, const width, no width dispatch | 0 NEON, a compact 33-line const-bound body, no `match`-W branch |

The load-bearing pair is the SoA-versus-scalar contrast: the headline vectorisation win is a real 437-instruction
NEON body against a zero-NEON scalar loop, not an optimiser artefact, and the scalar payload does not accidentally
gain SIMD nobody asked for.

Remaining ISA confirmations are run-and-gen-time (they live in the generated per-variant cdylibs, not the shared
runtime object): the `inproc_fnptr` rung-b cell must emit a real indirect `blr` (the `black_box` on the pointer
forces it; confirm on the generated variant), and the runtime-W scalar cell must keep its data-dependent register
loop bound (no const-W leak). These are checked when the variants are generated for the run.

## Trap checklist (agner confounds, chris traps, haoran traps)

| Trap | Handled by | Status |
|---|---|---|
| optimiser inlining across the boundary | separate cdylib + resolved fn pointer (genuine cross-object `blr`) | by construction |
| program const-fold in the runtime | program crosses as opaque wire bytes; runtime parses at init | by construction |
| memory-latency hiding per-record cost | one cache-resident program over W distinct seeds (the vertical shape) | by construction |
| dlsym in the timed region | symbols resolved in `setup` into the cell state, never in the cell | by construction |
| runtime-W seeing a const W | runtime-W takes `w` as a runtime argument (register bound) | disasm-confirm on variant (pending) |
| auto-vectorisation of the scalar cell | scalar entries disassembled: zero NEON | confirmed (above) |
| W=1 SoA not a fair scalar anchor | the anchor is the scalar `cr_execute1`; SoA is a distinct payload axis | by construction |
| the sink built as an inlinable closure | the real `#[repr(C)]` two-fn-ptr struct, passed by pointer, called indirect | by construction |
| the AoS-to-SoA transpose charged to the vectorisation win | (bench 2, unbuilt) transpose to be an explicit timed stage | pending bench 2 |
| reps-starvation at high W | N held fixed, so payload work per pass is constant across W | by construction |
| fixed-W flattering the dispatch table | run under a boundary-cold varying-W regime too | pending run-time regime |
| Rust/Rust cdylib reported as "the ABI cost" | the Zig cell (bench 8) is the real comparator; Rust/Rust is a same-toolchain lower bound | by construction |
| PMU instruction-slope cross-check on C_cross | mandatory acceptance gate at run time | pending run |

## What is not yet built (before the panel and the run)

- **Bench 2 (marshalling AoS/SoA-native/SoA-transposed).** The only unbuilt timing bench. It needs a multi-field
  record model added to the carrier (records are currently a scalar u64 seed to a u64 result, so AoS versus SoA is
  degenerate). This is a real carrier extension and a shape call on the F-field record.
- **The composition matrix** (`abi_boundary_w` / `abi_sink_w` / `abi_residency` curated entgrid families). It pins
  the winning layout/payload/entry from benches 1 to 4, which the run determines, so it is built after the
  individuals run (or as a full cross now).
- **The run itself.** Gated on this panel review. The variants are generated by `gen_matrix --features
  boundary,vertical`; the boundary variants need both dylibs staged and the `VEHJE_CARRIER_RUNTIME` /
  `VEHJE_CARRIER_ZIG` environment variables set at run time.

## For the panel

The question is neutral: is the built machinery sound to run on, and what if anything must change before a number
is trusted. The seven built families, their byte-exact cross-validation, and the ISA-shape evidence above are the
material. The pending items (bench 2, the run-time ISA confirmations, the PMU gate, the boundary-cold regime) are
named so the panel can judge whether any is a prerequisite for trusting the benches that ARE built, versus a
follow-on.
