# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 354% faster than the next best (carrier_disp_real_fntable)

carrier_disp_real_nullfloor (517.85 us) leads carrier_disp_real_fntable (2.35 ms) by 354%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 80% (significant)

carrier_disp_real_nullfloor is -2.11 ms (80%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_ifchainlin is an outlier: 6.2x slower than the field

carrier_disp_real_ifchainlin (3.22 ms) is 6.2x the fastest (517.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_nullfloor is fastest but the noisiest (CV 5.8%)

carrier_disp_real_nullfloor wins on median (517.85 us) yet has the highest variance (CV 5.8%), while carrier_disp_real_fntable is the steadiest (CV 0.2%, 2.35 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_disp_real_fntable shows alternating (throttle bounce) (autocorr -0.52)

carrier_disp_real_fntable's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_real_nullfloor} vs {carrier_disp_real_fntable, carrier_disp_real_bittree, carrier_disp_real_ifchain, carrier_disp_real_ifchainasc, carrier_disp_real_switch, carrier_disp_real_threaded, carrier_disp_real_ifchainlin} (354% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor} and a slow tier {carrier_disp_real_fntable, carrier_disp_real_bittree, carrier_disp_real_ifchain, carrier_disp_real_ifchainasc, carrier_disp_real_switch, carrier_disp_real_threaded, carrier_disp_real_ifchainlin} with a 354% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.2x the fastest

Fastest carrier_disp_real_nullfloor (517.85 us) to slowest carrier_disp_real_ifchainlin (3.22 ms): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 517854.2 ns median (-80.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 6.22x (fastest 517854.2 ns, slowest 3221318.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 2610102ns | 2604621ns | 2527638ns | 2585263ns | 2688592ns | -0.70% |
| carrier_disp_real_fntable | 2355402ns | 2354657ns | 2345958ns | 2354202ns | 2361925ns | -10.39% |
| carrier_disp_real_ifchain | 2619818ns | 2613564ns | 2604528ns | 2612593ns | 2638301ns | -0.33% |
| carrier_disp_real_ifchainasc | 2623663ns | 2623421ns | 2604131ns | 2618856ns | 2640641ns | -0.18% |
| carrier_disp_real_ifchainlin | 3210840ns | 3224697ns | 3171368ns | 3208893ns | 3233496ns | +22.15% |
| carrier_disp_real_nullfloor | 525388ns | 520485ns | 489740ns | 513258ns | 561406ns | -80.01% |
| carrier_disp_real_switch | 2628499ns | 2626565ns | 2620147ns | 2625343ns | 2637409ns | base |
| carrier_disp_real_threaded | 2921351ns | 2926638ns | 2878465ns | 2924453ns | 2938142ns | +11.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 2606749ns | 2524357ns | 2685138ns | -0.70% | 0.006 |
| carrier_disp_real_fntable | 2352069ns | 2342511ns | 2358529ns | -10.40% | 0.007 |
| carrier_disp_real_ifchain | 2616155ns | 2600342ns | 2635032ns | -0.34% | 0.006 |
| carrier_disp_real_ifchainasc | 2620188ns | 2600309ns | 2637533ns | -0.19% | 0.006 |
| carrier_disp_real_ifchainlin | 3207423ns | 3167556ns | 3230288ns | +22.18% | 0.005 |
| carrier_disp_real_nullfloor | 522405ns | 487447ns | 557953ns | -80.10% | 0.031 |
| carrier_disp_real_switch | 2625104ns | 2616701ns | 2634199ns | base | 0.006 |
| carrier_disp_real_threaded | 2917873ns | 2875245ns | 2934526ns | +11.15% | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_real_bittree | 16249969 | 19380100 | 0.838 | 0.99× |
| carrier_disp_real_fntable | 14650662 | 25968491 | 0.564 | 0.89× |
| carrier_disp_real_ifchain | 16380254 | 18784824 | 0.872 | 1.00× |
| carrier_disp_real_ifchainasc | 16376216 | 18782018 | 0.872 | 1.00× |
| carrier_disp_real_ifchainlin | 20045976 | 50992253 | 0.393 | 1.22× |
| carrier_disp_real_nullfloor | 3273720 | 16153528 | 0.203 | 0.20× |
| carrier_disp_real_switch | 16388258 | 18289126 | 0.896 | 1.00× |
| carrier_disp_real_threaded | 18260916 | 25383758 | 0.719 | 1.11× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.006 | 18.7% |
| carrier_disp_real_fntable | 0.007 | 20.7% |
| carrier_disp_real_ifchain | 0.006 | 18.7% |
| carrier_disp_real_ifchainasc | 0.006 | 18.6% |
| carrier_disp_real_ifchainlin | 0.005 | 15.1% |
| carrier_disp_real_nullfloor | 0.032 | 94.1% |
| carrier_disp_real_switch | 0.006 | 18.6% |
| carrier_disp_real_threaded | 0.006 | 16.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 2610102ns | 2610102ns | -0.70% |
| carrier_disp_real_fntable | 2355402ns | 2355402ns | -10.39% |
| carrier_disp_real_ifchain | 2619818ns | 2619818ns | -0.33% |
| carrier_disp_real_ifchainasc | 2623663ns | 2623663ns | -0.18% |
| carrier_disp_real_ifchainlin | 3210840ns | 3210840ns | +22.15% |
| carrier_disp_real_nullfloor | 525388ns | 525388ns | -80.01% |
| carrier_disp_real_switch | 2628499ns | 2628499ns | base |
| carrier_disp_real_threaded | 2921351ns | 2921351ns | +11.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 2622948ns | base | --- | [2618164, 2634199] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 2601153ns | no significant difference | [-84497, +60910]ns | [2533957, 2685138] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_real_fntable | 2351442ns | -269152.9ns (-10.3%) | [-283799, -266153]ns | [2346236, 2358529] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_real_ifchain | 2609905ns | no significant difference | [-23334, +9336]ns | [2603529, 2635032] | no | 0.3063 | 0.2188 | 0 |
| carrier_disp_real_ifchainasc | 2619925ns | no significant difference | [-19840, +7934]ns | [2603108, 2637533] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchainlin | 3221318ns | +599495.8ns (+22.9%) | [+542526, +604936]ns | [3170662, 3230288] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_real_nullfloor | 517854ns | -2109403.3ns (-80.4%) | [-2138192, -2060501]ns | [491407, 557953] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_real_threaded | 2923066ns | +299886.9ns (+11.4%) | [+273081, +305342]ns | [2896028, 2934526] | YES (adj: no) | 0.0547 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2620208ns | -3.7% | -10.3% | -0.5% | -0.8% | +22.8% | -79.4% | +9.7% |
| 2 | 2625688ns | -2.2% | -10.3% | +1.1% | -0.8% | +23.0% | -79.8% | +11.1% |
| 3 | 2619628ns | +1.7% | -10.2% | -0.7% | +0.3% | +23.1% | -81.4% | +11.5% |
| 4 | 2639571ns | -0.2% | -11.3% | -1.0% | +0.3% | +20.2% | -81.2% | +11.3% |
| 5 | 2628827ns | +3.0% | -10.2% | -0.5% | -0.4% | +22.9% | -80.8% | +11.5% |
| 6 | 2616701ns | -2.8% | -10.1% | -0.3% | +0.2% | +21.1% | -78.0% | +11.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | -0.034 | ok |
| carrier_disp_real_fntable | -0.516 | HIGH- (thermal bounce) |
| carrier_disp_real_ifchain | -0.487 | moderate- |
| carrier_disp_real_ifchainasc | 0.190 | ok |
| carrier_disp_real_ifchainlin | -0.405 | moderate- |
| carrier_disp_real_nullfloor | 0.070 | ok |
| carrier_disp_real_switch | -0.180 | ok |
| carrier_disp_real_threaded | 0.188 | ok |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 4/6, lost 2/6
- **carrier_disp_real_fntable**: won 6/6, lost 0/6
- **carrier_disp_real_ifchain**: won 5/6, lost 1/6
- **carrier_disp_real_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_real_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 2607064.0ns | 2606749.4ns | 100.0% | HIGH |
| carrier_disp_real_fntable | 2342581.6ns | 2352068.9ns | 99.6% | HIGH |
| carrier_disp_real_ifchain | 2618719.3ns | 2616155.4ns | 100.1% | HIGH |
| carrier_disp_real_ifchainasc | 2621410.4ns | 2620188.5ns | 100.0% | HIGH |
| carrier_disp_real_ifchainlin | 3210198.7ns | 3207422.9ns | 100.1% | HIGH |
| carrier_disp_real_nullfloor | 523524.2ns | 522404.7ns | 100.2% | HIGH |
| carrier_disp_real_switch | 2625278.1ns | 2625103.6ns | 100.0% | HIGH |
| carrier_disp_real_threaded | 2919867.4ns | 2917873.4ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 2524356.7-2685137.7 ns)
  2524356.7 |########################################
  2532395.8 |
  2540434.8 |########################################
  2548473.9 |
  2556512.9 |
  2564552.0 |########################################
  2572591.0 |
  2580630.1 |
  2588669.1 |
  2596708.2 |
  2604747.2 |
  2612786.2 |
  2620825.3 |
  2628864.4 |########################################
  2636903.4 |
  2644942.5 |
  2652981.5 |
  2661020.6 |########################################
  2669059.6 |
  2677098.7 |
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 2342511.2-2358529.0 ns)
  2342511.2 |########################################
  2343312.1 |
  2344113.0 |
  2344913.9 |
  2345714.8 |
  2346515.6 |
  2347316.5 |
  2348117.4 |
  2348918.3 |
  2349719.2 |########################################
  2350520.1 |########################################
  2351321.0 |########################################
  2352121.9 |
  2352922.7 |
  2353723.6 |
  2354524.5 |########################################
  2355325.4 |
  2356126.3 |
  2356927.2 |
  2357728.1 |
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 2600341.7-2635032.5 ns)
  2600341.7 |########################################
  2602076.2 |
  2603810.8 |
  2605545.3 |########################################
  2607279.9 |########################################
  2609014.4 |
  2610748.9 |########################################
  2612483.5 |
  2614218.0 |
  2615952.6 |########################################
  2617687.1 |
  2619421.6 |
  2621156.2 |
  2622890.7 |
  2624625.3 |
  2626359.8 |
  2628094.3 |
  2629828.9 |
  2631563.4 |
  2633298.0 |
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 2600309.2-2637532.7 ns)
  2600309.2 |########################################
  2602170.4 |
  2604031.6 |
  2605892.7 |########################################
  2607753.9 |
  2609615.1 |
  2611476.2 |
  2613337.4 |
  2615198.6 |
  2617059.8 |########################################
  2618921.0 |
  2620782.1 |
  2622643.3 |########################################
  2624504.5 |
  2626365.7 |########################################
  2628226.8 |
  2630088.0 |
  2631949.2 |
  2633810.4 |
  2635671.5 |
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 3167555.8-3230288.5 ns)
  3167555.8 |########################################
  3170692.4 |########################################
  3173829.1 |
  3176965.7 |
  3180102.3 |
  3183239.0 |
  3186375.6 |
  3189512.2 |
  3192648.9 |
  3195785.5 |
  3198922.1 |
  3202058.8 |
  3205195.4 |
  3208332.1 |
  3211468.7 |
  3214605.3 |########################################
  3217742.0 |
  3220878.6 |
  3224015.2 |########################################
  3227151.9 |########################################
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 487446.7-557952.9 ns)
  487446.7 |########################################
  490972.0 |
  494497.3 |########################################
  498022.6 |
  501547.9 |
  505073.2 |########################################
  508598.6 |
  512123.9 |
  515649.2 |
  519174.5 |
  522699.8 |
  526225.1 |
  529750.4 |########################################
  533275.7 |
  536801.0 |
  540326.3 |########################################
  543851.7 |
  547377.0 |
  550902.3 |
  554427.6 |
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 2616700.8-2634199.0 ns)
  2616700.8 |########################################
  2617575.7 |
  2618450.6 |
  2619325.5 |########################################
  2620200.4 |########################################
  2621075.3 |
  2621950.2 |
  2622825.2 |
  2623700.1 |
  2624575.0 |
  2625449.9 |########################################
  2626324.8 |
  2627199.7 |
  2628074.6 |########################################
  2628949.5 |
  2629824.4 |
  2630699.3 |
  2631574.2 |
  2632449.1 |
  2633324.0 |
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 2875245.4-2934526.0 ns)
  2875245.4 |########################################
  2878209.4 |
  2881173.5 |
  2884137.5 |
  2887101.5 |
  2890065.6 |
  2893029.6 |
  2895993.6 |
  2898957.7 |
  2901921.7 |
  2904885.7 |
  2907849.8 |
  2910813.8 |
  2913777.8 |
  2916741.9 |########################################
  2919705.9 |########################################
  2922669.9 |########################################
  2925634.0 |
  2928598.0 |########################################
  2931562.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=99.6% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=100.1% of algo (FFI overhead may distort results)
