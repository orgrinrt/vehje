# Record update (0% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s00**

## Highlights

Baseline for all deltas below: **rec_reuse_s00**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 14% faster than the next best (rec_reuse_s00)

rec_mut (67.06 us) leads rec_reuse_s00 (76.47 us) by 14%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_copy is an outlier: 6.8x slower than the field

rec_copy (452.89 us) is 6.8x the fastest (67.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rec_mut is fastest but the noisiest (CV 5.7%)

rec_mut wins on median (67.06 us) yet has the highest variance (CV 5.7%), while rec_copy is the steadiest (CV 0.9%, 452.89 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### rec_reuse_s00 shows alternating (throttle bounce) (autocorr -0.61)

rec_reuse_s00's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.8x the fastest

Fastest rec_mut (67.06 us) to slowest rec_copy (452.89 us): 6.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 67057.4 ns median (-12.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.75x (fastest 67057.4 ns, slowest 452891.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 453849ns | 455158ns | 447012ns | 453270ns | 458135ns | +486.09% |
| rec_mut | 69209ns | 69432ns | 63796ns | 68176ns | 73465ns | -10.62% |
| rec_reuse_s00 | 77436ns | 78955ns | 71719ns | 76852ns | 81172ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 451577ns | 444789ns | 455800ns | +501.82% | 0.036 |
| rec_mut | 66847ns | 61630ns | 70954ns | -10.91% | 0.245 |
| rec_reuse_s00 | 75036ns | 69514ns | 78691ns | base | 0.218 |

## Performance model

- Peak throughput: **0.266 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.036 | 13.6% |
| rec_mut | 0.244 | 91.9% |
| rec_reuse_s00 | 0.214 | 80.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 453849ns | 453849ns | +486.09% |
| rec_mut | 69209ns | 69209ns | -10.62% |
| rec_reuse_s00 | 77436ns | 77436ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s00 | 76470ns | base | --- | [69946, 78691] | --- | --- | --- | --- |
| rec_copy | 452892ns | +376079.8ns (+491.8%) | [+369571, +383975]ns | [446040, 455800] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 67057ns | -6775.5ns (-8.9%) | [-12728, -5064]ns | [62529, 70954] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s00 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 78196ns | +468.8% | -21.2% |
| 2 | 70377ns | +545.9% | -8.7% |
| 3 | 78603ns | +481.4% | -9.4% |
| 4 | 74743ns | +498.4% | -5.4% |
| 5 | 78780ns | +474.4% | -11.3% |
| 6 | 69514ns | +552.0% | -8.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | -0.280 | moderate- |
| rec_mut | 0.229 | moderate+ |
| rec_reuse_s00 | -0.608 | HIGH- (thermal bounce) |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21489.8ns | 451577.4ns | 4.8% |  |
| rec_mut | 22523.0ns | 66846.7ns | 33.7% | HIGH |
| rec_reuse_s00 | 22782.4ns | 75035.5ns | 30.4% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 444788.8-455799.8 ns)
  444788.8 |########################################
  445339.3 |
  445889.9 |
  446440.5 |
  446991.0 |########################################
  447541.5 |
  448092.1 |
  448642.6 |
  449193.2 |
  449743.8 |
  450294.3 |
  450844.8 |
  451395.4 |
  451946.0 |
  452496.5 |########################################
  453047.0 |########################################
  453597.6 |
  454148.1 |########################################
  454698.7 |
  455249.2 |
  (0 below, 1 above range)

rec_mut (n=6, range 61630.0-70953.8 ns)
  61630.0 |########################################
  62096.2 |
  62562.4 |
  63028.6 |########################################
  63494.8 |
  63960.9 |########################################
  64427.1 |
  64893.3 |
  65359.5 |
  65825.7 |
  66291.9 |
  66758.1 |
  67224.2 |
  67690.4 |
  68156.6 |
  68622.8 |
  69089.0 |
  69555.2 |########################################
  70021.4 |
  70487.6 |########################################
  (0 below, 1 above range)

rec_reuse_s00 (n=6, range 69514.2-78691.4 ns)
  69514.2 |########################################
  69973.1 |########################################
  70431.9 |
  70890.8 |
  71349.6 |
  71808.5 |
  72267.4 |
  72726.2 |
  73185.1 |
  73644.0 |
  74102.8 |
  74561.7 |########################################
  75020.6 |
  75479.4 |
  75938.3 |
  76397.1 |
  76856.0 |
  77314.9 |
  77773.7 |########################################
  78232.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **rec_mut**: bridge=33.8% of algo (FFI overhead may distort results)
- **rec_reuse_s00**: bridge=30.3% of algo (FFI overhead may distort results)
