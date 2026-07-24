# abi_soa_win (madd)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_madd_scalar_payload has the worst median (2.72 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_madd_null_entry at 2.76 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_madd_null_entry dominates: 39354% faster than the next best (abi_soa_win_madd_soa_payload)

abi_soa_win_madd_null_entry (2.76 us) leads abi_soa_win_madd_soa_payload (1.09 ms) by 39354%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_madd_null_entry beats baseline by 100% (significant)

abi_soa_win_madd_null_entry is -2.72 ms (100%) faster than baseline abi_soa_win_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_madd_scalar_payload is an outlier: 986.4x slower than the field

abi_soa_win_madd_scalar_payload (2.72 ms) is 986.4x the fastest (2.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 986.4x the fastest

Fastest abi_soa_win_madd_null_entry (2.76 us) to slowest abi_soa_win_madd_scalar_payload (2.72 ms): 986.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_madd_null_entry** at 2756.7 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 986.41x (fastest 2756.7 ns, slowest 2719178.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 5113ns | 5101ns | 4941ns | 5083ns | 5245ns | -99.81% |
| abi_soa_win_madd_scalar_payload | 2744284ns | 2722263ns | 2717800ns | 2721852ns | 2791176ns | base |
| abi_soa_win_madd_soa_payload | 1092025ns | 1090514ns | 1087064ns | 1089529ns | 1098250ns | -60.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 2761ns | 2678ns | 2827ns | -99.90% | 0.046 |
| abi_soa_win_madd_scalar_payload | 2741112ns | 2714857ns | 2787863ns | base | 0.000 |
| abi_soa_win_madd_soa_payload | 1089130ns | 1084433ns | 1095091ns | -60.27% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 26974.1 | 2807.7 | 2760.7 | n/a |
| abi_soa_win_madd_scalar_payload | 62818.5 | 2740814.0 | 2741111.9 | n/a |
| abi_soa_win_madd_soa_payload | 46515.5 | 1089433.5 | 1089130.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_soa_win_madd_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.046 | 97.1% |
| abi_soa_win_madd_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_madd_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_madd_null_entry | 5113ns | 5113ns | -99.81% |
| abi_soa_win_madd_scalar_payload | 2744284ns | 2744284ns | base |
| abi_soa_win_madd_soa_payload | 1092025ns | 1092025ns | -60.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_madd_scalar_payload | 2719178ns | base | --- | [2716294, 2787863] | --- | --- | --- | --- |
| abi_soa_win_madd_null_entry | 2757ns | -2716421.6ns (-99.9%) | [-2785129, -2713503]ns | [2698, 2827] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_madd_soa_payload | 1087601ns | -1632939.3ns (-60.1%) | [-1694784, -1628221]ns | [1084700, 1095091] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_madd_scalar_payload | abi_soa_win_madd_null_entry | abi_soa_win_madd_soa_payload |
|---|---|---|---|
| 1 | 2851376ns | -99.9% | -61.4% |
| 2 | 2718866ns | -99.9% | -60.0% |
| 3 | 2714857ns | -99.9% | -59.9% |
| 4 | 2719491ns | -99.9% | -60.1% |
| 5 | 2717731ns | -99.9% | -59.9% |
| 6 | 2724350ns | -99.9% | -60.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.208 | moderate+ |
| abi_soa_win_madd_scalar_payload | -0.028 | ok |
| abi_soa_win_madd_soa_payload | -0.030 | ok |

**Consistency summary:**

- **abi_soa_win_madd_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 119638.0ns | 2760.7ns | 4333.6% | HIGH |
| abi_soa_win_madd_scalar_payload | 8267423.2ns | 2741111.9ns | 301.6% | HIGH |
| abi_soa_win_madd_soa_payload | 3316475.6ns | 1089130.5ns | 304.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_madd_null_entry (n=6, range 2677.5-2827.3 ns)
   2677.5 |########################################
   2685.0 |
   2692.5 |
   2700.0 |
   2707.5 |
   2714.9 |########################################
   2722.4 |
   2729.9 |
   2737.4 |
   2744.9 |
   2752.4 |########################################
   2759.9 |########################################
   2767.4 |
   2774.9 |
   2782.4 |
   2789.9 |########################################
   2797.3 |
   2804.8 |
   2812.3 |
   2819.8 |
  (0 below, 1 above range)

abi_soa_win_madd_scalar_payload (n=6, range 2714857.1-2787863.3 ns)
  2714857.1 |########################################
  2718507.4 |########################################
  2722157.7 |####################
  2725808.0 |
  2729458.3 |
  2733108.6 |
  2736759.0 |
  2740409.3 |
  2744059.6 |
  2747709.9 |
  2751360.2 |
  2755010.5 |
  2758660.8 |
  2762311.1 |
  2765961.4 |
  2769611.8 |
  2773262.1 |
  2776912.4 |
  2780562.7 |
  2784213.0 |
  (0 below, 1 above range)

abi_soa_win_madd_soa_payload (n=6, range 1084432.9-1095091.0 ns)
  1084432.9 |########################################
  1084965.8 |########################################
  1085498.7 |
  1086031.6 |
  1086564.5 |
  1087097.4 |########################################
  1087630.3 |########################################
  1088163.3 |########################################
  1088696.2 |
  1089229.1 |
  1089762.0 |
  1090294.9 |
  1090827.8 |
  1091360.7 |
  1091893.6 |
  1092426.5 |
  1092959.4 |
  1093492.3 |
  1094025.2 |
  1094558.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_madd_null_entry**: bridge=4337.2% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_scalar_payload**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_soa_payload**: bridge=304.2% of algo (FFI overhead may distort results)
