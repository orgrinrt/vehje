# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: carrier_disp_threaded_v17** at 38988.1 ns median (-1.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.32x (fastest 38988.1 ns, slowest 51421.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 56070ns | 53880ns | 47302ns | 53848ns | 63786ns | +36.96% |
| carrier_disp_switch_v17 | 40937ns | 42117ns | 37805ns | 40908ns | 42547ns | base |
| carrier_disp_threaded_v17 | 41189ns | 41434ns | 38265ns | 41346ns | 42416ns | +0.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 53655ns | 45140ns | 61305ns | +39.33% | 0.019 |
| carrier_disp_switch_v17 | 38508ns | 35598ns | 40035ns | base | 0.027 |
| carrier_disp_threaded_v17 | 38717ns | 35968ns | 39813ns | +0.54% | 0.026 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_disp_switch_v17; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.020 | 69.2% |
| carrier_disp_switch_v17 | 0.026 | 89.9% |
| carrier_disp_threaded_v17 | 0.026 | 91.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 56070ns | 56070ns | +36.96% |
| carrier_disp_switch_v17 | 40937ns | 40937ns | base |
| carrier_disp_threaded_v17 | 41189ns | 41189ns | +0.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 39576ns | base | --- | [35913, 40035] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 51422ns | +13621.9ns (+34.4%) | [+10549, +21269]ns | [48238, 61305] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_disp_threaded_v17 | 38988ns | no significant difference | [-2436, +2891]ns | [37351, 39813] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 | carrier_disp_threaded_v17 |
|---|---|---|---|
| 1 | 35598ns | +26.8% | +8.8% |
| 2 | 36228ns | +41.7% | +7.3% |
| 3 | 40158ns | +67.3% | -10.4% |
| 4 | 39365ns | +30.8% | +2.5% |
| 5 | 39913ns | +38.8% | -1.6% |
| 6 | 39786ns | +29.0% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | -0.178 | ok |
| carrier_disp_switch_v17 | 0.352 | moderate+ |
| carrier_disp_threaded_v17 | -0.355 | moderate- |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 0/6, lost 6/6
- **carrier_disp_threaded_v17**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 163.0ns | 53654.8ns | 0.3% |  |
| carrier_disp_switch_v17 | 164.3ns | 38508.0ns | 0.4% |  |
| carrier_disp_threaded_v17 | 171.6ns | 38717.3ns | 0.4% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 45139.6-61304.8 ns)
  45139.6 |#############
  45947.9 |
  46756.1 |
  47564.4 |
  48372.6 |
  49180.9 |
  49989.1 |
  50797.4 |########################################
  51605.7 |
  52413.9 |
  53222.2 |
  54030.4 |
  54838.7 |#############
  55646.9 |
  56455.2 |
  57263.5 |
  58071.7 |
  58880.0 |
  59688.2 |
  60496.5 |
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 35597.9-40035.4 ns)
  35597.9 |########################################
  35819.8 |
  36041.7 |########################################
  36263.5 |
  36485.4 |
  36707.3 |
  36929.2 |
  37151.0 |
  37372.9 |
  37594.8 |
  37816.7 |
  38038.5 |
  38260.4 |
  38482.3 |
  38704.2 |
  38926.0 |
  39147.9 |########################################
  39369.8 |
  39591.7 |########################################
  39813.5 |########################################
  (0 below, 1 above range)

carrier_disp_threaded_v17 (n=6, range 35967.9-39812.7 ns)
  35967.9 |########################################
  36160.1 |
  36352.4 |
  36544.6 |
  36736.9 |
  36929.1 |
  37121.3 |
  37313.6 |
  37505.8 |
  37698.1 |
  37890.3 |
  38082.5 |
  38274.8 |
  38467.0 |
  38659.3 |########################################
  38851.5 |########################################
  39043.7 |########################################
  39236.0 |########################################
  39428.2 |
  39620.5 |
  (0 below, 1 above range)

```
