# Record update (60% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s60**

## Highlights

Baseline for all deltas below: **rec_reuse_s60**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 347% faster than the next best (rec_reuse_s60)

rec_mut (1.18 us) leads rec_reuse_s60 (5.25 us) by 347%, a clear separation rather than a photo finish. CV 6.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 76% (significant)

rec_mut is -4.00 us (76%) faster than baseline rec_reuse_s60, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 6.2x slower than the field

rec_copy (7.33 us) is 6.2x the fastest (1.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 6.2x the fastest

Fastest rec_mut (1.18 us) to slowest rec_copy (7.33 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 1175.7 ns median (-77.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.23x (fastest 1175.7 ns, slowest 7325.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 9746ns | 9563ns | 9123ns | 9469ns | 10474ns | +27.54% |
| rec_mut | 3509ns | 3399ns | 3293ns | 3386ns | 3802ns | -54.08% |
| rec_reuse_s60 | 7642ns | 7608ns | 7044ns | 7475ns | 8191ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 7413ns | 6946ns | 7920ns | +40.29% | 0.035 |
| rec_mut | 1209ns | 1137ns | 1305ns | -77.12% | 0.212 |
| rec_reuse_s60 | 5284ns | 4835ns | 5695ns | base | 0.048 |

## Performance model

- Peak throughput: **0.225 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.035 | 15.5% |
| rec_mut | 0.218 | 96.7% |
| rec_reuse_s60 | 0.049 | 21.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 9746ns | 9746ns | +27.54% |
| rec_mut | 3509ns | 3509ns | -54.08% |
| rec_reuse_s60 | 7642ns | 7642ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s60 | 5249ns | base | --- | [4909, 5695] | --- | --- | --- | --- |
| rec_copy | 7326ns | +2179.2ns (+41.5%) | [+1849, +2359]ns | [6995, 7920] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 1176ns | -4004.3ns (-76.3%) | [-4488, -3733]ns | [1146, 1305] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s60 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 5760ns | +30.1% | -80.3% |
| 2 | 5006ns | +42.9% | -76.9% |
| 3 | 4835ns | +45.7% | -75.8% |
| 4 | 4982ns | +39.4% | -76.3% |
| 5 | 5630ns | +39.9% | -77.3% |
| 6 | 5492ns | +45.0% | -75.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | 0.303 | moderate+ |
| rec_mut | 0.446 | moderate+ |
| rec_reuse_s60 | 0.126 | ok |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 20557.3ns | 7413.2ns | 277.3% | HIGH |
| rec_mut | 19410.0ns | 1209.1ns | 1605.3% | HIGH |
| rec_reuse_s60 | 21604.2ns | 5284.3ns | 408.8% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 6945.8-7919.6 ns)
   6945.8 |########################################
   6994.5 |
   7043.2 |########################################
   7091.9 |
   7140.6 |########################################
   7189.2 |
   7237.9 |
   7286.6 |
   7335.3 |
   7384.0 |
   7432.7 |
   7481.4 |########################################
   7530.1 |
   7578.8 |
   7627.5 |
   7676.2 |
   7724.8 |
   7773.5 |
   7822.2 |
   7870.9 |########################################
  (0 below, 1 above range)

rec_mut (n=6, range 1137.1-1305.4 ns)
   1137.1 |########################################
   1145.5 |
   1153.9 |########################################
   1162.3 |########################################
   1170.8 |
   1179.2 |########################################
   1187.6 |
   1196.0 |
   1204.4 |
   1212.8 |
   1221.2 |
   1229.7 |
   1238.1 |
   1246.5 |
   1254.9 |
   1263.3 |
   1271.7 |########################################
   1280.2 |
   1288.6 |
   1297.0 |
  (0 below, 1 above range)

rec_reuse_s60 (n=6, range 4835.0-5695.0 ns)
   4835.0 |####################
   4878.0 |
   4921.0 |
   4964.0 |########################################
   5007.0 |
   5050.0 |
   5093.0 |
   5136.0 |
   5179.0 |
   5222.0 |
   5265.0 |
   5308.0 |
   5351.0 |
   5394.0 |
   5437.0 |
   5480.0 |####################
   5523.0 |
   5566.0 |
   5609.0 |####################
   5652.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=273.0% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=1594.2% of algo (FFI overhead may distort results)
- **rec_reuse_s60**: bridge=410.3% of algo (FFI overhead may distort results)
