# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.17 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 3.95 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 54667% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (3.95 us) leads abi_soa_win_real_soa_payload (2.16 ms) by 54667%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.16 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 548.2x slower than the field

abi_soa_win_real_scalar_payload (2.17 ms) is 548.2x the fastest (3.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 548.2x the fastest

Fastest abi_soa_win_real_null_entry (3.95 us) to slowest abi_soa_win_real_scalar_payload (2.17 ms): 548.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 3950.4 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 548.19x (fastest 3950.4 ns, slowest 2165588.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 6247ns | 6188ns | 6165ns | 6187ns | 6377ns | -99.74% |
| abi_soa_win_real_scalar_payload | 2377647ns | 2168982ns | 2164362ns | 2168134ns | 2798559ns | base |
| abi_soa_win_real_soa_payload | 2168564ns | 2166672ns | 2162772ns | 2165810ns | 2175591ns | -8.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 3975ns | 3899ns | 4069ns | -99.83% | 0.001 |
| abi_soa_win_real_scalar_payload | 2374200ns | 2161204ns | 2794784ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 2165301ns | 2159685ns | 2172122ns | -8.80% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 27500.4 | 4120.3 | 3974.6 | n/a |
| abi_soa_win_real_scalar_payload | 77799.4 | 2266704.0 | 2374200.1 | n/a |
| abi_soa_win_real_soa_payload | 64284.2 | 2169117.6 | 2165301.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.001 | 98.7% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_real_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 6247ns | 6247ns | -99.74% |
| abi_soa_win_real_scalar_payload | 2377647ns | 2377647ns | base |
| abi_soa_win_real_soa_payload | 2168564ns | 2168564ns | -8.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2165589ns | base | --- | [2162228, 2794784] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 3950ns | -2161610.9ns (-99.8%) | [-2790756, -2158310]ns | [3904, 4069] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 2163509ns | no significant difference | [-625352, +2007]ns | [2160272, 2172122] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 3416815ns | -99.9% | -36.3% |
| 2 | 2172753ns | -99.8% | -0.5% |
| 3 | 2165132ns | -99.8% | +0.1% |
| 4 | 2161204ns | -99.8% | -0.1% |
| 5 | 2166045ns | -99.8% | -0.2% |
| 6 | 2163251ns | -99.8% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.122 | ok |
| abi_soa_win_real_scalar_payload | -0.027 | ok |
| abi_soa_win_real_soa_payload | -0.148 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 3/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 121946.5ns | 3974.6ns | 3068.2% | HIGH |
| abi_soa_win_real_scalar_payload | 6802251.2ns | 2374200.1ns | 286.5% | HIGH |
| abi_soa_win_real_soa_payload | 6572197.5ns | 2165301.1ns | 303.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 3898.7-4069.2 ns)
   3898.7 |########################################
   3907.2 |########################################
   3915.7 |
   3924.3 |
   3932.8 |########################################
   3941.3 |
   3949.8 |
   3958.4 |########################################
   3966.9 |
   3975.4 |
   3983.9 |
   3992.4 |
   4001.0 |
   4009.5 |
   4018.0 |
   4026.5 |
   4035.1 |
   4043.6 |########################################
   4052.1 |
   4060.6 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2161204.2-2794784.3 ns)
  2161204.2 |########################################
  2192883.2 |
  2224562.2 |
  2256241.2 |
  2287920.2 |
  2319599.2 |
  2351278.2 |
  2382957.3 |
  2414636.3 |
  2446315.3 |
  2477994.3 |
  2509673.3 |
  2541352.3 |
  2573031.3 |
  2604710.3 |
  2636389.3 |
  2668068.3 |
  2699747.3 |
  2731426.3 |
  2763105.3 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 2159685.0-2172122.3 ns)
  2159685.0 |########################################
  2160306.9 |########################################
  2160928.7 |
  2161550.6 |########################################
  2162172.5 |
  2162794.3 |
  2163416.2 |
  2164038.1 |
  2164659.9 |########################################
  2165281.8 |
  2165903.6 |
  2166525.5 |
  2167147.4 |########################################
  2167769.2 |
  2168391.1 |
  2169013.0 |
  2169634.8 |
  2170256.7 |
  2170878.6 |
  2171500.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=3082.2% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=303.7% of algo (FFI overhead may distort results)
