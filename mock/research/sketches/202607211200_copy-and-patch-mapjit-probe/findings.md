# Copy-and-patch MAP_JIT probe (expansion): native-tier mechanism feasible on macOS

**Date:** 2026-07-21 | **Outcome:** WORKS (on aarch64 macOS, JIT-permitted) | zig cc + macOS JIT APIs | `probe.c`
**Settles:** the native-tier core mechanism feasibility (Carmack break 3, Cluster B two-gate model gate-2).

## Result
The minimal copy-and-patch pipeline runs end to end on aarch64 macOS: `mmap(MAP_JIT)` allocates an executable
mapping, `pthread_jit_write_protect_np(0)` makes it writable, an aarch64 stencil (`add x0,x0,#HOLE; ret`) is
written, its immediate HOLE is PATCHED to 42, `pthread_jit_write_protect_np(1)` makes it executable,
`sys_icache_invalidate` flushes the icache, and calling `f(100)` returns 142 (the patched stencil). exit 0.

## Reading
- **The core copy-and-patch mechanism is feasible** on a permitting platform: MAP_JIT, the per-thread W^X toggle,
  the immediate-hole patch, and the arm64 icache invalidation all work. Writing a template + patching a hole +
  executing is the copy-and-patch primitive, confirmed.
- **Carmack break 3 (W^X reality) is real and handled.** The native tier needs MAP_JIT + the write-protect toggle
  + icache invalidation (all present here). On a JIT-permitted process it works; on iOS App Store / consoles /
  hardened processes without the entitlement, the `mmap(MAP_JIT)` (or the toggle) would FAIL, which is exactly
  what Cluster B's gate-2 run-time probe catches -> tier-down to the interpreter floor (fast per BN1/CFG-interp).
- **The two-gate capability model is validated:** the run-time executable-mapping probe succeeds here (native
  available), and would fail-and-tier-down on restricted platforms. The interpreter floor (1.7 ns/instr, BN1/CFG)
  handles those, so no consumer is broken by the native tier's absence.

## Scope (honest)
This proves the core mechanism (patch + execute a stencil) and the platform reality. It does NOT build the full
stencil-EXTRACTION toolchain (compiling each op to a CPS stencil, parsing object files, building patch tables),
which is the deferred weeks-of-object-format-work. But the hard platform question (does MAP_JIT copy-and-patch
work at all, and what happens where it does not) is answered: yes where permitted, tier-down where not.

## Design impact
The native tier's core mechanism is feasible on permitting platforms (macOS validated), and the two-gate model
(build-time profile + run-time probe -> tier-down) is the right shape, with the fast interpreter as the floor
everywhere. Native stays a ceiling/accelerator (SP7): it works where it works, and the floor covers the rest. The
stencil-extraction toolchain is the remaining build work, gated on the coverage experiment (SP1).

## Addendum: stencils compose by concatenation (the codegen half)
`chain.c` extends the probe: 4 single-instruction add-stencils (each `add x0,x0,#imm`, imm patched) are
CONCATENATED plus a final `ret`, and executing the composed code gives `f(1000)=1121` (1000 + 3+7+11+100). So a
lowered program is a concatenation of patched stencils, executed as one function, which is the copy-and-patch
codegen mechanism. Straight-line concatenation (fall-through stencils) works directly; control-flow transfer
between stencils (patch the next-continuation address, the register-indirect tail transfer from Cluster B) is the
additional mechanism for branches, not exercised here. Core codegen (patch + concatenate + execute) confirmed.

## Addendum 2: control-flow inter-stencil transfer (register-indirect, patched address)
`transfer.c`: stencil A (`add x0,x0,#5`) tail-transfers to stencil B (`add x0,x0,#10; ret`) via a register-
indirect branch whose target ADDRESS is patched in (`movz/movk x9, #<B's address>; br x9`), giving `f(0)=15`.
This is Cluster B's mechanism that dodges the aarch64 `CALL26`/`JUMP26` +-128MB branch-range limit: the next
stencil's address is a patched 64-bit immediate loaded into a register, then `br`, so stencils can be anywhere.

## Native-tier copy-and-patch: all three core mechanisms confirmed
1. Single stencil patch + execute (probe.c) -- MAP_JIT + W^X toggle + icache + immediate patch.
2. Concatenative composition (chain.c) -- a program = concatenated patched stencils.
3. Control-flow transfer (transfer.c) -- register-indirect `br` to a patched next-stencil address.
All work on aarch64 macOS. The native tier's core codegen is feasible; only the stencil-EXTRACTION toolchain
(compiling ops to CPS stencils, parsing object files, building patch tables) remains as engineering (the deferred
weeks-of-work), gated on the SP1 coverage experiment. Feasibility of the native accelerator is de-risked.
