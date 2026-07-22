# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_madd_regcache dominates: 19% faster than the next best (carrier_pre_madd_null)

carrier_pre_madd_regcache (38.05 us) leads carrier_pre_madd_null (45.44 us) by 19%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_madd_regcache beats baseline by 24% (significant)

carrier_pre_madd_regcache is -12.17 us (24%) faster than baseline carrier_pre_madd_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_pre_madd_regcache** at 38050.0 ns median (-24.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.32x (fastest 38050.0 ns, slowest 50378.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 52710ns | 52655ns | 52197ns | 52581ns | 53160ns | +0.30% |
| carrier_pre_madd_fntable | 50952ns | 51486ns | 48453ns | 51083ns | 52005ns | -3.05% |
| carrier_pre_madd_null | 47432ns | 47752ns | 44995ns | 47652ns | 48321ns | -9.74% |
| carrier_pre_madd_regcache | 40444ns | 40348ns | 40165ns | 40326ns | 40761ns | -23.04% |
| carrier_pre_madd_switch | 52553ns | 52657ns | 51755ns | 52534ns | 52981ns | base |
| carrier_pre_madd_threaded | 52110ns | 51940ns | 51459ns | 51864ns | 52805ns | -0.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 50411ns | 49880ns | 50838ns | +0.30% | 0.020 |
| carrier_pre_madd_fntable | 48717ns | 46310ns | 49738ns | -3.07% | 0.021 |
| carrier_pre_madd_null | 45151ns | 42791ns | 46024ns | -10.17% | 0.023 |
| carrier_pre_madd_regcache | 38127ns | 37867ns | 38390ns | -24.14% | 0.027 |
| carrier_pre_madd_switch | 50262ns | 49498ns | 50664ns | base | 0.020 |
| carrier_pre_madd_threaded | 49811ns | 49157ns | 50495ns | -0.90% | 0.021 |

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_pre_madd_regcache; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.020 | 75.2% |
| carrier_pre_madd_fntable | 0.021 | 76.9% |
| carrier_pre_madd_null | 0.023 | 83.3% |
| carrier_pre_madd_regcache | 0.027 | 99.5% |
| carrier_pre_madd_switch | 0.020 | 75.2% |
| carrier_pre_madd_threaded | 0.021 | 76.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 52710ns | 52710ns | +0.30% |
| carrier_pre_madd_fntable | 50952ns | 50952ns | -3.05% |
| carrier_pre_madd_null | 47432ns | 47432ns | -9.74% |
| carrier_pre_madd_regcache | 40444ns | 40444ns | -23.04% |
| carrier_pre_madd_switch | 52553ns | 52553ns | base |
| carrier_pre_madd_threaded | 52110ns | 52110ns | -0.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 50361ns | base | --- | [49761, 50664] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 50378ns | no significant difference | [-592, +876]ns | [50018, 50838] | no | 1.0000 | 1.0000 | 0 |
| carrier_pre_madd_fntable | 49220ns | -1169.6ns (-2.3%) | [-3265, -201]ns | [47192, 49738] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_null | 45439ns | -5066.4ns (-10.1%) | [-6529, -3738]ns | [43991, 46024] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 38050ns | -12169.6ns (-24.2%) | [-12577, -11657]ns | [37942, 38390] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 49660ns | no significant difference | [-1084, +418]ns | [49277, 50495] | no | 0.8594 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 50299ns | +1.5% | -7.9% | -9.4% | -24.3% | -2.3% |
| 2 | 49498ns | +2.0% | -0.6% | -6.7% | -23.2% | +0.3% |
| 3 | 50712ns | -0.9% | -2.9% | -10.7% | -23.9% | -0.9% |
| 4 | 50423ns | -0.5% | -1.7% | -15.1% | -24.6% | -2.0% |
| 5 | 50025ns | +1.2% | -0.2% | -8.4% | -23.7% | +1.4% |
| 6 | 50615ns | -1.5% | -5.0% | -10.7% | -25.2% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | -0.103 | ok |
| carrier_pre_madd_fntable | -0.025 | ok |
| carrier_pre_madd_null | -0.180 | ok |
| carrier_pre_madd_regcache | -0.342 | moderate- |
| carrier_pre_madd_switch | -0.424 | moderate- |
| carrier_pre_madd_threaded | -0.386 | moderate- |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 3/6, lost 3/6
- **carrier_pre_madd_fntable**: won 6/6, lost 0/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 6/6, lost 0/6
- **carrier_pre_madd_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 104386.6ns | 50411.3ns | 207.1% | HIGH |
| carrier_pre_madd_fntable | 100164.4ns | 48716.7ns | 205.6% | HIGH |
| carrier_pre_madd_null | 95965.4ns | 45151.2ns | 212.5% | HIGH |
| carrier_pre_madd_regcache | 116732.9ns | 38127.4ns | 306.2% | HIGH |
| carrier_pre_madd_switch | 102969.0ns | 50262.1ns | 204.9% | HIGH |
| carrier_pre_madd_threaded | 102546.0ns | 49811.0ns | 205.9% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 49879.6-50837.7 ns)
  49879.6 |########################################
  49927.5 |
  49975.4 |
  50023.3 |
  50071.2 |
  50119.1 |########################################
  50167.0 |
  50214.9 |
  50262.8 |########################################
  50310.7 |
  50358.6 |
  50406.6 |
  50454.5 |########################################
  50502.4 |
  50550.3 |
  50598.2 |########################################
  50646.1 |
  50694.0 |
  50741.9 |
  50789.8 |
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 46310.4-49737.9 ns)
  46310.4 |########################################
  46481.8 |
  46653.2 |
  46824.5 |
  46995.9 |
  47167.3 |
  47338.7 |
  47510.0 |
  47681.4 |
  47852.8 |
  48024.1 |########################################
  48195.5 |
  48366.9 |
  48538.3 |
  48709.6 |
  48881.0 |
  49052.4 |########################################
  49223.8 |########################################
  49395.1 |########################################
  49566.5 |
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 42790.8-46023.6 ns)
  42790.8 |########################################
  42952.4 |
  43114.1 |
  43275.7 |
  43437.4 |
  43599.0 |
  43760.6 |
  43922.3 |
  44083.9 |
  44245.5 |
  44407.2 |
  44568.8 |
  44730.5 |
  44892.1 |
  45053.7 |########################################
  45215.4 |########################################
  45377.0 |
  45538.6 |########################################
  45700.3 |########################################
  45861.9 |
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 37867.1-38390.2 ns)
  37867.1 |########################################
  37893.3 |
  37919.4 |
  37945.6 |
  37971.7 |
  37997.9 |########################################
  38024.0 |########################################
  38050.2 |########################################
  38076.3 |
  38102.5 |
  38128.6 |
  38154.8 |########################################
  38181.0 |
  38207.1 |
  38233.3 |
  38259.4 |
  38285.6 |
  38311.7 |
  38337.9 |
  38364.0 |
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 49497.9-50663.8 ns)
  49497.9 |########################################
  49556.2 |
  49614.5 |
  49672.8 |
  49731.1 |
  49789.4 |
  49847.7 |
  49905.9 |
  49964.2 |
  50022.5 |########################################
  50080.8 |
  50139.1 |
  50197.4 |
  50255.7 |########################################
  50314.0 |
  50372.3 |########################################
  50430.6 |
  50488.9 |
  50547.2 |
  50605.5 |########################################
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 49157.1-50495.4 ns)
  49157.1 |####################
  49224.0 |
  49290.9 |
  49357.9 |####################
  49424.8 |
  49491.7 |
  49558.6 |
  49625.5 |########################################
  49692.4 |
  49759.4 |
  49826.3 |
  49893.2 |
  49960.1 |
  50027.0 |
  50093.9 |
  50160.9 |
  50227.8 |####################
  50294.7 |
  50361.6 |
  50428.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=207.0% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=205.8% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=206.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=306.3% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=205.0% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=205.7% of algo (FFI overhead may distort results)
