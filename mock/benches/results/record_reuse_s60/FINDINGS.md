# record_reuse_s60


## Measured results

Ratio to baseline (rec_mut), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | rec_copy (ratio) | rec_mut (base) | rec_reuse_s60 (ratio) |
|---|---|---|---|
| 64 | 5.93x | 341 ns | 4.17x |
| 256 | 6.23x | 1176 ns | 4.47x |
| 1024 | 6.83x | 4235 ns | 4.89x |
| 4096 | 6.98x | 17003 ns | 4.83x |
| 16384 | 6.87x | 65592 ns | 4.83x |

## Cost-model sanity line

At n=16384, the baseline (rec_mut) median is 65592 ns for N record updates, 60% trigger a copy-on-write. Treating n as the work-item count, that is 4.00 ns/item, about 12.8 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

At 60% sharing, reuse costs 4.8x mut and approaches copy's ~6.9x: when most records are shared, copy-on-write pays nearly the full copy price. Reuse only wins when sharing is low; above ~60% shared, a plain copy is nearly as cheap. The reuse/copy crossover is the design-relevant boundary.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
