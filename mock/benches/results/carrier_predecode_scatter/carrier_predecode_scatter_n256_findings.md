# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null dominates: 22% faster than the next best (carrier_pre_scatter_direct)

carrier_pre_scatter_null (6.45 us) leads carrier_pre_scatter_direct (7.86 us) by 22%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_scatter_null beats baseline by 32% (significant)

carrier_pre_scatter_null is -3.03 us (32%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 2.1x slower than the field

carrier_pre_scatter_fntable (13.47 us) is 2.1x the fastest (6.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_scatter_switch shows alternating (throttle bounce) (autocorr -0.66)

carrier_pre_scatter_switch's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} vs {carrier_pre_scatter_fntable} (26% apart)

The field splits into a fast tier {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} and a slow tier {carrier_pre_scatter_fntable} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 6452.5 ns median (-31.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.09x (fastest 6452.5 ns, slowest 13466.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 10423ns | 10426ns | 10317ns | 10406ns | 10500ns | -13.75% |
| carrier_pre_scatter_fntable | 15794ns | 16022ns | 14069ns | 16008ns | 16337ns | +30.70% |
| carrier_pre_scatter_null | 9037ns | 9022ns | 8958ns | 9005ns | 9125ns | -25.22% |
| carrier_pre_scatter_regcache | 13263ns | 13246ns | 13197ns | 13240ns | 13331ns | +9.75% |
| carrier_pre_scatter_switch | 12085ns | 11999ns | 11867ns | 11966ns | 12372ns | base |
| carrier_pre_scatter_threaded | 11726ns | 11728ns | 11526ns | 11714ns | 11845ns | -2.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 7845ns | 7745ns | 7889ns | -17.54% | 0.033 |
| carrier_pre_scatter_fntable | 13258ns | 11785ns | 13712ns | +39.36% | 0.019 |
| carrier_pre_scatter_null | 6468ns | 6372ns | 6546ns | -32.01% | 0.040 |
| carrier_pre_scatter_regcache | 10679ns | 10623ns | 10739ns | +12.25% | 0.024 |
| carrier_pre_scatter_switch | 9514ns | 9390ns | 9669ns | base | 0.027 |
| carrier_pre_scatter_threaded | 9159ns | 9069ns | 9231ns | -3.73% | 0.028 |

## Performance model

- Peak throughput: **0.040 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.033 | 81.0% |
| carrier_pre_scatter_fntable | 0.019 | 47.3% |
| carrier_pre_scatter_null | 0.040 | 98.8% |
| carrier_pre_scatter_regcache | 0.024 | 59.8% |
| carrier_pre_scatter_switch | 0.027 | 67.4% |
| carrier_pre_scatter_threaded | 0.028 | 69.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 10423ns | 10423ns | -13.75% |
| carrier_pre_scatter_fntable | 15794ns | 15794ns | +30.70% |
| carrier_pre_scatter_null | 9037ns | 9037ns | -25.22% |
| carrier_pre_scatter_regcache | 13263ns | 13263ns | +9.75% |
| carrier_pre_scatter_switch | 12085ns | 12085ns | base |
| carrier_pre_scatter_threaded | 11726ns | 11726ns | -2.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 9460ns | base | --- | [9411, 9669] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 7865ns | -1674.3ns (-17.7%) | [-1799, -1532]ns | [7782, 7889] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 13466ns | +3937.1ns (+41.6%) | [+3000, +4295]ns | [12595, 13712] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_null | 6452ns | -3029.3ns (-32.0%) | [-3169, -2939]ns | [6405, 6546] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 10664ns | +1183.8ns (+12.5%) | [+984, +1327]ns | [10634, 10739] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_threaded | 9148ns | -332.5ns (-3.5%) | [-526, -207]ns | [9097, 9231] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 9433ns | -16.4% | +24.9% | -32.4% | +14.1% | -2.2% |
| 2 | 9758ns | -19.1% | +37.4% | -33.0% | +8.9% | -6.5% |
| 3 | 9390ns | -16.1% | +44.4% | -31.4% | +14.1% | -2.7% |
| 4 | 9581ns | -18.0% | +40.5% | -32.6% | +11.5% | -4.4% |
| 5 | 9443ns | -17.2% | +46.9% | -31.7% | +12.7% | -2.2% |
| 6 | 9478ns | -18.3% | +42.2% | -30.9% | +12.3% | -4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | 0.379 | moderate+ |
| carrier_pre_scatter_fntable | 0.051 | ok |
| carrier_pre_scatter_null | -0.459 | moderate- |
| carrier_pre_scatter_regcache | -0.390 | moderate- |
| carrier_pre_scatter_switch | -0.656 | HIGH- (thermal bounce) |
| carrier_pre_scatter_threaded | -0.396 | moderate- |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 0/6, lost 6/6
- **carrier_pre_scatter_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 91459.4ns | 7845.3ns | 1165.8% | HIGH |
| carrier_pre_scatter_fntable | 93124.4ns | 13257.8ns | 702.4% | HIGH |
| carrier_pre_scatter_null | 89901.8ns | 6467.9ns | 1390.0% | HIGH |
| carrier_pre_scatter_regcache | 94948.1ns | 10678.8ns | 889.1% | HIGH |
| carrier_pre_scatter_switch | 93156.3ns | 9513.7ns | 979.2% | HIGH |
| carrier_pre_scatter_threaded | 91154.2ns | 9158.6ns | 995.3% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 7744.6-7889.4 ns)
   7744.6 |########################################
   7751.8 |
   7759.1 |
   7766.3 |
   7773.6 |
   7780.8 |
   7788.0 |
   7795.3 |
   7802.5 |
   7809.7 |
   7817.0 |########################################
   7824.2 |
   7831.5 |
   7838.7 |
   7845.9 |
   7853.2 |########################################
   7860.4 |
   7867.6 |########################################
   7874.9 |
   7882.1 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 11784.6-13711.6 ns)
  11784.6 |####################
  11881.0 |
  11977.3 |
  12073.7 |
  12170.0 |
  12266.4 |
  12362.7 |
  12459.1 |
  12555.4 |
  12651.8 |
  12748.1 |
  12844.5 |
  12940.8 |
  13037.2 |
  13133.5 |
  13229.9 |
  13326.2 |####################
  13422.6 |########################################
  13518.9 |####################
  13615.3 |
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 6372.5-6546.5 ns)
   6372.5 |########################################
   6381.2 |
   6389.9 |
   6398.6 |
   6407.3 |
   6416.0 |
   6424.7 |
   6433.4 |########################################
   6442.1 |########################################
   6450.8 |
   6459.5 |########################################
   6468.2 |
   6476.9 |
   6485.6 |
   6494.3 |
   6503.0 |
   6511.7 |
   6520.4 |
   6529.1 |
   6537.8 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 10623.3-10738.8 ns)
  10623.3 |####################
  10629.1 |
  10634.8 |
  10640.6 |########################################
  10646.4 |
  10652.2 |
  10657.9 |
  10663.7 |
  10669.5 |
  10675.3 |
  10681.0 |####################
  10686.8 |
  10692.6 |
  10698.3 |
  10704.1 |
  10709.9 |####################
  10715.7 |
  10721.4 |
  10727.2 |
  10733.0 |
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 9389.6-9669.1 ns)
   9389.6 |####################
   9403.6 |
   9417.6 |
   9431.5 |########################################
   9445.5 |
   9459.5 |
   9473.5 |####################
   9487.4 |
   9501.4 |
   9515.4 |
   9529.4 |
   9543.4 |
   9557.3 |
   9571.3 |####################
   9585.3 |
   9599.3 |
   9613.2 |
   9627.2 |
   9641.2 |
   9655.2 |
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 9068.8-9231.0 ns)
   9068.8 |########################################
   9076.9 |
   9085.0 |
   9093.1 |
   9101.2 |
   9109.4 |
   9117.5 |########################################
   9125.6 |
   9133.7 |########################################
   9141.8 |
   9149.9 |
   9158.0 |########################################
   9166.1 |
   9174.3 |
   9182.4 |
   9190.5 |
   9198.6 |
   9206.7 |
   9214.8 |
   9222.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=1172.5% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=694.5% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=1393.5% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=890.1% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=986.6% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=995.9% of algo (FFI overhead may distort results)
