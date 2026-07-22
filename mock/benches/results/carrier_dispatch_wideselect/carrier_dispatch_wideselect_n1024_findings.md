# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 37% faster than the next best (carrier_disp_wideselect_ifchainasc)

carrier_disp_wideselect_nullfloor (30.52 us) leads carrier_disp_wideselect_ifchainasc (41.73 us) by 37%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 29% (significant)

carrier_disp_wideselect_nullfloor is -12.48 us (29%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 3.3x slower than the field

carrier_disp_wideselect_ifchainlin (99.57 us) is 3.3x the fastest (30.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_wideselect_threaded shows alternating (throttle bounce) (autocorr -0.77)

carrier_disp_wideselect_threaded's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} vs {carrier_disp_wideselect_ifchainlin} (89% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} and a slow tier {carrier_disp_wideselect_ifchainlin} with a 89% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest carrier_disp_wideselect_nullfloor (30.52 us) to slowest carrier_disp_wideselect_ifchainlin (99.57 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 30517.7 ns median (-28.7% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 3.26x (fastest 30517.7 ns, slowest 99570.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 50750ns | 50878ns | 49625ns | 50629ns | 51495ns | +13.45% |
| carrier_disp_wideselect_fntable | 56090ns | 55343ns | 54381ns | 55171ns | 58324ns | +25.39% |
| carrier_disp_wideselect_ifchain | 46758ns | 46940ns | 45890ns | 46633ns | 47379ns | +4.52% |
| carrier_disp_wideselect_ifchainasc | 44019ns | 44368ns | 42081ns | 43716ns | 45442ns | -1.60% |
| carrier_disp_wideselect_ifchainlin | 103511ns | 102208ns | 101529ns | 101994ns | 106779ns | +131.39% |
| carrier_disp_wideselect_nullfloor | 32685ns | 33009ns | 30867ns | 32536ns | 33816ns | -26.94% |
| carrier_disp_wideselect_switch | 44734ns | 45151ns | 41036ns | 45096ns | 46040ns | base |
| carrier_disp_wideselect_threaded | 49236ns | 49330ns | 45235ns | 48887ns | 51762ns | +10.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 48238ns | 47111ns | 48901ns | +14.16% | 0.021 |
| carrier_disp_wideselect_fntable | 53608ns | 51864ns | 55837ns | +26.87% | 0.019 |
| carrier_disp_wideselect_ifchain | 44253ns | 43467ns | 44889ns | +4.73% | 0.023 |
| carrier_disp_wideselect_ifchainasc | 41519ns | 39536ns | 42964ns | -1.74% | 0.025 |
| carrier_disp_wideselect_ifchainlin | 100922ns | 98846ns | 104278ns | +138.84% | 0.010 |
| carrier_disp_wideselect_nullfloor | 30224ns | 28523ns | 31334ns | -28.47% | 0.034 |
| carrier_disp_wideselect_switch | 42255ns | 38866ns | 43413ns | base | 0.024 |
| carrier_disp_wideselect_threaded | 46725ns | 42909ns | 49087ns | +10.58% | 0.022 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.021 | 58.9% |
| carrier_disp_wideselect_fntable | 0.019 | 54.0% |
| carrier_disp_wideselect_ifchain | 0.023 | 64.3% |
| carrier_disp_wideselect_ifchainasc | 0.025 | 68.4% |
| carrier_disp_wideselect_ifchainlin | 0.010 | 28.6% |
| carrier_disp_wideselect_nullfloor | 0.034 | 93.5% |
| carrier_disp_wideselect_switch | 0.024 | 66.6% |
| carrier_disp_wideselect_threaded | 0.022 | 61.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 50750ns | 50750ns | +13.45% |
| carrier_disp_wideselect_fntable | 56090ns | 56090ns | +25.39% |
| carrier_disp_wideselect_ifchain | 46758ns | 46758ns | +4.52% |
| carrier_disp_wideselect_ifchainasc | 44019ns | 44019ns | -1.60% |
| carrier_disp_wideselect_ifchainlin | 103511ns | 103511ns | +131.39% |
| carrier_disp_wideselect_nullfloor | 32685ns | 32685ns | -26.94% |
| carrier_disp_wideselect_switch | 44734ns | 44734ns | base |
| carrier_disp_wideselect_threaded | 49236ns | 49236ns | +10.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 42806ns | base | --- | [40545, 43413] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 48400ns | +5514.0ns (+12.9%) | [+4301, +8135]ns | [47414, 48901] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 52803ns | +10257.8ns (+24.0%) | [+9513, +14288]ns | [52183, 55837] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchain | 44363ns | +1304.2ns (+3.0%) | [+902, +3790]ns | [43508, 44889] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchainasc | 41729ns | no significant difference | [-2578, +1124]ns | [39864, 42964] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_wideselect_ifchainlin | 99571ns | +58703.1ns (+137.1%) | [+56346, +60952]ns | [98916, 104278] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 30518ns | -12480.6ns (-29.2%) | [-13999, -9613]ns | [28820, 31334] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 46784ns | +4628.0ns (+10.8%) | [+1249, +7533]ns | [44303, 49087] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 42696ns | +14.6% | +23.1% | +4.3% | -7.4% | +134.4% | -33.2% | +9.1% |
| 2 | 42915ns | +11.2% | +23.9% | +1.3% | -4.7% | +130.7% | -29.0% | +12.5% |
| 3 | 42225ns | +14.8% | +24.3% | +3.1% | +2.2% | +134.1% | -25.9% | +1.6% |
| 4 | 42942ns | +9.7% | +36.3% | +2.9% | -0.9% | +143.7% | -32.2% | +16.2% |
| 5 | 43884ns | +10.1% | +20.8% | +2.9% | -2.5% | +136.7% | -28.5% | +4.1% |
| 6 | 38866ns | +25.8% | +33.4% | +14.8% | +3.4% | +154.8% | -21.4% | +20.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | -0.320 | moderate- |
| carrier_disp_wideselect_fntable | -0.210 | moderate- |
| carrier_disp_wideselect_ifchain | 0.300 | moderate+ |
| carrier_disp_wideselect_ifchainasc | 0.131 | ok |
| carrier_disp_wideselect_ifchainlin | 0.096 | ok |
| carrier_disp_wideselect_nullfloor | -0.328 | moderate- |
| carrier_disp_wideselect_switch | -0.272 | moderate- |
| carrier_disp_wideselect_threaded | -0.774 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_wideselect_bittree**: won 0/6, lost 6/6
- **carrier_disp_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchain**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_wideselect_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_wideselect_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_wideselect_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 96662.9ns | 48238.0ns | 200.4% | HIGH |
| carrier_disp_wideselect_fntable | 108009.1ns | 53607.6ns | 201.5% | HIGH |
| carrier_disp_wideselect_ifchain | 88712.6ns | 44253.5ns | 200.5% | HIGH |
| carrier_disp_wideselect_ifchainasc | 113326.0ns | 41518.9ns | 273.0% | HIGH |
| carrier_disp_wideselect_ifchainlin | 101133.0ns | 100921.6ns | 100.2% | HIGH |
| carrier_disp_wideselect_nullfloor | 91561.0ns | 30224.0ns | 302.9% | HIGH |
| carrier_disp_wideselect_switch | 106413.6ns | 42254.8ns | 251.8% | HIGH |
| carrier_disp_wideselect_threaded | 93622.9ns | 46724.7ns | 200.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 47110.8-48900.8 ns)
  47110.8 |########################################
  47200.3 |
  47289.8 |
  47379.3 |
  47468.8 |
  47558.3 |
  47647.8 |########################################
  47737.3 |
  47826.8 |
  47916.3 |
  48005.8 |
  48095.3 |
  48184.8 |
  48274.3 |########################################
  48363.8 |
  48453.3 |########################################
  48542.8 |
  48632.3 |
  48721.8 |
  48811.3 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 51864.2-55836.9 ns)
  51864.2 |####################
  52062.8 |
  52261.5 |
  52460.1 |########################################
  52658.7 |
  52857.4 |####################
  53056.0 |####################
  53254.6 |
  53453.3 |
  53651.9 |
  53850.6 |
  54049.2 |
  54247.8 |
  54446.5 |
  54645.1 |
  54843.7 |
  55042.4 |
  55241.0 |
  55439.6 |
  55638.3 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 43467.1-44889.2 ns)
  43467.1 |########################################
  43538.2 |########################################
  43609.3 |
  43680.4 |
  43751.5 |
  43822.6 |
  43893.7 |
  43964.8 |
  44035.9 |
  44107.0 |
  44178.1 |########################################
  44249.2 |
  44320.3 |
  44391.4 |
  44462.5 |########################################
  44533.6 |
  44604.7 |########################################
  44675.8 |
  44746.9 |
  44818.0 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 39536.2-42963.9 ns)
  39536.2 |########################################
  39707.6 |
  39879.0 |
  40050.4 |########################################
  40221.8 |
  40393.1 |
  40564.5 |
  40735.9 |
  40907.3 |########################################
  41078.7 |
  41250.1 |
  41421.5 |
  41592.8 |
  41764.2 |
  41935.6 |
  42107.0 |
  42278.4 |
  42449.8 |########################################
  42621.2 |########################################
  42792.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 98846.2-104278.3 ns)
  98846.2 |########################################
  99117.8 |
  99389.4 |
  99661.0 |
  99932.6 |#############
  100204.2 |
  100475.8 |
  100747.4 |
  101019.0 |
  101290.6 |
  101562.2 |
  101833.9 |
  102105.5 |
  102377.1 |
  102648.7 |
  102920.3 |
  103191.9 |
  103463.5 |
  103735.1 |#############
  104006.7 |
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 28522.9-31334.2 ns)
  28522.9 |########################################
  28663.5 |
  28804.0 |
  28944.6 |
  29085.2 |########################################
  29225.7 |
  29366.3 |
  29506.9 |
  29647.4 |
  29788.0 |
  29928.5 |
  30069.1 |
  30209.7 |
  30350.2 |########################################
  30490.8 |########################################
  30631.4 |
  30771.9 |
  30912.5 |
  31053.1 |
  31193.6 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 38865.8-43413.3 ns)
  38865.8 |####################
  39093.2 |
  39320.6 |
  39547.9 |
  39775.3 |
  40002.7 |
  40230.1 |
  40457.4 |
  40684.8 |
  40912.2 |
  41139.6 |
  41367.0 |
  41594.3 |
  41821.7 |
  42049.1 |####################
  42276.5 |
  42503.8 |####################
  42731.2 |########################################
  42958.6 |
  43186.0 |
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 42909.2-49087.3 ns)
  42909.2 |########################################
  43218.1 |
  43527.0 |
  43835.9 |
  44144.8 |
  44453.7 |
  44762.6 |
  45071.5 |
  45380.4 |
  45689.3 |########################################
  45998.2 |
  46307.2 |########################################
  46616.1 |
  46925.0 |########################################
  47233.9 |
  47542.8 |
  47851.7 |
  48160.6 |########################################
  48469.5 |
  48778.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: bridge=200.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=203.4% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=271.9% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=300.3% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=246.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=200.3% of algo (FFI overhead may distort results)
