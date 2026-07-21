# record_reuse_s20


## Measured results

Ratio to baseline (rec_mut), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | rec_copy (ratio) | rec_mut (base) | rec_reuse_s20 (ratio) |
|---|---|---|---|
| 64 | 6.18x | 295 ns | 2.37x |
| 256 | 6.10x | 1291 ns | 2.30x |
| 1024 | 7.01x | 4519 ns | 2.44x |
| 4096 | 7.07x | 16476 ns | 2.49x |
| 16384 | 7.13x | 63782 ns | 2.51x |

## Cost-model sanity line

At n=16384, the baseline (rec_mut) median is 63782 ns for N record updates, 20% trigger a copy-on-write. Treating n as the work-item count, that is 3.89 ns/item, about 12.5 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

At 20% sharing, reuse costs 2.4x mut (it copies the shared 20% on write); copy stays ~7x. Reuse's cost rises with the shared fraction, as expected for copy-on-write.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
