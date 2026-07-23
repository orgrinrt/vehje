# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), wideselect profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_wideselect_null dominates: 213% faster than the next best (carrier_cold_wideselect_threaded)

carrier_cold_wideselect_null (6.01 us) leads carrier_cold_wideselect_threaded (18.84 us) by 213%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_wideselect_null beats baseline by 74% (significant)

carrier_cold_wideselect_null is -16.79 us (74%) faster than baseline carrier_cold_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_wideselect_fntable is an outlier: 4.6x slower than the field

carrier_cold_wideselect_fntable (27.45 us) is 4.6x the fastest (6.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cold_wideselect_null} vs {carrier_cold_wideselect_threaded, carrier_cold_wideselect_switch, carrier_cold_wideselect_fntable} (213% apart)

The field splits into a fast tier {carrier_cold_wideselect_null} and a slow tier {carrier_cold_wideselect_threaded, carrier_cold_wideselect_switch, carrier_cold_wideselect_fntable} with a 213% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.6x the fastest

Fastest carrier_cold_wideselect_null (6.01 us) to slowest carrier_cold_wideselect_fntable (27.45 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_cold_wideselect_null** at 6012.7 ns median (-73.5% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.57x (fastest 6012.7 ns, slowest 27449.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 29938ns | 29791ns | 28190ns | 29421ns | 31587ns | +21.87% |
| carrier_cold_wideselect_null | 8392ns | 8423ns | 7892ns | 8264ns | 8833ns | -65.84% |
| carrier_cold_wideselect_switch | 24566ns | 25111ns | 22187ns | 24328ns | 26112ns | base |
| carrier_cold_wideselect_threaded | 21896ns | 21113ns | 19203ns | 20657ns | 25102ns | -10.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 27429ns | 25882ns | 28940ns | +23.28% | 0.009 |
| carrier_cold_wideselect_null | 5946ns | 5722ns | 6090ns | -73.28% | 0.043 |
| carrier_cold_wideselect_switch | 22249ns | 19900ns | 23816ns | base | 0.012 |
| carrier_cold_wideselect_threaded | 19619ns | 16966ns | 22751ns | -11.82% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 346523 | 454961 | 0.762 | 0.96× |
| carrier_cold_wideselect_null | 297480 | 1393745 | 0.213 | 0.83× |
| carrier_cold_wideselect_switch | 360307 | 430710 | 0.837 | 1.00× |
| carrier_cold_wideselect_threaded | 348110 | 556871 | 0.625 | 0.97× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.045 Gops/s** (carrier_cold_wideselect_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_wideselect_fntable | 0.009 | 20.8% |
| carrier_cold_wideselect_null | 0.043 | 95.2% |
| carrier_cold_wideselect_switch | 0.011 | 25.2% |
| carrier_cold_wideselect_threaded | 0.014 | 30.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_wideselect_fntable | 29938ns | 29938ns | +21.87% |
| carrier_cold_wideselect_null | 8392ns | 8392ns | -65.84% |
| carrier_cold_wideselect_switch | 24566ns | 24566ns | base |
| carrier_cold_wideselect_threaded | 21896ns | 21896ns | -10.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_wideselect_switch | 22700ns | base | --- | [20232, 23816] | --- | --- | --- | --- |
| carrier_cold_wideselect_fntable | 27449ns | +5605.2ns (+24.7%) | [+3247, +6688]ns | [25898, 28940] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_wideselect_null | 6013ns | -16786.0ns (-73.9%) | [-17779, -14345]ns | [5735, 6090] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_wideselect_threaded | 18841ns | -2582.5ns (-11.4%) | [-4244, -1064]ns | [17265, 22751] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_wideselect_switch | carrier_cold_wideselect_fntable | carrier_cold_wideselect_null | carrier_cold_wideselect_threaded |
|---|---|---|---|---|
| 1 | 23116ns | +25.5% | -73.7% | -24.0% |
| 2 | 24031ns | +17.4% | -74.6% | -8.8% |
| 3 | 22284ns | +29.6% | -74.2% | -10.0% |
| 4 | 20563ns | +25.9% | -72.2% | -14.3% |
| 5 | 19900ns | +34.1% | -69.6% | -14.7% |
| 6 | 23600ns | +9.8% | -74.7% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_wideselect_fntable | 0.231 | moderate+ |
| carrier_cold_wideselect_null | 0.093 | ok |
| carrier_cold_wideselect_switch | 0.166 | ok |
| carrier_cold_wideselect_threaded | -0.270 | moderate- |

**Consistency summary:**

- **carrier_cold_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_cold_wideselect_null**: won 6/6, lost 0/6
- **carrier_cold_wideselect_threaded**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 85362.3ns | 27429.2ns | 311.2% | HIGH |
| carrier_cold_wideselect_null | 89441.1ns | 5945.9ns | 1504.3% | HIGH |
| carrier_cold_wideselect_switch | 92790.6ns | 22249.1ns | 417.1% | HIGH |
| carrier_cold_wideselect_threaded | 92185.2ns | 19619.0ns | 469.9% | HIGH |

## Distribution (algo ns)

```
carrier_cold_wideselect_fntable (n=6, range 25882.5-28940.2 ns)
  25882.5 |########################################
  26035.4 |
  26188.3 |
  26341.2 |
  26494.0 |
  26646.9 |####################
  26799.8 |
  26952.7 |
  27105.6 |
  27258.5 |
  27411.3 |
  27564.2 |
  27717.1 |
  27870.0 |
  28022.9 |
  28175.8 |####################
  28328.7 |
  28481.5 |
  28634.4 |
  28787.3 |####################
  (0 below, 1 above range)

carrier_cold_wideselect_null (n=6, range 5721.7-6089.6 ns)
   5721.7 |########################################
   5740.1 |########################################
   5758.5 |
   5776.9 |
   5795.3 |
   5813.7 |
   5832.1 |
   5850.5 |
   5868.9 |
   5887.3 |
   5905.6 |
   5924.0 |
   5942.4 |
   5960.8 |########################################
   5979.2 |
   5997.6 |
   6016.0 |
   6034.4 |########################################
   6052.8 |
   6071.2 |########################################
  (0 below, 1 above range)

carrier_cold_wideselect_switch (n=6, range 19900.4-23815.6 ns)
  19900.4 |########################################
  20096.2 |
  20291.9 |
  20487.7 |########################################
  20683.4 |
  20879.2 |
  21075.0 |
  21270.7 |
  21466.5 |
  21662.2 |
  21858.0 |
  22053.8 |
  22249.5 |########################################
  22445.3 |
  22641.0 |
  22836.8 |
  23032.6 |########################################
  23228.3 |
  23424.1 |########################################
  23619.8 |
  (0 below, 1 above range)

carrier_cold_wideselect_threaded (n=6, range 16965.8-22751.4 ns)
  16965.8 |####################
  17255.1 |
  17544.4 |########################################
  17833.6 |
  18122.9 |
  18412.2 |
  18701.5 |
  18990.8 |
  19280.1 |
  19569.3 |
  19858.6 |####################
  20147.9 |
  20437.2 |
  20726.5 |
  21015.8 |
  21305.0 |
  21594.3 |
  21883.6 |####################
  22172.9 |
  22462.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_wideselect_fntable**: bridge=309.1% of algo (FFI overhead may distort results)
- **carrier_cold_wideselect_null**: bridge=1484.2% of algo (FFI overhead may distort results)
- **carrier_cold_wideselect_switch**: bridge=413.3% of algo (FFI overhead may distort results)
- **carrier_cold_wideselect_threaded**: bridge=490.9% of algo (FFI overhead may distort results)
