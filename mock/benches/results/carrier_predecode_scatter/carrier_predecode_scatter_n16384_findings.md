# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null dominates: 257% faster than the next best (carrier_pre_scatter_direct)

carrier_pre_scatter_null (483.69 us) leads carrier_pre_scatter_direct (1.73 ms) by 257%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_scatter_null beats baseline by 77% (significant)

carrier_pre_scatter_null is -1.63 ms (77%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 5.0x slower than the field

carrier_pre_scatter_fntable (2.41 ms) is 5.0x the fastest (483.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_scatter_regcache shows alternating (throttle bounce) (autocorr -0.81)

carrier_pre_scatter_regcache's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_scatter_null} vs {carrier_pre_scatter_direct, carrier_pre_scatter_switch, carrier_pre_scatter_regcache, carrier_pre_scatter_threaded, carrier_pre_scatter_fntable} (257% apart)

The field splits into a fast tier {carrier_pre_scatter_null} and a slow tier {carrier_pre_scatter_direct, carrier_pre_scatter_switch, carrier_pre_scatter_regcache, carrier_pre_scatter_threaded, carrier_pre_scatter_fntable} with a 257% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.0x the fastest

Fastest carrier_pre_scatter_null (483.69 us) to slowest carrier_pre_scatter_fntable (2.41 ms): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 483691.4 ns median (-77.1% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.99x (fastest 483691.4 ns, slowest 2411280.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 1729863ns | 1731851ns | 1712629ns | 1729687ns | 1738746ns | -18.41% |
| carrier_pre_scatter_fntable | 2412611ns | 2414026ns | 2402072ns | 2412124ns | 2418612ns | +13.80% |
| carrier_pre_scatter_null | 488552ns | 485954ns | 477818ns | 485359ns | 498711ns | -76.96% |
| carrier_pre_scatter_regcache | 2231305ns | 2229772ns | 2219401ns | 2226397ns | 2244620ns | +5.24% |
| carrier_pre_scatter_switch | 2120120ns | 2117135ns | 2115559ns | 2116956ns | 2127148ns | base |
| carrier_pre_scatter_threaded | 2238398ns | 2241463ns | 2220909ns | 2237379ns | 2248672ns | +5.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 1727133ns | 1710215ns | 1735800ns | -18.43% | 0.009 |
| carrier_pre_scatter_fntable | 2409830ns | 2399316ns | 2415746ns | +13.81% | 0.007 |
| carrier_pre_scatter_null | 486291ns | 475644ns | 496365ns | -77.03% | 0.034 |
| carrier_pre_scatter_regcache | 2228490ns | 2216660ns | 2241604ns | +5.25% | 0.007 |
| carrier_pre_scatter_switch | 2117359ns | 2113005ns | 2124148ns | base | 0.008 |
| carrier_pre_scatter_threaded | 2235686ns | 2218319ns | 2245743ns | +5.59% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 11019086 | 9064105 | 1.216 | 0.81× |
| carrier_pre_scatter_fntable | 15249724 | 13556980 | 1.125 | 1.11× |
| carrier_pre_scatter_null | 3042828 | 11629264 | 0.262 | 0.22× |
| carrier_pre_scatter_regcache | 14121666 | 15645458 | 0.903 | 1.03× |
| carrier_pre_scatter_switch | 13687865 | 11550863 | 1.185 | 1.00× |
| carrier_pre_scatter_threaded | 14166086 | 11949154 | 1.186 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.009 | 27.5% |
| carrier_pre_scatter_fntable | 0.007 | 19.7% |
| carrier_pre_scatter_null | 0.034 | 98.3% |
| carrier_pre_scatter_regcache | 0.007 | 21.4% |
| carrier_pre_scatter_switch | 0.008 | 22.5% |
| carrier_pre_scatter_threaded | 0.007 | 21.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 1729863ns | 1729863ns | -18.41% |
| carrier_pre_scatter_fntable | 2412611ns | 2412611ns | +13.80% |
| carrier_pre_scatter_null | 488552ns | 488552ns | -76.96% |
| carrier_pre_scatter_regcache | 2231305ns | 2231305ns | +5.24% |
| carrier_pre_scatter_switch | 2120120ns | 2120120ns | base |
| carrier_pre_scatter_threaded | 2238398ns | 2238398ns | +5.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 2114391ns | base | --- | [2113537, 2124148] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 1729071ns | -387735.0ns (-18.3%) | [-404468, -378474]ns | [1716528, 1735800] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 2411280ns | +292172.9ns (+13.8%) | [+283770, +301471]ns | [2402464, 2415746] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_null | 483691ns | -1632832.3ns (-77.2%) | [-1642461, -1617910]ns | [478816, 496365] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 2227140ns | +112209.5ns (+5.3%) | [+100570, +120615]ns | [2216727, 2241604] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_scatter_threaded | 2238713ns | +119438.6ns (+5.6%) | [+108414, +127128]ns | [2222600, 2245743] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 2114070ns | -18.0% | +14.2% | -76.6% | +5.9% | +4.9% |
| 2 | 2119309ns | -18.2% | +13.5% | -77.1% | +4.6% | +6.0% |
| 3 | 2128987ns | -19.1% | +13.3% | -77.3% | +5.4% | +5.5% |
| 4 | 2113005ns | -19.1% | +14.1% | -77.2% | +4.9% | +6.1% |
| 5 | 2114302ns | -18.4% | +13.5% | -77.5% | +5.5% | +5.3% |
| 6 | 2114480ns | -17.8% | +14.3% | -76.5% | +5.1% | +5.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | 0.198 | ok |
| carrier_pre_scatter_fntable | -0.537 | HIGH- (thermal bounce) |
| carrier_pre_scatter_null | -0.182 | ok |
| carrier_pre_scatter_regcache | -0.808 | HIGH- (thermal bounce) |
| carrier_pre_scatter_switch | -0.066 | ok |
| carrier_pre_scatter_threaded | -0.110 | ok |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 0/6, lost 6/6
- **carrier_pre_scatter_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 1795857.1ns | 1727133.0ns | 104.0% | HIGH |
| carrier_pre_scatter_fntable | 2464752.9ns | 2409830.4ns | 102.3% | HIGH |
| carrier_pre_scatter_null | 483027.9ns | 486291.0ns | 99.3% | HIGH |
| carrier_pre_scatter_regcache | 2286094.8ns | 2228490.2ns | 102.6% | HIGH |
| carrier_pre_scatter_switch | 2257337.7ns | 2117358.8ns | 106.6% | HIGH |
| carrier_pre_scatter_threaded | 2292883.2ns | 2235685.6ns | 102.6% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 1710214.6-1735800.4 ns)
  1710214.6 |####################
  1711493.9 |
  1712773.2 |
  1714052.5 |
  1715331.8 |
  1716611.1 |
  1717890.3 |
  1719169.6 |
  1720448.9 |
  1721728.2 |####################
  1723007.5 |
  1724286.8 |####################
  1725566.1 |
  1726845.4 |
  1728124.7 |
  1729403.9 |
  1730683.2 |
  1731962.5 |
  1733241.8 |########################################
  1734521.1 |
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 2399316.2-2415746.2 ns)
  2399316.2 |####################
  2400137.7 |
  2400959.2 |
  2401780.7 |
  2402602.2 |
  2403423.7 |
  2404245.2 |
  2405066.7 |####################
  2405888.2 |
  2406709.7 |
  2407531.2 |
  2408352.7 |
  2409174.2 |
  2409995.7 |
  2410817.2 |########################################
  2411638.7 |
  2412460.2 |
  2413281.7 |
  2414103.2 |
  2414924.7 |####################
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 475643.8-496365.0 ns)
  475643.8 |####################
  476679.9 |
  477715.9 |
  478752.0 |
  479788.0 |
  480824.1 |
  481860.2 |########################################
  482896.2 |
  483932.3 |####################
  484968.3 |
  486004.4 |
  487040.5 |
  488076.5 |
  489112.6 |
  490148.6 |
  491184.7 |
  492220.8 |
  493256.8 |
  494292.9 |
  495328.9 |####################
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 2216660.0-2241603.8 ns)
  2216660.0 |########################################
  2217907.2 |
  2219154.4 |
  2220401.6 |
  2221648.8 |
  2222895.9 |####################
  2224143.1 |
  2225390.3 |
  2226637.5 |
  2227884.7 |
  2229131.9 |
  2230379.1 |####################
  2231626.2 |
  2232873.4 |
  2234120.6 |
  2235367.8 |
  2236615.0 |
  2237862.2 |####################
  2239109.4 |
  2240356.6 |
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 2113004.6-2124148.0 ns)
  2113004.6 |####################
  2113561.8 |####################
  2114118.9 |########################################
  2114676.1 |
  2115233.3 |
  2115790.4 |
  2116347.6 |
  2116904.8 |
  2117461.9 |
  2118019.1 |
  2118576.3 |
  2119133.4 |####################
  2119690.6 |
  2120247.8 |
  2120804.9 |
  2121362.1 |
  2121919.3 |
  2122476.4 |
  2123033.6 |
  2123590.8 |
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 2218318.8-2245743.4 ns)
  2218318.8 |########################################
  2219690.0 |
  2221061.3 |
  2222432.5 |
  2223803.7 |
  2225174.9 |
  2226546.2 |########################################
  2227917.4 |
  2229288.6 |
  2230659.8 |
  2232031.1 |
  2233402.3 |
  2234773.5 |
  2236144.8 |########################################
  2237516.0 |
  2238887.2 |
  2240258.4 |########################################
  2241629.7 |
  2243000.9 |
  2244372.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=103.9% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=102.2% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=102.6% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=106.8% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=102.5% of algo (FFI overhead may distort results)
