# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_opt_real_all, carrier_opt_real_eqsat) are a dead heat (<1%)

carrier_opt_real_all (31.69 us) and carrier_opt_real_eqsat (31.93 us) differ by 0.78%, inside the noise, even though the wider field spreads 10.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_opt_real_all** at 31687.9 ns median (-8.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.10x (fastest 31687.9 ns, slowest 34888.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 34105ns | 34253ns | 33126ns | 34122ns | 34569ns | -19.62% |
| carrier_opt_real_cse | 34082ns | 34596ns | 31400ns | 34448ns | 34874ns | -19.68% |
| carrier_opt_real_cseeqsat | 34729ns | 34584ns | 34139ns | 34574ns | 35256ns | -18.15% |
| carrier_opt_real_dce | 37312ns | 37428ns | 35990ns | 37294ns | 38001ns | -12.06% |
| carrier_opt_real_eqsat | 34601ns | 34463ns | 34282ns | 34415ns | 35038ns | -18.45% |
| carrier_opt_real_fold | 37009ns | 37055ns | 36682ns | 36967ns | 37237ns | -12.78% |
| carrier_opt_real_none | 42431ns | 37115ns | 37049ns | 37093ns | 53129ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 31565ns | 30681ns | 32010ns | -20.88% | 0.032 |
| carrier_opt_real_cse | 31558ns | 29076ns | 32313ns | -20.89% | 0.032 |
| carrier_opt_real_cseeqsat | 32230ns | 31767ns | 32652ns | -19.21% | 0.032 |
| carrier_opt_real_dce | 34728ns | 33515ns | 35294ns | -12.95% | 0.029 |
| carrier_opt_real_eqsat | 32080ns | 31764ns | 32477ns | -19.59% | 0.032 |
| carrier_opt_real_fold | 34489ns | 34134ns | 34716ns | -13.55% | 0.030 |
| carrier_opt_real_none | 39893ns | 34449ns | 50620ns | base | 0.026 |

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_opt_real_cse; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.032 | 91.8% |
| carrier_opt_real_cse | 0.032 | 90.7% |
| carrier_opt_real_cseeqsat | 0.032 | 90.5% |
| carrier_opt_real_dce | 0.029 | 83.3% |
| carrier_opt_real_eqsat | 0.032 | 91.1% |
| carrier_opt_real_fold | 0.030 | 84.2% |
| carrier_opt_real_none | 0.030 | 84.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 34105ns | 34105ns | -19.62% |
| carrier_opt_real_cse | 34082ns | 34082ns | -19.68% |
| carrier_opt_real_cseeqsat | 34729ns | 34729ns | -18.15% |
| carrier_opt_real_dce | 37312ns | 37312ns | -12.06% |
| carrier_opt_real_eqsat | 34601ns | 34601ns | -18.45% |
| carrier_opt_real_fold | 37009ns | 37009ns | -12.78% |
| carrier_opt_real_none | 42431ns | 42431ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 34587ns | base | --- | [34473, 50620] | --- | --- | --- | --- |
| carrier_opt_real_all | 31688ns | -3069.6ns (-8.9%) | [-19252, -2663]ns | [30997, 32010] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_real_cse | 32046ns | -2666.6ns (-7.7%) | [-20109, -2231]ns | [30314, 32313] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_real_cseeqsat | 32120ns | -2429.7ns (-7.0%) | [-18688, -1871]ns | [31919, 32652] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_real_dce | 34889ns | no significant difference | [-16468, +709]ns | [34000, 35294] | no | 0.2625 | 0.2188 | 0 |
| carrier_opt_real_eqsat | 31934ns | -2691.9ns (-7.8%) | [-18168, -2579]ns | [31829, 32477] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_real_fold | 34532ns | no significant difference | [-16159, +186]ns | [34220, 34716] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_cse | carrier_opt_real_cseeqsat | carrier_opt_real_dce | carrier_opt_real_eqsat | carrier_opt_real_fold |
|---|---|---|---|---|---|---|---|
| 1 | 34778ns | -11.8% | -16.4% | -8.7% | +0.0% | -8.2% | -0.7% |
| 2 | 34639ns | -9.6% | -6.8% | -7.1% | +2.7% | -7.8% | -1.5% |
| 3 | 34498ns | -8.2% | -6.8% | -4.0% | +1.4% | -7.6% | +0.1% |
| 4 | 34449ns | -8.0% | -6.2% | -6.9% | +0.1% | -7.8% | +1.0% |
| 5 | 34535ns | -7.4% | -8.6% | -6.9% | +1.4% | -7.4% | -0.7% |
| 6 | 66461ns | -51.8% | -51.9% | -51.7% | -49.6% | -50.4% | -47.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | 0.357 | moderate+ |
| carrier_opt_real_cse | -0.122 | ok |
| carrier_opt_real_cseeqsat | -0.134 | ok |
| carrier_opt_real_dce | -0.095 | ok |
| carrier_opt_real_eqsat | 0.050 | ok |
| carrier_opt_real_fold | -0.362 | moderate- |
| carrier_opt_real_none | -0.034 | ok |

**Consistency summary:**

- **carrier_opt_real_all**: won 6/6, lost 0/6
- **carrier_opt_real_cse**: won 6/6, lost 0/6
- **carrier_opt_real_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_real_dce**: won 1/6, lost 4/6
- **carrier_opt_real_eqsat**: won 6/6, lost 0/6
- **carrier_opt_real_fold**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 94868.9ns | 31565.0ns | 300.6% | HIGH |
| carrier_opt_real_cse | 94855.4ns | 31557.6ns | 300.6% | HIGH |
| carrier_opt_real_cseeqsat | 96832.3ns | 32230.3ns | 300.4% | HIGH |
| carrier_opt_real_dce | 104129.0ns | 34727.9ns | 299.8% | HIGH |
| carrier_opt_real_eqsat | 95795.3ns | 32080.1ns | 298.6% | HIGH |
| carrier_opt_real_fold | 103670.8ns | 34489.2ns | 300.6% | HIGH |
| carrier_opt_real_none | 109410.6ns | 39893.2ns | 274.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 30680.8-32010.4 ns)
  30680.8 |####################
  30747.3 |
  30813.8 |
  30880.2 |
  30946.7 |
  31013.2 |
  31079.7 |
  31146.2 |
  31212.6 |
  31279.1 |####################
  31345.6 |
  31412.1 |
  31478.6 |
  31545.0 |
  31611.5 |
  31678.0 |########################################
  31744.5 |
  31811.0 |
  31877.4 |
  31943.9 |####################
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 29076.2-32312.9 ns)
  29076.2 |########################################
  29238.0 |
  29399.9 |
  29561.7 |
  29723.5 |
  29885.4 |
  30047.2 |
  30209.0 |
  30370.9 |
  30532.7 |
  30694.6 |
  30856.4 |
  31018.2 |
  31180.1 |
  31341.9 |
  31503.7 |########################################
  31665.6 |
  31827.4 |########################################
  31989.2 |########################################
  32151.1 |########################################
  (0 below, 1 above range)

carrier_opt_real_cseeqsat (n=6, range 31766.7-32652.3 ns)
  31766.7 |########################################
  31811.0 |
  31855.3 |
  31899.5 |
  31943.8 |
  31988.1 |
  32032.4 |########################################
  32076.7 |########################################
  32120.9 |########################################
  32165.2 |########################################
  32209.5 |
  32253.8 |
  32298.1 |
  32342.3 |
  32386.6 |
  32430.9 |
  32475.2 |
  32519.5 |
  32563.7 |
  32608.0 |
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 33515.0-35294.2 ns)
  33515.0 |####################
  33604.0 |
  33692.9 |
  33781.9 |
  33870.8 |
  33959.8 |
  34048.8 |
  34137.7 |
  34226.7 |
  34315.6 |
  34404.6 |####################
  34493.6 |
  34582.5 |
  34671.5 |
  34760.4 |####################
  34849.4 |
  34938.4 |########################################
  35027.3 |
  35116.3 |
  35205.2 |
  (0 below, 1 above range)

carrier_opt_real_eqsat (n=6, range 31764.2-32477.3 ns)
  31764.2 |####################
  31799.9 |
  31835.5 |
  31871.2 |####################
  31906.8 |########################################
  31942.5 |
  31978.1 |####################
  32013.8 |
  32049.4 |
  32085.1 |
  32120.8 |
  32156.4 |
  32192.1 |
  32227.7 |
  32263.4 |
  32299.0 |
  32334.7 |
  32370.3 |
  32406.0 |
  32441.6 |
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 34133.8-34715.8 ns)
  34133.8 |####################
  34162.9 |
  34192.0 |
  34221.1 |
  34250.2 |
  34279.3 |####################
  34308.4 |
  34337.5 |
  34366.6 |
  34395.7 |
  34424.8 |
  34453.9 |
  34483.0 |
  34512.1 |########################################
  34541.2 |
  34570.3 |
  34599.4 |
  34628.5 |####################
  34657.6 |
  34686.7 |
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 34448.8-50619.6 ns)
  34448.8 |########################################
  35257.3 |
  36065.9 |
  36874.4 |
  37683.0 |
  38491.5 |
  39300.0 |
  40108.6 |
  40917.1 |
  41725.6 |
  42534.2 |
  43342.7 |
  44151.2 |
  44959.8 |
  45768.3 |
  46576.9 |
  47385.4 |
  48193.9 |
  49002.5 |
  49811.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=300.5% of algo (FFI overhead may distort results)
- **carrier_opt_real_cseeqsat**: bridge=300.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=299.8% of algo (FFI overhead may distort results)
- **carrier_opt_real_eqsat**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=300.5% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: CV=29.8% (high variance, measurements may be unstable)
- **carrier_opt_real_none**: bridge=300.5% of algo (FFI overhead may distort results)
