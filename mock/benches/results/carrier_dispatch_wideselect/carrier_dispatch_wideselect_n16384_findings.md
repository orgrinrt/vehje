# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 221% faster than the next best (carrier_disp_wideselect_bittree)

carrier_disp_wideselect_nullfloor (518.65 us) leads carrier_disp_wideselect_bittree (1.66 ms) by 221%, a clear separation rather than a photo finish. CV 5.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 76% (significant)

carrier_disp_wideselect_nullfloor is -1.62 ms (76%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 5.2x slower than the field

carrier_disp_wideselect_ifchainlin (2.71 ms) is 5.2x the fastest (518.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_wideselect_nullfloor is fastest but the noisiest (CV 5.4%)

carrier_disp_wideselect_nullfloor wins on median (518.65 us) yet has the highest variance (CV 5.4%), while carrier_disp_wideselect_switch is the steadiest (CV 0.4%, 2.13 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_disp_wideselect_fntable shows alternating (throttle bounce) (autocorr -0.58)

carrier_disp_wideselect_fntable's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_wideselect_nullfloor} vs {carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_ifchainlin} (221% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor} and a slow tier {carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_ifchainlin} with a 221% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest carrier_disp_wideselect_nullfloor (518.65 us) to slowest carrier_disp_wideselect_ifchainlin (2.71 ms): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 518649.6 ns median (-75.7% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.23x (fastest 518649.6 ns, slowest 2711992.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 1674558ns | 1667310ns | 1651958ns | 1666181ns | 1698422ns | -21.70% |
| carrier_disp_wideselect_fntable | 2076540ns | 2073060ns | 2036867ns | 2062322ns | 2117702ns | -2.90% |
| carrier_disp_wideselect_ifchain | 2201157ns | 2197744ns | 2153385ns | 2196615ns | 2231857ns | +2.93% |
| carrier_disp_wideselect_ifchainasc | 2141770ns | 2143666ns | 2114499ns | 2140445ns | 2157392ns | +0.15% |
| carrier_disp_wideselect_ifchainlin | 2717748ns | 2715883ns | 2685638ns | 2713432ns | 2740277ns | +27.08% |
| carrier_disp_wideselect_nullfloor | 527218ns | 521921ns | 488051ns | 520367ns | 557078ns | -75.35% |
| carrier_disp_wideselect_switch | 2138548ns | 2135899ns | 2127799ns | 2135286ns | 2148816ns | base |
| carrier_disp_wideselect_threaded | 2314458ns | 2314727ns | 2301496ns | 2311189ns | 2325843ns | +8.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 1670716ns | 1647915ns | 1694674ns | -21.73% | 0.010 |
| carrier_disp_wideselect_fntable | 2072778ns | 2033269ns | 2113751ns | -2.89% | 0.008 |
| carrier_disp_wideselect_ifchain | 2197203ns | 2149810ns | 2227466ns | +2.94% | 0.007 |
| carrier_disp_wideselect_ifchainasc | 2137714ns | 2110122ns | 2153171ns | +0.15% | 0.008 |
| carrier_disp_wideselect_ifchainlin | 2713700ns | 2681633ns | 2736142ns | +27.14% | 0.006 |
| carrier_disp_wideselect_nullfloor | 523917ns | 485054ns | 553770ns | -75.45% | 0.031 |
| carrier_disp_wideselect_switch | 2134484ns | 2123805ns | 2144913ns | base | 0.008 |
| carrier_disp_wideselect_threaded | 2310338ns | 2297456ns | 2321666ns | +8.24% | 0.007 |

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.010 | 29.2% |
| carrier_disp_wideselect_fntable | 0.008 | 23.4% |
| carrier_disp_wideselect_ifchain | 0.007 | 22.1% |
| carrier_disp_wideselect_ifchainasc | 0.008 | 22.7% |
| carrier_disp_wideselect_ifchainlin | 0.006 | 17.9% |
| carrier_disp_wideselect_nullfloor | 0.032 | 93.5% |
| carrier_disp_wideselect_switch | 0.008 | 22.8% |
| carrier_disp_wideselect_threaded | 0.007 | 21.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 1674558ns | 1674558ns | -21.70% |
| carrier_disp_wideselect_fntable | 2076540ns | 2076540ns | -2.90% |
| carrier_disp_wideselect_ifchain | 2201157ns | 2201157ns | +2.93% |
| carrier_disp_wideselect_ifchainasc | 2141770ns | 2141770ns | +0.15% |
| carrier_disp_wideselect_ifchainlin | 2717748ns | 2717748ns | +27.08% |
| carrier_disp_wideselect_nullfloor | 527218ns | 527218ns | -75.35% |
| carrier_disp_wideselect_switch | 2138548ns | 2138548ns | base |
| carrier_disp_wideselect_threaded | 2314458ns | 2314458ns | +8.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 2131762ns | base | --- | [2126778, 2144913] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 1663576ns | -470138.6ns (-22.1%) | [-484017, -437150]ns | [1653897, 1694674] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 2069170ns | -71876.8ns (-3.4%) | [-91364, -21878]ns | [2035414, 2113751] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchain | 2193942ns | +66742.3ns (+3.1%) | [+25290, +96126]ns | [2170202, 2227466] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchainasc | 2139882ns | no significant difference | [-12206, +23460]ns | [2120089, 2153171] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_wideselect_ifchainlin | 2711993ns | +574056.5ns (+26.9%) | [+558790, +604802]ns | [2692966, 2736142] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 518650ns | -1615713.1ns (-75.8%) | [-1637735, -1578252]ns | [499332, 553770] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 2310509ns | +175596.1ns (+8.2%) | [+164799, +187166]ns | [2298838, 2321666] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2151498ns | -22.2% | -3.8% | +1.8% | -0.0% | +26.4% | -75.4% | +8.2% |
| 2 | 2138328ns | -22.9% | -0.4% | +0.5% | -0.2% | +26.5% | -77.3% | +7.6% |
| 3 | 2123805ns | -21.7% | -4.1% | +3.4% | +1.5% | +27.3% | -75.6% | +9.0% |
| 4 | 2132930ns | -22.2% | -1.6% | +4.5% | -0.1% | +28.4% | -75.9% | +8.6% |
| 5 | 2129750ns | -19.5% | -4.5% | +4.5% | +0.7% | +28.3% | -72.8% | +7.9% |
| 6 | 2130595ns | -21.9% | -2.9% | +2.8% | -1.0% | +25.9% | -75.7% | +8.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | -0.237 | moderate- |
| carrier_disp_wideselect_fntable | -0.577 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_ifchain | 0.255 | moderate+ |
| carrier_disp_wideselect_ifchainasc | -0.368 | moderate- |
| carrier_disp_wideselect_ifchainlin | -0.147 | ok |
| carrier_disp_wideselect_nullfloor | -0.175 | ok |
| carrier_disp_wideselect_switch | 0.146 | ok |
| carrier_disp_wideselect_threaded | -0.343 | moderate- |

**Consistency summary:**

- **carrier_disp_wideselect_bittree**: won 6/6, lost 0/6
- **carrier_disp_wideselect_fntable**: won 6/6, lost 0/6
- **carrier_disp_wideselect_ifchain**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchainasc**: won 3/6, lost 2/6
- **carrier_disp_wideselect_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_wideselect_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_wideselect_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 1672302.2ns | 1670715.6ns | 100.1% | HIGH |
| carrier_disp_wideselect_fntable | 2014671.6ns | 2072778.2ns | 97.2% | HIGH |
| carrier_disp_wideselect_ifchain | 2200245.5ns | 2197203.5ns | 100.1% | HIGH |
| carrier_disp_wideselect_ifchainasc | 2140952.5ns | 2137714.0ns | 100.2% | HIGH |
| carrier_disp_wideselect_ifchainlin | 2716135.1ns | 2713700.2ns | 100.1% | HIGH |
| carrier_disp_wideselect_nullfloor | 526773.2ns | 523917.2ns | 100.5% | HIGH |
| carrier_disp_wideselect_switch | 2138020.8ns | 2134484.2ns | 100.2% | HIGH |
| carrier_disp_wideselect_threaded | 2314630.6ns | 2310337.8ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 1647914.6-1694673.8 ns)
  1647914.6 |####################
  1650252.6 |
  1652590.5 |
  1654928.5 |
  1657266.4 |
  1659604.4 |####################
  1661942.3 |########################################
  1664280.3 |
  1666618.3 |
  1668956.2 |
  1671294.2 |
  1673632.1 |####################
  1675970.1 |
  1678308.0 |
  1680646.0 |
  1682984.0 |
  1685321.9 |
  1687659.9 |
  1689997.8 |
  1692335.8 |
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 2033269.2-2113750.8 ns)
  2033269.2 |########################################
  2037293.3 |########################################
  2041317.4 |
  2045341.4 |
  2049365.5 |
  2053389.6 |
  2057413.7 |
  2061437.8 |
  2065461.9 |########################################
  2069485.9 |########################################
  2073510.0 |
  2077534.1 |
  2081558.2 |
  2085582.3 |
  2089606.4 |
  2093630.4 |
  2097654.5 |########################################
  2101678.6 |
  2105702.7 |
  2109726.8 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 2149809.6-2227465.8 ns)
  2149809.6 |####################
  2153692.4 |
  2157575.2 |
  2161458.0 |
  2165340.8 |
  2169223.6 |
  2173106.5 |
  2176989.3 |
  2180872.1 |
  2184754.9 |
  2188637.7 |########################################
  2192520.5 |
  2196403.3 |####################
  2200286.1 |
  2204168.9 |
  2208051.8 |
  2211934.6 |
  2215817.4 |
  2219700.2 |
  2223583.0 |####################
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 2110122.1-2153170.9 ns)
  2110122.1 |########################################
  2112274.5 |
  2114427.0 |
  2116579.4 |
  2118731.9 |
  2120884.3 |
  2123036.7 |
  2125189.2 |
  2127341.6 |
  2129494.0 |########################################
  2131646.5 |
  2133798.9 |########################################
  2135951.4 |
  2138103.8 |
  2140256.2 |
  2142408.7 |
  2144561.1 |########################################
  2146713.5 |
  2148866.0 |
  2151018.4 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 2681633.3-2736141.8 ns)
  2681633.3 |####################
  2684358.7 |
  2687084.2 |
  2689809.6 |
  2692535.0 |
  2695260.4 |
  2697985.9 |
  2700711.3 |
  2703436.7 |########################################
  2706162.1 |
  2708887.6 |
  2711613.0 |
  2714338.4 |
  2717063.9 |####################
  2719789.3 |
  2722514.7 |
  2725240.1 |
  2727965.6 |
  2730691.0 |####################
  2733416.4 |
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 485053.8-553769.8 ns)
  485053.8 |####################
  488489.6 |
  491925.4 |
  495361.2 |
  498797.0 |
  502232.8 |
  505668.6 |
  509104.4 |
  512540.2 |####################
  515976.0 |########################################
  519411.8 |
  522847.6 |
  526283.4 |####################
  529719.2 |
  533155.0 |
  536590.8 |
  540026.6 |
  543462.4 |
  546898.2 |
  550334.0 |
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 2123805.0-2144912.7 ns)
  2123805.0 |########################################
  2124860.4 |
  2125915.8 |
  2126971.2 |
  2128026.5 |
  2129081.9 |########################################
  2130137.3 |########################################
  2131192.7 |
  2132248.1 |########################################
  2133303.5 |
  2134358.9 |
  2135414.2 |
  2136469.6 |
  2137525.0 |########################################
  2138580.4 |
  2139635.8 |
  2140691.2 |
  2141746.5 |
  2142801.9 |
  2143857.3 |
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 2297455.8-2321666.2 ns)
  2297455.8 |########################################
  2298666.3 |
  2299876.8 |########################################
  2301087.4 |
  2302297.9 |
  2303508.4 |
  2304718.9 |
  2305929.5 |########################################
  2307140.0 |
  2308350.5 |
  2309561.0 |
  2310771.5 |
  2311982.1 |
  2313192.6 |
  2314403.1 |########################################
  2315613.6 |########################################
  2316824.2 |
  2318034.7 |
  2319245.2 |
  2320455.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=97.0% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=100.2% of algo (FFI overhead may distort results)
