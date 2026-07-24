# abi_soa_win (madd)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_madd_scalar_payload has the worst median (2.72 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_madd_null_entry at 2.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_madd_null_entry dominates: 42561% faster than the next best (abi_soa_win_madd_soa_payload)

abi_soa_win_madd_null_entry (2.55 us) leads abi_soa_win_madd_soa_payload (1.09 ms) by 42561%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_madd_null_entry beats baseline by 100% (significant)

abi_soa_win_madd_null_entry is -2.72 ms (100%) faster than baseline abi_soa_win_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_madd_scalar_payload is an outlier: 1065.9x slower than the field

abi_soa_win_madd_scalar_payload (2.72 ms) is 1065.9x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_madd_soa_payload shows alternating (throttle bounce) (autocorr -0.56)

abi_soa_win_madd_soa_payload's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 1065.9x the fastest

Fastest abi_soa_win_madd_null_entry (2.55 us) to slowest abi_soa_win_madd_scalar_payload (2.72 ms): 1065.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_madd_null_entry** at 2551.7 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1065.93x (fastest 2551.7 ns, slowest 2719886.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 4862ns | 4871ns | 4748ns | 4837ns | 4958ns | -99.83% |
| abi_soa_win_madd_scalar_payload | 2792023ns | 2723419ns | 2718289ns | 2722122ns | 2933742ns | base |
| abi_soa_win_madd_soa_payload | 1091781ns | 1091230ns | 1087458ns | 1090994ns | 1095124ns | -60.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 2543ns | 2467ns | 2606ns | -99.91% | 0.006 |
| abi_soa_win_madd_scalar_payload | 2788497ns | 2715330ns | 2929780ns | base | 0.000 |
| abi_soa_win_madd_soa_payload | 1089077ns | 1084870ns | 1092293ns | -60.94% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 27248.9 | 2629.9 | 2543.5 | n/a |
| abi_soa_win_madd_scalar_payload | 70176.1 | 2834851.3 | 2788496.9 | n/a |
| abi_soa_win_madd_soa_payload | 42812.6 | 1088490.6 | 1089076.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_soa_win_madd_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.006 | 96.7% |
| abi_soa_win_madd_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_madd_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_madd_null_entry | 4862ns | 4862ns | -99.83% |
| abi_soa_win_madd_scalar_payload | 2792023ns | 2792023ns | base |
| abi_soa_win_madd_soa_payload | 1091781ns | 1091781ns | -60.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_madd_scalar_payload | 2719887ns | base | --- | [2715824, 2929780] | --- | --- | --- | --- |
| abi_soa_win_madd_null_entry | 2552ns | -2717379.2ns (-99.9%) | [-2927263, -2713218]ns | [2473, 2606] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_madd_soa_payload | 1088562ns | -1632947.1ns (-60.0%) | [-1837487, -1627826]ns | [1086376, 1092293] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_madd_scalar_payload | abi_soa_win_madd_null_entry | abi_soa_win_madd_soa_payload |
|---|---|---|---|
| 1 | 2723316ns | -99.9% | -60.1% |
| 2 | 2716457ns | -99.9% | -59.9% |
| 3 | 2734460ns | -99.9% | -60.1% |
| 4 | 2716318ns | -99.9% | -59.9% |
| 5 | 3125100ns | -99.9% | -65.0% |
| 6 | 2715330ns | -99.9% | -60.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.059 | ok |
| abi_soa_win_madd_scalar_payload | -0.267 | moderate- |
| abi_soa_win_madd_soa_payload | -0.560 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_soa_win_madd_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 117672.2ns | 2543.5ns | 4626.4% | HIGH |
| abi_soa_win_madd_scalar_payload | 8586324.4ns | 2788496.9ns | 307.9% | HIGH |
| abi_soa_win_madd_soa_payload | 3308612.9ns | 1089076.9ns | 303.8% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_madd_null_entry (n=6, range 2466.7-2606.1 ns)
   2466.7 |########################################
   2473.7 |########################################
   2480.6 |
   2487.6 |
   2494.6 |
   2501.5 |
   2508.5 |
   2515.5 |
   2522.4 |
   2529.4 |
   2536.4 |
   2543.3 |########################################
   2550.3 |########################################
   2557.3 |
   2564.2 |
   2571.2 |
   2578.2 |
   2585.1 |########################################
   2592.1 |
   2599.1 |
  (0 below, 1 above range)

abi_soa_win_madd_scalar_payload (n=6, range 2715330.0-2929780.0 ns)
  2715330.0 |########################################
  2726052.5 |##########
  2736775.0 |
  2747497.5 |
  2758220.0 |
  2768942.5 |
  2779665.0 |
  2790387.5 |
  2801110.0 |
  2811832.5 |
  2822555.0 |
  2833277.5 |
  2844000.0 |
  2854722.5 |
  2865445.0 |
  2876167.5 |
  2886890.0 |
  2897612.5 |
  2908335.0 |
  2919057.5 |
  (0 below, 1 above range)

abi_soa_win_madd_soa_payload (n=6, range 1084870.4-1092293.1 ns)
  1084870.4 |########################################
  1085241.5 |
  1085612.7 |
  1085983.8 |
  1086354.9 |
  1086726.1 |
  1087097.2 |
  1087468.3 |
  1087839.5 |########################################
  1088210.6 |########################################
  1088581.8 |########################################
  1088952.9 |
  1089324.0 |
  1089695.2 |
  1090066.3 |
  1090437.4 |
  1090808.6 |########################################
  1091179.7 |
  1091550.8 |
  1091922.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_madd_null_entry**: bridge=4624.3% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_scalar_payload**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_soa_payload**: bridge=304.1% of algo (FFI overhead may distort results)
