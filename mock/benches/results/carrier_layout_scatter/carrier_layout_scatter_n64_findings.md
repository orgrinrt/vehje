# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 1.4% of the fastest

All 5 variants sit between 2.50 us and 2.54 us - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_scatter_rec12's edge over baseline is significant but tiny (25 ns, 0.99%)

carrier_lay_scatter_rec12 differs from baseline carrier_lay_scatter_rec24 by 25 ns (0.99%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_scatter_rec20** at 2502.1 ns median (-0.3% vs baseline)
- Spread: 1.01x (fastest 2502.1 ns, slowest 2537.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 4976ns | 5036ns | 4515ns | 5011ns | 5153ns | -2.51% |
| carrier_lay_scatter_rec16 | 5075ns | 5070ns | 4972ns | 5043ns | 5177ns | -0.56% |
| carrier_lay_scatter_rec20 | 5017ns | 5026ns | 4875ns | 5003ns | 5108ns | -1.71% |
| carrier_lay_scatter_rec24 | 5104ns | 5105ns | 4988ns | 5104ns | 5162ns | base |
| carrier_lay_scatter_rec32 | 5086ns | 5046ns | 4950ns | 5038ns | 5227ns | -0.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 2487ns | 2219ns | 2561ns | -1.28% | 0.026 |
| carrier_lay_scatter_rec16 | 2513ns | 2505ns | 2518ns | -0.25% | 0.025 |
| carrier_lay_scatter_rec20 | 2509ns | 2462ns | 2550ns | -0.40% | 0.026 |
| carrier_lay_scatter_rec24 | 2519ns | 2489ns | 2554ns | base | 0.025 |
| carrier_lay_scatter_rec32 | 2522ns | 2470ns | 2560ns | +0.12% | 0.025 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_lay_scatter_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.025 | 87.4% |
| carrier_lay_scatter_rec16 | 0.025 | 88.3% |
| carrier_lay_scatter_rec20 | 0.026 | 88.7% |
| carrier_lay_scatter_rec24 | 0.026 | 88.5% |
| carrier_lay_scatter_rec32 | 0.025 | 87.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 4976ns | 4976ns | -2.51% |
| carrier_lay_scatter_rec16 | 5075ns | 5075ns | -0.56% |
| carrier_lay_scatter_rec20 | 5017ns | 5017ns | -1.71% |
| carrier_lay_scatter_rec24 | 5104ns | 5104ns | base |
| carrier_lay_scatter_rec32 | 5086ns | 5086ns | -0.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 2508ns | base | --- | [2495, 2554] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 2537ns | no significant difference | [-184, +62]ns | [2362, 2561] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_scatter_rec16 | 2513ns | no significant difference | [-39, +13]ns | [2506, 2518] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_scatter_rec20 | 2502ns | no significant difference | [-67, +29]ns | [2475, 2550] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_scatter_rec32 | 2527ns | no significant difference | [-62, +65]ns | [2479, 2560] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 2576ns | -13.9% | -2.2% | -4.4% | -4.1% |
| 2 | 2508ns | -0.1% | +0.4% | -0.8% | -0.7% |
| 3 | 2489ns | +2.6% | +0.6% | +0.3% | +3.6% |
| 4 | 2532ns | -0.4% | -0.8% | +0.9% | -0.5% |
| 5 | 2500ns | +2.1% | +0.3% | +0.3% | +1.7% |
| 6 | 2509ns | +2.3% | +0.2% | +1.4% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.074 | ok |
| carrier_lay_scatter_rec16 | -0.033 | ok |
| carrier_lay_scatter_rec20 | 0.091 | ok |
| carrier_lay_scatter_rec24 | -0.157 | ok |
| carrier_lay_scatter_rec32 | -0.012 | ok |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 3/6, lost 3/6
- **carrier_lay_scatter_rec16**: won 2/6, lost 4/6
- **carrier_lay_scatter_rec20**: won 2/6, lost 4/6
- **carrier_lay_scatter_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 86400.2ns | 2486.7ns | 3474.5% | HIGH |
| carrier_lay_scatter_rec16 | 86768.7ns | 2512.7ns | 3453.2% | HIGH |
| carrier_lay_scatter_rec20 | 86610.3ns | 2508.9ns | 3452.2% | HIGH |
| carrier_lay_scatter_rec24 | 86896.4ns | 2519.0ns | 3449.7% | HIGH |
| carrier_lay_scatter_rec32 | 87169.8ns | 2522.1ns | 3456.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 2218.8-2561.2 ns)
   2218.8 |####################
   2235.9 |
   2253.0 |
   2270.2 |
   2287.3 |
   2304.4 |
   2321.5 |
   2338.7 |
   2355.8 |
   2372.9 |
   2390.0 |
   2407.1 |
   2424.3 |
   2441.4 |
   2458.5 |
   2475.6 |
   2492.8 |####################
   2509.9 |####################
   2527.0 |
   2544.1 |########################################
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 2504.6-2518.3 ns)
   2504.6 |########################################
   2505.3 |
   2506.0 |
   2506.7 |
   2507.3 |
   2508.0 |########################################
   2508.7 |
   2509.4 |
   2510.1 |
   2510.8 |
   2511.5 |########################################
   2512.2 |
   2512.8 |
   2513.5 |
   2514.2 |
   2514.9 |########################################
   2515.6 |
   2516.3 |
   2517.0 |########################################
   2517.7 |
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 2461.7-2549.6 ns)
   2461.7 |########################################
   2466.1 |
   2470.5 |
   2474.9 |
   2479.3 |
   2483.7 |
   2488.1 |########################################
   2492.4 |########################################
   2496.8 |
   2501.2 |
   2505.6 |########################################
   2510.0 |
   2514.4 |
   2518.8 |
   2523.2 |
   2527.6 |
   2532.0 |
   2536.4 |
   2540.8 |########################################
   2545.2 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 2489.2-2553.9 ns)
   2489.2 |########################################
   2492.4 |
   2495.7 |
   2498.9 |########################################
   2502.1 |
   2505.4 |########################################
   2508.6 |########################################
   2511.9 |
   2515.1 |
   2518.3 |
   2521.6 |
   2524.8 |
   2528.0 |
   2531.3 |########################################
   2534.5 |
   2537.8 |
   2541.0 |
   2544.2 |
   2547.5 |
   2550.7 |
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 2470.0-2560.0 ns)
   2470.0 |########################################
   2474.5 |
   2479.0 |
   2483.5 |
   2488.0 |########################################
   2492.5 |
   2497.0 |
   2501.5 |
   2506.0 |
   2510.5 |
   2515.0 |
   2519.5 |########################################
   2524.0 |
   2528.5 |
   2533.0 |########################################
   2537.5 |########################################
   2542.0 |
   2546.5 |
   2551.0 |
   2555.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: bridge=3409.5% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=3449.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: bridge=3461.7% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=3455.9% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=3446.7% of algo (FFI overhead may distort results)
