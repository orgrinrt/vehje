# A neural/ML-accelerated take on the runtime (panel insight 07)

**Task:** read the synthesis docs and the certified-generation panel, then give a genuinely useful take on
whether small CPU-friendly NN/ML inference (at load, at JIT time, or precomputed AOT) could accelerate parts of
the vehje runtime, whether ML fits the Rust-to-Zig boundary, and whether a small inference model could speed up
runtime questions outside interpretation itself (loading, allocation, aliasing, fencing, threading). This is a
worker-fork deliverable; it reports once and stops.

## The one hard line, stated first

ML has no place in the correctness path, and putting it there would destroy the entire point of the design.
Everything the panel certified (family inclusion, effect inclusion, non-null, dispatch totality, the
depth-ladder lease safety, bounds validation) is proof-shaped or decidable-check-shaped: it is either proven
once as a metatheorem or re-established by a total, type-checked traversal. A learned classifier that is right
99.9 percent of the time is worthless for any of these, because the 0.1 percent is an unsound dereference, a
missed inclusion violation, a host process aborted by a script that the model waved through. The whole
certified-generation resolution exists to make those guarantees hold by construction; a model that is
sometimes wrong is exactly dead end 1 (the unverified runtime checker) wearing a fashionable hat. So the rule
for this entire document: **ML is confined to policy, never to correctness.** Policy is a decision where a
wrong answer costs performance and the real check still runs to catch any safety consequence. Correctness is a
decision where a wrong answer is unsound. Every proposal below lives strictly on the policy side, and each names
what a wrong prediction actually costs so the boundary stays visible.

This is not a hedge, it is the industry's own settled position. LLVM's MLGO (Machine Learning Guided
Compiler Optimizations, Trofin et al., arXiv 2101.04808) ships a small embedded model that makes inlining and
register-allocation-eviction *policy* decisions inside a production compiler, and it is careful that the model
only ranks choices that are all correct; the verifier and the pass semantics are untouched. That is the exact
shape that fits vehje: a small model choosing among options that are all already safe.

## Where a small CPU model legitimately fits (all outside interpretation correctness)

The panel surfaced a cluster of runtime policy decisions that are today either hand-heuristic or unspecified,
and every one of them is a place where a wrong guess costs a re-size, a spill, or some wasted cache, never
safety. These are the candidates:

- **Arena and streaming-window sizing.** The transfer topic lends the runtime a fixed window and spills the
  rest. Sizing that window is a prediction: too small forces avoidable spills (Expert 4's cost), too large
  wastes the host's budget. A model that predicts a script's working-set size from cheap structural features
  would size the window to minimise spills within budget. Wrong prediction cost: an extra spill or some slack
  memory, both bounded and observable, never unsound.
- **Spill timing and chunk-boundary placement.** The streaming spine cuts the value graph into whole-subtree
  chunks. Where to cut, and when to spill versus hold, is a policy over the post-order arena that trades
  re-read cost against resident-window size. A learned policy could place boundaries to keep the lease-frontier
  accumulator small and cross-chunk references rare. Wrong cost: a suboptimal spill schedule, more I/O, never a
  broken lease (the cross-chunk lemma and the validator still hold regardless of where the cut lands).
- **Tier selection (interpret versus lower to bytecode).** This is the classic JIT tiering decision, and it is
  the single most natural ML fit in the whole runtime, because tiering is where real JITs already use profiling
  heuristics and where learned policies have a track record. Predict, from a script's shape and a cheap first
  touch, whether it is hot enough that lowering pays for itself, or cold enough to interpret directly. Wrong
  cost: a cold script needlessly compiled (wasted build) or a hot script left interpreted (slower), never a
  wrong result.
- **Eval-branch-quota and depth-cap sizing.** Expert 4 made the depth cap load-bearing for no_alloc under
  streaming and flagged the comptime branch quota as a real budget. Predicting a good quota or a good starting
  frontier-buffer size from the language table and a sample corpus is a policy; the hard cap still fires as a
  graceful error if the prediction is low. Wrong cost: a re-run at a higher quota, or a slightly oversized
  buffer.
- **Lease-promotion and layout hints.** The depth-ladder inference computes promotions exactly and soundly;
  but a model could *hint* which values are likely to promote so the arena lays them out for locality up front,
  or pre-sizes the output region. The inference remains the source of truth; the hint only reorders bytes.
  Wrong cost: worse cache behaviour, never a wrong lifetime.
- **Threading and work placement, where it applies.** The per-script runtime is mostly single-shot, so this is
  the weakest candidate for vehje's own runtime, but a consumer that fans scripts across cores (a mod loader, a
  batch document generator) has a scheduling-and-placement policy that is exactly the kind of thing learned
  schedulers target. Fencing and aliasing decisions, by contrast, are correctness-shaped (a wrong fence is a
  data race, a wrong aliasing assumption is unsound), so those stay out; only the *scheduling* of provably
  independent work is policy.

Note what is absent: nothing in interpretation itself, nothing in the checks, nothing in the lease proof. The
list is deliberately the optimisation rim, which is exactly the rim the panel kept saying was
best-effort-under-a-checked-guarantee.

## The model shape that actually fits the constraints (and it is a clean fit)

The design's constraints are not obstacles to a model here; they force it into the one shape that also happens
to be the cheapest and most deterministic.

- **Fixed-point, never float, so it is deterministic and legal.** The theory review's condition list bans
  floats from the dual-binding-time kernel, and ikiuni requires deterministic execution. A float model breaks
  both. A *quantised, integer/fixed-point* model breaks neither: integer inference is bit-exact and
  reproducible across machines. And vehje already has the substrate for this, arvo's `UFixed`/`IFixed`
  fixed-point types are the natural numeric domain for a quantised model, so the model's weights and its
  arithmetic are expressed in the stack's own primitives. This is a real convergence, not a coincidence: the
  no-float rule that the certified kernel needs is the same rule that makes the model deterministic, and arvo
  is the reason it costs nothing to obey.
- **Small and simple, so inference is microseconds on CPU.** The policy decisions above are low-dimensional
  (tens of features, not images or token streams). A gradient-boosted decision tree, a small random forest, or
  a single-hidden-layer quantised MLP is more than enough, and each evaluates in microseconds on one core with
  no accelerator. Decision-tree and GBDT inference is pure integer comparisons and adds; a tiny quantised MLP
  is fixed-point matmul. Both are trivially expressible as a small pure kernel.
- **The inference kernel is comptime-legal, so it rides the same rails as everything else.** A fixed-point
  tree-eval or matmul over caller-supplied buffers, with no floats, no allocator, and bounded loops, satisfies
  the theory review's six conditions, which means the model can be evaluated at *any* of the three loci the
  panel identified: at comptime for bundled content, at the native build step, or at load in the shipped
  binary. Nothing special is needed for it.

## The Rust-to-Zig boundary fit: the model is just more data in the language blob

This is where it clicks into the certified-generation architecture without adding anything new. Training is a
dev-time activity and lives entirely on the far side of the boundary: offline, in Rust or a Python trainer,
with floats and a GPU if you like, over a corpus of scripts for a given language. The *shipped* artifact is the
trained model *quantised to fixed-point weights*, and those weights are **data**, emitted into the same
language-definition blob that Giesen's wire-bridge already carries, decoded by the same codec, validated the
same way. The Zig runtime never contains a neural-network framework; it contains one small fixed-point
inference kernel (a few dozen lines) that reads the weights-as-data and produces a policy hint. So the model
obeys the design's own deepest principle exactly as the checkers do: **dev-time produces proven-or-trained
data, the runtime evaluates it with a small pure kernel, and nothing heavyweight crosses the boundary.** The
model is to policy what the mask table is to inclusion: language-specific data, emitted once, evaluated cheaply.

AOT versus JIT falls out naturally. The weights are computed AOT (trained dev-time, shipped in the blob). The
*inference* can run at load (score the arriving script once, set the window size and tier for its execution) or
be precomputed for bundled first-party content at the native build step. There is no runtime training, ever;
only fixed-point evaluation of frozen weights.

## The one genuinely novel angle: the features are already being computed

The reason this can be "hardly noticeable" is not that the model is small, it is that **the feature vector is a
free by-product of a pass the runtime already runs.** The load-time bounds validation, mask accumulation, and
depth-ladder lease inference are, per Giesen and Expert 4, a single linear scan over the post-order arena. That
same scan trivially accumulates exactly the cheap structural features a policy model wants: node count, maximum
and mean nesting depth, the family histogram, the link-to-consume ratio, the string-blob size, the fan-out
distribution. So the model's input is produced at zero marginal cost during a scan that must happen anyway, and
the only added work is one fixed-point tree or matmul evaluation at the end of the scan. That is how the
acceleration stays under the noise floor: the runtime scans once, gets its safety checks and its policy
features from the same pass, and spends a few microseconds turning the features into window size and tier. No
second traversal, no separate profiling pass.

## The honest limits (this is where the workspace's own rules bite)

- **Earn it with a bench, or do not build it.** The workspace's run-the-experiment rule applies hardest here. A
  hand-tuned heuristic (window = f(node_count, max_depth); tier = compile if node_count over N) almost always
  matches a model for a low-dimensional decision, and it is inspectable and needs no corpus. A model earns its
  place only where a bench shows both that the decision is hot and that a hand heuristic leaves measurable
  performance on the table. Start with the hand heuristic; introduce the model only against measured evidence,
  exactly as the panel demanded for every other optimistic lean.
- **The corpus problem.** A trained policy needs a representative corpus of scripts per language, which does not
  exist for a language that has not shipped. So this is a maturity feature, not a founding one: the hand
  heuristic ships first, the corpus accumulates from real use, and a model is trained when there is enough data
  to beat the heuristic. Designing the policy seam now (so a hint provider is swappable: hand heuristic today, a
  fixed-point model later, both behind one interface) is the founding move; training a model now would be
  premature.
- **A hint can never compromise safety, and the design must keep it that way.** The model's output must only
  ever influence sizing, placement, and tiering, all of which are re-checked or bounded downstream. If a
  predicted window is too small, the streaming path spills; if a predicted quota is too low, the hard cap fires
  gracefully; if a tier choice is wrong, the result is still correct, just slower. The moment a model's output
  is allowed to influence anything the checks depend on, the boundary is breached. State the invariant: policy
  hints are advisory inputs to bounded, checked mechanisms, never to the checks themselves.
- **Determinism forbids float and forbids ambient input.** The model must be fixed-point (above), and its
  features must come only from the script bytes, never from a clock, a machine-load reading, or any ambient
  runtime state, or two runs of the same script diverge. This keeps the policy deterministic, which ikiuni
  requires and which also keeps the "same script, same behaviour" contract the whole runtime rests on.
- **Staleness and versioning.** A model trained against one version of a language's families is stale when the
  language changes. The weights are versioned with the language blob and retrained dev-time on a pin bump, the
  same discipline as every other piece of generated data.

## Where it explicitly does not fit, said bluntly

Not the interpreter. Not the lease metatheorem or its inference-as-source-of-truth. Not family, effect, bounds,
or dispatch checks. Not aliasing or fencing decisions (those are correctness, and a wrong learned aliasing fact
is unsound). Not the certified-generation proofs. Any proposal to "learn" any of these is a category error that
trades the design's entire reason for existing against a marginal speed-up on a path that is already fast.

## Concrete minimal proposal

Design one swappable policy-hint interface now, taking the free feature vector from the load-time scan and
returning window size, tier choice, and spill granularity. Ship a hand-tuned fixed-point heuristic behind it as
the founding implementation. Instrument the three decisions so the corpus accumulates the features plus the
realised cost (spill count, compile-versus-interpret win, actual working set). When a language has shipped
enough to have a corpus and a bench shows a heuristic gap, train a small quantised (arvo fixed-point) GBDT or
single-layer model offline, emit its weights as data in the language blob, and evaluate it with the small
comptime-legal fixed-point kernel at load. The model never touches a check, never sees ambient state, never
uses a float, and never trains at runtime. That is the whole of where NN/ML belongs in this design: a
deterministic, data-shaped, bench-gated policy accelerator riding a scan the runtime already pays for, strictly
downstream of every guarantee, and introduced only when measured evidence says a hand heuristic is not enough.

## Sources

- MLGO: a Machine Learning Guided Compiler Optimizations Framework (Trofin et al., arXiv:2101.04808); the
  production LLVM inlining-for-size and regalloc-eviction models, embedded and integer-friendly, policy-only.
- Ithemal: Accurately Predicting Basic Block Throughput using a hierarchical LSTM (Mendis et al., ICML 2019),
  as the learned-cost-model precedent (and as a caution: it is a cost *estimate*, never a correctness input).
- CompilerGym / AutoPhase (Facebook/Berkeley), reinforcement learning for compiler pass ordering, the same
  policy-not-correctness posture.
- The Case for Learned Index Structures (Kraska et al., SIGMOD 2018), the general "small model replaces a
  hand-tuned heuristic where the distribution is learnable" argument, with the same "the structure still has to
  be correct, the model only chooses layout" discipline.
- Integer-only / quantised inference: gemmlowp and the integer-arithmetic-only inference line (Jacob et al.,
  "Quantization and Training of Neural Networks for Efficient Integer-Arithmetic-Only Inference," CVPR 2018),
  the basis for a deterministic fixed-point model.
- JIT tiering as a profiling-driven policy: kipp.ly, "A Deep Introduction to JIT Compilers" (cited elsewhere in
  the panel), for why tier selection is the natural learned-policy site.
- The panel and synthesis this builds on: `01_carmack.md` through `05_systems-review.md` in this directory (the
  three-loci split, the post-order single-scan finding, the depth cap, the fixed-point/no-float condition), and
  `202607201618_synth_ir-representation-and-no-alloc-arenas.md` and
  `202607201618_synth_purity-totality-effects-and-the-compile-runtime-split.md` for the allocation and
  compile-runtime-split surfaces the policy decisions sit on. arvo `UFixed`/`IFixed` as the fixed-point substrate.

## Amendment: NPUs, GPUs, and the idle-accelerator question

The question is whether the mostly-dormant NPUs now shipping in laptops and desktops, and the matrix units in
APUs and dGPUs that sit idle while a script is being AOT or JIT compiled, change the answer. They are real free
silicon and they are genuinely underutilised. The honest conclusion is that they do not change the answer for
vehje's own machinery or for its policy model, for four reasons that compound, and that the one place they
legitimately enter is not vehje at all but a consumer language whose values are tensors. Idle silicon you
cannot feed the right shape of work is not free performance; it is a mismatch, and this is a mismatch on every
axis that matters.

**Reason 1, the work is the wrong shape.** NPUs and GPU matrix units are dense, regular, data-parallel dataflow
engines: they win on big matmuls and convolutions with uniform control flow. Everything vehje's runtime
actually spends time on, the post-order load scan, mask accumulation, the depth-ladder lease inference, bounds
validation, and the interpretation itself, is the opposite: irregular, index-and-pointer-chasing,
branch-divergent, and largely a sequential dependency chain (the lease frontier and the post-order scan are
inherently ordered). This is the canonical worst case for both an NPU and a GPU; branch divergence and
scatter-gather demolish their throughput. There is no dense linear-algebra structure hiding in parsing or
checking or lease inference to hand them. Even the batch case (AOT-compiling a mod pack of thousands of
scripts) is embarrassingly parallel *across* scripts but each script is a small control-flow-heavy sequential
job, so it is a CPU-multicore workload (one core per script), not a GPU one, because the per-job work is not
SIMD-uniform. So the GPU being idle at compile time is true and useless: the compiler is the exact program you
cannot run on it.

**Reason 2, the dispatch overhead dwarfs the work.** The policy model this document proposes is tiny and runs
once per script at load, a few microseconds of fixed-point evaluation on one CPU core. Handing that to an NPU
or GPU costs far more than it saves: dispatching to an accelerator carries driver, queue-submission, and DMA
setup latency measured in tens to hundreds of microseconds at best, often into milliseconds, before a single
useful operation runs. You would pay more to move the work to the silicon than to do the work. Accelerators
amortise their dispatch cost only over large, repeated, sustained inference, which is precisely what vehje's
one-shot microsecond policy call is not.

**Reason 3, determinism forbids it on the execution path.** ikiuni and the whole "same script, same behaviour"
contract require bit-reproducible execution. NPU and GPU math is routinely not bit-reproducible across vendors,
drivers, and even runs: reduction orders differ, low-precision rounding differs, fast-math and non-associative
float paths differ. Anything on the deterministic execution path therefore cannot use them at all, and the
policy hint must be deterministic too (its output influences window size and tier, which must not vary run to
run). A fixed-point CPU model is bit-exact; an NPU/GPU model is not portable-reproducible. So even where the
throughput argument might tempt, the determinism gate closes it for the runtime path.

**Reason 4, the dependency breaks the embeddability identity.** vehje's entire differentiator is a minimal,
no_std, no-dependency Zig runtime that ships as one small per-target binary with no external toolchain. Using
an NPU means binding a vendor stack (Apple ANE via CoreML, Intel NPU via OpenVINO, AMD XDNA via the Ryzen AI
runtime, Qualcomm Hexagon, or Windows-only DirectML), each different, none portable, all heavyweight. Using a
GPU means binding Vulkan, Metal, D3D12, or CUDA. Dragging any of these into the runtime to use idle silicon for
wrong-shaped work is a catastrophic trade against the one identity the framework has. The `vehje-runtime-abi`
forbidden-imports lint already bans `std::fs`, `std::net`, and friends for exactly this posture; a GPU driver
dependency is the same category, larger.

**The one legitimate entry point, and it is a consumer's, not vehje's.** There is a real case where these units
matter, and locating it precisely is the useful part of the answer. If a *consumer language's own runtime
values are dense tensors*, an ML DSL, a shader or simulation language, a numerical scripting language, then that
consumer's computation is exactly the dense-matrix work NPUs and GPUs exist for. The correct architecture for
that is already in vehje's design and needs nothing new: the consumer declares a tensor family whose operations
are a runtime-environment effect (a `Writes<GpuEnv>`-shaped or host-call effect), and the runtime plumbs that
effect to a host-supplied backend at the effect boundary, exactly the way any other host call crosses. The
accelerator is then the *consumer's* dependency, brought at the host boundary, opt-in, and explicitly outside
the deterministic core (a consumer that routes to a GPU accepts determinism relaxation for those ops, or scopes
them out of its deterministic path). vehje-core never touches the silicon; it provides the effect seam that
lets a consumer route to it. This is not a workaround, it is the effect axis doing precisely the job it was
designed for, and it is the reason vehje does not need to "use" the NPU or GPU: it needs only to let a consumer
whose semantics are matrix-shaped reach them through the boundary the architecture already draws. The idle
silicon gets used, by the one workload that fits it, through the one seam that keeps it out of the deterministic
minimal core.

**On "it would be sitting idle anyway."** Idleness is not the criterion; fit is. Free silicon that can only run
the wrong shape of work, at a dispatch cost that exceeds the work, with non-reproducible results, behind a
vendor dependency that breaks the runtime's identity, is not an opportunity being missed. The CPU wins vehje's
machinery and its policy model on all four axes at once, and the accelerators win exactly one workload that is
not vehje's to run. The answer does not change; it sharpens, and it sharpens toward the effect-axis boundary
being the right and only place a matrix unit belongs in this stack.

### Amendment sources

- NPU programmability and vendor fragmentation (no portable API across Apple ANE / Intel NPU / AMD XDNA /
  Qualcomm Hexagon; CoreML, OpenVINO, Ryzen AI, DirectML as disjoint stacks): vendor SDK documentation; the
  general "NPUs are hard to target portably and are tuned for sustained large-model inference" position in
  current edge-AI surveys.
- Accelerator dispatch and offload latency dominating small one-shot workloads: the well-established
  kernel-launch-overhead and host-device transfer-cost literature (the same reason small GEMMs stay on CPU),
  e.g. GPU kernel-launch-overhead measurements and the roofline/offload-threshold analyses.
- Control-flow divergence and irregular-access penalties on SIMT hardware: the branch-divergence and
  memory-coalescing sections of standard GPU-architecture references (Hennessy and Patterson, data-parallel
  chapter; NVIDIA/AMD programming guides on warp/wavefront divergence).
- Non-determinism of GPU/NPU floating-point reductions across hardware and drivers: the reproducibility
  literature on non-associative parallel reductions and vendor fast-math modes (e.g. the numerical-reproducibility
  discussions behind deterministic-ML efforts such as `tf.config.experimental.enable_op_determinism` and the
  bitwise-reproducibility caveats in CUDA/cuDNN docs).
- The effect-axis-as-host-boundary reasoning is vehje's own settled design (`202607192358/` output-side and
  ABI topics; the `Reads<E>`/`Writes<E>` effect model), applied here to matrix-unit host calls.

## Amendment 2: expose the silicon to consumers, do not consume it internally

The previous amendment ruled out vehje using the NPU and GPU for its own pipeline. This one takes the inverse
and productive angle: vehje is a framework for building languages, so instead of consuming the silicon it can
*expose* it, making it easy for a language author, and through them a script writer, to define logic that runs
on the NPU or GPU. This is not a workaround bolted onto the earlier "no"; it is the earlier answer completed.
The previous amendment located the one legitimate place a matrix unit belongs (a consumer whose values are
tensors, reached through the effect boundary). This amendment observes that vehje already has the three seams
that turn that location into a first-class, ergonomic capability, and that the determinism gate the previous
amendment worried about is not a problem to solve but a safety mechanism the architecture already provides for
free. The whole thing composes out of pieces the round has already settled.

**The accelerator is just a target, which is the native-is-just-a-target insight already generalised.** The
three-codegens topic established that output generation is a spectrum and that native is not a special concept,
only one output target. A GPU target and an NPU target are the same move again: output definitions that lower a
program, or the parts of it that are dense and data-parallel, to accelerator code (SPIR-V or a compute kernel
for the GPU, an ONNX or CoreML graph for the NPU), while the irregular remainder interprets on the CPU. That is
a hybrid point on the interpret-to-transpile spectrum *within one program*: dense subgraphs transpile to the
accelerator, control-flow-heavy code interprets, and the output-spectrum machinery already supports the mix.
Compilation stays on the CPU (it is control-flow-heavy, the wrong shape for the accelerator, exactly as
amendment 1 found); only the *result* runs on the silicon. Compile on CPU, run on GPU, which is what every
accelerator toolchain does.

**The determinism boundary is enforced for free by the effect axis, which is the best part.** Amendment 1's
hard gate was that accelerator math is non-reproducible and therefore cannot touch the deterministic path.
Exposing the accelerator as a first-class effect turns that gate from a hazard into a checked contract at zero
extra cost. An accelerator family declares its operations as a `Writes<AcceleratorEnv>`-shaped effect. A
deterministic consumer's runtime declares `Permits` *without* `AcceleratorEnv`, so any script that reaches for a
GPU op is refused at check time, with the offending construct named, by exactly the inclusion-not-coverage
mechanism the output-side contract already ships. An opt-in consumer that accepts determinism relaxation
includes `AcceleratorEnv` in its `Permits` and the same script is allowed. So a language author does not have
to build a determinism firewall around accelerator access; the effect axis *is* the firewall, and it is
impossible to cross it accidentally. This is the effect model earning its keep spectacularly: the single
scariest property of GPU access (it breaks determinism) is contained by a mechanism vehje built for an unrelated
reason, and ikiuni's per-frame deterministic runtime and a batch tensor-crunching consumer coexist on the same
framework with the boundary between them statically checked.

**The value transport, the lease axis, and the no-alloc discipline all extend to tensor values unchanged.** The
value-transfer study's scalar-versus-bulk-versus-graph channel split already says bulk data crosses as a shared
region, not as a serialised graph; a tensor is the bulk channel, and it crosses to and from the accelerator as a
shared buffer, zero-copy where the backend allows, which is the `BackingStore`/`ArrayBuffer` pattern the study
recorded. The depth-ladder lease axis governs that buffer's lifetime the same way it governs any region: a GPU
buffer is a region with a lease, allocated in an arena-shaped pool, freed at region exit, and the same
proven-once inference decides when it dies. And no_alloc survives because the buffer is host-lent through the
same reserve/commit sink the transfer topic already built, now pointed at a GPU allocation instead of at the
output window. Nothing in the value model needs a new concept for tensors; they are large immutable values that
happen to live in device memory, and the existing machinery already describes exactly that.

**The standard library is what makes it "extremely easy," and it is a composition, not an invention.** vehje, or
a first-party companion, ships a reusable bundle: a tensor-and-parallel-compute family (node kinds for the dense
primitives, elementwise map, reduce, matmul, convolution, scan), its effect classification
(`Writes<AcceleratorEnv>`), its lease and value-transfer wiring (buffers as leased regions crossing as bulk
channels), and one or more accelerator targets behind a portable backend abstraction with a mandatory CPU
fallback. A language author composes that bundle in the way they compose any family and target, and their script
writers reach it through whatever surface syntax the language chooses (an array type with parallel operators, a
`@gpu` block, a tensor literal). The author writes no vendor code; the standard library owns the
Vulkan/Metal/DirectML/CoreML backends and the CPU fallback behind one interface, and the concrete backend is a
host-provided, swappable implementation, so the accelerator dependency is the *consumer's runtime's*, brought at
the host boundary, never vehje-core's. The framework identity holds: vehje-core stays minimal and
dependency-free; the accelerator library is an opt-in extension a consumer links, exactly like any other
consumer family.

**Honest caveats, because this is a large capability.** Portability across accelerators is a real abstraction
problem, not a free lunder; the sensible path is to wrap an existing portable compute layer (WebGPU/wgpu for the
GPU side gives Vulkan, Metal, and D3D12 from one API; ONNX Runtime or a small SPIR-V/CoreML-lowering layer for
the NPU side) rather than hand-roll one, with the prior art being the DSLs that already solved "make accelerator
programming easy from a high-level language": JAX and XLA, Triton, Halide's algorithm-schedule split, Mojo's
portable CPU-plus-accelerator model, and IREE's compile-a-program-to-many-accelerators pipeline. Not all logic
maps to the silicon, so the library must expose the dense-parallel primitives ergonomically and *discourage*
forcing irregular control flow onto the accelerator, which is a footgun that produces slow, divergent kernels;
the language surface should make the parallel-friendly subset the easy path and the accelerator effect a
deliberate opt-in, not a default. And this is a maturity feature, not a founding one: the *seam* already exists
(the effect axis, the target spectrum, the value-transfer channels, the lease axis all already accommodate it,
which is the whole point of this amendment), so the founding move is to keep those seams clean and general, and
the standard library gets built when a real consumer, an ML DSL, a shader language, a simulation scripting
language, actually needs it, per the run-the-experiment discipline.

**The payoff.** This turns amendment 1's "idle silicon vehje cannot use" into "idle silicon vehje makes
trivially and safely available to every language built on it." A language author gets accelerator targeting, a
statically-checked determinism boundary, tensor value-transport, and buffer lifetime management for free by
composing one standard library, instead of hand-rolling a vendor backend and a determinism firewall. And it is
not a new subsystem; it is the effect axis, the output-target spectrum, the value-transfer channels, and the
lease axis each doing precisely the job they were designed for, on a workload (dense tensor compute) that finally
fits the matrix units the previous amendment correctly kept out of vehje's own core. That is the right answer to
"could we expose these instead of consuming them": yes, and the architecture is already shaped to do it, with
the determinism gate as a feature rather than a cost.

### Amendment 2 sources

- Portable GPU compute abstraction: WebGPU and the `wgpu` implementation (one API over Vulkan, Metal, D3D12);
  SPIR-V as the portable GPU IR; Khronos Vulkan compute.
- High-level-language-to-accelerator DSLs as the ease-of-authoring prior art: JAX / XLA (functional array
  program to accelerator), Triton (Python-embedded GPU kernel DSL, Tillet et al.), Halide (algorithm-schedule
  separation, Ragan-Kelley et al., PLDI 2013), Mojo (portable CPU-plus-accelerator programming model), IREE
  (MLIR-based compile-once-run-on-many-accelerators).
- Portable NN inference across NPU/GPU/CPU backends: ONNX Runtime and its execution-provider model, as the
  precedent for one program dispatched to many vendor backends behind one interface.
- The determinism-as-effect-gate, tensor-as-bulk-channel, and buffer-as-leased-region reasoning is vehje's own
  settled design (the `Reads<E>`/`Writes<E>` effect model and inclusion-not-coverage from `202607192358/`; the
  scalar/bulk/graph channel split from `202607201618_synth_embeddable-runtime-value-model-and-host-boundary.md`;
  the depth-ladder lease axis from this panel), recomposed here for accelerator access.

## Amendment 3: the [Differentiable]-style simplification loop, generalised and made safe

Slang's `[Differentiable]` is the sharpest possible prompt for this, because what it actually did is not "add
autodiff," it is "let an attribute trigger a compile-time source transformation that synthesises a theory-heavy
capability the author would otherwise hand-write and get wrong." A graphics programmer marks a function
`[Differentiable]`, writes only the forward pass, and the compiler generates the reverse-mode derivative; the
chain rule lives in the compiler pass, not in the author's head. The general shape is: annotation, then a
certified program transformation, then a capability, with the theory hidden in the transformation. The question
is whether vehje can offer that loop for accelerated programs, and the answer is that vehje is unusually well
positioned to offer it, more so than Slang, because the transformation substrate, the safety gate, and the
plumbing are all already in the design, and because the same loop generalises past acceleration to any
theory-heavy capability at all.

**The transformation substrate already exists: it is macro expansion.** The three-codegens topic settled that
one of vehje's three codegens is macros, and that macros are precisely IR-to-IR transformation, "the point of
the IR." An attribute like `[Accelerated]` or `[Parallel]` is exactly a macro in that sense: it triggers an
IR-to-IR pass that rewrites the marked subgraph into its accelerator-lowered form plus the synthesised plumbing
(the buffer transfer through the bulk channel, the lease on the device region, the `Writes<AcceleratorEnv>`
effect, the CPU fallback). That is structurally identical to `[Differentiable]` triggering an autodiff
source-transformation: mark intent, and a compile-time transformation you did not write produces the capability.
vehje does not need a new mechanism for this loop; it needs to recognise that its macro codegen *is* the loop,
and that an accelerator standard library ships the specific transformation the way Slang ships its autodiff
pass.

**The loop generalises, and this is the more important point.** Because the mechanism is "attribute triggers a
family-provided, certified IR-to-IR transformation," acceleration is only one instance. The same loop
encapsulates autodiff itself (a `[Differentiable]` family for vehje, whose pass synthesises the derivative,
directly mirroring Slang), auto-vectorisation, gradient checkpointing, memoisation, and accelerator offload,
each as a family that ships an attribute and the transformation behind it. So the answer to "a similar
simplification loop" is not a one-off feature; it is a *general capability-by-annotation mechanism* that
vehje's macro codegen already provides, of which accelerator offload and Slang-style differentiability are two
worked examples. A language author includes the family, a script author writes the attribute, and the
theory-heavy transformation happens for free, uniformly across capabilities.

**The "accidental" level is real, and it is the JAX/Futhark model.** If the language's array or tensor type is
a family whose operations carry the `Writes<AcceleratorEnv>` effect and an auto-offload pass, then an author
writing ordinary array code, a `map`, a `reduce`, a `matmul`, has already expressed dense-parallel structure,
and the offload pass identifies those subgraphs and rewrites them to the accelerator target with no attribute
at all, when it is permitted and profitable. The author accidentally wrote an inference-accelerated program by
using the natural array API of a language that happens to include the accelerator family, exactly as a JAX user
gets XLA-compiled accelerator execution from writing NumPy-shaped code, or a Futhark user gets GPU code from
writing a functional array program. The acceleration is a transparent consequence of the primitives, not an
explicit act, which is the "accidental" op is after.

**Why vehje's loop is strictly safer than Slang's, which is the payoff.** Slang's `[Differentiable]` will
happily generate a derivative that is enormous or slow, and it has no notion of permission or determinism; the
transformation is trusted and unbounded. vehje's version is bounded on three axes it already owns:

- *The effect axis gates it.* Accidental or explicit offload emits a `Writes<AcceleratorEnv>` effect, and a
  deterministic consumer's `Permits` excludes `AcceleratorEnv`, so the offload pass cannot fire in a
  deterministic program, and a tensor op that would offload is refused at check time with the construct named.
  Accidental acceleration is therefore impossible exactly where it would break determinism, and permitted
  exactly where the consumer opted in. Slang has no such gate.
- *The transformation is certified.* This whole panel's resolution means the `[Accelerated]` pass is a
  generated, consistency-checked transformation, and the post-transformation IR is re-run through the same
  lease, effect, and inclusion checks as any other IR. So the transformation cannot produce an unsound, an
  unpermitted, or a badly-typed program; if the rewrite is wrong, the certified checks catch it. Slang's autodiff
  output is trusted; vehje's transformation output is re-checked by construction.
- *The policy gate makes it best-effort, never wrong.* Whether a given subgraph is worth offloading is the
  policy-model decision from the body of this document (is it dense and hot enough to beat the dispatch cost),
  and a wrong policy call means the subgraph runs on the CPU fallback, correct and merely slower, never a wrong
  result. So accidental acceleration degrades to correct CPU execution, always.

Put together, vehje can offer authors a `[Differentiable]`-grade "mark it and it works, no theory required"
experience, and additionally guarantee that the marked-or-accidental result stays sound, stays inside the
consumer's permitted effects, and stays correct even when the acceleration heuristic guesses wrong. That is a
safer simplification loop than the one that prompted the question.

**And there is a deterministic-accelerated path, which closes the loop with the rest of this document.** A
consumer that needs both acceleration and determinism (a networked simulation, a replay-grade engine script)
can include a *deterministic* accelerator backend: fixed reduction order plus arvo fixed-point arithmetic
instead of vendor floats, at a performance cost, so even accidental acceleration is bit-reproducible where the
language requires it. This is the same arvo-fixed-point-for-determinism move the policy model uses, applied to
the compute itself, and it means the effect axis can distinguish a `Writes<DeterministicAcceleratorEnv>` from a
`Writes<FastAcceleratorEnv>` and let a consumer pick, statically checked, which one its programs may reach. The
simplification loop, the accelerator exposure, the determinism gate, and the fixed-point substrate are one
coherent story.

**Honest caveats.** The auto-offload heuristic must be conservative (offload only clearly dense-parallel
subgraphs; leave irregular control flow on the CPU where it belongs, per amendment 1), and the language surface
should make the parallel-friendly patterns the natural, easy path so accidental acceleration is common and
accidental slow-divergent-kernels are hard to write. The transformation being certified is not automatic; it is
work, the same certified-generation work the panel scoped, applied to a rewrite pass rather than a checker. And
this remains a maturity feature riding a founding seam: the macro codegen, the effect axis, the family model,
the value channels, and the lease axis already make the loop expressible, so the founding move is to keep the
attribute-to-macro-to-certified-transformation path clean and general, and the specific `[Accelerated]` and
`[Differentiable]` families get built when a consumer wants them.

**The bottom line for this angle.** Yes, vehje can have a `[Differentiable]`-style loop, and it is not a bolt-on:
it is the macro codegen recognised as an attribute-triggered certified transformation, generalised to any
theory-heavy capability, made accidental through the natural array API, and made safe on three axes (permission,
certification, best-effort-with-fallback) that Slang's version does not have. The author marks an attribute or
just uses the tensor type, understands none of the acceleration, autodiff, or vendor-codegen theory, and gets a
program that is accelerated where profitable, sound always, permitted always, and deterministic where the
consumer demands it. That is the simplification loop, and vehje's architecture was already shaped to make it
both trivial and safe.

### Amendment 3 sources

- The direct prompt: Slang's `[Differentiable]` attribute and its automatic reverse-mode source-to-source
  autodiff (Bangaru et al., "SLANG.D: Fast, Modular and Differentiable Shader Programming," SIGGRAPH Asia 2023).
- The strongest general analog: JAX's composable function transformations `grad` (autodiff), `vmap`
  (auto-vectorisation), and `jit` (auto-compile-to-accelerator via XLA), each a certified program transformation
  that synthesises a theory-heavy capability from ordinary array code (Bradbury et al., JAX).
- Autodiff-by-annotation at the compiler level: Enzyme (Moses and Churavy, NeurIPS 2020), differentiating
  arbitrary LLVM IR by marking, as the "mark it, get the derivative" precedent one level below a source language.
- Accidental parallelism/acceleration via annotation or natural API: OpenMP and OpenACC pragmas, C++
  `std::execution::par` policies, and Futhark (a functional array language that auto-offloads to GPU), as the
  "use the natural primitive, get the acceleration" model.
- Halide's algorithm-and-schedule separation and Triton's kernel DSL, as the ergonomic-accelerator-authoring
  prior art already cited in amendment 2.
- The macro-codegen-as-IR-to-IR-transformation, the effect-axis gating, the certified-transformation guarantee,
  and the arvo-fixed-point deterministic path are vehje's own settled design (the three-codegens topic
  `202607201513`, the effect model in `202607192358/`, this panel's certified-generation resolution, and the
  fixed-point substrate), composed here into the simplification loop.
