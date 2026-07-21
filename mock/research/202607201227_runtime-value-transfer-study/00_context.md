# Context: how do mature runtimes transfer output values across the embed boundary?

## What vehje is

vehje is an **embeddable IR framework** (not a language). Front-ends (a "Grammar") lower source
into a small closed **Core IR** of 11 node forms (Lit, Var, Let, Lambda, Apply, Project, If, Match,
Iter, Interp, Raw). A back-end ("Target") either emits native code or the IR is serialized to a
tier-tagged **residual** and handed to a small **runtime** (written in Zig, compiled both as a static
lib `libvehje_runtime.a` for in-proc embedding and as a standalone exe `vehje-runtime`).

The whole stack is `#![no_std]`, **no alloc** (compile passes included). The IR lives in
caller-provided bump-arena regions. The residual tier-0 format is a flat serialized IR arena (fixed
seven-word node records + a flat child-index pool + a self-contained string blob), already
implemented at `mock/crates/vehje-runtime-abi/src/wire.rs` and `encode.rs`.

Two consumption paths:
- **path 1 (standard):** emit residuals, run the `vehje-runtime` exe as a subprocess (e.g. from a
  Deno/TS host).
- **path 2 (optional):** in-proc Rust bindings crate (`vehje-runtime-driver`) links the `.a` over a
  3-fn C ABI (`vehje_runtime_new` / `_free` / `_execute` returning i32).

## The open problem (what this research informs)

The runtime **interprets** a residual and produces an **output value**. That output can be huge and
deeply nested: full-program outputs, structs upon structs upon structs, arbitrarily large trees.
Two naive designs were both rejected as untenable:

- **out-buffer value:** the runtime writes the whole result into a caller-provided byte buffer. Too
  naive; unbounded size; hard to grow the design later.
- **retained handle + accessors:** the runtime keeps the value graph alive and hands back an opaque
  handle the host reads through accessor calls. Introduces lifetime/ownership concerns and would
  later force a copy-free disk serializer to bound residency.

Both are unoptimal, and this project cannot ship unoptimal. We need a more sophisticated,
first-principles design for the value-transfer model at the runtime/host boundary. Constraints that
shape it: no_std / no_alloc discipline, bounded memory residency even for enormous outputs, copy-free
where possible, must work for BOTH an in-proc `.a` embedding and a subprocess exe (where "the
boundary" is a pipe / file / shared region, not a function return), and it must be extensible to
future tiers (bytecode, native) without a redesign.

A decided sub-point: the **runtime is a dumb index evaluator**. The compile side pre-resolves `Var`
to indices; the residual carries indices, and any name blob is debug-only. So the value model does
not need name resolution at runtime.

## The research question (neutral)

Study how mature embeddable runtimes and VMs actually solve the value-transfer boundary between the
VM's internal value representation and an embedding host: **how a produced value crosses out, how
huge/nested values are handled without unbounded copies, who owns the memory, and what the lifetime
contract is.** Report the concrete mechanism (named APIs, the actual data model), *why* each design
ended up where it did (the hard-won lesson, including what earlier approaches they abandoned and
why), and the costs/tradeoffs. Reach conclusions from the primary sources; ground claims with real
API names, doc/spec links, and where possible file/line or spec-section citations.

Each agent covers one runtime family and writes its findings to its own file in this directory. A
synthesis pass afterward pulls the cross-cutting lessons and maps them onto the vehje boundary.
