# abi_soa_win (madd)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_madd_scalar_payload has the worst median (2.76 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_madd_null_entry at 3.39 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_madd_null_entry dominates: 81147% faster than the next best (abi_soa_win_madd_soa_payload)

abi_soa_win_madd_null_entry (3.39 us) leads abi_soa_win_madd_soa_payload (2.75 ms) by 81147%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_madd_null_entry beats baseline by 100% (significant)

abi_soa_win_madd_null_entry is -2.75 ms (100%) faster than baseline abi_soa_win_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_madd_scalar_payload is an outlier: 813.6x slower than the field

abi_soa_win_madd_scalar_payload (2.76 ms) is 813.6x the fastest (3.39 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 813.6x the fastest

Fastest abi_soa_win_madd_null_entry (3.39 us) to slowest abi_soa_win_madd_scalar_payload (2.76 ms): 813.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_madd_null_entry** at 3390.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 813.59x (fastest 3390.2 ns, slowest 2758238.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 5687ns | 5617ns | 5542ns | 5598ns | 5893ns | -99.80% |
| abi_soa_win_madd_scalar_payload | 2774997ns | 2761442ns | 2739692ns | 2757450ns | 2818971ns | base |
| abi_soa_win_madd_soa_payload | 2755803ns | 2757798ns | 2742616ns | 2754185ns | 2764823ns | -0.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 3428ns | 3344ns | 3546ns | -99.88% | 0.001 |
| abi_soa_win_madd_scalar_payload | 2771722ns | 2737021ns | 2815155ns | base | 0.000 |
| abi_soa_win_madd_soa_payload | 2752621ns | 2739823ns | 2761518ns | -0.69% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 26459.4 | 3521.7 | 3428.0 | n/a |
| abi_soa_win_madd_scalar_payload | 63779.4 | 2767545.3 | 2771721.8 | n/a |
| abi_soa_win_madd_soa_payload | 61785.1 | 2754162.8 | 2752621.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_madd_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.001 | 98.6% |
| abi_soa_win_madd_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_madd_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_madd_null_entry | 5687ns | 5687ns | -99.80% |
| abi_soa_win_madd_scalar_payload | 2774997ns | 2774997ns | base |
| abi_soa_win_madd_soa_payload | 2755803ns | 2755803ns | -0.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_madd_scalar_payload | 2758239ns | base | --- | [2741772, 2815155] | --- | --- | --- | --- |
| abi_soa_win_madd_null_entry | 3390ns | -2754852.9ns (-99.9%) | [-2811802, -2738226]ns | [3348, 3546] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_madd_soa_payload | 2754436ns | no significant difference | [-60509, +4973]ns | [2741910, 2761518] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_madd_scalar_payload | abi_soa_win_madd_null_entry | abi_soa_win_madd_soa_payload |
|---|---|---|---|
| 1 | 2759872ns | -99.9% | -0.0% |
| 2 | 2762217ns | -99.9% | -0.1% |
| 3 | 2746523ns | -99.9% | -0.1% |
| 4 | 2737021ns | -99.9% | +0.1% |
| 5 | 2756605ns | -99.9% | +0.3% |
| 6 | 2868093ns | -99.9% | -4.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.223 | moderate+ |
| abi_soa_win_madd_scalar_payload | 0.025 | ok |
| abi_soa_win_madd_soa_payload | -0.170 | ok |

**Consistency summary:**

- **abi_soa_win_madd_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_madd_soa_payload**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 119100.5ns | 3428.0ns | 3474.4% | HIGH |
| abi_soa_win_madd_scalar_payload | 8363911.6ns | 2771721.8ns | 301.8% | HIGH |
| abi_soa_win_madd_soa_payload | 8322360.2ns | 2752621.3ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_madd_null_entry (n=6, range 3344.2-3545.8 ns)
   3344.2 |########################################
   3354.3 |
   3364.4 |
   3374.4 |
   3384.5 |
   3394.6 |
   3404.7 |
   3414.8 |
   3424.8 |#############
   3434.9 |
   3445.0 |
   3455.1 |
   3465.2 |
   3475.2 |#############
   3485.3 |
   3495.4 |
   3505.5 |
   3515.6 |
   3525.6 |
   3535.7 |
  (0 below, 1 above range)

abi_soa_win_madd_scalar_payload (n=6, range 2737020.8-2815154.8 ns)
  2737020.8 |####################
  2740927.5 |
  2744834.2 |####################
  2748740.9 |
  2752647.6 |
  2756554.3 |########################################
  2760461.0 |####################
  2764367.7 |
  2768274.4 |
  2772181.1 |
  2776087.8 |
  2779994.5 |
  2783901.2 |
  2787807.9 |
  2791714.6 |
  2795621.3 |
  2799528.0 |
  2803434.7 |
  2807341.4 |
  2811248.1 |
  (0 below, 1 above range)

abi_soa_win_madd_soa_payload (n=6, range 2739823.3-2761518.0 ns)
  2739823.3 |####################
  2740908.0 |
  2741992.8 |
  2743077.5 |####################
  2744162.2 |
  2745247.0 |
  2746331.7 |
  2747416.4 |
  2748501.2 |
  2749585.9 |####################
  2750670.6 |
  2751755.4 |
  2752840.1 |
  2753924.8 |
  2755009.6 |
  2756094.3 |
  2757179.0 |
  2758263.8 |########################################
  2759348.5 |
  2760433.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_madd_null_entry**: bridge=3501.6% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_scalar_payload**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_soa_payload**: bridge=302.1% of algo (FFI overhead may distort results)
