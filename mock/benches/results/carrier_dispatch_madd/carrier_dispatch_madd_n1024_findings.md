# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_madd_threaded shows alternating (throttle bounce) (autocorr -0.84)

carrier_disp_madd_threaded's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 40862.1 ns median (-13.6% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.30x (fastest 40862.1 ns, slowest 53248.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 47345ns | 47192ns | 46065ns | 46839ns | 48743ns | -5.55% |
| carrier_disp_madd_fntable | 54810ns | 55050ns | 53085ns | 54711ns | 55821ns | +9.34% |
| carrier_disp_madd_ifchain | 50733ns | 50346ns | 49255ns | 50088ns | 52439ns | +1.21% |
| carrier_disp_madd_ifchainasc | 50130ns | 50119ns | 48965ns | 49948ns | 50986ns | +0.00% |
| carrier_disp_madd_ifchainlin | 46919ns | 46751ns | 45195ns | 46360ns | 48619ns | -6.40% |
| carrier_disp_madd_nullfloor | 43465ns | 43112ns | 42528ns | 42941ns | 44720ns | -13.29% |
| carrier_disp_madd_switch | 50128ns | 49476ns | 49220ns | 49402ns | 51673ns | base |
| carrier_disp_madd_threaded | 55631ns | 55465ns | 54061ns | 55045ns | 57296ns | +10.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 45094ns | 43889ns | 46443ns | -5.85% | 0.023 |
| carrier_disp_madd_fntable | 52508ns | 50656ns | 53552ns | +9.63% | 0.020 |
| carrier_disp_madd_ifchain | 48491ns | 47085ns | 50136ns | +1.24% | 0.021 |
| carrier_disp_madd_ifchainasc | 47915ns | 46808ns | 48746ns | +0.04% | 0.021 |
| carrier_disp_madd_ifchainlin | 44662ns | 43037ns | 46247ns | -6.75% | 0.023 |
| carrier_disp_madd_nullfloor | 41134ns | 40168ns | 42278ns | -14.12% | 0.025 |
| carrier_disp_madd_switch | 47896ns | 47054ns | 49331ns | base | 0.021 |
| carrier_disp_madd_threaded | 53365ns | 51846ns | 54910ns | +11.42% | 0.019 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 418603 | 1878047 | 0.223 | 0.93× |
| carrier_disp_madd_fntable | 493098 | 2475758 | 0.199 | 1.10× |
| carrier_disp_madd_ifchain | 448950 | 1854577 | 0.242 | 1.00× |
| carrier_disp_madd_ifchainasc | 449310 | 1853591 | 0.242 | 1.00× |
| carrier_disp_madd_ifchainlin | 417360 | 1984791 | 0.210 | 0.93× |
| carrier_disp_madd_nullfloor | 499443 | 2088596 | 0.239 | 1.11× |
| carrier_disp_madd_switch | 448925 | 1805502 | 0.249 | 1.00× |
| carrier_disp_madd_threaded | 492989 | 2493034 | 0.198 | 1.10× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.025 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.023 | 89.4% |
| carrier_disp_madd_fntable | 0.019 | 76.1% |
| carrier_disp_madd_ifchain | 0.021 | 83.5% |
| carrier_disp_madd_ifchainasc | 0.021 | 83.9% |
| carrier_disp_madd_ifchainlin | 0.023 | 90.2% |
| carrier_disp_madd_nullfloor | 0.025 | 98.3% |
| carrier_disp_madd_switch | 0.022 | 84.9% |
| carrier_disp_madd_threaded | 0.019 | 75.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 47345ns | 47345ns | -5.55% |
| carrier_disp_madd_fntable | 54810ns | 54810ns | +9.34% |
| carrier_disp_madd_ifchain | 50733ns | 50733ns | +1.21% |
| carrier_disp_madd_ifchainasc | 50130ns | 50130ns | +0.00% |
| carrier_disp_madd_ifchainlin | 46919ns | 46919ns | -6.40% |
| carrier_disp_madd_nullfloor | 43465ns | 43465ns | -13.29% |
| carrier_disp_madd_switch | 50128ns | 50128ns | base |
| carrier_disp_madd_threaded | 55631ns | 55631ns | +10.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 47290ns | base | --- | [47068, 49331] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 44911ns | -3072.5ns (-6.5%) | [-3816, -1519]ns | [43928, 46443] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_fntable | 52751ns | +4735.6ns (+10.0%) | [+3188, +5912]ns | [51221, 53552] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_ifchain | 48093ns | no significant difference | [-897, +2174]ns | [47244, 50136] | no | 0.2188 | 0.2188 | 0 |
| carrier_disp_madd_ifchainasc | 47876ns | no significant difference | [-1557, +1241]ns | [47121, 48746] | no | 0.2188 | 0.2188 | 0 |
| carrier_disp_madd_ifchainlin | 44519ns | -3344.0ns (-7.1%) | [-3911, -2446]ns | [43222, 46247] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_nullfloor | 40862ns | -6869.6ns (-14.5%) | [-7724, -5692]ns | [40263, 42278] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 53248ns | +4983.5ns (+10.5%) | [+4805, +6619]ns | [51938, 54910] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 47054ns | -6.7% | +7.7% | +0.1% | +1.1% | -8.5% | -14.6% | +10.2% |
| 2 | 50106ns | -8.8% | +5.5% | -3.6% | -6.6% | -6.5% | -16.3% | +9.8% |
| 3 | 47082ns | -6.3% | +14.0% | +1.8% | +3.5% | -7.3% | -14.1% | +10.8% |
| 4 | 48556ns | -4.5% | +10.1% | +1.9% | +0.4% | -6.5% | -15.0% | +12.9% |
| 5 | 47212ns | -6.9% | +9.7% | +0.4% | +0.5% | -8.1% | -14.5% | +10.2% |
| 6 | 47368ns | -1.8% | +11.1% | +7.3% | +1.7% | -3.6% | -10.0% | +14.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | -0.736 | HIGH- (thermal bounce) |
| carrier_disp_madd_fntable | 0.008 | ok |
| carrier_disp_madd_ifchain | -0.378 | moderate- |
| carrier_disp_madd_ifchainasc | -0.126 | ok |
| carrier_disp_madd_ifchainlin | -0.749 | HIGH- (thermal bounce) |
| carrier_disp_madd_nullfloor | -0.551 | HIGH- (thermal bounce) |
| carrier_disp_madd_switch | -0.576 | HIGH- (thermal bounce) |
| carrier_disp_madd_threaded | -0.838 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 6/6, lost 0/6
- **carrier_disp_madd_fntable**: won 0/6, lost 6/6
- **carrier_disp_madd_ifchain**: won 1/6, lost 4/6
- **carrier_disp_madd_ifchainasc**: won 1/6, lost 5/6
- **carrier_disp_madd_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 90321.6ns | 45093.9ns | 200.3% | HIGH |
| carrier_disp_madd_fntable | 106314.6ns | 52508.1ns | 202.5% | HIGH |
| carrier_disp_madd_ifchain | 97051.3ns | 48491.0ns | 200.1% | HIGH |
| carrier_disp_madd_ifchainasc | 95877.6ns | 47914.6ns | 200.1% | HIGH |
| carrier_disp_madd_ifchainlin | 90912.0ns | 44662.4ns | 203.6% | HIGH |
| carrier_disp_madd_nullfloor | 119971.0ns | 41134.3ns | 291.7% | HIGH |
| carrier_disp_madd_switch | 95859.3ns | 47896.2ns | 200.1% | HIGH |
| carrier_disp_madd_threaded | 106850.0ns | 53365.4ns | 200.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 43888.7-46442.9 ns)
  43888.7 |########################################
  44016.4 |####################
  44144.1 |
  44271.8 |
  44399.5 |
  44527.2 |
  44655.0 |
  44782.7 |
  44910.4 |
  45038.1 |
  45165.8 |
  45293.5 |
  45421.2 |
  45548.9 |
  45676.6 |####################
  45804.3 |
  45932.1 |
  46059.8 |
  46187.5 |
  46315.2 |####################
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 50655.8-53552.5 ns)
  50655.8 |########################################
  50800.6 |
  50945.5 |
  51090.3 |
  51235.1 |
  51380.0 |
  51524.8 |
  51669.6 |########################################
  51814.5 |
  51959.3 |
  52104.2 |
  52249.0 |
  52393.8 |
  52538.7 |########################################
  52683.5 |
  52828.3 |########################################
  52973.2 |
  53118.0 |
  53262.8 |
  53407.7 |########################################
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 47085.4-50135.7 ns)
  47085.4 |########################################
  47237.9 |
  47390.4 |########################################
  47542.9 |
  47695.5 |
  47848.0 |########################################
  48000.5 |
  48153.0 |########################################
  48305.5 |
  48458.0 |
  48610.5 |
  48763.0 |
  48915.6 |
  49068.1 |
  49220.6 |
  49373.1 |########################################
  49525.6 |
  49678.1 |
  49830.6 |
  49983.1 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 46807.5-48746.1 ns)
  46807.5 |########################################
  46904.4 |
  47001.4 |
  47098.3 |
  47195.2 |
  47292.1 |
  47389.1 |########################################
  47486.0 |########################################
  47582.9 |
  47679.8 |
  47776.8 |
  47873.7 |
  47970.6 |
  48067.6 |
  48164.5 |########################################
  48261.4 |
  48358.3 |
  48455.3 |
  48552.2 |
  48649.1 |########################################
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 43037.1-46246.7 ns)
  43037.1 |########################################
  43197.6 |
  43358.1 |########################################
  43518.5 |########################################
  43679.0 |
  43839.5 |
  44000.0 |
  44160.5 |
  44320.9 |
  44481.4 |
  44641.9 |
  44802.4 |
  44962.9 |
  45123.3 |
  45283.8 |########################################
  45444.3 |
  45604.8 |########################################
  45765.3 |
  45925.7 |
  46086.2 |
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 40168.3-42277.5 ns)
  40168.3 |########################################
  40273.8 |########################################
  40379.2 |########################################
  40484.7 |
  40590.1 |
  40695.6 |
  40801.1 |
  40906.5 |
  41012.0 |
  41117.4 |
  41222.9 |########################################
  41328.4 |
  41433.8 |
  41539.3 |
  41644.7 |
  41750.2 |
  41855.7 |########################################
  41961.1 |
  42066.6 |
  42172.0 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 47053.7-49330.8 ns)
  47053.7 |########################################
  47167.6 |####################
  47281.4 |####################
  47395.3 |
  47509.1 |
  47623.0 |
  47736.8 |
  47850.7 |
  47964.5 |
  48078.4 |
  48192.2 |
  48306.1 |
  48420.0 |
  48533.8 |####################
  48647.7 |
  48761.5 |
  48875.4 |
  48989.2 |
  49103.1 |
  49216.9 |
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 51846.2-54910.0 ns)
  51846.2 |########################################
  51999.4 |########################################
  52152.6 |########################################
  52305.8 |
  52459.0 |
  52612.1 |
  52765.3 |
  52918.5 |
  53071.7 |
  53224.9 |
  53378.1 |
  53531.3 |
  53684.5 |
  53837.7 |
  53990.9 |
  54144.1 |
  54297.2 |########################################
  54450.4 |
  54603.6 |
  54756.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: bridge=202.5% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=200.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=200.1% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=204.9% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=295.5% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=200.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=200.4% of algo (FFI overhead may distort results)
