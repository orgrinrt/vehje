# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_leaf_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_leaf_none has the worst median (46.54 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_leaf_all at 11.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_leaf_all dominates: 22% faster than the next best (carrier_opt_leaf_cse)

carrier_opt_leaf_all (11.68 us) leads carrier_opt_leaf_cse (14.20 us) by 22%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_leaf_all beats baseline by 75% (significant)

carrier_opt_leaf_all is -34.86 us (75%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_none is an outlier: 4.0x slower than the field

carrier_opt_leaf_none (46.54 us) is 4.0x the fastest (11.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_leaf_none shows alternating (throttle bounce) (autocorr -0.60)

carrier_opt_leaf_none's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_leaf_all, carrier_opt_leaf_cse, carrier_opt_leaf_cseeqsat, carrier_opt_leaf_eqsat} vs {carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} (53% apart)

The field splits into a fast tier {carrier_opt_leaf_all, carrier_opt_leaf_cse, carrier_opt_leaf_cseeqsat, carrier_opt_leaf_eqsat} and a slow tier {carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} with a 53% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest carrier_opt_leaf_all (11.68 us) to slowest carrier_opt_leaf_none (46.54 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 11675.0 ns median (-74.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 3.99x (fastest 11675.0 ns, slowest 46535.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 14228ns | 14226ns | 14143ns | 14209ns | 14297ns | -71.29% |
| carrier_opt_leaf_cse | 16615ns | 16753ns | 15322ns | 16696ns | 17139ns | -66.47% |
| carrier_opt_leaf_cseeqsat | 19979ns | 19851ns | 17060ns | 19778ns | 21741ns | -59.69% |
| carrier_opt_leaf_dce | 48590ns | 47429ns | 46434ns | 47144ns | 51836ns | -1.95% |
| carrier_opt_leaf_eqsat | 24798ns | 24671ns | 22772ns | 24606ns | 26097ns | -49.96% |
| carrier_opt_leaf_fold | 36302ns | 36473ns | 35176ns | 36334ns | 36818ns | -26.75% |
| carrier_opt_leaf_none | 49558ns | 49111ns | 46946ns | 48539ns | 52393ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 11658ns | 11570ns | 11715ns | -75.21% | 0.088 |
| carrier_opt_leaf_cse | 14084ns | 13031ns | 14600ns | -70.05% | 0.073 |
| carrier_opt_leaf_cseeqsat | 17410ns | 14468ns | 19160ns | -62.98% | 0.059 |
| carrier_opt_leaf_dce | 45996ns | 43968ns | 49205ns | -2.18% | 0.022 |
| carrier_opt_leaf_eqsat | 22288ns | 20402ns | 23527ns | -52.60% | 0.046 |
| carrier_opt_leaf_fold | 33711ns | 32616ns | 34195ns | -28.31% | 0.030 |
| carrier_opt_leaf_none | 47022ns | 44575ns | 49837ns | base | 0.022 |

## Performance model

- Peak throughput: **0.089 Gops/s** (carrier_opt_leaf_all; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.088 | 99.1% |
| carrier_opt_leaf_cse | 0.072 | 81.5% |
| carrier_opt_leaf_cseeqsat | 0.059 | 67.0% |
| carrier_opt_leaf_dce | 0.023 | 25.8% |
| carrier_opt_leaf_eqsat | 0.046 | 52.3% |
| carrier_opt_leaf_fold | 0.030 | 34.1% |
| carrier_opt_leaf_none | 0.022 | 24.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 14228ns | 14228ns | -71.29% |
| carrier_opt_leaf_cse | 16615ns | 16615ns | -66.47% |
| carrier_opt_leaf_cseeqsat | 19979ns | 19979ns | -59.69% |
| carrier_opt_leaf_dce | 48590ns | 48590ns | -1.95% |
| carrier_opt_leaf_eqsat | 24798ns | 24798ns | -49.96% |
| carrier_opt_leaf_fold | 36302ns | 36302ns | -26.75% |
| carrier_opt_leaf_none | 49558ns | 49558ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 46535ns | base | --- | [44695, 49837] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 11675ns | -34860.4ns (-74.9%) | [-38121, -33111]ns | [11584, 11715] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 14196ns | -32138.5ns (-69.1%) | [-35438, -31238]ns | [13456, 14600] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_cseeqsat | 17266ns | -28292.5ns (-60.8%) | [-33203, -27342]ns | [15803, 19160] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 44803ns | no significant difference | [-5652, +3496]ns | [43981, 49205] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_leaf_eqsat | 22141ns | -24451.7ns (-52.5%) | [-27758, -21992]ns | [21196, 23527] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_fold | 33898ns | -12533.9ns (-26.9%) | [-16700, -10701]ns | [33039, 34195] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_cse | carrier_opt_leaf_cseeqsat | carrier_opt_leaf_dce | carrier_opt_leaf_eqsat | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|---|
| 1 | 44575ns | -74.0% | -70.8% | -61.4% | -1.3% | -54.2% | -24.4% |
| 2 | 52631ns | -77.8% | -73.2% | -64.3% | -16.5% | -58.2% | -38.0% |
| 3 | 44814ns | -74.1% | -69.0% | -61.8% | +1.3% | -48.3% | -23.5% |
| 4 | 46847ns | -75.0% | -69.1% | -58.3% | -5.6% | -52.8% | -28.6% |
| 5 | 46224ns | -74.8% | -69.1% | -62.5% | +13.9% | -48.4% | -26.2% |
| 6 | 47042ns | -75.1% | -68.7% | -69.2% | -2.7% | -52.9% | -27.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | -0.523 | HIGH- (thermal bounce) |
| carrier_opt_leaf_cse | 0.057 | ok |
| carrier_opt_leaf_cseeqsat | -0.075 | ok |
| carrier_opt_leaf_dce | -0.125 | ok |
| carrier_opt_leaf_eqsat | -0.046 | ok |
| carrier_opt_leaf_fold | -0.377 | moderate- |
| carrier_opt_leaf_none | -0.595 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 4/6, lost 2/6
- **carrier_opt_leaf_eqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 93464.0ns | 11657.9ns | 801.7% | HIGH |
| carrier_opt_leaf_cse | 91853.5ns | 14084.0ns | 652.2% | HIGH |
| carrier_opt_leaf_cseeqsat | 92390.2ns | 17409.8ns | 530.7% | HIGH |
| carrier_opt_leaf_dce | 100664.6ns | 45996.4ns | 218.9% | HIGH |
| carrier_opt_leaf_eqsat | 93344.0ns | 22288.2ns | 418.8% | HIGH |
| carrier_opt_leaf_fold | 101376.7ns | 33710.5ns | 300.7% | HIGH |
| carrier_opt_leaf_none | 97323.7ns | 47022.2ns | 207.0% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 11570.0-11715.2 ns)
  11570.0 |########################################
  11577.3 |
  11584.5 |
  11591.8 |########################################
  11599.0 |
  11606.3 |
  11613.6 |
  11620.8 |
  11628.1 |
  11635.4 |
  11642.6 |
  11649.9 |########################################
  11657.1 |
  11664.4 |
  11671.7 |
  11678.9 |
  11686.2 |
  11693.5 |########################################
  11700.7 |########################################
  11708.0 |
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 13030.8-14599.6 ns)
  13030.8 |########################################
  13109.2 |
  13187.7 |
  13266.1 |
  13344.6 |
  13423.0 |
  13501.4 |
  13579.9 |
  13658.3 |
  13736.8 |
  13815.2 |########################################
  13893.6 |
  13972.1 |
  14050.5 |########################################
  14129.0 |
  14207.4 |
  14285.8 |########################################
  14364.3 |
  14442.7 |########################################
  14521.2 |
  (0 below, 1 above range)

carrier_opt_leaf_cseeqsat (n=6, range 14467.9-19160.0 ns)
  14467.9 |####################
  14702.5 |
  14937.1 |
  15171.7 |
  15406.3 |
  15640.9 |
  15875.5 |
  16110.1 |
  16344.7 |
  16579.3 |
  16814.0 |
  17048.6 |########################################
  17283.2 |####################
  17517.8 |
  17752.4 |
  17987.0 |
  18221.6 |
  18456.2 |
  18690.8 |####################
  18925.4 |
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 43968.3-49204.8 ns)
  43968.3 |########################################
  44230.1 |
  44491.9 |
  44753.8 |
  45015.6 |
  45277.4 |#############
  45539.2 |#############
  45801.1 |
  46062.9 |
  46324.7 |
  46586.5 |
  46848.3 |
  47110.2 |
  47372.0 |
  47633.8 |
  47895.6 |
  48157.5 |
  48419.3 |
  48681.1 |
  48942.9 |
  (0 below, 1 above range)

carrier_opt_leaf_eqsat (n=6, range 20402.5-23526.7 ns)
  20402.5 |####################
  20558.7 |
  20714.9 |
  20871.1 |
  21027.3 |
  21183.5 |
  21339.7 |
  21496.0 |
  21652.2 |
  21808.4 |
  21964.6 |########################################
  22120.8 |####################
  22277.0 |
  22433.2 |
  22589.4 |
  22745.6 |
  22901.8 |
  23058.0 |####################
  23214.2 |
  23370.4 |
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 32615.8-34195.4 ns)
  32615.8 |####################
  32694.8 |
  32773.8 |
  32852.7 |
  32931.7 |
  33010.7 |
  33089.7 |
  33168.7 |
  33247.6 |
  33326.6 |
  33405.6 |####################
  33484.6 |
  33563.6 |
  33642.5 |####################
  33721.5 |
  33800.5 |
  33879.5 |
  33958.5 |
  34037.4 |########################################
  34116.4 |
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 44575.4-49836.7 ns)
  44575.4 |########################################
  44838.5 |
  45101.5 |
  45364.6 |
  45627.7 |
  45890.7 |
  46153.8 |####################
  46416.8 |
  46679.9 |####################
  46943.0 |####################
  47206.0 |
  47469.1 |
  47732.2 |
  47995.2 |
  48258.3 |
  48521.3 |
  48784.4 |
  49047.5 |
  49310.5 |
  49573.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=801.7% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=641.1% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cseeqsat**: bridge=538.7% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=226.5% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_eqsat**: bridge=419.7% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=300.7% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=202.3% of algo (FFI overhead may distort results)
