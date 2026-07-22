# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_pre_madd_regcache, carrier_pre_madd_null) are a dead heat (<1%)

carrier_pre_madd_regcache (2.09 us) and carrier_pre_madd_null (2.09 us) differ by 0.01%, inside the noise, even though the wider field spreads 18.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_pre_madd_threaded shows alternating (throttle bounce) (autocorr -0.51)

carrier_pre_madd_threaded's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_madd_regcache** at 2090.4 ns median (-12.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.19x (fastest 2090.4 ns, slowest 2481.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 4710ns | 4710ns | 4633ns | 4697ns | 4768ns | -2.60% |
| carrier_pre_madd_fntable | 4844ns | 4947ns | 4280ns | 4924ns | 5005ns | +0.16% |
| carrier_pre_madd_null | 4592ns | 4577ns | 4509ns | 4556ns | 4687ns | -5.05% |
| carrier_pre_madd_regcache | 4572ns | 4543ns | 4509ns | 4541ns | 4649ns | -5.46% |
| carrier_pre_madd_switch | 4836ns | 4838ns | 4755ns | 4828ns | 4888ns | base |
| carrier_pre_madd_threaded | 4745ns | 4733ns | 4706ns | 4726ns | 4792ns | -1.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 2230ns | 2195ns | 2270ns | -6.45% | 0.029 |
| carrier_pre_madd_fntable | 2424ns | 2131ns | 2500ns | +1.69% | 0.026 |
| carrier_pre_madd_null | 2094ns | 2063ns | 2121ns | -12.15% | 0.031 |
| carrier_pre_madd_regcache | 2090ns | 2055ns | 2124ns | -12.30% | 0.031 |
| carrier_pre_madd_switch | 2384ns | 2342ns | 2406ns | base | 0.027 |
| carrier_pre_madd_threaded | 2256ns | 2235ns | 2275ns | -5.34% | 0.028 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_pre_madd_regcache; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.029 | 92.7% |
| carrier_pre_madd_fntable | 0.026 | 82.8% |
| carrier_pre_madd_null | 0.031 | 98.3% |
| carrier_pre_madd_regcache | 0.031 | 98.3% |
| carrier_pre_madd_switch | 0.027 | 86.1% |
| carrier_pre_madd_threaded | 0.028 | 91.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 4710ns | 4710ns | -2.60% |
| carrier_pre_madd_fntable | 4844ns | 4844ns | +0.16% |
| carrier_pre_madd_null | 4592ns | 4592ns | -5.05% |
| carrier_pre_madd_regcache | 4572ns | 4572ns | -5.46% |
| carrier_pre_madd_switch | 4836ns | 4836ns | base |
| carrier_pre_madd_threaded | 4745ns | 4745ns | -1.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 2388ns | base | --- | [2356, 2406] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 2217ns | -179.3ns (-7.5%) | [-193, -89]ns | [2203, 2270] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_madd_fntable | 2482ns | no significant difference | [-85, +115]ns | [2290, 2500] | no | 0.2188 | 0.2188 | 0 |
| carrier_pre_madd_null | 2091ns | -282.3ns (-11.8%) | [-325, -262]ns | [2070, 2121] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 2090ns | -301.4ns (-12.6%) | [-332, -246]ns | [2057, 2124] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 2252ns | -128.5ns (-5.4%) | [-150, -103]ns | [2242, 2275] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 2342ns | -3.4% | -9.0% | -11.9% | -8.8% | -4.0% |
| 2 | 2375ns | -4.1% | +4.6% | -10.8% | -12.0% | -5.1% |
| 3 | 2371ns | -7.4% | +5.1% | -11.3% | -13.2% | -5.7% |
| 4 | 2404ns | -7.6% | +4.3% | -13.6% | -12.1% | -4.7% |
| 5 | 2402ns | -7.9% | +3.2% | -13.4% | -14.4% | -6.3% |
| 6 | 2409ns | -8.2% | +1.7% | -11.9% | -13.2% | -6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | 0.138 | ok |
| carrier_pre_madd_fntable | -0.016 | ok |
| carrier_pre_madd_null | -0.297 | moderate- |
| carrier_pre_madd_regcache | -0.332 | moderate- |
| carrier_pre_madd_switch | 0.307 | moderate+ |
| carrier_pre_madd_threaded | -0.513 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 6/6, lost 0/6
- **carrier_pre_madd_fntable**: won 1/6, lost 5/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 6/6, lost 0/6
- **carrier_pre_madd_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 87552.8ns | 2229.8ns | 3926.4% | HIGH |
| carrier_pre_madd_fntable | 86810.3ns | 2424.0ns | 3581.3% | HIGH |
| carrier_pre_madd_null | 86586.9ns | 2093.9ns | 4135.2% | HIGH |
| carrier_pre_madd_regcache | 86204.6ns | 2090.4ns | 4123.8% | HIGH |
| carrier_pre_madd_switch | 88545.2ns | 2383.6ns | 3714.7% | HIGH |
| carrier_pre_madd_threaded | 86565.6ns | 2256.4ns | 3836.4% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 2195.4-2269.8 ns)
   2195.4 |####################
   2199.1 |
   2202.8 |
   2206.6 |
   2210.3 |########################################
   2214.0 |
   2217.7 |####################
   2221.4 |
   2225.2 |
   2228.9 |
   2232.6 |
   2236.3 |
   2240.0 |
   2243.8 |
   2247.5 |
   2251.2 |
   2254.9 |
   2258.6 |
   2262.4 |####################
   2266.1 |
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 2130.8-2500.0 ns)
   2130.8 |####################
   2149.3 |
   2167.7 |
   2186.2 |
   2204.6 |
   2223.1 |
   2241.6 |
   2260.0 |
   2278.5 |
   2296.9 |
   2315.4 |
   2333.9 |
   2352.3 |
   2370.8 |
   2389.2 |
   2407.7 |
   2426.2 |
   2444.6 |####################
   2463.1 |####################
   2481.5 |########################################
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 2062.9-2120.9 ns)
   2062.9 |####################
   2065.8 |
   2068.7 |
   2071.6 |
   2074.5 |
   2077.4 |########################################
   2080.3 |
   2083.2 |
   2086.1 |
   2089.0 |
   2091.9 |
   2094.8 |
   2097.7 |
   2100.6 |####################
   2103.5 |
   2106.4 |
   2109.3 |
   2112.2 |
   2115.1 |
   2118.0 |####################
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 2055.4-2123.9 ns)
   2055.4 |########################################
   2058.8 |
   2062.3 |
   2065.7 |
   2069.1 |
   2072.5 |
   2076.0 |
   2079.4 |
   2082.8 |
   2086.2 |####################
   2089.7 |####################
   2093.1 |
   2096.5 |
   2100.0 |
   2103.4 |
   2106.8 |
   2110.2 |####################
   2113.7 |
   2117.1 |
   2120.5 |
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 2341.7-2406.2 ns)
   2341.7 |########################################
   2344.9 |
   2348.2 |
   2351.4 |
   2354.6 |
   2357.8 |
   2361.1 |
   2364.3 |
   2367.5 |
   2370.7 |########################################
   2374.0 |########################################
   2377.2 |
   2380.4 |
   2383.7 |
   2386.9 |
   2390.1 |
   2393.3 |
   2396.6 |
   2399.8 |########################################
   2403.0 |########################################
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 2234.6-2275.2 ns)
   2234.6 |########################################
   2236.6 |
   2238.7 |
   2240.7 |
   2242.7 |
   2244.8 |
   2246.8 |########################################
   2248.8 |########################################
   2250.8 |
   2252.9 |########################################
   2254.9 |
   2256.9 |
   2259.0 |########################################
   2261.0 |
   2263.0 |
   2265.0 |
   2267.1 |
   2269.1 |
   2271.1 |
   2273.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=3931.8% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=3511.1% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=4141.6% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=4122.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=3707.8% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=3845.2% of algo (FFI overhead may distort results)
