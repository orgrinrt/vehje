# Fairness audit and built machinery: the runtime C ABI batched-execute benches

**Date:** 2026-07-23 (revised after the expert panel)
**What this is:** the pre-run record of what the arc BUILT, the fairness evidence for it, and the honest scope of
every number it can produce. It is the panel's input and now also carries the panel's outcome. No timing numbers
appear here; the discipline is that the machinery and its caveats are settled before any number exists.

The three-lens panel (agner-fog cost/fairness, chris-fallin entry-forms/codegen, haoran-xu breadth/honesty)
reviewed the built machinery; their full reviews sit beside this file
(`agner_fog_review-of-built-machinery.md`, `chris_fallin_review-of-built-machinery.md`,
`haoran_xu_review-of-built-machinery.md`). This document is revised to reflect what they found and what was fixed.

## The built machinery

One shared runtime cdylib plus one shared Zig object expose every entry point over the unmodified carrier
interpreter bodies; ten host bench families dlopen them and drive per-batch crossings in the timed region.

### The runtime objects

`mock/benches/carrier-runtime/` (standalone cdylib) exports, all wrapping `interpret_predecoded` (scalar) and
`interpret_vertical::<8>` (SoA): `cr_init`/`cr_free`; `cr_execute1` (W=1 anchor); `cr_null_entry` (empty-payload
crossing floor); `cr_execute_{scalar,soa}_runtime_w`; `cr_execute_{scalar,soa}_w{1..128}` (per-W monomorphised);
`cr_execute_{scalar,soa}_dispatch` (match-W); `cr_execute_sink_{batched,per_record}` (the `#[repr(C)]`
reserve/commit sink); `cr_execute_marshal_{aos,soa,null_aos}` (multi-field layout). `carrier-zig/interp.zig`
exports the scalar surface with a `zr_` prefix, folding the identical keep-alive so a Zig cell is byte-exact
against Rust.

The ABI batch width `W` (1..256, Wmax 128) is distinct from the fixed SIMD lane width 8: a `W`-record batch is
interpreted in `W/8` SoA-8 passes plus a scalar remainder, so a runtime-W entry can drive the SoA payload.

### The host bench families (`carrier/src/bench/boundary/`), all built and cross-validated

| Bench | Family | Axis | Cross-validation |
|---|---|---|---|
| 1 | `abi_cross_scalar` | crossing amortisation over W (C_cross) | ffi == in-process scalar, byte-exact |
| 2 | `abi_marshal` | AoS / SoA-native / SoA-transposed, over field count | all layouts agree + match in-process |
| 3 | `abi_soa_win` | the SoA-8 vectorisation win (headline) | SoA crossing == in-process SoA-8 |
| 4 | `abi_entry_form` | scalar-anchor / runtime-W / dispatch / per-W-set | anchor + per-W == runtime-W (in-process) |
| 5 | `abi_sink` | null / batched / per-record / +decode | arena == in-process per-record checksums |
| 6 | `abi_lifecycle` | held vs fresh-per-column vs fresh-per-batch | fresh == held handle fold |
| 7 | `abi_residency` | reused vs freshly allocated column | reused == fresh column fold |
| 8 | `abi_zig_entry` | the entry-form on Zig | Zig == Rust runtime, byte-exact |
| C | `abi_boundary_w` | composition: entry-form x payload x W + floors | (reuses the individual benches' machinery) |

Backbone: `pass(N, W) = (N/W)*C_cross + N*I_payload`, N=256 records fixed, W swept as the size axis; the C_cross
fit is an analysis-time OLS over the raw (W, time) points with `k = N/W` as x. Generation was the panel's
dominant finding (below); it is fixed and `gen_matrix` now produces all boundary variants.

## Panel outcome: what was found and what was fixed

1. **Generation was broken (all three, empirically).** `mockspace-bench-matrix`'s `generate.rs` hardcoded
   `TEMPLATE_SIZES = [64,256,1024,4096,16384]` and the variant template monomorphised `run::<N>` only over those,
   so no boundary family (W or field-count sweep) could generate a single variant. FIXED upstream: `sizes` is now a
   per-decl placeholder (mockspace dev `98031e4`, PR #293); `gen_matrix` generates all 22 families, 102 cells.
2. **The ISA-shape evidence misattributed its central claim (all three).** `cr_execute_soa_runtime_w`'s own body is
   ~59 lines, zero NEON; the NEON lives in the separately-compiled callee `interpret_vertical_checksum_into::<8>`
   (~284 lines), reached by `bl`. The SoA-is-real property HOLDS and is in fact stronger (every SoA entry form
   shares one compiled kernel), but the earlier table attributed the NEON to the wrong symbol. Corrected in the
   ISA table below.
3. **Residency measured an allocator round-trip, not cold residency (agner + haoran).** FIXED: the cell is renamed
   `fresh_alloc` and scoped honestly (allocation-plus-fill churn, the allocator recycles the freed block); genuine
   cold-page residency needs a non-recycled/evicted region and is a named follow-up, not this cell.
4. **Benches 6/7 test docstrings claimed a fold-equality the bodies never checked (haoran).** FIXED: both now carry
   real fold-equality cross-validation tests (fresh == held handle; reused == fresh column).

## The scope of the numbers (what a produced number does and does not mean)

The panel established that the machinery is fit to run, with specific numbers carrying caveats that must travel
with them. Stated before any number exists:

- **C_cross is measured warm (predicted crossing).** Every family is the `warm` regime crossing a single resolved
  pointer, so the indirect `blr` is perfectly predicted and C_cross is the best-case, lowest crossing cost. This is
  the CONSERVATIVE case for the ABI decision: if batching amortises the crossing even when the crossing is cheap
  (predicted), it amortises more when the crossing is expensive (mispredicted, the real varying-call-target case).
  So the warm result is a lower bound on the batching benefit and the direction of the "expose a batched entry"
  conclusion is robust to it; the boundary-cold regime (distinct call targets, agner) tightens the MAGNITUDE and
  can only strengthen the batch conclusion, never invert it. It is a named follow-up.
- **The high-W end of the pure-crossing floor is reps-caveated (haoran).** `null_entry` has no payload, so at large
  W its per-pass time falls below the calibration tick floor and `calibrate_reps` swings hard; the low-W end (many
  crossings, clear signal) is where C_cross is anchored and is unaffected. A minimum-reps floor that reports
  `batch_count` is a named follow-up; the amortisation curve is read from the low-W anchor.
- **Cross-language absolute deltas are payload-confounded (agner + haoran).** The Zig payload re-decodes the wire
  bytes per record while the Rust runtime uses a predecoded form, so only the floor-subtracted crossing and
  within-Zig entry-form deltas are fair; the Rust/Rust number is a same-toolchain lower bound, the Zig cell the
  real cross-language comparator, and Zig's `interpTail` shape (its module's own "load-bearing" cell) is not yet
  crossed. Named follow-ups.
- **Bench 2 is a gather-stride proxy, not a typed multi-field decode (agner + haoran).** It measures the AoS-vs-SoA
  access pattern and the explicit transpose over F XOR-combined fields, which is the marshalling-layout question; a
  typed record decode is a heavier extension not built.
- **The PMU instruction-slope gate is a run-time acceptance gate (agner), not yet enforced.** Every C_cross figure
  must have its instruction slope track its timing slope; this is applied when the run happens.

## ISA-shape gate (bench 0), corrected

Disassembly of the release runtime cdylib (`otool -tV`, aarch64), corrected per the panel's direct re-derivation:

| Entry | Claim | Evidence |
|---|---|---|
| `cr_execute_soa_runtime_w` | drives real NEON via a shared kernel | thin ~59-line body, `bl` into `interpret_vertical_checksum_into::<8>` (~284 lines, real NEON) shared by every SoA form |
| `cr_execute_scalar_runtime_w` | scalar, register loop bound, no auto-vec | 0 NEON, ~38-line tight GPR loop, data-dependent bound |
| `cr_execute_scalar_dispatch` | one symbol, branch on W to const bodies | a power-of-two/`clz` decision tree over the const arms (sharper than a naive chain; note for the cold regime) |
| `cr_execute_scalar_w64` | per-W, const width, no width dispatch | 0 NEON, compact const-bound body, no `match`-W branch |

The load-bearing property (SoA vectorises, scalar does not) holds; the correction is that the NEON is in the
shared callee, which is a stronger fairness property (all SoA entry forms execute the identical kernel).

## The follow-ups the panel named (magnitude-tighteners, not blockers)

None inverts the qualitative ABI decision; each tightens a magnitude or closes a secondary axis:

1. The boundary-cold regime (distinct call targets) for the true mispredicted-crossing C_cross.
2. The minimum-reps floor across the W sweep, reporting `batch_count`, for high-W floor precision.
3. The Zig `interpTail` cell and a predecoded-Zig payload for a fully payload-matched cross-language comparison.
4. The PMU instruction-slope gate enforced on every C_cross figure at run time.
5. A typed multi-field record decode, if the gather-stride marshalling proxy proves too coarse.

## What is trustworthy to conclude from a run

The amortisation SHAPE (does batching reduce per-record crossing cost, and where the knee is), the RELATIVE deltas
(entry-form differences, the SoA vectorisation win, the sink shapes' cost, the marshalling layouts), and the
selector-regret over `abi_boundary_w` (does one fixed ABI choice win across profiles and W, or does the optimum
move). These are the ABI decision, and they are robust to the caveats above. The absolute C_cross magnitude and the
`C_cross / I_payload` headline ratio's precision are the parts the named follow-ups tighten.
