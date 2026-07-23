# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 305% faster than the next best (carrier_disp_scatter_fntable)

carrier_disp_scatter_nullfloor (582.10 us) leads carrier_disp_scatter_fntable (2.36 ms) by 305%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 78% (significant)

carrier_disp_scatter_nullfloor is -2.04 ms (78%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_ifchainlin is an outlier: 5.6x slower than the field

carrier_disp_scatter_ifchainlin (3.26 ms) is 5.6x the fastest (582.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_scatter_nullfloor is fastest but the noisiest (CV 6.3%)

carrier_disp_scatter_nullfloor wins on median (582.10 us) yet has the highest variance (CV 6.3%), while carrier_disp_scatter_threaded is the steadiest (CV 0.3%, 2.94 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_disp_scatter_fntable shows alternating (throttle bounce) (autocorr -0.72)

carrier_disp_scatter_fntable's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_scatter_nullfloor} vs {carrier_disp_scatter_fntable, carrier_disp_scatter_bittree, carrier_disp_scatter_ifchain, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_switch, carrier_disp_scatter_threaded, carrier_disp_scatter_ifchainlin} (305% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor} and a slow tier {carrier_disp_scatter_fntable, carrier_disp_scatter_bittree, carrier_disp_scatter_ifchain, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_switch, carrier_disp_scatter_threaded, carrier_disp_scatter_ifchainlin} with a 305% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.6x the fastest

Fastest carrier_disp_scatter_nullfloor (582.10 us) to slowest carrier_disp_scatter_ifchainlin (3.26 ms): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 582098.9 ns median (-77.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.60x (fastest 582098.9 ns, slowest 3258829.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 2577545ns | 2582851ns | 2470124ns | 2574284ns | 2636146ns | -1.70% |
| carrier_disp_scatter_fntable | 2358315ns | 2359582ns | 2345416ns | 2354994ns | 2369746ns | -10.06% |
| carrier_disp_scatter_ifchain | 2609896ns | 2605086ns | 2590322ns | 2603041ns | 2629964ns | -0.47% |
| carrier_disp_scatter_ifchainasc | 2615375ns | 2614741ns | 2599358ns | 2610152ns | 2631219ns | -0.26% |
| carrier_disp_scatter_ifchainlin | 3269787ns | 3262106ns | 3249214ns | 3259664ns | 3295258ns | +24.70% |
| carrier_disp_scatter_nullfloor | 584647ns | 585387ns | 525571ns | 577785ns | 624477ns | -77.70% |
| carrier_disp_scatter_switch | 2622206ns | 2620743ns | 2609581ns | 2618749ns | 2633705ns | base |
| carrier_disp_scatter_threaded | 2941517ns | 2941121ns | 2928706ns | 2937878ns | 2953381ns | +12.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 2574015ns | 2466150ns | 2632756ns | -1.71% | 0.006 |
| carrier_disp_scatter_fntable | 2354936ns | 2341548ns | 2366656ns | -10.08% | 0.007 |
| carrier_disp_scatter_ifchain | 2606455ns | 2586359ns | 2626582ns | -0.47% | 0.006 |
| carrier_disp_scatter_ifchainasc | 2611978ns | 2596287ns | 2628145ns | -0.26% | 0.006 |
| carrier_disp_scatter_ifchainlin | 3266358ns | 3245915ns | 3291758ns | +24.73% | 0.005 |
| carrier_disp_scatter_nullfloor | 581419ns | 522981ns | 621059ns | -77.80% | 0.028 |
| carrier_disp_scatter_switch | 2618833ns | 2605656ns | 2630589ns | base | 0.006 |
| carrier_disp_scatter_threaded | 2938100ns | 2925546ns | 2949972ns | +12.19% | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 16040036 | 19842263 | 0.808 | 0.98× |
| carrier_disp_scatter_fntable | 14687784 | 26723852 | 0.550 | 0.90× |
| carrier_disp_scatter_ifchain | 16315050 | 19283834 | 0.846 | 1.00× |
| carrier_disp_scatter_ifchainasc | 16269658 | 19285060 | 0.844 | 1.00× |
| carrier_disp_scatter_ifchainlin | 20395074 | 52764454 | 0.387 | 1.25× |
| carrier_disp_scatter_nullfloor | 3657373 | 16563244 | 0.221 | 0.22× |
| carrier_disp_scatter_switch | 16349820 | 18759599 | 0.872 | 1.00× |
| carrier_disp_scatter_threaded | 18349114 | 26067797 | 0.704 | 1.12× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.006 | 20.3% |
| carrier_disp_scatter_fntable | 0.007 | 22.2% |
| carrier_disp_scatter_ifchain | 0.006 | 20.1% |
| carrier_disp_scatter_ifchainasc | 0.006 | 20.0% |
| carrier_disp_scatter_ifchainlin | 0.005 | 16.0% |
| carrier_disp_scatter_nullfloor | 0.028 | 89.8% |
| carrier_disp_scatter_switch | 0.006 | 20.0% |
| carrier_disp_scatter_threaded | 0.006 | 17.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 2577545ns | 2577545ns | -1.70% |
| carrier_disp_scatter_fntable | 2358315ns | 2358315ns | -10.06% |
| carrier_disp_scatter_ifchain | 2609896ns | 2609896ns | -0.47% |
| carrier_disp_scatter_ifchainasc | 2615375ns | 2615375ns | -0.26% |
| carrier_disp_scatter_ifchainlin | 3269787ns | 3269787ns | +24.70% |
| carrier_disp_scatter_nullfloor | 584647ns | 584647ns | -77.70% |
| carrier_disp_scatter_switch | 2622206ns | 2622206ns | base |
| carrier_disp_scatter_threaded | 2941517ns | 2941517ns | +12.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 2617536ns | base | --- | [2608374, 2630589] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 2579271ns | no significant difference | [-117975, +13961]ns | [2510018, 2632756] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_fntable | 2356212ns | -263547.5ns (-10.1%) | [-272032, -256111]ns | [2341942, 2366656] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_scatter_ifchain | 2601756ns | no significant difference | [-33197, +10384]ns | [2591026, 2626582] | no | 0.3063 | 0.2188 | 0 |
| carrier_disp_scatter_ifchainasc | 2610867ns | no significant difference | [-23210, +11946]ns | [2596923, 2628145] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainlin | 3258830ns | +645341.9ns (+24.7%) | [+629375, +667857]ns | [3248486, 3291758] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_scatter_nullfloor | 582099ns | -2038069.4ns (-77.9%) | [-2076436, -1997736]ns | [541099, 621059] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 2937644ns | +314854.6ns (+12.0%) | [+307726, +335219]ns | [2926684, 2949972] | YES (adj: no) | 0.0547 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2611092ns | +1.0% | -10.3% | +0.8% | +0.6% | +24.7% | -76.9% | +13.2% |
| 2 | 2621304ns | -2.6% | -9.6% | -0.0% | +0.3% | +25.1% | -78.7% | +11.7% |
| 3 | 2605656ns | -1.3% | -10.1% | -0.2% | -0.3% | +24.8% | -78.2% | +12.3% |
| 4 | 2634680ns | -6.4% | -10.4% | -1.5% | -0.4% | +23.8% | -77.4% | +11.8% |
| 5 | 2613768ns | -1.1% | -10.0% | -1.0% | -0.7% | +24.2% | -80.0% | +12.5% |
| 6 | 2626498ns | +0.1% | -10.0% | -0.9% | -1.1% | +25.8% | -75.6% | +11.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | -0.091 | ok |
| carrier_disp_scatter_fntable | -0.717 | HIGH- (thermal bounce) |
| carrier_disp_scatter_ifchain | 0.419 | moderate+ |
| carrier_disp_scatter_ifchainasc | -0.090 | ok |
| carrier_disp_scatter_ifchainlin | -0.389 | moderate- |
| carrier_disp_scatter_nullfloor | -0.570 | HIGH- (thermal bounce) |
| carrier_disp_scatter_switch | -0.660 | HIGH- (thermal bounce) |
| carrier_disp_scatter_threaded | -0.200 | ok |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 4/6, lost 1/6
- **carrier_disp_scatter_fntable**: won 6/6, lost 0/6
- **carrier_disp_scatter_ifchain**: won 4/6, lost 1/6
- **carrier_disp_scatter_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_scatter_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 2568392.9ns | 2574014.7ns | 99.8% | HIGH |
| carrier_disp_scatter_fntable | 2346072.2ns | 2354936.3ns | 99.6% | HIGH |
| carrier_disp_scatter_ifchain | 2607913.2ns | 2606454.8ns | 100.1% | HIGH |
| carrier_disp_scatter_ifchainasc | 2616783.5ns | 2611978.2ns | 100.2% | HIGH |
| carrier_disp_scatter_ifchainlin | 3270305.8ns | 3266357.7ns | 100.1% | HIGH |
| carrier_disp_scatter_nullfloor | 582699.9ns | 581419.0ns | 100.2% | HIGH |
| carrier_disp_scatter_switch | 2621985.8ns | 2618833.1ns | 100.1% | HIGH |
| carrier_disp_scatter_threaded | 2940780.8ns | 2938099.7ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 2466150.4-2632756.0 ns)
  2466150.4 |########################################
  2474480.7 |
  2482811.0 |
  2491141.2 |
  2499471.5 |
  2507801.8 |
  2516132.1 |
  2524462.4 |
  2532792.7 |
  2541122.9 |
  2549453.2 |########################################
  2557783.5 |
  2566113.8 |########################################
  2574444.1 |
  2582774.4 |########################################
  2591104.6 |
  2599434.9 |
  2607765.2 |
  2616095.5 |
  2624425.8 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 2341548.3-2366655.8 ns)
  2341548.3 |########################################
  2342803.7 |
  2344059.0 |
  2345314.4 |
  2346569.8 |
  2347825.2 |
  2349080.5 |
  2350335.9 |
  2351591.3 |
  2352846.7 |####################
  2354102.0 |
  2355357.4 |
  2356612.8 |
  2357868.2 |
  2359123.5 |####################
  2360378.9 |
  2361634.3 |
  2362889.7 |####################
  2364145.0 |
  2365400.4 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 2586358.8-2626581.7 ns)
  2586358.8 |########################################
  2588369.9 |
  2590381.1 |
  2592392.2 |
  2594403.4 |########################################
  2596414.5 |
  2598425.7 |########################################
  2600436.8 |
  2602448.0 |########################################
  2604459.1 |
  2606470.2 |
  2608481.4 |
  2610492.5 |
  2612503.7 |
  2614514.8 |
  2616526.0 |
  2618537.1 |########################################
  2620548.3 |
  2622559.4 |
  2624570.6 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 2596287.1-2628144.5 ns)
  2596287.1 |########################################
  2597880.0 |
  2599472.8 |
  2601065.7 |
  2602658.6 |
  2604251.5 |
  2605844.3 |
  2607437.2 |
  2609030.1 |
  2610623.0 |
  2612215.8 |
  2613808.7 |
  2615401.6 |
  2616994.4 |
  2618587.3 |
  2620180.2 |
  2621773.1 |
  2623365.9 |#############
  2624958.8 |
  2626551.7 |#############
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 3245915.4-3291757.7 ns)
  3245915.4 |########################################
  3248207.5 |
  3250499.6 |########################################
  3252791.7 |
  3255083.9 |########################################
  3257376.0 |
  3259668.1 |########################################
  3261960.2 |
  3264252.3 |
  3266544.4 |
  3268836.5 |
  3271128.7 |
  3273420.8 |
  3275712.9 |
  3278005.0 |########################################
  3280297.1 |
  3282589.2 |
  3284881.4 |
  3287173.5 |
  3289465.6 |
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 522980.8-621058.9 ns)
  522980.8 |########################################
  527884.7 |
  532788.6 |
  537692.5 |
  542596.4 |
  547500.3 |
  552404.2 |
  557308.2 |########################################
  562212.1 |
  567116.0 |########################################
  572019.9 |
  576923.8 |
  581827.7 |
  586731.6 |
  591635.5 |########################################
  596539.4 |
  601443.3 |########################################
  606347.2 |
  611251.1 |
  616155.0 |
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 2605656.2-2630589.3 ns)
  2605656.2 |########################################
  2606902.9 |
  2608149.5 |
  2609396.2 |
  2610642.8 |########################################
  2611889.5 |
  2613136.1 |########################################
  2614382.8 |
  2615629.5 |
  2616876.1 |
  2618122.8 |
  2619369.4 |
  2620616.1 |########################################
  2621862.7 |
  2623109.4 |
  2624356.1 |
  2625602.7 |########################################
  2626849.4 |
  2628096.0 |
  2629342.7 |
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 2925545.8-2949971.6 ns)
  2925545.8 |########################################
  2926767.1 |########################################
  2927988.4 |
  2929209.7 |
  2930431.0 |
  2931652.3 |
  2932873.6 |
  2934094.8 |
  2935316.1 |########################################
  2936537.4 |
  2937758.7 |
  2938980.0 |########################################
  2940201.3 |
  2941422.6 |
  2942643.9 |
  2943865.2 |########################################
  2945086.5 |
  2946307.8 |
  2947529.1 |
  2948750.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: bridge=99.7% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=100.1% of algo (FFI overhead may distort results)
