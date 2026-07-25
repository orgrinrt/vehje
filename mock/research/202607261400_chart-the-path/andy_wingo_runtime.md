# The complete vehje runtime (Andy Wingo)

## Canon gate outcome

**Aligned, and I proceed.** Checked against `mock/research/canon/the-soul-of-vehje-positive-catalogue.md`
and `mock/research/canon/the-inverse-of-vehje-negative-catalogue.md` (both read in full before any source),
`mock/research/202607260100_op-standing-design-calls.md`, and the ratification record at
`mock/research/202607241400_ir-to-ir-and-the-compile-runtime-line/op-ratification-answers.md`. I did not read
`mock/design_rounds/202607241615/202607241545_topic.the-vehje-canon.md` and I cite nothing from it; per the
brief it is the round that produced the canon, not the canon, and every section number past eight in it is
uncitable by construction.

The question asked is on the canon: it asks what the canon calls for, names the canon as governing, and puts
existence and locus in scope. Refusing it would leave the drift standing.

The state it builds on is misaligned in eleven places I can cite by `file:line`, all in section
"Unlicensed mechanisms found". Four of them are on the shipped runtime itself, which is my lens, and two of
those are silent-wrong-answer or crash bugs I reproduced rather than inferred. Four questions the answer
depends on are handed back in the last section rather than resolved here.

One procedural note that belongs in the gate rather than in the body: op's call PE2 orders the CR1
continuation-representation bench run **now**, and section "What can be proven now" argues that one of its two
candidates cannot be built before an earlier step lands. That is not a refusal of the call. It is a claim that
the call has a prerequisite, evidenced by the two artifacts already in the tree that carry the word
"multi-shot" and measure loops instead. I state it as a dependency, and the sequencing call remains op's.

## Verdict in one line

The shipped runtime has no control stack of its own: it is a `switch` recursing on the host C stack, so tail
calls, loops past ~256 iterations, deep programs, per-frame unwind work, fuel, re-entry, and every form of
continuation are all blocked behind one missing data structure, and building that structure first is the whole
of the ordered path, because every mechanism the canon names downstream of it is a property of frames the
engine does not have.

## What a complete engine here must contain

The canon fixes the runtime's shape more tightly than most designs fix anything, so this section is mostly
transcription plus the consequences nobody has drawn.

**One artifact that owns every per-script analysis.** `negative:38` kills the dual locus: "Rust never runs
per-script analyses, the runtime does, for all scripts". `positive:45` says the Rust compiler "never sees an
end-user script, never ships". So lexing, parsing, resolution, inference, monomorphisation, macro expansion,
lowering, optimisation, and execution are all runtime-side. The consequence that has not been drawn: the
entire **diagnostic surface** moved with them. `positive:39` makes diagnostics first-class ("informative
errors *and* actionable suggestions with span links that integrate with LSP, everywhere") and says on a
no-alloc substrate that is "real architecture". Today the runtime's only outward channel is an `i32`
(`runtime.zig:521-548`) plus a value sink. An LSP-grade diagnostic set has to cross that boundary, and there
is no field for it in either ABI definition. Moving the front end into Zig moved a large, unbudgeted surface
with it, and nothing in the tree has noticed.

**A memory contract the host can actually satisfy, covering every region.** `positive:45`: "no heap on either
side (`no_std`, no alloc, host-owned memory via caller-lent arenas and host-lent budgets; PE3: this is an
axiom on both sides)". Count the regions the canon itself names: the residual image; the environment/frame
arena; the value arena; the handler scratch; **N** expansion buffers (`positive:33`, "N host-provided
allocation spaces"); the CR1 continuation budget (`positive:25`); and the diagnostics budget
(`positive:39`, "diagnostics get their own host-lent budget discipline"). Seven kinds. The one C descriptor
that exists, `BatchColumn` (`vehje-runtime-abi/src/entry.rs:36-48`), lends exactly two: the residual and the
record column, plus a sink. There is no place in the ABI for the other five. Since the count is explicitly
host-configurable and explicitly N, the descriptor cannot be a fixed struct of named pointers; it has to be a
kind-tagged region vector with a count, or every new region is an ABI break at the one interface batch 8
recorded as outliving every mechanism.

Comparable systems, because the discipline here is stricter than any of them and that is worth being precise
about. Lua takes one `lua_Alloc` and calls it whenever it likes. Wasm takes one linear memory plus tables and
grows them. eBPF is the closest analogue: 512 bytes of verified stack, no unbounded loops, explicit maps for
anything larger, and a verifier that refuses rather than a runtime that faults. vehje is stricter than all
three (no allocator at all, not even a host-supplied one) which means it must be *more* explicit about
regions, not less. The current shape is less.

**An explicit control stack, and this is the keystone.** The canon does not use the phrase, but it mandates
the structure five times over, each time as a property of frames:

- `positive:23` names three discharges of one handler discipline, the third being "runtime resumable
  continuation". A continuation is a description of the rest of the computation. If the rest of the
  computation is the host C stack, there is nothing to describe and nothing to copy.
- `positive:25` designs CR1 as "no-alloc bounded multi-shot continuations on a host-lent budget" and names the
  fork as "segmented capture-and-reinstate over a host-lent budget versus compile-time
  CPS/defunctionalisation". Segments of what? Of a stack the engine owns. There is no other referent.
- `negative:91` states the rule outright for the compile side: recursion on IR depth "violat[es] the project's
  own bench-proven 'defunctionalization is the only route' rule, so a deep-skewed 50k-node program overflows
  the host stack in exactly the embedded contexts an embeddable framework courts". The rule is about
  representing a traversal as data. It applies at least as hard to the artifact that actually ships.
- The loop encodings the census settled (`202607251530`, made operational in the Zig sketch's README) turn
  `while`, `for`, and `loop` into self-recursive functions. That is the right encoding, and it is only
  workable with proper tail calls, which are a property of how frames are pushed.
- `positive:33` supports recursive macro expansion with a host-configurable depth cap defaulting to 255 as a
  "hang-guard diagnosed as a named error". A depth cap you can diagnose is a depth you can count. You count
  frames you own.

Every implementation I know that does any of this owns its frames. Guile captures a delimited continuation by
`memcpy`ing the stack slice between the prompt and the capture point out, and reinstates by copying it back;
that is only possible because the VM stack is an object the VM allocated. Wasmtime's stack-switching
implementation (WasmFX, the proposal at phase 3 with `cont.new` / `suspend` / `resume`) gives each
continuation its own stack segment from an engine-owned pool. CPython since 3.11 keeps frame data in a chunked
data stack the interpreter owns precisely so generators do not need a C frame. Koka avoids owning stacks by
compiling handlers away into evidence-passing plus a monad, which is the *other* candidate in the canon's own
fork, and it works exactly because it never needs to capture anything at run time. There is no fourth answer.
Either the engine owns frames, or the front end compiles control flow away before the engine sees it. The
canon's fork (`positive:25`) is exactly that choice, correctly identified.

**The unwind must propagate, not jump, and it must not share one mutable slot.** The design memo
`202607252100_handle-clause-representation.md` names the propagation requirement and its reason (a future
`finally` has to run at each frame), and both implementations honour propagation. Neither honours the second
half: the operand carrier is a single mutable struct shared by every frame (`runtime.zig:132-136`, and
`eval.zig:174-176` in the sketch). I proved below that this silently drops an operand. In a design whose
centre of gravity is prove-then-erase, a runtime that returns a closure where the program says 20 is the worst
available failure: the proof was discharged and the evaluator broke it anyway.

**Family computation owned by the runtime.** Standing call 2 is unambiguous: "Host should only basically
configure the runtime and give it the allocations, nothing more... it should *NOT* have to give the simple
things like fucking arithmetics". The Core has no `+`, so arithmetic is a family operation; therefore the
runtime must be able to *execute* a family operation from data. That in turn means the Core needs a
distinction the runtime does not currently draw: `Raw` today means "ask the host" (`runtime.zig:410-434`),
`Perform` means "unwind to a lexical handler" (`runtime.zig:355-369`), and there is no form that means "this
operation is serviced by the host". After call 2 the host callback is for genuinely foreign effects only, and
those need a form of their own. The algebra is short one generator from the runtime's side.

**The specialisation seam, and the fact that it is a build-graph edge.** `positive:21` puts ahead-of-time work
at LanguageAuthor, "specialising the general Zig runtime to the language definition at our build, which is the
first Futamura projection as a build step, the technique weval demonstrates for wasm interpreters in
production". `positive:45`: "one hand-authored engine comptime-specialised to the Rust-emitted data". Call 10:
"The rust should extract the specialisations." Zig `comptime` needs its input at Zig compile time, so the
Rust-emitted package must reach the Zig build as a file, via `@embedFile` or a build-step-generated module.
That is a real, unavoidable structural fact and it has a consequence: **the specialised runtime is a build
product of the language author, not of vehje**, and a host embedding two languages links two of them. Call 9
licenses the other shape ("a very generic, not-specified shared runtime is fine, as long as it does not expect
any kind of language") which is one binary that decodes the table at load time and dispatches through it. Both
are ratified. Which ships, or whether both do, is in the last section.

**Refusal at every bound, with the bound lent.** The engine's whole safety story is its named error set
(`runtime.zig:100-117`), which is the right shape and is the eBPF lesson correctly learned. It only holds if
every bound is (a) lent by the host and (b) checked. Today several are compile-time constants documented as
lent, and one (native stack depth) is neither named nor checked and terminates the process.

**The tiers, in the order the measurements put them.** Arena tree-walk is the reference semantics.
Predecoded-register CFG is the middle tier (`negative:53`: register beats stack "every profile (1.41x-3.06x)";
`cfg-interp-throughput/findings.md`: 1.70 ns/instr against the arena walk's 4.4 ns/node, and the reason given
is the memory profile, which is the right reason). Native by direct instruction selection is a spectrum point,
never the driver (`positive:35`, `negative:52`). Optimisation is fold + CSE + DCE always-on, e-graph parked
(`negative:50`). Dispatch is plain `switch` for the straight-line lean IR and tail-threading only for CFG
terminator transfers (`negative:51`, `interp-dispatch/findings.md`: switch 4.58 ns/op against labeled 6.24 and
tail 6.38).

**The batched-column entry.** `positive:35` calls it "the single biggest lever, vertical/SoA SIMD ~4.8x with
the FFI crossing effectively free (~9 ns)". It is a Rust type alias with a FIXME (`entry.rs:56-64`) and has no
Zig counterpart.

## What this one is missing, in dependency order

Numbered as a build order. Each entry names what it blocks.

**1. A region-lending descriptor with a count, replacing the fixed struct and the stack arrays.**
`vehje_runtime_execute` declares roughly 96 KiB of locals before evaluating anything: `slots: [1024]Binding`
at 32 bytes each, `vslots: [1024]ValueNode` at 32, `vpool: [2048]u32`, `vblob: [8192]u8`, and `buf: [16384]u8`
(`runtime.zig:529-535`, `runtime.zig:542`; sizes measured, not estimated). `host.zig:127` adds a
`[1024]u32` map. The module doc claims the opposite at `runtime.zig:15-17`: "Environments are a caller-lent
bump arena". They are a stack array. `host.zig:160-162` says of two compile-time constants "Both are lent
bounds like every other here", which is false of both and of the others. On any target with a 4 to 16 KiB
thread stack, this runtime overflows at the entry point, before depth is even a question. Blocks: everything,
because every later mechanism needs a region and the current answer is the C stack.

**2. An explicit frame stack, and `eval` defunctionalised over it.** `eval` (`runtime.zig:278`) recurses per
Core node and, in `Apply`, per argument application (`runtime.zig:351`). I measured the ceiling on a
right-nested `Let` chain against the shipped source: **ok at depth 3,000 and SIGSEGV at 3,500 in the default
build; ok at 10,000 and SIGSEGV at 11,000 in ReleaseFast.** Not `EnvFull`, not `Corrupt`, a segmentation fault
in `eval` at `runtime.zig:330`. Three things make this worse than it looks. Right-nested `Let` is precisely
what the `Anf` pass is *designed* to produce, so the shape is not adversarial, it is the intended residual
form. The bound varies by optimisation level, and a safety property that depends on the optimiser is not a
property. And `runtime.zig:330` is a syntactic tail call that Zig does not turn into one, so even the case
that looks free is not. This is the same defect `negative:91` catalogues for `infer` and `structurally_equal`
on the compile side, uncatalogued here, on the artifact that ships. Blocks: 3, 4, 6, 8, 9, 10, and the
completeness bar, since a stdlib written in Clause (`call 3`) will be the deepest program in the tree.

**3. Proper tail calls and an environment reclamation rule.** Once frames are data, a body in tail position
reuses the frame. Without it, every `while` iteration is a native frame *and* burns environment slots that
never come back: `Env.push` bumps and there is no pop (`runtime.zig:176-181`), by design, so a loop body with
`k` bindings terminates with `EnvFull` after roughly `1024/(k+1)` iterations, about 256 for a three-binding
body. The sketch has the identical structure (`eval.zig:176-182`) and its README says so plainly: "Loops are
bounded by the evaluator's stack... task #49". Blocks: every loop, therefore the stdlib, therefore call 3.

**4. A per-frame unwind carrier.** The carrier is one shared mutable struct (`runtime.zig:132-136`) passed to
every `eval`, and the `Handle` arm reads `unwind.args[k]` and re-reads `unwind.n` as its loop bound across
iterations (`runtime.zig:388-394`). I built the adversarial program: a two-operand clause whose body evaluates
an inner handled computation between binding operand 0 and operand 1. The inner `Perform` overwrites `op`,
`args`, and `n`; the outer loop's bound collapses to the inner arity and the second operand is never applied.
**Observed: the handler returns a `closure` where the program's value is `20`.** Silent wrong answer, no
error, in the pass whose whole identity is that the type system already proved the program safe. Blocks: any
handler nesting, which is to say `return` inside a loop, which the census makes the ordinary case.

**5. Family operations executed from data, and a form for host-serviced effects.** `runtime.zig:414` returns
`NoHost`, so a program that adds two integers cannot run without a host supplying addition. The in-source
defence at `runtime.zig:411-413` ("which is what keeps the framework family-free") is the exact sentence the
standing-calls memo already ruled wrong, and it is still there. Blocks: any real program, therefore the
stdlib, therefore calls 1 through 3 jointly.

**6. The specialisation seam consuming Rust-emitted bytes.** `grep -c comptime mock/runtime-zig/src/runtime.zig`
returns 0. `vehje-runtime-gen::generate` (`lib.rs:108-112`) does not extract anything: it wraps
caller-supplied `slices` in a struct and hashes them. The doc comment says it "compiles a language definition
into the validated-data package"; the function body is `Package { slices }` plus a hash fold. The manifest it
produces (`lib.rs:73-76`) exists so "the specialised runtime can prove at its own build that it was
specialised from exactly this package", and nothing on the Zig side reads a manifest. Blocks: certification
(`positive:29`), the differential check, and the whole of call 10.

**7. Version and tier enforcement on the residual.** `Image.parse` reads the magic at offset 0, then jumps to
words 3 through 7 (`runtime.zig:209-215`). Words 1 and 2, which the Rust serializer writes as `version` and
`tier` (`vehje-runtime-abi/src/wire/serialize.rs:181-183`), are never read by anything. The two sides already
disagree: Rust writes `VERSION = 2` (`serialize.rs:44`), the Zig test builder writes 1
(`test_support.zig:167`), `ABI_VERSION` in the C entry is a third number, 1 (`entry.rs:27`), and no code
compares any of them. A `Tier::Bytecode` image would be decoded as an arena image and evaluated as garbage.
Batch 8 recorded the compatibility policy as "design work owed before the first external consumer"; the
mechanism is present on one side and ignored on the other, which is worse than owed. Blocks: shipping to any
consumer, and step 9.

**8. The CR1 representation, benched, then built.** Detailed in the next two sections. Blocked by 2 and 4.

**9. The bytecode tier.** `Tier::Bytecode` is an enum variant with a wire code
(`serialize.rs:90`) and no producer and no consumer anywhere in the tree; `negative:90` catalogues the CFG
residual types as "defined and never constructed". The sketch measurement says this tier is worth 2.6x. It is
also where tail-threaded dispatch belongs and nowhere else (`negative:51`). Blocked by 2, because a CFG
interpreter is a loop over an explicit frame and register file.

**10. The batched-column entry.** Blocked by 2 for a reason worth stating: entering W-wide is a question about
how many frame stacks exist. W lanes over one image either share one control stack and diverge (which needs
lane masks and a per-lane program counter, i.e. a vectorised frame representation) or get W independent stacks
(which multiplies every lent region by W). That choice is not makeable while there are no frames.

**11. A diagnostics channel.** Nothing exists. The entry returns `i32` and the ABI has no field for a message,
a span, or a suggestion. Blocked by 1, since diagnostics need their own lent budget by name.

**The keystone is 2.** Items 3, 4, 8, 9, 10 are all direct consequences of it; 1 cannot be finalised without
knowing the region set that 2 and 8 determine; 5, 6, 7, 11 are independent and cheap by comparison. If one
thing gets built, it is the frame stack, and the reason is not performance. It is that six separate canon
mandates are each a statement about frames.

## What is proven, what is assumed

**Proven, with the executable behind it.**

- Source text to a value inside the runtime, with no Rust and no host in the path, over a subset of Clause:
  `mock/research/sketches/202607260900_clause-in-zig`, `zig test eval.zig`, 158 tests, all passing. I ran it.
  This settles the *locus* question empirically, which is the sketch's real contribution.
- Hindley-Milner inference with generalisation, trait resolution with coherence, associated types,
  monomorphisation, nominal enums with exhaustiveness over sums, patterns with guards and alternatives,
  modules as records, and constant-stage macros, all in Zig over the same image: same run.
- Comptime specialisation collapses an operation's interpretive walk to the arithmetic: `eval.zig:45-93`, where
  `prog` is `comptime` and the dispatch `inline for`s.
- Plain `switch` beats every threaded shape on the lean flat IR: `interp-dispatch/findings.md`, 4.58 ns/op
  against 6.24 (labeled) and 6.38 (tail), holding at an 11-op set.
- A register CFG interpreter runs 1.70 ns/instr with 44% terminators, against 4.4 ns/node for the arena walk:
  `cfg-interp-throughput/findings.md`. The stated reason (L1-resident register file against a 48-96 MB node
  array) is the right reason and makes the result transferable.
- Specialisation machinery is nearly free in size: 968 bytes at 4 families to 1232 at 250,
  `runtime-binary-size/findings.md`.
- Inline scalar payload 4.4 ns/node against 10.7 ns pool-indirected, cited at `value_arena.zig:10-13`.
- **New, this dispatch.** The shipped tree-walk segfaults on a right-nested `Let` chain at depth 3,500 in the
  default build and 11,000 in ReleaseFast (ok at 3,000 and 10,000 respectively), in `eval` at
  `runtime.zig:330`.
- **New, this dispatch.** A two-operand handler clause whose body evaluates an inner handled computation
  returns a `closure` instead of the value, silently dropping the second operand, because the unwind carrier is
  one shared mutable slot and `unwind.n` is re-read as the loop bound (`runtime.zig:388-394`).

**Assumed, with nothing executable behind it.**

- *That CR1 multi-shot is cheap.* Two artifacts in the tree carry the claim and neither contains a
  continuation. `mock/benches/multishot-scaling/ms.zig` is a mixed-radix odometer calling a fixed pure function
  `a = a*31 + c`; there is no handler, no perform, no environment, no IR, and it writes into a 160 MB buffer it
  obtained from `std.heap.page_allocator`, so its "~2.5 ns/resumption" is the cost of a nested loop streaming
  memory. `mock/benches/variants/hx_multishot__multishot/src/lib.rs` is a scalar accumulate with an inner loop
  of one to four iterations and a division, against a `__single` variant that is the same accumulate without
  the inner loop; the measured 4.5x at n=16384 is the cost of doing more arithmetic. Both are labelled as
  measuring bounded multi-shot handlers. Under `positive:57`, "the reporting layer lies before the harness core
  does" and "the framework's own soul can hide inside its own benchmark", these are the failure the discipline
  exists to catch, and they are the closest thing in the tree to evidence for the sharpest original claim in
  the canon.
- *That heap-free reinstatement is possible at all.* `positive:25` says so honestly: "Intended and unproven".
  Nothing since has moved it.
- *That the Rust-to-Zig specialisation seam works.* Zero `comptime` in the runtime; `generate` composes
  caller-supplied bytes. The sketch's comptime specialisation reads a table written in Zig source
  (`eval.zig:30-41`), so what is demonstrated is that Zig comptime specialises, not that the seam carries.
- *That the batched entry is vehje's biggest lever.* The ABI benches measure Rust cells. The Zig-entry family's
  auto-generated findings are dominated by a `zig_null` variant that does nothing and is reported as
  "dominates: 77156% faster... a safe default pick for this workload shape"
  (`results/abi_zig_entry_real/*findings.md`); the same pattern poisons `abi_soa_win_real`, where `null_entry`
  at 2.60 us leads a real variant at 882 us. The number in the canon may well be right; the cells that would
  certify it for the shipped runtime do not exist, and the reporting layer in front of them is unreliable in
  exactly the way `positive:57` predicts.
- *That the no-pop environment discipline supports the census workloads.* The `1024/(k+1)` bound is arithmetic
  from `runtime.zig:176-181` and `runtime.zig:529`, never run against a real program.
- *That the C ABI as shaped can carry the design.* It lends two of seven region kinds, has no diagnostics
  channel, and its two definitions disagree on both the return contract (`entry.rs:50-56` says nothing returns
  a status, `runtime.zig:521` returns `i32`) and the version word.

## What can be proven now by a sketch, and what cannot yet

**Provable now.**

*The frame stack.* Rewrite `eval` as a loop over an explicit frame stack lent by the caller, over the wire
format that already exists, keeping the same `EvalError` set. It must show four things: constant native stack
depth at a `Let` depth of one million; a self-recursive loop running ten million iterations in a fixed
environment region; the two adversarial programs above returning `20` and refusing by name rather than
faulting; and a per-node cost within noise of the current 4.4 ns, because a defunctionalised walk that costs
2x is a different design decision and should be surfaced as one. Leeway in the resulting shape: the frame
record's width and field set, whether frames and environments share one region or take two, whether tail
position is detected syntactically in the interpreter or marked by the lowering, and whether the frame stack
is one region or a segment list. Any of those is fine; what is not negotiable is that the traversal is data.

*The family-as-data seam, end to end from a file.* A Rust binary emits the operation table to disk; `build.zig`
consumes it; the resulting binary computes `1 + 2` with the table appearing nowhere in Zig source. It must show
that changing the table changes the program's behaviour with no Zig edit, and that the manifest hash the Rust
side wrote is checked at Zig comptime and refuses a mismatched package. Leeway: the encoding, `@embedFile`
against a generated module, and whether the check is a comptime assertion or a build step.

*Environment reclamation under the reach analysis.* One million loop iterations in a fixed 1024-slot region,
refusing by name only when a binding genuinely escapes. It must show a case that correctly refuses as well as
the case that correctly reclaims, because a reclaimer with no false-negative test is a use-after-free with a
green suite. Leeway: whether the rule is a compile-stage reach proof consumed as data or a runtime liveness
check, which is a real fork with a trusted-base consequence (last section, item 3).

*The CPS/defunctionalisation candidate of the CR1 fork, alone.* Compiling the handled body to a defunctionalised
continuation is a front-end transformation and needs no engine frames. It can be built and measured today, and
it should be, because if it wins the fork the segmented candidate never has to be built.

*Version and tier refusal.* Trivial, and worth doing in the same commit as anything else that touches
`Image.parse`, with a red test per rejected combination.

**Not provable yet, and why.**

*The segmented capture-and-reinstate candidate, and therefore the fork.* Capture cost is the cost of copying a
stack segment; resume cost is the cost of copying it back. Neither quantity exists until step 2 lands, because
there is no segment. Building a stand-in would measure a `memcpy` of a size someone chose, which is exactly the
class of artifact the canon's Part F catalogues, and the two existing "multishot" benches are the evidence that
this trap is live in this repository rather than hypothetical. What I would do with op's PE2 order: run the
half that is buildable (CPS/defunctionalisation) now, sequence the segmented half immediately behind the frame
stack, and treat the frame stack as part of the CR1 work rather than as a prerequisite competing with it.

One thing worth putting on the record for whoever builds the segmented candidate, because it is a real property
of this design that nobody has written down. The environment arena never pops and bindings are immutable
(`runtime.zig:13-17`, `176-181`). That means a captured environment is *still valid after the capture*, so a
resumption does not have to copy the environment at all: it copies the frame slice and records one index. That
is a genuinely cheap capture, cheaper than Guile's, and it is a gift from the no-pop discipline that looked
like a pure cost. The bill arrives on the other side: k resumptions of a body binding b names cost k*b
environment slots, monotonically, so multi-shot has a memory cost linear in resumptions that the "static
fit-proof" of `positive:25` must account for and currently does not name. That is the honest price of CR1 and
the budget-fit design should start from it.

*The batched entry.* Blocked as described in item 10.

*The differential check* (`vehje-runtime-gen/src/lib.rs:89-96`, already FIXME'd). It compares a reference
runtime against a specialised one. There is currently one of neither.

## Unlicensed mechanisms found

Stated plainly, each with the canon text and the `file:line`.

1. **Family operations dispatched to the host.** `runtime.zig:410-434`, refusing with `NoHost` at
   `runtime.zig:414`. Against standing call 2 ("it should *NOT* have to give the simple things like fucking
   arithmetics"). The in-source defence at `runtime.zig:411-413` was already identified as wrong by
   `202607260100_op-standing-design-calls.md:157-163` and is still in the tree unmarked.

2. **A process-global mutable session.** `runtime.zig:477`, `var the_session: Session`. `vehje_runtime_new`
   overwrites the single global and hands back a pointer to it (`runtime.zig:479-483`), so two handles alias
   and a second `new` silently invalidates the first. The comment at `runtime.zig:474-476` defends it as "A
   single session is enough: the handle is per-call in every current embedding". The canon's identity is
   "small, fast, embeddable" and the measured design centre is a batched entry over a column of records; a
   static mutable is neither host-lent nor caller-owned and nothing licenses it.

3. **Stack-allocated arenas presented as caller-lent.** `runtime.zig:529-535` and `runtime.zig:542`,
   roughly 96 KiB of locals in the C entry point; `host.zig:127`, a further 4 KiB. Against `positive:45`,
   "host-owned memory via caller-lent arenas and host-lent budgets". The module doc at `runtime.zig:15-17`
   asserts the arenas are caller-lent, which the code contradicts twenty lines from the bottom of the same
   file. This is `negative:86`'s named root cause exactly: "surface existence standing in for mandate
   satisfaction".

4. **Compile-time constants documented as lent bounds.** `host.zig:160-162`: "The widest image a handler may
   return, and the widest compound in it. Both are lent bounds like every other here". They are `const`.
   `runtime.zig:157-161` makes the same claim of `ARG_CAP`. Same canon text as 3.

5. **A wire field with no consumer and no marker.** The clause record's `resume` word is documented in two
   places (`runtime.zig:47`, `runtime.zig:242`) and read by nothing. `grep -rn "FIXME\|TODO"
   mock/runtime-zig/src/*.zig` returns **zero**. The workspace's `mark-placeholders-fixme.md` requires a
   greppable `// FIXME:` at every placeholder, and `negative:90` says of this class "each is owed a FIXME and
   a red test". The single artifact that actually ships carries not one.

6. **An unbounded, unnamed, process-terminating failure mode.** `runtime.zig:278` and every recursive call
   under it. Against `negative:91`'s rule and against the engine's own contract that a bound is a named
   refusal. Reproduced: SIGSEGV at `Let` depth 3,500 in the default build.

7. **A silent-correctness bug in the handler arm.** `runtime.zig:388-394`, the shared carrier. Reproduced: a
   two-operand clause returns a closure and drops an operand. This is the same category as the two bugs
   `negative:91` catalogues, found in the same way, in the shipped runtime rather than the compile side, and
   it is not on any list.

8. **Version and tier written and never checked.** `runtime.zig:209-215` against `serialize.rs:181-183`. Three
   version constants with two distinct values (`serialize.rs:44` is 2, `entry.rs:27` is 1,
   `test_support.zig:167` is 1) and no comparison anywhere.

9. **A bench outside the harness that allocates 160 MB from the heap.** `mock/benches/multishot-scaling/ms.zig`,
   `std.heap.page_allocator` allocating 20,000,000 `i64`. Against standing call 5 ("Even for zig-authored
   benches, you *NEED* to plug them into the harness") and against the workspace's
   `bench-in-bench-harness-never-sketches.md`. It has its own bespoke CSV and findings file outside
   `results/`.

10. **Two bench families named for a mechanism they do not contain.** `multishot-scaling/findings.md` reports
    "the bounded multi-shot handler enumerates its choice space at ~2-2.9 ns per resumption" for a program with
    no handler in it; `bench.toml:1022-1023` titles `hx_multishot` "Bounded multi-shot enumeration vs
    single-shot" over two variants that differ by an inner arithmetic loop. Against `positive:57`, "audit your
    own benches adversarially", and against `negative:65`'s own precedent, the inverted native cell tags. These
    two are cited nowhere I found as evidence for CR1, which is the only good news in this item; if they ever
    are, the citation is worthless.

11. **The findings generator promotes no-op variants to winners.** `results/abi_zig_entry_real/*findings.md`
    reports a null entry as "dominates: 77156% faster than the next best" and adds "A dominant, well-separated
    winner is a safe default pick for this workload shape"; `results/abi_soa_win_real` does the same. This is a
    harness defect rather than a design one, and it is the one thing here that gets cheaper the sooner it is
    fixed, because it silently degrades every future reading of the ABI family, which is the family the canon's
    biggest measured claim rests on.

Outside my lens but I was asked to report it: **the sketch is two artifacts in one file tree.** `clause.zig`
is the Clause surface syntax and `eval.zig` bakes Clause's operation table into the evaluator
(`eval.zig:30-41`, `cl.OP_MAKE_REC` and friends switched on inline at `eval.zig:474-540`). The repo's own
instruction is "Do not inline any consumer's surface syntax, family, or target into this repo", and call 2's
note says "The first-party Clause runtime is a different artifact from the framework". Promoting the sketch
as-is would inline a consumer into the framework repo. The seam between the language-agnostic core and the
Clause consumer has never been drawn, and drawing it is prerequisite work to promoting any of that code,
which makes it larger than it looks.

## Where the canon is genuinely silent

Four calls I will not make. Each with its bill on both sides and no preference of mine.

**1. Comptime-specialised per language, or one generic core reading a table at load.** `positive:21` and
`positive:45` say the engine is comptime-specialised to the Rust-emitted data at our build. Call 9 licenses
"a very generic, not-specified shared runtime... as long as it does not expect any kind of language" that is
hand-written and shipped rather than generated per language. Both are ratified and they are different
artifacts. Comptime specialisation buys the measured near-free dispatch skeleton (968 bytes at 4 families,
`runtime-binary-size/findings.md`) and inlines each operation's program away entirely (`eval.zig:45-93`); it
costs one runtime binary per language definition, so a host embedding two languages links two engines, and it
makes every language-definition edit a full rebuild of the engine. The generic core is one binary that works
with anything, which is what call 8's "N in, M out" implies if N languages are ever live in one process at run
time; it costs a load-time decode, an indirect dispatch per operation, and the loss of the specialisation that
the canon calls the first Futamura projection. Shipping both is coherent and is what I would expect a mature
system to end up with, and it doubles the surface that has to stay in agreement, which is the seam
`positive:31` already names as the trust leak. The canon does not say which, or whether both.

**2. Where the Clause front end's source lives.** It must *run* in the runtime artifact; `negative:38` is
unambiguous. `positive:45` names two artifacts and neither of them is "the Clause consumer's runtime", while
call 2 says the Clause runtime "is a different artifact from the framework" without saying where its source
sits or whether the framework ships front-end scaffolding. In-repo buys one build, one test suite, and the
ability to move today; it makes the framework hold a consumer's surface syntax, which the repo instruction
forbids and which is how a framework stops being generic. Out-of-repo forces the core-versus-consumer seam to
be real rather than aspirational, which is worth a great deal at exactly this moment because the seam has never
been drawn; it costs a cross-repo build for every change during the period when both sides are moving fastest.
This one sizes the schedule more than any other item here.

**3. Whether the environment arena may pop, and what that does to the trusted base.** Never popping is what
makes closures free, captured environments valid after an unwind, and (as I noted above) a segmented
continuation cheap to reinstate. It also caps every loop at roughly `1024/(k+1)` iterations, which no real
program survives. Reclaiming needs the reach analysis the check already computes. The part that makes this a
designer call rather than an optimisation: `positive:29` enumerates the trusted base by name ("the Zig
compiler, rustc, the C ABI marshalling, the cross-artifact byte-image hash, the still-open Rust-to-Zig emitter
seam, the caller-supplied sets, and the load verifier's decode"), and "a compile-stage lease analysis is
trusted for run-time memory safety" is not on that list. Adding it is an enlargement of the trusted rim, which
`positive:29` treats as the thing that must be stated rather than scattered. The alternative, a runtime
liveness check, keeps the rim as-is and pays per binding.

**4. Whether a resumption may cross the C ABI.** Closures are refused at the boundary today
(`runtime.zig:82-84`, `host.zig:86`) and the host contract is "the runtime retains nothing past the call"
(`host.zig:49-51`). `positive:23`'s three discharges include "runtime resumable continuation" without saying
whether the host may be the one that resumes. If it may, the ABI needs a continuation handle and a re-entry
point, the retains-nothing contract breaks, and the lifetime of a lent region stops being the call; that is
what buys async host effects, which is the thing every embedder asks for second. If it may not, host-serviced
operations are abort-only forever and the resumable discharge is purely internal, which keeps the boundary as
simple as it currently is. The choice constrains the ABI, and the ABI is the interface batch 8 recorded as
outliving every mechanism, so it wants deciding before the version word is spent rather than after.

Until next time, happy hacking.
