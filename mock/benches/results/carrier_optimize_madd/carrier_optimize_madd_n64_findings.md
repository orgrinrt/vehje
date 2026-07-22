# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_madd_all dominates: 350% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (431 ns) leads carrier_opt_madd_fold (1.94 us) by 350%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 82% (significant)

carrier_opt_madd_all is -1.90 us (82%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_eqsat is an outlier: 35.3x slower than the field

carrier_opt_madd_eqsat (15.20 us) is 35.3x the fastest (431 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_opt_madd_all, carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_none, carrier_opt_madd_dce} vs {carrier_opt_madd_cseeqsat, carrier_opt_madd_eqsat} (547% apart)

The field splits into a fast tier {carrier_opt_madd_all, carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_none, carrier_opt_madd_dce} and a slow tier {carrier_opt_madd_cseeqsat, carrier_opt_madd_eqsat} with a 547% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 35.3x the fastest

Fastest carrier_opt_madd_all (431 ns) to slowest carrier_opt_madd_eqsat (15.20 us): 35.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_opt_madd_dce's edge over baseline is significant but tiny (8 ns, 0.32%)

carrier_opt_madd_dce differs from baseline carrier_opt_madd_none by 8 ns (0.32%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_madd_all** at 430.6 ns median (-81.5% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 35.29x (fastest 430.6 ns, slowest 15198.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 2966ns | 2968ns | 2879ns | 2944ns | 3044ns | -39.32% |
| carrier_opt_madd_cse | 4546ns | 4627ns | 3948ns | 4583ns | 4791ns | -7.00% |
| carrier_opt_madd_cseeqsat | 17666ns | 17639ns | 17533ns | 17607ns | 17822ns | +261.38% |
| carrier_opt_madd_dce | 4868ns | 4853ns | 4765ns | 4831ns | 4974ns | -0.43% |
| carrier_opt_madd_eqsat | 17827ns | 17851ns | 17538ns | 17802ns | 18010ns | +264.67% |
| carrier_opt_madd_fold | 4542ns | 4527ns | 4441ns | 4502ns | 4651ns | -7.10% |
| carrier_opt_madd_none | 4889ns | 4910ns | 4785ns | 4872ns | 4965ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 430ns | 421ns | 434ns | -81.42% | 0.149 |
| carrier_opt_madd_cse | 2061ns | 1774ns | 2246ns | -10.83% | 0.031 |
| carrier_opt_madd_cseeqsat | 15131ns | 15042ns | 15236ns | +554.57% | 0.004 |
| carrier_opt_madd_dce | 2335ns | 2312ns | 2351ns | +1.01% | 0.027 |
| carrier_opt_madd_eqsat | 15195ns | 14983ns | 15370ns | +557.35% | 0.004 |
| carrier_opt_madd_fold | 1938ns | 1897ns | 1971ns | -16.16% | 0.033 |
| carrier_opt_madd_none | 2312ns | 2210ns | 2360ns | base | 0.028 |

## Performance model

- Peak throughput: **0.152 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.149 | 97.7% |
| carrier_opt_madd_cse | 0.031 | 20.5% |
| carrier_opt_madd_cseeqsat | 0.004 | 2.8% |
| carrier_opt_madd_dce | 0.027 | 18.0% |
| carrier_opt_madd_eqsat | 0.004 | 2.8% |
| carrier_opt_madd_fold | 0.033 | 21.7% |
| carrier_opt_madd_none | 0.028 | 18.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 2966ns | 2966ns | -39.32% |
| carrier_opt_madd_cse | 4546ns | 4546ns | -7.00% |
| carrier_opt_madd_cseeqsat | 17666ns | 17666ns | +261.38% |
| carrier_opt_madd_dce | 4868ns | 4868ns | -0.43% |
| carrier_opt_madd_eqsat | 17827ns | 17827ns | +264.67% |
| carrier_opt_madd_fold | 4542ns | 4542ns | -7.10% |
| carrier_opt_madd_none | 4889ns | 4889ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 2322ns | base | --- | [2253, 2360] | --- | --- | --- | --- |
| carrier_opt_madd_all | 431ns | -1895.2ns (-81.6%) | [-1930, -1821]ns | [424, 434] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_madd_cse | 2049ns | -227.7ns (-9.8%) | [-417, -106]ns | [1889, 2246] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_madd_cseeqsat | 15110ns | +12841.0ns (+553.1%) | [+12734, +12883]ns | [15047, 15236] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_madd_dce | 2334ns | no significant difference | [-30, +92]ns | [2320, 2351] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_madd_eqsat | 15198ns | +12869.4ns (+554.3%) | [+12675, +13107]ns | [15018, 15370] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_madd_fold | 1940ns | -374.2ns (-16.1%) | [-441, -305]ns | [1903, 1971] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_cse | carrier_opt_madd_cseeqsat | carrier_opt_madd_dce | carrier_opt_madd_eqsat | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|---|
| 1 | 2316ns | -81.8% | -23.4% | +556.9% | -0.2% | +562.3% | -16.2% |
| 2 | 2210ns | -80.4% | -7.8% | +582.3% | +6.7% | +597.1% | -13.6% |
| 3 | 2390ns | -82.1% | -7.8% | +538.4% | -2.3% | +529.8% | -18.8% |
| 4 | 2328ns | -81.5% | -1.7% | +550.6% | -0.0% | +550.8% | -16.0% |
| 5 | 2296ns | -81.2% | -12.8% | +555.1% | +1.6% | +552.5% | -13.5% |
| 6 | 2330ns | -81.3% | -11.6% | +546.0% | +0.7% | +554.4% | -18.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | -0.200 | ok |
| carrier_opt_madd_cse | 0.144 | ok |
| carrier_opt_madd_cseeqsat | -0.104 | ok |
| carrier_opt_madd_dce | -0.442 | moderate- |
| carrier_opt_madd_eqsat | 0.037 | ok |
| carrier_opt_madd_fold | -0.234 | moderate- |
| carrier_opt_madd_none | -0.441 | moderate- |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_cse**: won 6/6, lost 0/6
- **carrier_opt_madd_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_dce**: won 2/6, lost 3/6
- **carrier_opt_madd_eqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 84389.4ns | 429.6ns | 19644.5% | HIGH |
| carrier_opt_madd_cse | 86353.5ns | 2061.3ns | 4189.3% | HIGH |
| carrier_opt_madd_cseeqsat | 90955.7ns | 15130.9ns | 601.1% | HIGH |
| carrier_opt_madd_dce | 86572.8ns | 2334.9ns | 3707.7% | HIGH |
| carrier_opt_madd_eqsat | 91499.8ns | 15195.2ns | 602.2% | HIGH |
| carrier_opt_madd_fold | 86166.7ns | 1938.0ns | 4446.0% | HIGH |
| carrier_opt_madd_none | 86412.9ns | 2311.6ns | 3738.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 420.8-433.8 ns)
    420.8 |########################################
    421.4 |
    422.1 |
    422.7 |
    423.4 |
    424.0 |
    424.7 |
    425.3 |
    426.0 |
    426.6 |
    427.3 |########################################
    427.9 |
    428.6 |
    429.2 |########################################
    429.9 |
    430.5 |
    431.2 |########################################
    431.8 |
    432.5 |########################################
    433.1 |
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 1774.2-2246.5 ns)
   1774.2 |########################################
   1797.8 |
   1821.4 |
   1845.0 |
   1868.7 |
   1892.3 |
   1915.9 |
   1939.5 |
   1963.1 |
   1986.7 |########################################
   2010.3 |
   2034.0 |########################################
   2057.6 |########################################
   2081.2 |
   2104.8 |
   2128.4 |
   2152.0 |
   2175.7 |
   2199.3 |########################################
   2222.9 |
  (0 below, 1 above range)

carrier_opt_madd_cseeqsat (n=6, range 15041.7-15235.8 ns)
  15041.7 |########################################
  15051.4 |########################################
  15061.1 |
  15070.8 |########################################
  15080.5 |
  15090.2 |
  15099.9 |
  15109.6 |
  15119.3 |
  15129.0 |
  15138.8 |########################################
  15148.5 |
  15158.2 |
  15167.9 |
  15177.6 |
  15187.3 |
  15197.0 |
  15206.7 |########################################
  15216.4 |
  15226.1 |
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 2312.1-2351.2 ns)
   2312.1 |########################################
   2314.1 |
   2316.0 |
   2318.0 |
   2319.9 |
   2321.9 |
   2323.8 |
   2325.8 |########################################
   2327.8 |
   2329.7 |
   2331.7 |########################################
   2333.6 |########################################
   2335.6 |
   2337.5 |
   2339.5 |
   2341.5 |
   2343.4 |
   2345.4 |########################################
   2347.3 |
   2349.3 |
  (0 below, 1 above range)

carrier_opt_madd_eqsat (n=6, range 14982.9-15369.6 ns)
  14982.9 |########################################
  15002.2 |
  15021.6 |
  15040.9 |########################################
  15060.2 |
  15079.6 |
  15098.9 |
  15118.2 |
  15137.6 |########################################
  15156.9 |
  15176.2 |
  15195.6 |
  15214.9 |
  15234.3 |########################################
  15253.6 |
  15272.9 |
  15292.3 |
  15311.6 |
  15330.9 |########################################
  15350.3 |
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 1896.7-1970.8 ns)
   1896.7 |####################
   1900.4 |
   1904.1 |
   1907.8 |####################
   1911.5 |
   1915.2 |
   1918.9 |
   1922.6 |
   1926.3 |
   1930.0 |
   1933.8 |
   1937.5 |########################################
   1941.2 |
   1944.9 |
   1948.6 |
   1952.3 |####################
   1956.0 |
   1959.7 |
   1963.4 |
   1967.1 |
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 2209.6-2360.0 ns)
   2209.6 |########################################
   2217.1 |
   2224.6 |
   2232.2 |
   2239.7 |
   2247.2 |
   2254.7 |
   2262.2 |
   2269.8 |
   2277.3 |
   2284.8 |
   2292.3 |########################################
   2299.8 |
   2307.4 |
   2314.9 |########################################
   2322.4 |########################################
   2329.9 |########################################
   2337.4 |
   2345.0 |
   2352.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=19622.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=4219.1% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cseeqsat**: bridge=601.3% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=3709.3% of algo (FFI overhead may distort results)
- **carrier_opt_madd_eqsat**: bridge=602.1% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=4422.9% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=3721.7% of algo (FFI overhead may distort results)
