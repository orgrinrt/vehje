# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_tight_all dominates: 1222% faster than the next best (carrier_opt_tight_fold)

carrier_opt_tight_all (30.29 us) leads carrier_opt_tight_fold (400.61 us) by 1222%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 96% (significant)

carrier_opt_tight_all is -667.90 us (96%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_cseeqsat is an outlier: 67.7x slower than the field

carrier_opt_tight_cseeqsat (2.05 ms) is 67.7x the fastest (30.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_tight_cseeqsat shows alternating (throttle bounce) (autocorr -0.59)

carrier_opt_tight_cseeqsat's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_dce, carrier_opt_tight_none, carrier_opt_tight_eqsat, carrier_opt_tight_cseeqsat} (1222% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_dce, carrier_opt_tight_none, carrier_opt_tight_eqsat, carrier_opt_tight_cseeqsat} with a 1222% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 67.7x the fastest

Fastest carrier_opt_tight_all (30.29 us) to slowest carrier_opt_tight_cseeqsat (2.05 ms): 67.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_tight_all** at 30292.9 ns median (-95.7% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 67.70x (fastest 30292.9 ns, slowest 2050885.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 33025ns | 32628ns | 32401ns | 32584ns | 33998ns | -95.30% |
| carrier_opt_tight_cse | 654566ns | 653785ns | 653360ns | 653649ns | 656545ns | -6.80% |
| carrier_opt_tight_cseeqsat | 2054842ns | 2054590ns | 2049806ns | 2054185ns | 2058344ns | +192.59% |
| carrier_opt_tight_dce | 694821ns | 694514ns | 693223ns | 694404ns | 696247ns | -1.06% |
| carrier_opt_tight_eqsat | 2036504ns | 2036022ns | 2031268ns | 2034627ns | 2041937ns | +189.98% |
| carrier_opt_tight_fold | 403979ns | 403536ns | 402750ns | 403435ns | 405410ns | -42.48% |
| carrier_opt_tight_none | 702292ns | 701607ns | 699299ns | 701225ns | 705388ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 30635ns | 30065ns | 31521ns | -95.62% | 0.535 |
| carrier_opt_tight_cse | 651028ns | 649475ns | 653122ns | -6.84% | 0.025 |
| carrier_opt_tight_cseeqsat | 2051046ns | 2045732ns | 2054770ns | +193.49% | 0.008 |
| carrier_opt_tight_dce | 691136ns | 689359ns | 692767ns | -1.10% | 0.024 |
| carrier_opt_tight_eqsat | 2032808ns | 2027309ns | 2038586ns | +190.88% | 0.008 |
| carrier_opt_tight_fold | 400911ns | 399815ns | 402220ns | -42.63% | 0.041 |
| carrier_opt_tight_none | 698854ns | 695460ns | 702152ns | base | 0.023 |

## Performance model

- Peak throughput: **0.545 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.541 | 99.2% |
| carrier_opt_tight_cse | 0.025 | 4.6% |
| carrier_opt_tight_cseeqsat | 0.008 | 1.5% |
| carrier_opt_tight_dce | 0.024 | 4.4% |
| carrier_opt_tight_eqsat | 0.008 | 1.5% |
| carrier_opt_tight_fold | 0.041 | 7.5% |
| carrier_opt_tight_none | 0.023 | 4.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 33025ns | 33025ns | -95.30% |
| carrier_opt_tight_cse | 654566ns | 654566ns | -6.80% |
| carrier_opt_tight_cseeqsat | 2054842ns | 2054842ns | +192.59% |
| carrier_opt_tight_dce | 694821ns | 694821ns | -1.06% |
| carrier_opt_tight_eqsat | 2036504ns | 2036504ns | +189.98% |
| carrier_opt_tight_fold | 403979ns | 403979ns | -42.48% |
| carrier_opt_tight_none | 702292ns | 702292ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 698105ns | base | --- | [696305, 702152] | --- | --- | --- | --- |
| carrier_opt_tight_all | 30293ns | -667903.2ns (-95.7%) | [-671968, -664784]ns | [30092, 31521] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_tight_cse | 650168ns | -47676.0ns (-6.8%) | [-51853, -43947]ns | [649795, 653122] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_tight_cseeqsat | 2050885ns | +1352308.3ns (+193.7%) | [+1346438, +1357831]ns | [2047484, 2054770] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_tight_dce | 690891ns | -7353.8ns (-1.1%) | [-11939, -3860]ns | [689750, 692767] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_tight_eqsat | 2032229ns | +1335067.1ns (+191.2%) | [+1327366, +1339430]ns | [2027608, 2038586] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_tight_fold | 400615ns | -297887.9ns (-42.7%) | [-299932, -296009]ns | [399899, 402220] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_cse | carrier_opt_tight_cseeqsat | carrier_opt_tight_dce | carrier_opt_tight_eqsat | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|---|
| 1 | 697793ns | -95.7% | -6.8% | +193.8% | -0.8% | +190.9% | -42.7% |
| 2 | 698417ns | -95.7% | -6.1% | +194.1% | -1.1% | +191.8% | -42.6% |
| 3 | 699897ns | -95.7% | -7.1% | +193.1% | -1.5% | +191.4% | -42.7% |
| 4 | 704406ns | -95.7% | -7.7% | +190.4% | -1.9% | +187.8% | -42.8% |
| 5 | 695460ns | -95.5% | -6.5% | +195.6% | -0.3% | +191.6% | -42.4% |
| 6 | 697150ns | -95.4% | -6.8% | +193.9% | -1.0% | +191.9% | -42.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | 0.362 | moderate+ |
| carrier_opt_tight_cse | -0.232 | moderate- |
| carrier_opt_tight_cseeqsat | -0.592 | HIGH- (thermal bounce) |
| carrier_opt_tight_dce | -0.177 | ok |
| carrier_opt_tight_eqsat | -0.011 | ok |
| carrier_opt_tight_fold | 0.059 | ok |
| carrier_opt_tight_none | -0.152 | ok |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_cse**: won 6/6, lost 0/6
- **carrier_opt_tight_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_dce**: won 6/6, lost 0/6
- **carrier_opt_tight_eqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 92055.3ns | 30635.5ns | 300.5% | HIGH |
| carrier_opt_tight_cse | 653448.8ns | 651028.4ns | 100.4% | HIGH |
| carrier_opt_tight_cseeqsat | 2056316.8ns | 2051046.2ns | 100.3% | HIGH |
| carrier_opt_tight_dce | 693171.4ns | 691136.1ns | 100.3% | HIGH |
| carrier_opt_tight_eqsat | 2037373.1ns | 2032807.9ns | 100.2% | HIGH |
| carrier_opt_tight_fold | 403152.1ns | 400911.0ns | 100.6% | HIGH |
| carrier_opt_tight_none | 701319.8ns | 698853.8ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 30065.0-31521.2 ns)
  30065.0 |########################################
  30137.8 |
  30210.6 |
  30283.4 |########################################
  30356.2 |
  30429.1 |
  30501.9 |
  30574.7 |
  30647.5 |
  30720.3 |
  30793.1 |
  30865.9 |
  30938.8 |
  31011.6 |
  31084.4 |
  31157.2 |
  31230.0 |
  31302.8 |####################
  31375.6 |
  31448.4 |
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 649474.6-653121.7 ns)
  649474.6 |####################
  649657.0 |
  649839.3 |
  650021.7 |########################################
  650204.0 |####################
  650386.4 |####################
  650568.7 |
  650751.1 |
  650933.4 |
  651115.8 |
  651298.1 |
  651480.5 |
  651662.8 |
  651845.2 |
  652027.5 |
  652209.9 |
  652392.2 |
  652574.6 |
  652756.9 |
  652939.3 |
  (0 below, 1 above range)

carrier_opt_tight_cseeqsat (n=6, range 2045732.1-2054770.0 ns)
  2045732.1 |########################################
  2046184.0 |
  2046635.9 |
  2047087.8 |
  2047539.7 |
  2047991.6 |
  2048443.5 |
  2048895.4 |########################################
  2049347.3 |
  2049799.2 |
  2050251.1 |########################################
  2050702.9 |
  2051154.8 |########################################
  2051606.7 |
  2052058.6 |
  2052510.5 |
  2052962.4 |
  2053414.3 |########################################
  2053866.2 |
  2054318.1 |
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 689358.8-692766.7 ns)
  689358.8 |########################################
  689529.2 |
  689699.6 |
  689870.0 |
  690040.4 |########################################
  690210.8 |
  690381.2 |
  690551.6 |########################################
  690722.0 |
  690892.4 |
  691062.8 |########################################
  691233.1 |
  691403.5 |
  691573.9 |
  691744.3 |
  691914.7 |
  692085.1 |
  692255.5 |
  692425.9 |########################################
  692596.3 |
  (0 below, 1 above range)

carrier_opt_tight_eqsat (n=6, range 2027309.2-2038586.4 ns)
  2027309.2 |########################################
  2027873.1 |########################################
  2028436.9 |
  2029000.8 |
  2029564.6 |########################################
  2030128.5 |
  2030692.4 |
  2031256.2 |
  2031820.1 |
  2032384.0 |
  2032947.8 |
  2033511.7 |
  2034075.6 |
  2034639.4 |########################################
  2035203.3 |
  2035767.1 |
  2036331.0 |
  2036894.9 |
  2037458.7 |########################################
  2038022.6 |
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 399815.0-402219.6 ns)
  399815.0 |####################
  399935.2 |####################
  400055.5 |
  400175.7 |
  400295.9 |
  400416.1 |
  400536.4 |########################################
  400656.6 |
  400776.8 |
  400897.0 |
  401017.3 |
  401137.5 |####################
  401257.7 |
  401378.0 |
  401498.2 |
  401618.4 |
  401738.6 |
  401858.9 |
  401979.1 |
  402099.3 |
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 695460.4-702151.6 ns)
  695460.4 |########################################
  695795.0 |
  696129.5 |
  696464.1 |
  696798.7 |
  697133.2 |########################################
  697467.8 |########################################
  697802.3 |
  698136.9 |########################################
  698471.5 |
  698806.0 |
  699140.6 |
  699475.1 |
  699809.7 |########################################
  700144.3 |
  700478.8 |
  700813.4 |
  701148.0 |
  701482.5 |
  701817.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=300.8% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cseeqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_tight_eqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=100.3% of algo (FFI overhead may distort results)
