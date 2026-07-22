# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null dominates: 22% faster than the next best (carrier_pre_scatter_direct)

carrier_pre_scatter_null (25.83 us) leads carrier_pre_scatter_direct (31.53 us) by 22%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_scatter_null beats baseline by 32% (significant)

carrier_pre_scatter_null is -12.17 us (32%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 2.1x slower than the field

carrier_pre_scatter_fntable (53.41 us) is 2.1x the fastest (25.83 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} vs {carrier_pre_scatter_fntable} (26% apart)

The field splits into a fast tier {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} and a slow tier {carrier_pre_scatter_fntable} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 25833.8 ns median (-32.1% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.07x (fastest 25833.8 ns, slowest 53412.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 33982ns | 33826ns | 32064ns | 33644ns | 35448ns | -19.62% |
| carrier_pre_scatter_fntable | 55280ns | 55725ns | 52350ns | 55637ns | 56208ns | +30.75% |
| carrier_pre_scatter_null | 28574ns | 28116ns | 27463ns | 28080ns | 29872ns | -32.41% |
| carrier_pre_scatter_regcache | 44481ns | 44838ns | 43200ns | 44369ns | 45288ns | +5.21% |
| carrier_pre_scatter_switch | 42278ns | 40386ns | 39091ns | 40177ns | 47024ns | base |
| carrier_pre_scatter_threaded | 39129ns | 39313ns | 38459ns | 39175ns | 39395ns | -7.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 31703ns | 29910ns | 33094ns | -20.69% | 0.032 |
| carrier_pre_scatter_fntable | 52995ns | 50237ns | 53863ns | +32.57% | 0.019 |
| carrier_pre_scatter_null | 26161ns | 25232ns | 27179ns | -34.56% | 0.039 |
| carrier_pre_scatter_regcache | 42182ns | 40964ns | 42959ns | +5.52% | 0.024 |
| carrier_pre_scatter_switch | 39975ns | 36859ns | 44728ns | base | 0.026 |
| carrier_pre_scatter_threaded | 36792ns | 36138ns | 37062ns | -7.96% | 0.028 |

## Performance model

- Peak throughput: **0.041 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.032 | 80.0% |
| carrier_pre_scatter_fntable | 0.019 | 47.2% |
| carrier_pre_scatter_null | 0.040 | 97.7% |
| carrier_pre_scatter_regcache | 0.024 | 59.4% |
| carrier_pre_scatter_switch | 0.027 | 66.3% |
| carrier_pre_scatter_threaded | 0.028 | 68.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 33982ns | 33982ns | -19.62% |
| carrier_pre_scatter_fntable | 55280ns | 55280ns | +30.75% |
| carrier_pre_scatter_null | 28574ns | 28574ns | -32.41% |
| carrier_pre_scatter_regcache | 44481ns | 44481ns | +5.21% |
| carrier_pre_scatter_switch | 42278ns | 42278ns | base |
| carrier_pre_scatter_threaded | 39129ns | 39129ns | -7.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 38029ns | base | --- | [37169, 44728] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 31534ns | -6104.0ns (-16.1%) | [-14170, -4542]ns | [30481, 33094] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 53413ns | +14799.2ns (+38.9%) | [+7636, +16625]ns | [51710, 53863] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_scatter_null | 25834ns | -12167.1ns (-32.0%) | [-19153, -10123]ns | [25469, 27179] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 42512ns | no significant difference | [-2639, +5606]ns | [41076, 42959] | no | 0.2188 | 0.2188 | 0 |
| carrier_pre_scatter_threaded | 36962ns | -1271.7ns (-3.3%) | [-7669, -609]ns | [36352, 37062] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 37654ns | -16.1% | +33.4% | -31.7% | +14.0% | -2.9% |
| 2 | 36859ns | -8.4% | +46.9% | -23.1% | +16.1% | -2.0% |
| 3 | 38405ns | -19.1% | +39.0% | -32.3% | +10.0% | -3.8% |
| 4 | 37479ns | -16.0% | +42.6% | -31.3% | +9.3% | -1.3% |
| 5 | 38558ns | -15.9% | +37.9% | -32.8% | +6.8% | -4.1% |
| 6 | 50897ns | -41.2% | +5.3% | -50.4% | -15.5% | -27.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | -0.336 | moderate- |
| carrier_pre_scatter_fntable | -0.243 | moderate- |
| carrier_pre_scatter_null | -0.151 | ok |
| carrier_pre_scatter_regcache | 0.207 | moderate+ |
| carrier_pre_scatter_switch | 0.028 | ok |
| carrier_pre_scatter_threaded | 0.250 | moderate+ |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 1/6, lost 5/6
- **carrier_pre_scatter_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 98242.8ns | 31703.1ns | 309.9% | HIGH |
| carrier_pre_scatter_fntable | 108957.4ns | 52995.2ns | 205.6% | HIGH |
| carrier_pre_scatter_null | 106404.0ns | 26160.8ns | 406.7% | HIGH |
| carrier_pre_scatter_regcache | 115068.3ns | 42182.3ns | 272.8% | HIGH |
| carrier_pre_scatter_switch | 117311.1ns | 39975.2ns | 293.5% | HIGH |
| carrier_pre_scatter_threaded | 112959.4ns | 36792.0ns | 307.0% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 29910.0-33094.2 ns)
  29910.0 |########################################
  30069.2 |
  30228.4 |
  30387.6 |
  30546.8 |
  30706.0 |
  30865.2 |
  31024.5 |########################################
  31183.7 |
  31342.9 |########################################
  31502.1 |########################################
  31661.3 |
  31820.5 |
  31979.7 |
  32138.9 |
  32298.1 |########################################
  32457.3 |
  32616.5 |
  32775.7 |
  32934.9 |
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 50236.7-53862.9 ns)
  50236.7 |####################
  50418.0 |
  50599.3 |
  50780.6 |
  50961.9 |
  51143.3 |
  51324.6 |
  51505.9 |
  51687.2 |
  51868.5 |
  52049.8 |
  52231.1 |
  52412.4 |
  52593.8 |
  52775.1 |
  52956.4 |
  53137.7 |####################
  53319.0 |########################################
  53500.3 |####################
  53681.6 |
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 25232.1-27179.3 ns)
  25232.1 |########################################
  25329.5 |
  25426.8 |
  25524.2 |
  25621.5 |########################################
  25718.9 |########################################
  25816.3 |
  25913.6 |########################################
  26011.0 |########################################
  26108.4 |
  26205.7 |
  26303.1 |
  26400.4 |
  26497.8 |
  26595.2 |
  26692.5 |
  26789.9 |
  26887.3 |
  26984.6 |
  27082.0 |
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 40964.2-42959.2 ns)
  40964.2 |########################################
  41063.9 |
  41163.7 |########################################
  41263.4 |
  41363.2 |
  41462.9 |
  41562.7 |
  41662.4 |
  41762.2 |
  41861.9 |
  41961.7 |
  42061.4 |
  42161.2 |########################################
  42260.9 |
  42360.7 |
  42460.4 |
  42560.2 |
  42659.9 |
  42759.7 |########################################
  42859.4 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 36858.8-44727.5 ns)
  36858.8 |########################################
  37252.2 |########################################
  37645.7 |########################################
  38039.1 |########################################
  38432.5 |########################################
  38826.0 |
  39219.4 |
  39612.8 |
  40006.3 |
  40399.7 |
  40793.2 |
  41186.6 |
  41580.0 |
  41973.5 |
  42366.9 |
  42760.3 |
  43153.8 |
  43547.2 |
  43940.6 |
  44334.1 |
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 36137.9-37062.1 ns)
  36137.9 |####################
  36184.1 |
  36230.3 |
  36276.5 |
  36322.7 |
  36368.9 |
  36415.2 |
  36461.4 |
  36507.6 |
  36553.8 |####################
  36600.0 |
  36646.2 |
  36692.4 |
  36738.6 |
  36784.8 |
  36831.1 |
  36877.3 |
  36923.5 |####################
  36969.7 |########################################
  37015.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=311.2% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=205.3% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=410.0% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=267.9% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=299.7% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=306.9% of algo (FFI overhead may distort results)
