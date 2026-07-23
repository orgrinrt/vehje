# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 36% faster than the next best (carrier_disp_real_ifchain)

carrier_disp_real_nullfloor (29.88 us) leads carrier_disp_real_ifchain (40.78 us) by 36%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 27% (significant)

carrier_disp_real_nullfloor is -11.25 us (27%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_ifchainlin is an outlier: 2.9x slower than the field

carrier_disp_real_ifchainlin (85.91 us) is 2.9x the fastest (29.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_ifchainlin shows alternating (throttle bounce) (autocorr -0.71)

carrier_disp_real_ifchainlin's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_real_nullfloor, carrier_disp_real_ifchain, carrier_disp_real_switch, carrier_disp_real_ifchainasc, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} vs {carrier_disp_real_ifchainlin} (61% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor, carrier_disp_real_ifchain, carrier_disp_real_switch, carrier_disp_real_ifchainasc, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} and a slow tier {carrier_disp_real_ifchainlin} with a 61% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 29881.2 ns median (-27.8% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.87x (fastest 29881.2 ns, slowest 85906.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 52438ns | 52113ns | 51238ns | 51849ns | 53921ns | +18.83% |
| carrier_disp_real_fntable | 61817ns | 55870ns | 54090ns | 55606ns | 74998ns | +40.09% |
| carrier_disp_real_ifchain | 43173ns | 42950ns | 42564ns | 42908ns | 43875ns | -2.16% |
| carrier_disp_real_ifchainasc | 43921ns | 44031ns | 42967ns | 43793ns | 44589ns | -0.47% |
| carrier_disp_real_ifchainlin | 89240ns | 88364ns | 86538ns | 87858ns | 92666ns | +102.24% |
| carrier_disp_real_nullfloor | 32270ns | 32268ns | 30776ns | 32106ns | 33265ns | -26.87% |
| carrier_disp_real_switch | 44127ns | 43766ns | 42609ns | 43534ns | 45775ns | base |
| carrier_disp_real_threaded | 48400ns | 48694ns | 47472ns | 48319ns | 48986ns | +9.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 50072ns | 48920ns | 51434ns | +19.98% | 0.020 |
| carrier_disp_real_fntable | 59388ns | 51674ns | 72522ns | +42.30% | 0.017 |
| carrier_disp_real_ifchain | 40892ns | 40304ns | 41508ns | -2.02% | 0.025 |
| carrier_disp_real_ifchainasc | 41554ns | 40507ns | 42165ns | -0.43% | 0.025 |
| carrier_disp_real_ifchainlin | 86864ns | 84365ns | 90174ns | +108.14% | 0.012 |
| carrier_disp_real_nullfloor | 29900ns | 28591ns | 30822ns | -28.36% | 0.034 |
| carrier_disp_real_switch | 41734ns | 40300ns | 43324ns | base | 0.025 |
| carrier_disp_real_threaded | 46132ns | 45298ns | 46666ns | +10.54% | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_real_bittree | 466298 | 1825888 | 0.255 | 0.93× |
| carrier_disp_real_fntable | 535516 | 2340407 | 0.229 | 1.07× |
| carrier_disp_real_ifchain | 513596 | 2342746 | 0.219 | 1.03× |
| carrier_disp_real_ifchainasc | 515777 | 2331320 | 0.221 | 1.03× |
| carrier_disp_real_ifchainlin | 681346 | 4005822 | 0.170 | 1.36× |
| carrier_disp_real_nullfloor | 374637 | 2079295 | 0.180 | 0.75× |
| carrier_disp_real_switch | 500644 | 2219438 | 0.226 | 1.00× |
| carrier_disp_real_threaded | 432169 | 2388788 | 0.181 | 0.86× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.021 | 57.4% |
| carrier_disp_real_fntable | 0.019 | 53.5% |
| carrier_disp_real_ifchain | 0.025 | 70.1% |
| carrier_disp_real_ifchainasc | 0.025 | 68.5% |
| carrier_disp_real_ifchainlin | 0.012 | 33.3% |
| carrier_disp_real_nullfloor | 0.034 | 95.7% |
| carrier_disp_real_switch | 0.025 | 69.1% |
| carrier_disp_real_threaded | 0.022 | 61.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 52438ns | 52438ns | +18.83% |
| carrier_disp_real_fntable | 61817ns | 61817ns | +40.09% |
| carrier_disp_real_ifchain | 43173ns | 43173ns | -2.16% |
| carrier_disp_real_ifchainasc | 43921ns | 43921ns | -0.47% |
| carrier_disp_real_ifchainlin | 89240ns | 89240ns | +102.24% |
| carrier_disp_real_nullfloor | 32270ns | 32270ns | -26.87% |
| carrier_disp_real_switch | 44127ns | 44127ns | base |
| carrier_disp_real_threaded | 48400ns | 48400ns | +9.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 41404ns | base | --- | [40473, 43324] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 49839ns | +8701.4ns (+21.0%) | [+6209, +10103]ns | [48942, 51434] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_fntable | 53438ns | +12097.1ns (+29.2%) | [+11302, +29564]ns | [52204, 72522] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_ifchain | 40778ns | no significant difference | [-2308, +382]ns | [40390, 41508] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_real_ifchainasc | 41716ns | no significant difference | [-2543, +1692]ns | [40781, 42165] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_real_ifchainlin | 85906ns | +44402.4ns (+107.2%) | [+41779, +49210]ns | [84512, 90174] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_nullfloor | 29881ns | -11252.9ns (-27.2%) | [-13738, -10512]ns | [28996, 30822] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_threaded | 46388ns | +4686.6ns (+11.3%) | [+2601, +5907]ns | [45342, 46666] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 42360ns | +23.1% | +27.8% | -1.8% | -4.4% | +103.6% | -26.7% | +9.9% |
| 2 | 40647ns | +21.0% | +27.1% | -0.8% | +3.0% | +110.5% | -27.6% | +11.7% |
| 3 | 41629ns | +21.3% | +46.2% | -2.8% | +0.5% | +119.1% | -26.5% | +11.1% |
| 4 | 44287ns | +10.6% | +90.1% | -7.8% | -7.3% | +90.5% | -33.6% | +2.3% |
| 5 | 40300ns | +25.9% | +30.9% | +2.7% | +5.4% | +121.2% | -24.8% | +15.4% |
| 6 | 41180ns | +18.8% | +28.1% | -1.1% | +1.1% | +105.6% | -30.6% | +13.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | -0.502 | HIGH- (thermal bounce) |
| carrier_disp_real_fntable | -0.069 | ok |
| carrier_disp_real_ifchain | -0.200 | moderate- |
| carrier_disp_real_ifchainasc | -0.329 | moderate- |
| carrier_disp_real_ifchainlin | -0.710 | HIGH- (thermal bounce) |
| carrier_disp_real_nullfloor | -0.468 | moderate- |
| carrier_disp_real_switch | -0.354 | moderate- |
| carrier_disp_real_threaded | -0.293 | moderate- |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 0/6, lost 6/6
- **carrier_disp_real_fntable**: won 0/6, lost 6/6
- **carrier_disp_real_ifchain**: won 5/6, lost 1/6
- **carrier_disp_real_ifchainasc**: won 2/6, lost 4/6
- **carrier_disp_real_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 100277.5ns | 50071.6ns | 200.3% | HIGH |
| carrier_disp_real_fntable | 112302.1ns | 59388.1ns | 189.1% | HIGH |
| carrier_disp_real_ifchain | 122479.0ns | 40892.4ns | 299.5% | HIGH |
| carrier_disp_real_ifchainasc | 123471.3ns | 41554.0ns | 297.1% | HIGH |
| carrier_disp_real_ifchainlin | 133517.1ns | 86864.1ns | 153.7% | HIGH |
| carrier_disp_real_nullfloor | 93194.3ns | 29899.7ns | 311.7% | HIGH |
| carrier_disp_real_switch | 119942.0ns | 41733.8ns | 287.4% | HIGH |
| carrier_disp_real_threaded | 92373.7ns | 46131.9ns | 200.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 48920.0-51433.5 ns)
  48920.0 |########################################
  49045.7 |
  49171.3 |####################
  49297.0 |
  49422.7 |
  49548.4 |
  49674.1 |
  49799.7 |
  49925.4 |
  50051.1 |
  50176.8 |
  50302.4 |
  50428.1 |####################
  50553.8 |
  50679.4 |####################
  50805.1 |
  50930.8 |
  51056.5 |
  51182.2 |
  51307.8 |
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 51673.8-72522.1 ns)
  51673.8 |####################
  52716.2 |########################################
  53758.6 |####################
  54801.0 |
  55843.5 |
  56885.9 |
  57928.3 |
  58970.7 |
  60013.1 |####################
  61055.5 |
  62097.9 |
  63140.3 |
  64182.8 |
  65225.2 |
  66267.6 |
  67310.0 |
  68352.4 |
  69394.8 |
  70437.2 |
  71479.6 |
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 40304.2-41508.3 ns)
  40304.2 |########################################
  40364.4 |
  40424.6 |########################################
  40484.8 |
  40545.0 |
  40605.2 |
  40665.4 |
  40725.7 |########################################
  40785.9 |########################################
  40846.1 |
  40906.3 |
  40966.5 |
  41026.7 |
  41086.9 |
  41147.1 |
  41207.3 |
  41267.5 |
  41327.7 |
  41387.9 |########################################
  41448.1 |
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 40506.7-42165.4 ns)
  40506.7 |########################################
  40589.6 |
  40672.6 |
  40755.5 |
  40838.4 |
  40921.4 |
  41004.3 |########################################
  41087.2 |
  41170.2 |
  41253.1 |
  41336.1 |
  41419.0 |
  41501.9 |
  41584.9 |########################################
  41667.8 |
  41750.7 |########################################
  41833.7 |########################################
  41916.6 |
  41999.5 |
  42082.5 |
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 84365.0-90174.2 ns)
  84365.0 |########################################
  84655.5 |########################################
  84945.9 |
  85236.4 |
  85526.8 |########################################
  85817.3 |
  86107.8 |########################################
  86398.2 |
  86688.7 |
  86979.1 |
  87269.6 |
  87560.1 |
  87850.5 |
  88141.0 |
  88431.4 |
  88721.9 |
  89012.4 |########################################
  89302.8 |
  89593.3 |
  89883.7 |
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 28591.2-30822.1 ns)
  28591.2 |####################
  28702.7 |
  28814.3 |
  28925.8 |
  29037.4 |
  29148.9 |
  29260.5 |
  29372.0 |########################################
  29483.5 |
  29595.1 |
  29706.6 |
  29818.2 |
  29929.7 |
  30041.3 |
  30152.8 |
  30264.3 |####################
  30375.9 |
  30487.4 |####################
  30599.0 |
  30710.5 |
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 40300.0-43323.8 ns)
  40300.0 |########################################
  40451.2 |
  40602.4 |########################################
  40753.6 |
  40904.8 |
  41055.9 |########################################
  41207.1 |
  41358.3 |
  41509.5 |########################################
  41660.7 |
  41811.9 |
  41963.1 |
  42114.2 |
  42265.4 |########################################
  42416.6 |
  42567.8 |
  42719.0 |
  42870.2 |
  43021.4 |
  43172.6 |
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 45298.3-46665.8 ns)
  45298.3 |########################################
  45366.7 |########################################
  45435.1 |
  45503.4 |
  45571.8 |
  45640.2 |
  45708.6 |
  45776.9 |
  45845.3 |
  45913.7 |
  45982.1 |
  46050.4 |
  46118.8 |
  46187.2 |
  46255.6 |########################################
  46323.9 |
  46392.3 |
  46460.7 |########################################
  46529.1 |########################################
  46597.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=199.9% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=201.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=299.3% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=295.3% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=165.3% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=306.8% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=295.8% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=200.2% of algo (FFI overhead may distort results)
