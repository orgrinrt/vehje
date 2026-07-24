# abi_soa_win (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_wideselect_scalar_payload has the worst median (2.07 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_wideselect_null_entry at 4.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_wideselect_null_entry dominates: 42298% faster than the next best (abi_soa_win_wideselect_soa_payload)

abi_soa_win_wideselect_null_entry (4.87 us) leads abi_soa_win_wideselect_soa_payload (2.07 ms) by 42298%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_wideselect_null_entry beats baseline by 100% (significant)

abi_soa_win_wideselect_null_entry is -2.06 ms (100%) faster than baseline abi_soa_win_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_wideselect_scalar_payload is an outlier: 424.3x slower than the field

abi_soa_win_wideselect_scalar_payload (2.07 ms) is 424.3x the fastest (4.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 424.3x the fastest

Fastest abi_soa_win_wideselect_null_entry (4.87 us) to slowest abi_soa_win_wideselect_scalar_payload (2.07 ms): 424.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_wideselect_null_entry** at 4872.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 424.32x (fastest 4872.9 ns, slowest 2067681.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 7210ns | 7119ns | 6965ns | 7075ns | 7534ns | -99.65% |
| abi_soa_win_wideselect_scalar_payload | 2073546ns | 2070230ns | 2062210ns | 2068047ns | 2087464ns | base |
| abi_soa_win_wideselect_soa_payload | 2085116ns | 2068623ns | 2065077ns | 2067592ns | 2121421ns | +0.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 4931ns | 4759ns | 5141ns | -99.76% | 0.000 |
| abi_soa_win_wideselect_scalar_payload | 2070896ns | 2059662ns | 2084599ns | base | 0.000 |
| abi_soa_win_wideselect_soa_payload | 2082502ns | 2062561ns | 2118675ns | +0.56% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 28829.4 | 4996.3 | 4931.0 | n/a |
| abi_soa_win_wideselect_scalar_payload | 44340.9 | 2073887.0 | 2070895.9 | n/a |
| abi_soa_win_wideselect_soa_payload | 43587.3 | 2091176.2 | 2082502.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_soa_win_wideselect_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.000 | 97.7% |
| abi_soa_win_wideselect_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_wideselect_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 7210ns | 7210ns | -99.65% |
| abi_soa_win_wideselect_scalar_payload | 2073546ns | 2073546ns | base |
| abi_soa_win_wideselect_soa_payload | 2085116ns | 2085116ns | +0.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_scalar_payload | 2067681ns | base | --- | [2060407, 2084599] | --- | --- | --- | --- |
| abi_soa_win_wideselect_null_entry | 4873ns | -2062902.1ns (-99.8%) | [-2079611, -2055381]ns | [4779, 5141] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_wideselect_soa_payload | 2066034ns | no significant difference | [-8132, +41247]ns | [2062798, 2118675] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_wideselect_scalar_payload | abi_soa_win_wideselect_null_entry | abi_soa_win_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2059662ns | -99.7% | +0.4% |
| 2 | 2095192ns | -99.8% | +3.5% |
| 3 | 2067854ns | -99.8% | -0.3% |
| 4 | 2061152ns | -99.8% | +0.3% |
| 5 | 2074006ns | -99.8% | -0.5% |
| 6 | 2067508ns | -99.8% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.226 | moderate+ |
| abi_soa_win_wideselect_scalar_payload | -0.425 | moderate- |
| abi_soa_win_wideselect_soa_payload | -0.221 | moderate- |

**Consistency summary:**

- **abi_soa_win_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_wideselect_soa_payload**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 125789.9ns | 4931.0ns | 2551.0% | HIGH |
| abi_soa_win_wideselect_scalar_payload | 6279818.9ns | 2070895.9ns | 303.2% | HIGH |
| abi_soa_win_wideselect_soa_payload | 6308579.4ns | 2082502.3ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_wideselect_null_entry (n=6, range 4758.7-5141.0 ns)
   4758.7 |########################################
   4777.8 |
   4796.9 |########################################
   4816.1 |########################################
   4835.2 |
   4854.3 |
   4873.4 |
   4892.5 |
   4911.6 |########################################
   4930.8 |
   4949.9 |
   4969.0 |
   4988.1 |
   5007.2 |
   5026.3 |
   5045.5 |########################################
   5064.6 |
   5083.7 |
   5102.8 |
   5121.9 |
  (0 below, 1 above range)

abi_soa_win_wideselect_scalar_payload (n=6, range 2059662.1-2084599.4 ns)
  2059662.1 |####################
  2060909.0 |####################
  2062155.8 |
  2063402.7 |
  2064649.6 |
  2065896.4 |
  2067143.3 |########################################
  2068390.1 |
  2069637.0 |
  2070883.9 |
  2072130.7 |
  2073377.6 |####################
  2074624.5 |
  2075871.3 |
  2077118.2 |
  2078365.0 |
  2079611.9 |
  2080858.8 |
  2082105.6 |
  2083352.5 |
  (0 below, 1 above range)

abi_soa_win_wideselect_soa_payload (n=6, range 2062561.2-2118674.8 ns)
  2062561.2 |########################################
  2065366.9 |#############
  2068172.6 |#############
  2070978.2 |
  2073783.9 |
  2076589.6 |
  2079395.3 |
  2082201.0 |
  2085006.6 |
  2087812.3 |
  2090618.0 |
  2093423.7 |
  2096229.4 |
  2099035.0 |
  2101840.7 |
  2104646.4 |
  2107452.1 |
  2110257.8 |
  2113063.4 |
  2115869.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_wideselect_null_entry**: bridge=2586.9% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_scalar_payload**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_soa_payload**: bridge=302.2% of algo (FFI overhead may distort results)
