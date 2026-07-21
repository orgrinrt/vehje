# Regions, capabilities, and substructural memory: compacted, one section per thread

**Date:** 2026-07-20
**What this is:** a compaction of the region, capability, and substructural memory theory the certified-generation
panel continuation banked, across `202607201854_certgen_panel_continuation/02_settlements-and-next-architecture.md`,
`.../03_core-nodes-typesystem-and-zig-bridge.md`, and `.../05_systems-codegen-frontier-holes-and-novel-compositions.md`.
One section per thread, whole-picture prose with a bearing clause, dropping the exact constructions. For
specifics, read the named sources. This sits beside the five `202607201618_synth_*` domain docs and the two
adjacent-theory syntheses as banked reading, and it extends the region material already in
`202607201854_synth_adjacent-theory-for-certified-generation.md` (reachability types, the Tofte-Talpin lineage,
the region-leak hybrids) rather than repeating it. It carries only the theory, not the continuation's design
opinions. The common ground: how memory ownership and reference validity are tracked statically, how a producer
streams a large structure while keeping a bounded frontier, and how authority over a boundary is held as a
capability rather than reached for ambiently.

## The region inference algorithm: the algorithmic complement to the region calculus

Tofte and Birkedal's "A Region Inference Algorithm" (TOPLAS 20(4), 1998) is the concrete algorithm that assigns
region annotations to a program automatically, the algorithmic complement to the Tofte-Talpin region calculus
(whose soundness the adjacent-theory synth already carries). It infers where each value is allocated and when
each region is created and destroyed, on a stack discipline, by a constraint-based analysis over the program's
structure.

Bearing on vehje: the lease inference is a specialisation of region inference to the strictly-LIFO, lexically
nested fragment, where the general algorithm degenerates to depth-min propagation of a single lease depth over
the post-order structure. It is the algorithm the lease pass instantiates, and its LIFO degeneration is what
makes the pass a single bottom-up traversal rather than the general constraint solve.

## Closure conversion into regions: the higher-order capture case

Aiken, Fähndrich, and Levien's "Better Static Memory Management: Improving Region-Based Analysis of
Higher-Order Languages" (PLDI 1995), read with the closure-conversion treatment in the Tofte-Talpin lineage, is
the standard treatment of the hard case of region inference: a closure captures values and can outlive the scope
that built them, so the captured values must be placed in a region that dominates the closure's own lifetime, or
copied into the closure's region. It is the mechanism by which higher-order capture is made sound under a region
discipline rather than defeating it.

Bearing on vehje: closure capture is the load-bearing proof obligation for the lease axis, because the
immutable-value narrowing that makes the analysis cheap does not remove closure escape (a closure is immutable
yet still captures and outlives). Closure conversion into regions is the treatment of exactly that case, and it
is the concrete shape of the debt the paradigm audit left open around the lambda form.

## Substructural and linear typing: the affine reading of link and consume

Girard's "Linear Logic" (Theoretical Computer Science 50, 1987) and Wadler's "Linear Types Can Change the
World!" (1990) are the foundations of substructural typing, where a value's use is restricted: linear (used
exactly once), affine (used at most once), relevant (used at least once), against the unrestricted default that
freely duplicates and discards. The one-bit-per-use distinction between "this reference is retained" and "this
reference is spent" is precisely an affine-versus-unrestricted distinction.

Bearing on vehje: the lease consume bit is an affine (at-most-once) use of an operand's lease and the link bit
an unrestricted or borrowed use, so the lease schema is simultaneously a region discipline and a substructural
one. Naming the substructural home is what identifies where to reach when immutability is relaxed for a mutable
consumer: to affine ownership plus regions, the Cyclone hybrid read as a substructural extension, rather than to
a collector.

## One-shot and linear continuations: the frontier bound as continuation linearity

Bruggeman, Waddell, and Dybvig's "Representing Control in the Presence of One-Shot Continuations" (PLDI 1996)
and Berdine, O'Hearn, Reddy, and Thielecke's "Linear Continuation-Passing" (HOSC 15(2-3), 2002) study
continuations that are invoked at most once, and the cheaper representations and stronger reasoning that
one-shot use permits. A producer that emits a large structure incrementally while retaining only a bounded
working set is a coroutine whose suspended continuation is used exactly once, and the frontier-boundedness is a
statement of that continuation's linearity: no earlier suspended frame is retained because none is re-entered.

Bearing on vehje: the streaming spine's central efficiency claim (a bounded frontier over a chunked, post-order
emission) is the linearity of the producer's continuation. It is the theory that collapses the two separate
proof obligations the arc carries for streaming (the cross-chunk boundary property and the oversized-subtree
continuation property) into one linearity statement, proved over both the whole-subtree boundary and the
intra-subtree continuation chain.

## Separation logic: the ownership invariant as a proof vehicle

Reynolds's "Separation Logic: A Logic for Shared Mutable Data Structures" (LICS 2002), with the O'Hearn and
collaborators lineage, reasons about heap-manipulating programs through a separating conjunction that asserts two
assertions hold over disjoint portions of the heap, so ownership and non-aliasing become first-class in the
logic. "Reachability respects the lease order" is an ownership and separation statement, and separation logic is
the alternative proof vehicle for it should the region-calculus route strain.

Bearing on vehje: separation logic is the recorded fallback proof vehicle for the lease metatheorem, noted but
not adopted; the region calculus stays the chosen route (its LIFO fragment gives the weekend-of-induction
soundness proof), and separation logic is the heavier machinery to reach for only if the region route cannot
carry a needed case.

## The object-capability model and iteratees: authority as a lent capability

Dennis and Van Horn's "Programming Semantics for Multiprogrammed Computations" (CACM 1966) and Miller's "Robust
Composition" (PhD thesis, 2006) are the foundations of the object-capability model, in which the authority to
perform an operation is an unforgeable reference a component holds, granted explicitly, rather than an ambient
power any component can invoke. A component can do exactly what its held capabilities permit and no more, so
confinement is structural. Kiselyov's "Iteratees" (FLOPS 2012) is the functional formalisation of a pull-based
bounded-buffer consumer that drives its producer, where the consumer lends buffer space and the producer fills
it, and backpressure falls out because a full buffer suspends the producer.

Bearing on vehje: the object-capability model is the frame under which the runtime owns no I/O policy and the
host lends a sink capability (in-process buffer, pipe, or spill backend), so "the runtime can only touch what its
effect set permits" is a capability-confinement property rather than a convention. Iteratees are the formal
account of the reserve-and-commit pull sink and why its backpressure is inherent rather than added; the operating
system pipe buffer is the iteratee's bounded buffer for the standalone-executable path.

## Reference capabilities, revocable capabilities, and CHERI: the mutable and hardware-backed extensions

Three threads extend the capability and region readings toward the mutable case and toward hardware. "Reference
Capabilities for Flexible Memory Management" (2023) gives a reference-capability discipline that manages memory
without a tracing collector, a sound path for aliased mutable values. "Typestate via Revocable Capabilities"
(OOPSLA 2025) makes a capability revocable and ties typestate to its validity, so an operation permitted now can
be withdrawn and a later use refused. CHERI (the Capability Hardware Enhanced RISC Instructions architecture,
shipping on Arm Morello, with CHERIoT targeting embedded) makes a pointer an unforgeable hardware capability
carrying bounds and permissions the CPU enforces, so a use outside a capability's bounds or after its revocation
faults in silicon. The cautionary-and-enabling reference for a managed runtime on it is "Pitfalls in VM
Implementation on CHERI: Lessons from Porting CRuby" (2026), alongside a verified CHERI C temporal-safety memory
model (CPP 2025).

Bearing on vehje: reference and revocable capabilities are the shipping-adjacent fallbacks for the mutable,
aliased consumer where the pure-static immutable lease does not reach, an alternative to the research-only
reachability-types path. CHERI is the frame under which the lease axis could be hardware-backed (a region is a
capability with bounds, closing a region revokes it, a use-after-close faults in hardware), an additive target
rather than a core commitment, and the answer to what would make the lease axis's weakest link (no lifetime
types in the low-level target) strong. All three are banked as the map of what a relaxation of immutability, or a
hardware backstop, would cost.

## Sources

Region inference algorithm: Tofte, Birkedal, A Region Inference Algorithm, TOPLAS 20(4), 1998
(https://dl.acm.org/doi/10.1145/291891.291894).

Closure conversion into regions: Aiken, Fähndrich, Levien, Better Static Memory Management, PLDI 1995
(https://dl.acm.org/doi/10.1145/207110.207137); Tofte, Talpin closure-conversion treatment (Region-Based Memory
Management, I&C 132(2), 1997).

Substructural and linear typing: Girard, Linear Logic, TCS 50, 1987
(https://www.sciencedirect.com/science/article/pii/0304397587900454); Wadler, Linear Types Can Change the
World!, 1990 (https://homepages.inf.ed.ac.uk/wadler/papers/linear/linear.ps).

One-shot and linear continuations: Bruggeman, Waddell, Dybvig, Representing Control in the Presence of One-Shot
Continuations, PLDI 1996 (https://dl.acm.org/doi/10.1145/231379.231395); Berdine, O'Hearn, Reddy, Thielecke,
Linear Continuation-Passing, HOSC 15(2-3), 2002.

Separation logic: Reynolds, Separation Logic: A Logic for Shared Mutable Data Structures, LICS 2002
(https://www.cs.cmu.edu/~jcr/seplogic.pdf).

Object-capability and iteratees: Dennis, Van Horn, Programming Semantics for Multiprogrammed Computations, CACM
1966; Miller, Robust Composition (thesis), 2006 (https://papers.agoric.com/assets/pdf/papers/robust-composition.pdf);
Kiselyov, Iteratees, FLOPS 2012 (https://okmij.org/ftp/Haskell/Iteratee/describe.pdf).

Reference and revocable capabilities, CHERI: Reference Capabilities for Flexible Memory Management, 2023
(https://arxiv.org/pdf/2309.02983); Typestate via Revocable Capabilities, OOPSLA 2025
(https://dl.acm.org/doi/10.1145/3808323); Pitfalls in VM Implementation on CHERI: Lessons from Porting CRuby,
2026 (https://arxiv.org/pdf/2603.05645); A CHERI C Memory Model for Verified Temporal Safety, CPP 2025.
