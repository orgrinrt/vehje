# The built abi-bench machinery through the cost-and-fairness lens (Agner Fog)

**Date:** 2026-07-23
**What this is:** a pre-run review of the built runtime-C-ABI batched-execute machinery, read from source (not
from the fairness doc's word), with the shared runtime cdylib disassembled (`otool -tV`, aarch64) and every
cross-validation test run (`cargo test --features boundary,vertical --release`, 53+1 pass, Zig byte-exact
included). No timing bench was run. Provenance marks: **[M]** read directly from source/disasm I cite;
**[I]** inferred from aarch64 microarchitecture, to be confirmed by PMU before a number is trusted.

## Verdict in one line

The isolation backbone is sound and the correctness is proven, so the machinery is fit to run, but three of its
numbers cannot yet be trusted as stated: C_cross is measured only in the perfectly-predicted warm regime (the
boundary-cold regime the cost model needs is unbuilt), the residency bench measures an allocator round-trip not a
cold region, and the bench-0 disasm evidence misattributes the SoA NEON to the wrong symbol.

## What is sound (specific, cite file:line)

- **The two-object separation is genuine, which is the whole ballgame.** `carrier-runtime` is a standalone
  `[workspace]` cdylib built `lto=fat`, `opt-level=3`, `codegen-units=1` (`carrier-runtime/Cargo.toml`), dlopen'd
  at run time via `libloading` and called through a pointer resolved from `dlsym` (`bench-matrix/src/boundary.rs`
  `Runtime::open`/`resolve`). The variant's optimizer cannot see into it, so "W calls" and "one call over W
  records" cannot collapse into indistinguishable code. Confounds 1 and 2 are handled by construction, not by
  discipline. This is the strongest part of the design.
- **The program crosses as opaque wire bytes.** `cr_init(bytes, len)` parses REC24 at init and returns an opaque
  `*mut Handle` (`carrier-runtime/src/lib.rs:65-78`); the runtime cannot const-fold its interpret loop over a
  program it only sees as runtime data. Confound 2, confirmed in source.
- **Resolve is in setup, never in the cell.** `open_and_init` runs in the family `setup`, stores `entry`/`free`
  in `St` (`boundary/common.rs:141-148`); the timed cell only reads the stored pointer and calls it
  (`cross_column`, `common.rs:164-174`). No `dlsym` in the timed region.
- **One unchecked payload, unmodified.** Every entry wraps the carrier's own `interpret_predecoded`
  (`carrier-runtime/src/lib.rs:104`) and `interpret_vertical_checksum_into` (`:125`); there is no divergent second
  interpreter. A boundary number is the crossing plus the exact payload the in-process carrier benches measure.
- **Correctness is proven before any timing.** All six built families' byte-exact cross-validation tests pass,
  including `zig_crossing_matches_rust_byte_exact` **[M]** (I ran the suite). Floors are correctly exempt.
- **`cr_execute_scalar_runtime_w` is disasm-clean, confirmable NOW on the shared dylib.** Its body is a 38-line
  GPR loop with the loop bound in a register (`mov x19, x2; cmp x23, x19`, i.e. the runtime `w` argument), a `bl`
  to `interpret_predecoded` per record, and **zero NEON** **[M]** (disasm lines 1068-1106). This confirms confound
  5 (runtime-W must not see a const W) AND the scalar-auto-vectorization confound in one shot. The fairness doc
  marks the runtime-W register-bound check "pending on variant"; it is confirmable on the shared object and it
  passes, so the doc under-claims here.
- **`cr_null_entry` is a clean C_cross floor.** 8 instructions, a pure `ror #57 ^` seed fold (rotate-left-7),
  register bound, no interpret, no NEON **[M]** (disasm lines 2711-2718). `cr_execute_scalar_dispatch` is a
  382-line body with 77 branch instructions **[M]** (685-1067), matching its claim.
- **The sink is the real struct, called indirect.** `CrSink` is a `#[repr(C)]` two-function-pointer struct passed
  by pointer (`carrier-runtime/src/lib.rs:277-311`), so each reserve/commit is a genuine reverse crossing, not an
  inlined closure. Confound 8, handled by construction.
- **The measurement backbone is the right instrument.** N=256 fixed, W swept, `k = N/W`, fit `total(k) = S +
  k*C_cross`, C_cross read from the null floor. Every sweep value divides N_TOTAL so `k` is exact and no batch is
  partial (asserted in `cross.rs:208-210`). The floor ladder (rung a inlinable `inproc_direct`, rung b black-boxed
  indirect `inproc_fnptr`, rung c cross-object empty `null_entry`) is present and correctly shaped.

## Findings (numbered)

**1. Bench-0 disasm evidence misattributes the SoA NEON to the entry symbol.** The fairness doc's bench-0 table
says `cr_execute_soa_runtime_w` is "437 NEON-shaped lines." **[M]** The symbol body is 58 lines with **zero NEON**
(disasm 2014-2072); it is a thin GPR loop that `bl`s into `interpret_vertical_checksum_into` (disasm line at
0x27b0). The dylib's 574 NEON lines live in that callee, not in the named entry. Why it biases nothing in the
timing (the callee still runs) but corrupts the audit: bench-0's entire job is "confirm each entry is the shape
its label claims," and a reviewer checking `cr_execute_soa_runtime_w` against `otool` finds zero NEON and
concludes the evidence is fabricated. The load-bearing claim (SoA is real NEON, scalar is zero NEON) is true at
the dylib level; the per-symbol attribution is false. Fix: restate as "entry X is a GPR loop that `bl`s payload
symbol Y; Y is the 437-line NEON body," and count NEON in Y. File: fairness doc bench-0 table;
`carrier-runtime/src/lib.rs:118-134`.

**2. C_cross is measured only in the perfectly-predicted warm regime; the boundary-cold regime is unbuilt. This is
the most consequential gap.** Every family declares `regime: warm` (`cross.rs:98`, `soa.rs:40`, `entry.rs:144`,
`sink.rs:125`, `lifecycle.rs:67`, `residency.rs:31`, `zig.rs:74`, `matrix.rs:70`). The crossing target is a single
resolved pointer held constant in `St` across the whole calibrated loop, so the `blr` is perfectly predicted after
warmup and C_cross is the best-case predicted-crossing cost **[I]**. The many-residuals-per-frame deployment
thrashes the indirect predictor; a mispredicted cross-object crossing is several times more expensive (order 8-16+
cycles on a wide OoO aarch64 core, PMU-to-confirm) **[I]**. Because the headline "does batching pay" is the ratio
`C_cross / I_payload` (category D), and that ratio can invert between predicted and mispredicted crossings, a
warm-only conclusion can recommend the wrong ABI for the real deployment. The doc marks the boundary-cold regime
"pending run-time regime"; it is a prerequisite for a trustworthy deployment C_cross, not a follow-on. Fix: build
a boundary-cold cell shape that cycles distinct resolved entry pointers by iteration index (distinct call targets,
not distinct programs, which `cold_cycle` already does) before any C_cross is read as the deployment crossing
cost.

**3. The residency bench does not measure a cold input region; it measures an allocator round-trip on a hot
block.** `fresh_buffer` allocates `vec![0u64; N_TOTAL]` inside the cell (`residency.rs:50`). The allocator returns
the just-freed 2KB block every iteration, so after the first rep the "fresh, cold, re-pinned" region is
cache/TLB-resident **[I]**. Both `reused_buffer` and `fresh_buffer` fill all 256 elements via `fill_seeds`, so the
only measured delta is the `malloc`/`free` of a hot 2KB block, never page-fault, TLB-miss, or first-touch cost.
The registered-buffer question the bench claims to answer (does pinning a persistent input region matter) is not
answered. Fix: allocate from a rotating pool larger than the LLC, cycled by iteration index (or `madvise
DONTNEED`), so each pass touches a genuinely non-resident page, OR relabel the bench "allocator round-trip cost"
and drop the residency claim. File: `residency.rs:47-53`.

**4. Bench 8's Zig payload is a different interpreter than the Rust payload, so cross-language absolute deltas are
confounded.** Rust `cr_*` wraps `interpret_predecoded` (predecoded flat nodes, operands pre-resolved at init:
`predecode.rs:63-104`). Zig `zr_*` wraps `interpSwitch`, which re-reads op and up to three operands from the wire
bytes on every node on every call (`interp.zig:77-109`, driven by `scalarBatch` at `:277-285`). The Zig per-node
cost carries a byte re-decode the Rust predecoded form paid once. Byte-exact cross-validation passes **[M]**
because the semantics match, but the cost does not: `zig_runtime_w`'s absolute number is slower mostly for
interpreter-shape reasons, not ABI or callee-codegen. Yet the composition matrix drops `zig_runtime_w` into the
same oracle envelope as the Rust payload cells (`matrix.rs:131-136`) and the module doc calls it "the real
target-language cost" (`zig.rs:14-17`), which invites a wrong "the Zig ABI costs X more" conclusion. What IS fair:
the cross-language crossing via `zr_null` vs `cr_null` (both fold-only, comparable) and the entry-form deltas
WITHIN Zig (all `interpSwitch`, self-consistent). Fix: predecode inside the Zig handle to match, OR restrict Zig
conclusions to floor-subtracted crossing and within-Zig entry-form deltas and keep `zig_runtime_w` out of the
cross-language absolute-payload envelope. The stub-fidelity caveat names the fidelity gap but not this
cost-attribution confound.

**5. Doc/code mismatch: bench 2 (marshal) is built, and it is a gather-stride proxy, not a typed decode.** The
fairness doc's "What is not yet built" calls bench 2 "the only unbuilt timing bench." It is fully built:
`marshal.rs` is a four-cell family (`aos`/`soa_native`/`soa_transposed`/`marshal_null`) whose cross-validation
passes **[M]**. It synthesizes multi-field records by XOR-folding `f` fields into one seed
(`carrier-runtime/src/lib.rs:350-370`), so it measures the field-access memory pattern (record-major stride-`f`
read vs field-major streams) BEFORE any interpret, which is a legitimate marshalling-layout probe. But it is not
the "multi-field record model added to the carrier" the doc describes, and it does not touch gap A (structural
decode of the returned value-arena on the untrusted path), which remains genuinely unbuilt. Fix: correct the doc's
status; state that marshal measures gather stride, not typed decode; keep gap A on the ledger. File: fairness doc
status section; `marshal.rs:105-165`.

**6. The PMU instruction-slope gate is not enforced by the built machinery.** The fairness doc makes it "a
mandatory acceptance gate on every C_cross figure." As built it is a run-time analysis promise: nothing in the
cells or the `matrix_decls` binds a produced C_cross to a passing slope check, and `perf.rs` is a separate harness
capability. A C_cross can therefore be reported with no second witness that it is a real crossing cost rather than
a frequency-scaling or memory-stall artifact. Fix: wire the check into the analysis path so a C_cross whose
instruction-slope across the W-sweep does not track its timing-slope is rejected, not merely annotated.

**7. The min-reps floor across the W-sweep is unbuilt, and C_cross must be read only from the null floor.** The
synthesis lists a comparable-reps floor as required upstream; `scaffold.rs` has only `warm`/`cold_cycle`/`stream`,
and `calibrate_reps` only hits the 2048-tick wall-clock floor (`bench-core/src/lib.rs:463-483`). So rep count
varies across W by whatever clears the floor, and statistical power / first-touch contamination differ across the
sweep. Compounding this: the payload cells carry a constant `N*I_payload` intercept (256 interprets every pass,
independent of W), which for the heavy profiles dwarfs `k*C_cross`, so C_cross is near-unfittable from a payload
cell and MUST come from `null_entry` (small intercept). The fill of all 256 seeds every cell call
(`fill_seeds`, `common.rs:77-81`) is likewise a constant intercept, harmless to the slope but a further reason the
payload-cell slope is low-SNR. Fix: confirm the analysis reads C_cross exclusively from the floor cell; surface
`batch_count` per (cell, W); audit cross-W power before trusting the amortization knee.

**8. Minor: the SoA "vectorization win" also bundles an 8x reduction in intra-dylib dispatch calls, and low-W SoA
is not vectorized.** `soa_runtime_w` `bl`s the NEON payload once per 8-chunk; `scalar_runtime_w` `bl`s
`interpret_predecoded` once per record **[M]** (disasm 2014-2072 vs 1068-1106). The delta is the combined
column-form benefit (SIMD arithmetic plus 8x fewer dispatch calls), correctly labeled "column-SoA win," but a
reader expecting a pure-SIMD ratio should know dispatch amortization is part of it. Also, `soa_batch` at W<8 runs
entirely in the scalar remainder loop (`carrier-runtime/src/lib.rs:118-134`), so the low-W SoA points are not a
vectorized measurement and must not be read as one.

## The confound ledger (the fairness doc's trap checklist, adjudicated)

- **optimiser inlining across the boundary:** HANDLED by construction (separate cdylib, dlopen, pointer call).
  The cross-object `bl` into `carrier_runtime` symbols is confirmed **[M]**; the `blr` in the GENERATED host
  variant is confirmable only after variant generation (genuinely pending, not a defect).
- **program const-fold in the runtime:** HANDLED (`cr_init` parses opaque bytes, `lib.rs:65-78`).
- **memory-latency hiding per-record cost:** HANDLED (one cache-resident predecoded program over W distinct
  seeds, the vertical shape, not W pool-chased programs).
- **dlsym in the timed region:** HANDLED (resolve in setup, `common.rs:141-148`).
- **runtime-W seeing a const W:** HANDLED and CONFIRMED NOW (register loop bound, disasm 1068-1106). Doc says
  pending; it is not.
- **auto-vectorisation of the scalar cell:** HANDLED and CONFIRMED (zero NEON in `cr_execute_scalar_runtime_w`).
- **W=1 SoA not a fair scalar anchor:** HANDLED (anchor is scalar `cr_execute1`; SoA is a separate axis).
- **sink as an inlinable closure:** HANDLED (real `#[repr(C)]` struct, indirect; `lib.rs:277-311`).
- **AoS-to-SoA transpose charged to the vectorisation win:** HANDLED as an explicit timed stage
  (`marshal.rs:142-153`), but see finding 5 (marshal is a gather proxy).
- **reps-starvation at high W:** PARTIAL. N fixed keeps payload constant, but the min-reps floor is unbuilt
  (finding 7).
- **fixed-W flattering the dispatch table / indirect predictor:** NOT HANDLED. Boundary-cold regime unbuilt; all
  warm (finding 2). The largest hole.
- **Rust/Rust reported as "the ABI cost":** PARTIAL. Labeled a lower bound, but the Zig comparator is
  payload-confounded (finding 4).
- **PMU instruction-slope cross-check:** NOT ENFORCED (finding 6).
- **output-arena structural decode (gap A):** NOT BUILT. `batched_sink_decode` folds the arena
  (`sink.rs:142-153`) but that is a flat read, not the typed region decode on the untrusted path.
- **per-call env establishment (gap B):** HANDLED as the lifecycle bench (`fresh_per_column`/`fresh_per_batch`
  charge `cr_init`'s parse+predecode+malloc, `lifecycle.rs:82-114`), which is the intended re-establishment cost.
- **partial-batch/error path (gap C):** out of scope by the settled contract (fallibility a value property, no
  separate ABI channel).
- **cross-call pipelining (gap E):** NOT MEASURED, named as caveat. Note the fold `acc = rotate(acc) ^ entry(...)`
  is loop-carried on `acc`, but `entry`'s arguments do not depend on `acc`, so consecutive crossings can still
  overlap in the OoO window; the synchronous bench thus understates a pipelined deployment by an unmeasured
  amount.

## Open questions for the synthesiser (calls I cannot make alone)

- **Is the boundary-cold regime a prerequisite for clearing this panel, or may warm C_cross ship with cold as a
  fast-follow?** Tradeoff: warm-only C_cross is a defensible best-case lower bound on crossing cost and thus an
  upper bound on batching benefit, publishable with that caveat; but the "does batching pay" ratio can invert
  under mispredicted crossings, so a warm-only ruling risks recommending the wrong ABI for the multi-residual
  deployment the runtime actually targets.
- **Fix the Zig fidelity, or narrow the Zig claim?** Predecoding inside the Zig handle makes absolute
  cross-language deltas fair but diverges from the current shipped-runtime shape; narrowing to floor-subtracted
  crossing plus within-Zig entry-form deltas keeps the stub honest but forecloses an absolute cross-language
  execute number.
- **Fix residency to a genuine cold-page probe, or relabel it?** A rotating >LLC pool measures the real
  first-touch/TLB cost but is noisier and slower; the registered-buffer ABI decision may not actually hinge on it,
  in which case relabel to allocator-round-trip and move on.
- **Read C_cross exclusively from `null_entry`, or also from the payload cells as an r2 consistency check?** The
  payload-cell slope is buried under the constant interpret intercept and near-unfittable; using it only as a
  low-weight sanity signal is cheap but adds noise to the report.
