# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 43% faster than the next best (carrier_disp_real_switch)

carrier_disp_real_nullfloor (28.77 us) leads carrier_disp_real_switch (41.08 us) by 43%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 30% (significant)

carrier_disp_real_nullfloor is -12.27 us (30%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_ifchainlin is an outlier: 3.0x slower than the field

carrier_disp_real_ifchainlin (87.02 us) is 3.0x the fastest (28.77 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_ifchainlin shows alternating (throttle bounce) (autocorr -0.64)

carrier_disp_real_ifchainlin's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_real_nullfloor, carrier_disp_real_switch, carrier_disp_real_ifchain, carrier_disp_real_ifchainasc, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} vs {carrier_disp_real_ifchainlin} (58% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor, carrier_disp_real_switch, carrier_disp_real_ifchain, carrier_disp_real_ifchainasc, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} and a slow tier {carrier_disp_real_ifchainlin} with a 58% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.0x the fastest

Fastest carrier_disp_real_nullfloor (28.77 us) to slowest carrier_disp_real_ifchainlin (87.02 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 28770.2 ns median (-30.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 3.02x (fastest 28770.2 ns, slowest 87015.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 52344ns | 51572ns | 50777ns | 51359ns | 54606ns | +19.42% |
| carrier_disp_real_fntable | 58044ns | 57282ns | 55492ns | 56924ns | 60998ns | +32.43% |
| carrier_disp_real_ifchain | 43484ns | 43396ns | 42480ns | 43151ns | 44486ns | -0.79% |
| carrier_disp_real_ifchainasc | 47434ns | 44401ns | 43612ns | 44163ns | 54252ns | +8.22% |
| carrier_disp_real_ifchainlin | 89992ns | 89586ns | 86136ns | 88696ns | 93865ns | +105.31% |
| carrier_disp_real_nullfloor | 31350ns | 31086ns | 30760ns | 31017ns | 32145ns | -28.48% |
| carrier_disp_real_switch | 43831ns | 43536ns | 42856ns | 43340ns | 45055ns | base |
| carrier_disp_real_threaded | 48780ns | 48542ns | 47259ns | 48132ns | 50511ns | +11.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 49903ns | 48388ns | 52016ns | +20.58% | 0.021 |
| carrier_disp_real_fntable | 55666ns | 53226ns | 58611ns | +34.50% | 0.018 |
| carrier_disp_real_ifchain | 41089ns | 40193ns | 41970ns | -0.72% | 0.025 |
| carrier_disp_real_ifchainasc | 45008ns | 41034ns | 51810ns | +8.75% | 0.023 |
| carrier_disp_real_ifchainlin | 87406ns | 83415ns | 91292ns | +111.20% | 0.012 |
| carrier_disp_real_nullfloor | 28961ns | 28353ns | 29690ns | -30.02% | 0.035 |
| carrier_disp_real_switch | 41386ns | 40452ns | 42478ns | base | 0.025 |
| carrier_disp_real_threaded | 46441ns | 44890ns | 48085ns | +12.21% | 0.022 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.021 | 57.7% |
| carrier_disp_real_fntable | 0.019 | 51.6% |
| carrier_disp_real_ifchain | 0.025 | 69.0% |
| carrier_disp_real_ifchainasc | 0.024 | 67.5% |
| carrier_disp_real_ifchainlin | 0.012 | 32.6% |
| carrier_disp_real_nullfloor | 0.036 | 98.5% |
| carrier_disp_real_switch | 0.025 | 69.0% |
| carrier_disp_real_threaded | 0.022 | 61.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 52344ns | 52344ns | +19.42% |
| carrier_disp_real_fntable | 58044ns | 58044ns | +32.43% |
| carrier_disp_real_ifchain | 43484ns | 43484ns | -0.79% |
| carrier_disp_real_ifchainasc | 47434ns | 47434ns | +8.22% |
| carrier_disp_real_ifchainlin | 89992ns | 89992ns | +105.31% |
| carrier_disp_real_nullfloor | 31350ns | 31350ns | -28.48% |
| carrier_disp_real_switch | 43831ns | 43831ns | base |
| carrier_disp_real_threaded | 48780ns | 48780ns | +11.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 41078ns | base | --- | [40602, 42478] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 49168ns | +8124.6ns (+19.8%) | [+7468, +9957]ns | [48524, 52016] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_fntable | 54903ns | +13479.2ns (+32.8%) | [+12195, +17165]ns | [53483, 58611] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_ifchain | 41081ns | no significant difference | [-1803, +851]ns | [40217, 41970] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchainasc | 42004ns | no significant difference | [-984, +10732]ns | [41209, 51810] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_real_ifchainlin | 87015ns | +45892.8ns (+111.7%) | [+41512, +50655]ns | [83911, 91292] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_nullfloor | 28770ns | -12272.9ns (-29.9%) | [-14056, -10947]ns | [28421, 29690] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_threaded | 46290ns | +4719.0ns (+11.5%) | [+3976, +6469]ns | [44948, 48085] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 43295ns | +21.6% | +28.9% | -5.8% | -5.2% | +95.3% | -34.2% | +9.0% |
| 2 | 40753ns | +20.0% | +33.7% | +1.9% | +1.5% | +119.5% | -29.1% | +12.0% |
| 3 | 40452ns | +19.6% | +32.9% | +2.3% | +4.0% | +121.9% | -26.9% | +11.3% |
| 4 | 41333ns | +19.6% | +28.8% | -2.6% | +45.7% | +104.2% | -30.7% | +18.5% |
| 5 | 40822ns | +25.9% | +50.5% | -1.5% | +6.3% | +127.4% | -27.0% | +10.0% |
| 6 | 41661ns | +16.8% | +32.8% | +1.8% | +0.7% | +100.2% | -31.9% | +12.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | -0.207 | moderate- |
| carrier_disp_real_fntable | -0.206 | moderate- |
| carrier_disp_real_ifchain | -0.186 | ok |
| carrier_disp_real_ifchainasc | -0.140 | ok |
| carrier_disp_real_ifchainlin | -0.637 | HIGH- (thermal bounce) |
| carrier_disp_real_nullfloor | -0.557 | HIGH- (thermal bounce) |
| carrier_disp_real_switch | -0.130 | ok |
| carrier_disp_real_threaded | -0.631 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 0/6, lost 6/6
- **carrier_disp_real_fntable**: won 0/6, lost 6/6
- **carrier_disp_real_ifchain**: won 3/6, lost 3/6
- **carrier_disp_real_ifchainasc**: won 1/6, lost 5/6
- **carrier_disp_real_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 99954.7ns | 49902.7ns | 200.3% | HIGH |
| carrier_disp_real_fntable | 110950.7ns | 55665.8ns | 199.3% | HIGH |
| carrier_disp_real_ifchain | 122023.7ns | 41089.3ns | 297.0% | HIGH |
| carrier_disp_real_ifchainasc | 124293.2ns | 45007.6ns | 276.2% | HIGH |
| carrier_disp_real_ifchainlin | 127598.0ns | 87406.0ns | 146.0% | HIGH |
| carrier_disp_real_nullfloor | 97457.4ns | 28960.6ns | 336.5% | HIGH |
| carrier_disp_real_switch | 121772.8ns | 41386.1ns | 294.2% | HIGH |
| carrier_disp_real_threaded | 93231.8ns | 46440.6ns | 200.8% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 48387.5-52016.1 ns)
  48387.5 |########################################
  48568.9 |########################################
  48750.4 |########################################
  48931.8 |
  49113.2 |
  49294.6 |########################################
  49476.1 |
  49657.5 |
  49838.9 |
  50020.3 |
  50201.8 |
  50383.2 |
  50564.6 |
  50746.1 |
  50927.5 |
  51108.9 |
  51290.3 |########################################
  51471.8 |
  51653.2 |
  51834.6 |
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 53225.8-58611.1 ns)
  53225.8 |########################################
  53495.1 |########################################
  53764.3 |
  54033.6 |
  54302.9 |########################################
  54572.1 |
  54841.4 |
  55110.6 |########################################
  55379.9 |
  55649.2 |########################################
  55918.4 |
  56187.7 |
  56457.0 |
  56726.2 |
  56995.5 |
  57264.7 |
  57534.0 |
  57803.3 |
  58072.5 |
  58341.8 |
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 40193.3-41970.2 ns)
  40193.3 |########################################
  40282.1 |
  40371.0 |
  40459.8 |
  40548.7 |
  40637.5 |
  40726.4 |####################
  40815.2 |
  40904.1 |
  40992.9 |
  41081.8 |
  41170.6 |
  41259.4 |
  41348.3 |####################
  41437.1 |
  41526.0 |####################
  41614.8 |
  41703.7 |
  41792.5 |
  41881.4 |
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 41034.2-51810.2 ns)
  41034.2 |########################################
  41573.0 |########################################
  42111.8 |
  42650.6 |
  43189.4 |####################
  43728.2 |
  44267.0 |
  44805.8 |
  45344.6 |
  45883.4 |
  46422.2 |
  46961.0 |
  47499.8 |
  48038.6 |
  48577.4 |
  49116.2 |
  49655.0 |
  50193.8 |
  50732.6 |
  51271.4 |
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 83414.6-91291.9 ns)
  83414.6 |####################
  83808.5 |
  84202.3 |########################################
  84596.2 |
  84990.1 |
  85383.9 |
  85777.8 |
  86171.6 |
  86565.5 |
  86959.4 |
  87353.2 |
  87747.1 |
  88141.0 |
  88534.8 |
  88928.7 |
  89322.5 |####################
  89716.4 |####################
  90110.3 |
  90504.1 |
  90898.0 |
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 28352.9-29690.0 ns)
  28352.9 |########################################
  28419.8 |
  28486.6 |########################################
  28553.5 |
  28620.3 |########################################
  28687.2 |
  28754.0 |
  28820.9 |
  28887.7 |########################################
  28954.6 |
  29021.5 |
  29088.3 |
  29155.2 |
  29222.0 |
  29288.9 |
  29355.7 |
  29422.6 |
  29489.4 |
  29556.3 |########################################
  29623.1 |
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 40452.1-42477.9 ns)
  40452.1 |########################################
  40553.4 |
  40654.7 |########################################
  40756.0 |########################################
  40857.3 |
  40958.6 |
  41059.8 |
  41161.1 |
  41262.4 |########################################
  41363.7 |
  41465.0 |
  41566.3 |########################################
  41667.6 |
  41768.9 |
  41870.2 |
  41971.4 |
  42072.7 |
  42174.0 |
  42275.3 |
  42376.6 |
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 44889.6-48084.6 ns)
  44889.6 |########################################
  45049.3 |
  45209.1 |
  45368.8 |
  45528.6 |####################
  45688.3 |
  45848.1 |
  46007.8 |
  46167.6 |
  46327.3 |
  46487.1 |
  46646.8 |
  46806.6 |####################
  46966.3 |
  47126.1 |####################
  47285.8 |
  47445.6 |
  47605.3 |
  47765.1 |
  47924.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=200.5% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=298.4% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=296.5% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=148.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=327.4% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=295.8% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=200.3% of algo (FFI overhead may distort results)
