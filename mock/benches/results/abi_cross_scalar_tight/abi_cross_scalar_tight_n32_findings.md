# abi_cross_scalar (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_tight_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_tight_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_tight_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_tight_inproc_direct has the worst median (2.20 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_tight_null_entry at 2.38 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_tight_null_entry dominates: 91648% faster than the next best (abi_cross_scalar_tight_ffi_batched_scalar)

abi_cross_scalar_tight_null_entry (2.38 us) leads abi_cross_scalar_tight_ffi_batched_scalar (2.18 ms) by 91648%, a clear separation rather than a photo finish. CV 17.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_tight_null_entry beats baseline by 100% (significant)

abi_cross_scalar_tight_null_entry is -2.19 ms (100%) faster than baseline abi_cross_scalar_tight_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_tight_inproc_direct is an outlier: 924.3x slower than the field

abi_cross_scalar_tight_inproc_direct (2.20 ms) is 924.3x the fastest (2.38 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_tight_null_entry} vs {abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_inproc_direct} (91648% apart)

The field splits into a fast tier {abi_cross_scalar_tight_null_entry} and a slow tier {abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_inproc_direct} with a 91648% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 924.3x the fastest

Fastest abi_cross_scalar_tight_null_entry (2.38 us) to slowest abi_cross_scalar_tight_inproc_direct (2.20 ms): 924.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_scalar_tight_ffi_batched_scalar is inconsistent: worst-20% is 1.5x its best-20%

abi_cross_scalar_tight_ffi_batched_scalar's best 20% of batches run at 2.12 ms but its worst 20% at 3.23 ms (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_cross_scalar_tight_null_entry** at 2377.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 924.32x (fastest 2377.3 ns, slowest 2197382.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2514889ns | 2185175ns | 2120292ns | 2168458ns | 3231835ns | +14.85% |
| abi_cross_scalar_tight_inproc_direct | 2189708ns | 2202091ns | 2070434ns | 2160443ns | 2293241ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2387250ns | 2196625ns | 2058379ns | 2163885ns | 2886734ns | +9.02% |
| abi_cross_scalar_tight_null_entry | 5060ns | 4777ns | 4626ns | 4730ns | 5774ns | -99.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2510094ns | 2116672ns | 3225663ns | +14.86% | 0.000 |
| abi_cross_scalar_tight_inproc_direct | 2185327ns | 2066482ns | 2288565ns | base | 0.000 |
| abi_cross_scalar_tight_inproc_fnptr | 2382451ns | 2054639ns | 2880629ns | +9.02% | 0.000 |
| abi_cross_scalar_tight_null_entry | 2550ns | 2326ns | 2942ns | -99.88% | 0.013 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 100825.8 | 2532359.5 | 2510094.3 | n/a |
| abi_cross_scalar_tight_inproc_direct | 9941.8 | 2194438.7 | 2185326.8 | n/a |
| abi_cross_scalar_tight_inproc_fnptr | 11520.3 | 2441463.4 | 2382450.8 | n/a |
| abi_cross_scalar_tight_null_entry | 35023.9 | 2753.6 | 2549.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_scalar_tight_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_tight_null_entry | 0.013 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2514889ns | 2514889ns | +14.85% |
| abi_cross_scalar_tight_inproc_direct | 2189708ns | 2189708ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2387250ns | 2387250ns | +9.02% |
| abi_cross_scalar_tight_null_entry | 5060ns | 5060ns | -99.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_inproc_direct | 2197383ns | base | --- | [2070032, 2288565] | --- | --- | --- | --- |
| abi_cross_scalar_tight_ffi_batched_scalar | 2181126ns | no significant difference | [-58475, +969258]ns | [2123494, 3225663] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_tight_inproc_fnptr | 2192039ns | no significant difference | [-19359, +606079]ns | [2074685, 2880629] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_tight_null_entry | 2377ns | -2194446.5ns (-99.9%) | [-2286220, -2067664]ns | [2330, 2942] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_tight_inproc_direct | abi_cross_scalar_tight_ffi_batched_scalar | abi_cross_scalar_tight_inproc_fnptr | abi_cross_scalar_tight_null_entry |
|---|---|---|---|---|
| 1 | 2262315ns | +80.2% | +5.4% | -99.8% |
| 2 | 2286786ns | -5.0% | +47.7% | -99.9% |
| 3 | 2066482ns | +6.0% | -0.6% | -99.9% |
| 4 | 2290344ns | +3.7% | -0.7% | -99.9% |
| 5 | 2132450ns | -0.1% | -1.1% | -99.9% |
| 6 | 2073583ns | +2.1% | +1.0% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | -0.059 | ok |
| abi_cross_scalar_tight_inproc_direct | -0.289 | moderate- |
| abi_cross_scalar_tight_inproc_fnptr | -0.143 | ok |
| abi_cross_scalar_tight_null_entry | -0.080 | ok |

**Consistency summary:**

- **abi_cross_scalar_tight_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_tight_inproc_fnptr**: won 3/6, lost 3/6
- **abi_cross_scalar_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 7594774.5ns | 2510094.3ns | 302.6% | HIGH |
| abi_cross_scalar_tight_inproc_direct | 6598367.2ns | 2185326.8ns | 301.9% | HIGH |
| abi_cross_scalar_tight_inproc_fnptr | 7246199.9ns | 2382450.8ns | 304.1% | HIGH |
| abi_cross_scalar_tight_null_entry | 125458.4ns | 2549.9ns | 4920.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_tight_ffi_batched_scalar (n=6, range 2116671.7-3225662.9 ns)
  2116671.7 |########################################
  2172121.3 |#############
  2227570.8 |
  2283020.4 |
  2338469.9 |#############
  2393919.5 |
  2449369.1 |
  2504818.6 |
  2560268.2 |
  2615717.7 |
  2671167.3 |
  2726616.9 |
  2782066.4 |
  2837516.0 |
  2892965.5 |
  2948415.1 |
  3003864.7 |
  3059314.2 |
  3114763.8 |
  3170213.3 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_direct (n=6, range 2066481.7-2288565.0 ns)
  2066481.7 |########################################
  2077585.9 |
  2088690.0 |
  2099794.2 |
  2110898.4 |
  2122002.5 |####################
  2133106.7 |
  2144210.9 |
  2155315.0 |
  2166419.2 |
  2177523.4 |
  2188627.5 |
  2199731.7 |
  2210835.8 |
  2221940.0 |
  2233044.2 |
  2244148.3 |
  2255252.5 |####################
  2266356.7 |
  2277460.8 |####################
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_fnptr (n=6, range 2054638.8-2880629.1 ns)
  2054638.8 |########################################
  2095938.3 |####################
  2137237.8 |
  2178537.4 |
  2219836.9 |
  2261136.4 |####################
  2302435.9 |
  2343735.4 |####################
  2385034.9 |
  2426334.5 |
  2467634.0 |
  2508933.5 |
  2550233.0 |
  2591532.5 |
  2632832.0 |
  2674131.6 |
  2715431.1 |
  2756730.6 |
  2798030.1 |
  2839329.6 |
  (0 below, 1 above range)

abi_cross_scalar_tight_null_entry (n=6, range 2326.2-2942.5 ns)
   2326.2 |########################################
   2357.0 |####################
   2387.8 |########################################
   2418.6 |
   2449.5 |
   2480.3 |
   2511.1 |
   2541.9 |
   2572.7 |
   2603.5 |
   2634.3 |
   2665.2 |
   2696.0 |
   2726.8 |
   2757.6 |
   2788.4 |
   2819.2 |
   2850.1 |
   2880.9 |
   2911.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_tight_ffi_batched_scalar**: CV=28.1% (high variance, measurements may be unstable)
- **abi_cross_scalar_tight_ffi_batched_scalar**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_direct**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_fnptr**: bridge=300.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_null_entry**: bridge=5031.8% of algo (FFI overhead may distort results)
