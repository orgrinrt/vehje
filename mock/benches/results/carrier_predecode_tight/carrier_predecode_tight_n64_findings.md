# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_pre_tight_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_pre_tight_switch has the worst median (2.26 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_pre_tight_null at 1.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_pre_tight_null dominates: 25% faster than the next best (carrier_pre_tight_direct)

carrier_pre_tight_null (1.58 us) leads carrier_pre_tight_direct (1.97 us) by 25%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_tight_null beats baseline by 30% (significant)

carrier_pre_tight_null is -669 ns (30%) faster than baseline carrier_pre_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_pre_tight_null** at 1583.1 ns median (-29.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 1.43x (fastest 1583.1 ns, slowest 2256.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 4448ns | 4447ns | 4407ns | 4442ns | 4476ns | -6.21% |
| carrier_pre_tight_fntable | 4520ns | 4616ns | 3990ns | 4612ns | 4648ns | -4.68% |
| carrier_pre_tight_null | 4104ns | 4069ns | 4031ns | 4062ns | 4202ns | -13.47% |
| carrier_pre_tight_regcache | 4699ns | 4690ns | 4633ns | 4677ns | 4765ns | -0.92% |
| carrier_pre_tight_switch | 4742ns | 4754ns | 4686ns | 4742ns | 4770ns | base |
| carrier_pre_tight_threaded | 4582ns | 4596ns | 4505ns | 4580ns | 4624ns | -3.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 1974ns | 1956ns | 1991ns | -12.54% | 0.032 |
| carrier_pre_tight_fntable | 2093ns | 1842ns | 2155ns | -7.28% | 0.031 |
| carrier_pre_tight_null | 1595ns | 1572ns | 1630ns | -29.33% | 0.040 |
| carrier_pre_tight_regcache | 2205ns | 2164ns | 2234ns | -2.33% | 0.029 |
| carrier_pre_tight_switch | 2257ns | 2243ns | 2270ns | base | 0.028 |
| carrier_pre_tight_threaded | 2081ns | 2049ns | 2108ns | -7.82% | 0.031 |

## Performance model

- Peak throughput: **0.041 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.032 | 79.7% |
| carrier_pre_tight_fntable | 0.030 | 73.5% |
| carrier_pre_tight_null | 0.040 | 99.3% |
| carrier_pre_tight_regcache | 0.029 | 71.3% |
| carrier_pre_tight_switch | 0.028 | 69.7% |
| carrier_pre_tight_threaded | 0.031 | 75.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 4448ns | 4448ns | -6.21% |
| carrier_pre_tight_fntable | 4520ns | 4520ns | -4.68% |
| carrier_pre_tight_null | 4104ns | 4104ns | -13.47% |
| carrier_pre_tight_regcache | 4699ns | 4699ns | -0.92% |
| carrier_pre_tight_switch | 4742ns | 4742ns | base |
| carrier_pre_tight_threaded | 4582ns | 4582ns | -3.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 2256ns | base | --- | [2246, 2270] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 1971ns | -288.5ns (-12.8%) | [-296, -264]ns | [1959, 1991] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_tight_fntable | 2139ns | -123.1ns (-5.5%) | [-269, -102]ns | [1984, 2155] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_tight_null | 1583ns | -669.4ns (-29.7%) | [-684, -632]ns | [1572, 1630] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 2204ns | -63.5ns (-2.8%) | [-82, -12]ns | [2176, 2234] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| carrier_pre_tight_threaded | 2074ns | -176.9ns (-7.8%) | [-195, -158]ns | [2060, 2108] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 2248ns | -11.0% | -18.1% | -30.1% | +0.2% | -7.9% |
| 2 | 2261ns | -13.2% | -4.3% | -29.3% | -3.2% | -6.5% |
| 3 | 2270ns | -12.8% | -5.7% | -26.8% | -3.4% | -8.6% |
| 4 | 2252ns | -13.1% | -5.6% | -30.1% | -3.9% | -7.8% |
| 5 | 2243ns | -12.5% | -4.7% | -29.3% | -1.3% | -8.6% |
| 6 | 2270ns | -12.6% | -5.3% | -30.4% | -2.4% | -7.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | -0.246 | moderate- |
| carrier_pre_tight_fntable | -0.113 | ok |
| carrier_pre_tight_null | -0.172 | ok |
| carrier_pre_tight_regcache | -0.092 | ok |
| carrier_pre_tight_switch | -0.240 | moderate- |
| carrier_pre_tight_threaded | -0.367 | moderate- |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 6/6, lost 0/6
- **carrier_pre_tight_fntable**: won 6/6, lost 0/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 5/6, lost 1/6
- **carrier_pre_tight_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 87070.6ns | 1974.1ns | 4410.7% | HIGH |
| carrier_pre_tight_fntable | 86529.4ns | 2092.8ns | 4134.7% | HIGH |
| carrier_pre_tight_null | 86297.6ns | 1595.3ns | 5409.6% | HIGH |
| carrier_pre_tight_regcache | 85914.2ns | 2204.7ns | 3896.8% | HIGH |
| carrier_pre_tight_switch | 89602.1ns | 2257.2ns | 3969.6% | HIGH |
| carrier_pre_tight_threaded | 86409.8ns | 2080.7ns | 4152.9% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 1956.2-1991.5 ns)
   1956.2 |########################################
   1958.0 |
   1959.7 |
   1961.5 |########################################
   1963.2 |########################################
   1965.0 |
   1966.8 |
   1968.5 |
   1970.3 |
   1972.1 |
   1973.8 |
   1975.6 |
   1977.4 |
   1979.1 |########################################
   1980.9 |
   1982.6 |########################################
   1984.4 |
   1986.2 |
   1987.9 |
   1989.7 |
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 1841.7-2155.4 ns)
   1841.7 |#############
   1857.4 |
   1873.1 |
   1888.8 |
   1904.4 |
   1920.1 |
   1935.8 |
   1951.5 |
   1967.2 |
   1982.9 |
   1998.6 |
   2014.2 |
   2029.9 |
   2045.6 |
   2061.3 |
   2077.0 |
   2092.7 |
   2108.3 |
   2124.0 |########################################
   2139.7 |#############
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 1571.7-1630.4 ns)
   1571.7 |########################################
   1574.6 |
   1577.6 |####################
   1580.5 |
   1583.4 |####################
   1586.4 |
   1589.3 |
   1592.2 |
   1595.2 |
   1598.1 |####################
   1601.1 |
   1604.0 |
   1606.9 |
   1609.9 |
   1612.8 |
   1615.7 |
   1618.7 |
   1621.6 |
   1624.5 |
   1627.5 |
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 2164.2-2233.9 ns)
   2164.2 |####################
   2167.7 |
   2171.2 |
   2174.7 |
   2178.1 |
   2181.6 |
   2185.1 |
   2188.6 |####################
   2192.1 |####################
   2195.6 |
   2199.1 |
   2202.6 |
   2206.0 |
   2209.5 |
   2213.0 |########################################
   2216.5 |
   2220.0 |
   2223.5 |
   2227.0 |
   2230.5 |
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 2242.9-2269.8 ns)
   2242.9 |########################################
   2244.2 |
   2245.6 |
   2246.9 |
   2248.3 |########################################
   2249.6 |
   2251.0 |########################################
   2252.3 |
   2253.7 |
   2255.0 |
   2256.4 |
   2257.7 |
   2259.0 |
   2260.4 |########################################
   2261.7 |
   2263.1 |
   2264.4 |
   2265.8 |
   2267.1 |
   2268.5 |########################################
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 2049.2-2107.5 ns)
   2049.2 |####################
   2052.1 |
   2055.0 |
   2057.9 |
   2060.9 |
   2063.8 |
   2066.7 |
   2069.6 |####################
   2072.5 |########################################
   2075.4 |
   2078.3 |
   2081.3 |
   2084.2 |
   2087.1 |
   2090.0 |
   2092.9 |
   2095.8 |
   2098.8 |
   2101.7 |####################
   2104.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=4418.8% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=4069.5% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=5446.3% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=3903.1% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=3978.3% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=4163.4% of algo (FFI overhead may distort results)
