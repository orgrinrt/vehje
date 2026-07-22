# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_tight_all dominates: 938% faster than the next best (carrier_opt_tight_fold)

carrier_opt_tight_all (2.62 us) leads carrier_opt_tight_fold (27.18 us) by 938%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 93% (significant)

carrier_opt_tight_all is -33.13 us (93%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_eqsat is an outlier: 41.2x slower than the field

carrier_opt_tight_eqsat (107.79 us) is 41.2x the fastest (2.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_tight_cseeqsat shows alternating (throttle bounce) (autocorr -0.68)

carrier_opt_tight_cseeqsat's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_dce, carrier_opt_tight_cseeqsat, carrier_opt_tight_eqsat} (938% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_dce, carrier_opt_tight_cseeqsat, carrier_opt_tight_eqsat} with a 938% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 41.2x the fastest

Fastest carrier_opt_tight_all (2.62 us) to slowest carrier_opt_tight_eqsat (107.79 us): 41.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_tight_all** at 2618.9 ns median (-92.7% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 41.16x (fastest 2618.9 ns, slowest 107794.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 5086ns | 5098ns | 4952ns | 5091ns | 5145ns | -86.84% |
| carrier_opt_tight_cse | 36614ns | 37287ns | 34116ns | 36283ns | 38359ns | -5.24% |
| carrier_opt_tight_cseeqsat | 109478ns | 109332ns | 107678ns | 109265ns | 110697ns | +183.35% |
| carrier_opt_tight_dce | 38271ns | 38295ns | 37660ns | 38204ns | 38677ns | -0.95% |
| carrier_opt_tight_eqsat | 110470ns | 110253ns | 109055ns | 110081ns | 111760ns | +185.91% |
| carrier_opt_tight_fold | 29774ns | 29704ns | 29318ns | 29609ns | 30251ns | -22.94% |
| carrier_opt_tight_none | 38637ns | 38178ns | 37705ns | 38022ns | 40028ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 2621ns | 2585ns | 2656ns | -92.75% | 0.391 |
| carrier_opt_tight_cse | 34268ns | 31923ns | 35869ns | -5.21% | 0.030 |
| carrier_opt_tight_cseeqsat | 107095ns | 105430ns | 108407ns | +196.24% | 0.010 |
| carrier_opt_tight_dce | 35886ns | 35313ns | 36283ns | -0.73% | 0.029 |
| carrier_opt_tight_eqsat | 108038ns | 106796ns | 109217ns | +198.84% | 0.009 |
| carrier_opt_tight_fold | 27275ns | 26884ns | 27760ns | -24.55% | 0.038 |
| carrier_opt_tight_none | 36152ns | 35352ns | 37356ns | base | 0.028 |

## Performance model

- Peak throughput: **0.396 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.391 | 98.7% |
| carrier_opt_tight_cse | 0.029 | 7.4% |
| carrier_opt_tight_cseeqsat | 0.010 | 2.4% |
| carrier_opt_tight_dce | 0.029 | 7.2% |
| carrier_opt_tight_eqsat | 0.009 | 2.4% |
| carrier_opt_tight_fold | 0.038 | 9.5% |
| carrier_opt_tight_none | 0.029 | 7.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 5086ns | 5086ns | -86.84% |
| carrier_opt_tight_cse | 36614ns | 36614ns | -5.24% |
| carrier_opt_tight_cseeqsat | 109478ns | 109478ns | +183.35% |
| carrier_opt_tight_dce | 38271ns | 38271ns | -0.95% |
| carrier_opt_tight_eqsat | 110470ns | 110470ns | +185.91% |
| carrier_opt_tight_fold | 29774ns | 29774ns | -22.94% |
| carrier_opt_tight_none | 38637ns | 38637ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 35739ns | base | --- | [35361, 37356] | --- | --- | --- | --- |
| carrier_opt_tight_all | 2619ns | -33127.9ns (-92.7%) | [-34759, -32705]ns | [2589, 2656] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_cse | 34923ns | -2067.7ns (-5.8%) | [-3349, -235]ns | [32012, 35869] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| carrier_opt_tight_cseeqsat | 106901ns | +70420.2ns (+197.0%) | [+69698, +72711]ns | [105978, 108407] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_dce | 35920ns | no significant difference | [-1074, +460]ns | [35457, 36283] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_tight_eqsat | 107794ns | +71860.4ns (+201.1%) | [+71365, +72433]ns | [107104, 109217] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_fold | 27181ns | -8854.8ns (-24.8%) | [-9993, -7783]ns | [26884, 27760] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_cse | carrier_opt_tight_cseeqsat | carrier_opt_tight_dce | carrier_opt_tight_eqsat | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|---|
| 1 | 35352ns | -92.5% | -9.2% | +202.5% | +2.1% | +204.1% | -21.5% |
| 2 | 36066ns | -92.8% | -2.8% | +196.3% | -0.9% | +197.8% | -25.5% |
| 3 | 36797ns | -92.9% | -2.4% | +189.5% | -1.5% | +195.9% | -26.8% |
| 4 | 35412ns | -92.6% | +1.2% | +208.5% | +0.5% | +201.6% | -24.1% |
| 5 | 35369ns | -92.5% | -9.7% | +198.1% | -0.2% | +205.6% | -22.5% |
| 6 | 37916ns | -93.2% | -8.3% | +183.7% | -4.2% | +188.9% | -26.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | -0.369 | moderate- |
| carrier_opt_tight_cse | -0.170 | ok |
| carrier_opt_tight_cseeqsat | -0.678 | HIGH- (thermal bounce) |
| carrier_opt_tight_dce | -0.333 | moderate- |
| carrier_opt_tight_eqsat | -0.236 | moderate- |
| carrier_opt_tight_fold | 0.103 | ok |
| carrier_opt_tight_none | -0.237 | moderate- |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_cse**: won 5/6, lost 1/6
- **carrier_opt_tight_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_dce**: won 4/6, lost 2/6
- **carrier_opt_tight_eqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 85842.8ns | 2621.3ns | 3274.8% | HIGH |
| carrier_opt_tight_cse | 103008.0ns | 34268.1ns | 300.6% | HIGH |
| carrier_opt_tight_cseeqsat | 107527.9ns | 107095.2ns | 100.4% | HIGH |
| carrier_opt_tight_dce | 107898.3ns | 35886.4ns | 300.7% | HIGH |
| carrier_opt_tight_eqsat | 108381.4ns | 108038.2ns | 100.3% | HIGH |
| carrier_opt_tight_fold | 108664.1ns | 27275.0ns | 398.4% | HIGH |
| carrier_opt_tight_none | 108564.8ns | 36152.1ns | 300.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 2584.6-2655.6 ns)
   2584.6 |########################################
   2588.2 |
   2591.7 |########################################
   2595.2 |
   2598.8 |
   2602.3 |
   2605.9 |
   2609.4 |########################################
   2613.0 |
   2616.5 |
   2620.1 |
   2623.7 |
   2627.2 |########################################
   2630.8 |
   2634.3 |
   2637.8 |
   2641.4 |
   2644.9 |
   2648.5 |
   2652.0 |########################################
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 31922.9-35869.2 ns)
  31922.9 |########################################
  32120.2 |
  32317.5 |
  32514.8 |
  32712.2 |
  32909.5 |
  33106.8 |
  33304.1 |
  33501.4 |
  33698.7 |
  33896.0 |
  34093.3 |
  34290.7 |
  34488.0 |
  34685.3 |####################
  34882.6 |####################
  35079.9 |
  35277.2 |
  35474.5 |
  35671.8 |####################
  (0 below, 1 above range)

carrier_opt_tight_cseeqsat (n=6, range 105429.6-108406.6 ns)
  105429.6 |########################################
  105578.5 |
  105727.3 |
  105876.2 |
  106025.0 |
  106173.9 |
  106322.7 |
  106471.6 |########################################
  106620.4 |
  106769.3 |########################################
  106918.1 |########################################
  107067.0 |
  107215.8 |
  107364.7 |
  107513.5 |########################################
  107662.4 |
  107811.2 |
  107960.1 |
  108108.9 |
  108257.8 |
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 35313.3-36282.7 ns)
  35313.3 |########################################
  35361.8 |
  35410.2 |
  35458.7 |
  35507.2 |
  35555.7 |########################################
  35604.1 |
  35652.6 |
  35701.1 |
  35749.5 |########################################
  35798.0 |
  35846.5 |
  35894.9 |
  35943.4 |
  35991.9 |
  36040.3 |########################################
  36088.8 |
  36137.3 |
  36185.8 |
  36234.2 |########################################
  (0 below, 1 above range)

carrier_opt_tight_eqsat (n=6, range 106795.8-109216.6 ns)
  106795.8 |####################
  106916.8 |
  107037.9 |
  107158.9 |
  107280.0 |
  107401.0 |########################################
  107522.1 |
  107643.1 |
  107764.1 |
  107885.2 |
  108006.2 |####################
  108127.3 |
  108248.3 |
  108369.4 |
  108490.4 |
  108611.4 |
  108732.5 |
  108853.5 |####################
  108974.6 |
  109095.6 |
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 26884.2-27759.8 ns)
  26884.2 |########################################
  26928.0 |####################
  26971.8 |
  27015.5 |
  27059.3 |
  27103.1 |
  27146.9 |
  27190.6 |
  27234.4 |
  27278.2 |
  27322.0 |
  27365.8 |
  27409.5 |####################
  27453.3 |
  27497.1 |
  27540.9 |
  27584.6 |
  27628.4 |
  27672.2 |
  27716.0 |####################
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 35352.5-37356.2 ns)
  35352.5 |########################################
  35452.7 |
  35552.9 |
  35653.1 |
  35753.2 |
  35853.4 |
  35953.6 |
  36053.8 |#############
  36154.0 |
  36254.2 |
  36354.4 |
  36454.6 |
  36554.8 |
  36654.9 |
  36755.1 |#############
  36855.3 |
  36955.5 |
  37055.7 |
  37155.9 |
  37256.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=3271.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cseeqsat**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_tight_eqsat**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=399.5% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=300.5% of algo (FFI overhead may distort results)
