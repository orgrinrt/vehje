# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_madd_all dominates: 1534% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (1.60 us) leads carrier_opt_madd_fold (26.10 us) by 1534%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 96% (significant)

carrier_opt_madd_all is -39.33 us (96%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_cseeqsat is an outlier: 142.9x slower than the field

carrier_opt_madd_cseeqsat (228.25 us) is 142.9x the fastest (1.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_opt_madd_all} vs {carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none, carrier_opt_madd_eqsat, carrier_opt_madd_cseeqsat} (1534% apart)

The field splits into a fast tier {carrier_opt_madd_all} and a slow tier {carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none, carrier_opt_madd_eqsat, carrier_opt_madd_cseeqsat} with a 1534% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 142.9x the fastest

Fastest carrier_opt_madd_all (1.60 us) to slowest carrier_opt_madd_cseeqsat (228.25 us): 142.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_madd_all** at 1597.5 ns median (-96.1% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 142.88x (fastest 1597.5 ns, slowest 228248.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 3970ns | 3975ns | 3829ns | 3959ns | 4058ns | -90.79% |
| carrier_opt_madd_cse | 40832ns | 40902ns | 38886ns | 40620ns | 42124ns | -5.26% |
| carrier_opt_madd_cseeqsat | 232007ns | 230762ns | 229521ns | 230387ns | 235680ns | +438.32% |
| carrier_opt_madd_dce | 42342ns | 42457ns | 41710ns | 42258ns | 42785ns | -1.75% |
| carrier_opt_madd_eqsat | 230540ns | 230581ns | 228105ns | 230364ns | 232022ns | +434.91% |
| carrier_opt_madd_fold | 28707ns | 28568ns | 28260ns | 28482ns | 29269ns | -33.39% |
| carrier_opt_madd_none | 43099ns | 43338ns | 41102ns | 42826ns | 44505ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 1602ns | 1583ns | 1625ns | -96.06% | 0.639 |
| carrier_opt_madd_cse | 38419ns | 36411ns | 39710ns | -5.44% | 0.027 |
| carrier_opt_madd_cseeqsat | 229426ns | 226834ns | 232909ns | +464.71% | 0.004 |
| carrier_opt_madd_dce | 39891ns | 39313ns | 40290ns | -1.81% | 0.026 |
| carrier_opt_madd_eqsat | 227900ns | 225736ns | 229468ns | +460.95% | 0.004 |
| carrier_opt_madd_fold | 26260ns | 25846ns | 26824ns | -35.36% | 0.039 |
| carrier_opt_madd_none | 40627ns | 38710ns | 41867ns | base | 0.025 |

## Performance model

- Peak throughput: **0.647 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.641 | 99.1% |
| carrier_opt_madd_cse | 0.027 | 4.1% |
| carrier_opt_madd_cseeqsat | 0.004 | 0.7% |
| carrier_opt_madd_dce | 0.026 | 4.0% |
| carrier_opt_madd_eqsat | 0.004 | 0.7% |
| carrier_opt_madd_fold | 0.039 | 6.1% |
| carrier_opt_madd_none | 0.025 | 3.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 3970ns | 3970ns | -90.79% |
| carrier_opt_madd_cse | 40832ns | 40832ns | -5.26% |
| carrier_opt_madd_cseeqsat | 232007ns | 232007ns | +438.32% |
| carrier_opt_madd_dce | 42342ns | 42342ns | -1.75% |
| carrier_opt_madd_eqsat | 230540ns | 230540ns | +434.91% |
| carrier_opt_madd_fold | 28707ns | 28707ns | -33.39% |
| carrier_opt_madd_none | 43099ns | 43099ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 40947ns | base | --- | [39069, 41867] | --- | --- | --- | --- |
| carrier_opt_madd_all | 1598ns | -39325.6ns (-96.0%) | [-40266, -37484]ns | [1584, 1625] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_madd_cse | 38545ns | -2295.7ns (-5.6%) | [-4301, -29]ns | [37001, 39710] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| carrier_opt_madd_cseeqsat | 228248ns | +186963.2ns (+456.6%) | [+185802, +193632]ns | [227122, 232909] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_madd_dce | 39969ns | no significant difference | [-1700, +669]ns | [39415, 40290] | no | 0.2188 | 0.2188 | 0 |
| carrier_opt_madd_eqsat | 227831ns | +187269.2ns (+457.3%) | [+184981, +189566]ns | [226399, 229468] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_madd_fold | 26099ns | -14521.4ns (-35.5%) | [-16009, -12571]ns | [25857, 26824] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_cse | carrier_opt_madd_cseeqsat | carrier_opt_madd_dce | carrier_opt_madd_eqsat | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|---|
| 1 | 41276ns | -96.0% | -11.8% | +449.6% | -3.4% | +454.2% | -35.3% |
| 2 | 38710ns | -95.9% | +2.4% | +513.2% | +3.8% | +486.6% | -32.7% |
| 3 | 42404ns | -96.3% | -6.1% | +438.7% | -4.7% | +432.3% | -39.0% |
| 4 | 41330ns | -96.1% | -9.0% | +452.8% | -3.0% | +451.6% | -37.4% |
| 5 | 40618ns | -96.0% | -4.9% | +459.9% | -2.7% | +460.6% | -35.6% |
| 6 | 39427ns | -96.0% | -2.5% | +478.4% | -0.3% | +483.9% | -31.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | -0.240 | moderate- |
| carrier_opt_madd_cse | -0.255 | moderate- |
| carrier_opt_madd_cseeqsat | -0.292 | moderate- |
| carrier_opt_madd_dce | 0.442 | moderate+ |
| carrier_opt_madd_eqsat | 0.045 | ok |
| carrier_opt_madd_fold | 0.115 | ok |
| carrier_opt_madd_none | -0.370 | moderate- |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_cse**: won 5/6, lost 1/6
- **carrier_opt_madd_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_dce**: won 5/6, lost 1/6
- **carrier_opt_madd_eqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 85686.8ns | 1602.1ns | 5348.5% | HIGH |
| carrier_opt_madd_cse | 115381.7ns | 38418.8ns | 300.3% | HIGH |
| carrier_opt_madd_cseeqsat | 230272.7ns | 229426.4ns | 100.4% | HIGH |
| carrier_opt_madd_dce | 119541.5ns | 39891.2ns | 299.7% | HIGH |
| carrier_opt_madd_eqsat | 229219.5ns | 227899.5ns | 100.6% | HIGH |
| carrier_opt_madd_fold | 105117.4ns | 26260.2ns | 400.3% | HIGH |
| carrier_opt_madd_none | 109821.4ns | 40627.4ns | 270.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 1583.3-1625.0 ns)
   1583.3 |########################################
   1585.4 |####################
   1587.5 |
   1589.6 |
   1591.6 |
   1593.7 |
   1595.8 |
   1597.9 |
   1600.0 |
   1602.1 |
   1604.2 |
   1606.2 |
   1608.3 |####################
   1610.4 |
   1612.5 |
   1614.6 |
   1616.7 |####################
   1618.7 |
   1620.8 |
   1622.9 |
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 36411.2-39710.2 ns)
  36411.2 |########################################
  36576.1 |
  36741.1 |
  36906.0 |
  37071.0 |
  37235.9 |
  37400.9 |
  37565.8 |########################################
  37730.8 |
  37895.8 |
  38060.7 |
  38225.6 |
  38390.6 |########################################
  38555.5 |########################################
  38720.5 |
  38885.4 |
  39050.4 |
  39215.3 |
  39380.3 |
  39545.2 |########################################
  (0 below, 1 above range)

carrier_opt_madd_cseeqsat (n=6, range 226833.8-232909.2 ns)
  226833.8 |####################
  227137.6 |####################
  227441.3 |
  227745.1 |####################
  228048.9 |
  228352.6 |########################################
  228656.4 |
  228960.2 |
  229264.0 |
  229567.7 |
  229871.5 |
  230175.3 |
  230479.0 |
  230782.8 |
  231086.6 |
  231390.4 |
  231694.1 |
  231997.9 |
  232301.7 |
  232605.4 |
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 39313.3-40289.8 ns)
  39313.3 |########################################
  39362.1 |
  39411.0 |
  39459.8 |
  39508.6 |########################################
  39557.4 |
  39606.2 |
  39655.1 |
  39703.9 |
  39752.7 |
  39801.6 |
  39850.4 |########################################
  39899.2 |
  39948.0 |
  39996.9 |
  40045.7 |########################################
  40094.5 |
  40143.3 |########################################
  40192.2 |
  40241.0 |
  (0 below, 1 above range)

carrier_opt_madd_eqsat (n=6, range 225735.8-229467.9 ns)
  225735.8 |########################################
  225922.4 |
  226109.0 |
  226295.6 |
  226482.2 |
  226668.8 |
  226855.4 |
  227042.0 |########################################
  227228.6 |
  227415.2 |
  227601.8 |########################################
  227788.5 |########################################
  227975.1 |
  228161.7 |
  228348.3 |
  228534.9 |
  228721.5 |########################################
  228908.1 |
  229094.7 |
  229281.3 |
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 25846.2-26824.4 ns)
  25846.2 |########################################
  25895.1 |
  25944.0 |
  25992.9 |
  26041.8 |####################
  26090.8 |####################
  26139.7 |
  26188.6 |
  26237.5 |
  26286.4 |
  26335.3 |
  26384.2 |
  26433.1 |
  26482.0 |
  26530.9 |
  26579.9 |
  26628.8 |
  26677.7 |####################
  26726.6 |
  26775.5 |
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 38710.4-41866.7 ns)
  38710.4 |####################
  38868.2 |
  39026.0 |
  39183.8 |
  39341.7 |####################
  39499.5 |
  39657.3 |
  39815.1 |
  39972.9 |
  40130.7 |
  40288.6 |
  40446.4 |
  40604.2 |####################
  40762.0 |
  40919.8 |
  41077.6 |
  41235.4 |########################################
  41393.3 |
  41551.1 |
  41708.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=5379.1% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=300.5% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cseeqsat**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=299.7% of algo (FFI overhead may distort results)
- **carrier_opt_madd_eqsat**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=400.1% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=271.1% of algo (FFI overhead may distort results)
