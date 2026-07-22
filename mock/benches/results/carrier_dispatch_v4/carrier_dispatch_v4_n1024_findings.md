# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_threaded_v4 shows warm-up / thermal drift (autocorr +0.61)

carrier_disp_threaded_v4's per-pass series has lag-1 autocorrelation +0.61, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_disp_switch_v4)

The baseline carrier_disp_switch_v4 is the fastest (34.94 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_disp_switch_v4) is the fastest** at 34943.1 ns median
- 2 variants significantly slower than baseline
- Spread: 1.26x (fastest 34943.1 ns, slowest 43997.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 47729ns | 46208ns | 44964ns | 46043ns | 51641ns | +26.50% |
| carrier_disp_switch_v4 | 37730ns | 37218ns | 35852ns | 36813ns | 40046ns | base |
| carrier_disp_threaded_v4 | 38731ns | 38987ns | 36409ns | 38146ns | 40769ns | +2.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 45459ns | 42829ns | 49200ns | +28.31% | 0.023 |
| carrier_disp_switch_v4 | 35430ns | 33680ns | 37602ns | base | 0.029 |
| carrier_disp_threaded_v4 | 36401ns | 34228ns | 38321ns | +2.74% | 0.028 |

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_disp_switch_v4; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.023 | 76.6% |
| carrier_disp_switch_v4 | 0.029 | 96.4% |
| carrier_disp_threaded_v4 | 0.028 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 47729ns | 47729ns | +26.50% |
| carrier_disp_switch_v4 | 37730ns | 37730ns | base |
| carrier_disp_threaded_v4 | 38731ns | 38731ns | +2.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 34943ns | base | --- | [33746, 37602] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 43997ns | +9851.2ns (+28.2%) | [+8638, +11598]ns | [43182, 49200] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_threaded_v4 | 36646ns | +701.7ns (+2.0%) | [+317, +1892]ns | [34235, 38321] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 | carrier_disp_threaded_v4 |
|---|---|---|---|
| 1 | 35359ns | +21.1% | +8.1% |
| 2 | 38207ns | +29.9% | +0.6% |
| 3 | 36997ns | +31.8% | +2.3% |
| 4 | 34527ns | +28.4% | +2.7% |
| 5 | 33680ns | +29.3% | +1.7% |
| 6 | 33812ns | +29.1% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | 0.108 | ok |
| carrier_disp_switch_v4 | 0.429 | moderate+ |
| carrier_disp_threaded_v4 | 0.606 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 0/6, lost 6/6
- **carrier_disp_threaded_v4**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 156.0ns | 45459.5ns | 0.3% |  |
| carrier_disp_switch_v4 | 154.2ns | 35430.3ns | 0.4% |  |
| carrier_disp_threaded_v4 | 160.8ns | 36400.6ns | 0.4% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 42828.8-49199.8 ns)
  42828.8 |####################
  43147.3 |
  43465.9 |########################################
  43784.4 |
  44103.0 |####################
  44421.5 |
  44740.1 |
  45058.6 |
  45377.2 |
  45695.7 |
  46014.3 |
  46332.8 |
  46651.4 |
  46969.9 |
  47288.5 |
  47607.0 |
  47925.6 |
  48244.1 |
  48562.7 |####################
  48881.2 |
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 33680.0-37601.7 ns)
  33680.0 |########################################
  33876.1 |
  34072.2 |
  34268.3 |
  34464.3 |####################
  34660.4 |
  34856.5 |
  35052.6 |
  35248.7 |####################
  35444.8 |
  35640.8 |
  35836.9 |
  36033.0 |
  36229.1 |
  36425.2 |
  36621.3 |
  36817.4 |####################
  37013.4 |
  37209.5 |
  37405.6 |
  (0 below, 1 above range)

carrier_disp_threaded_v4 (n=6, range 34227.9-38320.8 ns)
  34227.9 |########################################
  34432.5 |
  34637.2 |
  34841.8 |
  35046.5 |
  35251.1 |####################
  35455.8 |
  35660.4 |
  35865.1 |
  36069.7 |
  36274.4 |
  36479.0 |
  36683.6 |
  36888.3 |
  37092.9 |
  37297.6 |
  37502.2 |
  37706.9 |####################
  37911.5 |
  38116.2 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_threaded_v4**: autocorrelation=0.61 (measurement drift or warm-up artifact)
