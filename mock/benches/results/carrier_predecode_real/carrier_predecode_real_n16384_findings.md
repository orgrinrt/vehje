# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 242% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (508.15 us) leads carrier_pre_real_direct (1.74 ms) by 242%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 76% (significant)

carrier_pre_real_null is -1.64 ms (76%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 4.8x slower than the field

carrier_pre_real_fntable (2.45 ms) is 4.8x the fastest (508.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_real_null} vs {carrier_pre_real_direct, carrier_pre_real_switch, carrier_pre_real_threaded, carrier_pre_real_regcache, carrier_pre_real_fntable} (242% apart)

The field splits into a fast tier {carrier_pre_real_null} and a slow tier {carrier_pre_real_direct, carrier_pre_real_switch, carrier_pre_real_threaded, carrier_pre_real_regcache, carrier_pre_real_fntable} with a 242% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.8x the fastest

Fastest carrier_pre_real_null (508.15 us) to slowest carrier_pre_real_fntable (2.45 ms): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_real_null** at 508152.9 ns median (-76.4% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.82x (fastest 508152.9 ns, slowest 2450734.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 1742126ns | 1739312ns | 1733812ns | 1737679ns | 1752953ns | -19.11% |
| carrier_pre_real_fntable | 2455651ns | 2453882ns | 2438527ns | 2452592ns | 2468801ns | +14.02% |
| carrier_pre_real_null | 513079ns | 510581ns | 506215ns | 509520ns | 521849ns | -76.18% |
| carrier_pre_real_regcache | 2262962ns | 2263013ns | 2243250ns | 2257267ns | 2281360ns | +5.08% |
| carrier_pre_real_switch | 2153637ns | 2155033ns | 2141617ns | 2150934ns | 2163703ns | base |
| carrier_pre_real_threaded | 2257008ns | 2262380ns | 2229115ns | 2259080ns | 2267847ns | +4.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 1739245ns | 1731141ns | 1749962ns | -19.13% | 0.009 |
| carrier_pre_real_fntable | 2452488ns | 2435480ns | 2465496ns | +14.04% | 0.007 |
| carrier_pre_real_null | 510621ns | 503942ns | 519186ns | -76.26% | 0.032 |
| carrier_pre_real_regcache | 2259880ns | 2240485ns | 2278138ns | +5.08% | 0.007 |
| carrier_pre_real_switch | 2150627ns | 2138712ns | 2160563ns | base | 0.008 |
| carrier_pre_real_threaded | 2253747ns | 2226433ns | 2264303ns | +4.79% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_real_direct | 11098671 | 8895487 | 1.248 | 0.81× |
| carrier_pre_real_fntable | 15323958 | 13447363 | 1.140 | 1.12× |
| carrier_pre_real_null | 3399244 | 11530515 | 0.295 | 0.25× |
| carrier_pre_real_regcache | 14205338 | 15388440 | 0.923 | 1.03× |
| carrier_pre_real_switch | 13735339 | 11447430 | 1.200 | 1.00× |
| carrier_pre_real_threaded | 14191627 | 11784806 | 1.204 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.009 | 29.0% |
| carrier_pre_real_fntable | 0.007 | 20.6% |
| carrier_pre_real_null | 0.032 | 99.2% |
| carrier_pre_real_regcache | 0.007 | 22.3% |
| carrier_pre_real_switch | 0.008 | 23.4% |
| carrier_pre_real_threaded | 0.007 | 22.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 1742126ns | 1742126ns | -19.11% |
| carrier_pre_real_fntable | 2455651ns | 2455651ns | +14.02% |
| carrier_pre_real_null | 513079ns | 513079ns | -76.18% |
| carrier_pre_real_regcache | 2262962ns | 2262962ns | +5.08% |
| carrier_pre_real_switch | 2153637ns | 2153637ns | base |
| carrier_pre_real_threaded | 2257008ns | 2257008ns | +4.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 2152032ns | base | --- | [2139286, 2160563] | --- | --- | --- | --- |
| carrier_pre_real_direct | 1736426ns | -407938.2ns (-19.0%) | [-420723, -405484]ns | [1731348, 1749962] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_fntable | 2450735ns | +307673.8ns (+14.3%) | [+283218, +314693]ns | [2441234, 2465496] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_null | 508153ns | -1638429.0ns (-76.1%) | [-1654126, -1627464]ns | [504523, 519186] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_regcache | 2259817ns | +112958.6ns (+5.2%) | [+84537, +130265]ns | [2241686, 2278138] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_threaded | 2259049ns | +103603.4ns (+4.8%) | [+87043, +118715]ns | [2237890, 2264303] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 2157035ns | -18.9% | +14.2% | -76.6% | +5.7% | +4.8% |
| 2 | 2153858ns | -18.7% | +13.1% | -76.3% | +5.3% | +4.9% |
| 3 | 2164092ns | -19.9% | +13.2% | -76.5% | +3.5% | +4.5% |
| 4 | 2139860ns | -19.1% | +14.6% | -76.3% | +5.2% | +5.9% |
| 5 | 2150206ns | -19.1% | +14.7% | -75.5% | +4.3% | +3.5% |
| 6 | 2138712ns | -19.1% | +14.4% | -76.4% | +6.4% | +5.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | 0.221 | moderate+ |
| carrier_pre_real_fntable | -0.319 | moderate- |
| carrier_pre_real_null | -0.378 | moderate- |
| carrier_pre_real_regcache | 0.038 | ok |
| carrier_pre_real_switch | -0.145 | ok |
| carrier_pre_real_threaded | -0.056 | ok |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 0/6, lost 6/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 0/6, lost 6/6
- **carrier_pre_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 1821100.2ns | 1739245.5ns | 104.7% | HIGH |
| carrier_pre_real_fntable | 2518161.8ns | 2452488.3ns | 102.7% | HIGH |
| carrier_pre_real_null | 589797.6ns | 510620.6ns | 115.5% | HIGH |
| carrier_pre_real_regcache | 2325752.4ns | 2259880.4ns | 102.9% | HIGH |
| carrier_pre_real_switch | 2296468.9ns | 2150626.9ns | 106.8% | HIGH |
| carrier_pre_real_threaded | 2319635.7ns | 2253747.4ns | 102.9% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 1731141.2-1749962.5 ns)
  1731141.2 |########################################
  1732082.3 |
  1733023.3 |####################
  1733964.4 |
  1734905.5 |
  1735846.5 |
  1736787.6 |
  1737728.7 |
  1738669.7 |
  1739610.8 |####################
  1740551.9 |
  1741492.9 |
  1742434.0 |
  1743375.0 |
  1744316.1 |
  1745257.2 |
  1746198.2 |
  1747139.3 |
  1748080.4 |
  1749021.4 |####################
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 2435480.0-2465496.5 ns)
  2435480.0 |########################################
  2436980.8 |
  2438481.6 |
  2439982.5 |
  2441483.3 |
  2442984.1 |
  2444484.9 |
  2445985.8 |########################################
  2447486.6 |########################################
  2448987.4 |
  2450488.2 |
  2451989.0 |########################################
  2453489.9 |
  2454990.7 |
  2456491.5 |
  2457992.3 |
  2459493.2 |
  2460994.0 |
  2462494.8 |
  2463995.6 |########################################
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 503942.1-519186.0 ns)
  503942.1 |########################################
  504704.3 |########################################
  505466.5 |
  506228.7 |
  506990.9 |########################################
  507753.1 |
  508515.3 |########################################
  509277.5 |
  510039.7 |########################################
  510801.9 |
  511564.1 |
  512326.3 |
  513088.5 |
  513850.7 |
  514612.9 |
  515375.1 |
  516137.3 |
  516899.5 |
  517661.7 |
  518423.9 |
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 2240485.0-2278137.9 ns)
  2240485.0 |########################################
  2242367.6 |########################################
  2244250.3 |
  2246132.9 |
  2248015.6 |
  2249898.2 |########################################
  2251780.9 |
  2253663.5 |
  2255546.2 |
  2257428.8 |
  2259311.5 |
  2261194.1 |
  2263076.7 |
  2264959.4 |
  2266842.0 |########################################
  2268724.7 |
  2270607.3 |
  2272490.0 |
  2274372.6 |########################################
  2276255.3 |
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 2138712.1-2160563.2 ns)
  2138712.1 |########################################
  2139804.7 |########################################
  2140897.2 |
  2141989.8 |
  2143082.3 |
  2144174.9 |
  2145267.4 |
  2146360.0 |
  2147452.5 |
  2148545.1 |
  2149637.6 |########################################
  2150730.2 |
  2151822.7 |
  2152915.3 |########################################
  2154007.8 |
  2155100.4 |
  2156192.9 |########################################
  2157285.5 |
  2158378.0 |
  2159470.6 |
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 2226433.3-2264302.9 ns)
  2226433.3 |########################################
  2228326.8 |
  2230220.3 |
  2232113.7 |
  2234007.2 |
  2235900.7 |
  2237794.2 |
  2239687.7 |
  2241581.1 |
  2243474.6 |
  2245368.1 |
  2247261.6 |
  2249155.1 |########################################
  2251048.5 |
  2252942.0 |
  2254835.5 |
  2256729.0 |########################################
  2258622.5 |########################################
  2260515.9 |########################################
  2262409.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=104.7% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=102.8% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=114.9% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=103.0% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: bridge=106.7% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=103.0% of algo (FFI overhead may distort results)
