# The built abi-bench machinery through the entry-forms-and-codegen lens (Chris Fallin)

**Scope:** pre-run review of the runtime C ABI batched-execute bench arc's built machinery, through the lens
of entry-point form soundness: the entry cells, monomorphisation, the two-object split, whether each cell is
the codegen shape it claims, and whether the crossings are genuine. No timing numbers exist yet; none were
taken here either. Everything below is read from the actual source, the actual compiled `carrier-runtime`
release cdylib disassembled directly, and one real invocation of the generator binary, not from the audit
doc's own claims about itself.

## Verdict in one line

The callee-side entry-form codegen (the four forms wrapping the identical carrier bodies, the two-object
split, the resolved-pointer crossing) is sound and, where I checked it against the audit doc's own evidence
table, mostly holds up or is stronger than claimed; but the machinery as a whole is **not** sound to run on,
because the standard generation path (`gen_matrix`) cannot produce a single boundary-bench variant crate
today for any of the eight built families: every one of them declares a swept-size value set the codegen
template rejects outright, so the "the run itself... gated on this panel review" framing in the audit doc
describes a step that does not currently exist to gate.

## What is sound (specific, cite file:line)

- **The two-object split is real and airtight independent of anything else in this review.**
  `carrier/src/bench/boundary/common.rs:116-125` (`StCross`) stores a `Runtime` (a `libloading::Library`) plus
  a `CrEntryW` function-pointer field resolved once via `mockspace-bench-matrix/src/boundary.rs:72-77`
  (`Runtime::resolve`, `self.lib.get(name)` then deref-copy). That pointer is a runtime value obtained from
  `dlsym` against a *separate compiled artifact on disk*; the calling compilation unit has no visibility into
  what code is behind it, so `cross_column`'s call (`common.rs:170`, `entry(handle, seeds.as_ptr().add(off),
  batch)`) cannot be inlined or devirtualised by any optimisation level, in any crate that eventually hosts it.
  This holds regardless of the generation-pipeline break below; it is a property of the call shape, not of
  whether a variant crate currently exists.
- **The symbol is resolved in `setup`, never in the timed cell** (chris-fallin trap 3). Every `open_and_init`
  / `open_anchor` / `open_mono` / `open_zig_cross` helper in `common.rs:141-158`, `entry.rs:64-71,107-118`,
  `zig.rs:34-62` resolves `rt.resolve(...)` and stores the pointer in the state struct built once; the `cell`
  closures (`entry.rs:151-200`) only read `s.entry` / `s.exec1` / `s.mono`, never resolve. Correct.
- **`per_w_set`'s signature structurally forecloses a runtime W.** `CrEntryMono = unsafe extern "C" fn(*mut
  c_void, *const u64) -> u64` (`common.rs:47`) has no width parameter at all; there is no argument through
  which a caller could even attempt to pass a runtime `w`. Confirmed by disassembly: `cr_execute_scalar_w64`'s
  loop bound is `cmp x22, #0x40` (a literal), never a register compare. This is the strongest of the four
  forms' soundness claims: it cannot accidentally regress into dispatch-table shape, by construction of its C
  signature, not by convention.
- **`runtime_w`'s loop bound is a genuine register, not a folded constant.** `cr_execute_scalar_runtime_w`
  disassembles to `cmp x23, x19` where `x19` traces directly to the incoming `w: usize` argument register (`mov
  x19, x2`); confirmed zero NEON instructions in its 39-line body. Matches the claim exactly.
- **The SoA vectorised kernel is shared, byte-identical, and called identically from every SoA-payload entry
  form.** See Finding 2 below for the correction to the audit doc's per-function attribution; the underlying
  fact this was trying to show is true and, once correctly attributed, stronger than claimed: `runtime_w`,
  all eight `soa_w{K}` monomorphised symbols, and `soa_dispatch` all `bl` into the exact same compiled
  instance of `interpret_vertical_checksum_into::<8>` (one label, `carrier-runtime` disasm offset `0x8a4`),
  so there is zero risk of one entry form getting differently-vectorised code than another.
- **`scalar_batch`/`soa_batch` genuinely wrap different unmodified carrier bodies**
  (`carrier-runtime/src/lib.rs:100-134`), and the SoA runtime-W form is reachable at all only because the SIMD
  lane width is fixed at `LANES = 8` (`lib.rs:43`) and decoupled from the ABI batch width `W`, which is chunked
  internally (`while i + LANES <= w`). This resolves, for this specific implementation, the tension the fork
  docs raise about "runtime-W can only wrap the scalar interpreter": that claim is true of a naive
  `Simd<u64, W>`-parameterised runtime-W design, and false of the built one, which fixes the lane count and
  chunks the runtime batch width around it. Confirmed working end to end (`cargo test --features
  boundary,vertical`, `soa_crossing_matches_in_process_byte_exact`, green).
- **Cross-validation is real and green.** `cargo test --features boundary,vertical --release` in
  `mock/benches/carrier`: 53 unit tests plus the Zig cross-validation test, all pass, including
  `entry_forms_match_in_process_byte_exact`, `entry_forms_agree_within_a_payload`,
  `sink_writes_per_record_checksums_batched_equals_per_record`, `marshal_aos_equals_soa_for_the_same_records`,
  `soa_lane_l_equals_scalar_seed_l_fidelity`, `zig_matches_rust_byte_exact`. The byte-exactness claims in the
  audit doc are correct, verified independently.
- **The dispatch-table form genuinely dispatches at the callee side, not degenerated to a fixed body.**
  Disassembly of `cr_execute_scalar_dispatch` shows real branch structure over `w` (see Finding 4), distinct
  code from both `runtime_w` and `per_w_set`.

## Findings

### 1. The generation pipeline cannot produce a single boundary-bench variant, for any of the eight built families

**The problem.** `gen_matrix` (`mock/benches/src/bin/gen_matrix.rs`), run against the actual built decls,
fails immediately:

```
FAILED: matrix 'abi_cross_scalar' requests size 1, which the variant template does not monomorphize
(allowed: [64, 256, 1024, 4096, 16384]); add it to LIB_TEMPLATE's bench_variant sizes or drop it from the decl
```

This is not one family's bug. Every boundary `bench_matrix!` invocation declares a `sizes:` list that shares
no useful overlap with `TEMPLATE_SIZES = [64, 256, 1024, 4096, 16384]`
(`mockspace/bench-matrix/src/generate.rs:46`), the fixed list hardcoded into `LIB_TEMPLATE`
(`generate.rs:24-37`, the literal `sizes = [64, 256, 1024, 4096, 16384]` baked into
`#[bench_variant(...)]`, never substituted per decl):

| Family | file:line | declared `sizes:` | overlap with `TEMPLATE_SIZES` |
|---|---|---|---|
| `abi_cross_scalar` (bench 1) | `cross.rs:95` | `[1,2,4,8,16,32,64,128,256]` | `{64,256}` |
| `abi_marshal` (bench 2) | `marshal.rs:112` | `[1,2,4,8,16]` | `{}` (none) |
| `abi_soa_win` (bench 3) | `soa.rs:37` | `[1,2,4,8,16,32,64,128,256]` | `{64,256}` |
| `abi_entry_form` (bench 4) | `entry.rs:141` | `[1,2,4,8,16,32,64,128,256]` | `{64,256}` |
| `abi_sink` (bench 5) | `sink.rs:122` | `[1,2,4,8,16,32,64,128,256]` | `{64,256}` |
| `abi_lifecycle` (bench 6) | `lifecycle.rs:64` | `[1,2,4,8,16,32,64,128,256]` | `{64,256}` |
| `abi_residency` (bench 7) | `residency.rs:28` | `[1,2,4,8,16,32,64,128,256]` | `{64,256}` |
| `abi_zig_entry` (bench 8) | `zig.rs:71` | `[1,2,4,8,16,32,64,128,256]` | `{64,256}` |
| `abi_boundary_w` (composition) | `matrix.rs:67` | `[1,2,4,8,16,32,64,128,256]` | `{64,256}` |

`generate_all` (`generate.rs:93-106`) walks decls in this order and rejects the very first, so the actual
error names `abi_cross_scalar`, but the table above shows all nine fail the same check. Even where a size
happens to intersect (64, 256), the intersection is coincidental and does not rescue the family: the whole
point of the W-sweep is the amortisation curve from crossing-dominated (W=1) to payload-dominated (W=256),
and only two of nine declared points survive.

**Why this is not merely a validation false-positive.** Even bypassing `generate_all`'s pre-check, the
generated dispatcher itself could not serve the missing sizes. `#[bench_variant]`'s expansion
(`mockspace/bench-macro/src/lib.rs:320-335`) emits exactly one `match n { 64 => ..., 256 => ..., 1024 => ...,
4096 => ..., 16384 => ..., other => panic!(...) }` per variant, because `LIB_TEMPLATE`'s `sizes = [64, 256,
1024, 4096, 16384]` is a fixed string literal, not one of the five substituted keys (`{name}`,
`{scaffold_fn}`, `{setup_path}`, `{op_path}`, `{sweep_value}`, `generate.rs:21-23`). A boundary variant
generated today (if the size-list validation were relaxed) would compile a `bench_entry` whose only callable
`n` values are 64, 256, 1024, 4096, 16384; calling it with `n=1` (or 2, 4, 8, 16, 32, 128) would hit the
`other => panic!("unsupported n=...")` arm at run time, across the FFI boundary, inside a worker process.

**Why this is exactly the collision the design doc's own Fact 4 named, unresolved.** The synthesis doc states
as a verified structural fact: "`BenchEntryFn`'s `n` is the size slot (`bench-core/src/lib.rs:429-430`).
Consequence: batch width W and program size collide if both ride `n`/`sizes`; they must be separated," and
later claims this was handled: "**Batch width W**... dedicated to its own slot, never sharing `n` with
program size (fact 4)." That claim is false as built. `n` is not merely shared between W and "program size";
it is shared among **three** distinct semantic axes across the boundary families, none of which is the
`TEMPLATE_SIZES` axis the codegen template actually monomorphizes over:

- `abi_entry_form` / `abi_soa_win` / most others: `n` = the ABI batch width `W` (`entry.rs:147-149`,
  `open_and_init(profile, n, ...)` where the third parameter of `open_and_init` is named `w`).
- `abi_marshal`: `n` = the record's field count `f` (`marshal.rs:118-120`, "`n` (the swept size) is the field
  count `f`; the record count is fixed at `N_RECORDS`").
- The ordinary (non-boundary) carrier benches this template was originally built for: `n` = program/column
  size, which is genuinely what `TEMPLATE_SIZES` was chosen for.

**The concrete fix.** This is an upstream `mockspace-bench-matrix` change (`generate.rs`), not a vehje-side
workaround, per this workspace's own "fix the stack upstream, never reinvent in the consumer" discipline: give
`MatrixSpec`/`LIB_TEMPLATE` a per-decl size list (substitute `{sizes}` into the `#[bench_variant(...)]`
attribute instead of hardcoding it), and let `generate_all`'s validation check each decl's sizes are self-
consistent rather than against one fixed global list. Until that lands, the honest state is: **zero** of the
eight boundary bench families, plus the composition family, can be generated into a runnable variant. Nothing
downstream of this (ISA confirmation on generated variants, PMU cross-check, the run itself) can start.

### 2. The ISA-shape gate's own evidence table misattributes its central claim; the underlying fact is fine once correctly isolated

**The problem.** `fairness-audit-and-built-machinery.md`'s evidence table states: `cr_execute_soa_runtime_w` |
real NEON vectorisation | 437 NEON-shaped lines (`v*.2d`/`ld1`/`fmla`/`.16b`). I rebuilt the release
`carrier-runtime` cdylib and disassembled it directly (`otool -tV`, aarch64), isolating exactly the
`_cr_execute_soa_runtime_w` label to the next label. Its body is 59 lines, contains **zero** NEON-tagged
instructions, and consists of scalar register moves, one branch-heavy loop-selection preamble, and two `bl`
calls: one to the Rust-mangled `interpret_vertical_checksum_into::<8>` (the actual vector kernel, defined once
elsewhere in the same object at a different label, 284 lines, 104 lines carrying NEON suffixes: `ld1r.2d`,
`neg.2d`, `cmeq.2d`, `dup.2d`, `and.16b`, `cmhi.2d`, `sub.2d`, `mvn.16b`, ...), and one to
`interpret_predecoded` (the scalar remainder path).

**Why this corrupts a conclusion, if left unfixed.** It doesn't corrupt the underlying performance claim
(vectorisation genuinely happens and is real NEON), but it does mean the audit doc's own gating evidence, as
written, was not actually produced by isolating the claimed symbol; either the count was taken across the
whole `otool` output without delimiting the function, or against a different build, or is simply wrong. A
panel that accepts this row on the doc's word rather than re-deriving it is trusting a self-report that fails
its own verification when checked. The gate's entire purpose (per its own framing, "so a spurious delta
between two cells that compiled to the same code cannot be mistaken for a real one") requires the isolation to
actually be done correctly; here it evidently was not, at least in what got written down.

**The concrete fix.** Correct the evidence table's row to name where the vectorised body actually lives (a
shared, non-exported, single-instance kernel called via a genuine intra-object `bl` from every SoA entry
form), and note this as a *stronger* fairness property than the original framing implied: because `runtime_w`,
every `soa_w{K}`, and `soa_dispatch` all call the identical compiled instance, there is no entry-form-dependent
vectorisation drift possible; the only thing that can differ between SoA entry forms is how W binds to the
call, never what code executes once it does. Re-run the isolation methodology (delimit each symbol's own
disassembled range before counting) for every row in the table before the panel treats any of them as settled.

### 3. Doc-vs-source drift: the audit doc undercounts what is actually built

**The problem.** The audit doc states bench 2 (marshalling) is "the only unbuilt timing bench" and describes
the composition matrix as built "after the individuals run (or as a full cross now)," implying it is not yet
built. Direct inspection shows both exist, compile, and pass tests today: `marshal.rs` declares `abi_marshal`
with `aos` / `soa` / `marshal_null` cells and a passing cross-validation test
(`layouts_agree_and_match_in_process_byte_exact`), and `matrix.rs` declares `abi_boundary_w`, a real 9-cell
composition (`scalar_runtime_w`, `soa_runtime_w`, `scalar_dispatch`, `soa_dispatch`, `scalar_per_w`,
`soa_per_w`, `scalar_anchor`, `zig_runtime_w`, `null_entry`), with its own passing shape test.

**Why this matters, even though it errs toward caution rather than overclaiming completeness.** A doc whose
own "what's built" inventory is stale is not a document the panel can rely on without re-deriving from
source, which defeats part of the point of handing the panel a written audit rather than the raw tree. It is
also the same failure mode `cl-claim-sketch-discipline.md` names for locked changelists (a claim about source
state that does not match source reality), applied here to a research/audit doc rather than a locked CL; the
discipline still applies to keeping the audit's own inventory honest.

**The concrete fix.** Update the "built machinery" table and the "not yet built" section to reflect that all
nine families (eight individual plus the composition) exist in source with passing correctness tests, and
that the only genuinely unbuilt thing is the run itself, now further blocked by Finding 1.

### 4. The dispatch-table form lowers to a power-of-two/`clz` decision tree, not the "one predictable branch" framing implies, which sharpens (does not soften) the pending cold-regime concern

**The problem.** The audit doc characterises `cr_execute_scalar_dispatch` as "one symbol, branch on W to const
bodies" with "77 branch-shaped lines... over a 382-line body," which I confirm numerically (383 lines, 77
branch-shaped lines by my own count, matching closely). But the actual lowering is not a linear compare-chain
or a plain jump table on the numeric value of `w`; LLVM compiled the 9-arm `match` into a power-of-two test
(`sub x8, x2, #1; eor x9, x2, x8; cmp x9, x8; b.ls ...`) followed by `rbit`/`clz` to compute `log2(w)` and a
small nested-compare tree on that result. This is correct (proven by the passing cross-validation tests) and,
if anything, cheaper in the common case than a naive chain, but it means trap 4's concern ("a fixed w across
the whole run flatters the dispatch table... not the real workload's shape") is understated rather than
overstated in the current doc: this exact bit-trick path takes the identical 4-5-instruction route on every
call at a fixed `w`, which is about as branch-predictor-friendly as a warm-regime cell can get, so the delta
between the declared-pending warm-only measurement and the also-declared-pending varying-`w` regime is likely
to be larger, not smaller, than a naive-chain mental model would suggest. This does not block anything (the
varying-`w` regime is already on the pending list), but the panel should read that pending item as more
load-bearing than the current phrasing implies.

**The concrete fix.** No code change; a phrasing correction in the trap-checklist row for "fixed-W flattering
the dispatch table," and treat the boundary-cold/varying-`w` regime as a precondition for trusting any
`dispatch_table` number under the warm regime alone, not merely a nice-to-have follow-on.

### 5. Every ISA-shape confirmation marked "pending, run-and-gen-time" is now pending indefinitely, not merely until the run

The audit doc lists four items as checked only "when the variants are generated for the run": the
`inproc_fnptr` rung-b indirect `blr`, the runtime-W scalar cell's register-bound loop *on the generated
variant*, and (implicitly) every ISA claim about the actual timed call site rather than the shared runtime
object's own exported entries. Given Finding 1, there is currently no generated variant of any boundary cell
to disassemble; the ISA-shape gate (bench 0) is therefore only half-complete; not "pending the run" but
pending a prerequisite fix that has no landing date yet. The doc should say so plainly rather than folding it
into the same "pending" bucket as things that will resolve automatically once `gen_matrix` is re-run.

## The entry-form soundness ledger

| Form | Claimed shape | Built as that shape? | Disasm evidence |
|---|---|---|---|
| `scalar_anchor` (`cr_execute1`) | per-record, W-invariant, always crosses N_TOTAL times | Yes | Trivial callee: `interpret_predecoded` + `checksum`, no width parameter exists in the signature at all; `anchor_column` (`entry.rs:76-82`) loops N_TOTAL times in the HOST, one crossing per record. Cannot be confused with any other form; it has no W to bind. |
| `runtime_w` (`cr_execute_scalar_runtime_w` / `cr_execute_soa_runtime_w`) | one symbol, `w` a genuine register-bound runtime argument, no unroll | Yes, confirmed by disasm | Scalar: 39-line body, loop bound `cmp x23, x19` (`x19` traces to the `w` argument register), zero NEON. SoA: 59-line body, same register-bound shape, dispatches into the shared vector kernel via `bl` (not inlined; see Finding 2). Neither shows a literal loop bound; both are genuinely W-agnostic at compile time. |
| `dispatch_table` (`cr_execute_scalar_dispatch` / `cr_execute_soa_dispatch`) | one symbol, runtime `w`, callee-side branch to const bodies | Yes, but the branch is a power-of-two/`clz` decision tree, not a naive chain or a plain jump table (Finding 4) | 383-line body, 77 branch-shaped lines; confirmed distinct instruction sequence from both `runtime_w` and `per_w_set`, so it is not accidentally degenerating into either. |
| `per_w_set` (`cr_execute_scalar_w{K}` / `cr_execute_soa_w{K}`) | N distinct symbols, W baked into the exported signature, no width argument, caller resolves once | Yes, and structurally so: the `CrEntryMono` type (`fn(*mut c_void, *const u64) -> u64`) has no slot for a runtime width to occupy | `cr_execute_scalar_w64`: 34-line body, loop bound `cmp x22, #0x40` (a compiled-in literal). Cannot regress to dispatch-table shape without changing its exported C signature, which nothing in the harness does. |
| Payload orthogonality (scalar vs SoA), all forms | the SIMD width (8) is decoupled from the ABI width W, so runtime-W legitimately reaches the SoA payload, contrary to the fork docs' "structurally impossible" framing for a naive design | Yes | `LANES: usize = 8` fixed const (`lib.rs:43`); `soa_batch` chunks the runtime `w` by `LANES` internally; the single shared `interpret_vertical_checksum_into::<8>` kernel is called identically from `runtime_w`, every `soa_w{K}`, and `soa_dispatch` (one label, confirmed by disasm), so entry-form choice never changes which vectorised code runs, only how W binds to the call. |
| Two-object split (all forms) | genuine cross-object call the optimiser cannot inline | Architecturally yes, independent of Finding 1 | The resolved pointer is a `libloading::Symbol` deref against a `dlopen`'d separate `.dylib`; no compilation unit that will ever host the timed cell can see through it. This part of the design survives the generation-pipeline break untouched; it is the one piece of the entry-form architecture that is sound *before* Finding 1 is fixed, not merely after. |
| Zig entry-form floor (`zr_*`) | mirrors the Rust `cr_*` scalar family byte-exact, so the entry-form conclusion transfers to the real target language | Yes, confirmed by `cargo test`'s `zig_matches_rust_byte_exact` and `zig_crossing_matches_rust_byte_exact` | `scalarBatch` in `interp.zig` is `inline fn` called with a comptime-literal width for the per-W set, mirroring the Rust `mono!` macro's `#[inline(always)]` + literal-width pattern; the same structural argument (const width unrolls, runtime width stays register-bound) applies, though I did not independently disassemble the Zig object's machine code the way I did the Rust one (out of scope for the time spent here; worth doing before the panel trusts the Zig row of any future ISA table the same way it should now distrust the SoA row above). |

## Open questions for the synthesiser

Whether the fix to Finding 1 should separate "batch width W" from "the `#[bench_variant]` const-generic size
slot" by giving the boundary families their own `LIB_TEMPLATE` variant (a template that substitutes a
per-decl size list into the `#[bench_variant(...)]` attribute), or by widening `generate_all`'s validation to
accept any decl-declared size list and always emitting exactly that list into the generated dispatcher. The
first keeps the ordinary carrier benches' template untouched and adds a second, boundary-shaped template; the
second is a single shared template made fully data-driven, at the cost of every family (including the
existing carrier ones) now depending on the per-decl list being emitted correctly. Both resolve Finding 1;
neither is obviously cheaper, and the choice affects how much of `mockspace-bench-matrix` changes versus how
much stays fixed.

Whether the field-count axis in `abi_marshal` (currently riding the same overloaded `n` slot as everything
else, semantically a third distinct meaning alongside "program size" and "batch width W") should get its own
named slot in `MatrixDecl`, or whether the reconciled fix for W generalizes cleanly enough that "whatever this
family's swept axis actually means" is simply left to the family's own `setup` function to interpret, with no
attempt to give the codegen layer semantic knowledge of what a decl's `sizes:` represents beyond "the set of
values the generated dispatcher must be able to accept."

Whether the ISA-shape gate's evidence table should be regenerated by a script (grep between two label
boundaries, mechanically, per symbol) rather than assembled by hand, given that the one row I re-derived by
hand did not match a hand-authored claim in the doc; a scripted `isa_audit.py`-style pass over every claimed
symbol would remove the class of error Finding 2 found, at the cost of writing and maintaining that tooling
now rather than trusting manual disassembly reading per symbol per family as the composition matrix grows.
