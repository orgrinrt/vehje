# Partial-eval specialization: fold ratio on a block-structured template (reduction metric)

3 variants, 6 samples per variant.
Baseline: **pe_struct_sf50**

## Highlights

Baseline for all deltas below: **pe_struct_sf50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### pe_struct_sf70 dominates: 13% faster than the next best (pe_struct_sf50)

pe_struct_sf70 (2.29 ms) leads pe_struct_sf50 (2.58 ms) by 13%, a clear separation rather than a photo finish. CV 9.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### pe_struct_sf70 is fastest but the noisiest (CV 9.2%)

pe_struct_sf70 wins on median (2.29 ms) yet has the highest variance (CV 9.2%), while pe_struct_sf90 is the steadiest (CV 2.0%, 2.66 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: pe_struct_sf70** at 2291527.5 ns median (-11.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 2291527.5 ns, slowest 2656850.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_struct_sf50 | 2603011ns | 2581738ns | 2561952ns | 2578624ns | 2660121ns | base |
| pe_struct_sf70 | 2403454ns | 2294417ns | 2260324ns | 2284333ns | 2653702ns | -7.67% |
| pe_struct_sf90 | 2669200ns | 2659636ns | 2597604ns | 2648901ns | 2735445ns | +2.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_struct_sf50 | 2600181ns | 2559184ns | 2657246ns | base | 0.006 |
| pe_struct_sf70 | 2400577ns | 2257385ns | 2650731ns | -7.68% | 0.007 |
| pe_struct_sf90 | 2666363ns | 2594734ns | 2732512ns | +2.55% | 0.006 |

## Performance model

- Peak throughput: **0.007 Gops/s** (pe_struct_sf70; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_struct_sf50 | 0.006 | 87.5% |
| pe_struct_sf70 | 0.007 | 98.5% |
| pe_struct_sf90 | 0.006 | 85.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_struct_sf50 | 2603011ns | 2603011ns | base |
| pe_struct_sf70 | 2403454ns | 2403454ns | -7.67% |
| pe_struct_sf90 | 2669200ns | 2669200ns | +2.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_struct_sf50 | 2578997ns | base | --- | [2564301, 2657246] | --- | --- | --- | --- |
| pe_struct_sf70 | 2291528ns | -272773.1ns (-10.6%) | [-319525, -6515]ns | [2259472, 2650731] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| pe_struct_sf90 | 2656851ns | +70591.2ns (+2.7%) | [+6004, +121950]ns | [2609727, 2732512] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_struct_sf50 | pe_struct_sf70 | pe_struct_sf90 |
|---|---|---|---|
| 1 | 2725670ns | +4.5% | -0.2% |
| 2 | 2581934ns | -12.4% | +3.3% |
| 3 | 2588822ns | -5.2% | +6.1% |
| 4 | 2569418ns | -9.9% | +2.2% |
| 5 | 2559184ns | -11.3% | +3.4% |
| 6 | 2576060ns | -12.4% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_struct_sf50 | 0.027 | ok |
| pe_struct_sf70 | -0.164 | ok |
| pe_struct_sf90 | -0.049 | ok |

**Consistency summary:**

- **pe_struct_sf70**: won 5/6, lost 1/6
- **pe_struct_sf90**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_struct_sf50 | 33.6ns | 2600181.3ns | 0.0% |  |
| pe_struct_sf70 | 44.9ns | 2400576.8ns | 0.0% |  |
| pe_struct_sf90 | 35.8ns | 2666363.1ns | 0.0% |  |

## Distribution (algo ns)

```
pe_struct_sf50 (n=6, range 2559183.8-2657246.0 ns)
  2559183.8 |########################################
  2564086.9 |
  2568990.0 |########################################
  2573893.1 |########################################
  2578796.2 |########################################
  2583699.4 |
  2588602.5 |########################################
  2593505.6 |
  2598408.7 |
  2603311.8 |
  2608214.9 |
  2613118.0 |
  2618021.1 |
  2622924.3 |
  2627827.4 |
  2632730.5 |
  2637633.6 |
  2642536.7 |
  2647439.8 |
  2652342.9 |
  (0 below, 1 above range)

pe_struct_sf70 (n=6, range 2257385.4-2650731.0 ns)
  2257385.4 |########################################
  2277052.7 |
  2296720.0 |#############
  2316387.2 |
  2336054.5 |
  2355721.8 |
  2375389.1 |
  2395056.4 |
  2414723.7 |
  2434390.9 |#############
  2454058.2 |
  2473725.5 |
  2493392.8 |
  2513060.1 |
  2532727.4 |
  2552394.6 |
  2572061.9 |
  2591729.2 |
  2611396.5 |
  2631063.8 |
  (0 below, 1 above range)

pe_struct_sf90 (n=6, range 2594733.8-2732511.6 ns)
  2594733.8 |########################################
  2601622.7 |
  2608511.6 |
  2615400.5 |
  2622289.4 |########################################
  2629178.3 |
  2636067.2 |
  2642956.0 |########################################
  2649844.9 |
  2656733.8 |
  2663622.7 |########################################
  2670511.6 |
  2677400.5 |
  2684289.4 |
  2691178.3 |
  2698067.2 |
  2704956.1 |
  2711845.0 |
  2718733.9 |########################################
  2725622.8 |
  (0 below, 1 above range)

```
