# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_madd_all dominates: 1485% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (25.15 us) leads carrier_opt_madd_fold (398.63 us) by 1485%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 96% (significant)

carrier_opt_madd_all is -622.91 us (96%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_eqsat is an outlier: 142.0x slower than the field

carrier_opt_madd_eqsat (3.57 ms) is 142.0x the fastest (25.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_opt_madd_all} vs {carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none, carrier_opt_madd_cseeqsat, carrier_opt_madd_eqsat} (1485% apart)

The field splits into a fast tier {carrier_opt_madd_all} and a slow tier {carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none, carrier_opt_madd_cseeqsat, carrier_opt_madd_eqsat} with a 1485% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 142.0x the fastest

Fastest carrier_opt_madd_all (25.15 us) to slowest carrier_opt_madd_eqsat (3.57 ms): 142.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_madd_all** at 25148.8 ns median (-96.1% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 141.97x (fastest 25148.8 ns, slowest 3570471.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 27590ns | 27554ns | 27437ns | 27517ns | 27776ns | -95.77% |
| carrier_opt_madd_cse | 605006ns | 604992ns | 604531ns | 604874ns | 605441ns | -7.24% |
| carrier_opt_madd_cseeqsat | 3576721ns | 3572709ns | 3567643ns | 3572254ns | 3587962ns | +448.40% |
| carrier_opt_madd_dce | 633283ns | 632532ns | 632459ns | 632513ns | 634849ns | -2.90% |
| carrier_opt_madd_eqsat | 3575141ns | 3574289ns | 3556136ns | 3572745ns | 3588238ns | +448.15% |
| carrier_opt_madd_fold | 402059ns | 401901ns | 401402ns | 401814ns | 402754ns | -38.35% |
| carrier_opt_madd_none | 652215ns | 651817ns | 651563ns | 651746ns | 653246ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 25195ns | 25140ns | 25295ns | -96.12% | 0.650 |
| carrier_opt_madd_cse | 601442ns | 600956ns | 601848ns | -7.29% | 0.027 |
| carrier_opt_madd_cseeqsat | 3572828ns | 3563739ns | 3583946ns | +450.75% | 0.005 |
| carrier_opt_madd_dce | 629635ns | 628690ns | 631189ns | -2.94% | 0.026 |
| carrier_opt_madd_eqsat | 3571312ns | 3551975ns | 3584742ns | +450.51% | 0.005 |
| carrier_opt_madd_fold | 398942ns | 398279ns | 399760ns | -38.50% | 0.041 |
| carrier_opt_madd_none | 648724ns | 647855ns | 650242ns | base | 0.025 |

## Performance model

- Peak throughput: **0.652 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.651 | 100.0% |
| carrier_opt_madd_cse | 0.027 | 4.2% |
| carrier_opt_madd_cseeqsat | 0.005 | 0.7% |
| carrier_opt_madd_dce | 0.026 | 4.0% |
| carrier_opt_madd_eqsat | 0.005 | 0.7% |
| carrier_opt_madd_fold | 0.041 | 6.3% |
| carrier_opt_madd_none | 0.025 | 3.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 27590ns | 27590ns | -95.77% |
| carrier_opt_madd_cse | 605006ns | 605006ns | -7.24% |
| carrier_opt_madd_cseeqsat | 3576721ns | 3576721ns | +448.40% |
| carrier_opt_madd_dce | 633283ns | 633283ns | -2.90% |
| carrier_opt_madd_eqsat | 3575141ns | 3575141ns | +448.15% |
| carrier_opt_madd_fold | 402059ns | 402059ns | -38.35% |
| carrier_opt_madd_none | 652215ns | 652215ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 648072ns | base | --- | [647858, 650242] | --- | --- | --- | --- |
| carrier_opt_madd_all | 25149ns | -622911.4ns (-96.1%) | [-624960, -622716]ns | [25141, 25295] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_cse | 601469ns | -46921.9ns (-7.2%) | [-48395, -46530]ns | [601009, 601848] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_cseeqsat | 3568961ns | +2920151.0ns (+450.6%) | [+2916215, +2935947]ns | [3565578, 3583946] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_dce | 628995ns | -18971.4ns (-2.9%) | [-21411, -16883]ns | [628723, 631189] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_eqsat | 3570471ns | +2922472.4ns (+450.9%) | [+2910791, +2934499]ns | [3558722, 3584742] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_fold | 398630ns | -249368.2ns (-38.5%) | [-251808, -248171]ns | [398435, 399760] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_cse | carrier_opt_madd_cseeqsat | carrier_opt_madd_dce | carrier_opt_madd_eqsat | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|---|
| 1 | 649614ns | -96.1% | -7.3% | +449.4% | -3.2% | +450.5% | -38.6% |
| 2 | 648136ns | -96.1% | -7.2% | +455.0% | -2.5% | +451.3% | -38.5% |
| 3 | 647861ns | -96.1% | -7.2% | +451.2% | -2.9% | +450.7% | -38.5% |
| 4 | 648007ns | -96.1% | -7.3% | +450.8% | -2.7% | +450.2% | -38.2% |
| 5 | 647855ns | -96.1% | -7.2% | +450.1% | -3.0% | +448.3% | -38.4% |
| 6 | 650871ns | -96.1% | -7.6% | +448.1% | -3.4% | +452.1% | -38.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | -0.081 | ok |
| carrier_opt_madd_cse | 0.281 | moderate+ |
| carrier_opt_madd_cseeqsat | -0.064 | ok |
| carrier_opt_madd_dce | -0.418 | moderate- |
| carrier_opt_madd_eqsat | -0.310 | moderate- |
| carrier_opt_madd_fold | -0.106 | ok |
| carrier_opt_madd_none | -0.082 | ok |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_cse**: won 6/6, lost 0/6
- **carrier_opt_madd_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_dce**: won 6/6, lost 0/6
- **carrier_opt_madd_eqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 100836.3ns | 25194.9ns | 400.2% | HIGH |
| carrier_opt_madd_cse | 603562.5ns | 601442.0ns | 100.4% | HIGH |
| carrier_opt_madd_cseeqsat | 3581028.9ns | 3572828.5ns | 100.2% | HIGH |
| carrier_opt_madd_dce | 631387.7ns | 629635.5ns | 100.3% | HIGH |
| carrier_opt_madd_eqsat | 3578294.7ns | 3571311.5ns | 100.2% | HIGH |
| carrier_opt_madd_fold | 400810.8ns | 398941.6ns | 100.5% | HIGH |
| carrier_opt_madd_none | 651169.9ns | 648724.1ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 25140.4-25294.8 ns)
  25140.4 |########################################
  25148.1 |#############
  25155.8 |
  25163.6 |#############
  25171.3 |
  25179.0 |
  25186.7 |
  25194.4 |
  25202.1 |
  25209.9 |
  25217.6 |
  25225.3 |
  25233.0 |
  25240.7 |
  25248.4 |
  25256.2 |
  25263.9 |
  25271.6 |
  25279.3 |
  25287.0 |
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 600956.2-601847.7 ns)
  600956.2 |########################################
  601000.8 |
  601045.3 |########################################
  601089.9 |
  601134.5 |
  601179.1 |
  601223.6 |
  601268.2 |
  601312.8 |########################################
  601357.4 |
  601401.9 |
  601446.5 |
  601491.1 |
  601535.7 |
  601580.2 |########################################
  601624.8 |########################################
  601669.4 |
  601714.0 |
  601758.5 |
  601803.1 |
  (0 below, 1 above range)

carrier_opt_madd_cseeqsat (n=6, range 3563739.2-3583946.0 ns)
  3563739.2 |####################
  3564749.5 |
  3565759.9 |
  3566770.2 |####################
  3567780.6 |
  3568790.9 |########################################
  3569801.2 |
  3570811.6 |####################
  3571821.9 |
  3572832.3 |
  3573842.6 |
  3574852.9 |
  3575863.3 |
  3576873.6 |
  3577884.0 |
  3578894.3 |
  3579904.6 |
  3580915.0 |
  3581925.3 |
  3582935.7 |
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 628690.4-631188.8 ns)
  628690.4 |########################################
  628815.3 |####################
  628940.2 |
  629065.2 |####################
  629190.1 |
  629315.0 |
  629439.9 |
  629564.8 |
  629689.7 |
  629814.7 |
  629939.6 |
  630064.5 |
  630189.4 |
  630314.3 |
  630439.2 |
  630564.2 |####################
  630689.1 |
  630814.0 |
  630938.9 |
  631063.8 |
  (0 below, 1 above range)

carrier_opt_madd_eqsat (n=6, range 3551975.4-3584741.9 ns)
  3551975.4 |########################################
  3553613.7 |
  3555252.0 |
  3556890.4 |
  3558528.7 |
  3560167.0 |
  3561805.3 |
  3563443.7 |
  3565082.0 |########################################
  3566720.3 |########################################
  3568358.6 |
  3569996.9 |
  3571635.3 |########################################
  3573273.6 |
  3574911.9 |########################################
  3576550.2 |
  3578188.6 |
  3579826.9 |
  3581465.2 |
  3583103.5 |
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 398278.8-399759.8 ns)
  398278.8 |####################
  398352.8 |
  398426.9 |
  398500.9 |
  398575.0 |########################################
  398649.0 |####################
  398723.1 |
  398797.1 |####################
  398871.2 |
  398945.2 |
  399019.3 |
  399093.3 |
  399167.4 |
  399241.4 |
  399315.5 |
  399389.5 |
  399463.6 |
  399537.6 |
  399611.7 |
  399685.7 |
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 647855.0-650242.5 ns)
  647855.0 |########################################
  647974.4 |####################
  648093.8 |####################
  648213.1 |
  648332.5 |
  648451.9 |
  648571.2 |
  648690.6 |
  648810.0 |
  648929.4 |
  649048.8 |
  649168.1 |
  649287.5 |
  649406.9 |
  649526.2 |####################
  649645.6 |
  649765.0 |
  649884.4 |
  650003.8 |
  650123.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=400.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cseeqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_madd_eqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=100.4% of algo (FFI overhead may distort results)
