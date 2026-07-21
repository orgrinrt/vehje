# The Runtime Execution Layer: Native Tier, Stencil Toolchain, and Interpreter Hot Loop

**Date:** 2026-07-20
**Phase:** research (Cluster B of the Carmack-rebuttal debate, `202607202205_topic.carmack-rebuttal-full-design-not-retier.md`)
**Scope:** design the runtime execution layer the arc has not written: the native tier as runtime code
generation with a full platform-capability model (problem 3), the stencil extraction toolchain specified and
budgeted (problem 4), and the interpreter hot loop at the weight the lease axis got (problem 5). Builds on
Cluster A (`202607202210_design_dual-locus-relational-engine.md`) for the load-verify residual engine whose
per-load cost is budgeted here against ikiuni's per-frame budget.
**Answers:** problems 3, 4, 5 of the rebuttal agenda, inside the maximal shape (native tier fully specified
across platforms, a first-class interpreter floor, the stencil toolchain), never a reduced one.
**Source design:** topics 2001 (converged shape, copy-and-patch native tier), 1513 (native as a target not a
tier, the three codegens), 1315 (the value-arena the interpreter walks); the Carmack reaction
`202607202122` (breaks 3, 4, and the interpreter-hot-loop and platform-capability items in What is missing);
Cluster A `202607202210`.

## Verdict

The runtime execution layer holds at full strength with one framing inversion the Carmack reaction already
forced and this design makes structural: the interpreter is the certified floor present and identical on every
platform, and copy-and-patch native is a per-platform accelerator gated by two independent checks (a
build-time capability profile per composed runtime plus a run-time executable-mapping probe), with a tested
tier-down to the interpreter that preserves certified generation by construction, because both the interpreter
arm and the stencil arm are projections of the one semantic definition and compute the same function on the
same residual bytes. The stencil toolchain is not open-ended research; it is a bounded port of CPython 3.13's
shipping `Tools/jit` design into Rust driving the LLVM object machinery that Zig already ships, with per-target
relocation tables as the real work, and it lands in a named trusted computing base bound to the
semantic-definition hash by manifest. The interpreter hot loop is the piece that actually decides vehje's
per-frame identity, and its design falls out of decisions the value-arena already made: children-before-parents
emission plus backward-only child indices turn tree evaluation into a single forward linear scan over a
constant-stride arena, dispatched by Zig `@call(.always_tail)` tail-call threading with the walk state pinned
in registers, small operands inlined into a 16-byte node record so the hot binop case costs zero pool
indirections. The Cluster A load-verify residual engine runs once per arriving script at load, off the frame
budget entirely; the only thing the per-frame budget sees is this interpreter walk, which is exactly what the
hot-loop design minimizes. No demotion anywhere: the native tier is fully specified where platforms permit and
tiers down cleanly where they do not, and the tier-down is a designed, tested path, not an assumption.

## Problem 3: The native tier as runtime code generation, with a platform-capability model

### The constraint restated at the machine level

Copy-and-patch writes machine-code stencil bytes into a memory region and then executes that region. From the
operating system's view this is indistinguishable from any other just-in-time code generator: it needs a
memory mapping that is at some point writable and at some later point executable. Every hardened platform
enforces W^X (write XOR execute): no mapping is both writable and executable at the same instant, and mapping
executable memory at all requires a permission the embedder may not hold.
[CPython's PEP 744](https://peps.python.org/pep-0744/) confirms this is exactly how a shipping copy-and-patch
JIT is constrained: the design ensures "at no point is the data both writable and executable," and on macOS
"it appears that macOS releases should enable the JIT Entitlement for the Hardened Runtime." The vehje slogan
"no compiler at the embed site" is satisfied by copy-and-patch; the slogan it does not satisfy is "no JIT
permissions at the embed site," and that second one is the one that bites on the platforms a game-embedded
runtime targets.

The per-platform reality, which drives the whole capability model:

| Platform class | Executable mapping | Mechanism | Native tier |
|---|---|---|---|
| macOS (Apple Silicon, Intel) | permitted with entitlement | `MAP_JIT` mmap, per-thread `pthread_jit_write_protect_np` toggle, `com.apple.security.cs.allow-jit` under hardened runtime, `sys_icache_invalidate` after patch on arm64 | yes, run-time probed |
| Linux, BSD (desktop, server) | usually permitted | `mmap(PROT_READ\|PROT_WRITE)` then `mprotect(PROT_EXEC)`, or a dual-mapping of one physical page (one RW view, one RX view) | yes, run-time probed |
| Windows (x64, arm64) | usually permitted | `VirtualAlloc(PAGE_READWRITE)` then `VirtualProtect(PAGE_EXECUTE_READ)`, `FlushInstructionCache` | yes, run-time probed |
| Android | usually permitted (ART itself JITs) | as Linux; SELinux `execmem` domain may deny | yes, run-time probed |
| iOS, tvOS, watchOS (App Store) | forbidden to third parties | only WebKit holds the `dynamic-codesigning` entitlement; `allow-jit` is not granted to App Store apps | no, interpreter floor |
| Consoles (PS5, Switch, Xbox) | forbidden | dynamic code execution prohibited by platform policy and cert requirements | no, interpreter floor |
| Runtime hosted inside wasm | impossible | wasm cannot map executable memory without host cooperation | no, interpreter floor |

The platforms in the bottom three rows are not edge cases for a game scripting runtime: iOS and the three
consoles are among the primary ship targets ikiuni cares about, and they are exactly the platforms that forbid
the native tier. This is why the interpreter cannot be a fallback bolted on after the JIT; it is the product,
and native is the accelerator that exists on the platforms that permit it.

### The two-gate capability model

Availability of the native tier is decided by two independent gates, both of which must pass, because one is a
property of the shipped binary and the other is a property of the running process.

The first gate is a **build-time capability profile per composed runtime**, keyed to the target triple. When
the language compiler performs runtime generation (topic 1513) for a target platform whose class categorically
forbids executable mapping (iOS App Store, the consoles, a wasm host), the stencil table is simply not linked
into the composed runtime. That runtime is interpreter-only, and it is smaller for it: it ships no stencil
bytes, no patcher, and no platform mapping shim, which also removes those from its trusted computing base and
removes a certification red flag on platforms whose review process scrutinizes any executable-memory use.
Shipping stencil bytes to a console is dead weight the platform holder will reject; the build profile is where
that is decided, once, per target.

The second gate is a **run-time executable-mapping probe**, performed once at composed-runtime initialization on
any platform whose build profile did link the stencil table. Even on macOS, Linux, Windows, and Android, the
specific process may run under a policy that denies executable mapping: a hardened-runtime macOS process
without the `allow-jit` entitlement, a Linux process under an SELinux policy with `execmem` denied or a seccomp
filter rejecting `mprotect(PROT_EXEC)`, a Windows process with Arbitrary Code Guard enabled, an Android app on
a device with a restrictive SELinux domain. The probe attempts the full sequence on a single throwaway page:
map a one-page JIT region, write a trivial stencil (a function that returns a sentinel), toggle it executable
(the `pthread_jit_write_protect_np` toggle on macOS, the `mprotect` transition elsewhere), call it, and check
the sentinel. If any step fails with `EPERM`, `EACCES`, an exception, or a wrong result, the native tier is
disabled for this process and every script executes on the interpreter. This is the tested tier-down path the
Carmack reaction demanded ("graceful tier-down must be a tested path, not an assumption"): it is one real
mapping attempt executed and verified at init, not an inference from platform identity, so it catches the case
where the platform class permits JIT in general but this deployment forbids it in particular.

The two gates are orthogonal to a third axis that is not a capability question at all: **heat**. Once the
native tier is available (both gates pass), Deegen's tiered discipline decides which code gets stencils: hot
functions are stenciled, cold code stays interpreted, tiering is driven by an execution counter
([Deegen, Xu and Kjolstad, arXiv:2411.11469](https://arxiv.org/abs/2411.11469)). If the native tier is
unavailable (either gate fails), everything interprets. Separating platform capability (gates one and two)
from optimization policy (heat) keeps the tier-down clean: an unavailable native tier is exactly a heat
threshold of infinity, and the interpreter path is unchanged whether native is absent by platform or merely
not-yet-hot.

### Why tier-down preserves the certified-generation guarantee

The certified-generation guarantee is a property of the residual, not of how the residual executes. The
family, effect, and lease proofs are discharged in Rust at dev time for bundled scripts, and by the Cluster A
load-verify residual engine at load for arriving scripts, before any execution strategy is chosen. The residual
bytes are identical across strategies. What differs between the interpreter and the native tier is only the
machine that walks those bytes, and the two machines are provably the same function because both are generated
from the one semantic definition: the Deegen triple emits, per family operation, an interpreter arm, a
copy-and-patch stencil, and a differential test, all from one source (topic 2001 stage 4;
[Deegen](https://arxiv.org/abs/2411.11469) is the existence proof that interpreter and baseline JIT generated
from one definition agree). So tier-down from native to interpreter changes the executor, never the computed
result, and the certified-generation guarantee (which is about what the residual means, discharged before
execution) survives untouched. A script that verified safe at load runs the same observable behavior on the
interpreter as it would have on stencils; the tier-down is a performance event, never a correctness event.

One honest weighting the Carmack reaction is right about and this design carries forward: the measured wins for
baseline copy-and-patch JITs on dynamic languages are real but modest without inline caches and type feedback,
and vehje's residual is already statically checked flat IR, so the interpreter overhead a baseline JIT removes
is smaller here than for a Lua or a Python whose interpreter re-checks types every op. The native tier's win
concentrates in dispatch-bound arithmetic inner loops, where burning operands and the next-op address directly
into instructions removes the dispatch and the operand-fetch indirection
([Copy-and-Patch, Xu and Kjolstad, OOPSLA 2021, arXiv:2011.13127](https://arxiv.org/abs/2011.13127) reports
generated code an order of magnitude faster than interpretation and 14 percent faster than LLVM -O0, and 39 to
63 percent over Chrome's Liftoff baseline on Coremark and PolyBenchC). That the interpreter is where the
per-frame ceiling is most likely decided (problem 5) is not a reason to demote the native tier; it is a reason
to design the interpreter to the same weight, and to gate the stencil TCB's cost on a bench that shows the
native tier beats the tail-call interpreter by enough to justify it (open question for Cluster C).

## Problem 4: The stencil extraction toolchain, specified and budgeted

### The template calling convention

A stencil is one family operation compiled to machine code with holes where runtime values must be inserted
([Copy-and-Patch, arXiv:2011.13127](https://arxiv.org/abs/2011.13127): stencils are binary implementation
variants "with holes where missing values must be inserted during code generation"). To make stencils
composable by simple concatenation, each op is written once, generated from the one semantic definition (the
same source that generates the interpreter arm), as a continuation-passing-style function: it performs the op,
then tail-calls a continuation function pointer that patching will resolve to the next stencil. Two calling-
convention properties are load-bearing and both are settled precedent in CPython 3.13:

Guaranteed tail calls (`musttail`) so the inter-stencil transfer compiles to a `jmp`, not a `call`, giving no C
stack growth across a chain of arbitrarily many ops. [PEP 744](https://peps.python.org/pep-0744/) states clang
is required precisely because it "is the only C compiler with support for guaranteed tail calls (`musttail`),
which are required by CPython's continuation-passing-style approach," without which "the tail-recursive calls
between templates could result in unbounded C stack growth."

A caller-preserves-nothing convention (`preserve_none`, the LLVM `ghccc` lineage) so the live VM state (the
node pointer, the results pointer, the value-stack top) stays pinned in fixed registers across every dispatch
with no spill or reload. The [musttail interpreter work, Haberman 2021](https://blog.reverberate.org/2021/04/21/musttail-efficient-interpreters.html)
shows why this matters beyond stencils: splitting an interpreter into per-op tail-called functions lets the
compiler allocate registers for each op independently, so a cold slow-path op cannot spill the hot path's
registers, which is the effect that produced protobuf parsing over 2 GB/s.

The stencil templates are generated in a C or Zig form the toolchain compiles with `clang -O2`, preserve_none,
musttail, one function per op and per specialization (an op may have several stencils: an integer-add variant
and a float-add variant, an arity-2 and an arity-N call variant). Generating the templates from the one
semantic definition is what keeps the stencil arm and the interpreter arm the same function; the templates are
not hand-written assembly.

### The extraction pass

Extraction is a build-time step in the Rust dev-time compiler layer, run once per language per target triple,
using the LLVM object machinery that Zig already ships (Zig bundles clang and LLVM, so the object parser and
disassembler are present at our build without a new dependency). The pass, cloning CPython's `Tools/jit` shape:

1. Emit the template source for every op and specialization from the one semantic definition.
2. Compile each with `clang -O2 -fno-asynchronous-unwind-tables`, preserve_none, musttail, targeting the triple.
3. Parse the resulting object file per its format (Mach-O, ELF, or COFF), locating each op function's `.text`
   byte range and any `.rodata` it references. [PEP 744](https://peps.python.org/pep-0744/) confirms LLVM is
   used here for "object file parsing and disassembly."
4. Read the relocation records against each op function and classify each into a hole kind (the next-continuation
   address, an immediate operand, a constant-pool pointer, a host-function id).
5. Serialize each op's stencil into the stencil-table data blob: the code bytes, the rodata, and the patch
   table, content-addressed, one table per target triple, with a manifest entry binding the semantic-definition
   hash to the stencil-table hash.

The output ships as static read-only data in the composed runtime; the extraction pass and the clang it drives
never run at the embed site. This is the native-tier analogue of the content-validation build step already
blessed in topic 2001, and it inherits that topic's hermetic-build precedent (the Nix and Bazel lineage) for
why a pinned build-time toolchain is a trusted computing base member: its inputs and toolchain are fixed.

### The patch-table format

Per stencil, a compact record: `{ code_offset, code_len, rodata: bytes, holes: [Hole] }`, where each hole is
`{ offset, kind, selector }`. The `kind` is the target relocation kind (below); the `selector` names what
fills the hole at patch time: `NEXT_CONTINUATION` (the address of the next stencil in the emitted chain),
`IMMEDIATE(operand_slot k)` (a small operand read from the residual node record), `CONST_POOL(idx)` (a pointer
into the emitted constant pool), or `HOST_FN(id)` (a resolved host-call trampoline). At emit time the patcher
copies the stencil's code bytes into the code buffer, then for each hole writes the resolved value at
`code_offset + offset` using the relocation kind's encoding.

Burning the operand and the next-op address into the instruction stream is exactly what buys the win over
interpretation: [Copy-and-Patch, arXiv:2011.13127](https://arxiv.org/abs/2011.13127) and
[PEP 744](https://peps.python.org/pep-0744/) both attribute the speedup to moving "the values or addresses of
arguments, constants, and cached values directly into machine instructions" and off heap-allocated frames into
registers.

### Per-target ABI edge cases

The relocation kinds are the real per-target work, and they are the bulk of the toolchain's cost:

- **x86-64**: `R_X86_64_64` (absolute 64-bit immediate), `R_X86_64_PC32` and `R_X86_64_PLT32` (PC-relative
  32-bit for near calls and data references). A 64-bit immediate fits one `mov`; instruction cache is coherent
  so no explicit flush is needed after patching.
- **aarch64**: a 64-bit immediate needs the four-instruction `movz`/`movk` sequence
  (`R_AARCH64_MOVW_UABS_G0`..`G3`), and PC-relative page addressing needs the `adrp`/`add` pair
  (`R_AARCH64_ADR_PREL_PG_HI21` plus `ADD_ABS_LO12_NC`). Branch relocations (`CALL26`, `JUMP26`) reach only
  plus or minus 128 MB, so a stencil calling a host function farther than that needs a veneer, or the transfer
  is made register-indirect. The instruction cache is not coherent with the data cache on arm64, so every
  patched region needs `sys_icache_invalidate` (macOS) or the equivalent before execution.
- **Object formats**: Mach-O prefixes symbols with an underscore and encodes relocations differently from ELF;
  COFF differs again and carries `.pdata`/`.xdata` unwind data on Windows. The extraction pass normalizes all
  three into the single patch-table hole-kind vocabulary above.
- **Unwinding and stack**: templates are compiled `-fno-asynchronous-unwind-tables` and carry no C frame
  (preserve_none, no prologue or epilogue), so they keep the host's stack pointer and 16-byte stack alignment
  and add no unwind info. The cost is that a native crash or a signal handler walking through stenciled code
  has no unwind metadata, a debugging burden noted in the TCB.

The design preference that dodges the worst edge cases: make every inter-stencil transfer register-indirect (a
`preserve_none` tail call through a register holding the next stencil's address), so the `CALL26`/`JUMP26`
range limit never applies and stencils are position-independent by construction. The next-continuation hole is
then a register load of a 64-bit address (the `movz`/`movk` sequence on arm64, a single `mov` on x86-64), not
a relative branch.

### The budget

The "weeks of grubby object-format work in the TCB" the Carmack reaction filed is real but bounded and mostly a
port, not open-ended research. CPython 3.13 shipped this exact design in a `Tools/jit` driver of a few thousand
lines of Python calling LLVM; the vehje version is that driver rewritten in Rust calling the Zig-shipped LLVM
object API. The one-time cost concentrates in the per-arch relocation tables (roughly six kinds for x86-64,
roughly ten for aarch64) and the three object formats; the ongoing cost is additive per new target triple. The
concrete framing of the budget, produced by designing the thing rather than asserting a duration: it is a port
of a shipping reference implementation whose hardest parts (the CPS template convention, the object parse, the
relocation classification) are already solved and documented in that reference, plus the vehje-specific
manifest-hash binding of stencils to the semantic definition. That binding is the one genuinely new piece and
it is cheap: a content hash checked at runtime-generation time, the same certified-generation identity
mechanism Cluster A uses for the dual-locus engine.

### Placement in the trusted computing base

The native tier adds three members to the named TCB (topic 2001 already names the load verifier and, from
Cluster A, the decode-to-EDB boundary): the stencil compiler (the clang at our build, trusted by the hermetic-
build precedent), the patcher (the runtime code that writes holes into the code buffer, trusted because it
executes what it writes), and the platform mapping shim (the `MAP_JIT`/`mprotect`/`VirtualProtect` wrapper).
The stencil-table data is not trusted by assertion: it is content-addressed and manifest-checked against the
semantic-definition hash at runtime-generation time, so a corrupted or mismatched table is caught before it is
ever patched or executed. The interpreter is not in this addition; it is the reference semantics from which the
stencils are generated and the certified floor the whole tier-down rests on.

## Problem 5: The interpreter hot loop

This is where vehje's per-frame performance identity is decided, before any native tier exists, and it is the
floor present on every platform including the ones that forbid stencils. It is owed the weight the lease axis
got, and unlike the lease axis its best design falls almost entirely out of decisions the value-arena already
made.

### The arena walk is a forward linear scan, not a tree recursion

The single most consequential fact, and it is a gift from topic 1315's emission discipline: the residual is
emitted depth-first, children before parents, and child indices are relative and point strictly backward (a
child's node id is strictly less than its parent's, which is also the acyclicity-by-construction the typed
structural decode relies on, topic 2055 A6 and Cluster A section 5). Therefore a single forward linear pass
over the node array, from index 0 to the last node, evaluates every node after its children have already been
evaluated. There is no tree recursion, no explicit work stack, no pointer chasing, and no interpreter call
stack. The interpreter is a loop `for n in 0..len { dispatch(node[n]) }`, and when it evaluates node `n` it
reads the already-computed results of nodes at indices less than `n` from a parallel results array indexed by
node id.

The cache behavior this produces is the best an interpreter can have. The node array is scanned strictly
front-to-back, one sequential stream the hardware prefetcher handles perfectly. The results array is written
strictly front-to-back, a second sequential stream. The only non-sequential accesses are the backward reads
into the results array for a node's children, and those target recently written entries that are still in L1 or
L2. This is the A-normal-form or SSA linear-scan evaluation order, and it is available for free because the
residual is already in that shape (topic 2001 stage 2 normalizes to A-normal form, and the arena emits
children-first). Contrast the naive fixed-width-record-plus-child-pool interpreter the Carmack reaction warned
about, where "every operand access is one extra indirection" and evaluation is a recursive tree walk chasing
child indices in random order: that interpreter pointer-chases and mispredicts; this one streams.

### Dispatch shape: tail-call threading via Zig `@call(.always_tail)`

Three dispatch shapes, from slowest and most portable to fastest and most demanding:

A `switch` over the op tag compiles to a single function with one bounded indirect branch. It is the portable
floor and it mispredicts: a single dispatch site cannot learn per-op-pair correlations, and the one giant
function defeats the register allocator. It is the last-resort shape for a target whose toolchain supports
nothing better.

Computed-goto (direct or token threading, GCC and Clang labels-as-values) gives each op its own `goto
*dispatch[next_op]` site, so the branch target buffer can learn per-op-pair patterns. [Ertl and Gregg's work on
interpreter dispatch](https://en.wikipedia.org/wiki/Threaded_code) established that threaded dispatch reduces
branch mispredictions relative to switch, with the win architecture-dependent (their measurements show the best
threading variant changing by microarchitecture). The classic figure is a large fraction, up to roughly 2x on
dispatch-bound loops on older cores, narrowing to tens of percent on modern cores with better branch
predictors.

Tail-call threading (the Deegen and CPython route) makes each op a separate function ending in a guaranteed
tail call to the next op's handler, with a caller-preserves-nothing convention. Zig expresses this directly
with `@call(.always_tail, next_handler, args)`, and because Zig lowers through LLVM it can reach the same
`musttail` plus preserve_none code the CPython JIT and the protobuf parser use. This shape wins over
computed-goto for the register-allocation reason the [musttail
work](https://blog.reverberate.org/2021/04/21/musttail-efficient-interpreters.html) names: each op function is
optimized independently, so a cold op cannot spill the hot path's registers, and the VM state stays pinned in
registers across every dispatch. [Deegen](https://arxiv.org/abs/2411.11469) generates exactly this shape and
its generated interpreter runs 31 percent faster than LuaJIT's hand-written assembly interpreter and 179
percent faster than PUC Lua, which is the strongest evidence that generated tail-call threading is not merely
competitive with expert assembly but ahead of it.

The decision: tail-call threading in Zig via `@call(.always_tail)` is the primary dispatch shape, generated
from the one semantic definition (topic 2001 stage 4), with computed-goto and switch as generated fallbacks the
build selects per target when the toolchain cannot guarantee the tail call. The linear-scan structure makes the
"next handler" trivial: it is the handler for `node[n+1]`, so the tail-call chain is the forward walk itself,
and preserve_none pins the three walk pointers (node, results, blob) in registers for the entire residual. The
measured difference to expect, stated so the bench has a target: switch as baseline, computed-goto roughly plus
15 to 30 percent, tail-call threading roughly plus 30 to 100 percent on dispatch-bound behavior scripts where
the op body is a few instructions and dispatch dominates. The exact number is workload-dependent and must be
benched on the census corpus and on real ikiuni behavior-script shapes per the bench-in-harness discipline; the
literature fixes the ordering and the rough magnitude, not the vehje number.

### Value-node operand packing: the 16-byte record with small operands inlined

The Carmack reaction's specific fix, "small-operand inlining into the node record is the standard fix and it is
a wire-format decision," is the design that removes the per-operand indirection. The node record is fixed-width
at 16 bytes, aligned to 16 bytes, constant stride:

`[ op: u16 ][ flags: u8 ][ arity: u8 ][ operand0: u32 ][ operand1: u32 ]`

Each operand slot is a tagged 32-bit word: a 3-bit tag in the high bits selects `IMM_INT` (a small immediate
integer, sign-extended), `CONST_IDX` (an index into a float or wide-constant pool), `CHILD_IDX` (a backward
node id whose already-computed result is read from the results array), `POOL_SPAN` (an offset and count into
the flat child-index pool, for arity greater than two), or `BLOB_REF` (an offset and length into the byte
blob); the low 29 bits carry the payload, and 29 bits of child index covers 512 million nodes, far past any
input-length bound `N` a script reaches.

The consequence: an op of arity at most two, which is the overwhelming majority (binops, unops, variable
references, literals, comparisons, the arithmetic that dominates a game behavior-script inner loop), carries
both operands inline in the record and costs zero pool indirections. Only arity greater than two (calls,
record construction, varargs) spends operand1 as a `POOL_SPAN` into the flat child-index pool, one indirection
amortized over many children. This is the standard "inline the small case, spill the wide case to a side pool"
of production bytecode VMs (LuaJIT's fixed 32-bit instruction with inlined operand fields, Dalvik's register
operands, CPython's inline caches), applied to the arena record.

Why 16 bytes and not 8: an 8-byte record `[op:u16][flags:u8][arity:u8][operand0:u32]` holds one inline operand,
which forces a pool indirection on every binop, the exact cost the fix exists to remove. Sixteen bytes buys two
inline operands and covers binops with zero indirection, at the cost of doubling arena size, still cache-
friendly at four nodes per 64-byte line. For an arithmetic-heavy per-frame workload that is the right trade. A
variable-width encoding (LEB128-style operand packing) is rejected outright: it would shrink the arena but
destroy the constant stride that makes the walk a branch-predictable, prefetchable, SIMD-decodable linear scan,
trading the per-frame identity away for arena size, the wrong direction for the piece that decides per-frame
cost.

### Region tags without back-patching

Carmack break 5 (the shared-node region tag) interacts with this layout and is resolved cleanly by keeping
region assignment out of the record. The IR-residual node needs no region tag at all: the lease and region
proofs are discharged before the residual exists, so the interpreter node carries only op and operands. The
value-arena node (the produced value crossing back through the sink) does carry region information, and it is
kept in a side structure, a region-id column indexed by node id, finalized at chunk close, exactly as break 5
prescribed ("nodes carry a region id that indexes a region table, finalised at chunk close, nodes never touched
twice"). The region id is the `flags` byte when the live-region count fits in 256 (bounded by environment-width
`W`), or a parallel `[u16; N]` column when `W` exceeds 256. Either way the 16-byte record is never back-patched;
only the side column is written at chunk close, which preserves the no-back-patching property the streaming sink
requires (topic 1315).

### The Cluster A residual engine's per-load cost against the per-frame budget

The load-verify residual engine from Cluster A runs once per arriving script at load, not per frame, and this
is the fact that makes the per-frame budget tractable. When a mod or an arriving script is first loaded (at
level load or mod init, not in the frame loop), the runtime runs the structural decode (linear, SIMD-friendly,
O(N) over arena size), the arriving-value lease residual (the reachability closure, bounded to `W` rounds and
`N*W` tuples, Cluster A section 2), and the numeric residual (a tnum fixpoint bounded by lattice height). All of
that is bounded by the three structural constants and is a one-time load-time cost measured in the load budget
of seconds, which O(N) linear decode trivially fits. Its verdict, safe-or-unsafe plus the region tags, is baked
into the residual once; the engine does not re-run per frame.

What the per-frame budget sees is only the interpreter walk of the already-verified residual (or its stenciled
native form on platforms that permit it). At 60 frames per second the budget is 16.6 ms. A game runs on the
order of hundreds to low thousands of script invocations per frame (per-entity behavior scripts); each
invocation is a forward linear walk of a small residual. With tail-call threaded dispatch at roughly one to
three nanoseconds per node, a hundred-node behavior script costs roughly 100 to 300 nanoseconds, and a thousand
such invocations cost roughly 100 to 300 microseconds, on the order of one to two percent of the frame. The
load-verify residual engine contributes zero to this because it already ran at load. So the per-frame identity
is decided entirely by per-node dispatch cost and node count per script, which is exactly what the dispatch
shape and the small-operand packing minimize, and the Cluster A engine is a load-time concern sized by the
three bounds, off the frame budget by construction.

The Cluster A open question "how much of the engine ships" resolves for the game runtime as follows: the
composed runtime for ikiuni ships the load-verify residual subset (decode plus lease residual plus numeric
residual), not the full equality-saturation lowering engine, which stays dev-time for bundled scripts.
Arriving scripts (mods) are decoded, verified, and interpreted directly in their un-lowered form; the
optimization that lowering would buy them is deferred to heat-based stenciling on platforms that permit the
native tier, which is the larger win for a hot mod loop than eqsat lowering would be, and which requires no
lowering engine in the shipped runtime. This keeps the shipped runtime small and keeps the per-frame path free
of any fixpoint machinery.

## The wire-format decisions this forces, consolidated

These must be fixed now because they are in the serialized format and cannot change after scripts and value
buffers exist in the wild. They are the interpreter design's non-negotiable couplings to the arena.

The node record is fixed at 16 bytes, aligned to 16 bytes, constant stride, so node `n` is at `base + 16*n`,
pure index arithmetic with no per-node size table. This keeps the walk branch-predictable, keeps the typed
structural decode a constant-stride SIMD scan (topic 2055 A6, topic 1315 SIMD pass), and keeps the arena at
four nodes per cache line.

The record layout is `[op: u16][flags: u8][arity: u8][operand0: u32][operand1: u32]`, two inline operand slots.
Each operand slot is a 3-bit tag plus 29-bit payload over the tag vocabulary `IMM_INT`, `CONST_IDX`,
`CHILD_IDX`, `POOL_SPAN`, `BLOB_REF`. Arity greater than two spends operand1 as a `POOL_SPAN` into the flat
child-index pool.

Three contiguous regions, relative indices only, no absolute pointers: the node array (16-byte stride), the
child-index pool (a `[u32]` array), and the byte blob. This is the same zero-copy relative-indexed shape topic
1315 settled, so the same bytes are the in-process representation and the pipe wire form.

Emission is depth-first, children before parents, and child indices point strictly backward (child id less than
parent id). This is simultaneously the acyclicity-by-construction the decode relies on and the property that
makes evaluation a forward linear scan. It is not merely an optimization; the interpreter's whole cache story
depends on it, so it is a format invariant, not a producer convenience.

Region information lives in a side column indexed by node id, finalized at chunk close, never in the record, so
shared-node region assignment is a side-table write and the record is never back-patched. The column is the
`flags` byte when `W` is at most 256, else a parallel `[u16; N]`.

Overflow policy is per locus, and it must be stated in the format because it changes the widths chosen. At dev
time (bundled scripts) a script exceeding the reachability-bitmask width `W`, or exceeding a relation capacity,
is a hard error: the language author sees it and raises the width in the composed-runtime build. At load time
(arriving scripts) the same overflow demotes the overflowing references to the per-reference generational
residual (as if avoidance had failed, topic 2001 point 5) rather than refusing to load, so a mod does not brick
on a width the host chose. Never brick, always degrade, is the same principle as the native-tier tier-down.

## What remains genuinely original work

Every ingredient ships somewhere; the compositions below do not, and this is where the Cluster C honest-keeper
pass should attack.

The tail-call-threaded interpreter generated from the one semantic definition into Zig `@call(.always_tail)`
handlers with preserve_none-pinned state, walking a children-before-parents constant-stride arena with
backward-only child indices as a forward linear post-order evaluation. Deegen ships tail-call-threaded
generated interpreters, LuaJIT ships fixed-width inlined-operand bytecode, and A-normal-form linear evaluation
is textbook, but the composition (a linear forward-scan post-order evaluator over a zero-copy relative-indexed
arena that is simultaneously the wire format, generated into Zig from the same definition that generates the
stencils) is unbuilt.

The stencil extraction toolchain in Rust driving the Zig-shipped LLVM object machinery, emitting per-triple
patch tables bound by manifest hash to the semantic-definition hash. CPython's `Tools/jit` is the reference for
the extraction, but it is Python and CPython-specific; a reusable extractor keyed to an arbitrary generated-
language's templates, with the certified-generation binding of the stencil table to the semantic definition, is
new.

The two-gate platform-capability model (build-time profile plus run-time executable-mapping probe) with a
tested tier-down that provably preserves certified generation because both the interpreter and the stencil arm
are projections of one definition. No shipping JIT couples a categorical build-time capability profile to a
per-process run-time probe with a proven-equivalent interpreter floor generated from the same source; the
closest precedent (CPython's `PYTHON_JIT=0`) is a coarse on-off switch, not a certified tier-down.

The four-way reuse of one 16-byte record: it is the interpreter's operand encoding, the value-arena wire
format, the SIMD-decodable structural-verify input, and the linear-forward-evaluation order, all at once. Each
use has prior art; the single record serving all four with no impedance between them is the vehje-specific
synthesis, and it is what makes one format decision buy the per-frame path, the wire path, the verify path, and
the eval order together.

## Open questions for the honest-keeper pass (Cluster C)

The 16-byte record with two inline operands assumes arity at most two dominates ikiuni behavior scripts. If
calls and record construction (arity greater than two) are hotter than assumed, the `POOL_SPAN` indirection
lands on the hot path, and a 24-byte record with three inline operands may be the right width. This is
bench-decidable and must be measured on real behavior-script arity distributions before the format locks,
because the width cannot change afterward.

Zig `@call(.always_tail)` plus LLVM must actually guarantee the tail call on every ship target, including the
console toolchains, which may use restricted LLVM backends. If a console cannot guarantee the tail call, its
interpreter floor falls to computed-goto or switch, a measurable slowdown on exactly the platforms that also
forbid the native tier, a double hit on the platforms least able to absorb it. This must be verified per
console toolchain, and it is the strongest argument for sketching the dispatch shape on the real target
toolchains first.

Whether Zig exposes a preserve_none-equivalent calling convention or only the always-tail intrinsic. If Zig can
guarantee the tail call but not the caller-preserves-nothing convention, the walk state spills across dispatch
and much of the win evaporates; the mitigation (a hot state struct the compiler is coaxed to keep in registers,
or inline-asm register pinning) is uglier and less portable. This is a real risk to the whole dispatch design
and should be the first sketch of the runtime arc.

Whether the native tier actually beats the tail-call interpreter by enough to justify the stencil TCB and the
two-gate capability complexity, given the Carmack observation that vehje's residual is already statically
checked so the interpreter overhead a baseline JIT removes is smaller here than in a dynamic language. The
gating bench: stencil a hot residual, compare against tail-call interpreting the same residual, on ikiuni
behavior scripts. If the win is small (under roughly 20 percent), the native tier may be a desktop-and-macOS
luxury and the interpreter-only console and iOS builds lose little, which would re-weight how much of the
stencil toolchain budget is worth paying up front.

The region-id side column's width: does the live-region count `W` ever exceed 256 in real mods with wide
closure captures (Carmack break 6's case)? The live-region-width distribution should be measured in the same
instrumentation pass as the Cluster A avoidance-rate experiment, since it decides whether the region id fits the
`flags` byte or needs a parallel `[u16; N]` column.

Whether arriving scripts need a load-time lowering pass or whether decode-plus-verify-plus-interpret-directly,
with heat-based stenciling for hot loops, is enough. The lean is interpret-directly to keep the shipped runtime
free of the lowering engine, but if real mod inner loops are hot and un-lowered, the per-frame budget suffers on
platforms where those loops also cannot be stenciled (iOS, consoles). This must be validated against real mod
workloads before the shipped-runtime engine subset is fixed.

## See also

The rebuttal agenda this answers (topic 2205, problems 3, 4, 5, and the platform-capability and interpreter-
hot-loop items in the Carmack reaction's What is missing). Cluster A (`202607202210`) for the load-verify
residual engine whose per-load cost is budgeted here. The grounding topics: 2001 (converged shape, copy-and-
patch native tier, the Deegen triple), 1513 (native as a target not a tier, the three codegens, runtime
generation), 1315 (the value-arena, fixed-width records, flat child-index pool, byte blob, children-first
emission, the reserve/commit sink). Cluster C is the honest-keeper pass that reads this and Cluster A and
attacks the open questions above.

## Sources

- [Copy-and-Patch Compilation, Xu and Kjolstad, OOPSLA 2021, arXiv:2011.13127](https://arxiv.org/abs/2011.13127)
- [PEP 744, JIT Compilation (CPython 3.13 copy-and-patch), Brandt Bucher](https://peps.python.org/pep-0744/)
- [CPython 3.13 What's New, experimental JIT](https://docs.python.org/3/whatsnew/3.13.html)
- [Deegen, automated high-performance VM and baseline-JIT generation (LuaJIT Remake), Xu and Kjolstad, arXiv:2411.11469](https://arxiv.org/abs/2411.11469)
- [Parsing Protobuf at 2+GB/s: tail calls in C (musttail, preserve_none), Haberman, 2021](https://blog.reverberate.org/2021/04/21/musttail-efficient-interpreters.html)
- [Threaded code and interpreter dispatch (Ertl and Gregg lineage)](https://en.wikipedia.org/wiki/Threaded_code)

Two claims rest on domain knowledge rather than a fetched source and are flagged as such. The macOS
`MAP_JIT`/`pthread_jit_write_protect_np` per-thread W^X toggle, the iOS `dynamic-codesigning` restriction to
WebKit, and the console dynamic-code prohibition are stated from platform engineering knowledge; PEP 744
confirms only the macOS hardened-runtime JIT entitlement. Chris Fallin's weval (WebAssembly partial evaluation
by the first Futamura projection, specializing a wasm interpreter to a program to produce AOT wasm, reported in
the SpiderMonkey.wasm context) is cited from knowledge because the primary post did not fetch; it is used only
as the adjacent-technique reference for partial-evaluation-as-specialization, not as a load-bearing number.
