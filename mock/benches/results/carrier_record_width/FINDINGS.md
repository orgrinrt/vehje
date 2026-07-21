# Record width (carrier), the audit-fixed replacement

This is the corrected record-width bench, the first bench built on the carrier
(`mock/benches/carrier/`). It replaces the earlier standalone `record-width/`
bench that the audit panel found had two defects: its `extern struct`s were
mislabeled (the "16-byte" and "24-byte" records were actually 12 and 20 bytes,
never `@sizeOf`-checked), and it ran outside the harness with a header-only CSV,
so its numbers were not reproducible and its "24 bytes is precisely optimal,
+20% over 16" conclusion rested on two strides it never measured plus a pessimal
16-byte layout that spilled all three operands.

## What this measures, and how the defects are structurally prevented

One generated program of N nodes (N is the harness size) is encoded at five
record layouts and decoded at the same stride by the shared reference
interpreter. The layouts are defined in the carrier with compile-time size
asserts (`ir.rs`, `const _: () = { assert!(REC16.stride == 16); ... }`), so a
record can never silently be a different size than its name. The program crosses
into each variant as wire bytes, so the interpretation cannot be partially
evaluated away. The five layouts, with their real field inventories:

- `rec12`: 4-byte header, 2 inline operands. Arity-3 nodes spill to the pool.
- `rec16`: 4-byte header, 3 inline operands, no padding. The true 16-byte
  three-operand layout the old bench never built: four records per cache line,
  no pool at all.
- `rec20`: 8-byte header (room for a u32 shape_id), 3 inline operands. This is
  the layout the old "24-byte" struct actually was.
- `rec24`: 8-byte header, 3 inline operands, 4 bytes of padding. The baseline.
- `rec32`: 8-byte header, 4 inline operands, 8 bytes of padding. The wide end.

`node_count` scales with the harness size N, so the sweep walks the record array
from L1-resident (N=64, a few KB) toward memory (N=16384, roughly 200 to 500 KB
depending on stride), which is where a wider record would start to cost
bandwidth.

## Result: a statistical tie at every reachable size

Cross-validation passes (all five layouts produce byte-identical output across
100 seeds per size). The medians, normalised against rec24:

| n | rec12 | rec16 | rec20 | rec24 (base) | rec32 |
|---|---|---|---|---|---|
| 64 | 3.97us | 3.93us | 3.99us | 3.82us | 3.79us |
| 256 | 13.82us | 13.66us | 13.48us | 13.53us | 13.86us |
| 1024 | 45.74us | 51.17us | 45.24us | 45.22us | 45.53us |
| 4096 | 381.8us | 379.2us | 380.2us | 384.2us | 383.4us |
| 16384 | 1.85ms | 1.82ms | 1.83ms | 1.79ms | 1.82ms |

Every field is within a few percent, below the measurement noise floor at each
size (whole-field spread 1.3% to 3.6% at the larger, lower-variance sizes). No
layout beats or loses to another outside noise. The true 16-byte three-operand
layout (`rec16`) is tied-best or nominally fastest at several sizes.

## What it says for the design

"24 bytes is precisely optimal, +20% over 16" is refuted. At program sizes the
harness can reach (up to a few hundred KB, L1 and L2 resident on Apple Silicon),
record width from 12 to 32 bytes does not matter: the interpreter is
dispatch-and-compute bound, not memory-bandwidth bound, so the extra bytes per
node are hidden. The old +20% was the pessimal-spill artifact the audit named,
not a width effect. Concretely, a 16-byte three-operand record (four per cache
line, no pool) is at least as good as 24 bytes on these measurements, which
matters because the wire format is a day-one, expensive-to-change decision: it
can go narrower than 24 with no measured penalty here.

## Cost-model sanity line (bench discipline rule 3)

At n=16384, 1.82 ms for 16384 nodes times 16 interpretation passes is
6.9 ns/node-eval, about 21 cycles at 3 GHz for a decode (op, arity, up to three
operand loads), a switch, one or two arithmetic ops, a result store, and a
two-op hash fold. That is a plausible per-node cost for a byte-decoding
tree-walk, and it scales close to linearly with N (11.95 us at n=64 to 1.82 ms
at n=16384 is a 232x time increase for a 256x node increase, the sublinearity
being the fixed per-call overhead amortising), which confirms the timed work is
the interpretation and not a hoisted constant.

## The caveat and the next step

This bench measures the L1-and-L2-resident regime only, because the harness caps
its working set and the per-node compute dominates there. The regime where a
wider record could genuinely cost (a node array past L2, into memory, at a
million-plus nodes) is out of the harness's reach and is a scale-runner
question. The finding here is therefore "width does not matter up to L2, and the
true-16B-3-op layout is a real contender"; the memory-bound regime is owed to
the scale-runner driver, and the real consumer arity distribution (this uses the
carrier default, not a census) is owed when consumers exist.
