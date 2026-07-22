# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 41% faster than the next best (carrier_disp_scatter_ifchain)

carrier_disp_scatter_nullfloor (7.93 us) leads carrier_disp_scatter_ifchain (11.17 us) by 41%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 29% (significant)

carrier_disp_scatter_nullfloor is -3.27 us (29%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_ifchainlin is an outlier: 3.0x slower than the field

carrier_disp_scatter_ifchainlin (23.84 us) is 3.0x the fastest (7.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_scatter_nullfloor shows alternating (throttle bounce) (autocorr -0.65)

carrier_disp_scatter_nullfloor's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_scatter_nullfloor, carrier_disp_scatter_ifchain, carrier_disp_scatter_switch, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} vs {carrier_disp_scatter_ifchainlin} (67% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor, carrier_disp_scatter_ifchain, carrier_disp_scatter_switch, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} and a slow tier {carrier_disp_scatter_ifchainlin} with a 67% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.0x the fastest

Fastest carrier_disp_scatter_nullfloor (7.93 us) to slowest carrier_disp_scatter_ifchainlin (23.84 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_disp_scatter_ifchain's edge over baseline is significant but tiny (-13 ns, 0.12%)

carrier_disp_scatter_ifchain differs from baseline carrier_disp_scatter_switch by -13 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 7926.4 ns median (-29.1% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 3.01x (fastest 7926.4 ns, slowest 23840.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 16278ns | 16143ns | 16101ns | 16131ns | 16586ns | +18.40% |
| carrier_disp_scatter_fntable | 16956ns | 16859ns | 16736ns | 16855ns | 17217ns | +23.33% |
| carrier_disp_scatter_ifchain | 13814ns | 13779ns | 13754ns | 13774ns | 13905ns | +0.48% |
| carrier_disp_scatter_ifchainasc | 13616ns | 13876ns | 12102ns | 13869ns | 13992ns | -0.97% |
| carrier_disp_scatter_ifchainlin | 26478ns | 26483ns | 25963ns | 26416ns | 26828ns | +92.58% |
| carrier_disp_scatter_nullfloor | 10516ns | 10499ns | 10330ns | 10483ns | 10657ns | -23.51% |
| carrier_disp_scatter_switch | 13749ns | 13821ns | 13425ns | 13710ns | 13969ns | base |
| carrier_disp_scatter_threaded | 15161ns | 15148ns | 14973ns | 15124ns | 15310ns | +10.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 13691ns | 13503ns | 13931ns | +22.65% | 0.019 |
| carrier_disp_scatter_fntable | 14314ns | 14184ns | 14512ns | +28.23% | 0.018 |
| carrier_disp_scatter_ifchain | 11201ns | 11110ns | 11307ns | +0.35% | 0.023 |
| carrier_disp_scatter_ifchainasc | 11036ns | 9805ns | 11402ns | -1.14% | 0.023 |
| carrier_disp_scatter_ifchainlin | 23886ns | 23531ns | 24171ns | +113.98% | 0.011 |
| carrier_disp_scatter_nullfloor | 7925ns | 7856ns | 7979ns | -29.00% | 0.032 |
| carrier_disp_scatter_switch | 11163ns | 10979ns | 11324ns | base | 0.023 |
| carrier_disp_scatter_threaded | 12631ns | 12554ns | 12701ns | +13.15% | 0.020 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.019 | 57.7% |
| carrier_disp_scatter_fntable | 0.018 | 55.2% |
| carrier_disp_scatter_ifchain | 0.023 | 70.3% |
| carrier_disp_scatter_ifchainasc | 0.023 | 69.9% |
| carrier_disp_scatter_ifchainlin | 0.011 | 33.0% |
| carrier_disp_scatter_nullfloor | 0.032 | 99.1% |
| carrier_disp_scatter_switch | 0.023 | 70.3% |
| carrier_disp_scatter_threaded | 0.020 | 62.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 16278ns | 16278ns | +18.40% |
| carrier_disp_scatter_fntable | 16956ns | 16956ns | +23.33% |
| carrier_disp_scatter_ifchain | 13814ns | 13814ns | +0.48% |
| carrier_disp_scatter_ifchainasc | 13616ns | 13616ns | -0.97% |
| carrier_disp_scatter_ifchainlin | 26478ns | 26478ns | +92.58% |
| carrier_disp_scatter_nullfloor | 10516ns | 10516ns | -23.51% |
| carrier_disp_scatter_switch | 13749ns | 13749ns | base |
| carrier_disp_scatter_threaded | 15161ns | 15161ns | +10.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 11181ns | base | --- | [10982, 11324] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 13614ns | +2571.2ns (+23.0%) | [+2279, +2736]ns | [13529, 13931] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_fntable | 14236ns | +3142.7ns (+28.1%) | [+2909, +3404]ns | [14196, 14512] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_ifchain | 11171ns | no significant difference | [-186, +315]ns | [11125, 11307] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_scatter_ifchainasc | 11244ns | no significant difference | [-734, +347]ns | [10461, 11402] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_scatter_ifchainlin | 23841ns | +12780.0ns (+114.3%) | [+12525, +12865]ns | [23646, 24171] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_nullfloor | 7926ns | -3275.0ns (-29.3%) | [-3398, -3040]ns | [7871, 7979] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 12628ns | +1418.1ns (+12.7%) | [+1268, +1718]ns | [12564, 12701] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 11133ns | +23.9% | +27.7% | +0.1% | -11.9% | +111.4% | -29.2% | +13.4% |
| 2 | 11258ns | +25.0% | +26.7% | -0.8% | -1.2% | +112.4% | -29.3% | +11.5% |
| 3 | 11390ns | +19.6% | +24.7% | -2.5% | -1.1% | +112.2% | -30.0% | +10.9% |
| 4 | 11230ns | +20.7% | +30.0% | -0.3% | +1.3% | +115.3% | -30.0% | +12.0% |
| 5 | 10979ns | +23.0% | +29.2% | +1.8% | +2.3% | +116.4% | -27.2% | +16.1% |
| 6 | 10986ns | +23.8% | +31.3% | +3.9% | +4.0% | +116.4% | -28.1% | +15.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | 0.291 | moderate+ |
| carrier_disp_scatter_fntable | -0.522 | HIGH- (thermal bounce) |
| carrier_disp_scatter_ifchain | 0.008 | ok |
| carrier_disp_scatter_ifchainasc | 0.071 | ok |
| carrier_disp_scatter_ifchainlin | 0.182 | ok |
| carrier_disp_scatter_nullfloor | -0.645 | HIGH- (thermal bounce) |
| carrier_disp_scatter_switch | 0.413 | moderate+ |
| carrier_disp_scatter_threaded | -0.129 | ok |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 0/6, lost 6/6
- **carrier_disp_scatter_fntable**: won 0/6, lost 6/6
- **carrier_disp_scatter_ifchain**: won 3/6, lost 2/6
- **carrier_disp_scatter_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_scatter_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 91826.3ns | 13691.2ns | 670.7% | HIGH |
| carrier_disp_scatter_fntable | 96487.1ns | 14314.3ns | 674.1% | HIGH |
| carrier_disp_scatter_ifchain | 90010.3ns | 11201.2ns | 803.6% | HIGH |
| carrier_disp_scatter_ifchainasc | 90091.4ns | 11035.6ns | 816.4% | HIGH |
| carrier_disp_scatter_ifchainlin | 95627.8ns | 23886.1ns | 400.3% | HIGH |
| carrier_disp_scatter_nullfloor | 87293.2ns | 7925.2ns | 1101.5% | HIGH |
| carrier_disp_scatter_switch | 89596.1ns | 11162.6ns | 802.6% | HIGH |
| carrier_disp_scatter_threaded | 88792.4ns | 12630.8ns | 703.0% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 13503.3-13930.9 ns)
  13503.3 |########################################
  13524.7 |
  13546.1 |########################################
  13567.4 |
  13588.8 |########################################
  13610.2 |########################################
  13631.6 |
  13652.9 |
  13674.3 |
  13695.7 |
  13717.1 |
  13738.5 |
  13759.8 |
  13781.2 |########################################
  13802.6 |
  13824.0 |
  13845.3 |
  13866.7 |
  13888.1 |
  13909.5 |
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 14183.8-14511.7 ns)
  14183.8 |####################
  14200.2 |########################################
  14216.6 |
  14233.0 |
  14249.4 |####################
  14265.8 |
  14282.2 |
  14298.6 |
  14315.0 |
  14331.4 |
  14347.8 |
  14364.1 |
  14380.5 |
  14396.9 |
  14413.3 |####################
  14429.7 |
  14446.1 |
  14462.5 |
  14478.9 |
  14495.3 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 11109.6-11307.1 ns)
  11109.6 |########################################
  11119.5 |
  11129.4 |
  11139.2 |########################################
  11149.1 |
  11159.0 |########################################
  11168.9 |########################################
  11178.7 |
  11188.6 |########################################
  11198.5 |
  11208.4 |
  11218.2 |
  11228.1 |
  11238.0 |
  11247.9 |
  11257.7 |
  11267.6 |
  11277.5 |
  11287.4 |
  11297.2 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 9805.0-11401.6 ns)
   9805.0 |########################################
   9884.8 |
   9964.7 |
  10044.5 |
  10124.3 |
  10204.2 |
  10284.0 |
  10363.8 |
  10443.7 |
  10523.5 |
  10603.3 |
  10683.2 |
  10763.0 |
  10842.8 |
  10922.7 |
  11002.5 |
  11082.3 |########################################
  11162.2 |########################################
  11242.0 |########################################
  11321.8 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 23530.8-24171.5 ns)
  23530.8 |####################
  23562.8 |
  23594.9 |
  23626.9 |
  23658.9 |
  23691.0 |
  23723.0 |
  23755.0 |########################################
  23787.1 |
  23819.1 |
  23851.1 |
  23883.2 |####################
  23915.2 |
  23947.2 |
  23979.3 |
  24011.3 |
  24043.3 |
  24075.4 |
  24107.4 |
  24139.4 |####################
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 7856.2-7978.5 ns)
   7856.2 |########################################
   7862.3 |
   7868.4 |
   7874.6 |
   7880.7 |########################################
   7886.8 |
   7892.9 |########################################
   7899.0 |
   7905.1 |
   7911.3 |
   7917.4 |
   7923.5 |
   7929.6 |
   7935.7 |
   7941.8 |
   7948.0 |
   7954.1 |########################################
   7960.2 |
   7966.3 |########################################
   7972.4 |
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 10978.8-11324.0 ns)
  10978.8 |########################################
  10996.1 |
  11013.3 |
  11030.6 |
  11047.8 |
  11065.1 |
  11082.3 |
  11099.6 |
  11116.9 |####################
  11134.1 |
  11151.4 |
  11168.6 |
  11185.9 |
  11203.1 |
  11220.4 |####################
  11237.7 |
  11254.9 |####################
  11272.2 |
  11289.4 |
  11306.7 |
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 12553.8-12700.8 ns)
  12553.8 |########################################
  12561.1 |
  12568.5 |########################################
  12575.8 |
  12583.2 |
  12590.5 |
  12597.9 |
  12605.2 |
  12612.6 |
  12619.9 |########################################
  12627.3 |########################################
  12634.6 |
  12642.0 |
  12649.3 |
  12656.7 |########################################
  12664.0 |
  12671.4 |
  12678.8 |
  12686.1 |
  12693.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=677.6% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: bridge=678.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=804.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=800.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=400.3% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=1101.5% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: bridge=800.4% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=702.1% of algo (FFI overhead may distort results)
