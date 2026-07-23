# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), leaf profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_leaf_null beats baseline by 46% (significant)

carrier_cold_leaf_null is -4.86 us (46%) faster than baseline carrier_cold_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_leaf_fntable is an outlier: 3.1x slower than the field

carrier_cold_leaf_fntable (15.96 us) is 3.1x the fastest (5.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_cold_leaf_fntable shows alternating (throttle bounce) (autocorr -0.60)

carrier_cold_leaf_fntable's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_cold_leaf_null, carrier_cold_leaf_threaded} vs {carrier_cold_leaf_switch, carrier_cold_leaf_fntable} (93% apart)

The field splits into a fast tier {carrier_cold_leaf_null, carrier_cold_leaf_threaded} and a slow tier {carrier_cold_leaf_switch, carrier_cold_leaf_fntable} with a 93% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_cold_leaf_null (5.11 us) to slowest carrier_cold_leaf_fntable (15.96 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_cold_leaf_threaded is inconsistent: worst-20% is 1.5x its best-20%

carrier_cold_leaf_threaded's best 20% of batches run at 5.23 us but its worst 20% at 8.03 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_cold_leaf_null** at 5109.4 ns median (-52.0% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.12x (fastest 5109.4 ns, slowest 15956.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_leaf_fntable | 18843ns | 18668ns | 17510ns | 18518ns | 19998ns | +41.74% |
| carrier_cold_leaf_null | 7840ns | 7537ns | 6929ns | 7341ns | 9045ns | -41.03% |
| carrier_cold_leaf_switch | 13294ns | 13424ns | 11244ns | 12998ns | 14764ns | base |
| carrier_cold_leaf_threaded | 8727ns | 7842ns | 7468ns | 7794ns | 10756ns | -34.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_leaf_fntable | 16137ns | 15057ns | 17110ns | +52.60% | 0.016 |
| carrier_cold_leaf_null | 5391ns | 4660ns | 6398ns | -49.02% | 0.047 |
| carrier_cold_leaf_switch | 10575ns | 8912ns | 11765ns | base | 0.024 |
| carrier_cold_leaf_threaded | 6284ns | 5232ns | 8031ns | -40.58% | 0.041 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_leaf_fntable | 327472 | 683981 | 0.479 | 1.29× |
| carrier_cold_leaf_null | 283851 | 1258914 | 0.225 | 1.12× |
| carrier_cold_leaf_switch | 253937 | 659906 | 0.385 | 1.00× |
| carrier_cold_leaf_threaded | 290236 | 1335804 | 0.217 | 1.14× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.055 Gops/s** (carrier_cold_leaf_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_leaf_fntable | 0.016 | 29.2% |
| carrier_cold_leaf_null | 0.050 | 91.2% |
| carrier_cold_leaf_switch | 0.024 | 43.8% |
| carrier_cold_leaf_threaded | 0.046 | 84.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_leaf_fntable | 18843ns | 18843ns | +41.74% |
| carrier_cold_leaf_null | 7840ns | 7840ns | -41.03% |
| carrier_cold_leaf_switch | 13294ns | 13294ns | base |
| carrier_cold_leaf_threaded | 8727ns | 8727ns | -34.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_leaf_switch | 10649ns | base | --- | [9311, 11765] | --- | --- | --- | --- |
| carrier_cold_leaf_fntable | 15957ns | +5976.9ns (+56.1%) | [+3580, +7130]ns | [15345, 17110] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_leaf_null | 5109ns | -4860.8ns (-45.6%) | [-6587, -4103]ns | [4667, 6398] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_leaf_threaded | 5519ns | -4742.5ns (-44.5%) | [-6424, -1706]ns | [5301, 8031] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_leaf_switch | carrier_cold_leaf_fntable | carrier_cold_leaf_null | carrier_cold_leaf_threaded |
|---|---|---|---|---|
| 1 | 8912ns | +78.4% | -47.6% | -39.7% |
| 2 | 11862ns | +31.8% | -52.0% | -54.1% |
| 3 | 10231ns | +56.5% | -48.2% | +1.3% |
| 4 | 11067ns | +55.8% | -35.8% | -48.5% |
| 5 | 11668ns | +29.0% | -60.1% | -55.2% |
| 6 | 9710ns | +74.9% | -49.4% | -42.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_leaf_fntable | -0.601 | HIGH- (thermal bounce) |
| carrier_cold_leaf_null | -0.303 | moderate- |
| carrier_cold_leaf_switch | -0.470 | moderate- |
| carrier_cold_leaf_threaded | -0.183 | ok |

**Consistency summary:**

- **carrier_cold_leaf_fntable**: won 0/6, lost 6/6
- **carrier_cold_leaf_null**: won 6/6, lost 0/6
- **carrier_cold_leaf_threaded**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_leaf_fntable | 93423.9ns | 16137.3ns | 578.9% | HIGH |
| carrier_cold_leaf_null | 89041.0ns | 5391.4ns | 1651.5% | HIGH |
| carrier_cold_leaf_switch | 77157.4ns | 10574.9ns | 729.6% | HIGH |
| carrier_cold_leaf_threaded | 90151.3ns | 6283.8ns | 1434.7% | HIGH |

## Distribution (algo ns)

```
carrier_cold_leaf_fntable (n=6, range 15056.7-17110.2 ns)
  15056.7 |########################################
  15159.4 |
  15262.1 |
  15364.7 |
  15467.4 |
  15570.1 |########################################
  15672.8 |
  15775.4 |
  15878.1 |########################################
  15980.8 |########################################
  16083.4 |
  16186.1 |
  16288.8 |
  16391.5 |
  16494.1 |
  16596.8 |
  16699.5 |
  16802.2 |
  16904.8 |########################################
  17007.5 |
  (0 below, 1 above range)

carrier_cold_leaf_null (n=6, range 4660.4-6398.1 ns)
   4660.4 |########################################
   4747.3 |
   4834.2 |####################
   4921.1 |
   5007.9 |
   5094.8 |
   5181.7 |
   5268.6 |####################
   5355.5 |
   5442.4 |
   5529.2 |
   5616.1 |####################
   5703.0 |
   5789.9 |
   5876.8 |
   5963.7 |
   6050.6 |
   6137.4 |
   6224.3 |
   6311.2 |
  (0 below, 1 above range)

carrier_cold_leaf_switch (n=6, range 8912.1-11764.8 ns)
   8912.1 |########################################
   9054.7 |
   9197.4 |
   9340.0 |
   9482.6 |
   9625.3 |########################################
   9767.9 |
   9910.5 |
  10053.2 |
  10195.8 |########################################
  10338.5 |
  10481.1 |
  10623.7 |
  10766.4 |
  10909.0 |
  11051.6 |########################################
  11194.3 |
  11336.9 |
  11479.5 |
  11622.2 |########################################
  (0 below, 1 above range)

carrier_cold_leaf_threaded (n=6, range 5232.1-8030.9 ns)
   5232.1 |########################################
   5372.0 |####################
   5512.0 |####################
   5651.9 |####################
   5791.9 |
   5931.8 |
   6071.7 |
   6211.7 |
   6351.6 |
   6491.5 |
   6631.5 |
   6771.4 |
   6911.4 |
   7051.3 |
   7191.2 |
   7331.2 |
   7471.1 |
   7611.0 |
   7751.0 |
   7890.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_leaf_fntable**: bridge=585.7% of algo (FFI overhead may distort results)
- **carrier_cold_leaf_null**: bridge=1734.0% of algo (FFI overhead may distort results)
- **carrier_cold_leaf_switch**: bridge=726.2% of algo (FFI overhead may distort results)
- **carrier_cold_leaf_threaded**: CV=29.1% (high variance, measurements may be unstable)
- **carrier_cold_leaf_threaded**: bridge=1586.7% of algo (FFI overhead may distort results)
