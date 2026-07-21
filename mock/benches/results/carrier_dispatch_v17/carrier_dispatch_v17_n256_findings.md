# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_switch_v17 dominates: 17% faster than the next best (carrier_disp_fntable_v17)

carrier_disp_switch_v17 (10.34 us) leads carrier_disp_fntable_v17 (12.13 us) by 17%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_switch_v17 is fastest but the noisiest (CV 6.4%)

carrier_disp_switch_v17 wins on median (10.34 us) yet has the highest variance (CV 6.4%), while carrier_disp_fntable_v17 is the steadiest (CV 5.7%, 12.13 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_disp_switch_v17)

The baseline carrier_disp_switch_v17 is the fastest (10.34 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_disp_switch_v17) is the fastest** at 10342.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 10342.5 ns, slowest 12135.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 14464ns | 14736ns | 12848ns | 14516ns | 15193ns | +14.54% |
| carrier_disp_switch_v17 | 12627ns | 12893ns | 10948ns | 12757ns | 13272ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 11908ns | 10568ns | 12497ns | +17.55% | 0.021 |
| carrier_disp_switch_v17 | 10130ns | 8764ns | 10652ns | base | 0.025 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_disp_switch_v17; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.021 | 72.2% |
| carrier_disp_switch_v17 | 0.025 | 84.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 14464ns | 14464ns | +14.54% |
| carrier_disp_switch_v17 | 12627ns | 12627ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 10342ns | base | --- | [9396, 10652] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 12135ns | +1676.2ns (+16.2%) | [+1481, +2177]ns | [11092, 12497] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 |
|---|---|---|
| 1 | 8764ns | +20.6% |
| 2 | 10141ns | +14.5% |
| 3 | 10659ns | +14.5% |
| 4 | 10028ns | +21.1% |
| 5 | 10645ns | +14.0% |
| 6 | 10544ns | +21.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | 0.218 | moderate+ |
| carrier_disp_switch_v17 | 0.037 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 88.2ns | 11908.1ns | 0.7% |  |
| carrier_disp_switch_v17 | 86.9ns | 10130.0ns | 0.9% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 10567.9-12497.1 ns)
  10567.9 |#############
  10664.4 |
  10760.8 |
  10857.3 |
  10953.7 |
  11050.2 |
  11146.7 |
  11243.1 |
  11339.6 |
  11436.0 |
  11532.5 |#############
  11629.0 |
  11725.4 |
  11821.9 |
  11918.3 |
  12014.8 |
  12111.3 |########################################
  12207.7 |
  12304.2 |
  12400.6 |
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 8763.8-10651.9 ns)
   8763.8 |########################################
   8858.2 |
   8952.6 |
   9047.0 |
   9141.4 |
   9235.8 |
   9330.2 |
   9424.6 |
   9519.0 |
   9613.4 |
   9707.9 |
   9802.3 |
   9896.7 |
   9991.1 |########################################
  10085.5 |########################################
  10179.9 |
  10274.3 |
  10368.7 |
  10463.1 |########################################
  10557.5 |########################################
  (0 below, 1 above range)

```
