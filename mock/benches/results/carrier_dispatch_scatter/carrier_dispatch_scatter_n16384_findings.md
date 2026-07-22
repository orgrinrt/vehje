# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 289% faster than the next best (carrier_disp_scatter_fntable)

carrier_disp_scatter_nullfloor (599.66 us) leads carrier_disp_scatter_fntable (2.33 ms) by 289%, a clear separation rather than a photo finish. CV 5.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 77% (significant)

carrier_disp_scatter_nullfloor is -2.01 ms (77%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_ifchainlin is an outlier: 5.4x slower than the field

carrier_disp_scatter_ifchainlin (3.24 ms) is 5.4x the fastest (599.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_scatter_nullfloor} vs {carrier_disp_scatter_fntable, carrier_disp_scatter_switch, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_ifchain, carrier_disp_scatter_bittree, carrier_disp_scatter_threaded, carrier_disp_scatter_ifchainlin} (289% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor} and a slow tier {carrier_disp_scatter_fntable, carrier_disp_scatter_switch, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_ifchain, carrier_disp_scatter_bittree, carrier_disp_scatter_threaded, carrier_disp_scatter_ifchainlin} with a 289% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.4x the fastest

Fastest carrier_disp_scatter_nullfloor (599.66 us) to slowest carrier_disp_scatter_ifchainlin (3.24 ms): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 599661.7 ns median (-76.9% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.40x (fastest 599661.7 ns, slowest 3239503.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 2677847ns | 2675203ns | 2572499ns | 2661207ns | 2755481ns | -0.09% |
| carrier_disp_scatter_fntable | 2336563ns | 2334911ns | 2328208ns | 2334392ns | 2343998ns | -12.82% |
| carrier_disp_scatter_ifchain | 2602510ns | 2602721ns | 2583100ns | 2598417ns | 2618354ns | -2.90% |
| carrier_disp_scatter_ifchainasc | 2597203ns | 2600783ns | 2552505ns | 2595527ns | 2622064ns | -3.10% |
| carrier_disp_scatter_ifchainlin | 3247348ns | 3243471ns | 3212524ns | 3235849ns | 3282007ns | +21.16% |
| carrier_disp_scatter_nullfloor | 608010ns | 603373ns | 567897ns | 597886ns | 643253ns | -77.31% |
| carrier_disp_scatter_switch | 2680224ns | 2596959ns | 2585015ns | 2593330ns | 2858172ns | base |
| carrier_disp_scatter_threaded | 2893230ns | 2903679ns | 2864180ns | 2890553ns | 2911770ns | +7.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 2674077ns | 2568666ns | 2751558ns | -0.08% | 0.006 |
| carrier_disp_scatter_fntable | 2332746ns | 2324355ns | 2340348ns | -12.84% | 0.007 |
| carrier_disp_scatter_ifchain | 2599157ns | 2580302ns | 2614268ns | -2.88% | 0.006 |
| carrier_disp_scatter_ifchainasc | 2593603ns | 2549108ns | 2618203ns | -3.09% | 0.006 |
| carrier_disp_scatter_ifchainlin | 3243558ns | 3208698ns | 3278124ns | +21.19% | 0.005 |
| carrier_disp_scatter_nullfloor | 604413ns | 564180ns | 639776ns | -77.42% | 0.027 |
| carrier_disp_scatter_switch | 2676335ns | 2581307ns | 2854139ns | base | 0.006 |
| carrier_disp_scatter_threaded | 2889401ns | 2860356ns | 2907800ns | +7.96% | 0.006 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.006 | 21.1% |
| carrier_disp_scatter_fntable | 0.007 | 24.2% |
| carrier_disp_scatter_ifchain | 0.006 | 21.7% |
| carrier_disp_scatter_ifchainasc | 0.006 | 21.7% |
| carrier_disp_scatter_ifchainlin | 0.005 | 17.4% |
| carrier_disp_scatter_nullfloor | 0.027 | 94.1% |
| carrier_disp_scatter_switch | 0.006 | 21.8% |
| carrier_disp_scatter_threaded | 0.006 | 19.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 2677847ns | 2677847ns | -0.09% |
| carrier_disp_scatter_fntable | 2336563ns | 2336563ns | -12.82% |
| carrier_disp_scatter_ifchain | 2602510ns | 2602510ns | -2.90% |
| carrier_disp_scatter_ifchainasc | 2597203ns | 2597203ns | -3.10% |
| carrier_disp_scatter_ifchainlin | 3247348ns | 3247348ns | +21.16% |
| carrier_disp_scatter_nullfloor | 608010ns | 608010ns | -77.31% |
| carrier_disp_scatter_switch | 2680224ns | 2680224ns | base |
| carrier_disp_scatter_threaded | 2893230ns | 2893230ns | +7.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 2593118ns | base | --- | [2581748, 2854139] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 2671644ns | no significant difference | [-247343, +169810]ns | [2599030, 2751558] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_fntable | 2330880ns | -265457.7ns (-10.2%) | [-513791, -251518]ns | [2327010, 2340348] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_scatter_ifchain | 2599751ns | no significant difference | [-263643, +21676]ns | [2583452, 2614268] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainasc | 2597357ns | no significant difference | [-263377, +11183]ns | [2565249, 2618203] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainlin | 3239503ns | +628442.5ns (+24.2%) | [+400403, +672822]ns | [3213046, 3278124] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_scatter_nullfloor | 599662ns | -2007359.0ns (-77.4%) | [-2230786, -1977620]ns | [573802, 639776] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 2900003ns | +293118.9ns (+11.3%) | [+26999, +319081]ns | [2860400, 2907800] | YES (adj: no) | 0.3828 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2586981ns | +2.6% | -10.2% | +1.0% | -1.5% | +26.2% | -78.2% | +12.4% |
| 2 | 2581307ns | +8.7% | -9.7% | +0.2% | +0.0% | +25.9% | -77.4% | +12.3% |
| 3 | 3093491ns | -15.0% | -24.2% | -16.6% | -15.8% | +6.4% | -78.8% | -6.2% |
| 4 | 2614787ns | +2.8% | -10.6% | -0.5% | +0.3% | +23.0% | -76.4% | +9.4% |
| 5 | 2599255ns | -1.2% | -10.3% | +0.6% | +0.5% | +24.3% | -77.6% | +11.8% |
| 6 | 2582190ns | +4.4% | -9.7% | +0.6% | +0.3% | +24.3% | -75.8% | +10.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | -0.411 | moderate- |
| carrier_disp_scatter_fntable | 0.150 | ok |
| carrier_disp_scatter_ifchain | 0.045 | ok |
| carrier_disp_scatter_ifchainasc | 0.354 | moderate+ |
| carrier_disp_scatter_ifchainlin | -0.002 | ok |
| carrier_disp_scatter_nullfloor | -0.054 | ok |
| carrier_disp_scatter_switch | -0.214 | moderate- |
| carrier_disp_scatter_threaded | -0.434 | moderate- |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 2/6, lost 4/6
- **carrier_disp_scatter_fntable**: won 6/6, lost 0/6
- **carrier_disp_scatter_ifchain**: won 2/6, lost 4/6
- **carrier_disp_scatter_ifchainasc**: won 2/6, lost 3/6
- **carrier_disp_scatter_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 2671740.5ns | 2674077.2ns | 99.9% | HIGH |
| carrier_disp_scatter_fntable | 2323265.4ns | 2332746.3ns | 99.6% | HIGH |
| carrier_disp_scatter_ifchain | 2601009.4ns | 2599157.2ns | 100.1% | HIGH |
| carrier_disp_scatter_ifchainasc | 2596435.6ns | 2593603.2ns | 100.1% | HIGH |
| carrier_disp_scatter_ifchainlin | 3248500.1ns | 3243557.6ns | 100.2% | HIGH |
| carrier_disp_scatter_nullfloor | 605689.8ns | 604413.2ns | 100.2% | HIGH |
| carrier_disp_scatter_switch | 2675031.6ns | 2676335.0ns | 100.0% | HIGH |
| carrier_disp_scatter_threaded | 2888967.5ns | 2889401.2ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 2568665.8-2751558.1 ns)
  2568665.8 |####################
  2577810.4 |
  2586955.0 |
  2596099.6 |
  2605244.3 |
  2614388.9 |
  2623533.5 |####################
  2632678.1 |
  2641822.7 |
  2650967.3 |####################
  2660111.9 |
  2669256.6 |
  2678401.2 |
  2687545.8 |########################################
  2696690.4 |
  2705835.0 |
  2714979.6 |
  2724124.3 |
  2733268.9 |
  2742413.5 |
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 2324354.6-2340348.2 ns)
  2324354.6 |####################
  2325154.3 |
  2325954.0 |
  2326753.6 |
  2327553.3 |
  2328353.0 |
  2329152.7 |####################
  2329952.3 |
  2330752.0 |########################################
  2331551.7 |
  2332351.4 |
  2333151.1 |
  2333950.7 |
  2334750.4 |
  2335550.1 |
  2336349.8 |####################
  2337149.4 |
  2337949.1 |
  2338748.8 |
  2339548.5 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 2580301.7-2614268.2 ns)
  2580301.7 |########################################
  2582000.0 |
  2583698.3 |
  2585396.7 |########################################
  2587095.0 |
  2588793.3 |
  2590491.6 |
  2592190.0 |
  2593888.3 |
  2595586.6 |
  2597284.9 |########################################
  2598983.2 |
  2600681.6 |########################################
  2602379.9 |
  2604078.2 |
  2605776.5 |
  2607474.9 |
  2609173.2 |
  2610871.5 |
  2612569.8 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 2549108.3-2618203.3 ns)
  2549108.3 |########################################
  2552563.0 |
  2556017.8 |
  2559472.5 |
  2562927.3 |
  2566382.0 |
  2569836.8 |
  2573291.5 |
  2576746.3 |
  2580201.0 |########################################
  2583655.8 |
  2587110.5 |########################################
  2590565.3 |
  2594020.0 |
  2597474.8 |
  2600929.5 |
  2604384.3 |########################################
  2607839.0 |
  2611293.8 |########################################
  2614748.5 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 3208697.5-3278124.0 ns)
  3208697.5 |########################################
  3212168.8 |
  3215640.1 |########################################
  3219111.5 |
  3222582.8 |
  3226054.1 |
  3229525.4 |########################################
  3232996.8 |
  3236468.1 |
  3239939.4 |
  3243410.7 |
  3246882.0 |########################################
  3250353.4 |
  3253824.7 |
  3257296.0 |
  3260767.3 |
  3264238.7 |########################################
  3267710.0 |
  3271181.3 |
  3274652.6 |
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 564180.0-639776.1 ns)
  564180.0 |####################
  567959.8 |
  571739.6 |
  575519.4 |
  579299.2 |
  583079.0 |########################################
  586858.8 |
  590638.6 |
  594418.4 |
  598198.2 |
  601978.0 |
  605757.8 |
  609537.6 |
  613317.4 |####################
  617097.2 |
  620877.0 |
  624656.8 |####################
  628436.6 |
  632216.4 |
  635996.2 |
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 2581306.7-2854138.8 ns)
  2581306.7 |########################################
  2594948.3 |#############
  2608589.9 |#############
  2622231.5 |
  2635873.1 |
  2649514.7 |
  2663156.3 |
  2676797.9 |
  2690439.5 |
  2704081.1 |
  2717722.7 |
  2731364.3 |
  2745005.9 |
  2758647.5 |
  2772289.1 |
  2785930.7 |
  2799572.3 |
  2813213.9 |
  2826855.5 |
  2840497.1 |
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 2860355.8-2907800.0 ns)
  2860355.8 |########################################
  2862728.0 |
  2865100.2 |
  2867472.4 |
  2869844.6 |
  2872216.8 |
  2874589.1 |
  2876961.3 |
  2879333.5 |
  2881705.7 |
  2884077.9 |
  2886450.1 |
  2888822.3 |
  2891194.5 |
  2893566.7 |
  2895939.0 |####################
  2898311.2 |
  2900683.4 |####################
  2903055.6 |
  2905427.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: bridge=99.6% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=100.0% of algo (FFI overhead may distort results)
