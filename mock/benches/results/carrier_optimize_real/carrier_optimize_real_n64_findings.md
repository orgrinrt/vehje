# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_real_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_real_none has the worst median (2.08 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_real_all at 1.72 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_real_dce shows alternating (throttle bounce) (autocorr -0.62)

carrier_opt_real_dce's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_opt_real_dce's edge over baseline is significant but tiny (-12 ns, 0.59%)

carrier_opt_real_dce differs from baseline carrier_opt_real_none by -12 ns (0.59%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_real_all** at 1720.2 ns median (-17.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.21x (fastest 1720.2 ns, slowest 2076.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 4035ns | 4031ns | 3787ns | 3964ns | 4267ns | -8.30% |
| carrier_opt_real_canon | 4268ns | 4358ns | 3949ns | 4228ns | 4488ns | -3.01% |
| carrier_opt_real_cse | 4180ns | 4150ns | 3924ns | 4084ns | 4451ns | -5.01% |
| carrier_opt_real_dce | 4365ns | 4313ns | 4160ns | 4267ns | 4614ns | -0.81% |
| carrier_opt_real_fold | 4192ns | 4102ns | 4021ns | 4094ns | 4424ns | -4.74% |
| carrier_opt_real_none | 4401ns | 4385ns | 4170ns | 4340ns | 4607ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 1720ns | 1628ns | 1804ns | -17.44% | 0.037 |
| carrier_opt_real_canon | 1932ns | 1796ns | 2018ns | -7.23% | 0.033 |
| carrier_opt_real_cse | 1902ns | 1795ns | 2022ns | -8.70% | 0.034 |
| carrier_opt_real_dce | 2069ns | 1975ns | 2189ns | -0.68% | 0.031 |
| carrier_opt_real_fold | 1976ns | 1891ns | 2091ns | -5.14% | 0.032 |
| carrier_opt_real_none | 2083ns | 1984ns | 2179ns | base | 0.031 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_real_all | 263528 | 1521042 | 0.173 | 1.00× |
| carrier_opt_real_canon | 262070 | 1448775 | 0.181 | 0.99× |
| carrier_opt_real_cse | 266995 | 1476570 | 0.181 | 1.01× |
| carrier_opt_real_dce | 266016 | 1425477 | 0.187 | 1.01× |
| carrier_opt_real_fold | 274540 | 1478888 | 0.186 | 1.04× |
| carrier_opt_real_none | 263998 | 1418733 | 0.186 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_opt_real_all; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.037 | 94.6% |
| carrier_opt_real_canon | 0.032 | 82.1% |
| carrier_opt_real_cse | 0.034 | 86.2% |
| carrier_opt_real_dce | 0.031 | 79.7% |
| carrier_opt_real_fold | 0.033 | 84.3% |
| carrier_opt_real_none | 0.031 | 78.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 4035ns | 4035ns | -8.30% |
| carrier_opt_real_canon | 4268ns | 4268ns | -3.01% |
| carrier_opt_real_cse | 4180ns | 4180ns | -5.01% |
| carrier_opt_real_dce | 4365ns | 4365ns | -0.81% |
| carrier_opt_real_fold | 4192ns | 4192ns | -4.74% |
| carrier_opt_real_none | 4401ns | 4401ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 2077ns | base | --- | [1994, 2179] | --- | --- | --- | --- |
| carrier_opt_real_all | 1720ns | -368.4ns (-17.7%) | [-446, -276]ns | [1635, 1804] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_real_canon | 1983ns | -166.2ns (-8.0%) | [-216, -69]ns | [1796, 2018] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_real_cse | 1888ns | -186.2ns (-9.0%) | [-270, -87]ns | [1797, 2022] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_real_dce | 2043ns | no significant difference | [-101, +71]ns | [1975, 2189] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_real_fold | 1931ns | -97.3ns (-4.7%) | [-152, -72]ns | [1906, 2091] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_canon | carrier_opt_real_cse | carrier_opt_real_dce | carrier_opt_real_fold |
|---|---|---|---|---|---|---|
| 1 | 2131ns | -14.7% | -5.7% | -15.6% | -7.3% | -9.0% |
| 2 | 2167ns | -22.7% | -6.6% | -6.4% | -0.1% | -4.4% |
| 3 | 1984ns | -17.2% | -0.9% | -1.8% | +6.1% | -3.2% |
| 4 | 2003ns | -11.9% | -10.3% | -10.4% | -1.1% | -5.6% |
| 5 | 2191ns | -18.2% | -8.7% | -8.0% | +1.0% | -3.7% |
| 6 | 2022ns | -19.5% | -11.2% | -9.7% | -2.3% | -4.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | -0.234 | moderate- |
| carrier_opt_real_canon | -0.223 | moderate- |
| carrier_opt_real_cse | -0.559 | HIGH- (thermal bounce) |
| carrier_opt_real_dce | -0.616 | HIGH- (thermal bounce) |
| carrier_opt_real_fold | -0.543 | HIGH- (thermal bounce) |
| carrier_opt_real_none | -0.284 | moderate- |

**Consistency summary:**

- **carrier_opt_real_all**: won 6/6, lost 0/6
- **carrier_opt_real_canon**: won 6/6, lost 0/6
- **carrier_opt_real_cse**: won 6/6, lost 0/6
- **carrier_opt_real_dce**: won 3/6, lost 2/6
- **carrier_opt_real_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 85327.7ns | 1719.9ns | 4961.3% | HIGH |
| carrier_opt_real_canon | 85471.1ns | 1932.5ns | 4422.9% | HIGH |
| carrier_opt_real_cse | 85513.9ns | 1901.9ns | 4496.2% | HIGH |
| carrier_opt_real_dce | 85385.5ns | 2069.0ns | 4126.9% | HIGH |
| carrier_opt_real_fold | 85652.5ns | 1976.1ns | 4334.4% | HIGH |
| carrier_opt_real_none | 85729.5ns | 2083.2ns | 4115.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 1627.5-1804.4 ns)
   1627.5 |########################################
   1636.3 |########################################
   1645.2 |
   1654.0 |
   1662.9 |
   1671.7 |########################################
   1680.6 |
   1689.4 |
   1698.3 |
   1707.1 |
   1716.0 |
   1724.8 |
   1733.6 |
   1742.5 |
   1751.3 |
   1760.2 |########################################
   1769.0 |
   1777.9 |
   1786.7 |########################################
   1795.6 |
  (0 below, 1 above range)

carrier_opt_real_canon (n=6, range 1796.2-2017.7 ns)
   1796.2 |########################################
   1807.3 |
   1818.4 |
   1829.4 |
   1840.5 |
   1851.6 |
   1862.7 |
   1873.7 |
   1884.8 |
   1895.9 |
   1907.0 |
   1918.0 |
   1929.1 |
   1940.2 |
   1951.2 |
   1962.3 |####################
   1973.4 |
   1984.5 |
   1995.5 |####################
   2006.6 |####################
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 1795.0-2021.7 ns)
   1795.0 |########################################
   1806.3 |
   1817.7 |####################
   1829.0 |
   1840.3 |
   1851.7 |
   1863.0 |
   1874.3 |
   1885.7 |
   1897.0 |
   1908.3 |
   1919.7 |
   1931.0 |
   1942.3 |####################
   1953.7 |
   1965.0 |
   1976.3 |
   1987.7 |
   1999.0 |
   2010.3 |####################
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 1974.6-2188.9 ns)
   1974.6 |########################################
   1985.3 |
   1996.0 |
   2006.8 |
   2017.5 |
   2028.2 |
   2038.9 |
   2049.6 |
   2060.3 |
   2071.1 |
   2081.8 |
   2092.5 |
   2103.2 |#############
   2113.9 |
   2124.6 |
   2135.4 |
   2146.1 |
   2156.8 |#############
   2167.5 |
   2178.2 |
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 1890.8-2091.2 ns)
   1890.8 |########################################
   1900.8 |
   1910.8 |########################################
   1920.9 |########################################
   1930.9 |########################################
   1940.9 |
   1950.9 |
   1961.0 |
   1971.0 |
   1981.0 |
   1991.0 |
   2001.0 |
   2011.1 |
   2021.1 |
   2031.1 |
   2041.1 |
   2051.2 |
   2061.2 |
   2071.2 |########################################
   2081.2 |
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 1984.2-2179.1 ns)
   1984.2 |########################################
   1993.9 |########################################
   2003.7 |
   2013.4 |########################################
   2023.2 |
   2032.9 |
   2042.7 |
   2052.4 |
   2062.2 |
   2071.9 |
   2081.7 |
   2091.4 |
   2101.2 |
   2110.9 |
   2120.7 |
   2130.4 |########################################
   2140.2 |
   2149.9 |
   2159.7 |########################################
   2169.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=4966.3% of algo (FFI overhead may distort results)
- **carrier_opt_real_canon**: bridge=4308.7% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=4530.2% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=4182.8% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=4435.6% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: bridge=4127.5% of algo (FFI overhead may distort results)
