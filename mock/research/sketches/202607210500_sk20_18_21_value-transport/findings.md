# SK20 / SK18 / SK21 findings: the value-transport subsystem

**Date:** 2026-07-21
**Outcome:** WORKS (arena encode/decode, typed structural decode, reserve/commit sink + zero-copy wire form).
**Settles:** the value-arena format (1315), the untrusted-load typed decode (2055 A6 / Cluster C), and the
reserve/commit sink + cross-chunk lemma (1315/1845), on the pinned toolchain.
**Toolchain:** Zig 0.16.0. Artifacts: `arena.zig`, `demo.zig`, `sink.zig`.

## SK20: the value-arena (encode/decode, children-first, no back-patch, zero-copy)

Fixed-width 16B records + a flat `[]u32` child-index pool + a byte blob, all relative (backward) indices.
Emission is children-first, so a parent's child node-indices are already known when the parent record is emitted:
no back-patching, confirmed (the encoder never revisits an emitted node). A zero-copy reader walks the arena
without deserialising (sums scalar leaves to 117 over a `record{10, record{3,4,"hi"}, 100}`, correct). The three
regions plus relative indices mean the bytes ARE the wire form.

## SK18: the typed structural decode (untrusted load path)

A single linear pass over an untrusted arena validates completeness: every record's child indices are strictly
backward (`< own index`), which is acyclicity BY CONSTRUCTION (a forward or out-of-range child fails); every pool
span and blob ref is in range; and depth is bounded by a linear DP over the backward DAG (no recursion). Confirmed:
the trusted arena validates true; a corrupted arena with a forward child index (a cycle / OOB) validates false; a
blob overrun validates false. This is parse-don't-validate: complete, linear, and it feeds the engine only
range-typed facts. It is NOT a fixpoint (a single fold), confirming the 2055 A6 resolution.

## SK21: the reserve/commit sink + the zero-copy wire form + cross-chunk

The sink is the C-ABI shape from 1315: a `#[repr(C)]`/`extern struct` of two function pointers (`reserve(hint) ->
[*]u8`, `commit(n)`) plus opaque userdata. A host implements it with a lent buffer and a write cursor (the
backpressure point is inside `reserve`). Serialising the arena's three regions through the sink writes 76 bytes;
reading them back reinterprets the host bytes as the arena regions ZERO-COPY (sum=12, valid=true), so the same
relative-indexed bytes are the in-process representation and the wire form, no pointer fixups.

The cross-chunk-implies-promoted property is structural: a chunk boundary sits only at a whole-subtree root, so
within a chunk all child ids are strictly backward, and a cross-chunk reference is by construction to an
already-committed promoted root. The same backward-index invariant SK18's decode checks is what enforces it, so
the cross-chunk lemma (1845 flagged build-blocking) holds by the chunker's construction, not as a trailing proof.

## Load-bearing finding: zero-copy read requires aligned regions

Reinterpreting the wire bytes as the `Node`/`u32` regions requires those regions to be aligned (the type system
rejects an align-1 reinterpret; an `@alignCast` is valid only because the host buffer is page-aligned and the 32B
header keeps the node region 8-aligned). So the format MUST guarantee region alignment (align the buffer, pad the
header/regions to the max element alignment). This is exactly the alignment-and-stride-predictable requirement the
SIMD structural pass (2001, simdjson) already mandates, so the two constraints are one: fix the wire format with
aligned, padded regions, decided now.

## Design impact
- The value-arena, the typed decode, and the sink are all buildable and no-alloc-shaped on the pinned toolchain.
- Fix the wire format with aligned/padded regions (serves both zero-copy read and the SIMD decode).
- The cross-chunk lemma is a construction property enforced by the backward-index invariant, not a separate proof.
