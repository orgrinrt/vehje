# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_fntable_v17 dominates: 19% faster than the next best (carrier_disp_switch_v17)

carrier_disp_fntable_v17 (2.15 ms) leads carrier_disp_switch_v17 (2.56 ms) by 19%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: carrier_disp_fntable_v17** at 2153484.4 ns median (-15.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.31x (fastest 2153484.4 ns, slowest 2819723.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 2156569ns | 2157017ns | 2143325ns | 2155837ns | 2164288ns | -15.62% |
| carrier_disp_switch_v17 | 2555771ns | 2558996ns | 2520492ns | 2553115ns | 2577394ns | base |
| carrier_disp_threaded_v17 | 2824430ns | 2823343ns | 2799630ns | 2817282ns | 2847552ns | +10.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 2153252ns | 2139838ns | 2161061ns | -15.64% | 0.008 |
| carrier_disp_switch_v17 | 2552566ns | 2517741ns | 2574267ns | base | 0.006 |
| carrier_disp_threaded_v17 | 2820742ns | 2795896ns | 2843797ns | +10.51% | 0.006 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_disp_fntable_v17; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.008 | 99.4% |
| carrier_disp_switch_v17 | 0.006 | 83.7% |
| carrier_disp_threaded_v17 | 0.006 | 75.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 2156569ns | 2156569ns | -15.62% |
| carrier_disp_switch_v17 | 2555771ns | 2555771ns | base |
| carrier_disp_threaded_v17 | 2824430ns | 2824430ns | +10.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 2555669ns | base | --- | [2527761, 2574267] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 2153484ns | -402784.4ns (-15.8%) | [-427752, -367404]ns | [2145211, 2161061] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_threaded_v17 | 2819724ns | +274094.6ns (+10.7%) | [+245200, +285235]ns | [2798706, 2843797] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 | carrier_disp_threaded_v17 |
|---|---|---|---|
| 1 | 2537782ns | -14.6% | +11.2% |
| 2 | 2557570ns | -15.9% | +9.5% |
| 3 | 2517741ns | -14.5% | +11.0% |
| 4 | 2553768ns | -15.6% | +10.6% |
| 5 | 2571081ns | -16.8% | +9.6% |
| 6 | 2577453ns | -16.5% | +11.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | -0.164 | ok |
| carrier_disp_switch_v17 | 0.080 | ok |
| carrier_disp_threaded_v17 | 0.081 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 6/6, lost 0/6
- **carrier_disp_threaded_v17**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 2422.2ns | 2153252.3ns | 0.1% |  |
| carrier_disp_switch_v17 | 2510.9ns | 2552565.7ns | 0.1% |  |
| carrier_disp_threaded_v17 | 2737.4ns | 2820742.1ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 2139837.9-2161061.0 ns)
  2139837.9 |########################################
  2140899.1 |
  2141960.2 |
  2143021.4 |
  2144082.5 |
  2145143.7 |
  2146204.8 |
  2147266.0 |
  2148327.2 |
  2149388.3 |
  2150449.5 |########################################
  2151510.6 |
  2152571.8 |########################################
  2153632.9 |########################################
  2154694.1 |########################################
  2155755.3 |
  2156816.4 |
  2157877.6 |
  2158938.7 |
  2159999.9 |
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 2517740.8-2574267.0 ns)
  2517740.8 |########################################
  2520567.1 |
  2523393.4 |
  2526219.7 |
  2529046.0 |
  2531872.4 |
  2534698.7 |
  2537525.0 |########################################
  2540351.3 |
  2543177.6 |
  2546003.9 |
  2548830.2 |
  2551656.5 |########################################
  2554482.9 |
  2557309.2 |########################################
  2560135.5 |
  2562961.8 |
  2565788.1 |
  2568614.4 |########################################
  2571440.7 |
  (0 below, 1 above range)

carrier_disp_threaded_v17 (n=6, range 2795896.2-2843796.9 ns)
  2795896.2 |########################################
  2798291.2 |
  2800686.3 |########################################
  2803081.3 |
  2805476.3 |
  2807871.4 |
  2810266.4 |
  2812661.4 |
  2815056.5 |
  2817451.5 |########################################
  2819846.5 |########################################
  2822241.6 |########################################
  2824636.6 |
  2827031.6 |
  2829426.7 |
  2831821.7 |
  2834216.7 |
  2836611.8 |
  2839006.8 |
  2841401.8 |
  (0 below, 1 above range)

```
