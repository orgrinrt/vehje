# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_pre_tight_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_pre_tight_switch has the worst median (2.18 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_pre_tight_null at 1.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_pre_tight_null dominates: 25% faster than the next best (carrier_pre_tight_direct)

carrier_pre_tight_null (1.53 us) leads carrier_pre_tight_direct (1.92 us) by 25%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_tight_null beats baseline by 29% (significant)

carrier_pre_tight_null is -637 ns (29%) faster than baseline carrier_pre_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {carrier_pre_tight_null} vs {carrier_pre_tight_direct, carrier_pre_tight_threaded, carrier_pre_tight_fntable, carrier_pre_tight_regcache, carrier_pre_tight_switch} (25% apart)

The field splits into a fast tier {carrier_pre_tight_null} and a slow tier {carrier_pre_tight_direct, carrier_pre_tight_threaded, carrier_pre_tight_fntable, carrier_pre_tight_regcache, carrier_pre_tight_switch} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_pre_tight_regcache's edge over baseline is significant but tiny (-33 ns, 1.51%)

carrier_pre_tight_regcache differs from baseline carrier_pre_tight_switch by -33 ns (1.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_pre_tight_null** at 1532.3 ns median (-29.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.42x (fastest 1532.3 ns, slowest 2177.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 4238ns | 4298ns | 4038ns | 4249ns | 4321ns | -6.17% |
| carrier_pre_tight_fntable | 4379ns | 4449ns | 4018ns | 4413ns | 4508ns | -3.05% |
| carrier_pre_tight_null | 3891ns | 3920ns | 3818ns | 3892ns | 3924ns | -13.86% |
| carrier_pre_tight_regcache | 4519ns | 4519ns | 4501ns | 4516ns | 4532ns | +0.05% |
| carrier_pre_tight_switch | 4516ns | 4519ns | 4425ns | 4507ns | 4576ns | base |
| carrier_pre_tight_threaded | 4372ns | 4367ns | 4293ns | 4353ns | 4439ns | -3.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 1892ns | 1797ns | 1933ns | -12.67% | 0.034 |
| carrier_pre_tight_fntable | 2035ns | 1873ns | 2079ns | -6.11% | 0.031 |
| carrier_pre_tight_null | 1527ns | 1503ns | 1546ns | -29.52% | 0.042 |
| carrier_pre_tight_regcache | 2145ns | 2135ns | 2155ns | -1.00% | 0.030 |
| carrier_pre_tight_switch | 2167ns | 2100ns | 2197ns | base | 0.030 |
| carrier_pre_tight_threaded | 1990ns | 1957ns | 2002ns | -8.14% | 0.032 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_tight_direct | 261834 | 735760 | 0.356 | 0.97× |
| carrier_pre_tight_fntable | 265529 | 1091712 | 0.243 | 0.98× |
| carrier_pre_tight_null | 254872 | 1239718 | 0.206 | 0.94× |
| carrier_pre_tight_regcache | 261907 | 1213787 | 0.216 | 0.97× |
| carrier_pre_tight_switch | 270014 | 877416 | 0.308 | 1.00× |
| carrier_pre_tight_threaded | 258704 | 966464 | 0.268 | 0.96× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.043 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.033 | 78.3% |
| carrier_pre_tight_fntable | 0.031 | 72.6% |
| carrier_pre_tight_null | 0.042 | 98.1% |
| carrier_pre_tight_regcache | 0.030 | 70.1% |
| carrier_pre_tight_switch | 0.029 | 69.0% |
| carrier_pre_tight_threaded | 0.032 | 75.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 4238ns | 4238ns | -6.17% |
| carrier_pre_tight_fntable | 4379ns | 4379ns | -3.05% |
| carrier_pre_tight_null | 3891ns | 3891ns | -13.86% |
| carrier_pre_tight_regcache | 4519ns | 4519ns | +0.05% |
| carrier_pre_tight_switch | 4516ns | 4516ns | base |
| carrier_pre_tight_threaded | 4372ns | 4372ns | -3.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 2177ns | base | --- | [2126, 2197] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 1919ns | -269.6ns (-12.4%) | [-301, -253]ns | [1825, 1933] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_tight_fntable | 2070ns | -113.9ns (-5.2%) | [-183, -100]ns | [1955, 2079] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_tight_null | 1532ns | -637.1ns (-29.3%) | [-678, -604]ns | [1503, 1546] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 2146ns | no significant difference | [-51, +19]ns | [2136, 2155] | no | 0.2188 | 0.2188 | 0 |
| carrier_pre_tight_threaded | 1998ns | -189.4ns (-8.7%) | [-214, -126]ns | [1972, 2002] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 2100ns | -14.4% | -10.8% | -28.4% | +1.9% | -4.7% |
| 2 | 2153ns | -13.9% | -5.4% | -28.4% | -0.1% | -7.1% |
| 3 | 2185ns | -12.1% | -5.1% | -29.1% | -2.3% | -8.7% |
| 4 | 2209ns | -12.4% | -6.3% | -31.0% | -2.4% | -9.3% |
| 5 | 2179ns | -11.4% | -4.4% | -29.3% | -1.2% | -10.2% |
| 6 | 2176ns | -11.8% | -4.8% | -30.9% | -1.8% | -8.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | 0.419 | moderate+ |
| carrier_pre_tight_fntable | 0.145 | ok |
| carrier_pre_tight_null | -0.192 | ok |
| carrier_pre_tight_regcache | -0.455 | moderate- |
| carrier_pre_tight_switch | 0.293 | moderate+ |
| carrier_pre_tight_threaded | -0.040 | ok |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 6/6, lost 0/6
- **carrier_pre_tight_fntable**: won 6/6, lost 0/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 4/6, lost 1/6
- **carrier_pre_tight_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 86934.0ns | 1892.5ns | 4593.6% | HIGH |
| carrier_pre_tight_fntable | 87285.2ns | 2034.6ns | 4289.9% | HIGH |
| carrier_pre_tight_null | 85542.9ns | 1527.3ns | 5601.0% | HIGH |
| carrier_pre_tight_regcache | 87824.8ns | 2145.3ns | 4093.9% | HIGH |
| carrier_pre_tight_switch | 89457.4ns | 2167.0ns | 4128.3% | HIGH |
| carrier_pre_tight_threaded | 86575.3ns | 1990.5ns | 4349.5% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 1796.7-1933.1 ns)
   1796.7 |########################################
   1803.5 |
   1810.3 |
   1817.2 |
   1824.0 |
   1830.8 |
   1837.6 |
   1844.4 |
   1851.3 |########################################
   1858.1 |
   1864.9 |
   1871.7 |
   1878.5 |
   1885.4 |
   1892.2 |
   1899.0 |
   1905.8 |
   1912.6 |########################################
   1919.5 |########################################
   1926.3 |########################################
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 1873.3-2078.8 ns)
   1873.3 |#############
   1883.6 |
   1893.8 |
   1904.1 |
   1914.4 |
   1924.7 |
   1935.0 |
   1945.2 |
   1955.5 |
   1965.8 |
   1976.1 |
   1986.3 |
   1996.6 |
   2006.9 |
   2017.2 |
   2027.4 |#############
   2037.7 |
   2048.0 |
   2058.2 |
   2068.5 |########################################
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 1503.3-1546.2 ns)
   1503.3 |########################################
   1505.4 |
   1507.6 |
   1509.7 |
   1511.9 |
   1514.0 |
   1516.2 |
   1518.3 |
   1520.5 |
   1522.6 |
   1524.8 |####################
   1526.9 |
   1529.1 |
   1531.2 |
   1533.4 |
   1535.5 |
   1537.7 |####################
   1539.8 |
   1542.0 |####################
   1544.1 |
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 2134.6-2154.6 ns)
   2134.6 |########################################
   2135.6 |
   2136.6 |########################################
   2137.6 |
   2138.6 |
   2139.6 |########################################
   2140.6 |
   2141.6 |
   2142.6 |
   2143.6 |
   2144.6 |
   2145.6 |
   2146.6 |
   2147.6 |
   2148.6 |
   2149.6 |
   2150.6 |########################################
   2151.6 |########################################
   2152.6 |
   2153.6 |
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 2100.0-2197.1 ns)
   2100.0 |########################################
   2104.9 |
   2109.7 |
   2114.6 |
   2119.4 |
   2124.3 |
   2129.1 |
   2134.0 |
   2138.8 |
   2143.7 |
   2148.6 |########################################
   2153.4 |
   2158.3 |
   2163.1 |
   2168.0 |
   2172.8 |########################################
   2177.7 |########################################
   2182.5 |########################################
   2187.4 |
   2192.2 |
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 1957.1-2002.3 ns)
   1957.1 |########################################
   1959.4 |
   1961.6 |
   1963.9 |
   1966.1 |
   1968.4 |
   1970.7 |
   1972.9 |
   1975.2 |
   1977.4 |
   1979.7 |
   1982.0 |
   1984.2 |########################################
   1986.5 |
   1988.7 |
   1991.0 |
   1993.3 |
   1995.5 |########################################
   1997.8 |########################################
   2000.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=4527.7% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=4210.1% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=5578.2% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=4094.7% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=4096.9% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=4324.7% of algo (FFI overhead may distort results)
