# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_switch_v4 dominates: 15% faster than the next best (carrier_disp_fntable_v4)

carrier_disp_switch_v4 (9.90 us) leads carrier_disp_fntable_v4 (11.34 us) by 15%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_switch_v4 is fastest but the noisiest (CV 6.4%)

carrier_disp_switch_v4 wins on median (9.90 us) yet has the highest variance (CV 6.4%), while carrier_disp_fntable_v4 is the steadiest (CV 5.2%, 11.34 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_disp_switch_v4)

The baseline carrier_disp_switch_v4 is the fastest (9.90 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_disp_switch_v4) is the fastest** at 9898.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.15x (fastest 9898.0 ns, slowest 11338.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 13553ns | 13935ns | 12135ns | 13655ns | 14108ns | +11.15% |
| carrier_disp_switch_v4 | 12193ns | 12497ns | 10582ns | 12236ns | 12935ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 11007ns | 9845ns | 11438ns | +14.47% | 0.023 |
| carrier_disp_switch_v4 | 9616ns | 8375ns | 10122ns | base | 0.027 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_disp_switch_v4; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.023 | 73.9% |
| carrier_disp_switch_v4 | 0.026 | 84.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 13553ns | 13553ns | +11.15% |
| carrier_disp_switch_v4 | 12193ns | 12193ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 9898ns | base | --- | [8827, 10122] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 11338ns | +1447.1ns (+14.6%) | [+949, +1778]ns | [10245, 11438] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 |
|---|---|---|
| 1 | 8375ns | +17.5% |
| 2 | 9962ns | +6.9% |
| 3 | 9917ns | +14.4% |
| 4 | 9879ns | +15.2% |
| 5 | 10283ns | +11.8% |
| 6 | 9279ns | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | 0.369 | moderate+ |
| carrier_disp_switch_v4 | -0.124 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 90.8ns | 11007.1ns | 0.8% |  |
| carrier_disp_switch_v4 | 90.6ns | 9615.8ns | 0.9% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 9844.6-11438.1 ns)
   9844.6 |####################
   9924.3 |
  10004.0 |
  10083.6 |
  10163.3 |
  10243.0 |
  10322.6 |
  10402.3 |
  10482.0 |
  10561.7 |
  10641.4 |####################
  10721.0 |
  10800.7 |
  10880.4 |
  10960.1 |
  11039.7 |
  11119.4 |
  11199.1 |
  11278.8 |########################################
  11358.4 |####################
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 8375.0-10122.5 ns)
   8375.0 |####################
   8462.4 |
   8549.8 |
   8637.1 |
   8724.5 |
   8811.9 |
   8899.2 |
   8986.6 |
   9074.0 |
   9161.4 |
   9248.8 |####################
   9336.1 |
   9423.5 |
   9510.9 |
   9598.2 |
   9685.6 |
   9773.0 |
   9860.4 |########################################
   9947.8 |####################
  10035.1 |
  (0 below, 1 above range)

```
