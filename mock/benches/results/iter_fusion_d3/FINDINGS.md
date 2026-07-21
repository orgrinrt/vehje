# iter_fusion_d3


## Measured results

Ratio to baseline (iterfuse_push3), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | iterfuse_mat3 (ratio) | iterfuse_pull3 (ratio) | iterfuse_push3 (base) |
|---|---|---|---|
| 64 | 1.80x | 1.05x | 783 ns |
| 256 | 1.95x | 1.27x | 2898 ns |
| 1024 | 1.70x | 1.19x | 12498 ns |
| 4096 | 1.17x | 1.08x | 141073 ns |
| 16384 | 1.31x | 0.99x | 894552 ns |

## Cost-model sanity line

At n=16384, the baseline (iterfuse_push3) median is 894552 ns for push3: N elements through a 3-stage fused pipeline. Treating n as the work-item count, that is 54.60 ns/item, about 174.7 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

At depth 3, push and pull tie (within noise at large n); materialize costs 1.2x to 1.95x. Deeper fusion chains favor push/pull equally over materializing intermediates. No n=4096 anomaly here (unlike depth 2).

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
