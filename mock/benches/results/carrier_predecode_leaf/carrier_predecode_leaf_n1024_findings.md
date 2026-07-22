# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_direct beats baseline by 25% (significant)

carrier_pre_leaf_direct is -6.51 us (25%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_leaf_fntable is an outlier: 2.7x slower than the field

carrier_pre_leaf_fntable (54.73 us) is 2.7x the fastest (20.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} vs {carrier_pre_leaf_fntable} (98% apart)

The field splits into a fast tier {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} and a slow tier {carrier_pre_leaf_fntable} with a 98% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_leaf_direct** at 20145.8 ns median (-22.8% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.72x (fastest 20145.8 ns, slowest 54731.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 22413ns | 22511ns | 21119ns | 22319ns | 23201ns | -22.00% |
| carrier_pre_leaf_fntable | 56412ns | 57092ns | 52941ns | 56498ns | 58019ns | +96.32% |
| carrier_pre_leaf_null | 22746ns | 22725ns | 21946ns | 22500ns | 23517ns | -20.84% |
| carrier_pre_leaf_regcache | 29952ns | 29966ns | 27271ns | 29764ns | 31575ns | +4.24% |
| carrier_pre_leaf_switch | 28735ns | 28473ns | 27121ns | 28348ns | 30123ns | base |
| carrier_pre_leaf_threaded | 25339ns | 25256ns | 25117ns | 25248ns | 25587ns | -11.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 20055ns | 18892ns | 20754ns | -23.93% | 0.051 |
| carrier_pre_leaf_fntable | 54081ns | 50790ns | 55660ns | +105.12% | 0.019 |
| carrier_pre_leaf_null | 20372ns | 19638ns | 21048ns | -22.73% | 0.050 |
| carrier_pre_leaf_regcache | 27634ns | 25108ns | 29203ns | +4.81% | 0.037 |
| carrier_pre_leaf_switch | 26365ns | 24880ns | 27681ns | base | 0.039 |
| carrier_pre_leaf_threaded | 22968ns | 22726ns | 23218ns | -12.88% | 0.045 |

## Performance model

- Peak throughput: **0.054 Gops/s** (carrier_pre_leaf_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.051 | 93.8% |
| carrier_pre_leaf_fntable | 0.019 | 34.5% |
| carrier_pre_leaf_null | 0.050 | 92.7% |
| carrier_pre_leaf_regcache | 0.037 | 68.4% |
| carrier_pre_leaf_switch | 0.039 | 72.4% |
| carrier_pre_leaf_threaded | 0.045 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 22413ns | 22413ns | -22.00% |
| carrier_pre_leaf_fntable | 56412ns | 56412ns | +96.32% |
| carrier_pre_leaf_null | 22746ns | 22746ns | -20.84% |
| carrier_pre_leaf_regcache | 29952ns | 29952ns | +4.24% |
| carrier_pre_leaf_switch | 28735ns | 28735ns | base |
| carrier_pre_leaf_threaded | 25339ns | 25339ns | -11.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 26096ns | base | --- | [25317, 27681] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 20146ns | -6511.0ns (-25.0%) | [-7459, -4960]ns | [19266, 20754] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 54731ns | +28621.0ns (+109.7%) | [+24615, +29912]ns | [51852, 55660] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_null | 20374ns | -5841.1ns (-22.4%) | [-7693, -4446]ns | [19693, 21048] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 27619ns | no significant difference | [-742, +3296]ns | [26081, 29203] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_leaf_threaded | 22899ns | -3156.5ns (-12.1%) | [-4783, -2252]ns | [22788, 23218] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 28415ns | -27.0% | +78.7% | -30.9% | -1.6% | -19.5% |
| 2 | 24880ns | -16.5% | +120.0% | -14.2% | +9.7% | -7.9% |
| 3 | 25755ns | -23.7% | +112.5% | -20.8% | +13.8% | -11.8% |
| 4 | 26948ns | -25.6% | +104.9% | -24.5% | +0.4% | -15.0% |
| 5 | 26133ns | -27.7% | +114.7% | -20.6% | -3.9% | -12.6% |
| 6 | 26059ns | -22.3% | +103.1% | -24.2% | +11.7% | -9.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | -0.005 | ok |
| carrier_pre_leaf_fntable | -0.058 | ok |
| carrier_pre_leaf_null | -0.462 | moderate- |
| carrier_pre_leaf_regcache | -0.329 | moderate- |
| carrier_pre_leaf_switch | -0.352 | moderate- |
| carrier_pre_leaf_threaded | -0.081 | ok |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 2/6, lost 4/6
- **carrier_pre_leaf_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 102769.7ns | 20055.2ns | 512.4% | HIGH |
| carrier_pre_leaf_fntable | 110153.2ns | 54081.0ns | 203.7% | HIGH |
| carrier_pre_leaf_null | 103628.5ns | 20371.7ns | 508.7% | HIGH |
| carrier_pre_leaf_regcache | 103054.8ns | 27634.3ns | 372.9% | HIGH |
| carrier_pre_leaf_switch | 106135.8ns | 26364.9ns | 402.6% | HIGH |
| carrier_pre_leaf_threaded | 94503.6ns | 22968.0ns | 411.5% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 18892.1-20753.6 ns)
  18892.1 |########################################
  18985.2 |
  19078.2 |
  19171.3 |
  19264.4 |
  19357.5 |
  19450.5 |
  19543.6 |
  19636.7 |########################################
  19729.8 |
  19822.8 |
  19915.9 |
  20009.0 |########################################
  20102.0 |
  20195.1 |########################################
  20288.2 |
  20381.3 |
  20474.3 |
  20567.4 |
  20660.5 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 50790.0-55659.6 ns)
  50790.0 |####################
  51033.5 |
  51277.0 |
  51520.4 |
  51763.9 |
  52007.4 |
  52250.9 |
  52494.3 |
  52737.8 |####################
  52981.3 |
  53224.8 |
  53468.3 |
  53711.7 |
  53955.2 |
  54198.7 |
  54442.2 |
  54685.6 |########################################
  54929.1 |
  55172.6 |####################
  55416.1 |
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 19637.9-21048.1 ns)
  19637.9 |########################################
  19708.4 |########################################
  19778.9 |
  19849.4 |
  19919.9 |
  19990.5 |
  20061.0 |
  20131.5 |
  20202.0 |
  20272.5 |########################################
  20343.0 |########################################
  20413.5 |
  20484.0 |
  20554.5 |
  20625.0 |
  20695.5 |########################################
  20766.1 |
  20836.6 |
  20907.1 |
  20977.6 |
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 25108.3-29202.9 ns)
  25108.3 |########################################
  25313.0 |
  25517.8 |
  25722.5 |
  25927.2 |
  26132.0 |
  26336.7 |
  26541.4 |
  26746.1 |
  26950.9 |########################################
  27155.6 |########################################
  27360.3 |
  27565.1 |
  27769.8 |########################################
  27974.5 |
  28179.2 |
  28384.0 |
  28588.7 |
  28793.4 |
  28998.2 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 24879.6-27681.5 ns)
  24879.6 |####################
  25019.7 |
  25159.8 |
  25299.9 |
  25440.0 |
  25580.1 |
  25720.2 |####################
  25860.2 |
  26000.3 |########################################
  26140.4 |
  26280.5 |
  26420.6 |
  26560.7 |
  26700.8 |
  26840.9 |####################
  26981.0 |
  27121.1 |
  27261.2 |
  27401.3 |
  27541.4 |
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 22726.2-23217.9 ns)
  22726.2 |########################################
  22750.8 |
  22775.4 |
  22800.0 |
  22824.5 |########################################
  22849.1 |
  22873.7 |########################################
  22898.3 |########################################
  22922.9 |########################################
  22947.5 |
  22972.1 |
  22996.6 |
  23021.2 |
  23045.8 |
  23070.4 |
  23095.0 |
  23119.6 |
  23144.1 |
  23168.7 |
  23193.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=515.3% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=203.5% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=508.6% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=377.8% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=408.0% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=411.9% of algo (FFI overhead may distort results)
