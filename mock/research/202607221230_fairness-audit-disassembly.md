# Fairness audit: ISA-level disassembly of every dispatch cell

**Date:** 2026-07-22
**Status:** audit evidence, part of the composition-matrix machinery build (topic
`202607221700_topic.bench-composition-matrix-synthesis.md`, the fairness-gate mandate). Records the
disassembly confirmation that every dispatch cell's ISA-level codegen matches its label, and that no cell
gets an accidental advantage. Growing document: extended as new dispatch cells land. No benches run.

## Method

A dedicated `mock/benches/disasm-probe/` cdylib exports each interpreter as a non-inlined
`#[no_mangle] extern "C"` symbol (`di_switch`, `di_fntable`, ...), built with the exact variant profile
(`opt-level=3`, `lto="fat"`, `codegen-units=1`) so the codegen matches what a real variant would measure.
`llvm-objdump -d` disassembles the dylib; each symbol (and each internal preserve-none handler) is
classified by the instructions that distinguish the dispatch shapes on aarch64:

- `br xN` (register-indirect branch) is the jump-table tail or a guaranteed tail call.
- `blr xN` (register-indirect call) is a function-pointer-table indirect call.
- `stp x19..x28` (store-pair of a callee-saved register) is a callee-saved spill; a preserve-none handler
  must have none across its dispatch.

The probe is an audit tool, not a bench; it is never timed.

## Per-cell ISA confirmation

Measured on the built `libdisasm_probe.dylib` (M1, nightly-2026-05-28, variant profile):

| cell | label | ISA evidence | verdict |
|---|---|---|---|
| switch (`interpret`) | jump table | `di_switch`: one `br xN` (indirect jump through the table), zero `blr` | CONFIRMED |
| flat switch (`interpret_predecoded`) | jump table | `di_pre_switch`: one `br xN`, zero `blr` | CONFIRMED |
| fntable (`interpret_fntable`) | indirect call, survives LTO | `di_fntable`: one `blr xN`, zero `br` | CONFIRMED (not devirtualized) |
| bit-tree (`interpret_bittree`) | balanced comparison tree | `di_bittree`: zero `br`, zero `blr` (pure compare + conditional branch) | CONFIRMED |
| threaded wire (`interp_threaded`) | preserve-none context-threaded | all 17 handlers `h_const..h_input`: `spills=0`, one tail `br` each | CONFIRMED, no ABI fallback |
| threaded flat (`threaded_flat`) | preserve-none context-threaded | all 17 handlers: `spills=0`, one tail `br` each | CONFIRMED, no ABI fallback |
| threaded direct (`threaded_direct`) | preserve-none, handler ptr per node | all 17 handlers: `spills=0`, one tail `br` each | CONFIRMED, no ABI fallback |

The single most important confirmation: `rust_preserve_none_cc` is an incomplete nightly feature, and the
worry (topic Part 0) was that it might silently fall back to the standard ABI on some handler, in which case
the threaded cells would be paying callee-saved spills and measuring something that is not preserve-none.
The disassembly shows **zero callee-saved spills across all ~51 handlers of all three threaded families**,
every one ending in exactly one tail `br`. The mechanism is real everywhere; the threaded cells are the
shape they claim to be.

## CFG (control-flow) dispatch cells

The three CFG dispatch shapes run a register VM of blocks over the nested-loop and
branchy kernels (real loops and branches, which the straight-line cells lack). All three
reach the register file through the same `access::rload` / `access::rstore` as the
straight-line cells, so the CFG axis varies dispatch alone. Measured on
`libdisasm_probe.dylib` (`di_cfg_switch`, `di_cfg_fntable`, and the `cfg::threaded`
handlers):

| cell | label | ISA evidence | verdict |
|---|---|---|---|
| CFG switch (`cfg::interp`) | match dispatch, compare chain | `di_cfg_switch`: zero `br xN`, zero `blr` (the 5-arm op match lowers to a compare/cmov chain, not a jump table) | CONFIRMED (compare chain, not jump table) |
| CFG fntable (`cfg::interp_fntable`) | indirect call, survives LTO | `di_cfg_fntable`: one `blr xN`, zero `br` | CONFIRMED (not devirtualized) |
| CFG threaded (`cfg::threaded`) | preserve-none context-threaded over control flow | all 8 handlers `spills=0`, `blr=0`; the 7 continuing handlers (`h_set/add/sub/mul/and/jmp/brnz`) end in one tail `br`; `h_ret` is terminal (`br=0`) | CONFIRMED, no ABI fallback |

Two honest notes, not defects:

- The CFG switch is a compare chain, not a jump table, because it dispatches only 5
  register ops. The straight-line switch dispatches 17 IR opcodes and lowers to a jump
  table (`br`). Both are legitimate `match` dispatch; the ISA realization tracks the arm
  count, and the label is recorded as what it is per cell (jump table for the 17-op
  straight-line switch, compare chain for the 5-op CFG switch), never conflated.
- `cfg::threaded`'s `h_jmp` and `h_brnz` are the load-bearing addition: a loop back-edge is
  a `become` into an earlier flat index and a branch is a `become` into one of two
  targets, so the branch-heavy kernels exercise the indirect tail transfer that the
  straight-line threaded cells cannot. Their `spills=0` + single tail `br` confirms the
  control-transfer handlers are still preserve-none, no std-ABI fallback on the branch.

The CFG switch and fntable cells were refactored to the shared `access` primitive as part
of this cell (they previously used checked `regs[i]` array indexing); the refactor closes
the same check-vs-no-check confound at the source for the CFG axis that the fidelity gate
closed for the straight-line axis. Register indices are `< NREG` by construction of the
kernel builders, so the unchecked access is sound.

## No accidental advantage (the check-vs-no-check class)

The confound both prior audits found (threaded used unchecked raw pointers while switch/fntable used checked
slice indexing) is closed at the source by the fidelity gate: every dispatch shape reaches operands and
stores results through the one shared `access::rload` / `access::rstore` (unchecked, sound post-validation).
So the operand-access code is identical across cells and the only thing the dispatch axis varies is
dispatch. This is source-verified (one primitive, used by every interpreter) and consistent with the
disassembly (the handlers' operand loads are plain `ldr`, no bounds-check compare-and-branch in any cell).

The `stp x19..x28` counts on the `di_*` WRAPPER symbols (3 to 6 each) are the wrapper's own frame doing the
`Vec` allocation, wire parse, and post-pass checksum, not the dispatch loop, and are identical in kind
across every wrapper; they are excluded from the dispatch comparison (the wrapper is audit scaffolding, not
a measured cell).

## Cells whose fairness is not a dispatch-label question

- Stages (`optimize`, `fusion`, `liveness`): program-to-program or execution-shape transforms, not dispatch
  shapes. Their fairness is semantic preservation, verified by cross-validation on the live-out (sink) fold
  across all six profiles and many seeds, plus the shared access primitive in their interpreters.
- Vertical/SoA SIMD: a genuinely different evaluation shape (one dispatch over W inputs). Its advantage
  (dispatch amortised over W) is the real property under test, not an artefact; cross-validated per lane.
- regcache: uses the shared access primitive; its per-operand compare (cache hit test) is a real cost of
  the technique, faithfully represented.
- Trace / superblock (`trace`): a control-flow structure transform (linearize the hot self-loop into a
  guarded superblock), not a dispatch shape. Its fairness is that it walks the same path as the base CFG
  interpreter, verified by cross-validation on the full `(result, ninstr, nterm)` triple (identical instr and
  terminator counts prove the same back-edges and branches), plus the shared access primitive in both. There
  is no dispatch-label question: the win it measures (dropping the per-iteration block index and terminator
  match on the hot path) is a real property of linearization, not an artefact.
- Copy-and-patch (`copypatch`): native machine-code generation. There is no dispatch at all (that is the
  point), so there is no dispatch-label or check-vs-no-check question. The fairness contract is purely
  semantic: the generated aarch64 code fills the identical `results` array as the interpreter, verified
  byte-exact against the switch interpreter across all six profiles and many seeds (the profiles draw the
  full opcode vocabulary, so every op's stencil is exercised; a wrong encoding on any op would diverge the
  checksum). The cost it measures is the honest one: a large codegen `S` term (one emit pass per node) buys a
  near-zero-dispatch native `I` term. The imm12 window (nodes and consts below 4096) is a stated scope of
  this realization, and `emit` declines rather than miscompiles above it.

## Zig (cross-language) dispatch cells

The Zig cell consumes the identical REC24 wire bytes and folds the identical checksum as the Rust
interpreters, cross-validated byte-exact through the `zigcheck` CLI (a Rust test pipes `[seed][program]` and
compares checksums across all six profiles and five seeds; `zigcheck` also asserts the two Zig shapes agree
internally). Measured on `libcarrier_zig.dylib` (M1, Zig 0.16, ReleaseFast):

| cell | label | ISA evidence | verdict |
|---|---|---|---|
| Zig switch (`carrier_zig_switch`) | match dispatch, jump table | one `br xN` (the 17-arm switch jump table), zero `blr` | CONFIRMED (jump table) |
| Zig tail (`carrier_zig_tail`) | guaranteed-tail-call token threading | every opcode handler (`hConst`, `hAdd`, `hMul`, `hSelect`, `hInput`, ...) ends in one indirect `br` (the threaded dispatch through the handler table, `step` inlined into each) with `bl=0`, `blr=0`, `ret=1` | CONFIRMED, true tail threading |

The load-bearing confirmation: `@call(.always_tail)` lowered to a genuine indirect tail branch (`br`) in every
handler, with **zero regular calls** (`bl`/`blr`) across the dispatch. So the Zig tail cell has no stack growth
and no call overhead per node, the token-threaded shape Rust needs the nightly `become` feature to express.
This is the one dispatch shape a Rust-only carrier cannot ask, and the shipped vehje runtime is Zig, so the
cross-language cell is load-bearing rather than a curiosity. Its operand access is the same wire-format read as
the Rust switch (both index the REC24 record inline), so the cross-language comparison is dispatch plus the
Zig-vs-Rust codegen difference, faithfully what a Zig runtime would pay.

## Review-driven additions and corrections (2026-07-22)

An independent machinery review (`202607221930_carrier-machinery-review-haoran-xu.md`) re-derived the ISA
evidence and found asymmetries the whole-symbol classifier above structurally could not catch. The corrections
that landed, with their own ISA evidence:

- **CFG fn-table dispatch paid two dead operand loads per SET.** The old `CFn = fn(u64, u64, u64)` forced the
  fntable call site to load `regs[a]`/`regs[b]` for every instruction including SET (which ignores them),
  because the indirect call could not prove the callee discards them. `di_cfg_fntable` showed two unconditional
  `ldr` before the `blr`; switch and threaded paid none. Fixed: `CFn = fn(&Instr, *const u64)` so each op loads
  only what it needs, mirroring the straight-line fntable.
- **The if-chain axis was never ISA-checked, and it collapses to a jump table.** `di_ifchain` shows the
  SimplifyCFG lowering (`ldrb w0, [x12, x16]` byte-index table load, then `br x17`), an O(1) computed jump like
  switch, so on the natural cell the frequency-ordering is moot. Added `interpret_ifchain_linear` (each `opcode
  == k` behind a `black_box` barrier): `di_ifchain_linear` has **0** computed `br xN` and **63** `cmp`/`b.eq`
  cascade branches, a genuine linear scan where frequency-ordering carries weight. Both are kept: the natural
  cell answers "what a source if-chain actually costs once compiled," the linear cell "what the textbook
  technique costs in isolation."
- **Vertical/SoA SIMD had zero ISA confirmation** and is the cell most weight rests on. Added `di_vertical4` /
  `di_vertical8`. Both show genuine packed NEON in the dispatch loop (`add.2d`, `sub.2d`, `cmhi.2d`, `cmeq.2d`,
  `ushl.2d`, `neg.2d`, `bsl.16b`, `bif.16b`; W=8 doubles W=4's op counts), not W duplicated scalar sequences.
  The auto-vectorizer did not silently scalarize it, so its "SIMD win" number is real.
- **The native ceiling folded its checksum inline** (`native_madd`, two rotate-xor per chain step) while every
  interpreter had moved its fold post-pass, loading the denominator every ratio normalizes against with
  overhead the numerators no longer paid. Fixed: `native_madd` writes `results[]` with no fold, a driver folds
  once, matching the interpreter cost shape exactly. Not an ISA question, a cost-shape parity one.
- **Copy-and-patch was direct instruction-selecting codegen, not the stencil mechanism it cited.** The old cell
  is relabelled `copypatch` (direct native codegen, per-node isel, the near-native comparator). The real
  copy-and-patch is now `stencil.rs`: toolchain-assembled per-op stencils, extracted via linker labels, copied
  and hole-patched per node with no isel. Byte-exact against the interpreter and the direct cell across every
  opcode. The cost-shape difference (copy+patch vs isel per node) is what the two cells now measure apart.

Per-opcode note: the reproduce script below classifies whole symbols (`stp`/`br`/`blr` counts). It cannot see a
per-opcode load-count asymmetry inside an otherwise-fine symbol (the CFG SET finding) or a cascade that became a
jump table (the if-chain finding, whose `br=1` reads the same as a real jump table). A stronger audit diffs the
per-opcode instruction schedule across cells; the two findings above were found by reading the specific opcode
paths by hand, and that generalization is the recommended next audit method.

## Not yet built / flagged

- perfect-hash dispatch: degenerate to the fn-pointer table for the dense contiguous opcode set (0..16); a
  representative perfect-hash cell needs a sparse opcode design decision (panel / op). A degenerate cell
  would misrepresent the technique, so it is not built.
- copy-and-patch / stencil above the imm12 window (nodes/consts >= 4096): a register-materialized base address
  is the labelled refinement; both native cells decline rather than miscompile.

## Reproduce

```
cd mock/benches/disasm-probe && cargo build --release
llvm-objdump -d target/release/libdisasm_probe.dylib | \
  awk '/^[0-9a-f]+ <.*>:/{n=$0;s=0;t=0} /\tstp\t(x19|x20|x21|x22|x23|x24|x25|x26|x27|x28)/{s++} /\tbr\tx/{t++} /\tret/{if(t>0)printf "spills=%d %s\n",s,n}'
```
