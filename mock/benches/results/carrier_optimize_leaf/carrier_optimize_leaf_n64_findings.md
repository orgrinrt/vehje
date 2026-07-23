# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_leaf_all dominates: 11% faster than the next best (carrier_opt_leaf_canon)

carrier_opt_leaf_all (1.14 us) leads carrier_opt_leaf_canon (1.26 us) by 11%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_leaf_all beats baseline by 42% (significant)

carrier_opt_leaf_all is -838 ns (42%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_all is fastest but the noisiest (CV 5.7%)

carrier_opt_leaf_all wins on median (1.14 us) yet has the highest variance (CV 5.7%), while carrier_opt_leaf_none is the steadiest (CV 3.6%, 1.97 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_opt_leaf_fold shows alternating (throttle bounce) (autocorr -0.61)

carrier_opt_leaf_fold's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_leaf_all, carrier_opt_leaf_canon, carrier_opt_leaf_cse} vs {carrier_opt_leaf_dce, carrier_opt_leaf_none, carrier_opt_leaf_fold} (41% apart)

The field splits into a fast tier {carrier_opt_leaf_all, carrier_opt_leaf_canon, carrier_opt_leaf_cse} and a slow tier {carrier_opt_leaf_dce, carrier_opt_leaf_none, carrier_opt_leaf_fold} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 1135.0 ns median (-42.5% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.85x (fastest 1135.0 ns, slowest 2094.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 3601ns | 3641ns | 3157ns | 3528ns | 3934ns | -17.79% |
| carrier_opt_leaf_canon | 3639ns | 3632ns | 3373ns | 3591ns | 3845ns | -16.93% |
| carrier_opt_leaf_cse | 3722ns | 3784ns | 3340ns | 3701ns | 3945ns | -15.04% |
| carrier_opt_leaf_dce | 4192ns | 4018ns | 3956ns | 4004ns | 4591ns | -4.32% |
| carrier_opt_leaf_fold | 4550ns | 4674ns | 4115ns | 4490ns | 4857ns | +3.86% |
| carrier_opt_leaf_none | 4381ns | 4419ns | 3976ns | 4407ns | 4545ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 1101ns | 1001ns | 1156ns | -43.37% | 0.058 |
| carrier_opt_leaf_canon | 1259ns | 1186ns | 1331ns | -35.22% | 0.051 |
| carrier_opt_leaf_cse | 1261ns | 1175ns | 1342ns | -35.11% | 0.051 |
| carrier_opt_leaf_dce | 1845ns | 1764ns | 1983ns | -5.08% | 0.035 |
| carrier_opt_leaf_fold | 2057ns | 1937ns | 2138ns | +5.84% | 0.031 |
| carrier_opt_leaf_none | 1943ns | 1787ns | 1985ns | base | 0.033 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_leaf_all | 254717 | 1591048 | 0.160 | 0.98× |
| carrier_opt_leaf_canon | 264138 | 1657985 | 0.159 | 1.02× |
| carrier_opt_leaf_cse | 264135 | 1651655 | 0.160 | 1.02× |
| carrier_opt_leaf_dce | 269948 | 1689092 | 0.160 | 1.04× |
| carrier_opt_leaf_fold | 266816 | 1525398 | 0.175 | 1.03× |
| carrier_opt_leaf_none | 259298 | 1615944 | 0.160 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.064 Gops/s** (carrier_opt_leaf_all; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.056 | 88.2% |
| carrier_opt_leaf_canon | 0.051 | 79.6% |
| carrier_opt_leaf_cse | 0.051 | 79.4% |
| carrier_opt_leaf_dce | 0.036 | 56.1% |
| carrier_opt_leaf_fold | 0.031 | 47.8% |
| carrier_opt_leaf_none | 0.032 | 50.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 3601ns | 3601ns | -17.79% |
| carrier_opt_leaf_canon | 3639ns | 3639ns | -16.93% |
| carrier_opt_leaf_cse | 3722ns | 3722ns | -15.04% |
| carrier_opt_leaf_dce | 4192ns | 4192ns | -4.32% |
| carrier_opt_leaf_fold | 4550ns | 4550ns | +3.86% |
| carrier_opt_leaf_none | 4381ns | 4381ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 1974ns | base | --- | [1870, 1985] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 1135ns | -838.2ns (-42.5%) | [-895, -795]ns | [1010, 1156] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_leaf_canon | 1258ns | -668.1ns (-33.8%) | [-766, -619]ns | [1187, 1331] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 1261ns | -678.4ns (-34.4%) | [-762, -607]ns | [1179, 1342] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 1782ns | no significant difference | [-201, +2]ns | [1768, 1983] | no | 0.2188 | 0.2188 | 0 |
| carrier_opt_leaf_fold | 2095ns | +137.3ns (+7.0%) | [+43, +160]ns | [1937, 2138] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_canon | carrier_opt_leaf_cse | carrier_opt_leaf_dce | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|
| 1 | 1972ns | -40.8% | -34.2% | -40.0% | -10.5% | +8.5% |
| 2 | 1953ns | -47.8% | -37.6% | -30.8% | -9.2% | -0.8% |
| 3 | 1985ns | -42.5% | -40.1% | -35.5% | -9.8% | +6.3% |
| 4 | 1977ns | -42.1% | -33.5% | -37.2% | +0.5% | +5.2% |
| 5 | 1787ns | -44.0% | -33.6% | -34.3% | -0.9% | +8.4% |
| 6 | 1985ns | -43.1% | -32.1% | -32.8% | -0.2% | +7.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | -0.558 | HIGH- (thermal bounce) |
| carrier_opt_leaf_canon | -0.541 | HIGH- (thermal bounce) |
| carrier_opt_leaf_cse | -0.371 | moderate- |
| carrier_opt_leaf_dce | -0.313 | moderate- |
| carrier_opt_leaf_fold | -0.605 | HIGH- (thermal bounce) |
| carrier_opt_leaf_none | -0.321 | moderate- |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_canon**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 5/6, lost 1/6
- **carrier_opt_leaf_fold**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 84531.9ns | 1100.6ns | 7680.9% | HIGH |
| carrier_opt_leaf_canon | 85438.4ns | 1258.8ns | 6787.3% | HIGH |
| carrier_opt_leaf_cse | 85429.0ns | 1260.9ns | 6775.2% | HIGH |
| carrier_opt_leaf_dce | 85509.9ns | 1844.6ns | 4635.7% | HIGH |
| carrier_opt_leaf_fold | 86507.3ns | 2056.7ns | 4206.2% | HIGH |
| carrier_opt_leaf_none | 86108.0ns | 1943.3ns | 4431.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 1000.8-1156.5 ns)
   1000.8 |########################################
   1008.6 |
   1016.4 |########################################
   1024.1 |
   1031.9 |
   1039.7 |
   1047.5 |
   1055.3 |
   1063.1 |
   1070.8 |
   1078.6 |
   1086.4 |
   1094.2 |
   1102.0 |
   1109.8 |
   1117.5 |
   1125.3 |########################################
   1133.1 |########################################
   1140.9 |########################################
   1148.7 |
  (0 below, 1 above range)

carrier_opt_leaf_canon (n=6, range 1185.8-1331.5 ns)
   1185.8 |########################################
   1193.1 |
   1200.4 |
   1207.6 |
   1214.9 |####################
   1222.2 |
   1229.5 |
   1236.8 |
   1244.1 |
   1251.3 |
   1258.6 |
   1265.9 |
   1273.2 |
   1280.5 |
   1287.8 |
   1295.0 |####################
   1302.3 |
   1309.6 |####################
   1316.9 |
   1324.2 |
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 1175.0-1342.5 ns)
   1175.0 |########################################
   1183.4 |########################################
   1191.7 |
   1200.1 |
   1208.5 |
   1216.9 |
   1225.2 |
   1233.6 |########################################
   1242.0 |
   1250.4 |
   1258.7 |
   1267.1 |
   1275.5 |########################################
   1283.8 |
   1292.2 |
   1300.6 |
   1309.0 |
   1317.3 |
   1325.7 |########################################
   1334.1 |
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 1764.2-1983.3 ns)
   1764.2 |########################################
   1775.2 |
   1786.1 |#############
   1797.1 |
   1808.0 |
   1819.0 |
   1829.9 |
   1840.9 |
   1851.8 |
   1862.8 |
   1873.8 |
   1884.7 |
   1895.7 |
   1906.6 |
   1917.6 |
   1928.5 |
   1939.5 |
   1950.4 |
   1961.4 |
   1972.3 |#############
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 1936.7-2138.1 ns)
   1936.7 |########################################
   1946.8 |
   1956.8 |
   1966.9 |
   1977.0 |
   1987.1 |
   1997.1 |
   2007.2 |
   2017.3 |
   2027.4 |
   2037.4 |
   2047.5 |
   2057.6 |
   2067.6 |
   2077.7 |####################
   2087.8 |
   2097.9 |
   2107.9 |####################
   2118.0 |
   2128.1 |####################
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 1787.1-1985.2 ns)
   1787.1 |####################
   1797.0 |
   1806.9 |
   1816.8 |
   1826.7 |
   1836.6 |
   1846.5 |
   1856.4 |
   1866.3 |
   1876.2 |
   1886.2 |
   1896.1 |
   1906.0 |
   1915.9 |
   1925.8 |
   1935.7 |
   1945.6 |####################
   1955.5 |
   1965.4 |####################
   1975.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=7427.6% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_canon**: bridge=6784.0% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=6780.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=4799.1% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=4138.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=4346.2% of algo (FFI overhead may distort results)
