# SK25 findings: C-host panic-free arm (embeddability contract)

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 + zig cc | artifacts `vehje_rt.zig`, `host.c`
**Settles:** the embeddability contract (a C host embeds the Zig runtime over the C ABI; no panic crosses),
folding with SK7.

## Result
A Zig runtime function `vehje_decode(ptr, len, out) -> c_int` exported `callconv(.c)`, compiled to an object,
linked into a C program via `zig cc`. The C host calls it with good input (rc=0, out=0x1234), out-of-range input
(rc=2), and truncated input (rc=1). All three return error codes; the C host survives every call. No panic
crosses the C ABI boundary.

## Reading
This is the concrete embeddability demonstration of the two-artifact model: the Zig composed runtime exports a C
ABI, an arbitrary host (here C) embeds it, errors are return codes, and no Rust and no panic exist at the embed
site. It confirms SK7's discipline (error unions internally, error CODES at the C ABI) end to end across a real
foreign host, which is the load-bearing property for "embeddable, no compiler/runtime at the embed site."

## Design impact
The runtime ABI surface is panic-free by construction (error codes at the boundary, error unions inside). A C
(or any C-ABI) host embeds the runtime safely. This is the embeddability identity validated with a real foreign
caller.
