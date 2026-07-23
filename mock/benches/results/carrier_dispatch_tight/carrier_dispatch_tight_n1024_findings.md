# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 36% faster than the next best (carrier_disp_tight_switch)

carrier_disp_tight_nullfloor (31.46 us) leads carrier_disp_tight_switch (42.83 us) by 36%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_nullfloor beats baseline by 27% (significant)

carrier_disp_tight_nullfloor is -11.62 us (27%) faster than baseline carrier_disp_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_tight_fntable shows alternating (throttle bounce) (autocorr -0.79)

carrier_disp_tight_fntable's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_tight_nullfloor} vs {carrier_disp_tight_switch, carrier_disp_tight_bittree, carrier_disp_tight_ifchain, carrier_disp_tight_ifchainasc, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} (36% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor} and a slow tier {carrier_disp_tight_switch, carrier_disp_tight_bittree, carrier_disp_tight_ifchain, carrier_disp_tight_ifchainasc, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_tight_ifchain's edge over baseline is significant but tiny (30 ns, 0.07%)

carrier_disp_tight_ifchain differs from baseline carrier_disp_tight_switch by 30 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 31458.0 ns median (-26.5% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.83x (fastest 31458.0 ns, slowest 57560.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 45691ns | 45150ns | 44392ns | 44985ns | 47399ns | -0.29% |
| carrier_disp_tight_fntable | 54374ns | 54377ns | 52156ns | 53676ns | 56529ns | +18.66% |
| carrier_disp_tight_ifchain | 45380ns | 45518ns | 44308ns | 45333ns | 45985ns | -0.97% |
| carrier_disp_tight_ifchainasc | 45882ns | 45762ns | 44483ns | 45440ns | 47245ns | +0.13% |
| carrier_disp_tight_ifchainlin | 60375ns | 59772ns | 58952ns | 59733ns | 62050ns | +31.75% |
| carrier_disp_tight_nullfloor | 34166ns | 33743ns | 33128ns | 33538ns | 35629ns | -25.44% |
| carrier_disp_tight_switch | 45824ns | 45083ns | 44748ns | 45013ns | 47579ns | base |
| carrier_disp_tight_threaded | 48901ns | 48417ns | 48086ns | 48332ns | 50161ns | +6.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 43332ns | 42216ns | 44926ns | -0.49% | 0.024 |
| carrier_disp_tight_fntable | 51992ns | 49920ns | 54003ns | +19.39% | 0.020 |
| carrier_disp_tight_ifchain | 43111ns | 42120ns | 43712ns | -1.00% | 0.024 |
| carrier_disp_tight_ifchainasc | 43619ns | 42246ns | 44935ns | +0.16% | 0.023 |
| carrier_disp_tight_ifchainlin | 58055ns | 56497ns | 59606ns | +33.31% | 0.018 |
| carrier_disp_tight_nullfloor | 31854ns | 30951ns | 33149ns | -26.85% | 0.032 |
| carrier_disp_tight_switch | 43548ns | 42515ns | 45229ns | base | 0.024 |
| carrier_disp_tight_threaded | 46629ns | 45915ns | 47788ns | +7.08% | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 475580 | 2245834 | 0.212 | 1.02× |
| carrier_disp_tight_fntable | 482164 | 2476218 | 0.195 | 1.04× |
| carrier_disp_tight_ifchain | 486403 | 2219568 | 0.219 | 1.05× |
| carrier_disp_tight_ifchainasc | 450436 | 2053067 | 0.219 | 0.97× |
| carrier_disp_tight_ifchainlin | 544078 | 2739866 | 0.199 | 1.17× |
| carrier_disp_tight_nullfloor | 393949 | 2127906 | 0.185 | 0.85× |
| carrier_disp_tight_switch | 464711 | 2051560 | 0.227 | 1.00× |
| carrier_disp_tight_threaded | 437818 | 2491842 | 0.176 | 0.94× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.024 | 72.2% |
| carrier_disp_tight_fntable | 0.020 | 59.5% |
| carrier_disp_tight_ifchain | 0.024 | 71.5% |
| carrier_disp_tight_ifchainasc | 0.024 | 71.2% |
| carrier_disp_tight_ifchainlin | 0.018 | 53.8% |
| carrier_disp_tight_nullfloor | 0.033 | 98.4% |
| carrier_disp_tight_switch | 0.024 | 72.3% |
| carrier_disp_tight_threaded | 0.022 | 67.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 45691ns | 45691ns | -0.29% |
| carrier_disp_tight_fntable | 54374ns | 54374ns | +18.66% |
| carrier_disp_tight_ifchain | 45380ns | 45380ns | -0.97% |
| carrier_disp_tight_ifchainasc | 45882ns | 45882ns | +0.13% |
| carrier_disp_tight_ifchainlin | 60375ns | 60375ns | +31.75% |
| carrier_disp_tight_nullfloor | 34166ns | 34166ns | -25.44% |
| carrier_disp_tight_switch | 45824ns | 45824ns | base |
| carrier_disp_tight_threaded | 48901ns | 48901ns | +6.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 42827ns | base | --- | [42586, 45229] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 42853ns | no significant difference | [-773, +395]ns | [42218, 44926] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_tight_fntable | 51988ns | +7616.6ns (+17.8%) | [+7192, +10524]ns | [49984, 54003] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 43282ns | no significant difference | [-2148, +808]ns | [42338, 43712] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_tight_ifchainasc | 43487ns | no significant difference | [-585, +865]ns | [42434, 44935] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_tight_ifchainlin | 57561ns | +14974.2ns (+35.0%) | [+12422, +16126]ns | [56999, 59606] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 31458ns | -11625.0ns (-27.1%) | [-13124, -10331]ns | [30956, 33149] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 46139ns | +3532.1ns (+8.2%) | [+1404, +4309]ns | [45960, 47788] | YES (adj: no) | 0.3828 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 42699ns | -0.6% | +17.2% | +2.6% | -0.2% | +34.7% | -27.5% | +8.2% |
| 2 | 42955ns | +0.7% | +25.4% | +1.2% | +3.2% | +38.4% | -22.9% | +8.3% |
| 3 | 42658ns | -1.0% | +18.4% | +1.1% | +0.9% | +34.9% | -27.4% | +7.6% |
| 4 | 46454ns | -2.4% | +15.1% | -6.1% | -1.9% | +21.6% | -31.2% | -1.0% |
| 5 | 42515ns | -0.7% | +17.4% | -0.9% | -0.6% | +35.4% | -27.2% | +8.4% |
| 6 | 44004ns | +1.1% | +23.0% | -3.3% | -0.1% | +35.8% | -24.6% | +11.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | -0.648 | HIGH- (thermal bounce) |
| carrier_disp_tight_fntable | -0.786 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchain | 0.141 | ok |
| carrier_disp_tight_ifchainasc | -0.709 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchainlin | -0.093 | ok |
| carrier_disp_tight_nullfloor | -0.629 | HIGH- (thermal bounce) |
| carrier_disp_tight_switch | -0.434 | moderate- |
| carrier_disp_tight_threaded | -0.057 | ok |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 4/6, lost 2/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 3/6, lost 3/6
- **carrier_disp_tight_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 109430.0ns | 43332.5ns | 252.5% | HIGH |
| carrier_disp_tight_fntable | 105069.6ns | 51991.5ns | 202.1% | HIGH |
| carrier_disp_tight_ifchain | 111825.8ns | 43111.0ns | 259.4% | HIGH |
| carrier_disp_tight_ifchainasc | 101349.5ns | 43618.7ns | 232.4% | HIGH |
| carrier_disp_tight_ifchainlin | 116281.5ns | 58055.0ns | 200.3% | HIGH |
| carrier_disp_tight_nullfloor | 95628.8ns | 31854.1ns | 300.2% | HIGH |
| carrier_disp_tight_switch | 105060.1ns | 43547.6ns | 241.3% | HIGH |
| carrier_disp_tight_threaded | 93329.7ns | 46629.0ns | 200.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 42216.2-44926.4 ns)
  42216.2 |########################################
  42351.7 |####################
  42487.2 |
  42622.7 |
  42758.2 |
  42893.8 |
  43029.3 |
  43164.8 |####################
  43300.3 |
  43435.8 |
  43571.3 |
  43706.8 |
  43842.3 |
  43977.9 |
  44113.4 |
  44248.9 |
  44384.4 |####################
  44519.9 |
  44655.4 |
  44790.9 |
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 49920.0-54003.2 ns)
  49920.0 |########################################
  50124.2 |
  50328.3 |####################
  50532.5 |
  50736.6 |
  50940.8 |
  51144.9 |
  51349.1 |
  51553.3 |
  51757.4 |
  51961.6 |
  52165.7 |
  52369.9 |
  52574.0 |
  52778.2 |
  52982.4 |
  53186.5 |
  53390.7 |####################
  53594.8 |
  53799.0 |####################
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 42120.4-43712.5 ns)
  42120.4 |########################################
  42200.0 |
  42279.6 |
  42359.2 |
  42438.8 |
  42518.4 |########################################
  42598.0 |
  42677.6 |
  42757.2 |
  42836.8 |
  42916.4 |
  42996.1 |
  43075.7 |########################################
  43155.3 |
  43234.9 |
  43314.5 |
  43394.1 |########################################
  43473.7 |
  43553.3 |########################################
  43632.9 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 42245.8-44935.2 ns)
  42245.8 |########################################
  42380.3 |
  42514.7 |########################################
  42649.2 |
  42783.7 |
  42918.2 |########################################
  43052.6 |
  43187.1 |
  43321.6 |
  43456.0 |
  43590.5 |
  43725.0 |
  43859.4 |########################################
  43993.9 |
  44128.4 |
  44262.8 |########################################
  44397.3 |
  44531.8 |
  44666.3 |
  44800.7 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 56497.1-59605.6 ns)
  56497.1 |#############
  56652.5 |
  56807.9 |
  56963.4 |
  57118.8 |
  57274.2 |
  57429.7 |########################################
  57585.1 |
  57740.5 |
  57895.9 |
  58051.3 |
  58206.8 |
  58362.2 |
  58517.6 |
  58673.0 |
  58828.5 |
  58983.9 |
  59139.3 |
  59294.8 |#############
  59450.2 |
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 30951.2-33148.6 ns)
  30951.2 |########################################
  31061.1 |
  31170.9 |
  31280.8 |
  31390.7 |
  31500.5 |
  31610.4 |
  31720.3 |
  31830.1 |
  31940.0 |#############
  32049.9 |
  32159.7 |
  32269.6 |
  32379.5 |
  32489.3 |
  32599.2 |
  32709.1 |
  32818.9 |
  32928.8 |
  33038.7 |#############
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 42515.0-45229.2 ns)
  42515.0 |####################
  42650.7 |########################################
  42786.4 |
  42922.1 |####################
  43057.8 |
  43193.6 |
  43329.3 |
  43465.0 |
  43600.7 |
  43736.4 |
  43872.1 |####################
  44007.8 |
  44143.5 |
  44279.2 |
  44414.9 |
  44550.7 |
  44686.4 |
  44822.1 |
  44957.8 |
  45093.5 |
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 45915.4-47788.1 ns)
  45915.4 |########################################
  46009.0 |####################
  46102.7 |####################
  46196.3 |
  46289.9 |
  46383.6 |
  46477.2 |####################
  46570.8 |
  46664.5 |
  46758.1 |
  46851.8 |
  46945.4 |
  47039.0 |
  47132.7 |
  47226.3 |
  47319.9 |
  47413.6 |
  47507.2 |
  47600.8 |
  47694.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=264.5% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=202.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=254.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=216.8% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=200.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=300.4% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=250.4% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=200.1% of algo (FFI overhead may distort results)
