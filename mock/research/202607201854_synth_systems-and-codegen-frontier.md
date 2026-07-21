# Systems and codegen frontier: compacted, one section per thread

**Date:** 2026-07-20
**What this is:** a compaction of the systems-and-codegen frontier the certified-generation panel continuation
banked, chiefly the frontier-holes doc
`202607201854_certgen_panel_continuation/05_systems-codegen-frontier-holes-and-novel-compositions.md`, with the
IR and parsing infrastructure banked in `.../03_core-nodes-typesystem-and-zig-bridge.md` and the hermetic-build
note in `.../02_settlements-and-next-architecture.md`. One section per thread, whole-picture prose with a bearing
clause, dropping the exact numbers and code. For specifics, read the named sources. This sits beside the five
`202607201618_synth_*` domain docs and the theory-side syntheses as banked reading; where those docs bank the
programming-languages theory, this one banks the shipped, benchmarked systems and the IR infrastructure that
realise the arc's mechanisms. It carries only what the systems are and do, not the continuation's design
opinions.

## Runtime generation, shipped: Deegen

Deegen (Xu, "Deegen: A JIT-Capable VM Generator for Dynamic Languages," PACMPL 2026, arXiv 2411.11469, with the
LuaJIT-Remake blog series) takes one semantic description of a language's bytecodes and generates, from that
single source, an optimised interpreter, a baseline JIT, and the tier-up logic between them. The generated Lua
virtual machine beats the reference PUC Lua interpreter by a large margin and comes within roughly a third of
LuaJIT's optimising JIT, from a bytecode-semantics definition that stayed a bounded semantic language rather than
sprawling. Its build-time mechanism is a real optimising compiler (LLVM) plus copy-and-patch for the JIT, with
nothing but the generated artifact shipping.

Bearing on vehje: Deegen is the existence proof, benchmarked, that generating a fast runtime from a single
language definition works and at what speed, which the arc otherwise reasons about from first principles. Its
single-semantic-source shape reframes the arc's pivotal experiment (enumerate the primitive vocabulary across
the census families) into authoring one semantic definition and measuring its coverage, with the interpreter and
the JIT as byproducts. It also establishes that comptime specialisation over data tables is one build-time
mechanism among several, not the only one, since Deegen specialises through LLVM at build time.

## Copy-and-patch compilation: a code generator shipped as data

Copy-and-patch (Xu and Kjolstad, "Copy-and-Patch Compilation," OOPSLA 2021, arXiv 2011.13127, now the CPython
3.13 JIT) pre-compiles, at build time and with a full optimising compiler, a library of machine-code stencils,
one per operation, each a code fragment with holes for operands, live values, and successor addresses. At the
point of use it selects the stencils for a program's operations, concatenates them, and patches the holes, which
is a bounded relocation pass (a memcpy plus a fixup table) with no compiler present. The result is native code
produced in microseconds without an optimising compiler at run time, at baseline-JIT quality.

Bearing on vehje: copy-and-patch is the shipped shape that produces native code for an input unknown until load
without any compiler at the embed site, which is the one thing a no-toolchain runtime otherwise cannot do. It is
the mechanism by which native could become a real execution tier for arriving scripts rather than a bundled-only
target, with the stencils compiled by a trusted compiler at our build time and only the stencil data shipping.

## The eBPF verifier and tristate numbers: load-time safety by abstract interpretation

The Linux eBPF verifier proves an untrusted bytecode program safe (memory bounds, value ranges, no wild access)
by abstract interpretation at load time, before a trusted JIT is allowed to run it. Its abstract domain is a
published, soundness-proven object: tristate numbers (tnum), where each bit is known-zero, known-one, or unknown,
with a formally verified and optimal multiply merged into Linux (Vishwanathan et al., "Sound, Precise, and Fast
Abstract Interpretation with Tristate Numbers," CGO 2022), and the "Verifying the Verifier" (Agni, CAV 2023)
work mechanically checking the verifier's own range analysis. The verifier's dominant scaling cost is path
explosion at branches.

Bearing on vehje: the eBPF verifier is the production realisation of vehje's safety model (prove the arriving
artifact safe at load, then run it fast), with tnum as a proven abstract domain to borrow rather than hand-roll
for the load-time range and bounds check. Its path-explosion cost is the pressure a bounded analysis relieves,
and the arc's finite depth cap is exactly such a bound, so the same finite-height quantity that bounds the
no-alloc frontier also bounds this verifier's exploration.

## simdjson: structural validation of flat data at throughput

simdjson (Langdale and Lemire, "Parsing Gigabytes of JSON per Second," VLDB Journal 2019) validates and indexes
untrusted structured data at gigabytes per second using SIMD, over exactly the flat, self-describing shape a
value-arena has, by staging the work into a structural pass that finds the shape and a second pass that reads it.
It requires the input layout to be SIMD-friendly (aligned, predictable strides).

Bearing on vehje: simdjson is the throughput half of validating untrusted structured bytes, the SIMD structural
pass that the value-arena's flat, index-linked layout is already close to admitting. It is the reference for
running the bounds-and-shape half of the untrusted-load check at throughput, conditional on the arena format
carrying the alignment and stride predictability the technique needs, which is a cheap format decision now and an
expensive change later.

## WebAssembly as a sandboxing substrate: a portable validated target

WebAssembly's design provides a portable bytecode with a load-time validator that establishes memory and
control-flow safety before execution, studied formally as a sandboxing substrate ("Provably-Safe Multilingual
Software Sandboxing using WebAssembly," CMU) and shipped in production as fine-grained in-process isolation (RLBox
with wasm2c in Firefox 95; the Wasmtime security model). The validator is the load verifier, and the safety is a
property of the format checked once per module.

Bearing on vehje: WebAssembly is the reference for a single portable target with a formally-studied load
validator, an alternative to per-architecture native where the validation is the verifier the design already
needs. It is the shape a portable baseline tier could take, where lowering to WebAssembly and shipping one small
embedded runtime with a load-time validator folds the untrusted-load check into the target's own validation.

## Equality saturation: the compile-stage rewrite as saturate-then-extract

Equality saturation (egg, "egg: Fast and Extensible Equality Saturation," PACMPL 2021; egglog, adding a Datalog
side, PLDI 2023; Oatlog, an ahead-of-time-compiled e-graph engine, EGRAPHS 2025) rewrites a program by building
an e-graph of all equivalent forms reachable by a rule set and then extracting the best one, rather than applying
rewrites destructively in an order that can foreclose a better result. egglog's Datalog side expresses side
conditions on rewrites, including ordering constraints, as rules.

Bearing on vehje: equality saturation is the frontier for the IR-to-IR compile stage (macro expansion, constant
folding, and common-subexpression elimination together), a saturate-then-extract that supersedes hand-rolled
order-dependent rewriting, and its Datalog side is where the LMS effect-ordering edges become extraction
constraints (do not extract a form that reorders an effect past a dependent read). The known cost is that these
engines allocate, so a bounded, fixed-capacity e-graph over host-lent memory is the open no-alloc question the
technique raises.

## MLIR, SPIR-V capabilities, and Slang: the output-spectrum and inclusion model at production scale

MLIR (Lattner et al., "MLIR: Scaling Compiler Infrastructure for Domain Specific Computation," CGO 2021, with
IREE as the model-to-many-hardware existence proof) is a compiler infrastructure of composable dialects lowered
to many backends at production scale, where a domain's operations are a dialect and generic passes work over an
open, growing operation set. SPIR-V's `OpCapability` model, checked by `spirv-val` against a declared target
environment, is the shipped form of a target declaring the feature set it supports and refusing the rest by name.
Slang's multi-target intermediate representation emits SPIR-V, HLSL, and more from one front end.

Bearing on vehje: MLIR and IREE are the production-scale existence proof of the output-generation spectrum (one
representation, many targets, an open operation set with per-target support), with a family being precisely a
dialect; SPIR-V `OpCapability` is the shipped precedent for the inclusion-not-coverage proof (a target declares
its supported set and refuses the rest by name); and Slang is the closest single-repository analogue of the
one-representation-many-targets shape. Together they are the systems-audience vocabulary in which the arc's
output and inclusion axes are already-shipped abstractions.

## IR and parsing infrastructure: attribute grammars, PEG and GLL, and the effect-region IR

Three IR and parsing foundations complete the picture. Knuth's "Semantics of Context-Free Languages" (1968)
introduces attribute grammars, in which a grammar is annotated with synthesised attributes (computed bottom-up)
and inherited attributes (computed top-down), evaluated by a general engine over the parse tree; an L-attributed
grammar is evaluable in one left-to-right pass. PEG (Ford, "Parsing Expression Grammars," 2004) and GLL
(Scott and Johnstone) are the parsing formalisms whose grammars are a declarative table a general parser
interprets, PEG for unambiguous ordered-choice grammars and GLL for general context-free grammars. Sea-of-nodes
(Click and Paleczny, 1995) and the Regionalised Value-State Dependence Graph (RVSDG, Bahmann et al., 2015) are
intermediate representations that carry data, control, and effect dependencies as first-class edges rather than
as a fixed statement order, with RVSDG additionally nesting regions.

Bearing on vehje: attribute grammars are the frame under which the lease inference is a single bottom-up
synthesised-attribute evaluation over the post-order structure (which is why the one-pass, no-extra-walk
constraint is the natural shape of an L-attributed evaluation, not just an optimisation), and under which
grammar-as-data means a declarative grammar table plus a general parser. PEG and GLL are the data shape such a
grammar table can take. Sea-of-nodes and RVSDG are the IR shapes to reach for if the effect-edge representation
ever grows past A-normal form toward parallel evaluation or aggressive reordering, and RVSDG's nested regions
compose with the lease region nesting.

## Hermetic and reproducible builds: the trusted build step's engineering precedent

The Nix and Bazel lineage of hermetic, reproducible build systems establishes the engineering practice of a
build step whose output is trusted because its inputs and its toolchain are pinned and its execution is isolated,
so the same inputs always produce the same output and the step is auditable.

Bearing on vehje: hermetic builds are the engineering precedent for a build-time content-validation step (and,
under the native-tier direction, a stencil-compilation step) as a deterministic, version-pinned member of the
trusted computing base, trusted because its inputs and toolchain are fixed rather than because it is proven line
by line.

## Sources

Deegen: Xu, Deegen: A JIT-Capable VM Generator for Dynamic Languages, PACMPL 2026, arXiv 2411.11469
(https://arxiv.org/pdf/2411.11469); LuaJIT-Remake blog series (https://sillycross.github.io).

Copy-and-patch: Xu, Kjolstad, Copy-and-Patch Compilation, OOPSLA 2021, arXiv 2011.13127
(https://arxiv.org/pdf/2011.13127); CPython 3.13 experimental JIT (bytecode copy-and-patch).

eBPF verifier and tristate numbers: Vishwanathan et al., Sound, Precise, and Fast Abstract Interpretation with
Tristate Numbers, CGO 2022 (https://doi.org/10.1109/CGO53902.2022.9741267); Verifying the Verifier: eBPF Range
Analysis Verification (Agni), CAV 2023 (https://people.cs.rutgers.edu/~sn349/papers/agni-cav2023.pdf).

simdjson: Langdale, Lemire, Parsing Gigabytes of JSON per Second, VLDB Journal 2019
(https://arxiv.org/abs/1902.08318).

WebAssembly sandboxing: Provably-Safe Multilingual Software Sandboxing using WebAssembly (CMU)
(https://www.andrew.cmu.edu/user/bparno/papers/wasm-sandboxing.pdf); RLBox and wasm2c in Firefox 95
(https://hacks.mozilla.org/2021/12/webassembly-and-back-again-fine-grained-sandboxing-in-firefox-95/); Wasmtime
security model (https://docs.wasmtime.dev/security.html).

Equality saturation: egg: Fast and Extensible Equality Saturation, PACMPL 2021
(https://dl.acm.org/doi/10.1145/3434304); egglog (Datalog plus equality saturation), PLDI 2023; Oatlog
(AOT-compiled e-graph engine), EGRAPHS 2025.

MLIR, SPIR-V, Slang: Lattner et al., MLIR: Scaling Compiler Infrastructure for Domain Specific Computation, CGO
2021 (https://arxiv.org/abs/2002.11054); IREE (https://iree.dev); SPIR-V `OpCapability` and `spirv-val`; Slang
multi-target IR (https://shader-slang.org).

IR and parsing: Knuth, Semantics of Context-Free Languages, Mathematical Systems Theory 2, 1968; Ford, Parsing
Expression Grammars, POPL 2004 (https://bford.info/pub/lang/peg.pdf); Scott, Johnstone, GLL parsing; Click,
Paleczny, A Simple Graph-Based Intermediate Representation, 1995; Bahmann et al., RVSDG, 2015
(https://arxiv.org/abs/1912.05036).

Hermetic builds: Nix (https://nixos.org) and Bazel (https://bazel.build) hermetic-build lineage.
