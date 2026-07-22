# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_leaf_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_leaf_none has the worst median (1.02 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_leaf_all at 151.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_leaf_all dominates: 95% faster than the next best (carrier_opt_leaf_eqsat)

carrier_opt_leaf_all (151.52 us) leads carrier_opt_leaf_eqsat (295.12 us) by 95%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_leaf_all beats baseline by 85% (significant)

carrier_opt_leaf_all is -870.84 us (85%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_none is an outlier: 6.7x slower than the field

carrier_opt_leaf_none (1.02 ms) is 6.7x the fastest (151.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_leaf_eqsat shows alternating (throttle bounce) (autocorr -0.55)

carrier_opt_leaf_eqsat's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_leaf_all, carrier_opt_leaf_eqsat, carrier_opt_leaf_cseeqsat, carrier_opt_leaf_cse, carrier_opt_leaf_fold} vs {carrier_opt_leaf_dce, carrier_opt_leaf_none} (108% apart)

The field splits into a fast tier {carrier_opt_leaf_all, carrier_opt_leaf_eqsat, carrier_opt_leaf_cseeqsat, carrier_opt_leaf_cse, carrier_opt_leaf_fold} and a slow tier {carrier_opt_leaf_dce, carrier_opt_leaf_none} with a 108% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.7x the fastest

Fastest carrier_opt_leaf_all (151.52 us) to slowest carrier_opt_leaf_none (1.02 ms): 6.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 151518.8 ns median (-85.2% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 6.75x (fastest 151518.8 ns, slowest 1022055.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 154108ns | 154053ns | 149825ns | 153969ns | 156457ns | -84.97% |
| carrier_opt_leaf_cse | 345514ns | 344535ns | 342580ns | 343980ns | 349281ns | -66.31% |
| carrier_opt_leaf_cseeqsat | 297890ns | 297856ns | 296303ns | 297795ns | 298824ns | -70.95% |
| carrier_opt_leaf_dce | 1019082ns | 1020342ns | 1008623ns | 1020130ns | 1022739ns | -0.64% |
| carrier_opt_leaf_eqsat | 298659ns | 298216ns | 296723ns | 298097ns | 300469ns | -70.88% |
| carrier_opt_leaf_fold | 493615ns | 492995ns | 488955ns | 492571ns | 497512ns | -51.87% |
| carrier_opt_leaf_none | 1025608ns | 1025862ns | 1017519ns | 1025223ns | 1030230ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 151420ns | 147125ns | 153706ns | -85.18% | 0.108 |
| carrier_opt_leaf_cse | 342456ns | 339661ns | 346473ns | -66.49% | 0.048 |
| carrier_opt_leaf_cseeqsat | 295061ns | 293565ns | 295891ns | -71.12% | 0.056 |
| carrier_opt_leaf_dce | 1015398ns | 1005592ns | 1018790ns | -0.63% | 0.016 |
| carrier_opt_leaf_eqsat | 295671ns | 293909ns | 297572ns | -71.06% | 0.055 |
| carrier_opt_leaf_fold | 490400ns | 485636ns | 494569ns | -52.01% | 0.033 |
| carrier_opt_leaf_none | 1021803ns | 1013541ns | 1026424ns | base | 0.016 |

## Performance model

- Peak throughput: **0.111 Gops/s** (carrier_opt_leaf_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.108 | 97.1% |
| carrier_opt_leaf_cse | 0.048 | 43.1% |
| carrier_opt_leaf_cseeqsat | 0.056 | 49.8% |
| carrier_opt_leaf_dce | 0.016 | 14.5% |
| carrier_opt_leaf_eqsat | 0.056 | 49.9% |
| carrier_opt_leaf_fold | 0.033 | 30.0% |
| carrier_opt_leaf_none | 0.016 | 14.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 154108ns | 154108ns | -84.97% |
| carrier_opt_leaf_cse | 345514ns | 345514ns | -66.31% |
| carrier_opt_leaf_cseeqsat | 297890ns | 297890ns | -70.95% |
| carrier_opt_leaf_dce | 1019082ns | 1019082ns | -0.64% |
| carrier_opt_leaf_eqsat | 298659ns | 298659ns | -70.88% |
| carrier_opt_leaf_fold | 493615ns | 493615ns | -51.87% |
| carrier_opt_leaf_none | 1025608ns | 1025608ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 1022056ns | base | --- | [1016931, 1026424] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 151519ns | -870841.4ns (-85.2%) | [-875875, -864435]ns | [149034, 153706] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 341180ns | -678600.8ns (-66.4%) | [-686708, -672734]ns | [339715, 346473] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_cseeqsat | 295141ns | -726423.3ns (-71.1%) | [-731790, -722013]ns | [294152, 295891] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 1016614ns | no significant difference | [-14975, +641]ns | [1010791, 1018790] | no | 0.2188 | 0.2188 | 0 |
| carrier_opt_leaf_eqsat | 295116ns | -726851.0ns (-71.1%) | [-730072, -721475]ns | [294323, 297572] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_fold | 489638ns | -531417.3ns (-52.0%) | [-536812, -525981]ns | [486993, 494569] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_cse | carrier_opt_leaf_cseeqsat | carrier_opt_leaf_dce | carrier_opt_leaf_eqsat | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|---|
| 1 | 1013541ns | -85.0% | -66.5% | -70.9% | +0.3% | -70.9% | -52.1% |
| 2 | 1022027ns | -85.2% | -66.5% | -71.0% | -0.5% | -71.2% | -52.2% |
| 3 | 1026668ns | -85.7% | -66.9% | -71.2% | -0.9% | -71.2% | -52.4% |
| 4 | 1022085ns | -85.2% | -65.7% | -71.1% | -0.2% | -70.9% | -51.8% |
| 5 | 1020321ns | -85.0% | -66.4% | -71.1% | -0.4% | -71.2% | -51.4% |
| 6 | 1026179ns | -85.0% | -66.9% | -71.4% | -2.0% | -71.0% | -52.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | 0.239 | moderate+ |
| carrier_opt_leaf_cse | -0.255 | moderate- |
| carrier_opt_leaf_cseeqsat | 0.318 | moderate+ |
| carrier_opt_leaf_dce | 0.080 | ok |
| carrier_opt_leaf_eqsat | -0.546 | HIGH- (thermal bounce) |
| carrier_opt_leaf_fold | 0.361 | moderate+ |
| carrier_opt_leaf_none | -0.056 | ok |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 5/6, lost 1/6
- **carrier_opt_leaf_eqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 151952.1ns | 151419.7ns | 100.4% | HIGH |
| carrier_opt_leaf_cse | 342923.6ns | 342455.9ns | 100.1% | HIGH |
| carrier_opt_leaf_cseeqsat | 295581.9ns | 295061.4ns | 100.2% | HIGH |
| carrier_opt_leaf_dce | 1016441.0ns | 1015398.3ns | 100.1% | HIGH |
| carrier_opt_leaf_eqsat | 296241.6ns | 295670.6ns | 100.2% | HIGH |
| carrier_opt_leaf_fold | 492543.6ns | 490399.8ns | 100.4% | HIGH |
| carrier_opt_leaf_none | 1022956.7ns | 1021803.5ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 147125.0-153705.9 ns)
  147125.0 |####################
  147454.0 |
  147783.1 |
  148112.1 |
  148441.2 |
  148770.2 |
  149099.3 |
  149428.3 |
  149757.3 |
  150086.4 |
  150415.4 |
  150744.5 |####################
  151073.5 |
  151402.6 |########################################
  151731.6 |
  152060.6 |
  152389.7 |
  152718.7 |
  153047.8 |
  153376.8 |####################
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 339660.8-346472.7 ns)
  339660.8 |########################################
  340001.4 |
  340342.0 |
  340682.6 |
  341023.2 |
  341363.8 |
  341704.4 |
  342045.0 |
  342385.6 |##########################
  342726.2 |
  343066.8 |
  343407.3 |
  343747.9 |
  344088.5 |
  344429.1 |
  344769.7 |
  345110.3 |
  345450.9 |
  345791.5 |
  346132.1 |
  (0 below, 1 above range)

carrier_opt_leaf_cseeqsat (n=6, range 293564.6-295891.5 ns)
  293564.6 |####################
  293680.9 |
  293797.3 |
  293913.6 |
  294030.0 |
  294146.3 |
  294262.7 |
  294379.0 |
  294495.3 |
  294611.7 |
  294728.0 |####################
  294844.4 |
  294960.7 |
  295077.1 |########################################
  295193.4 |
  295309.7 |
  295426.1 |
  295542.4 |
  295658.8 |####################
  295775.1 |
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 1005592.5-1018789.6 ns)
  1005592.5 |####################
  1006252.4 |
  1006912.2 |
  1007572.1 |
  1008231.9 |
  1008891.8 |
  1009551.6 |
  1010211.5 |
  1010871.3 |
  1011531.2 |
  1012191.1 |
  1012850.9 |
  1013510.8 |
  1014170.6 |
  1014830.5 |
  1015490.3 |####################
  1016150.2 |########################################
  1016810.0 |####################
  1017469.9 |
  1018129.7 |
  (0 below, 1 above range)

carrier_opt_leaf_eqsat (n=6, range 293908.8-297572.5 ns)
  293908.8 |########################################
  294092.0 |
  294275.2 |
  294458.4 |
  294641.5 |########################################
  294824.7 |########################################
  295007.9 |
  295191.1 |########################################
  295374.3 |
  295557.5 |
  295740.7 |
  295923.8 |
  296107.0 |
  296290.2 |
  296473.4 |
  296656.6 |
  296839.8 |
  297022.9 |
  297206.1 |
  297389.3 |########################################
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 485636.2-494568.9 ns)
  485636.2 |####################
  486082.8 |
  486529.5 |
  486976.1 |
  487422.8 |
  487869.4 |
  488316.0 |########################################
  488762.7 |
  489209.3 |
  489655.9 |
  490102.6 |
  490549.2 |####################
  490995.8 |
  491442.5 |
  491889.1 |
  492335.8 |
  492782.4 |####################
  493229.0 |
  493675.7 |
  494122.3 |
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 1013541.2-1026423.6 ns)
  1013541.2 |####################
  1014185.3 |
  1014829.4 |
  1015473.6 |
  1016117.7 |
  1016761.8 |
  1017405.9 |
  1018050.0 |
  1018694.1 |
  1019338.3 |
  1019982.4 |####################
  1020626.5 |
  1021270.6 |
  1021914.7 |########################################
  1022558.8 |
  1023203.0 |
  1023847.1 |
  1024491.2 |
  1025135.3 |
  1025779.4 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cseeqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_eqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=100.1% of algo (FFI overhead may distort results)
