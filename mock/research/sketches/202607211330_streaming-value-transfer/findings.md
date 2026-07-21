# Streaming value-transfer (expansion): bounded residency for huge outputs

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 | artifact `stream.zig`
**Settles:** 1315's streaming-with-backpressure for outputs that exceed a residency budget.

## Result
A 5.86 MB value (488,281 records, a depth-8 fanout-5 record tree) is emitted depth-first children-first through a
single 4 KB host-lent window. The window fills, `commit`/flush fires (the host consumes the chunk, residency
resets), and emission continues. Peak resident = 4092 bytes (<= the 4096 budget), 1432 flushes. So a value 1430x
the budget streams in 4 KB chunks with peak residency = the budget (peak/total = 0.0007).

## Reading
Bounded residency for enormous outputs is confirmed: the value never materialises whole. Depth-first
children-first emission means a parent record is emitted after its children (already flushed), so a cross-chunk
reference is to an already-committed child (the cross-chunk-implies-promoted property, SK21). The flush is the
backpressure point (a real sink blocks in `reserve` until the host drains). This is exactly 1315's streaming
model, and it means a full-program output or a deeply nested value crosses to the host in bounded memory
regardless of its total size.

## Design impact
The value transport handles arbitrarily large outputs in bounded residency: stream in whole-subtree chunks
through the reserve/commit sink, peak memory = the host-lent budget, not the total value. Combined with the
value-arena (SK20), the typed decode (SK18), and the sink (SK21), the transport is complete: small values cross
in one window, huge values stream in bounded chunks, and the untrusted path validates each chunk. No consumer's
output size can exhaust runtime memory.
