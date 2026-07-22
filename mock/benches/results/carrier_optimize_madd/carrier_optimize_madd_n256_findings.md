# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_madd_all dominates: 1056% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (606 ns) leads carrier_opt_madd_fold (7.01 us) by 1056%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 94% (significant)

carrier_opt_madd_all is -9.22 us (94%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_eqsat is an outlier: 88.9x slower than the field

carrier_opt_madd_eqsat (53.87 us) is 88.9x the fastest (606 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_madd_cseeqsat shows alternating (throttle bounce) (autocorr -0.84)

carrier_opt_madd_cseeqsat's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_madd_all} vs {carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_none, carrier_opt_madd_dce, carrier_opt_madd_cseeqsat, carrier_opt_madd_eqsat} (1056% apart)

The field splits into a fast tier {carrier_opt_madd_all} and a slow tier {carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_none, carrier_opt_madd_dce, carrier_opt_madd_cseeqsat, carrier_opt_madd_eqsat} with a 1056% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 88.9x the fastest

Fastest carrier_opt_madd_all (606 ns) to slowest carrier_opt_madd_eqsat (53.87 us): 88.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_madd_all** at 606.2 ns median (-93.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 88.86x (fastest 606.2 ns, slowest 53871.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 3052ns | 3090ns | 2845ns | 3056ns | 3150ns | -75.02% |
| carrier_opt_madd_cse | 11478ns | 11573ns | 10390ns | 11370ns | 12182ns | -6.04% |
| carrier_opt_madd_cseeqsat | 56077ns | 55951ns | 55558ns | 55855ns | 56670ns | +359.05% |
| carrier_opt_madd_dce | 12344ns | 12507ns | 11542ns | 12321ns | 12780ns | +1.05% |
| carrier_opt_madd_eqsat | 56120ns | 56503ns | 53548ns | 56306ns | 57126ns | +359.40% |
| carrier_opt_madd_fold | 9437ns | 9529ns | 9002ns | 9470ns | 9605ns | -22.75% |
| carrier_opt_madd_none | 12216ns | 12205ns | 11691ns | 12076ns | 12688ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 609ns | 583ns | 637ns | -93.78% | 0.420 |
| carrier_opt_madd_cse | 9043ns | 8236ns | 9545ns | -7.64% | 0.028 |
| carrier_opt_madd_cseeqsat | 53555ns | 53028ns | 54037ns | +447.00% | 0.005 |
| carrier_opt_madd_dce | 9770ns | 9222ns | 10131ns | -0.21% | 0.026 |
| carrier_opt_madd_eqsat | 53484ns | 51009ns | 54441ns | +446.27% | 0.005 |
| carrier_opt_madd_fold | 6940ns | 6493ns | 7107ns | -29.12% | 0.037 |
| carrier_opt_madd_none | 9791ns | 9332ns | 10211ns | base | 0.026 |

## Performance model

- Peak throughput: **0.439 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.422 | 96.2% |
| carrier_opt_madd_cse | 0.028 | 6.4% |
| carrier_opt_madd_cseeqsat | 0.005 | 1.1% |
| carrier_opt_madd_dce | 0.026 | 5.9% |
| carrier_opt_madd_eqsat | 0.005 | 1.1% |
| carrier_opt_madd_fold | 0.037 | 8.3% |
| carrier_opt_madd_none | 0.026 | 5.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 3052ns | 3052ns | -75.02% |
| carrier_opt_madd_cse | 11478ns | 11478ns | -6.04% |
| carrier_opt_madd_cseeqsat | 56077ns | 56077ns | +359.05% |
| carrier_opt_madd_dce | 12344ns | 12344ns | +1.05% |
| carrier_opt_madd_eqsat | 56120ns | 56120ns | +359.40% |
| carrier_opt_madd_fold | 9437ns | 9437ns | -22.75% |
| carrier_opt_madd_none | 12216ns | 12216ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 9807ns | base | --- | [9354, 10211] | --- | --- | --- | --- |
| carrier_opt_madd_all | 606ns | -9221.9ns (-94.0%) | [-9607, -8716]ns | [583, 637] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_madd_cse | 9133ns | -671.3ns (-6.8%) | [-1356, -216]ns | [8451, 9545] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| carrier_opt_madd_cseeqsat | 53524ns | +43816.6ns (+446.8%) | [+42894, +44583]ns | [53105, 54037] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_madd_dce | 9935ns | no significant difference | [-544, +613]ns | [9243, 10131] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_madd_eqsat | 53871ns | +44384.8ns (+452.6%) | [+41929, +44767]ns | [52140, 54441] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_madd_fold | 7008ns | -2831.9ns (-28.9%) | [-3421, -2300]ns | [6704, 7107] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_cse | carrier_opt_madd_cseeqsat | carrier_opt_madd_dce | carrier_opt_madd_eqsat | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|---|
| 1 | 10039ns | -94.1% | -18.0% | +430.8% | -8.1% | +441.9% | -31.1% |
| 2 | 9332ns | -93.2% | -5.9% | +476.6% | +6.6% | +476.0% | -25.2% |
| 3 | 10227ns | -93.9% | -6.2% | +418.5% | -1.5% | +398.8% | -30.7% |
| 4 | 9576ns | -93.9% | -9.5% | +466.7% | +6.4% | +463.8% | -26.5% |
| 5 | 10195ns | -94.3% | -6.9% | +421.6% | -2.7% | +422.5% | -36.3% |
| 6 | 9375ns | -93.1% | +1.3% | +473.5% | -1.2% | +481.2% | -24.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | -0.200 | moderate- |
| carrier_opt_madd_cse | -0.065 | ok |
| carrier_opt_madd_cseeqsat | -0.836 | HIGH- (thermal bounce) |
| carrier_opt_madd_dce | 0.081 | ok |
| carrier_opt_madd_eqsat | -0.239 | moderate- |
| carrier_opt_madd_fold | -0.401 | moderate- |
| carrier_opt_madd_none | -0.784 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_cse**: won 5/6, lost 1/6
- **carrier_opt_madd_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_dce**: won 4/6, lost 2/6
- **carrier_opt_madd_eqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 84059.0ns | 608.9ns | 13803.9% | HIGH |
| carrier_opt_madd_cse | 89162.8ns | 9043.0ns | 986.0% | HIGH |
| carrier_opt_madd_cseeqsat | 107380.0ns | 53555.1ns | 200.5% | HIGH |
| carrier_opt_madd_dce | 90492.3ns | 9769.7ns | 926.3% | HIGH |
| carrier_opt_madd_eqsat | 107225.9ns | 53484.0ns | 200.5% | HIGH |
| carrier_opt_madd_fold | 90795.1ns | 6939.9ns | 1308.3% | HIGH |
| carrier_opt_madd_none | 90968.5ns | 9790.7ns | 929.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 583.3-637.3 ns)
    583.3 |########################################
    586.0 |####################
    588.7 |
    591.4 |
    594.1 |
    596.8 |
    599.5 |
    602.2 |
    604.9 |
    607.6 |
    610.3 |
    613.0 |
    615.7 |
    618.4 |
    621.1 |
    623.8 |####################
    626.5 |
    629.2 |
    631.9 |####################
    634.6 |
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 8235.8-9544.8 ns)
   8235.8 |####################
   8301.2 |
   8366.7 |
   8432.1 |
   8497.6 |
   8563.0 |
   8628.5 |####################
   8693.9 |
   8759.4 |####################
   8824.8 |
   8890.3 |
   8955.7 |
   9021.2 |
   9086.6 |
   9152.1 |
   9217.5 |
   9283.0 |
   9348.4 |
   9413.9 |
   9479.3 |########################################
  (0 below, 1 above range)

carrier_opt_madd_cseeqsat (n=6, range 53027.5-54037.1 ns)
  53027.5 |########################################
  53078.0 |
  53128.5 |
  53178.9 |########################################
  53229.4 |
  53279.9 |########################################
  53330.4 |
  53380.8 |
  53431.3 |
  53481.8 |
  53532.3 |
  53582.8 |
  53633.2 |
  53683.7 |
  53734.2 |########################################
  53784.7 |########################################
  53835.1 |
  53885.6 |
  53936.1 |
  53986.6 |
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 9222.5-10131.0 ns)
   9222.5 |########################################
   9267.9 |
   9313.4 |
   9358.8 |
   9404.2 |
   9449.6 |
   9495.1 |
   9540.5 |
   9585.9 |
   9631.3 |
   9676.8 |
   9722.2 |
   9767.6 |
   9813.1 |
   9858.5 |
   9903.9 |########################################
   9949.3 |
   9994.8 |
  10040.2 |####################
  10085.6 |
  (0 below, 1 above range)

carrier_opt_madd_eqsat (n=6, range 51008.8-54441.2 ns)
  51008.8 |########################################
  51180.4 |
  51352.0 |
  51523.7 |
  51695.3 |
  51866.9 |
  52038.5 |
  52210.2 |
  52381.8 |
  52553.4 |
  52725.0 |
  52896.6 |
  53068.3 |
  53239.9 |########################################
  53411.5 |
  53583.1 |########################################
  53754.8 |
  53926.4 |########################################
  54098.0 |
  54269.6 |########################################
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 6493.3-7107.1 ns)
   6493.3 |########################################
   6524.0 |
   6554.7 |
   6585.4 |
   6616.1 |
   6646.8 |
   6677.4 |
   6708.1 |
   6738.8 |
   6769.5 |
   6800.2 |
   6830.9 |
   6861.6 |
   6892.3 |########################################
   6923.0 |
   6953.7 |########################################
   6984.3 |
   7015.0 |########################################
   7045.7 |
   7076.4 |########################################
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 9332.5-10211.0 ns)
   9332.5 |########################################
   9376.4 |
   9420.4 |
   9464.3 |
   9508.2 |
   9552.1 |####################
   9596.1 |
   9640.0 |
   9683.9 |
   9727.8 |
   9771.8 |
   9815.7 |
   9859.6 |
   9903.6 |
   9947.5 |
   9991.4 |
  10035.3 |####################
  10079.3 |
  10123.2 |
  10167.1 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=13854.8% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=974.0% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cseeqsat**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=911.0% of algo (FFI overhead may distort results)
- **carrier_opt_madd_eqsat**: bridge=200.7% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=1296.2% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=922.7% of algo (FFI overhead may distort results)
