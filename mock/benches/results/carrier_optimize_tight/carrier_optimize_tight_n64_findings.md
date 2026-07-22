# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_tight_all dominates: 315% faster than the next best (carrier_opt_tight_cse)

carrier_opt_tight_all (459 ns) leads carrier_opt_tight_cse (1.90 us) by 315%, a clear separation rather than a photo finish. CV 0.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 76% (significant)

carrier_opt_tight_all is -1.46 us (76%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_eqsat is an outlier: 4.9x slower than the field

carrier_opt_tight_eqsat (2.27 us) is 4.9x the fastest (459 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_tight_fold shows alternating (throttle bounce) (autocorr -0.52)

carrier_opt_tight_fold's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_fold, carrier_opt_tight_dce, carrier_opt_tight_cseeqsat, carrier_opt_tight_eqsat} (315% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_fold, carrier_opt_tight_dce, carrier_opt_tight_cseeqsat, carrier_opt_tight_eqsat} with a 315% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.9x the fastest

Fastest carrier_opt_tight_all (459 ns) to slowest carrier_opt_tight_eqsat (2.27 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_opt_tight_cse's edge over baseline is significant but tiny (-29 ns, 1.51%)

carrier_opt_tight_cse differs from baseline carrier_opt_tight_none by -29 ns (1.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_tight_all** at 458.7 ns median (-76.1% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.94x (fastest 458.7 ns, slowest 2268.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 3004ns | 2989ns | 2934ns | 2974ns | 3082ns | -31.90% |
| carrier_opt_tight_cse | 4341ns | 4396ns | 3860ns | 4382ns | 4519ns | -1.59% |
| carrier_opt_tight_cseeqsat | 4744ns | 4707ns | 4704ns | 4706ns | 4820ns | +7.54% |
| carrier_opt_tight_dce | 4502ns | 4485ns | 4352ns | 4468ns | 4629ns | +2.07% |
| carrier_opt_tight_eqsat | 4787ns | 4767ns | 4751ns | 4766ns | 4838ns | +8.53% |
| carrier_opt_tight_fold | 4499ns | 4448ns | 4409ns | 4436ns | 4638ns | +2.00% |
| carrier_opt_tight_none | 4411ns | 4449ns | 4130ns | 4423ns | 4532ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 459ns | 458ns | 459ns | -76.05% | 0.140 |
| carrier_opt_tight_cse | 1862ns | 1675ns | 1909ns | -2.80% | 0.034 |
| carrier_opt_tight_cseeqsat | 2260ns | 2235ns | 2301ns | +17.97% | 0.028 |
| carrier_opt_tight_dce | 1942ns | 1890ns | 1976ns | +1.40% | 0.033 |
| carrier_opt_tight_eqsat | 2268ns | 2236ns | 2297ns | +18.40% | 0.028 |
| carrier_opt_tight_fold | 1935ns | 1912ns | 1958ns | +1.01% | 0.033 |
| carrier_opt_tight_none | 1916ns | 1805ns | 1967ns | base | 0.033 |

## Performance model

- Peak throughput: **0.140 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.140 | 99.9% |
| carrier_opt_tight_cse | 0.034 | 24.1% |
| carrier_opt_tight_cseeqsat | 0.029 | 20.4% |
| carrier_opt_tight_dce | 0.033 | 23.6% |
| carrier_opt_tight_eqsat | 0.028 | 20.2% |
| carrier_opt_tight_fold | 0.033 | 23.8% |
| carrier_opt_tight_none | 0.033 | 23.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 3004ns | 3004ns | -31.90% |
| carrier_opt_tight_cse | 4341ns | 4341ns | -1.59% |
| carrier_opt_tight_cseeqsat | 4744ns | 4744ns | +7.54% |
| carrier_opt_tight_dce | 4502ns | 4502ns | +2.07% |
| carrier_opt_tight_eqsat | 4787ns | 4787ns | +8.53% |
| carrier_opt_tight_fold | 4499ns | 4499ns | +2.00% |
| carrier_opt_tight_none | 4411ns | 4411ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 1922ns | base | --- | [1857, 1967] | --- | --- | --- | --- |
| carrier_opt_tight_all | 459ns | -1463.8ns (-76.1%) | [-1508, -1399]ns | [458, 459] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_cse | 1902ns | -29.0ns (-1.5%) | [-116, -16]ns | [1775, 1909] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_cseeqsat | 2243ns | +335.2ns (+17.4%) | [+269, +429]ns | [2236, 2301] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_dce | 1943ns | no significant difference | [-42, +91]ns | [1909, 1976] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_tight_eqsat | 2268ns | +358.5ns (+18.7%) | [+290, +409]ns | [2239, 2297] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_fold | 1929ns | no significant difference | [-40, +74]ns | [1918, 1958] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_cse | carrier_opt_tight_cseeqsat | carrier_opt_tight_dce | carrier_opt_tight_eqsat | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|---|
| 1 | 1805ns | -74.6% | -7.2% | +23.9% | +6.7% | +23.8% | +5.9% |
| 2 | 1928ns | -76.2% | -1.3% | +17.2% | +3.2% | +19.2% | +1.9% |
| 3 | 1931ns | -76.2% | -1.2% | +15.9% | +1.6% | +16.1% | -0.3% |
| 4 | 1918ns | -76.1% | -0.4% | +22.2% | +1.6% | +18.1% | +0.5% |
| 5 | 2003ns | -77.1% | -5.1% | +11.5% | -3.3% | +13.4% | -3.6% |
| 6 | 1909ns | -76.0% | -1.7% | +17.8% | -1.0% | +20.3% | +2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | -0.021 | ok |
| carrier_opt_tight_cse | -0.028 | ok |
| carrier_opt_tight_cseeqsat | -0.410 | moderate- |
| carrier_opt_tight_dce | 0.106 | ok |
| carrier_opt_tight_eqsat | -0.448 | moderate- |
| carrier_opt_tight_fold | -0.521 | HIGH- (thermal bounce) |
| carrier_opt_tight_none | -0.075 | ok |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_cse**: won 6/6, lost 0/6
- **carrier_opt_tight_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_dce**: won 2/6, lost 4/6
- **carrier_opt_tight_eqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_fold**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 84579.6ns | 458.7ns | 18438.3% | HIGH |
| carrier_opt_tight_cse | 86062.1ns | 1862.0ns | 4622.0% | HIGH |
| carrier_opt_tight_cseeqsat | 86062.3ns | 2259.8ns | 3808.3% | HIGH |
| carrier_opt_tight_dce | 85963.8ns | 1942.4ns | 4425.8% | HIGH |
| carrier_opt_tight_eqsat | 86131.4ns | 2268.2ns | 3797.4% | HIGH |
| carrier_opt_tight_fold | 85823.6ns | 1935.0ns | 4435.3% | HIGH |
| carrier_opt_tight_none | 85929.3ns | 1915.6ns | 4485.7% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 458.3-458.9 ns)
    458.3 |##########
    458.3 |
    458.4 |
    458.4 |
    458.4 |
    458.5 |
    458.5 |
    458.5 |
    458.6 |
    458.6 |
    458.6 |
    458.7 |
    458.7 |########################################
    458.7 |
    458.8 |
    458.8 |
    458.8 |
    458.9 |
    458.9 |
    458.9 |
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 1675.0-1908.8 ns)
   1675.0 |#############
   1686.7 |
   1698.4 |
   1710.1 |
   1721.8 |
   1733.4 |
   1745.1 |
   1756.8 |
   1768.5 |
   1780.2 |
   1791.9 |
   1803.6 |
   1815.2 |
   1826.9 |
   1838.6 |
   1850.3 |
   1862.0 |
   1873.7 |#############
   1885.4 |
   1897.1 |########################################
  (0 below, 1 above range)

carrier_opt_tight_cseeqsat (n=6, range 2234.6-2300.6 ns)
   2234.6 |########################################
   2237.9 |####################
   2241.2 |
   2244.5 |
   2247.8 |####################
   2251.1 |
   2254.4 |
   2257.7 |####################
   2261.0 |
   2264.3 |
   2267.6 |
   2270.9 |
   2274.2 |
   2277.5 |
   2280.8 |
   2284.1 |
   2287.4 |
   2290.7 |
   2294.0 |
   2297.3 |
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 1890.4-1975.6 ns)
   1890.4 |########################################
   1894.7 |
   1898.9 |
   1903.2 |
   1907.4 |
   1911.7 |
   1916.0 |
   1920.2 |
   1924.5 |########################################
   1928.7 |
   1933.0 |########################################
   1937.3 |
   1941.5 |
   1945.8 |########################################
   1950.0 |
   1954.3 |
   1958.6 |########################################
   1962.8 |
   1967.1 |
   1971.3 |
  (0 below, 1 above range)

carrier_opt_tight_eqsat (n=6, range 2235.8-2297.1 ns)
   2235.8 |########################################
   2238.9 |
   2241.9 |########################################
   2245.0 |
   2248.1 |
   2251.1 |
   2254.2 |
   2257.2 |
   2260.3 |
   2263.4 |########################################
   2266.4 |
   2269.5 |########################################
   2272.6 |
   2275.6 |
   2278.7 |
   2281.7 |
   2284.8 |
   2287.9 |
   2290.9 |
   2294.0 |########################################
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 1911.7-1957.5 ns)
   1911.7 |########################################
   1914.0 |
   1916.3 |
   1918.6 |
   1920.9 |
   1923.2 |########################################
   1925.4 |
   1927.7 |########################################
   1930.0 |########################################
   1932.3 |
   1934.6 |
   1936.9 |
   1939.2 |
   1941.5 |
   1943.8 |
   1946.0 |
   1948.3 |########################################
   1950.6 |
   1952.9 |
   1955.2 |
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 1805.4-1967.2 ns)
   1805.4 |####################
   1813.5 |
   1821.6 |
   1829.7 |
   1837.8 |
   1845.9 |
   1854.0 |
   1862.0 |
   1870.1 |
   1878.2 |
   1886.3 |
   1894.4 |
   1902.5 |####################
   1910.6 |####################
   1918.7 |
   1926.8 |########################################
   1934.9 |
   1943.0 |
   1951.1 |
   1959.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=18414.3% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=4519.2% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cseeqsat**: bridge=3837.7% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=4422.2% of algo (FFI overhead may distort results)
- **carrier_opt_tight_eqsat**: bridge=3793.1% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=4442.8% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=4469.0% of algo (FFI overhead may distort results)
