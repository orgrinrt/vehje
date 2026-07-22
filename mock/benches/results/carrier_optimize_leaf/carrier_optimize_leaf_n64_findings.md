# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_leaf_all dominates: 20% faster than the next best (carrier_opt_leaf_cse)

carrier_opt_leaf_all (1.14 us) leads carrier_opt_leaf_cse (1.38 us) by 20%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_leaf_all beats baseline by 43% (significant)

carrier_opt_leaf_all is -880 ns (43%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_cseeqsat shows alternating (throttle bounce) (autocorr -0.58)

carrier_opt_leaf_cseeqsat's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_leaf_all, carrier_opt_leaf_cse} vs {carrier_opt_leaf_eqsat, carrier_opt_leaf_cseeqsat, carrier_opt_leaf_dce, carrier_opt_leaf_none, carrier_opt_leaf_fold} (36% apart)

The field splits into a fast tier {carrier_opt_leaf_all, carrier_opt_leaf_cse} and a slow tier {carrier_opt_leaf_eqsat, carrier_opt_leaf_cseeqsat, carrier_opt_leaf_dce, carrier_opt_leaf_none, carrier_opt_leaf_fold} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_opt_leaf_dce's edge over baseline is significant but tiny (-11 ns, 0.55%)

carrier_opt_leaf_dce differs from baseline carrier_opt_leaf_none by -11 ns (0.55%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 1143.8 ns median (-43.5% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 1143.8 ns, slowest 2186.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 3726ns | 3732ns | 3617ns | 3716ns | 3796ns | -20.07% |
| carrier_opt_leaf_cse | 3871ns | 3961ns | 3402ns | 3926ns | 4023ns | -16.96% |
| carrier_opt_leaf_cseeqsat | 4461ns | 4454ns | 4385ns | 4448ns | 4519ns | -4.32% |
| carrier_opt_leaf_dce | 4590ns | 4621ns | 4458ns | 4583ns | 4668ns | -1.54% |
| carrier_opt_leaf_eqsat | 4507ns | 4486ns | 4425ns | 4479ns | 4591ns | -3.32% |
| carrier_opt_leaf_fold | 4783ns | 4777ns | 4654ns | 4768ns | 4870ns | +2.59% |
| carrier_opt_leaf_none | 4662ns | 4683ns | 4524ns | 4658ns | 4738ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 1147ns | 1136ns | 1159ns | -43.36% | 0.056 |
| carrier_opt_leaf_cse | 1341ns | 1172ns | 1385ns | -33.78% | 0.048 |
| carrier_opt_leaf_cseeqsat | 1891ns | 1858ns | 1920ns | -6.62% | 0.034 |
| carrier_opt_leaf_dce | 2019ns | 2007ns | 2030ns | -0.29% | 0.032 |
| carrier_opt_leaf_eqsat | 1880ns | 1848ns | 1907ns | -7.14% | 0.034 |
| carrier_opt_leaf_fold | 2186ns | 2164ns | 2205ns | +7.95% | 0.029 |
| carrier_opt_leaf_none | 2025ns | 1996ns | 2044ns | base | 0.032 |

## Performance model

- Peak throughput: **0.056 Gops/s** (carrier_opt_leaf_all; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.056 | 99.3% |
| carrier_opt_leaf_cse | 0.047 | 82.6% |
| carrier_opt_leaf_cseeqsat | 0.034 | 60.3% |
| carrier_opt_leaf_dce | 0.032 | 56.3% |
| carrier_opt_leaf_eqsat | 0.034 | 60.6% |
| carrier_opt_leaf_fold | 0.029 | 52.0% |
| carrier_opt_leaf_none | 0.032 | 56.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 3726ns | 3726ns | -20.07% |
| carrier_opt_leaf_cse | 3871ns | 3871ns | -16.96% |
| carrier_opt_leaf_cseeqsat | 4461ns | 4461ns | -4.32% |
| carrier_opt_leaf_dce | 4590ns | 4590ns | -1.54% |
| carrier_opt_leaf_eqsat | 4507ns | 4507ns | -3.32% |
| carrier_opt_leaf_fold | 4783ns | 4783ns | +2.59% |
| carrier_opt_leaf_none | 4662ns | 4662ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 2025ns | base | --- | [2005, 2044] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 1144ns | -879.8ns (-43.5%) | [-900, -854]ns | [1138, 1159] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 1375ns | -650.6ns (-32.1%) | [-775, -626]ns | [1262, 1385] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_cseeqsat | 1885ns | -120.7ns (-6.0%) | [-167, -114]ns | [1867, 1920] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 2019ns | no significant difference | [-28, +21]ns | [2008, 2030] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_leaf_eqsat | 1873ns | -149.2ns (-7.4%) | [-183, -102]ns | [1860, 1907] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_fold | 2186ns | +154.8ns (+7.6%) | [+146, +183]ns | [2167, 2205] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_cse | carrier_opt_leaf_cseeqsat | carrier_opt_leaf_dce | carrier_opt_leaf_eqsat | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|---|
| 1 | 2028ns | -42.2% | -42.2% | -7.5% | -0.9% | -7.7% | +7.0% |
| 2 | 2014ns | -43.4% | -31.1% | -6.3% | -0.2% | -7.1% | +7.4% |
| 3 | 1996ns | -42.7% | -31.4% | -5.7% | +1.7% | -3.3% | +10.4% |
| 4 | 2021ns | -43.8% | -31.7% | -5.6% | +0.4% | -6.9% | +7.7% |
| 5 | 2041ns | -44.0% | -32.3% | -9.0% | -1.7% | -9.5% | +7.6% |
| 6 | 2048ns | -44.0% | -33.9% | -5.6% | -1.0% | -8.4% | +7.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | -0.098 | ok |
| carrier_opt_leaf_cse | -0.099 | ok |
| carrier_opt_leaf_cseeqsat | -0.579 | HIGH- (thermal bounce) |
| carrier_opt_leaf_dce | -0.199 | ok |
| carrier_opt_leaf_eqsat | -0.032 | ok |
| carrier_opt_leaf_fold | -0.046 | ok |
| carrier_opt_leaf_none | 0.394 | moderate+ |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 4/6, lost 2/6
- **carrier_opt_leaf_eqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_fold**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 76874.5ns | 1146.9ns | 6702.9% | HIGH |
| carrier_opt_leaf_cse | 85945.0ns | 1340.8ns | 6409.9% | HIGH |
| carrier_opt_leaf_cseeqsat | 85981.3ns | 1890.7ns | 4547.6% | HIGH |
| carrier_opt_leaf_dce | 86135.8ns | 2018.8ns | 4266.6% | HIGH |
| carrier_opt_leaf_eqsat | 86174.0ns | 1880.1ns | 4583.6% | HIGH |
| carrier_opt_leaf_fold | 86861.5ns | 2185.8ns | 3974.0% | HIGH |
| carrier_opt_leaf_none | 86030.8ns | 2024.7ns | 4249.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 1135.8-1158.9 ns)
   1135.8 |####################
   1137.0 |
   1138.1 |
   1139.3 |####################
   1140.4 |
   1141.6 |
   1142.7 |########################################
   1143.9 |
   1145.1 |####################
   1146.2 |
   1147.4 |
   1148.5 |
   1149.7 |
   1150.8 |
   1152.0 |
   1153.2 |
   1154.3 |
   1155.5 |
   1156.6 |
   1157.8 |
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 1172.1-1385.0 ns)
   1172.1 |####################
   1182.7 |
   1193.4 |
   1204.0 |
   1214.7 |
   1225.3 |
   1236.0 |
   1246.6 |
   1257.2 |
   1267.9 |
   1278.5 |
   1289.2 |
   1299.8 |
   1310.5 |
   1321.1 |
   1331.7 |
   1342.4 |####################
   1353.0 |
   1363.7 |####################
   1374.3 |########################################
  (0 below, 1 above range)

carrier_opt_leaf_cseeqsat (n=6, range 1857.9-1920.2 ns)
   1857.9 |########################################
   1861.0 |
   1864.1 |
   1867.2 |
   1870.4 |
   1873.5 |
   1876.6 |########################################
   1879.7 |########################################
   1882.8 |
   1885.9 |########################################
   1889.0 |
   1892.2 |
   1895.3 |
   1898.4 |
   1901.5 |
   1904.6 |########################################
   1907.7 |
   1910.9 |
   1914.0 |
   1917.1 |
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 2006.7-2029.8 ns)
   2006.7 |########################################
   2007.9 |
   2009.0 |########################################
   2010.2 |########################################
   2011.3 |
   2012.5 |
   2013.6 |
   2014.8 |
   2015.9 |
   2017.1 |
   2018.2 |
   2019.4 |
   2020.6 |
   2021.7 |
   2022.9 |
   2024.0 |
   2025.2 |
   2026.3 |########################################
   2027.5 |
   2028.6 |########################################
  (0 below, 1 above range)

carrier_opt_leaf_eqsat (n=6, range 1848.3-1906.8 ns)
   1848.3 |####################
   1851.2 |
   1854.2 |
   1857.1 |
   1860.0 |
   1862.9 |
   1865.9 |
   1868.8 |####################
   1871.7 |########################################
   1874.6 |
   1877.6 |
   1880.5 |####################
   1883.4 |
   1886.4 |
   1889.3 |
   1892.2 |
   1895.1 |
   1898.1 |
   1901.0 |
   1903.9 |
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 2163.8-2204.6 ns)
   2163.8 |########################################
   2165.8 |
   2167.9 |########################################
   2169.9 |
   2172.0 |
   2174.0 |
   2176.0 |########################################
   2178.1 |
   2180.1 |
   2182.1 |
   2184.2 |
   2186.2 |
   2188.2 |
   2190.3 |
   2192.3 |
   2194.4 |########################################
   2196.4 |
   2198.4 |
   2200.5 |
   2202.5 |########################################
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 1996.2-2044.3 ns)
   1996.2 |########################################
   1998.6 |
   2001.0 |
   2003.4 |
   2005.8 |
   2008.2 |
   2010.6 |
   2013.1 |########################################
   2015.5 |
   2017.9 |
   2020.3 |########################################
   2022.7 |
   2025.1 |
   2027.5 |########################################
   2029.9 |
   2032.3 |
   2034.7 |
   2037.1 |
   2039.5 |########################################
   2041.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=6769.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=6255.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cseeqsat**: bridge=4564.5% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=4266.4% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_eqsat**: bridge=4594.7% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=3971.7% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=4256.2% of algo (FFI overhead may distort results)
