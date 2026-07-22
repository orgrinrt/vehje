# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_fntable_v17 is inconsistent: worst-20% is 2.6x its best-20%

carrier_disp_fntable_v17's best 20% of batches run at 10.00 us but its worst 20% at 25.91 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_disp_threaded_v17** at 8997.2 ns median (-2.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 8997.2 ns, slowest 10527.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 21378ns | 12770ns | 12140ns | 12637ns | 39110ns | +66.23% |
| carrier_disp_switch_v17 | 12860ns | 11535ns | 11025ns | 11436ns | 15915ns | base |
| carrier_disp_threaded_v17 | 11321ns | 11239ns | 10894ns | 11196ns | 11722ns | -11.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 15510ns | 9999ns | 25909ns | +47.43% | 0.017 |
| carrier_disp_switch_v17 | 10520ns | 8842ns | 13358ns | base | 0.024 |
| carrier_disp_threaded_v17 | 9072ns | 8722ns | 9390ns | -13.77% | 0.028 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_disp_threaded_v17; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.024 | 82.9% |
| carrier_disp_switch_v17 | 0.028 | 94.3% |
| carrier_disp_threaded_v17 | 0.028 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 21378ns | 21378ns | +66.23% |
| carrier_disp_switch_v17 | 12860ns | 12860ns | base |
| carrier_disp_threaded_v17 | 11321ns | 11321ns | -11.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 9246ns | base | --- | [8958, 13358] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 10527ns | +1419.5ns (+15.4%) | [+626, +12922]ns | [10092, 25909] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_threaded_v17 | 8997ns | -139.6ns (-1.5%) | [-4088, -118]ns | [8828, 9390] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 | carrier_disp_threaded_v17 |
|---|---|---|---|
| 1 | 9860ns | +1.4% | -1.2% |
| 2 | 16855ns | +143.3% | -46.7% |
| 3 | 9148ns | +15.1% | -1.5% |
| 4 | 9343ns | +15.6% | -3.3% |
| 5 | 9073ns | +12.3% | -1.5% |
| 6 | 8842ns | +19.1% | -1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | -0.247 | moderate- |
| carrier_disp_switch_v17 | -0.146 | ok |
| carrier_disp_threaded_v17 | 0.005 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 0/6, lost 6/6
- **carrier_disp_threaded_v17**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 95.3ns | 15509.7ns | 0.6% |  |
| carrier_disp_switch_v17 | 88.3ns | 10520.3ns | 0.8% |  |
| carrier_disp_threaded_v17 | 79.7ns | 9071.7ns | 0.9% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 9999.2-25909.2 ns)
   9999.2 |########################################
  10794.7 |##########
  11590.2 |
  12385.7 |
  13181.2 |
  13976.7 |
  14772.2 |
  15567.7 |
  16363.2 |
  17158.7 |
  17954.2 |
  18749.7 |
  19545.2 |
  20340.7 |
  21136.2 |
  21931.7 |
  22727.2 |
  23522.7 |
  24318.2 |
  25113.7 |
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 8842.1-13357.5 ns)
   8842.1 |####################
   9067.9 |########################################
   9293.6 |####################
   9519.4 |
   9745.2 |####################
   9971.0 |
  10196.7 |
  10422.5 |
  10648.3 |
  10874.0 |
  11099.8 |
  11325.6 |
  11551.3 |
  11777.1 |
  12002.9 |
  12228.6 |
  12454.4 |
  12680.2 |
  12906.0 |
  13131.7 |
  (0 below, 1 above range)

carrier_disp_threaded_v17 (n=6, range 8722.5-9389.8 ns)
   8722.5 |########################################
   8755.9 |
   8789.2 |
   8822.6 |
   8856.0 |
   8889.3 |
   8922.7 |########################################
   8956.1 |########################################
   8989.4 |########################################
   9022.8 |########################################
   9056.1 |
   9089.5 |
   9122.9 |
   9156.2 |
   9189.6 |
   9223.0 |
   9256.3 |
   9289.7 |
   9323.1 |
   9356.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_fntable_v17**: CV=73.6% (high variance, measurements may be unstable)
- **carrier_disp_switch_v17**: CV=27.1% (high variance, measurements may be unstable)
