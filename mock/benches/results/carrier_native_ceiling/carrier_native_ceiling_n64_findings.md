# Native ceiling: switch vs fn-table interp vs shape-specialized native, opaque program (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_ceiling_native**

## Highlights

Baseline for all deltas below: **carrier_ceiling_native**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_ceiling_native dominates: 105% faster than the next best (carrier_ceiling_switch)

carrier_ceiling_native (1.51 us) leads carrier_ceiling_switch (3.09 us) by 105%, a clear separation rather than a photo finish. CV 21.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceiling_fntable is an outlier: 2.1x slower than the field

carrier_ceiling_fntable (3.18 us) is 2.1x the fastest (1.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (carrier_ceiling_native)

The baseline carrier_ceiling_native is the fastest (1.51 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### carrier_ceiling_fntable is inconsistent: worst-20% is 1.5x its best-20%

carrier_ceiling_fntable's best 20% of batches run at 2.93 us but its worst 20% at 4.48 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (carrier_ceiling_native) is the fastest** at 1507.1 ns median
- 2 variants significantly slower than baseline
- Spread: 2.11x (fastest 1507.1 ns, slowest 3181.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceiling_fntable | 6665ns | 5986ns | 5506ns | 5866ns | 8443ns | +39.73% |
| carrier_ceiling_native | 4770ns | 4321ns | 4217ns | 4309ns | 5738ns | base |
| carrier_ceiling_switch | 6399ns | 5900ns | 5439ns | 5896ns | 7633ns | +34.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceiling_fntable | 3547ns | 2928ns | 4482ns | +117.59% | 0.018 |
| carrier_ceiling_native | 1630ns | 1363ns | 1967ns | base | 0.039 |
| carrier_ceiling_switch | 3347ns | 2852ns | 3984ns | +105.31% | 0.019 |

## Performance model

- Peak throughput: **0.047 Gops/s** (carrier_ceiling_native; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceiling_fntable | 0.020 | 42.8% |
| carrier_ceiling_native | 0.042 | 90.5% |
| carrier_ceiling_switch | 0.021 | 44.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceiling_fntable | 6665ns | 6665ns | +39.73% |
| carrier_ceiling_native | 4770ns | 4770ns | base |
| carrier_ceiling_switch | 6399ns | 6399ns | +34.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceiling_native | 1507ns | base | --- | [1417, 1967] | --- | --- | --- | --- |
| carrier_ceiling_fntable | 3182ns | +1745.2ns (+115.8%) | [+1490, +2516]ns | [2978, 4482] | YES | 0.0313 | 0.0313 | 0 |
| carrier_ceiling_switch | 3090ns | +1673.0ns (+111.0%) | [+1461, +2017]ns | [2968, 3984] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceiling_native | carrier_ceiling_fntable | carrier_ceiling_switch |
|---|---|---|---|
| 1 | 2327ns | +119.0% | +83.3% |
| 2 | 1607ns | +140.8% | +130.4% |
| 3 | 1470ns | +105.9% | +109.9% |
| 4 | 1363ns | +132.6% | +126.9% |
| 5 | 1510ns | +111.5% | +104.2% |
| 6 | 1504ns | +94.6% | +89.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceiling_fntable | 0.257 | moderate+ |
| carrier_ceiling_native | 0.126 | ok |
| carrier_ceiling_switch | 0.350 | moderate+ |

**Consistency summary:**

- **carrier_ceiling_fntable**: won 0/6, lost 6/6
- **carrier_ceiling_switch**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceiling_fntable | 75.8ns | 3547.3ns | 2.1% |  |
| carrier_ceiling_native | 5.9ns | 1630.3ns | 0.4% |  |
| carrier_ceiling_switch | 75.2ns | 3347.1ns | 2.2% |  |

## Distribution (algo ns)

```
carrier_ceiling_fntable (n=6, range 2927.5-4482.4 ns)
   2927.5 |####################
   3005.2 |####################
   3083.0 |
   3160.7 |########################################
   3238.5 |
   3316.2 |
   3394.0 |
   3471.7 |
   3549.5 |
   3627.2 |
   3705.0 |
   3782.7 |
   3860.5 |####################
   3938.2 |
   4016.0 |
   4093.7 |
   4171.5 |
   4249.2 |
   4327.0 |
   4404.7 |
  (0 below, 1 above range)

carrier_ceiling_native (n=6, range 1363.3-1966.9 ns)
   1363.3 |####################
   1393.5 |
   1423.7 |
   1453.8 |####################
   1484.0 |########################################
   1514.2 |
   1544.4 |
   1574.6 |
   1604.7 |####################
   1634.9 |
   1665.1 |
   1695.3 |
   1725.5 |
   1755.6 |
   1785.8 |
   1816.0 |
   1846.2 |
   1876.4 |
   1906.5 |
   1936.7 |
  (0 below, 1 above range)

carrier_ceiling_switch (n=6, range 2852.1-3983.5 ns)
   2852.1 |#############
   2908.7 |
   2965.2 |
   3021.8 |
   3078.4 |########################################
   3134.9 |
   3191.5 |
   3248.1 |
   3304.7 |
   3361.2 |
   3417.8 |
   3474.4 |
   3530.9 |
   3587.5 |
   3644.1 |
   3700.7 |#############
   3757.2 |
   3813.8 |
   3870.4 |
   3926.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceiling_fntable**: CV=21.3% (high variance, measurements may be unstable)
