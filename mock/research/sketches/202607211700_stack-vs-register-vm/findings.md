# Register-VM vs stack-VM interpreter bytecode (bench-driven fork resolution)

**Date:** 2026-07-21
**Type:** Zig-native microbench, ReleaseFast, 4M arithmetic expressions `((k0*k1)+k2)` each, 7-run best,
switch dispatch (per BN1). Zig 0.16.0, aarch64. Artifact: `vm.zig`.
**Settles:** the interpreter bytecode's operand model. BN1 settled dispatch (switch beats tail-threading) and
cfg-interp settled the CFG-of-blocks model, but not the register-vs-stack fork, which is the single biggest
interpreter-design lever (Lua moved from a stack VM to a register VM between 5.0 and 5.1 for exactly this reason).

## The fork

- **Register VM:** instructions name their operand registers, and an operand may be a register OR a constant
  (Lua's RK encoding: high bit of the operand byte selects the constant pool). So `((k0*k1)+k2)` is 3
  instructions: `MUL r0, K0, K1` / `ADD r0, r0, K2` / `ACC r0`. Fewer, fatter instructions; a wider decode (each
  operand may be a register or a constant); values live in a register file with no push/pop traffic.
- **Stack VM:** instructions are operand-less and work on a value stack. The same expression is 6 instructions:
  `PUSHK k0` / `PUSHK k1` / `MUL` / `PUSHK k2` / `ADD` / `ACC`. More, thinner instructions; every op touches the
  stack pointer, forming a serial dependency chain (each op reads the slot the previous op wrote).

## Results

Both VMs computed identical results (`chk=299200000` each, correctness confirmed):

| VM | instr/expr | ns/instr | ns/expr | code bytes/expr |
|---|---|---|---|---|
| register (RK) | 3 | 1.01 | **3.03** | 12 (3 x 4B) |
| stack | 6 | 0.94 | 5.65 | 48 (6 x 8B, this encoding) |

## The finding: register VM is ~1.86x faster per expression; recommend it for the interpreter tier

The register VM evaluates each expression in **3.03 ns vs the stack VM's 5.65 ns, 1.86x faster**. The mechanism is
exactly the Lua 5.0-to-5.1 result: the register VM executes HALF the instructions (3 vs 6), because direct
operand addressing folds the stack VM's separate PUSH instructions into the arithmetic instructions' operands.
The per-instruction cost is nearly equal (register 1.01 ns, stack 0.94 ns): the register VM's slightly higher
per-instruction cost from RK-operand decoding (a branch on the constant bit per operand) is more than repaid by
halving the instruction count. The stack VM's per-op push/pop and its stack-pointer dependency chain roughly
cancel the register VM's wider decode, so the win comes down to instruction count, and the register VM issues
half as many.

For vehje's interpreter tier this points one way: **use a register VM**. It matches the other interpreter findings
(switch dispatch, NaN-box values, the fast CFG-of-blocks floor) that all push the interpreter floor to be fast
enough that the native tier is a ceiling rather than a necessity. A 1.86x per-expression speedup on the
arithmetic hot path is the largest single interpreter lever measured tonight.

## Code size: encoding-dependent, the classic counter-consideration

The table shows the register code 4x SMALLER, but that is an artifact of this bench's stack encoding (8 bytes per
instruction, a `u32` arg field padded). The honest, classic tradeoff is the opposite on compactness: a tightly
encoded stack VM uses 1-byte opcodes with a separate operand stream, so its bytecode is typically MORE compact on
disk than a register VM's (register instructions carry three operand bytes each). With 1-byte stack opcodes this
program would be ~6 bytes/expr for the stack VM vs ~12 bytes/expr for the register VM, reversing the size column.

So the real tradeoff is: **register wins execution speed (fewer instructions), stack wins raw bytecode
compactness (thinner instructions).** For vehje the speed matters more (the interpreter floor is the thing being
kept fast), and the compactness gap is addressable separately: register bytecode compresses well (the operand
bytes are low-entropy and repetitive), and the cross-mod streaming/mmap story already handles size. The register
VM's larger in-memory footprint is also mitigated by the 24-byte node record decision (BN2): the same inline-
operand form serves the register bytecode, so there is one operand model across the arena and the VM.

## Design impact

- The interpreter tier is a register VM with RK operands (register-or-constant), switch dispatch, NaN-box values,
  over the CFG-of-blocks model. Every interpreter-tier bench now converges on the same shape.
- Bytecode compilation (the cheap-lowering stage) emits register instructions directly; the register allocation
  is a simple linear scan within a block (the block-local value lifetimes the cfg-interp model already carries),
  not a full graph-colouring allocator, so the compile cost stays in the cheap-lowering budget.
- Compactness is a compression/streaming concern, not an argument for a stack VM: the 1.86x execution win is not
  worth trading for smaller bytecode when the size gap closes under compression and the mmap/streaming path
  already bounds resident size.

## Boundary

Arithmetic-dense workload (the interpreter hot path). A control-flow-heavy or call-heavy program shifts more cost
onto dispatch and the frame stack (measured in cfg-interp), where the register/stack operand model matters less;
but arithmetic and value-shuffling dominate real script bodies, so the 1.86x is representative of the hot path.
The register file here is small (16 registers); a program needing more than fits would spill, but block-local
register pressure in a scripting language is low (the cfg-interp block model keeps live sets small).

## Artifacts
- `vm.zig` (register-VM with RK operands + stack-VM, same `((k0*k1)+k2)` program compiled to both, instruction
  count + timing + code size for each).
