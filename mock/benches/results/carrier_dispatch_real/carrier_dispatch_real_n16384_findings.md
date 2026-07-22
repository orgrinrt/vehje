# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 319% faster than the next best (carrier_disp_real_fntable)

carrier_disp_real_nullfloor (560.59 us) leads carrier_disp_real_fntable (2.35 ms) by 319%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 78% (significant)

carrier_disp_real_nullfloor is -2.05 ms (78%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_ifchainlin is an outlier: 5.8x slower than the field

carrier_disp_real_ifchainlin (3.27 ms) is 5.8x the fastest (560.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_nullfloor is fastest but the noisiest (CV 6.3%)

carrier_disp_real_nullfloor wins on median (560.59 us) yet has the highest variance (CV 6.3%), while carrier_disp_real_switch is the steadiest (CV 0.2%, 2.61 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {carrier_disp_real_nullfloor} vs {carrier_disp_real_fntable, carrier_disp_real_ifchain, carrier_disp_real_switch, carrier_disp_real_ifchainasc, carrier_disp_real_bittree, carrier_disp_real_threaded, carrier_disp_real_ifchainlin} (319% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor} and a slow tier {carrier_disp_real_fntable, carrier_disp_real_ifchain, carrier_disp_real_switch, carrier_disp_real_ifchainasc, carrier_disp_real_bittree, carrier_disp_real_threaded, carrier_disp_real_ifchainlin} with a 319% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.8x the fastest

Fastest carrier_disp_real_nullfloor (560.59 us) to slowest carrier_disp_real_ifchainlin (3.27 ms): 5.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 560587.3 ns median (-78.5% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.83x (fastest 560587.3 ns, slowest 3266609.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 2673623ns | 2692603ns | 2562040ns | 2663628ns | 2744405ns | +2.19% |
| carrier_disp_real_fntable | 2349615ns | 2350629ns | 2339134ns | 2349107ns | 2355619ns | -10.20% |
| carrier_disp_real_ifchain | 2606973ns | 2612315ns | 2581328ns | 2605152ns | 2622526ns | -0.36% |
| carrier_disp_real_ifchainasc | 2617271ns | 2618216ns | 2607975ns | 2616126ns | 2623635ns | +0.03% |
| carrier_disp_real_ifchainlin | 3286966ns | 3270561ns | 3217274ns | 3259286ns | 3363332ns | +25.63% |
| carrier_disp_real_nullfloor | 581977ns | 564241ns | 552870ns | 562193ns | 626205ns | -77.76% |
| carrier_disp_real_switch | 2616404ns | 2617223ns | 2603993ns | 2616710ns | 2622150ns | base |
| carrier_disp_real_threaded | 2922918ns | 2924877ns | 2912432ns | 2921445ns | 2930370ns | +11.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 2669660ns | 2557765ns | 2740648ns | +2.19% | 0.006 |
| carrier_disp_real_fntable | 2345772ns | 2335399ns | 2351762ns | -10.21% | 0.007 |
| carrier_disp_real_ifchain | 2603046ns | 2577486ns | 2618313ns | -0.36% | 0.006 |
| carrier_disp_real_ifchainasc | 2613243ns | 2603755ns | 2619506ns | +0.03% | 0.006 |
| carrier_disp_real_ifchainlin | 3283024ns | 3213188ns | 3359669ns | +25.67% | 0.005 |
| carrier_disp_real_nullfloor | 578395ns | 549385ns | 622529ns | -77.86% | 0.028 |
| carrier_disp_real_switch | 2612515ns | 2600254ns | 2618328ns | base | 0.006 |
| carrier_disp_real_threaded | 2918968ns | 2908712ns | 2926546ns | +11.73% | 0.006 |

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.006 | 20.4% |
| carrier_disp_real_fntable | 0.007 | 23.4% |
| carrier_disp_real_ifchain | 0.006 | 21.1% |
| carrier_disp_real_ifchainasc | 0.006 | 21.0% |
| carrier_disp_real_ifchainlin | 0.005 | 16.8% |
| carrier_disp_real_nullfloor | 0.029 | 98.0% |
| carrier_disp_real_switch | 0.006 | 21.0% |
| carrier_disp_real_threaded | 0.006 | 18.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 2673623ns | 2673623ns | +2.19% |
| carrier_disp_real_fntable | 2349615ns | 2349615ns | -10.20% |
| carrier_disp_real_ifchain | 2606973ns | 2606973ns | -0.36% |
| carrier_disp_real_ifchainasc | 2617271ns | 2617271ns | +0.03% |
| carrier_disp_real_ifchainlin | 3286966ns | 3286966ns | +25.63% |
| carrier_disp_real_nullfloor | 581977ns | 581977ns | -77.76% |
| carrier_disp_real_switch | 2616404ns | 2616404ns | base |
| carrier_disp_real_threaded | 2922918ns | 2922918ns | +11.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 2613287ns | base | --- | [2605931, 2618328] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 2688695ns | no significant difference | [-32461, +125372]ns | [2579639, 2740648] | no | 0.9625 | 0.6875 | 0 |
| carrier_disp_real_fntable | 2346760ns | -269087.5ns (-10.3%) | [-274494, -256650]ns | [2338794, 2351762] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_real_ifchain | 2608616ns | no significant difference | [-36120, +11146]ns | [2582208, 2618313] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchainasc | 2614110ns | no significant difference | [-5207, +5984]ns | [2606112, 2619506] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchainlin | 3266609ns | +659680.2ns (+25.2%) | [+608214, +743632]ns | [3222794, 3359669] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_real_nullfloor | 560587ns | -2046612.3ns (-78.3%) | [-2063699, -1992051]ns | [552068, 622529] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_real_threaded | 2920850ns | +306747.5ns (+11.7%) | [+298858, +313751]ns | [2909508, 2926546] | YES (adj: no) | 0.0547 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2611607ns | -2.1% | -10.2% | +0.4% | -0.1% | +24.3% | -79.0% | +12.0% |
| 2 | 2612592ns | -0.4% | -10.6% | +0.1% | +0.3% | +23.7% | -77.2% | +11.6% |
| 3 | 2620088ns | +2.9% | -10.3% | -1.3% | -0.0% | +30.6% | -78.8% | +11.6% |
| 4 | 2613982ns | +3.2% | -10.4% | -0.3% | +0.2% | +26.2% | -78.8% | +12.0% |
| 5 | 2616569ns | +6.4% | -10.0% | -1.5% | -0.3% | +22.8% | -75.2% | +11.2% |
| 6 | 2600254ns | +3.1% | -9.6% | +0.4% | +0.1% | +26.4% | -78.2% | +11.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | 0.351 | moderate+ |
| carrier_disp_real_fntable | -0.149 | ok |
| carrier_disp_real_ifchain | -0.193 | ok |
| carrier_disp_real_ifchainasc | 0.246 | moderate+ |
| carrier_disp_real_ifchainlin | -0.155 | ok |
| carrier_disp_real_nullfloor | -0.389 | moderate- |
| carrier_disp_real_switch | -0.142 | ok |
| carrier_disp_real_threaded | 0.108 | ok |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 2/6, lost 4/6
- **carrier_disp_real_fntable**: won 6/6, lost 0/6
- **carrier_disp_real_ifchain**: won 3/6, lost 2/6
- **carrier_disp_real_ifchainasc**: won 2/6, lost 3/6
- **carrier_disp_real_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 2672129.9ns | 2669660.5ns | 100.1% | HIGH |
| carrier_disp_real_fntable | 2334897.4ns | 2345771.8ns | 99.5% | HIGH |
| carrier_disp_real_ifchain | 2607739.7ns | 2603045.5ns | 100.2% | HIGH |
| carrier_disp_real_ifchainasc | 2616779.8ns | 2613242.8ns | 100.1% | HIGH |
| carrier_disp_real_ifchainlin | 3553894.8ns | 3283023.9ns | 108.3% | HIGH |
| carrier_disp_real_nullfloor | 580313.2ns | 578394.7ns | 100.3% | HIGH |
| carrier_disp_real_switch | 2615909.7ns | 2612515.4ns | 100.1% | HIGH |
| carrier_disp_real_threaded | 2919258.9ns | 2918967.7ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 2557765.4-2740647.9 ns)
  2557765.4 |####################
  2566909.5 |
  2576053.6 |
  2585197.8 |
  2594341.9 |####################
  2603486.0 |
  2612630.1 |
  2621774.3 |
  2630918.4 |
  2640062.5 |
  2649206.7 |
  2658350.8 |
  2667494.9 |
  2676639.0 |####################
  2685783.2 |
  2694927.3 |########################################
  2704071.4 |
  2713215.5 |
  2722359.7 |
  2731503.8 |
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 2335399.2-2351761.8 ns)
  2335399.2 |########################################
  2336217.3 |
  2337035.5 |
  2337853.6 |
  2338671.7 |
  2339489.9 |
  2340308.0 |
  2341126.1 |
  2341944.3 |########################################
  2342762.4 |
  2343580.5 |
  2344398.7 |########################################
  2345216.8 |
  2346034.9 |
  2346853.1 |
  2347671.2 |
  2348489.3 |########################################
  2349307.5 |########################################
  2350125.6 |
  2350943.7 |
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 2577485.8-2618312.7 ns)
  2577485.8 |########################################
  2579527.1 |
  2581568.5 |
  2583609.8 |
  2585651.2 |########################################
  2587692.5 |
  2589733.9 |
  2591775.2 |
  2593816.6 |
  2595857.9 |
  2597899.2 |
  2599940.6 |
  2601981.9 |
  2604023.3 |########################################
  2606064.6 |
  2608106.0 |
  2610147.3 |########################################
  2612188.7 |########################################
  2614230.0 |
  2616271.4 |
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 2603755.0-2619506.2 ns)
  2603755.0 |####################
  2604542.6 |
  2605330.1 |
  2606117.7 |
  2606905.2 |
  2607692.8 |####################
  2608480.4 |
  2609267.9 |####################
  2610055.5 |
  2610843.1 |
  2611630.6 |
  2612418.2 |
  2613205.8 |
  2613993.3 |
  2614780.9 |
  2615568.4 |
  2616356.0 |
  2617143.6 |
  2617931.1 |
  2618718.7 |########################################
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 3213187.5-3359668.5 ns)
  3213187.5 |########################################
  3220511.6 |
  3227835.6 |########################################
  3235159.7 |
  3242483.7 |########################################
  3249807.8 |
  3257131.8 |
  3264455.9 |
  3271779.9 |
  3279104.0 |
  3286428.0 |########################################
  3293752.1 |########################################
  3301076.1 |
  3308400.2 |
  3315724.2 |
  3323048.3 |
  3330372.3 |
  3337696.4 |
  3345020.4 |
  3352344.5 |
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 549384.6-622529.2 ns)
  549384.6 |####################
  553041.8 |########################################
  556699.1 |
  560356.3 |
  564013.5 |####################
  567670.8 |
  571328.0 |
  574985.2 |
  578642.4 |
  582299.7 |
  585956.9 |
  589614.1 |
  593271.4 |####################
  596928.6 |
  600585.8 |
  604243.0 |
  607900.3 |
  611557.5 |
  615214.7 |
  618872.0 |
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 2600254.2-2618328.4 ns)
  2600254.2 |########################################
  2601157.9 |
  2602061.6 |
  2602965.3 |
  2603869.0 |
  2604772.7 |
  2605676.4 |
  2606580.2 |
  2607483.9 |
  2608387.6 |
  2609291.3 |
  2610195.0 |
  2611098.7 |########################################
  2612002.4 |########################################
  2612906.1 |
  2613809.8 |########################################
  2614713.5 |
  2615617.2 |
  2616520.9 |########################################
  2617424.6 |
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 2908711.7-2926546.0 ns)
  2908711.7 |########################################
  2909603.4 |########################################
  2910495.1 |
  2911386.8 |
  2912278.6 |
  2913170.3 |
  2914062.0 |
  2914953.7 |
  2915845.4 |########################################
  2916737.1 |
  2917628.9 |
  2918520.6 |
  2919412.3 |
  2920304.0 |
  2921195.7 |
  2922087.4 |
  2922979.1 |
  2923870.9 |
  2924762.6 |########################################
  2925654.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=99.5% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=100.0% of algo (FFI overhead may distort results)
