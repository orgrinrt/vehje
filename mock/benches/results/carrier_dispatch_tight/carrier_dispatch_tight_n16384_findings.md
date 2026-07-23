# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 43% faster than the next best (carrier_disp_tight_bittree)

carrier_disp_tight_nullfloor (511.01 us) leads carrier_disp_tight_bittree (729.82 us) by 43%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_nullfloor beats baseline by 39% (significant)

carrier_disp_tight_nullfloor is -329.21 us (39%) faster than baseline carrier_disp_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_tight_nullfloor shows alternating (throttle bounce) (autocorr -0.51)

carrier_disp_tight_nullfloor's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_tight_nullfloor} vs {carrier_disp_tight_bittree, carrier_disp_tight_switch, carrier_disp_tight_ifchainasc, carrier_disp_tight_ifchain, carrier_disp_tight_fntable, carrier_disp_tight_threaded, carrier_disp_tight_ifchainlin} (43% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor} and a slow tier {carrier_disp_tight_bittree, carrier_disp_tight_switch, carrier_disp_tight_ifchainasc, carrier_disp_tight_ifchain, carrier_disp_tight_fntable, carrier_disp_tight_threaded, carrier_disp_tight_ifchainlin} with a 43% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 511012.1 ns median (-38.9% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.92x (fastest 511012.1 ns, slowest 980073.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 732357ns | 732789ns | 727924ns | 732741ns | 733997ns | -12.96% |
| carrier_disp_tight_fntable | 953374ns | 954296ns | 944818ns | 951137ns | 961006ns | +13.30% |
| carrier_disp_tight_ifchain | 852589ns | 851692ns | 846298ns | 850105ns | 859462ns | +1.32% |
| carrier_disp_tight_ifchainasc | 842657ns | 841115ns | 838197ns | 840459ns | 848184ns | +0.14% |
| carrier_disp_tight_ifchainlin | 984815ns | 983432ns | 978573ns | 982875ns | 990847ns | +17.04% |
| carrier_disp_tight_nullfloor | 512446ns | 513418ns | 504892ns | 510637ns | 518936ns | -39.10% |
| carrier_disp_tight_switch | 841441ns | 839602ns | 834001ns | 839034ns | 848772ns | base |
| carrier_disp_tight_threaded | 975087ns | 975822ns | 969104ns | 973657ns | 980224ns | +15.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 729118ns | 724369ns | 730826ns | -13.03% | 0.022 |
| carrier_disp_tight_fntable | 950319ns | 941186ns | 958320ns | +13.36% | 0.017 |
| carrier_disp_tight_ifchain | 849468ns | 843046ns | 856878ns | +1.33% | 0.019 |
| carrier_disp_tight_ifchainasc | 839322ns | 834248ns | 845482ns | +0.12% | 0.020 |
| carrier_disp_tight_ifchainlin | 981639ns | 975116ns | 988146ns | +17.09% | 0.017 |
| carrier_disp_tight_nullfloor | 509743ns | 501496ns | 516550ns | -39.20% | 0.032 |
| carrier_disp_tight_switch | 838335ns | 830360ns | 846205ns | base | 0.020 |
| carrier_disp_tight_threaded | 972055ns | 965287ns | 977343ns | +15.95% | 0.017 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 4576578 | 20272629 | 0.226 | 0.87× |
| carrier_disp_tight_fntable | 5971370 | 27014908 | 0.221 | 1.14× |
| carrier_disp_tight_ifchain | 5311891 | 19667144 | 0.270 | 1.01× |
| carrier_disp_tight_ifchainasc | 5270621 | 19667079 | 0.268 | 1.00× |
| carrier_disp_tight_ifchainlin | 6151707 | 28986914 | 0.212 | 1.17× |
| carrier_disp_tight_nullfloor | 3170336 | 17027604 | 0.186 | 0.60× |
| carrier_disp_tight_switch | 5248810 | 19142729 | 0.274 | 1.00× |
| carrier_disp_tight_threaded | 6058174 | 26482597 | 0.229 | 1.15× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.022 | 68.7% |
| carrier_disp_tight_fntable | 0.017 | 52.7% |
| carrier_disp_tight_ifchain | 0.019 | 59.1% |
| carrier_disp_tight_ifchainasc | 0.020 | 59.9% |
| carrier_disp_tight_ifchainlin | 0.017 | 51.2% |
| carrier_disp_tight_nullfloor | 0.032 | 98.1% |
| carrier_disp_tight_switch | 0.020 | 59.9% |
| carrier_disp_tight_threaded | 0.017 | 51.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 732357ns | 732357ns | -12.96% |
| carrier_disp_tight_fntable | 953374ns | 953374ns | +13.30% |
| carrier_disp_tight_ifchain | 852589ns | 852589ns | +1.32% |
| carrier_disp_tight_ifchainasc | 842657ns | 842657ns | +0.14% |
| carrier_disp_tight_ifchainlin | 984815ns | 984815ns | +17.04% |
| carrier_disp_tight_nullfloor | 512446ns | 512446ns | -39.10% |
| carrier_disp_tight_switch | 841441ns | 841441ns | base |
| carrier_disp_tight_threaded | 975087ns | 975087ns | +15.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 836544ns | base | --- | [832255, 846205] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 729824ns | -109566.0ns (-13.1%) | [-116006, -102077]ns | [726705, 730826] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_fntable | 951314ns | +112181.2ns (+13.4%) | [+98249, +125521]ns | [941323, 958320] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 848427ns | no significant difference | [-1240, +21124]ns | [843098, 856878] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_tight_ifchainasc | 837675ns | no significant difference | [-11397, +9691]ns | [834808, 845482] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_tight_ifchainlin | 980074ns | +144983.5ns (+17.3%) | [+130888, +154041]ns | [976697, 988146] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 511012ns | -329214.6ns (-39.4%) | [-333991, -322570]ns | [501667, 516550] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 973307ns | +130011.9ns (+15.5%) | [+128737, +142412]ns | [965515, 977343] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 837850ns | -12.9% | +12.4% | +2.7% | +1.5% | +18.1% | -38.4% | +15.3% |
| 2 | 830360ns | -12.1% | +15.4% | +2.4% | +0.8% | +18.8% | -39.6% | +16.9% |
| 3 | 835238ns | -13.3% | +14.8% | +0.9% | +0.4% | +17.1% | -38.7% | +15.6% |
| 4 | 846502ns | -13.9% | +11.2% | +0.0% | -1.4% | +15.7% | -38.9% | +15.3% |
| 5 | 834150ns | -12.5% | +14.5% | +2.3% | +0.8% | +17.6% | -39.8% | +17.3% |
| 6 | 845909ns | -13.5% | +12.0% | -0.3% | -1.2% | +15.3% | -39.7% | +15.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | -0.037 | ok |
| carrier_disp_tight_fntable | -0.421 | moderate- |
| carrier_disp_tight_ifchain | -0.068 | ok |
| carrier_disp_tight_ifchainasc | -0.160 | ok |
| carrier_disp_tight_ifchainlin | 0.247 | moderate+ |
| carrier_disp_tight_nullfloor | -0.509 | HIGH- (thermal bounce) |
| carrier_disp_tight_switch | -0.291 | moderate- |
| carrier_disp_tight_threaded | 0.258 | moderate+ |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 6/6, lost 0/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 1/6, lost 4/6
- **carrier_disp_tight_ifchainasc**: won 2/6, lost 4/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 730316.4ns | 729118.2ns | 100.2% | HIGH |
| carrier_disp_tight_fntable | 963400.0ns | 950318.6ns | 101.4% | HIGH |
| carrier_disp_tight_ifchain | 850901.5ns | 849467.8ns | 100.2% | HIGH |
| carrier_disp_tight_ifchainasc | 840796.5ns | 839321.8ns | 100.2% | HIGH |
| carrier_disp_tight_ifchainlin | 983147.3ns | 981638.9ns | 100.2% | HIGH |
| carrier_disp_tight_nullfloor | 510676.6ns | 509743.0ns | 100.2% | HIGH |
| carrier_disp_tight_switch | 839416.9ns | 838334.8ns | 100.1% | HIGH |
| carrier_disp_tight_threaded | 974776.0ns | 972055.0ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 724369.2-730826.1 ns)
  724369.2 |########################################
  724692.0 |
  725014.9 |
  725337.7 |
  725660.6 |
  725983.4 |
  726306.3 |
  726629.1 |
  726951.9 |
  727274.8 |
  727597.6 |
  727920.5 |
  728243.3 |
  728566.2 |
  728889.0 |########################################
  729211.8 |
  729534.7 |########################################
  729857.5 |########################################
  730180.4 |########################################
  730503.2 |
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 941186.2-958319.6 ns)
  941186.2 |########################################
  942042.9 |
  942899.5 |
  943756.2 |
  944612.9 |
  945469.5 |
  946326.2 |
  947182.9 |####################
  948039.6 |
  948896.2 |
  949752.9 |
  950609.6 |
  951466.2 |
  952322.9 |
  953179.6 |
  954036.2 |
  954892.9 |####################
  955749.6 |
  956606.3 |
  957462.9 |####################
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 843046.2-856878.3 ns)
  843046.2 |########################################
  843737.8 |
  844429.4 |
  845121.0 |
  845812.6 |
  846504.2 |####################
  847195.8 |
  847887.5 |
  848579.1 |
  849270.7 |
  849962.3 |####################
  850653.9 |
  851345.5 |
  852037.1 |
  852728.7 |####################
  853420.3 |
  854111.9 |
  854803.5 |
  855495.1 |
  856186.7 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 834248.3-845482.1 ns)
  834248.3 |########################################
  834810.0 |########################################
  835371.7 |
  835933.4 |
  836495.1 |########################################
  837056.8 |
  837618.4 |
  838180.1 |########################################
  838741.8 |
  839303.5 |
  839865.2 |########################################
  840426.9 |
  840988.6 |
  841550.3 |
  842112.0 |
  842673.7 |
  843235.3 |
  843797.0 |
  844358.7 |
  844920.4 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 975116.2-988145.6 ns)
  975116.2 |########################################
  975767.7 |
  976419.1 |
  977070.6 |
  977722.1 |########################################
  978373.6 |
  979025.0 |########################################
  979676.5 |
  980328.0 |
  980979.5 |########################################
  981630.9 |
  982282.4 |
  982933.9 |
  983585.3 |
  984236.8 |
  984888.3 |
  985539.8 |
  986191.2 |########################################
  986842.7 |
  987494.2 |
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 501496.2-516550.0 ns)
  501496.2 |########################################
  502248.9 |
  503001.6 |
  503754.3 |
  504507.0 |
  505259.7 |
  506012.3 |
  506765.0 |
  507517.7 |
  508270.4 |
  509023.1 |
  509775.8 |####################
  510528.5 |
  511281.2 |####################
  512033.9 |
  512786.5 |
  513539.2 |
  514291.9 |
  515044.6 |
  515797.3 |####################
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 830359.6-846205.4 ns)
  830359.6 |########################################
  831151.9 |
  831944.2 |
  832736.5 |
  833528.8 |########################################
  834321.1 |
  835113.4 |########################################
  835905.6 |
  836697.9 |
  837490.2 |########################################
  838282.5 |
  839074.8 |
  839867.1 |
  840659.4 |
  841451.7 |
  842244.0 |
  843036.3 |
  843828.6 |
  844620.9 |
  845413.2 |########################################
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 965286.7-977342.9 ns)
  965286.7 |########################################
  965889.5 |
  966492.3 |
  967095.1 |
  967697.9 |
  968300.8 |
  968903.6 |
  969506.4 |
  970109.2 |
  970712.0 |####################
  971314.8 |
  971917.6 |
  972520.4 |
  973123.3 |
  973726.1 |
  974328.9 |
  974931.7 |
  975534.5 |########################################
  976137.3 |
  976740.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=101.4% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=100.2% of algo (FFI overhead may distort results)
