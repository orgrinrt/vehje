# Dual-locus single-shared-object (expansion): the counter-audit's resolution, concrete

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 + zig cc + rustc | artifacts `engine.zig`, `rust_host.rs`, `c_host.c`
**Settles:** the dual-locus identity divergence (Cluster C Finding 2), by the counter-audit's return-to-canon fix.

## Result
The ONE hand-authored Zig engine is compiled ONCE to a C-ABI static library (`libvehje_engine.a`). A Rust dev-side
host and a C runtime-side host both link that SAME object and call it. Both produce bit-identical output
(8337503924153436829).

## Reading
This is the counter-audit's dual-locus resolution made concrete: instead of two backends (a Rust generator
emitting specialised Rust source AND specialised Zig source, which could deterministically diverge), there is ONE
compiled engine that both loci link. Identity is bit-identical BY CONSTRUCTION because it is the same object, not
tested for agreement. The dev-time Rust locus links the compiled Zig engine over FFI (exactly as the counter-audit
described); the runtime locus (here a C host, standing in for the composed Zig runtime) links the same object.

So Cluster C's dual-locus divergence concern dissolves by returning to the canonical single-engine shape (1627/1845):
Rust emits validated DATA, the hand-authored Zig engine (comptime-specialised to that data) is compiled once, and
both loci link it. The differential-test harness (SK23) remains as defence in depth, but it is not the primary
assurance a two-backend design would be forced to lean on.

## Design impact
The engine is one compiled object, linked by both loci, bit-identical by construction. No two-backend divergence,
no merge-gate mitigation needed. This is the return-to-canon the counter-audit and the identity-recenter both
pointed to, validated with a real Rust+C cross-link.
