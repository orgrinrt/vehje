# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_wideselect_rec20 is fastest but the noisiest (CV 13.5%)

carrier_lay_wideselect_rec20 wins on median (2.13 ms) yet has the highest variance (CV 13.5%), while carrier_lay_wideselect_rec12 is the steadiest (CV 0.5%, 2.13 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_lay_wideselect_rec24 shows alternating (throttle bounce) (autocorr -0.53)

carrier_lay_wideselect_rec24's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (27.21 us) is smaller than the fastest variant's own run-to-run std-dev (287.53 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_wideselect_rec20 vs stability leader carrier_lay_wideselect_rec12 (+0% speed for 29.5x steadier)

carrier_lay_wideselect_rec20 is fastest (2.13 ms, CV 13.5%); carrier_lay_wideselect_rec12 gives up 0.2% median for 29.5x lower variance (CV 0.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.3% of the fastest

All 5 variants sit between 2.13 ms and 2.15 ms - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_wideselect_rec20** at 2125310.0 ns median (-0.7% vs baseline)
- Spread: 1.01x (fastest 2125310.0 ns, slowest 2152521.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2136719ns | 2132150ns | 2126832ns | 2131013ns | 2150223ns | -0.68% |
| carrier_lay_wideselect_rec16 | 2137385ns | 2133830ns | 2117736ns | 2129805ns | 2158581ns | -0.65% |
| carrier_lay_wideselect_rec20 | 2255009ns | 2128336ns | 2089442ns | 2120918ns | 2538928ns | +4.81% |
| carrier_lay_wideselect_rec24 | 2151430ns | 2143511ns | 2111573ns | 2142024ns | 2185468ns | base |
| carrier_lay_wideselect_rec32 | 2144714ns | 2155558ns | 2090000ns | 2151003ns | 2162635ns | -0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2133721ns | 2124180ns | 2146942ns | -0.68% | 0.008 |
| carrier_lay_wideselect_rec16 | 2134326ns | 2114948ns | 2155429ns | -0.65% | 0.008 |
| carrier_lay_wideselect_rec20 | 2251991ns | 2086233ns | 2535748ns | +4.83% | 0.007 |
| carrier_lay_wideselect_rec24 | 2148277ns | 2109221ns | 2182253ns | base | 0.008 |
| carrier_lay_wideselect_rec32 | 2141613ns | 2087566ns | 2159307ns | -0.31% | 0.008 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_lay_wideselect_rec20; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.008 | 98.0% |
| carrier_lay_wideselect_rec16 | 0.008 | 97.9% |
| carrier_lay_wideselect_rec20 | 0.008 | 98.2% |
| carrier_lay_wideselect_rec24 | 0.008 | 97.5% |
| carrier_lay_wideselect_rec32 | 0.008 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2136719ns | 2136719ns | -0.68% |
| carrier_lay_wideselect_rec16 | 2137385ns | 2137385ns | -0.65% |
| carrier_lay_wideselect_rec20 | 2255009ns | 2255009ns | +4.81% |
| carrier_lay_wideselect_rec24 | 2151430ns | 2151430ns | base |
| carrier_lay_wideselect_rec32 | 2144714ns | 2144714ns | -0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 2139996ns | base | --- | [2122581, 2182253] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 2129221ns | no significant difference | [-56963, +20512]ns | [2125001, 2146942] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_wideselect_rec16 | 2130729ns | no significant difference | [-48733, +21665]ns | [2116819, 2155429] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_wideselect_rec20 | 2125310ns | no significant difference | [-39190, +353495]ns | [2094917, 2535748] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_wideselect_rec32 | 2152522ns | no significant difference | [-39615, +19382]ns | [2113011, 2159307] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 2143637ns | +0.3% | +1.3% | -2.7% | +0.8% |
| 2 | 2149629ns | -1.1% | -0.6% | +1.4% | -0.5% |
| 3 | 2136355ns | -0.2% | -1.0% | -1.0% | +0.5% |
| 4 | 2135940ns | -0.5% | -0.8% | -0.0% | +1.0% |
| 5 | 2214877ns | -4.1% | -3.4% | +30.6% | -2.6% |
| 6 | 2109221ns | +1.7% | +0.7% | -0.3% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | -0.204 | moderate- |
| carrier_lay_wideselect_rec16 | 0.111 | ok |
| carrier_lay_wideselect_rec20 | -0.266 | moderate- |
| carrier_lay_wideselect_rec24 | -0.525 | HIGH- (thermal bounce) |
| carrier_lay_wideselect_rec32 | -0.153 | ok |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 4/6, lost 2/6
- **carrier_lay_wideselect_rec16**: won 4/6, lost 2/6
- **carrier_lay_wideselect_rec20**: won 3/6, lost 2/6
- **carrier_lay_wideselect_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2137119.0ns | 2133721.0ns | 100.2% | HIGH |
| carrier_lay_wideselect_rec16 | 2140500.0ns | 2134326.0ns | 100.3% | HIGH |
| carrier_lay_wideselect_rec20 | 2317025.4ns | 2251991.4ns | 102.9% | HIGH |
| carrier_lay_wideselect_rec24 | 2284387.9ns | 2148276.6ns | 106.3% | HIGH |
| carrier_lay_wideselect_rec32 | 2143730.4ns | 2141613.1ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 2124179.6-2146941.7 ns)
  2124179.6 |####################
  2125317.7 |########################################
  2126455.8 |
  2127593.9 |
  2128732.0 |
  2129870.1 |
  2131008.2 |####################
  2132146.3 |
  2133284.4 |
  2134422.5 |
  2135560.6 |
  2136698.7 |
  2137836.8 |
  2138974.9 |
  2140113.0 |
  2141251.1 |
  2142389.2 |
  2143527.3 |####################
  2144665.4 |
  2145803.5 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 2114947.9-2155429.4 ns)
  2114947.9 |####################
  2116972.0 |####################
  2118996.0 |
  2121020.1 |
  2123044.2 |####################
  2125068.3 |
  2127092.4 |
  2129116.4 |
  2131140.5 |
  2133164.6 |
  2135188.6 |
  2137212.7 |########################################
  2139236.8 |
  2141260.9 |
  2143284.9 |
  2145309.0 |
  2147333.1 |
  2149357.2 |
  2151381.2 |
  2153405.3 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 2086232.9-2535747.5 ns)
  2086232.9 |########################################
  2108708.6 |####################
  2131184.4 |####################
  2153660.1 |
  2176135.8 |####################
  2198611.5 |
  2221087.3 |
  2243563.0 |
  2266038.7 |
  2288514.5 |
  2310990.2 |
  2333465.9 |
  2355941.7 |
  2378417.4 |
  2400893.1 |
  2423368.9 |
  2445844.6 |
  2468320.3 |
  2490796.0 |
  2513271.8 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 2109221.2-2182252.9 ns)
  2109221.2 |####################
  2112872.8 |
  2116524.4 |
  2120176.0 |
  2123827.5 |
  2127479.1 |
  2131130.7 |
  2134782.3 |########################################
  2138433.9 |
  2142085.5 |####################
  2145737.1 |
  2149388.6 |####################
  2153040.2 |
  2156691.8 |
  2160343.4 |
  2163995.0 |
  2167646.6 |
  2171298.1 |
  2174949.7 |
  2178601.3 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 2087565.8-2159306.7 ns)
  2087565.8 |####################
  2091152.8 |
  2094739.9 |
  2098326.9 |
  2101914.0 |
  2105501.0 |
  2109088.1 |
  2112675.1 |
  2116262.2 |
  2119849.2 |
  2123436.2 |
  2127023.3 |
  2130610.3 |
  2134197.4 |
  2137784.4 |####################
  2141371.5 |
  2144958.5 |####################
  2148545.6 |
  2152132.6 |
  2155719.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=100.0% of algo (FFI overhead may distort results)
