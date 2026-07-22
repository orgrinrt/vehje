# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 34% faster than the next best (carrier_disp_wideselect_ifchainasc)

carrier_disp_wideselect_nullfloor (8.05 us) leads carrier_disp_wideselect_ifchainasc (10.81 us) by 34%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 26% (significant)

carrier_disp_wideselect_nullfloor is -2.81 us (26%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 3.0x slower than the field

carrier_disp_wideselect_ifchainlin (24.42 us) is 3.0x the fastest (8.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} vs {carrier_disp_wideselect_ifchainlin} (84% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} and a slow tier {carrier_disp_wideselect_ifchainlin} with a 84% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.0x the fastest

Fastest carrier_disp_wideselect_nullfloor (8.05 us) to slowest carrier_disp_wideselect_ifchainlin (24.42 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_disp_wideselect_ifchainasc's edge over baseline is significant but tiny (2 ns, 0.02%)

carrier_disp_wideselect_ifchainasc differs from baseline carrier_disp_wideselect_switch by 2 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 8051.2 ns median (-25.9% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 3.03x (fastest 8051.2 ns, slowest 24418.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 14911ns | 14877ns | 14650ns | 14858ns | 15122ns | +12.42% |
| carrier_disp_wideselect_fntable | 15901ns | 15909ns | 15638ns | 15867ns | 16085ns | +19.88% |
| carrier_disp_wideselect_ifchain | 14030ns | 13922ns | 13824ns | 13903ns | 14326ns | +5.77% |
| carrier_disp_wideselect_ifchainasc | 13203ns | 13324ns | 12061ns | 13293ns | 13638ns | -0.47% |
| carrier_disp_wideselect_ifchainlin | 27016ns | 26993ns | 26645ns | 26986ns | 27248ns | +103.67% |
| carrier_disp_wideselect_nullfloor | 10619ns | 10713ns | 10230ns | 10654ns | 10761ns | -19.95% |
| carrier_disp_wideselect_switch | 13265ns | 13407ns | 12719ns | 13328ns | 13442ns | base |
| carrier_disp_wideselect_threaded | 14710ns | 14713ns | 14555ns | 14674ns | 14842ns | +10.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 12286ns | 12132ns | 12387ns | +14.02% | 0.021 |
| carrier_disp_wideselect_fntable | 13276ns | 13190ns | 13384ns | +23.20% | 0.019 |
| carrier_disp_wideselect_ifchain | 11449ns | 11258ns | 11670ns | +6.25% | 0.022 |
| carrier_disp_wideselect_ifchainasc | 10692ns | 9770ns | 11016ns | -0.77% | 0.024 |
| carrier_disp_wideselect_ifchainlin | 24374ns | 24011ns | 24501ns | +126.20% | 0.011 |
| carrier_disp_wideselect_nullfloor | 8028ns | 7797ns | 8123ns | -25.50% | 0.032 |
| carrier_disp_wideselect_switch | 10775ns | 10360ns | 10927ns | base | 0.024 |
| carrier_disp_wideselect_threaded | 12076ns | 11975ns | 12173ns | +12.07% | 0.021 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.021 | 63.4% |
| carrier_disp_wideselect_fntable | 0.019 | 58.9% |
| carrier_disp_wideselect_ifchain | 0.022 | 68.4% |
| carrier_disp_wideselect_ifchainasc | 0.024 | 72.1% |
| carrier_disp_wideselect_ifchainlin | 0.010 | 31.9% |
| carrier_disp_wideselect_nullfloor | 0.032 | 96.8% |
| carrier_disp_wideselect_switch | 0.024 | 71.8% |
| carrier_disp_wideselect_threaded | 0.021 | 64.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 14911ns | 14911ns | +12.42% |
| carrier_disp_wideselect_fntable | 15901ns | 15901ns | +19.88% |
| carrier_disp_wideselect_ifchain | 14030ns | 14030ns | +5.77% |
| carrier_disp_wideselect_ifchainasc | 13203ns | 13203ns | -0.47% |
| carrier_disp_wideselect_ifchainlin | 27016ns | 27016ns | +103.67% |
| carrier_disp_wideselect_nullfloor | 10619ns | 10619ns | -19.95% |
| carrier_disp_wideselect_switch | 13265ns | 13265ns | base |
| carrier_disp_wideselect_threaded | 14710ns | 14710ns | +10.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 10860ns | base | --- | [10539, 10927] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 12297ns | +1506.1ns (+13.9%) | [+1263, +1763]ns | [12174, 12387] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 13248ns | +2473.6ns (+22.8%) | [+2338, +2690]ns | [13195, 13384] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchain | 11400ns | +526.1ns (+4.8%) | [+429, +1066]ns | [11278, 11670] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchainasc | 10811ns | no significant difference | [-611, +359]ns | [10249, 11016] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_wideselect_ifchainlin | 24418ns | +13609.1ns (+125.3%) | [+13500, +13686]ns | [24202, 24501] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 8051ns | -2806.0ns (-25.8%) | [-2946, -2491]ns | [7909, 8123] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 12067ns | +1253.5ns (+11.5%) | [+1061, +1588]ns | [11988, 12173] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 10826ns | +14.5% | +23.5% | +5.5% | -9.8% | +125.3% | -28.0% | +12.8% |
| 2 | 10894ns | +12.1% | +21.5% | +4.5% | -1.5% | +125.4% | -26.2% | +10.3% |
| 3 | 10928ns | +11.0% | +22.6% | +3.4% | +2.3% | +123.4% | -26.2% | +9.8% |
| 4 | 10926ns | +13.3% | +21.4% | +4.7% | -0.6% | +123.7% | -25.2% | +9.6% |
| 5 | 10718ns | +14.6% | +23.1% | +5.0% | +0.7% | +127.9% | -24.6% | +13.1% |
| 6 | 10360ns | +18.9% | +27.4% | +14.9% | +4.6% | +131.8% | -22.6% | +17.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | -0.232 | moderate- |
| carrier_disp_wideselect_fntable | -0.087 | ok |
| carrier_disp_wideselect_ifchain | -0.261 | moderate- |
| carrier_disp_wideselect_ifchainasc | 0.083 | ok |
| carrier_disp_wideselect_ifchainlin | -0.016 | ok |
| carrier_disp_wideselect_nullfloor | 0.129 | ok |
| carrier_disp_wideselect_switch | 0.261 | moderate+ |
| carrier_disp_wideselect_threaded | 0.029 | ok |

**Consistency summary:**

- **carrier_disp_wideselect_bittree**: won 0/6, lost 6/6
- **carrier_disp_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchain**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_wideselect_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_wideselect_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_wideselect_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 95801.6ns | 12286.0ns | 779.8% | HIGH |
| carrier_disp_wideselect_fntable | 93276.4ns | 13275.5ns | 702.6% | HIGH |
| carrier_disp_wideselect_ifchain | 91254.7ns | 11449.2ns | 797.0% | HIGH |
| carrier_disp_wideselect_ifchainasc | 87953.2ns | 10691.8ns | 822.6% | HIGH |
| carrier_disp_wideselect_ifchainlin | 97510.1ns | 24373.9ns | 400.1% | HIGH |
| carrier_disp_wideselect_nullfloor | 88731.6ns | 8027.6ns | 1105.3% | HIGH |
| carrier_disp_wideselect_switch | 87630.2ns | 10775.3ns | 813.3% | HIGH |
| carrier_disp_wideselect_threaded | 95805.7ns | 12075.8ns | 793.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 12132.1-12386.6 ns)
  12132.1 |########################################
  12144.8 |
  12157.6 |
  12170.3 |
  12183.0 |
  12195.7 |
  12208.5 |########################################
  12221.2 |
  12233.9 |
  12246.6 |
  12259.4 |
  12272.1 |########################################
  12284.8 |
  12297.6 |
  12310.3 |########################################
  12323.0 |
  12335.7 |
  12348.5 |
  12361.2 |
  12373.9 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 13190.0-13384.3 ns)
  13190.0 |########################################
  13199.7 |
  13209.4 |
  13219.2 |
  13228.9 |####################
  13238.6 |
  13248.3 |
  13258.0 |####################
  13267.7 |
  13277.5 |
  13287.2 |
  13296.9 |
  13306.6 |
  13316.3 |
  13326.0 |
  13335.8 |
  13345.5 |
  13355.2 |
  13364.9 |####################
  13374.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 11257.9-11669.5 ns)
  11257.9 |########################################
  11278.5 |########################################
  11299.1 |
  11319.6 |
  11340.2 |
  11360.8 |
  11381.4 |########################################
  11402.0 |########################################
  11422.6 |########################################
  11443.1 |
  11463.7 |
  11484.3 |
  11504.9 |
  11525.5 |
  11546.1 |
  11566.6 |
  11587.2 |
  11607.8 |
  11628.4 |
  11649.0 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 9770.0-11016.2 ns)
   9770.0 |####################
   9832.3 |
   9894.6 |
   9956.9 |
  10019.2 |
  10081.6 |
  10143.9 |
  10206.2 |
  10268.5 |
  10330.8 |
  10393.1 |
  10455.4 |
  10517.8 |
  10580.1 |
  10642.4 |
  10704.7 |####################
  10767.0 |####################
  10829.3 |########################################
  10891.6 |
  10953.9 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 24011.2-24501.2 ns)
  24011.2 |####################
  24035.7 |
  24060.2 |
  24084.7 |
  24109.2 |
  24133.7 |
  24158.2 |
  24182.7 |
  24207.2 |
  24231.7 |
  24256.2 |
  24280.7 |
  24305.2 |
  24329.7 |
  24354.2 |
  24378.7 |####################
  24403.2 |########################################
  24427.7 |####################
  24452.2 |
  24476.7 |
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 7797.1-8123.1 ns)
   7797.1 |########################################
   7813.4 |
   7829.7 |
   7846.0 |
   7862.3 |
   7878.6 |
   7894.9 |
   7911.2 |
   7927.5 |
   7943.8 |
   7960.1 |
   7976.4 |
   7992.7 |
   8009.0 |########################################
   8025.3 |########################################
   8041.6 |
   8057.9 |########################################
   8074.2 |########################################
   8090.5 |
   8106.8 |
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 10360.0-10926.8 ns)
  10360.0 |########################################
  10388.3 |
  10416.7 |
  10445.0 |
  10473.4 |
  10501.7 |
  10530.1 |
  10558.4 |
  10586.7 |
  10615.1 |
  10643.4 |
  10671.8 |
  10700.1 |########################################
  10728.5 |
  10756.8 |
  10785.1 |
  10813.5 |########################################
  10841.8 |
  10870.2 |########################################
  10898.5 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 11975.4-12172.7 ns)
  11975.4 |########################################
  11985.3 |
  11995.1 |########################################
  12005.0 |########################################
  12014.9 |
  12024.7 |
  12034.6 |
  12044.5 |
  12054.3 |
  12064.2 |
  12074.0 |
  12083.9 |
  12093.8 |
  12103.6 |
  12113.5 |########################################
  12123.4 |########################################
  12133.2 |
  12143.1 |
  12153.0 |
  12162.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: bridge=777.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=702.4% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=801.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=811.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=400.5% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=1102.4% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=808.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=793.3% of algo (FFI overhead may distort results)
