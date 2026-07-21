# Certified generation: building on audit 1 (audit 2, Giesen)

**Date:** 2026-07-20
**Subject:** `202607201627_topic.compiling-the-proof-into-generated-code.md`, read after the four prior topics
in chronological order, then `01_carmack.md` as the input this audit builds on.
**Method:** read the five synthesis docs and the five topics in order, then the first audit; independently
verified the Zig claims that carry weight in what follows (`@Type` union reification and its production use,
lazy semantic analysis and the state of the std forcing helpers under 0.16, panic behaviour at the C ABI); and
verified the region-inference history the lease axis leans on (Tofte-Talpin, the MLKit
collector-retrofit papers, Cyclone) plus the one modern system that ships the generation-counted-handle
discipline the first audit asked about (Vale). Where I reason about the lease schema and the ref-type
tractability, that is first-principles work over this round's own topics, and I say so.

## Verdict in one paragraph

The resolution holds, the comptime-specialiser reframe is the right architecture, and audit 1's corrections
are almost all real: I confirm the certification accounting, the lazy-analysis gate, the allocator-signature
demotion, and the panic-across-the-C-ABI finding against current Zig behaviour, and every one of them should
land in the design. I push back in two places. First, audit 1's split of the lease temporal half into
"dev-time content is certified generation, runtime scripts are a hand-authored algorithm" is a false
dichotomy: under this round's own three-codegens topic the Rust side never sees content at all, so both halves
run the same inference implementation, once at comptime over bundled content and once at load over arriving
scripts, and the right design response is to make that one implementation deliberately singular, proven once,
folded by the compiler for the early case, which is the same one-source-two-times stratification the mask
check already uses. Second, the MLKit cautionary tale is weaker here than audit 1 (and the parent audit)
credits, because region inference's classic failure mode, over-promotion, is an unbounded leak only in
long-running processes; in a per-script, host-lent-budget, torn-down-at-end execution it is a bounded peak
memory cost with an existing overflow path, which changes the risk calculus for debt 2. On the three questions
audit 1 left me: the distinct-ref-types trick is even narrower than stated (it separates region kinds, never
region instances, and the default per-body lease makes every interesting lease boundary a same-kind instance
boundary, invisible to types), but the monomorphisation fear dissolves once the typing moves onto arena
handles instead of refs; generation-counted handles are worth building, but strictly as the comptime-gated
verification harness for the lease proof, never the shipped mechanism, with Vale as measured prior art for
both the cost and the elision argument; and the lease schema can in fact be made small enough to be
table-driven, and I lay out a concrete candidate below (a depth-ladder lease with one link-or-consume bit per
operand position) whose metatheorem is tractable and whose consumer-facing surface is sound by construction
under a monotonicity rule. The design is close; what it needs now is the schema pinned and the one-family
experiment widened to include the lease harness and the negative corpus.

## Audit of the first expert

**Holds, independently confirmed.**

- The certification-accounting correction (finding 1) is right and is the most important thing in the audit.
  Under the comptime reframe rustc types data, not a generator; the typed-staged-metaprogramming citations
  (LMS, typed Template Haskell, Terra) attach to the rejected Rust-emits-Zig-source path. The
  CompCert-pretty-printer analogy for the data bridge is apt: the unverified step is small and exactly where
  silent divergence would live. I go further on the mitigation below (the bridge should not be a printer at
  all).
- Lazy analysis gating the second certification (finding 2) is confirmed and current. Zig semantically
  analyses only what is referenced; generic and comptime-parametrised code is checked at instantiation. One
  sharpening: the std helper audit 1 gestures at (`refAllDeclsRecursive`) is itself churning; the 0.16 cycle
  moved or banished it out of `std.testing`, per the ziggit thread on exactly that question. The forcing root
  must be our own comptime walk over the generated and engine decls, owned by us, not a std convenience that
  version-churns. This also reinforces the pinning point: Zig's std surface has churned harder across
  0.14, 0.15, 0.16 (the 0.15 writer overhaul, the 0.16 `std.Io` and type-resolution redesigns) than rustc
  nightly churns for us; the exact-version pin is not optional and 0.16.0 is the natural pin candidate as of
  this writing.
- The safety-check demotion (finding 4) is confirmed: checks exist in Debug and ReleaseSafe only, a failed
  check panics, and a Zig panic does not unwind across the C ABI; the process dies. For an embeddable runtime
  that is a host denial of service. The untrusted path must be error unions end to end, and the panic-free
  invariant belongs next to the workspace's other embeddability correctness rules, as audit 1 says.
- Allocator-free signatures as convention-not-guarantee (finding 3): confirmed. Restated enforcement
  (freestanding-target lazy failure, source gate, explicit fixed-buffer arenas) is the honest form.
- Parse-don't-validate as the correct reading of the load-time rejection (finding 6): agreed without
  reservation, and the phrasing fix ("the runtime check is derived and total, not absent") should be adopted
  verbatim into the topic.

**Thin.**

- Finding 1 stops at "promote the comptime re-checks to a systematic rule." That is translation validation
  over an ad-hoc encoding, which is the second-best answer. The best answer removes the ad-hoc encoding: see
  the wire-format unification move below. Audit 1 names the disease precisely and prescribes a bandage.
- The exhaustive-switch discussion certifies totality of dispatch but not correctness of the tag-to-handler
  mapping. A switch can be exhaustive and still route family X's node through family Y's semantics if the
  wiring is positional and the positions drift. The fix is structural: name-keyed comptime dispatch
  (`inline else` over the tag, `@field` on the handler namespace by tag name), so a mismatch is a missing-decl
  compile error rather than a silent transposition. Cheap to state now, painful to discover as a
  wrong-semantics bug later.
- The lease-schema question (audit 1's open question 2) is posed but not attempted. It is answerable now, from
  the material already in this round, and answering it collapses most of debt 3. Below.

**Wrong, or at least mis-cut.**

- The two-category split in finding 5: "for dev-time-known content, the region open/close nesting is computed
  under the Rust-side proof and baked into emitted structure; that genuinely is certified generation. For
  runtime-arriving scripts, the per-script region inference runs as engine code, which is dead end 1's shape."
  The first half does not survive contact with the three-codegens topic. The Rust side compiles the language,
  never any content; there is no Rust-side act that computes a particular program's region nesting. Whatever
  dev-time-known content exists (first-party scripts bundled by a language author, test corpora baked into the
  binary) is lowered by the same composed-runtime machinery, at our build time via comptime instead of at load.
  So both halves of the temporal story run the same inference; the difference is binding time, not provenance.
  That is not a demotion of the first half, it is a promotion of the whole: the design should make the
  inference a single implementation, written once in the engine, proven once against the schema metatheorem,
  executed by the Zig compiler's comptime interpreter for bundled content and by the shipped binary at load
  for arriving scripts. One source, stratified by the compiler, exactly like the mask check. Audit 1's framing
  would have led to two implementations with a fidelity seam between them, which is the very failure the
  primary topic exists to kill.
- The MLKit weight in debt 2 (inherited from the parent audit and reaffirmed here) needs recalibrating. The
  history is real: pure region inference retained too much, and the MLKit added a copying collector
  (Hallenberg, Elsman, Tofte's combined scheme) because promotion into long-lived regions accumulated garbage
  the region discipline could not free. But the accumulation is a function of process lifetime. MLKit ran
  whole programs; a promoted value lived until a region that might never die. Vehje's runtime executes one
  script against a host-lent output region and tears everything down at body end; the streaming path bounds
  residency for outputs beyond the budget. In that model the worst consequence of over-promotion is a higher
  peak inside a budget the host already set, observable and reportable, not an unbounded leak. Cyclone's
  hybrid (regions plus uniques plus RC plus a collected heap) similarly answered the needs of general systems
  programming with arbitrary lifetimes, which is not this execution model. Debt 2 does not vanish (the Lua
  mutable-alias question is untouched and I scope it below), but "the lineage is the evidence against the
  axis" overstates once the execution model is priced in.

## My own findings (what the panel has not yet said)

**1. The distinct-ref-types trick separates region kinds, and every interesting lease boundary is a
same-kind instance boundary.** This is the sharpest limit and neither the topic nor audit 1 states it. A
comptime-generated ref type is a type; types exist per region kind (a static, small vocabulary), never per
region instance (a dynamic, unbounded population). The lease topic's default lease is per evaluation body, and
bodies nest recursively: every `Let` body inside a `Let` body is another instance of the same kind. A ref
escaping an inner body into an outer body, the single dominant error class the lease axis exists to catch, is
a same-kind, cross-instance flow. No Zig type distinguishes the two instances, so the trick cannot see the
error at all. What the trick genuinely separates is arena classes: the residual IR arena, the engine's scratch
regions, the output value-arena, perhaps a persistent host-session region. Call it three or four nominal
handle types. That is worth having (it kills a real class of engine plumbing bugs, a scratch index passed
where an output index belongs), but it is engine hygiene, not lease enforcement. The topic's caveat ("the one
least certain to carry its weight") was correctly aimed; the answer is now concrete: re-scope the mechanism to
arena-class handle types and stop associating it with the lease axis.

**2. The monomorphisation question dissolves once the typing moves off the refs and onto the arena handles.**
Audit 1 asked how far the region parameter can be threaded through the evaluator before it monomorphises
combinatorially. The answer is: do not thread it. Two placements were conflated in the proposal. Placing
region types on node fields multiplies node types by region kind per ref field and forces the evaluator
generic over every combination; that is the combinatorial path, and it is also impossible in principle for the
runtime-script path, because a script's region assignment is computed at load, and runtime data cannot
instantiate comptime types. Placing region types on the arena handles (the allocator-and-accessor surface)
costs one instantiation per arena class of the dozen or so allocation and read helpers, order tens of small
functions total, and leaves the evaluator monomorphic over plain `u32` refs. This is exactly the discipline
the IR synthesis records from cranelift's entity vocabulary: indices stay plain, containers are typed. It also
preserves the transfer topic's core property that the in-memory bytes and the wire bytes are the same, since
refs stay bare indices in both. Comptime memoisation (same args, same type) guarantees the handle types are
nominal singletons, so the API-level separation is real. Concrete answer to audit 1's question 5a: the trick
is tractable precisely when confined to handles, and intractable (and pointless) anywhere else.

**3. Generation-counted region handles: yes, and strictly as the verification harness.** Audit 1's question 5b
asked whether any Zig-expressible temporal discipline exists short of the schema metatheorem. It exists, it is
well-trodden, and its correct role here is narrow. The mechanism is the generational index: each region
instance carries a generation counter, a checked ref is (index, generation), deref compares generations, reset
bumps the counter, so a stale ref fails deterministically instead of reading reused memory. Prior art is deep:
slotmap and generational-arena in Rust, the ECS handle discipline, and above all Vale, whose generational
references are this exact mechanism as a language's memory-safety story, measured at roughly 2 to 11 percent
overhead in their benchmarks. Two facts decide the role. First, the lease topic already rejected per-node
reference counts because they "reintroduce exactly the dynamic per-node tracking the static-lease design
exists to remove"; a shipped per-deref generation check is the same category at lower cost, so shipping it
would concede the axis. Second, Vale's own regions work is the elision argument in production form: while a
region is immutable, generation checks are provably unnecessary and removed. Vehje's produced values are
immutable by design, and the lease proof claims no stale deref exists. So: build the generation-checked ref as
a comptime-gated debug layer (`if (comptime lease_checks)`), on in Debug and ReleaseSafe and CI, compiled to
nothing in the shipped ReleaseFast binary. It is then the executable oracle of the lease metatheorem: the
proof claims the checks can never fire; CI runs the whole corpus with them armed; any trap is a soundness bug
in the inference or a wrong per-family declaration (finding 4 below), caught mechanically. This is the
catalogue-the-edge-case-as-a-test discipline applied to a proof, and it costs the shipped path zero.

**4. The lease schema can be small, and here is a concrete candidate.** Audit 1's question 5c asked whether
the schema can be made small enough that the inference is table-driven and the metatheorem tractable. Working
from this round's own structure rather than from Tofte-Talpin generality, I claim yes, with this shape:

- **Leases form a ladder, not a lattice.** The lease topic's default region is the evaluation body, and bodies
  nest strictly on a stack. So a lease is a lexical depth: lease(v) is the shallowest body depth at which v is
  used, and "escapes" means "used at a depth shallower than its binder." There is no general region
  polymorphism, no region subtyping graph, just integer depth comparison on the binder stack. This is
  de Bruijn-level reasoning, decades old and fully understood.
- **The per-family declaration is one bit per operand position: link or consume.** A node kind either retains
  a reference to an operand in its result (link: constructors, record builders, closures capturing free
  variables) or fully consumes it into a fresh scalar or fresh structure (consume: arithmetic, comparisons,
  folds). The inference is then a single propagation: a value's lease is the minimum depth over all linking
  uses, transitively. Eleven core forms is an eleven-row table; a family extension declares its bits exactly
  the way it already declares effect classification. This is deliberately the same shape as the effect axis,
  and it inherits the same discipline the purity synthesis records for LMS summaries: a wrong declaration is a
  silent soundness bug, so the conservative direction (declare link when unsure) must be the documented
  default, and the generation-check harness from finding 3 is the mechanical test that catches a wrong bit
  (fuzz the family's semantics under the armed build; a stale-deref trap indicts the declaration).
- **The metatheorem is tractable to state.** "If every family's link bits are conservative, then depth-min
  inference assigns every value a lease no shallower than any use, hence no reference into a reset region is
  ever dereferenced." Proof by induction over the structural evaluation of the eleven forms plus the family
  obligation as a lemma per family. This is a paper proof of one fixed algorithm over one fixed rule shape,
  done once, exactly the branch debt 3 hoped existed. The expressiveness bound, stated honestly: leases live
  on the lexical body stack only; any lifetime that is dynamic (a value stored into a mutable structure, a
  lifetime decided by runtime control flow) is not expressible and promotes.
- **Promotion is the sound fallback, which changes the failure stance.** A value the inference cannot place
  dies at the outermost lease: the output region, append-only for the execution, torn down at body end. That
  is whole-value mode, already the transfer topic's degenerate case, already correct. So inference failure
  need not be a hard error for soundness; it costs memory frugality inside a host-set budget, not safety. The
  lease topic's failure-is-a-compile-error stance survives as an ergonomics-and-frugality policy (surface the
  escape to the author, per strict-by-design), but the design should record that the safety of the axis does
  not depend on that policy, because that is what makes the Lua case scopeable instead of fatal: mutable
  aliased tables degrade precision (every store links, most things promote), never soundness. The MLKit
  recalibration above is what makes this an acceptable degradation here when it was not acceptable there.
- **Explicit annotations are monotone, therefore consumer-safe.** The consumer-surface hook the lease topic
  left open gets one rule: an explicit lease may only lengthen the inferred lease, never shorten it
  (effective lease = max(inferred, declared)). Lengthening is always sound (the value merely lives longer), so
  the entire consumer-facing annotation surface is incapable of introducing unsoundness. The only
  soundness-bearing consumer input in the whole axis is the link/consume bits, one bit per operand, testable
  by the harness. That is a small, auditable trusted surface, and it should be stated as such in the design.

**5. The wire order already makes the inference linear.** A connection the round has not noticed: the transfer
topic fixed depth-first, children-before-parents emission with no back-patching, so every intra-arena link
points backward and every subtree occupies a contiguous index interval ending at its root. That makes the
arriving script's flat arena already a post-order traversal in memory. The load-time work (bounds validation
from the transfer topic, family and effect mask accumulation, and the depth-ladder lease inference) all become
linear index-order scans over a contiguous buffer, no pointer-chasing, no recursion, no visitor machinery; the
lease pass wants the body-interval structure, which binder nodes carry for free (a binder's body is the
contiguous interval between its body start and itself), and DAG sharing falls out correctly because a
backward ref across a closed interval is exactly an escape to the common enclosing body, which is exactly the
lease the shared value needs. Whether it fuses into one forward pass or one forward plus one backward pass is
an implementation detail for the experiment; the point is the complexity class and the cache behaviour are
already paid for by a decision the round made for a different reason. The design should claim this on purpose
rather than get it by accident.

## Building beyond: concrete design moves

**Move 1: the bridge is not a printer, it is the wire format, decoded at comptime.** Audit 1's finding 1 left
the Rust-to-Zig data bridge as an unverified printer plus systematic comptime re-checks. Remove the printer.
The language definition data (family table, effect masks, lease rule bits, wire layout) should cross as a
binary blob in the same wire-format family the runtime already speaks for residuals, embedded via
`@embedFile`, and decoded at comptime by the same decoder the shipped runtime uses at load. Then there is no
second, ad-hoc Zig-syntax encoding to trust: the encoder is `encode.rs`, already a named deliverable with its
own tests and its own untrusted-path validator; the decoder is engine code, certified by the Zig compiler and
exercised at two binding times. The translation-validation checks audit 1 wants still run, but now over
decoded structures, and the negative corpus becomes one corpus with two firing surfaces: a corrupted language
blob must `@compileError` at our build through the comptime decode, and the same corrupted bytes must be
rejected at load by the same code path. One format, one decoder, two times: the same stratification argument
the primary topic makes for the inclusion check, applied to the bridge itself. Cost to watch: comptime
interpretation of a decoder over a real language table will be slow and will need a stated
`@setEvalBranchQuota` policy (set at the top of the governing comptime stack, per the synthesis's ratchet
note); if comptime decode time turns out unacceptable, the fallback is generated consts plus the re-check
rule, which is audit 1's position, and the choice is benchable. Either way the wire schema stays the single
source of truth.

**6. One certified inference, two binding times (the corrected lease story).** Pulling findings 3, 4, 5 and
the audit-of-audit correction together, the lease axis's honest architecture is: a fixed depth-ladder schema
with per-family link bits (the only consumer soundness surface); one inference implementation in the engine,
proven once on paper against the schema; executed at comptime for bundled content and at load for arriving
scripts, so dev-time content gets its lease errors at our build with zero shipped cost and runtime scripts
get a linear-scan load check; generation-checked refs as the armed oracle in checked builds; and arena-class
handle types as engine hygiene. Nothing in that list is dead end 1, because nothing in it is an unproven
hand-authored checker: the algorithm is proven once as a metatheorem, the declarations are one-bit and
mechanically testable, and the folding for the early path is the compiler's, not a second implementation.
This is what "certified" can honestly mean for the temporal half, and it is stronger than audit 1's split
suggested was available.

**Move 2: widen the one-family experiment into the shape that can actually fail.** Combining audit 1's
negative-test demand with the findings above, the experiment that earns the design is: one family, two node
kinds (one linking constructor, one consuming op), comptime-reified tag union and name-keyed dispatch, the
wire-blob bridge with comptime decode, the depth-ladder inference as one engine function run at both binding
times, generation-checked refs armed, and a negative corpus with four members: a mask missing a family the
dispatch covers (must `@compileError`), a corrupted wire blob (must fail both surfaces), a script escaping an
inner-body value (must be caught by inference at load, and by the armed generation check if inference is
deliberately disabled), and a family with a deliberately wrong consume bit (must trap under the armed harness,
demonstrating the declaration-testing loop). Plus the forced-instantiation root as our own comptime walk. An
experiment with those arms tests every load-bearing claim this panel has made; the original scope tested
almost none of them.

**Move 3: adopt the mechanism re-scopes as topic amendments.** Concretely, the primary topic's mechanism list
should be amended: distinct ref types re-scoped to arena-class handles and detached from the lease axis;
allocator-free signatures restated as freestanding-target plus source-gate enforcement; built-in safety checks
demoted to checked-build backstop with the panic-free error-union invariant stated for every host-drivable
path; the exhaustive switch supplemented with name-keyed dispatch; the comptime consistency checks superseded
by (or, if move 1's comptime-decode cost disappoints, retained as) the wire-bridge validation; and the
co-emitted tests joined by the owned forcing root, with the Zig version pinned exactly (0.16.0 as of this
writing) in the same workspace rule that pins rustc.

## Open questions for op and the next experts

1. **Accept the depth-ladder lease schema as the debt 2/3 resolution?** It picks debt 3's fixed-pre-proven
   branch with a named expressiveness bound (lexical-stack leases only, dynamic lifetimes promote), makes
   promotion the sound fallback inside the host budget, keeps failure-is-an-error as policy rather than
   soundness, and scopes Lua to precision loss. If accepted, the metatheorem write-up and the link-bit
   vocabulary become named deliverables of the lease round. If rejected, the rejection should name which
   inexpressible lifetime actually matters for the census consumers, because that is the only grounds on which
   a richer (and less provable) schema earns its cost.
2. **Wire-blob bridge versus generated consts.** Move 1 removes the printer from the trusted base at the cost
   of comptime decode time. The next expert with Zig build-time experience should estimate or measure whether
   comptime-decoding a realistic language table (say tens of families, hundreds of node kinds) stays inside
   sane build times under the quota, or whether the fallback is the default and the blob is only the negative-
   test surface.
3. **Generation-check scope.** I have scoped the armed build to CI and checked builds. Is there a consumer
   tier that should be offered ReleaseSafe-with-armed-leases as a shipping option (a host that prefers a
   deterministic trap report over maximum speed while integrating)? That is a distribution question, not a
   design question, but it should be decided rather than defaulted.
4. **For the next expert specifically:** the load-time pass fusion (finding 5) claims bounds validation, mask
   accumulation, and lease inference ride one or two linear scans over the post-order arena. Probe the depth
   assignment against the chunked streaming case from the transfer topic, where a parent references
   (chunk, index) pairs across self-contained mini-arenas; the interval argument holds within a chunk, and the
   cross-chunk refs need the lease story stated (my expectation: a cross-chunk ref is by construction a
   consumed-or-output-promoted value, because the chunk boundary is a whole-subtree boundary, but that is an
   expectation, not a proof).
5. **The macro-expander debt stands.** Nothing in this audit or the last touches per-script macro expansion
   producing new IR under no-alloc at run time. It remains the hardest unpaid instance from the parent audit,
   it is not a lease question, and it should be the subject of its own experiment before the doc phase, per
   the debt topic's own list.

## Bottom line

Adopt audit 1's corrections in full, with two amendments: the lease temporal half is one proven inference at
two binding times, not a certified half and a hand-authored half; and the MLKit history argues for care, not
against the axis, once the per-script bounded execution model is priced in. The three questions audit 1 posed
are all answerable now, and answered above: ref types per region kind are engine hygiene confined to arena
handles (and tractable exactly there); generation-counted handles are the armed oracle for the proof, worth
building and never shipping hot; and the schema can be a depth ladder with one link bit per operand, small
enough to table-drive, small enough to prove once, with a consumer surface that is sound by monotonicity.
The certified-generation resolution survives both audits intact, and its honest boundary is now drawn
precisely: shapes and masks are certified by construction, the bridge is certified by reusing the wire codec
at comptime, the temporal half is certified by a once-proven algorithm folded early where content is known,
and the only trusted human inputs left standing are eleven rows of link bits and their family-extension
analogues, each of which the armed harness can mechanically indict. Build the widened experiment before
writing the doc CL; it is small, and every claim above is falsifiable inside it.
