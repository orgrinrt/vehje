# The carrier composition matrix through the interpreter-generation lens (Haoran Xu)

**Date:** 2026-07-22
**Scope:** review of `mock/benches/carrier/` (the shared IR, generators, dispatch cells, staged
pipeline), `mock/benches/carrier-zig/`, `mock/benches/disasm-probe/`, against the mandate in
`mock/design_rounds/202607221700_topic.bench-composition-matrix-synthesis.md` and the build-complete
claim in `202607221915_topic.bench-composition-matrix-build-complete.md`, cross-checked against
`mock/research/202607221230_fairness-audit-disassembly.md`. I read the source, ran `cargo test` (default
features and with `threaded,vertical,jit` on this M1), rebuilt `disasm-probe` and re-derived a chunk of
the ISA evidence myself from `llvm-objdump` output rather than trusting the audit doc's table at face
value.

I built LuaJIT Remake to show a hand-tuned interpreter can beat a JIT on real workloads, and I co-authored
copy-and-patch compilation (Xu and Kjolstad, PLDI 2021), so I read a claim like "copy-and-patch cell,
byte-exact, ISA-confirmed" the way I read my own code review comments: with the disassembly open next to
the claim, not instead of it. The short version of what follows is that this arc did more of that than
almost any bench I have reviewed in this workspace, and it still has real gaps, some of exactly the kind
the mandate itself worries about.

## Verdict in one line

Not ready to run: the discipline on the cells that were audited is genuinely strong, but at least two
concrete asymmetries slipped past the audit's own method on the two most load-bearing reference cells (the
native ceiling and one CFG dispatch shape), the if-chain cell most likely does not measure what it claims
to measure, the single cell billed as the potential headline result (vertical SIMD) has zero ISA
confirmation, and no timing harness exists yet for anything this arc built, so today the "matrix" produces
cross-validated correctness and nothing else.

## What is strong (specific, cite file:line)

The shared unchecked access primitive is real and it is actually shared, not shared-in-name. Every
interpreter in `interp.rs`, `interp_threaded.rs`, `cfg.rs`, `cfg_threaded.rs`, `predecode.rs`, and
`vertical.rs` reads and writes through `access::rload`/`rstore` (`access.rs:22-32`), and `cargo test`
confirms this at the semantic level (`dispatch_shapes_agree`, `profiles_well_formed_distinct_and_agree`,
`lib.rs:117-141`). I re-derived the ISA-level part of this claim myself rather than trusting the audit
document: building `disasm-probe` and running `llvm-objdump` plus the audit's own `awk` classifier on this
M1 shows `di_switch` and `di_fntable`'s operand loads are plain `ldr` with no bounds-check
compare-and-branch, matching the claim in `202607221230_fairness-audit-disassembly.md:79-86`.

The preserve-none threaded dispatch claim is real, and I confirmed it independently. `interp_threaded.rs`'s
handler table (`TABLE`, lines 101-104) and every one of `cfg_threaded.rs`'s op and terminator handlers
(`h_set`/`h_add`/`h_jmp`/`h_brnz`/`h_ret`, lines 93-157) compile, on this machine, to exactly what the audit
claims: every straight-line handler I checked (`h_const`, `h_add`, `h_select`, `h_input` across
`interp_threaded`, `predecode::threaded_flat`, `predecode::threaded_direct`) shows `spills=0`, one tail
`br`, zero `blr`. This is the single most important fact the whole threaded axis rests on (that
`rust_preserve_none_cc` did not silently fall back to the standard ABI, which would have made the "no
callee-saved spill" story false), and it holds.

The record-layout compile-time contract (`ir.rs:114-187`) is a genuinely good piece of engineering: a
`Layout` cannot be named `REC24` and actually be 20 bytes, because a `const _: ()` block asserts every
named stride at compile time. This is exactly the kind of thing a careless bench gets wrong once and never
notices; here it cannot happen.

The Zig cross-language cell is the most valuable single addition in this arc, and it is done honestly.
`carrier-zig/interp.zig`'s tail cell exercises a dispatch shape (`@call(.always_tail)` token threading)
that this repo's own admission is that Rust cannot express without an incomplete nightly feature, on
identical wire bytes, cross-validated byte-exact (`zigcheck`, confirmed green in my test run: `test
zig_matches_rust_byte_exact ... ok`). The doc's claim that every handler ends in one indirect `br` with
`bl=0, blr=0` (`202607221230...md:126-135`) is the right thing to check and, from what I can tell of the
crossval harness, is checked the right way (byte-exact output, not just "it ran").

The named-absence discipline for perfect-hash is correct reasoning, not a dodge. A minimal perfect hash
over a dense contiguous `0..16` opcode space degenerates to array indexing, i.e. exactly the fntable cell
already built; building a second copy of fntable and calling it "perfect hash" would be the caricature the
mandate explicitly forbids. Good call, correctly reasoned, `202607221230...md:139-141`.

The copy-and-patch cell's own imm12-window honesty (`copypatch.rs:19-28`, tested at
`copypatch.rs:300-309`, `oversize_program_declines`) is the right shape for a scope limit: decline rather
than silently miscompile above 4095 nodes/consts. This is the correct posture even though (see Finding 4)
the cell is not quite the technique its name and citation claim.

## Findings (numbered)

### 1. The native-ceiling reference floor was never migrated to the fidelity gate's own checksum discipline

**Problem.** `access.rs:1-20` states the fidelity gate's central fix: move the per-node rolling hash out of
every interpreter's hot loop, fold once post-pass. `interp.rs`'s module doc (lines 1-11) says this
explicitly happened: "the per-node rolling hash that used to live in the dispatch loop is gone." It did
not happen in `native.rs`. `native_madd` (`native.rs:66-104`) still folds `hash = hash.rotate_left(7) ^ v`
inline, once per const load (line 84) and **twice per chain step** (lines 98 and 100, once for the `mul`
intermediate and once for the `add` result), inside the same loop that does the actual compute. This is
the exact per-node-hash-in-the-hot-loop shape the gate removed everywhere else.

**Why it distorts.** The native ceiling is one of the two reference floors (`202607221700...md` Part 1,
"Attribution: two reference floors") every profile's interpretation slope gets normalized against. Every
interpreter now pays zero folding cost in its hot loop (they write to `results[]` and the caller folds
once, `access.rs:34-45`); `native_madd` pays two rotate-xor operations per step in its hot loop, on top of
the actual mul/add it is supposed to be the undiluted ceiling for. A ceiling that is itself carrying
diluting overhead the things being measured against it no longer carry will read as slower than a true
native ceiling, which compresses the reported interp/native ratio and understates real interpretation
overhead in exactly the direction that makes every interpreter look comparatively better than it is. The
test that would have caught this (`native_madd_matches_interp`, `lib.rs:193-208`) only checks that the
folded value agrees with the interpreter's post-pass checksum, which it will, by construction, regardless
of where the fold happens; correctness tests do not check cost-shape parity, which is the whole reason this
review exists.

**Concrete fix.** Give `native_madd` a `results: &mut [u64]` parameter, write each intermediate the same way
the interpreters do (`results[i] = v`, no fold), and let the caller run `access::checksum(results)` once,
exactly as `run_over_input` does for the interpreters (`interp.rs:352-359`). This is a small, mechanical
change; it should happen before this cell is ever the denominator of a reported ratio.

### 2. CFG fn-pointer-table dispatch pays two dead-operand loads per SET node that switch and threaded both eliminate; confirmed by disassembly, not just source-reading

**Problem.** `cfg.rs`'s switch cell (`interp`, lines 70-78) short-circuits: the `match` arm for `op::SET`
is `ins.imm`, no register read at all. The fntable cell (`interp_fntable`, lines 118-131) calls
`CTABLE[ins.op as usize](ins.imm, rload(rp, ins.a as u32), rload(rp, ins.b as u32))`: because this is a
call through a runtime-resolved function pointer, Rust's argument evaluation is eager and the compiler
cannot know statically that the callee (which might be `c_set`, which ignores both extra arguments) will
discard them, so both loads execute unconditionally for every instruction including SET.

I did not take this on faith from reading the source. I rebuilt `disasm-probe` and disassembled it, and
the machine code confirms exactly this split, at three separate sites:

- `cfg.rs` switch: no load instructions for SET at all (it is a literal move of `ins.imm`).
- `cfg_threaded.rs` `h_set` (the `op_handler!` macro instantiation at line 93): despite the macro's source
  unconditionally computing `rload(regs, ins.a)` and `rload(regs, ins.b)` before calling `combine`
  (`cfg_threaded.rs:78-84`), the disassembled `h_set` (symbol
  `...cfg_threaded5h_set`, address `0x30b78`) shows exactly one `ldr` (the `imm` field) and zero loads of
  `regs[a]`/`regs[b]`: LLVM proved the closure `|imm, _a, _b| imm` never reads them and eliminated the dead
  loads inside that one function body.
- `cfg.rs` fntable (`di_cfg_fntable`, address `0xe94`): inside the inner instruction loop, immediately
  before the `blr` at offset `0xf60`, the disassembly shows `ldr x1, [x26, x9, lsl #3]` (loading
  `regs[a]`) and `ldr x2, [x26, x9, lsl #3]` (loading `regs[b]`) unconditionally, for every instruction
  regardless of opcode, because the call site services all five possible callees through one indirect
  call and the compiler cannot prove any one of them is unreachable.

**Why it distorts.** SET is not a rare op in the CFG kernels: it appears once per outer-loop iteration in
`build_nested_loop` (`cfg.rs:166-168`, resetting the inner counter) and five times in `build_branchy`'s
entry block. Every SET the fntable cell dispatches costs two extra loads that neither the switch cell nor
the threaded cell pays, for reasons that have nothing to do with "function-pointer-table dispatch" as a
technique and everything to do with how eager argument evaluation interacts with an indirect call site.
This is precisely the class of "obvious and embarrassing in hindsight" asymmetry
`202607221700...md`'s Amendment section calls out, and the audit's own reproduce script
(`202607221230...md:145-151`) would never catch it: it only classifies `stp`/`br`/`blr` presence per
symbol, not per-opcode load counts within a symbol.

**Concrete fix.** Change `CFn`'s signature so the table holds `fn(&Instr, *const u64) -> u64` and lets
each op function decide what to load, mirroring how the straight-line fntable (`interp.rs:63-104`) already
does it (each `f_*` function calls its own `rload`s, so the eager-evaluation problem cannot arise because
there is nothing being eagerly evaluated at the call site). File: `cfg.rs:96-131`.

### 3. The frequency-ordered if-chain most likely does not measure a linear compare cascade; the ISA audit table has no row for it

**Problem.** `interp.rs:124-129`'s doc comment calls the frequency-ordered if-chain "the honest strongest
form of the technique" and treats reordering hot ops to the front as the load-bearing fidelity fix
(restated in `202607221700...md` Part 0 and the Amendment's fidelity-gate list, item 2). I disassembled
`di_ifchain` myself. Inside its inner dispatch loop, at file offset `0x1954`, the sequence is:

```
ldrb w0, [x12, x16]      ; load a byte from a lookup table, indexed by the opcode
add x17, x17, x0, lsl #2 ; compute a branch target: base address + table_byte * 4
br x17                   ; indirect branch to the computed target
```

`x12` is set up earlier via `adrp`/`add` to a static table baked into the binary. This is LLVM's standard
jump-table-via-byte-index lowering (`SimplifyCFG`'s switch formation recognizing the sequential
`if opcode == k` chain over a small dense integer range and canonicalizing it), not a linear scan. Contrast
`di_bittree`, which genuinely shows `br=0, blr=0` in the same audit (a real compare tree, no computed
branch): the if-chain and the bit-tree were meant to be two different dispatch mechanisms and only one of
them still is one at the ISA level.

**Why it distorts.** If the if-chain compiles to essentially the same O(1) computed-jump dispatch as
switch, then (a) the frequency-ordering fix the mandate treated as important is very likely moot, because a
jump table does not care what order the source-level comparisons were written in, and (b) any measured
"if-chain vs switch" delta is measuring two near-identical lowerings, not "linear scan vs jump table" as
the axis's whole reason for existing claims. The `202607221230...md` "Per-cell ISA confirmation" table
has rows for switch, flat switch, fntable, bit-tree, and every threaded family. It has no row for if-chain
at all, ascending or frequency-ordered, so this was never checked.

**Concrete fix.** Either (a) accept and report the finding as-is ("if-chain and switch converge to the same
mechanism under LLVM; this axis measures compiler canonicalization, not the textbook technique"), or (b)
force the intended lowering with a barrier between arms (`core::hint::black_box` on the comparison result,
or restructuring the cascade as a chain of `#[inline(never)]` calls) and accept that this is now testing an
artificially preserved technique rather than how a real embedded interpreter's own compiler would actually
lower this code. Either way, add a `di_ifchain`/`di_ifchain_ascending` ISA check to the audit table before
trusting any number from this axis. File: `interp.rs:130-243`; audit gap:
`202607221230_fairness-audit-disassembly.md` (no if-chain row).

### 4. The copy-and-patch cell is a hand-encoded native emitter, not Xu and Kjolstad's stencil-copy-and-patch mechanism, and the citation overclaims fidelity

**Problem.** `copypatch.rs:1-10` names the technique and cites the paper I co-authored. What is actually
built is a direct native-code generator: `emit()` (`copypatch.rs:123-186`) hand-selects an aarch64
instruction sequence per IR node using Rust functions that encode raw 32-bit opcodes (`ldr`, `add`, `mul`,
`csel`, etc., lines 46-118) and concatenates the words. The defining mechanism of copy-and-patch as
published is different: each operation's code is produced by compiling a small stencil function through a
real backend (originally LLVM, later shown to work through other backends), and the JIT step is a `memcpy`
of that precompiled template's bytes with the linker-recorded relocations patched in at codegen time,
specifically so the JIT author never hand-selects instructions. What is here is closer to what the
JIT literature calls a template compiler or a "baseline" native-code generator (in spirit similar to
early V8 full-codegen or SpiderMonkey's baseline, not to the stencil-extraction pipeline the citation
points at).

**Why it distorts.** This is not a correctness problem (the doc comment's honesty about scope, and the
byte-exact cross-validation against the interpreter, are both real and good). It is a representativeness
problem the mandate cares about directly ("every composition gets a proper, sound, actually representative
implementation... not a caricature"). A genuine stencil-copy pipeline pays real costs this hand-tuned
emitter does not: each stencil is itself a compiled function boundary (register save/restore or a
calling-convention-constrained argument shuffle at minimum, unless the stencils are compiled with a custom
calling convention specifically to avoid it, which is itself extra engineering the real technique bears and
this cell does not), and the codegen step is a template copy plus relocation patch rather than a
from-scratch instruction-selection pass written by hand in Rust. This cell's `S` term (codegen cost) and
its `I` term (near-zero dispatch) are both plausibly more optimistic than what a faithful realization of
copy-and-patch would produce. If this number ships labeled "copy-and-patch," a reader who knows the
technique (as I do) will read it as a claim about the published mechanism's real cost, not about an
idealized upper bound on the near-native tier.

**Concrete fix.** Relabel honestly: "direct native codegen (near-native ceiling; a template-JIT upper bound,
not a stencil-extraction realization of copy-and-patch)". If the actual copy-and-patch mechanism is wanted
as a comparator, it needs a real stencil pipeline (compile per-op template functions through a backend that
records relocations, memcpy plus patch at codegen time), which is a materially larger build, not a rename.
File: `copypatch.rs:1-28` (doc/citation), `123-186` (the mechanism).

### 5. Vertical/SoA SIMD, the cell billed as the potential headline result, has zero ISA-level confirmation

**Problem.** The build-complete doc calls vertical interpretation "the standout idea," and the synthesis
doc calls it "potentially the single largest result the matrix can produce" and demands it be first-class.
`disasm-probe/src/lib.rs` wraps every other beyond-runtime cell (`di_switch`, `di_fntable`, `di_bittree`,
`di_ifchain`, the predecoded family, all three CFG shapes) but has no `di_vertical` wrapper and no `Simd`
import at all. `202607221230...md`'s "Cells whose fairness is not a dispatch-label question" section
asserts vertical/SoA's fairness is "a genuinely different evaluation shape... its advantage is the real
property under test, not an artefact," which is a claim about semantics (backed by the lane
cross-validation test, which does pass: `vertical::tests::vertical_lanes_match_scalar`), not a claim about
codegen quality, and no codegen evidence is offered.

**Why it distorts.** This is the specific axis where codegen quality is not a side detail, it is the whole
result. `Simd<u64, W>` is a portable abstraction; whether `interpret_vertical::<8>` actually lowers to
packed NEON vector instructions or gets scalarized into W independent scalar operations on a target whose
native vector register is 128 bits (two `u64` lanes) depends entirely on how well the auto-vectorizer
handles the per-node match arms with `SimdOrd`/`SimdPartialEq` select operations, and that is exactly the
kind of thing that silently degrades. A vertical cell that quietly runs as W unrolled scalar loops would
still cross-validate correctly (same values, same checksum) while reporting a "SIMD win" that is really
just loop unrolling, or would report no win at all for the wrong reason (scalarization overhead masking
what real SIMD would have delivered). Both directions are exactly "what would make a beyond-runtime cell
silently win or lose for the wrong reason," and the one axis specifically flagged as the likely headline
is the one with no instrument on it.

**Concrete fix.** Add `di_vertical4`/`di_vertical8` wrappers to `disasm-probe` and confirm packed
`ins.2d`/`fmla`-class NEON instructions appear in the dispatch loop, not W duplicated scalar instruction
sequences, before this cell's numbers are trusted. File: `disasm-probe/src/lib.rs` (missing wrapper),
`vertical.rs:34-75` (the loop to confirm).

### 6. `eqsat` is quietly absent from the `optimize` stage composition, not named as an absence

**Problem.** The synthesis doc's staged-pipeline section states the optimize stage as "none / CSE / bounded
eqsat / CSE+eqsat" (`202607221700...md`, "Part 3", stage 1) and the execution-roadmap section repeats it
("optimize: none, CSE..., bounded eqsat..., const-fold, DCE, and their compositions (at least
none/CSE/eqsat/CSE+eqsat...)"). The shipped `optimize()` function (`optimize.rs:76`) takes exactly
`(prog: &Program, cse: bool, fold: bool, dce: bool)`: there is no eqsat parameter and no call into the
`eqsat` module anywhere in `optimize.rs`. `eqsat.rs` is a fully separate module: its `EGraph` (lines 37-61)
operates on a synthetic `Op`/`build_chain` representation (line 262) that has nothing to do with
`carrier::ir::Program`, and its own tests (`cap_is_respected`, `bounded_and_unbounded_extract_equal_value`,
`unbounded_grows_far_larger_than_bounded`) never touch a carrier-generated program.

**Why it distorts.** This is not a correctness bug (both modules work fine on their own terms). It is a
mandate-compliance gap: the mandate is explicit that "a proposal that turned out to be a strawman on this
IR is the only thing that is not built, and each such case is named with its reason, never quietly
dropped" (`202607221915...md`, "The mandate, restated in full", part 1). eqsat's absence from the optimize
composition is real, and it is not named anywhere as a deliberate exclusion the way perfect-hash and
interned-operands are (`202607221230...md:139-141`). A reader of the build-complete doc's "working-set
stages" paragraph (which lists CSE, fold, DCE, fusion, liveness, output-building, correctly, and simply
omits eqsat) would reasonably conclude the four-way optimize composition from the design phase shipped as
designed. It did not.

**Concrete fix.** Either wire a bounded-eqsat pass over `carrier::ir::Program` into `optimize()` as a fourth
strategy (the design's own stated highest-leverage axis), or add eqsat to the named-absence list with the
same rigor as perfect-hash, stating specifically why a `Program`-level integration was skipped. File:
`optimize.rs:76` (the function signature that should carry it), `eqsat.rs` (the disconnected module).

### 7. No bench.toml or variant-crate wiring exists for anything this arc built; today's "matrix" is cross-validated source, not a runnable bench

**Problem.** `bench.toml`'s only carrier sections are `carrier_record_width`, `carrier_dispatch_v4`,
`carrier_dispatch_v17`, and `carrier_predecode`, all pointing at `variants/carrier_*` crates that predate
this arc (the old A1/A3-era Python-generated variants named directly in the fairness audit's "confound"
history). There is no bench.toml section, and no `variants/` crate, for: the fidelity-gate-normalized
switch/fntable/ifchain/bittree, any threaded family, any CFG dispatch shape, any of the six new profiles,
vertical SIMD, copy-and-patch, trace, the Zig cell, stack bytecode, or any staged-pipeline composition. The
build-complete doc's own "state and next step" section admits this: "the remaining steps before a run are
mechanical: re-pin the vehje bench crates past the mockspace branch once it merges, and regenerate the
carrier variants through the new matrix generator that replaces the Python scripts." That matrix generator
lives on an unmerged sibling-repo branch (`mockspace`'s `feat/bench-harness-cost-model-and-matrix`), not in
this repo.

**Why it distorts.** It does not distort a comparison; it means there is, today, no comparison to distort.
Every cell in this arc is proven correct by `cargo test` (which I ran and confirmed green, 44 tests with
`threaded,vertical,jit` on this machine, plus the Zig crossval). None of them can produce a single timed
number without either the upstream harness feature landing or someone hand-writing a `bench.toml` section
and a variant crate for at least the Tier-0 spine, which is exactly the hand-rolled-Python pattern the
"everything upstreamable goes upstream" mandate exists to retire. This is the direct, load-bearing answer
to "what does it still need before a run": a runnable harness, not more cells.

**Concrete fix.** None needed from the carrier side; this is a sequencing fact to state plainly rather than
a defect to patch. Worth flagging loudly in any status report so "build-complete" is not read as
"run-ready."

### 8. Sink (live-out) cardinality varies non-obviously across the six profiles, confounding cross-profile comparison of the stages whose cost scales with it

**Problem.** `optimize::sinks` (`optimize.rs:31-`) defines live-outs as nodes referenced by no later node.
Whether a node ends up referenced depends on `locality_window`: with a small window (`p_tight`:
`locality_window: 8`, `p_madd`: `locality_window: 4`), a node falls permanently out of every later node's
reachable operand range quickly, so a meaningfully sized band of recent nodes stays unreferenced (a sink)
at any given point. With `p_scatter`'s `locality_window: usize::MAX` (`gen.rs:136-160`), any node can be
referenced by any later node for the entire remaining program, so the expected sink count for the same
node count is structurally much smaller. This is an emergent property of the locality axis, not something
any profile states or the tests check.

**Why it distorts.** The fusion, liveness, and output-building stages (`fusion.rs`, `liveness.rs`,
`output_building.rs`) all cost scale with sink count (fusion never fuses a sink away, output-building's
gather/inline cost is proportional to the sink set, liveness's `out_slots` never free). Comparing, say,
"fusion helps 12% on P_real but only 3% on P_scatter" without reporting each profile's actual sink count
risks attributing a locality-driven cardinality difference to the stage itself. This is exactly the kind of
non-obvious confound the mandate's "any other asymmetry imaginable is hunted deliberately" clause is meant
to catch, and it is specific to the newly designed profile axis, so it could not have been caught by an
audit written before the profiles existed.

**Concrete fix.** Report sink count per profile alongside every stage-level number (cheap: `sinks(prog).len()`
is already computed for cross-validation). File: `gen.rs` (the locality_window values per profile),
`optimize.rs:31-` (the sinks definition).

### 9. Interior INPUT nodes can appear under uniform weights, contradicting the generator's own stated intent (minor, not a fairness bug)

**Problem.** `gen.rs:221-227`'s doc comment states "the next leaves are CONST" as if only node 0 can be
INPUT. `uniform_weights()` (line 80-82) gives `op::INPUT` weight 1 like every other op, and
`GenParams::default_point` (hence `p_real`) uses it, so `weighted_op` can and does draw INPUT for interior
nodes with roughly 1-in-17 probability past the leaf-seed phase (`gen.rs:241-253`).

**Why it does not distort.** INPUT is a legitimate arity-0 leaf (`op::ARITY[16] == 0`) wherever it appears,
so `is_well_formed` accepts it and every interpreter handles it correctly (confirmed:
`generated_program_is_well_formed` passes). If anything, more input-dependent interior nodes make
partial evaluation of the program by an over-eager optimizer less likely, not more.

**Concrete fix.** A one-line comment fix ("CONST dominates but INPUT can recur under uniform weights") so
the module doc matches what the code does; not urgent. File: `gen.rs:221-227`.

### 10. The interned-operand exclusion is argued globally but the profile most likely to falsify it (P_madd) was not checked

**Problem.** `202607221230...md:139-141` states interned operands are not built because "a value-DAG
where operand tuples do not repeat" makes it a strawman, citing this as settled. `p_madd`
(`gen.rs:106-113`) is specifically designed as "a single repeated multiply-add motif, tight locality"
(`op_correlation: 900`, `locality_window: 4`), which is exactly the condition under which the SAME operand
tuple (the same two earlier node indices, in the same relative positions) recurs across the program, since
a highly correlated stream over a 4-wide window has few distinct tuples to draw from.

**Why it distorts, if wrong.** If P_madd (or a future profile like it) does produce meaningfully repeated
operand tuples, then the global "does not compose" verdict was reached without checking the one profile
built specifically to stress exactly this property, and a genuinely representative technique would have
been excluded on an untested premise, which the mandate treats as no better than a caricature ("a
non-representative cell is worse than a named absence" cuts both ways: an absence justified by an untested
claim is not yet earned).

**Concrete fix.** Before finalizing the exclusion, measure actual distinct-operand-tuple counts on P_madd
(cheap: a `HashSet<(u32,u32)>` over binary-op operand pairs) to confirm the strawman claim holds even on
the profile designed to challenge it.

## The fairness question (cell by cell)

**Switch, fntable, bittree (straight-line).** Sound. ISA-confirmed by the existing audit and independently
re-derived here. No open concern.

**If-chain, both orders (straight-line).** Likely unsound as a distinct technique per Finding 3: the
compiled code is probably a jump table, not a linear cascade, and was never ISA-checked.

**Threaded (all three straight-line families, and all CFG handlers except the eager-load site).** Sound,
and the strongest-audited cell in the whole arc: zero spills, real tail branches, confirmed independently
by me on this machine, not just accepted from the doc.

**CFG switch and fntable.** Switch is sound. Fntable carries the SET eager-load asymmetry from Finding 2,
confirmed by disassembly, not present in the doc's audit.

**CFG threaded.** Sound; the same op_handler macro that gives fntable its eager-load problem gets its dead
loads eliminated by the compiler when the callee is statically known inside its own body, which is the
subtle and slightly lucky reason threaded does not share fntable's defect here.

**Vertical/SoA SIMD.** Semantically sound (cross-validated per lane). Codegen quality entirely unverified;
see Finding 5. This is the cell where "sound but unaudited" is least acceptable given how much weight the
design places on it.

**Copy-and-patch.** Semantically sound (byte-exact against the interpreter across every op). Representative
of "a near-native tier exists between interpreter and full native," not representative of the specific
named technique; see Finding 4. Not a fairness bug against other cells (it does not borrow anyone else's
budget), but a mislabeling risk against the literature it cites.

**Trace/superblock.** Sound, and honestly scoped (single self-loop only, stated plainly as a limitation, not
hidden). No open concern beyond the scope already disclosed.

**Zig switch and tail.** Sound, and the most rigorously ISA-confirmed cell in the arc (I did not
independently redo the Zig disassembly given the session's remaining budget, but the Rust-Zig
cross-validation passed on this machine and the claimed mechanism, `@call(.always_tail)` lowering to a real
indirect tail branch with zero `bl`/`blr`, is exactly what I would expect from that Zig feature and is not
a surprising claim to accept).

**Optimize, fusion, liveness, output-building (stages).** Each individually sound on its own cross-validation
contract. Two composability gaps reduce how much the matrix can actually say cross-profile: eqsat's silent
absence from optimize (Finding 6) and sink-cardinality confounding stage comparisons across profiles
(Finding 8).

**Value representation (static/tagged/nanbox).** Internally sound, three-way cross-validated, and its scope
limitation (monomorphic per-site typing, explicitly not testing megamorphic misprediction) is honestly
stated in its own doc comment. It is not a composable cell today: it runs a wholly separate mini-IR, so
calling it "the value-representation axis" of "the matrix" overstates its integration. It is its own
correct, isolated island bench.

**Native ceiling.** Semantically correct, cost-shape wrong; see Finding 1. This is the single most
consequential fairness finding in this review because every other profile's numbers are meant to be read
relative to it.

**Null-dispatch floor.** Sound by design; correctly excluded from cross-validation (it computes a different
result on purpose) and correctly folds its own checksum.

## Novel angles worth stealing

**Derive every dispatch cell's per-op body from one semantic table, the way Deegen derives every execution
tier from one bytecode definition.** The twelve arithmetic/logical binops (ADD, SUB, MUL, AND, OR, XOR,
SHL, SHR, MIN, MAX, EQ, LT) are hand-transcribed independently at minimum seven times in this codebase:
`interp.rs`'s `bin!` macro and its separate `binop!` macro (two copies in one file), `interp_threaded.rs`'s
`bin_h!`, `vertical.rs`'s per-op `match`, `copypatch.rs`'s per-op `match` (as aarch64 encoders), `stackbc.rs`'s
`bin()`, and `trace.rs`'s smaller-vocabulary `eval_instr`. Cross-validation catches semantic drift after the
fact, which is a real safety net and it works (every test I ran passed), but the generation-time duplication
is exactly the failure mode Deegen exists to remove: a semantics change (say, making SHR arithmetic instead
of logical) has seven places to land correctly and six chances to silently not. The fix is mechanical and
in keeping with this arc's own subject matter: one `macro_rules!` or const table naming each op's pure
`fn(u64, u64) -> u64` (or the aarch64-encoder equivalent for copy-and-patch) once, with every dispatch cell's
codegen (the switch arm, the fntable function, the threaded handler, the CFG register op, the copy-and-patch
stencil, the vertical SIMD lane-op, the stack-bytecode `bin`) generated or instantiated from that one
definition. A bench built to prove "write once, derive every tier" beats "hand-maintain N tiers" should not
itself be the counterexample.

**Generalize the ISA audit from label-matching to a per-opcode instruction-schedule diff.** The current
audit (`202607221230...md`'s reproduce script) classifies a whole symbol by counting `stp`/`br`/`blr`
occurrences across the entire function. That is exactly coarse enough to miss Finding 2 (a load-count
asymmetry on one specific opcode, buried inside a symbol that otherwise looks fine) and Finding 3 (a
cascade that collapsed into a jump table, which the existing classifier would have flagged as "br=1", the
same signature as a genuine jump table, had anyone looked). A stronger, still cheap, audit: for each
dispatch cell, disassemble the code path reached for each individual opcode (or, for threaded dispatch,
each individual handler) and diff the instruction count and instruction mix against every other cell's
path for the same opcode. Any two cells claiming to compute the same op should show the same load/store
count for that op; a divergence is either a real, disclosed technique difference (fine) or exactly the kind
of asymmetry this review found by hand (not fine). This is the same instinct that makes Deegen's own build
step compare generated interpreter and JIT-tier code against the semantic definition; applying it here
would have caught two of this review's findings mechanically instead of by a human reading raw hex.

Sources: Xu, H. and Kjolstad, F., "Copy-and-Patch Compilation: A Fast Compilation Technique for High-Level
Languages and Bytecode," PLDI 2021 (cited by name in `copypatch.rs:6`; the mechanism description in Finding
4 is drawn from that paper's own account of stencil extraction via compiled templates and relocation
patching, not from a web search this session).

## Open questions (calls you cannot make alone)

**If-chain: accept the jump-table collapse and report it, or fight the compiler to preserve the technique.**
Accepting means stating plainly that if-chain and switch converge under LLVM and the axis mostly measures
that convergence, not the textbook linear-scan-versus-jump-table tradeoff. Fighting it (barriers between
comparisons, `#[inline(never)]` per arm) means the cell no longer represents how a real interpreter's own
compiler would actually lower this code, only how it lowers under artificial constraints. Both are honest
positions; they answer different questions ("what does source-level if-chain actually cost once compiled"
versus "what would the textbook technique cost in isolation").

**Copy-and-patch: relabel the current cell, or build the real stencil-extraction mechanism.** Relabeling is
cheap and immediately honest. Building the real mechanism (a second compilation pipeline: per-op stencil
templates compiled through a backend, relocations recorded and patched at codegen time) is a materially
larger engineering investment whose payoff is a cell whose number means exactly what its name and citation
claim, at the cost of real build time this arc has not budgeted.

**Vertical SIMD: ISA-confirm before running, or run and treat scalarization as a post-hoc finding.**
Confirming first (a `disasm-probe` wrapper, cheap) catches a silent-loss (or silent-not-really-SIMD-win)
failure mode before it ever produces a number anyone reports. Running first accepts the risk that the
headline result of the whole arc could be an artifact of auto-vectorizer behavior discovered only after the
fact, which is expensive to walk back if it has already been cited.

**eqsat in the optimize stage: build the real `Program`-level integration, or formally narrow the named
scope.** Building it matches the design's own stated "highest-leverage axis" claim and the mandate's "build
everything" instruction literally. Narrowing it (naming eqsat as excluded, the same way perfect-hash and
interned-operands are named) accepts a smaller optimize axis (CSE/fold/DCE only) as the honestly-labeled
delivered scope, at the cost of not measuring the one optimization the design called out as most
consequential for downstream working-set size.

**Sequencing the harness dependency: wait for the upstream mockspace branch, or hand-wire a minimal
bench.toml now.** Waiting matches the "everything upstreamable goes upstream, never hand-roll a generator
again" discipline and avoids throwaway scaffolding. Hand-wiring gets a first real timing number sooner (at
minimum for the Tier-0 spine), at the cost of writing exactly the kind of one-off `bench.toml` section this
arc's own mandate says should not need writing by hand anymore.
