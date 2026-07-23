# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 28% faster than the next best (carrier_disp_wideselect_ifchainasc)

carrier_disp_wideselect_nullfloor (7.87 us) leads carrier_disp_wideselect_ifchainasc (10.11 us) by 28%, a clear separation rather than a photo finish. CV 4.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 25% (significant)

carrier_disp_wideselect_nullfloor is -2.65 us (25%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 3.0x slower than the field

carrier_disp_wideselect_ifchainlin (23.92 us) is 3.0x the fastest (7.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_wideselect_fntable shows alternating (throttle bounce) (autocorr -0.86)

carrier_disp_wideselect_fntable's per-pass series has lag-1 autocorrelation -0.86, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} vs {carrier_disp_wideselect_ifchainlin} (94% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} and a slow tier {carrier_disp_wideselect_ifchainlin} with a 94% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.0x the fastest

Fastest carrier_disp_wideselect_nullfloor (7.87 us) to slowest carrier_disp_wideselect_ifchainlin (23.92 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 7866.6 ns median (-24.5% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 3.04x (fastest 7866.6 ns, slowest 23917.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 14718ns | 14753ns | 14374ns | 14689ns | 14934ns | +15.86% |
| carrier_disp_wideselect_fntable | 14877ns | 14753ns | 14112ns | 14546ns | 15755ns | +17.11% |
| carrier_disp_wideselect_ifchain | 13296ns | 13229ns | 13116ns | 13204ns | 13524ns | +4.67% |
| carrier_disp_wideselect_ifchainasc | 12437ns | 12462ns | 11521ns | 12185ns | 13274ns | -2.09% |
| carrier_disp_wideselect_ifchainlin | 26431ns | 26482ns | 26100ns | 26463ns | 26548ns | +108.06% |
| carrier_disp_wideselect_nullfloor | 10067ns | 10298ns | 9265ns | 10017ns | 10544ns | -20.75% |
| carrier_disp_wideselect_switch | 12703ns | 12831ns | 11718ns | 12716ns | 13177ns | base |
| carrier_disp_wideselect_threaded | 13919ns | 14110ns | 12944ns | 13882ns | 14461ns | +9.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 12165ns | 11857ns | 12339ns | +18.59% | 0.021 |
| carrier_disp_wideselect_fntable | 12483ns | 11859ns | 13230ns | +21.70% | 0.021 |
| carrier_disp_wideselect_ifchain | 10863ns | 10698ns | 11082ns | +5.90% | 0.024 |
| carrier_disp_wideselect_ifchainasc | 10083ns | 9387ns | 10753ns | -1.71% | 0.025 |
| carrier_disp_wideselect_ifchainlin | 23901ns | 23735ns | 24019ns | +133.00% | 0.011 |
| carrier_disp_wideselect_nullfloor | 7672ns | 7102ns | 8002ns | -25.21% | 0.033 |
| carrier_disp_wideselect_switch | 10258ns | 9448ns | 10602ns | base | 0.025 |
| carrier_disp_wideselect_threaded | 11555ns | 10779ns | 12027ns | +12.65% | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 309003 | 1405476 | 0.220 | 1.00× |
| carrier_disp_wideselect_fntable | 322701 | 1738426 | 0.186 | 1.04× |
| carrier_disp_wideselect_ifchain | 306056 | 1464358 | 0.209 | 0.99× |
| carrier_disp_wideselect_ifchainasc | 315633 | 1594202 | 0.198 | 1.02× |
| carrier_disp_wideselect_ifchainlin | 346746 | 2081504 | 0.167 | 1.12× |
| carrier_disp_wideselect_nullfloor | 287088 | 1638394 | 0.175 | 0.93× |
| carrier_disp_wideselect_switch | 309224 | 1520082 | 0.203 | 1.00× |
| carrier_disp_wideselect_threaded | 309164 | 1850766 | 0.167 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.021 | 58.2% |
| carrier_disp_wideselect_fntable | 0.021 | 57.5% |
| carrier_disp_wideselect_ifchain | 0.024 | 65.8% |
| carrier_disp_wideselect_ifchainasc | 0.025 | 70.3% |
| carrier_disp_wideselect_ifchainlin | 0.011 | 29.7% |
| carrier_disp_wideselect_nullfloor | 0.033 | 90.3% |
| carrier_disp_wideselect_switch | 0.025 | 68.2% |
| carrier_disp_wideselect_threaded | 0.022 | 60.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 14718ns | 14718ns | +15.86% |
| carrier_disp_wideselect_fntable | 14877ns | 14877ns | +17.11% |
| carrier_disp_wideselect_ifchain | 13296ns | 13296ns | +4.67% |
| carrier_disp_wideselect_ifchainasc | 12437ns | 12437ns | -2.09% |
| carrier_disp_wideselect_ifchainlin | 26431ns | 26431ns | +108.06% |
| carrier_disp_wideselect_nullfloor | 10067ns | 10067ns | -20.75% |
| carrier_disp_wideselect_switch | 12703ns | 12703ns | base |
| carrier_disp_wideselect_threaded | 13919ns | 13919ns | +9.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 10415ns | base | --- | [9756, 10602] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 12196ns | +1884.1ns (+18.1%) | [+1536, +2302]ns | [11960, 12339] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 12347ns | +2307.5ns (+22.2%) | [+1424, +2946]ns | [11873, 13230] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchain | 10791ns | +510.4ns (+4.9%) | [+113, +1191]ns | [10716, 11082] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchainasc | 10106ns | no significant difference | [-1213, +894]ns | [9390, 10753] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_wideselect_ifchainlin | 23918ns | +13555.5ns (+130.1%) | [+13202, +14172]ns | [23767, 24019] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 7867ns | -2645.8ns (-25.4%) | [-2990, -2121]ns | [7149, 8002] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 11686ns | +1424.2ns (+13.7%) | [+351, +2118]ns | [10953, 12027] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 10631ns | +14.9% | +20.4% | +1.0% | -11.7% | +123.3% | -25.5% | +4.7% |
| 2 | 10325ns | +20.3% | +14.9% | +7.2% | -7.3% | +132.3% | -31.2% | +15.5% |
| 3 | 10506ns | +15.9% | +25.4% | +2.6% | +3.3% | +128.1% | -24.5% | +11.8% |
| 4 | 9448ns | +25.5% | +25.9% | +14.3% | +12.8% | +151.9% | -23.8% | +23.0% |
| 5 | 10063ns | +21.8% | +32.0% | +10.3% | +5.8% | +139.1% | -19.8% | +20.5% |
| 6 | 10573ns | +14.1% | +12.4% | +1.2% | -11.2% | +125.8% | -26.1% | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | -0.146 | ok |
| carrier_disp_wideselect_fntable | -0.859 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_ifchain | -0.605 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_ifchainasc | 0.133 | ok |
| carrier_disp_wideselect_ifchainlin | -0.478 | moderate- |
| carrier_disp_wideselect_nullfloor | -0.629 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_switch | -0.063 | ok |
| carrier_disp_wideselect_threaded | -0.370 | moderate- |

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
| carrier_disp_wideselect_bittree | 95750.9ns | 12165.0ns | 787.1% | HIGH |
| carrier_disp_wideselect_fntable | 93138.4ns | 12483.3ns | 746.1% | HIGH |
| carrier_disp_wideselect_ifchain | 91867.4ns | 10862.8ns | 845.7% | HIGH |
| carrier_disp_wideselect_ifchainasc | 93702.3ns | 10082.7ns | 929.3% | HIGH |
| carrier_disp_wideselect_ifchainlin | 95629.4ns | 23901.0ns | 400.1% | HIGH |
| carrier_disp_wideselect_nullfloor | 88811.9ns | 7672.2ns | 1157.6% | HIGH |
| carrier_disp_wideselect_switch | 93378.6ns | 10257.8ns | 910.3% | HIGH |
| carrier_disp_wideselect_threaded | 92214.7ns | 11555.4ns | 798.0% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 11856.7-12339.3 ns)
  11856.7 |########################################
  11880.8 |
  11905.0 |
  11929.1 |
  11953.2 |
  11977.4 |
  12001.5 |
  12025.6 |
  12049.8 |########################################
  12073.9 |
  12098.0 |
  12122.2 |
  12146.3 |
  12170.4 |########################################
  12194.6 |########################################
  12218.7 |
  12242.8 |########################################
  12267.0 |
  12291.1 |
  12315.2 |
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 11859.2-13230.2 ns)
  11859.2 |########################################
  11927.8 |
  11996.3 |
  12064.9 |
  12133.4 |
  12202.0 |
  12270.5 |
  12339.1 |
  12407.6 |
  12476.2 |
  12544.7 |
  12613.2 |
  12681.8 |
  12750.4 |#############
  12818.9 |
  12887.5 |
  12956.0 |
  13024.6 |
  13093.1 |
  13161.7 |#############
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 10697.5-11081.6 ns)
  10697.5 |########################################
  10716.7 |########################################
  10735.9 |
  10755.1 |
  10774.3 |########################################
  10793.5 |########################################
  10812.7 |
  10832.0 |
  10851.2 |
  10870.4 |
  10889.6 |
  10908.8 |
  10928.0 |
  10947.2 |
  10966.4 |
  10985.6 |
  11004.8 |
  11024.0 |
  11043.2 |
  11062.4 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 9387.1-10752.9 ns)
   9387.1 |########################################
   9455.4 |
   9523.7 |####################
   9592.0 |
   9660.3 |
   9728.5 |
   9796.8 |
   9865.1 |
   9933.4 |
  10001.7 |
  10070.0 |
  10138.3 |
  10206.6 |
  10274.9 |
  10343.2 |
  10411.5 |
  10479.7 |
  10548.0 |
  10616.3 |########################################
  10684.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 23735.4-24018.8 ns)
  23735.4 |########################################
  23749.6 |
  23763.7 |
  23777.9 |
  23792.1 |########################################
  23806.2 |
  23820.4 |
  23834.6 |
  23848.7 |
  23862.9 |########################################
  23877.1 |
  23891.2 |
  23905.4 |
  23919.6 |
  23933.7 |
  23947.9 |########################################
  23962.1 |
  23976.2 |########################################
  23990.4 |
  24004.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 7102.1-8001.5 ns)
   7102.1 |####################
   7147.1 |
   7192.0 |####################
   7237.0 |
   7282.0 |
   7327.0 |
   7371.9 |
   7416.9 |
   7461.9 |
   7506.8 |
   7551.8 |
   7596.8 |
   7641.7 |
   7686.7 |
   7731.7 |
   7776.6 |####################
   7821.6 |
   7866.6 |
   7911.6 |########################################
   7956.5 |
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 9447.9-10602.2 ns)
   9447.9 |########################################
   9505.6 |
   9563.3 |
   9621.1 |
   9678.8 |
   9736.5 |
   9794.2 |
   9851.9 |
   9909.6 |
   9967.4 |
  10025.1 |########################################
  10082.8 |
  10140.5 |
  10198.2 |
  10255.9 |
  10313.7 |########################################
  10371.4 |
  10429.1 |
  10486.8 |########################################
  10544.5 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 10778.8-12026.9 ns)
  10778.8 |########################################
  10841.2 |
  10903.6 |
  10966.0 |
  11028.4 |
  11090.8 |########################################
  11153.2 |
  11215.6 |
  11278.0 |
  11340.4 |
  11402.9 |
  11465.3 |
  11527.7 |
  11590.1 |########################################
  11652.5 |
  11714.9 |########################################
  11777.3 |
  11839.7 |
  11902.1 |########################################
  11964.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: bridge=785.9% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=755.9% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=852.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=932.8% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=400.5% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=1122.9% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=900.5% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=801.7% of algo (FFI overhead may distort results)
