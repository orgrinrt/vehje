# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vr_static) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vr_static has the worst median (2.43 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vr_nanbox at 2.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vr_tagged is inconsistent: worst-20% is 1.6x its best-20%

carrier_vr_tagged's best 20% of batches run at 2.14 us but its worst 20% at 3.33 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_vr_nanbox** at 2191.2 ns median (-9.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.11x (fastest 2191.2 ns, slowest 2425.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 4590ns | 4632ns | 4219ns | 4513ns | 4891ns | -3.07% |
| carrier_vr_static | 4736ns | 4729ns | 4620ns | 4702ns | 4844ns | base |
| carrier_vr_tagged | 5190ns | 4704ns | 4434ns | 4644ns | 6387ns | +9.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 2184ns | 2005ns | 2334ns | -9.72% | 0.029 |
| carrier_vr_static | 2419ns | 2354ns | 2469ns | base | 0.026 |
| carrier_vr_tagged | 2601ns | 2142ns | 3329ns | +7.55% | 0.025 |

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_vr_nanbox; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.029 | 91.5% |
| carrier_vr_static | 0.026 | 82.7% |
| carrier_vr_tagged | 0.028 | 86.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 4590ns | 4590ns | -3.07% |
| carrier_vr_static | 4736ns | 4736ns | base |
| carrier_vr_tagged | 5190ns | 5190ns | +9.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 2426ns | base | --- | [2361, 2469] | --- | --- | --- | --- |
| carrier_vr_nanbox | 2191ns | -262.1ns (-10.8%) | [-344, -99]ns | [2026, 2334] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_vr_tagged | 2319ns | no significant difference | [-220, +869]ns | [2156, 3329] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 2354ns | -14.8% | -9.0% |
| 2 | 2471ns | -13.8% | -7.9% |
| 3 | 2467ns | -5.5% | +16.7% |
| 4 | 2398ns | -2.6% | -9.5% |
| 5 | 2368ns | -13.6% | -0.2% |
| 6 | 2454ns | -8.2% | +54.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | -0.060 | ok |
| carrier_vr_static | -0.191 | ok |
| carrier_vr_tagged | -0.118 | ok |

**Consistency summary:**

- **carrier_vr_nanbox**: won 6/6, lost 0/6
- **carrier_vr_tagged**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 87901.9ns | 2183.5ns | 4025.7% | HIGH |
| carrier_vr_static | 86715.1ns | 2418.6ns | 3585.4% | HIGH |
| carrier_vr_tagged | 91467.3ns | 2601.2ns | 3516.3% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 2005.4-2333.5 ns)
   2005.4 |########################################
   2021.8 |
   2038.2 |########################################
   2054.6 |
   2071.0 |
   2087.4 |
   2103.8 |
   2120.2 |########################################
   2136.6 |
   2153.0 |
   2169.4 |
   2185.9 |
   2202.3 |
   2218.7 |
   2235.1 |
   2251.5 |########################################
   2267.9 |
   2284.3 |
   2300.7 |
   2317.1 |########################################
  (0 below, 1 above range)

carrier_vr_static (n=6, range 2353.8-2468.9 ns)
   2353.8 |########################################
   2359.6 |
   2365.3 |########################################
   2371.1 |
   2376.8 |
   2382.6 |
   2388.3 |
   2394.1 |########################################
   2399.9 |
   2405.6 |
   2411.4 |
   2417.1 |
   2422.9 |
   2428.6 |
   2434.4 |
   2440.2 |
   2445.9 |
   2451.7 |########################################
   2457.4 |
   2463.2 |########################################
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 2142.5-3329.2 ns)
   2142.5 |########################################
   2201.8 |
   2261.2 |####################
   2320.5 |####################
   2379.8 |
   2439.2 |
   2498.5 |
   2557.8 |
   2617.2 |
   2676.5 |
   2735.8 |
   2795.2 |
   2854.5 |####################
   2913.8 |
   2973.2 |
   3032.5 |
   3091.8 |
   3151.2 |
   3210.5 |
   3269.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: bridge=3992.0% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=3563.7% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: CV=22.3% (high variance, measurements may be unstable)
- **carrier_vr_tagged**: bridge=3810.6% of algo (FFI overhead may distort results)
