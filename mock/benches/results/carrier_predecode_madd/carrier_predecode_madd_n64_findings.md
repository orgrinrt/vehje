# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_pre_madd_regcache, carrier_pre_madd_null) are a dead heat (<1%)

carrier_pre_madd_regcache (2.06 us) and carrier_pre_madd_null (2.08 us) differ by 0.97%, inside the noise, even though the wider field spreads 19.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_pre_madd_regcache** at 2060.2 ns median (-13.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.19x (fastest 2060.2 ns, slowest 2456.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 4609ns | 4587ns | 4551ns | 4584ns | 4674ns | -3.88% |
| carrier_pre_madd_fntable | 4786ns | 4890ns | 4288ns | 4756ns | 5080ns | -0.17% |
| carrier_pre_madd_null | 4510ns | 4510ns | 4453ns | 4500ns | 4553ns | -5.93% |
| carrier_pre_madd_regcache | 4486ns | 4496ns | 4436ns | 4477ns | 4525ns | -6.43% |
| carrier_pre_madd_switch | 4794ns | 4794ns | 4745ns | 4785ns | 4834ns | base |
| carrier_pre_madd_threaded | 4558ns | 4628ns | 4195ns | 4625ns | 4640ns | -4.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 2185ns | 2155ns | 2220ns | -7.97% | 0.029 |
| carrier_pre_madd_fntable | 2432ns | 2153ns | 2637ns | +2.42% | 0.026 |
| carrier_pre_madd_null | 2082ns | 2058ns | 2102ns | -12.34% | 0.031 |
| carrier_pre_madd_regcache | 2061ns | 2042ns | 2080ns | -13.21% | 0.031 |
| carrier_pre_madd_switch | 2375ns | 2355ns | 2393ns | base | 0.027 |
| carrier_pre_madd_threaded | 2185ns | 2005ns | 2225ns | -8.01% | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_madd_direct | 254409 | 643886 | 0.395 | 0.97× |
| carrier_pre_madd_fntable | 267776 | 937359 | 0.286 | 1.02× |
| carrier_pre_madd_null | 251820 | 930760 | 0.271 | 0.96× |
| carrier_pre_madd_regcache | 251097 | 1231584 | 0.204 | 0.96× |
| carrier_pre_madd_switch | 262485 | 807992 | 0.325 | 1.00× |
| carrier_pre_madd_threaded | 258553 | 887262 | 0.291 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_pre_madd_threaded; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.029 | 92.0% |
| carrier_pre_madd_fntable | 0.026 | 81.6% |
| carrier_pre_madd_null | 0.031 | 96.4% |
| carrier_pre_madd_regcache | 0.031 | 97.3% |
| carrier_pre_madd_switch | 0.027 | 84.5% |
| carrier_pre_madd_threaded | 0.029 | 90.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 4609ns | 4609ns | -3.88% |
| carrier_pre_madd_fntable | 4786ns | 4786ns | -0.17% |
| carrier_pre_madd_null | 4510ns | 4510ns | -5.93% |
| carrier_pre_madd_regcache | 4486ns | 4486ns | -6.43% |
| carrier_pre_madd_switch | 4794ns | 4794ns | base |
| carrier_pre_madd_threaded | 4558ns | 4558ns | -4.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 2374ns | base | --- | [2358, 2393] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 2178ns | -196.2ns (-8.3%) | [-214, -158]ns | [2158, 2220] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_madd_fntable | 2456ns | no significant difference | [-154, +261]ns | [2203, 2637] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_madd_null | 2080ns | -289.1ns (-12.2%) | [-323, -267]ns | [2063, 2102] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 2060ns | -315.4ns (-13.3%) | [-338, -287]ns | [2042, 2080] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 2218ns | -159.0ns (-6.7%) | [-269, -142]ns | [2110, 2225] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 2355ns | -8.5% | -8.5% | -12.6% | -11.3% | -5.8% |
| 2 | 2380ns | -6.8% | +17.4% | -13.1% | -13.0% | -6.8% |
| 3 | 2375ns | -6.5% | +3.9% | -11.9% | -13.2% | -6.3% |
| 4 | 2361ns | -8.5% | -4.6% | -11.6% | -13.5% | -15.1% |
| 5 | 2405ns | -9.4% | +1.6% | -13.8% | -14.4% | -7.6% |
| 6 | 2372ns | -8.1% | +4.5% | -11.0% | -13.9% | -6.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | -0.119 | ok |
| carrier_pre_madd_fntable | -0.393 | moderate- |
| carrier_pre_madd_null | -0.044 | ok |
| carrier_pre_madd_regcache | 0.260 | moderate+ |
| carrier_pre_madd_switch | -0.405 | moderate- |
| carrier_pre_madd_threaded | -0.276 | moderate- |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 6/6, lost 0/6
- **carrier_pre_madd_fntable**: won 2/6, lost 4/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 6/6, lost 0/6
- **carrier_pre_madd_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 86907.4ns | 2185.3ns | 3976.9% | HIGH |
| carrier_pre_madd_fntable | 88606.6ns | 2432.0ns | 3643.3% | HIGH |
| carrier_pre_madd_null | 86342.9ns | 2081.7ns | 4147.6% | HIGH |
| carrier_pre_madd_regcache | 85762.7ns | 2061.0ns | 4161.3% | HIGH |
| carrier_pre_madd_switch | 89988.7ns | 2374.7ns | 3789.6% | HIGH |
| carrier_pre_madd_threaded | 86465.8ns | 2184.5ns | 3958.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 2154.6-2219.6 ns)
   2154.6 |####################
   2157.8 |
   2161.1 |####################
   2164.3 |
   2167.6 |
   2170.8 |
   2174.1 |
   2177.3 |########################################
   2180.6 |
   2183.8 |
   2187.1 |
   2190.3 |
   2193.6 |
   2196.8 |
   2200.1 |
   2203.3 |
   2206.6 |
   2209.8 |
   2213.1 |
   2216.3 |####################
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 2153.3-2636.7 ns)
   2153.3 |####################
   2177.5 |
   2201.6 |
   2225.8 |
   2250.0 |####################
   2274.2 |
   2298.3 |
   2322.5 |
   2346.7 |
   2370.8 |
   2395.0 |
   2419.2 |
   2443.3 |########################################
   2467.5 |####################
   2491.7 |
   2515.8 |
   2540.0 |
   2564.2 |
   2588.4 |
   2612.5 |
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 2057.9-2102.3 ns)
   2057.9 |########################################
   2060.1 |
   2062.3 |
   2064.6 |
   2066.8 |########################################
   2069.0 |
   2071.2 |########################################
   2073.4 |
   2075.7 |
   2077.9 |
   2080.1 |
   2082.3 |
   2084.5 |
   2086.8 |########################################
   2089.0 |
   2091.2 |
   2093.4 |########################################
   2095.6 |
   2097.9 |
   2100.1 |
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 2042.1-2080.2 ns)
   2042.1 |########################################
   2044.0 |
   2045.9 |
   2047.8 |
   2049.7 |
   2051.6 |
   2053.5 |
   2055.4 |
   2057.3 |####################
   2059.2 |
   2061.1 |####################
   2063.1 |
   2065.0 |
   2066.9 |
   2068.8 |
   2070.7 |####################
   2072.6 |
   2074.5 |
   2076.4 |
   2078.3 |
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 2354.6-2392.7 ns)
   2354.6 |########################################
   2356.5 |
   2358.4 |
   2360.3 |########################################
   2362.2 |
   2364.1 |
   2366.0 |
   2367.9 |
   2369.8 |########################################
   2371.7 |
   2373.6 |########################################
   2375.6 |
   2377.5 |
   2379.4 |########################################
   2381.3 |
   2383.2 |
   2385.1 |
   2387.0 |
   2388.9 |
   2390.8 |
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 2004.6-2224.8 ns)
   2004.6 |##########
   2015.6 |
   2026.6 |
   2037.6 |
   2048.6 |
   2059.7 |
   2070.7 |
   2081.7 |
   2092.7 |
   2103.7 |
   2114.7 |
   2125.7 |
   2136.7 |
   2147.7 |
   2158.7 |
   2169.8 |
   2180.8 |
   2191.8 |
   2202.8 |
   2213.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=3990.3% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=3551.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=4155.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=4165.4% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=3793.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=3894.1% of algo (FFI overhead may distort results)
