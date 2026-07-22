# Value-arena throughput: encode / zero-copy parse / interpret at scale

**Strength: measurement** (CNTVCT_EL0, 5 runs median; the interp checksum cross-validated across record
layouts). Ported from the old standalone Zig bench onto the carrier.

## The question

The wire-format value arena is built (encode), read back (parse), and consumed (interpret). What is each
stage's throughput at scale, and where does the pipeline spend its time?

## Result (8M-node program)

| layout | wire | encode | parse | interpret |
|---|---|---|---|---|
| rec16 | 128 MB | 69.9 ms (1.7 GiB/s) | ~0 (zero-copy) | 79.4 ms (1.5 GiB/s) |
| rec24 | 192 MB | 73.0 ms (2.4 GiB/s) | ~0 (zero-copy) | 77.7 ms (2.3 GiB/s) |

## The finding

Three facts, each design-relevant:

- **Parse is free.** `Decoded::parse` is zero-copy: it validates the header and wraps the byte arena without
  touching the node data, so it is below the timer resolution regardless of wire size. The design's zero-copy
  read path holds at 8M nodes; there is no deserialize cost to amortize.
- **Encode is the build cost, ~2 GiB/s.** Building the wire arena (children-first, from the in-memory program)
  runs at 1.7-2.4 GiB/s. This is the one-time cost of materialising the arena; it scales with wire size.
- **Interpret is latency-bound, not bandwidth-bound.** Interpreting takes the SAME wall time (~78 ms) for the
  128 MB rec16 arena and the 192 MB rec24 arena, so the GiB/s figure differs only because the wire sizes
  differ. This is the same result the record-width scale bench found: the interpreter is bound by the per-node
  dependency latency, not by streaming the wire, so a wider record costs no interpreter time (it only costs
  bandwidth headroom). It confirms, from the throughput side, that 16 B is as good as 24 B for the interpreter.

Design implication (op's call): the zero-copy parse is a genuine win (free read-back); the encode is the
arena's real one-time cost; and the interpreter's insensitivity to record width (latency-bound) means the wire
format should be sized for the encode/bandwidth budget, not for interpreter speed, where it does not matter.

## Cost-model / boundary

interpret at 78 ms for 8M nodes is ~9.75 ns/node, consistent with the arena-locality bench's ~9.8 ns/op and
the record-width scale bench's ~10 ns/node, all memory-latency-bound on the dependency chain. Boundary: encode
and interpret are the load-bearing stages; parse being zero-copy means the "parse throughput" is not a
meaningful number (it does no work), which is itself the finding.
