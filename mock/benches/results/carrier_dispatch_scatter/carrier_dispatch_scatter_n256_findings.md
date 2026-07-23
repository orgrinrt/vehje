# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 47% faster than the next best (carrier_disp_scatter_ifchainasc)

carrier_disp_scatter_nullfloor (7.57 us) leads carrier_disp_scatter_ifchainasc (11.11 us) by 47%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 31% (significant)

carrier_disp_scatter_nullfloor is -3.40 us (31%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_ifchainlin is an outlier: 3.0x slower than the field

carrier_disp_scatter_ifchainlin (22.83 us) is 3.0x the fastest (7.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_scatter_threaded shows alternating (throttle bounce) (autocorr -0.59)

carrier_disp_scatter_threaded's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_scatter_nullfloor, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_ifchain, carrier_disp_scatter_switch, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} vs {carrier_disp_scatter_ifchainlin} (63% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_ifchain, carrier_disp_scatter_switch, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} and a slow tier {carrier_disp_scatter_ifchainlin} with a 63% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.0x the fastest

Fastest carrier_disp_scatter_nullfloor (7.57 us) to slowest carrier_disp_scatter_ifchainlin (22.83 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_disp_scatter_ifchain's edge over baseline is significant but tiny (-12 ns, 0.11%)

carrier_disp_scatter_ifchain differs from baseline carrier_disp_scatter_switch by -12 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 7567.5 ns median (-32.2% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 3.02x (fastest 7567.5 ns, slowest 22825.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 15323ns | 15658ns | 14289ns | 15232ns | 15977ns | +12.79% |
| carrier_disp_scatter_fntable | 16621ns | 16592ns | 16332ns | 16507ns | 16937ns | +22.34% |
| carrier_disp_scatter_ifchain | 13378ns | 13571ns | 12219ns | 13430ns | 13878ns | -1.53% |
| carrier_disp_scatter_ifchainasc | 13246ns | 13669ns | 12089ns | 13202ns | 13892ns | -2.50% |
| carrier_disp_scatter_ifchainlin | 25106ns | 25203ns | 23569ns | 24952ns | 26106ns | +84.80% |
| carrier_disp_scatter_nullfloor | 9966ns | 9905ns | 9416ns | 9781ns | 10518ns | -26.64% |
| carrier_disp_scatter_switch | 13586ns | 13685ns | 13232ns | 13572ns | 13783ns | base |
| carrier_disp_scatter_threaded | 14592ns | 15067ns | 13473ns | 14580ns | 15168ns | +7.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 12934ns | 12114ns | 13479ns | +16.86% | 0.020 |
| carrier_disp_scatter_fntable | 14020ns | 13684ns | 14291ns | +26.69% | 0.018 |
| carrier_disp_scatter_ifchain | 10919ns | 10000ns | 11294ns | -1.34% | 0.023 |
| carrier_disp_scatter_ifchainasc | 10767ns | 9859ns | 11258ns | -2.71% | 0.024 |
| carrier_disp_scatter_ifchainlin | 22751ns | 21400ns | 23640ns | +105.57% | 0.011 |
| carrier_disp_scatter_nullfloor | 7597ns | 7104ns | 8029ns | -31.36% | 0.034 |
| carrier_disp_scatter_switch | 11067ns | 10776ns | 11245ns | base | 0.023 |
| carrier_disp_scatter_threaded | 12214ns | 11296ns | 12717ns | +10.36% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 319557 | 1286714 | 0.248 | 1.09× |
| carrier_disp_scatter_fntable | 315546 | 1586547 | 0.199 | 1.08× |
| carrier_disp_scatter_ifchain | 298518 | 1404010 | 0.213 | 1.02× |
| carrier_disp_scatter_ifchainasc | 300464 | 1412876 | 0.213 | 1.03× |
| carrier_disp_scatter_ifchainlin | 343820 | 2046954 | 0.168 | 1.18× |
| carrier_disp_scatter_nullfloor | 288174 | 1635319 | 0.176 | 0.99× |
| carrier_disp_scatter_switch | 292068 | 1342109 | 0.218 | 1.00× |
| carrier_disp_scatter_threaded | 303070 | 1706110 | 0.178 | 1.04× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.019 | 53.8% |
| carrier_disp_scatter_fntable | 0.018 | 50.7% |
| carrier_disp_scatter_ifchain | 0.023 | 63.9% |
| carrier_disp_scatter_ifchainasc | 0.023 | 64.0% |
| carrier_disp_scatter_ifchainlin | 0.011 | 31.1% |
| carrier_disp_scatter_nullfloor | 0.034 | 93.9% |
| carrier_disp_scatter_switch | 0.023 | 63.7% |
| carrier_disp_scatter_threaded | 0.020 | 56.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 15323ns | 15323ns | +12.79% |
| carrier_disp_scatter_fntable | 16621ns | 16621ns | +22.34% |
| carrier_disp_scatter_ifchain | 13378ns | 13378ns | -1.53% |
| carrier_disp_scatter_ifchainasc | 13246ns | 13246ns | -2.50% |
| carrier_disp_scatter_ifchainlin | 25106ns | 25106ns | +84.80% |
| carrier_disp_scatter_nullfloor | 9966ns | 9966ns | -26.64% |
| carrier_disp_scatter_switch | 13586ns | 13586ns | base |
| carrier_disp_scatter_threaded | 14592ns | 14592ns | +7.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 11159ns | base | --- | [10797, 11245] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 13207ns | +1997.9ns (+17.9%) | [+1152, +2449]ns | [12115, 13479] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_fntable | 14020ns | +3017.7ns (+27.0%) | [+2519, +3323]ns | [13750, 14291] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_ifchain | 11113ns | no significant difference | [-623, +190]ns | [10350, 11294] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_scatter_ifchainasc | 11107ns | no significant difference | [-1310, +461]ns | [9935, 11258] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainlin | 22826ns | +11796.2ns (+105.7%) | [+10615, +12641]ns | [21788, 23640] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_nullfloor | 7568ns | -3404.8ns (-30.5%) | [-3814, -3193]ns | [7194, 8029] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 12604ns | +1488.5ns (+13.3%) | [+99, +1852]ns | [11320, 12717] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 11197ns | +16.8% | +24.4% | +0.7% | -11.9% | +98.0% | -35.0% | +12.7% |
| 2 | 11149ns | +8.7% | +29.7% | +1.5% | -0.3% | +91.9% | -26.9% | +1.3% |
| 3 | 11169ns | +21.3% | +23.7% | -4.2% | -0.6% | +110.0% | -30.5% | +13.9% |
| 4 | 11293ns | +18.7% | +21.2% | -0.9% | -11.4% | +108.2% | -30.0% | +0.5% |
| 5 | 10819ns | +23.3% | +30.5% | +2.0% | +4.9% | +105.2% | -34.3% | +16.3% |
| 6 | 10776ns | +12.4% | +31.0% | -7.2% | +3.7% | +120.6% | -31.6% | +18.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | -0.219 | moderate- |
| carrier_disp_scatter_fntable | -0.231 | moderate- |
| carrier_disp_scatter_ifchain | -0.064 | ok |
| carrier_disp_scatter_ifchainasc | -0.308 | moderate- |
| carrier_disp_scatter_ifchainlin | -0.137 | ok |
| carrier_disp_scatter_nullfloor | -0.094 | ok |
| carrier_disp_scatter_switch | 0.251 | moderate+ |
| carrier_disp_scatter_threaded | -0.588 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 0/6, lost 6/6
- **carrier_disp_scatter_fntable**: won 0/6, lost 6/6
- **carrier_disp_scatter_ifchain**: won 3/6, lost 3/6
- **carrier_disp_scatter_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_scatter_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 93936.2ns | 12933.5ns | 726.3% | HIGH |
| carrier_disp_scatter_fntable | 95926.9ns | 14020.5ns | 684.2% | HIGH |
| carrier_disp_scatter_ifchain | 90425.0ns | 10918.7ns | 828.2% | HIGH |
| carrier_disp_scatter_ifchainasc | 89772.2ns | 10766.8ns | 833.8% | HIGH |
| carrier_disp_scatter_ifchainlin | 91447.9ns | 22751.0ns | 402.0% | HIGH |
| carrier_disp_scatter_nullfloor | 88633.8ns | 7596.5ns | 1166.8% | HIGH |
| carrier_disp_scatter_switch | 89971.5ns | 11067.1ns | 813.0% | HIGH |
| carrier_disp_scatter_threaded | 89659.8ns | 12213.6ns | 734.1% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 12113.8-13478.8 ns)
  12113.8 |########################################
  12182.0 |
  12250.3 |
  12318.5 |
  12386.8 |
  12455.0 |
  12523.3 |
  12591.5 |
  12659.8 |
  12728.0 |
  12796.3 |
  12864.5 |
  12932.8 |
  13001.0 |
  13069.3 |####################
  13137.5 |
  13205.8 |
  13274.0 |####################
  13342.3 |####################
  13410.5 |
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 13683.8-14291.2 ns)
  13683.8 |####################
  13714.2 |
  13744.5 |
  13774.9 |
  13805.3 |####################
  13835.7 |
  13866.0 |
  13896.4 |
  13926.8 |####################
  13957.2 |
  13987.5 |
  14017.9 |
  14048.3 |
  14078.6 |
  14109.0 |########################################
  14139.4 |
  14169.8 |
  14200.1 |
  14230.5 |
  14260.9 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 10000.4-11293.8 ns)
  10000.4 |########################################
  10065.1 |
  10129.7 |
  10194.4 |
  10259.1 |
  10323.7 |
  10388.4 |
  10453.1 |
  10517.7 |
  10582.4 |
  10647.1 |########################################
  10711.7 |
  10776.4 |
  10841.1 |
  10905.7 |
  10970.4 |########################################
  11035.1 |
  11099.7 |
  11164.4 |########################################
  11229.1 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 9859.2-11258.2 ns)
   9859.2 |####################
   9929.1 |
   9999.1 |####################
  10069.0 |
  10139.0 |
  10208.9 |
  10278.9 |
  10348.8 |
  10418.8 |
  10488.7 |
  10558.7 |
  10628.6 |
  10698.6 |
  10768.5 |
  10838.5 |
  10908.4 |
  10978.4 |
  11048.3 |########################################
  11118.3 |####################
  11188.2 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 21400.4-23640.0 ns)
  21400.4 |####################
  21512.4 |
  21624.4 |
  21736.3 |
  21848.3 |
  21960.3 |
  22072.3 |####################
  22184.3 |####################
  22296.2 |
  22408.2 |
  22520.2 |
  22632.2 |
  22744.2 |
  22856.1 |
  22968.1 |
  23080.1 |
  23192.1 |
  23304.1 |
  23416.0 |########################################
  23528.0 |
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 7104.2-8028.6 ns)
   7104.2 |########################################
   7150.4 |
   7196.6 |
   7242.9 |########################################
   7289.1 |
   7335.3 |########################################
   7381.5 |
   7427.7 |
   7473.9 |
   7520.2 |
   7566.4 |
   7612.6 |
   7658.8 |
   7705.0 |
   7751.2 |########################################
   7797.5 |
   7843.7 |
   7889.9 |########################################
   7936.1 |
   7982.3 |
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 10775.8-11245.0 ns)
  10775.8 |########################################
  10799.3 |########################################
  10822.7 |
  10846.2 |
  10869.6 |
  10893.1 |
  10916.6 |
  10940.0 |
  10963.5 |
  10986.9 |
  11010.4 |
  11033.9 |
  11057.3 |
  11080.8 |
  11104.2 |
  11127.7 |########################################
  11151.2 |########################################
  11174.6 |########################################
  11198.1 |
  11221.5 |
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 11296.2-12716.9 ns)
  11296.2 |########################################
  11367.2 |
  11438.3 |
  11509.3 |
  11580.3 |
  11651.4 |
  11722.4 |
  11793.4 |
  11864.5 |
  11935.5 |
  12006.5 |
  12077.6 |
  12148.6 |
  12219.6 |
  12290.7 |
  12361.7 |
  12432.7 |
  12503.8 |
  12574.8 |########################################
  12645.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=710.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: bridge=684.5% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=812.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=806.4% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=400.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=1165.3% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: bridge=805.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=710.2% of algo (FFI overhead may distort results)
