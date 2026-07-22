# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_threaded_v17 is fastest but the noisiest (CV 13.6%)

carrier_disp_threaded_v17 wins on median (2.31 us) yet has the highest variance (CV 13.6%), while carrier_disp_fntable_v17 is the steadiest (CV 9.6%, 2.64 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_disp_threaded_v17** at 2315.0 ns median (-4.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.14x (fastest 2315.0 ns, slowest 2635.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 5152ns | 4975ns | 4691ns | 4908ns | 5748ns | +3.17% |
| carrier_disp_switch_v17 | 4994ns | 4964ns | 4305ns | 4746ns | 5710ns | base |
| carrier_disp_threaded_v17 | 4882ns | 4794ns | 4225ns | 4620ns | 5602ns | -2.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 2723ns | 2484ns | 3045ns | +11.22% | 0.024 |
| carrier_disp_switch_v17 | 2448ns | 2125ns | 2791ns | base | 0.026 |
| carrier_disp_threaded_v17 | 2371ns | 2065ns | 2734ns | -3.13% | 0.027 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_disp_threaded_v17; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.024 | 78.4% |
| carrier_disp_switch_v17 | 0.026 | 85.3% |
| carrier_disp_threaded_v17 | 0.028 | 89.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 5152ns | 5152ns | +3.17% |
| carrier_disp_switch_v17 | 4994ns | 4994ns | base |
| carrier_disp_threaded_v17 | 4882ns | 4882ns | -2.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 2422ns | base | --- | [2130, 2791] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 2635ns | no significant difference | [-48, +526]ns | [2488, 3045] | no | 0.2188 | 0.2188 | 0 |
| carrier_disp_threaded_v17 | 2315ns | -64.8ns (-2.7%) | [-124, -41]ns | [2066, 2734] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 | carrier_disp_threaded_v17 |
|---|---|---|---|
| 1 | 3008ns | -9.2% | -2.5% |
| 2 | 2136ns | +16.3% | -3.3% |
| 3 | 2125ns | +19.6% | -2.8% |
| 4 | 2309ns | +7.9% | -7.5% |
| 5 | 2536ns | +25.0% | -1.7% |
| 6 | 2575ns | +13.4% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | 0.178 | ok |
| carrier_disp_switch_v17 | -0.053 | ok |
| carrier_disp_threaded_v17 | -0.025 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 1/6, lost 5/6
- **carrier_disp_threaded_v17**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 58.9ns | 2722.7ns | 2.2% |  |
| carrier_disp_switch_v17 | 64.6ns | 2448.0ns | 2.6% |  |
| carrier_disp_threaded_v17 | 61.8ns | 2371.4ns | 2.6% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 2484.2-3045.0 ns)
   2484.2 |########################################
   2512.2 |
   2540.3 |####################
   2568.3 |
   2596.4 |
   2624.4 |
   2652.4 |
   2680.5 |
   2708.5 |####################
   2736.6 |
   2764.6 |
   2792.6 |
   2820.7 |
   2848.7 |
   2876.8 |
   2904.8 |####################
   2932.8 |
   2960.9 |
   2988.9 |
   3017.0 |
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 2124.6-2791.2 ns)
   2124.6 |########################################
   2157.9 |
   2191.3 |
   2224.6 |
   2257.9 |
   2291.3 |####################
   2324.6 |
   2357.9 |
   2391.3 |
   2424.6 |
   2457.9 |
   2491.3 |
   2524.6 |####################
   2557.9 |####################
   2591.3 |
   2624.6 |
   2657.9 |
   2691.3 |
   2724.6 |
   2757.9 |
  (0 below, 1 above range)

carrier_disp_threaded_v17 (n=6, range 2065.4-2733.6 ns)
   2065.4 |########################################
   2098.8 |
   2132.2 |####################
   2165.6 |
   2199.0 |
   2232.4 |
   2265.8 |
   2299.3 |
   2332.7 |
   2366.1 |
   2399.5 |
   2432.9 |
   2466.3 |####################
   2499.7 |
   2533.1 |####################
   2566.5 |
   2599.9 |
   2633.3 |
   2666.7 |
   2700.1 |
  (0 below, 1 above range)

```
