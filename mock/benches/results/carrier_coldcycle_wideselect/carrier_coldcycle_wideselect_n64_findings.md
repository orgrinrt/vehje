# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), wideselect profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_wideselect_null dominates: 44% faster than the next best (carrier_cold_wideselect_threaded)

carrier_cold_wideselect_null (1.25 us) leads carrier_cold_wideselect_threaded (1.80 us) by 44%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_wideselect_null beats baseline by 40% (significant)

carrier_cold_wideselect_null is -855 ns (40%) faster than baseline carrier_cold_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_wideselect_fntable is an outlier: 2.2x slower than the field

carrier_cold_wideselect_fntable (2.72 us) is 2.2x the fastest (1.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_cold_wideselect_threaded shows alternating (throttle bounce) (autocorr -0.57)

carrier_cold_wideselect_threaded's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_cold_wideselect_null} vs {carrier_cold_wideselect_threaded, carrier_cold_wideselect_switch, carrier_cold_wideselect_fntable} (44% apart)

The field splits into a fast tier {carrier_cold_wideselect_null} and a slow tier {carrier_cold_wideselect_threaded, carrier_cold_wideselect_switch, carrier_cold_wideselect_fntable} with a 44% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cold_wideselect_null** at 1248.1 ns median (-41.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.18x (fastest 1248.1 ns, slowest 2718.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 5130ns | 5109ns | 4812ns | 5074ns | 5372ns | +15.16% |
| carrier_cold_wideselect_null | 3580ns | 3576ns | 3353ns | 3516ns | 3789ns | -19.63% |
| carrier_cold_wideselect_switch | 4455ns | 4490ns | 4148ns | 4382ns | 4717ns | base |
| carrier_cold_wideselect_threaded | 4194ns | 4066ns | 3963ns | 4049ns | 4526ns | -5.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 2756ns | 2600ns | 2919ns | +31.80% | 0.023 |
| carrier_cold_wideselect_null | 1251ns | 1192ns | 1312ns | -40.18% | 0.051 |
| carrier_cold_wideselect_switch | 2091ns | 1960ns | 2183ns | base | 0.031 |
| carrier_cold_wideselect_threaded | 1871ns | 1798ns | 2011ns | -10.52% | 0.034 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 239260 | 754068 | 0.317 | 0.83× |
| carrier_cold_wideselect_null | 267162 | 1462793 | 0.183 | 0.92× |
| carrier_cold_wideselect_switch | 289944 | 950849 | 0.305 | 1.00× |
| carrier_cold_wideselect_threaded | 271831 | 1085225 | 0.250 | 0.94× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.054 Gops/s** (carrier_cold_wideselect_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_wideselect_fntable | 0.024 | 43.8% |
| carrier_cold_wideselect_null | 0.051 | 95.5% |
| carrier_cold_wideselect_switch | 0.030 | 56.1% |
| carrier_cold_wideselect_threaded | 0.035 | 66.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_wideselect_fntable | 5130ns | 5130ns | +15.16% |
| carrier_cold_wideselect_null | 3580ns | 3580ns | -19.63% |
| carrier_cold_wideselect_switch | 4455ns | 4455ns | base |
| carrier_cold_wideselect_threaded | 4194ns | 4194ns | -5.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_wideselect_switch | 2124ns | base | --- | [1966, 2183] | --- | --- | --- | --- |
| carrier_cold_wideselect_fntable | 2718ns | +655.7ns (+30.9%) | [+594, +745]ns | [2630, 2919] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_wideselect_null | 1248ns | -854.8ns (-40.2%) | [-892, -774]ns | [1192, 1312] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_wideselect_threaded | 1803ns | -202.4ns (-9.5%) | [-324, -133]ns | [1798, 2011] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_wideselect_switch | carrier_cold_wideselect_fntable | carrier_cold_wideselect_null | carrier_cold_wideselect_threaded |
|---|---|---|---|---|
| 1 | 2129ns | +27.3% | -39.7% | -15.6% |
| 2 | 2155ns | +36.7% | -42.4% | -5.1% |
| 3 | 1971ns | +31.9% | -39.5% | -8.7% |
| 4 | 1960ns | +35.7% | -39.2% | -7.9% |
| 5 | 2211ns | +30.8% | -39.3% | -10.5% |
| 6 | 2119ns | +28.6% | -40.7% | -14.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_wideselect_fntable | -0.443 | moderate- |
| carrier_cold_wideselect_null | -0.075 | ok |
| carrier_cold_wideselect_switch | -0.037 | ok |
| carrier_cold_wideselect_threaded | -0.571 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_cold_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_cold_wideselect_null**: won 6/6, lost 0/6
- **carrier_cold_wideselect_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 74951.0ns | 2755.8ns | 2719.7% | HIGH |
| carrier_cold_wideselect_null | 85401.3ns | 1250.7ns | 6828.0% | HIGH |
| carrier_cold_wideselect_switch | 93574.7ns | 2090.9ns | 4475.3% | HIGH |
| carrier_cold_wideselect_threaded | 85343.1ns | 1870.9ns | 4561.6% | HIGH |

## Distribution (algo ns)

```
carrier_cold_wideselect_fntable (n=6, range 2600.4-2918.9 ns)
   2600.4 |########################################
   2616.3 |
   2632.3 |
   2648.2 |########################################
   2664.1 |
   2680.0 |
   2696.0 |########################################
   2711.9 |########################################
   2727.8 |
   2743.7 |
   2759.7 |
   2775.6 |
   2791.5 |
   2807.5 |
   2823.4 |
   2839.3 |
   2855.2 |
   2871.2 |
   2887.1 |########################################
   2903.0 |
  (0 below, 1 above range)

carrier_cold_wideselect_null (n=6, range 1191.7-1312.1 ns)
   1191.7 |########################################
   1197.7 |
   1203.7 |
   1209.8 |
   1215.8 |
   1221.8 |
   1227.8 |
   1233.8 |
   1239.8 |####################
   1245.9 |
   1251.9 |####################
   1257.9 |
   1263.9 |
   1269.9 |
   1275.9 |
   1282.0 |####################
   1288.0 |
   1294.0 |
   1300.0 |
   1306.0 |
  (0 below, 1 above range)

carrier_cold_wideselect_switch (n=6, range 1960.4-2182.9 ns)
   1960.4 |########################################
   1971.5 |
   1982.7 |
   1993.8 |
   2004.9 |
   2016.0 |
   2027.1 |
   2038.3 |
   2049.4 |
   2060.5 |
   2071.6 |
   2082.8 |
   2093.9 |
   2105.0 |
   2116.1 |####################
   2127.3 |####################
   2138.4 |
   2149.5 |####################
   2160.6 |
   2171.8 |
  (0 below, 1 above range)

carrier_cold_wideselect_threaded (n=6, range 1797.5-2011.2 ns)
   1797.5 |########################################
   1808.2 |
   1818.9 |
   1829.6 |
   1840.2 |
   1850.9 |
   1861.6 |
   1872.3 |
   1883.0 |
   1893.7 |
   1904.4 |
   1915.1 |
   1925.8 |
   1936.4 |
   1947.1 |
   1957.8 |
   1968.5 |##########
   1979.2 |
   1989.9 |
   2000.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_wideselect_fntable**: bridge=2752.9% of algo (FFI overhead may distort results)
- **carrier_cold_wideselect_null**: bridge=6858.2% of algo (FFI overhead may distort results)
- **carrier_cold_wideselect_switch**: bridge=4418.7% of algo (FFI overhead may distort results)
- **carrier_cold_wideselect_threaded**: bridge=4749.4% of algo (FFI overhead may distort results)
