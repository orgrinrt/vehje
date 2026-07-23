# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), madd profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_madd_null dominates: 25% faster than the next best (carrier_cold_madd_threaded)

carrier_cold_madd_null (1.82 us) leads carrier_cold_madd_threaded (2.27 us) by 25%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_madd_null beats baseline by 20% (significant)

carrier_cold_madd_null is -468 ns (20%) faster than baseline carrier_cold_madd_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_madd_threaded's edge over baseline is significant but tiny (-43 ns, 1.89%)

carrier_cold_madd_threaded differs from baseline carrier_cold_madd_switch by -43 ns (1.89%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_cold_madd_null** at 1818.3 ns median (-20.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.39x (fastest 1818.3 ns, slowest 2528.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_madd_fntable | 4811ns | 4803ns | 4588ns | 4769ns | 4985ns | +3.66% |
| carrier_cold_madd_null | 4137ns | 4130ns | 3973ns | 4089ns | 4292ns | -10.85% |
| carrier_cold_madd_switch | 4641ns | 4594ns | 4521ns | 4587ns | 4782ns | base |
| carrier_cold_madd_threaded | 4594ns | 4654ns | 4255ns | 4546ns | 4836ns | -1.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_madd_fntable | 2524ns | 2421ns | 2614ns | +9.66% | 0.025 |
| carrier_cold_madd_null | 1828ns | 1774ns | 1878ns | -20.56% | 0.035 |
| carrier_cold_madd_switch | 2301ns | 2229ns | 2379ns | base | 0.028 |
| carrier_cold_madd_threaded | 2263ns | 2127ns | 2391ns | -1.66% | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_madd_fntable | 277949 | 890391 | 0.312 | 0.99× |
| carrier_cold_madd_null | 274020 | 1059176 | 0.259 | 0.97× |
| carrier_cold_madd_switch | 281761 | 831940 | 0.339 | 1.00× |
| carrier_cold_madd_threaded | 273880 | 860333 | 0.318 | 0.97× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_cold_madd_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_madd_fntable | 0.025 | 70.1% |
| carrier_cold_madd_null | 0.035 | 97.6% |
| carrier_cold_madd_switch | 0.028 | 77.7% |
| carrier_cold_madd_threaded | 0.028 | 78.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_madd_fntable | 4811ns | 4811ns | +3.66% |
| carrier_cold_madd_null | 4137ns | 4137ns | -10.85% |
| carrier_cold_madd_switch | 4641ns | 4641ns | base |
| carrier_cold_madd_threaded | 4594ns | 4594ns | -1.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_madd_switch | 2284ns | base | --- | [2241, 2379] | --- | --- | --- | --- |
| carrier_cold_madd_fntable | 2529ns | +160.8ns (+7.0%) | [+134, +372]ns | [2428, 2614] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_madd_null | 1818ns | -467.5ns (-20.5%) | [-588, -364]ns | [1788, 1878] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_madd_threaded | 2267ns | no significant difference | [-139, +68]ns | [2132, 2391] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_madd_switch | carrier_cold_madd_fntable | carrier_cold_madd_null | carrier_cold_madd_threaded |
|---|---|---|---|---|
| 1 | 2229ns | +14.7% | -17.0% | -4.1% |
| 2 | 2253ns | +7.5% | -18.8% | -2.3% |
| 3 | 2313ns | +5.3% | -22.1% | -8.0% |
| 4 | 2255ns | +18.5% | -15.5% | +7.0% |
| 5 | 2402ns | +6.4% | -26.1% | -1.4% |
| 6 | 2356ns | +6.2% | -23.3% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_madd_fntable | -0.077 | ok |
| carrier_cold_madd_null | -0.484 | moderate- |
| carrier_cold_madd_switch | 0.140 | ok |
| carrier_cold_madd_threaded | 0.251 | moderate+ |

**Consistency summary:**

- **carrier_cold_madd_fntable**: won 0/6, lost 6/6
- **carrier_cold_madd_null**: won 6/6, lost 0/6
- **carrier_cold_madd_threaded**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_madd_fntable | 86554.3ns | 2523.7ns | 3429.6% | HIGH |
| carrier_cold_madd_null | 85906.1ns | 1828.2ns | 4698.8% | HIGH |
| carrier_cold_madd_switch | 89263.3ns | 2301.4ns | 3878.7% | HIGH |
| carrier_cold_madd_threaded | 86196.6ns | 2263.2ns | 3808.5% | HIGH |

## Distribution (algo ns)

```
carrier_cold_madd_fntable (n=6, range 2420.8-2614.2 ns)
   2420.8 |####################
   2430.5 |####################
   2440.1 |
   2449.8 |
   2459.5 |
   2469.1 |
   2478.8 |
   2488.5 |
   2498.1 |####################
   2507.8 |
   2517.5 |
   2527.1 |
   2536.8 |
   2546.5 |########################################
   2556.1 |
   2565.8 |
   2575.5 |
   2585.1 |
   2594.8 |
   2604.5 |
  (0 below, 1 above range)

carrier_cold_madd_null (n=6, range 1773.8-1878.1 ns)
   1773.8 |########################################
   1779.0 |
   1784.2 |
   1789.4 |
   1794.7 |
   1799.9 |########################################
   1805.1 |########################################
   1810.3 |
   1815.5 |
   1820.7 |
   1825.9 |########################################
   1831.2 |
   1836.4 |
   1841.6 |
   1846.8 |########################################
   1852.0 |
   1857.2 |
   1862.5 |
   1867.7 |
   1872.9 |
  (0 below, 1 above range)

carrier_cold_madd_switch (n=6, range 2229.2-2378.9 ns)
   2229.2 |####################
   2236.7 |
   2244.2 |
   2251.7 |########################################
   2259.1 |
   2266.6 |
   2274.1 |
   2281.6 |
   2289.1 |
   2296.6 |
   2304.1 |
   2311.6 |####################
   2319.0 |
   2326.5 |
   2334.0 |
   2341.5 |
   2349.0 |####################
   2356.5 |
   2364.0 |
   2371.5 |
  (0 below, 1 above range)

carrier_cold_madd_threaded (n=6, range 2127.1-2390.6 ns)
   2127.1 |########################################
   2140.3 |
   2153.4 |
   2166.6 |
   2179.8 |
   2193.0 |####################
   2206.2 |
   2219.3 |
   2232.5 |
   2245.7 |
   2258.9 |
   2272.0 |
   2285.2 |
   2298.4 |
   2311.6 |
   2324.7 |####################
   2337.9 |
   2351.1 |
   2364.3 |####################
   2377.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_madd_fntable**: bridge=3425.5% of algo (FFI overhead may distort results)
- **carrier_cold_madd_null**: bridge=4730.3% of algo (FFI overhead may distort results)
- **carrier_cold_madd_switch**: bridge=3908.3% of algo (FFI overhead may distort results)
- **carrier_cold_madd_threaded**: bridge=3800.3% of algo (FFI overhead may distort results)
