# abi_soa_win (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_wideselect_scalar_payload has the worst median (2.06 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_wideselect_null_entry at 2.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_wideselect_null_entry dominates: 36201% faster than the next best (abi_soa_win_wideselect_soa_payload)

abi_soa_win_wideselect_null_entry (2.53 us) leads abi_soa_win_wideselect_soa_payload (916.89 us) by 36201%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_wideselect_null_entry beats baseline by 100% (significant)

abi_soa_win_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_soa_win_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_wideselect_scalar_payload is an outlier: 814.2x slower than the field

abi_soa_win_wideselect_scalar_payload (2.06 ms) is 814.2x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 814.2x the fastest

Fastest abi_soa_win_wideselect_null_entry (2.53 us) to slowest abi_soa_win_wideselect_scalar_payload (2.06 ms): 814.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_wideselect_null_entry** at 2525.8 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 814.20x (fastest 2525.8 ns, slowest 2056511.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 4839ns | 4815ns | 4669ns | 4790ns | 4999ns | -99.77% |
| abi_soa_win_wideselect_scalar_payload | 2059668ns | 2059083ns | 2050462ns | 2058482ns | 2066051ns | base |
| abi_soa_win_wideselect_soa_payload | 921372ns | 919301ns | 917138ns | 918854ns | 927266ns | -55.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 2536ns | 2439ns | 2621ns | -99.88% | 0.006 |
| abi_soa_win_wideselect_scalar_payload | 2056994ns | 2047710ns | 2063302ns | base | 0.000 |
| abi_soa_win_wideselect_soa_payload | 918932ns | 914701ns | 924776ns | -55.33% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 28279.1 | 2615.9 | 2536.3 | n/a |
| abi_soa_win_wideselect_scalar_payload | 42094.0 | 2055437.0 | 2056994.3 | n/a |
| abi_soa_win_wideselect_soa_payload | 34707.2 | 918449.1 | 918931.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_soa_win_wideselect_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.006 | 96.6% |
| abi_soa_win_wideselect_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_wideselect_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 4839ns | 4839ns | -99.77% |
| abi_soa_win_wideselect_scalar_payload | 2059668ns | 2059668ns | base |
| abi_soa_win_wideselect_soa_payload | 921372ns | 921372ns | -55.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_scalar_payload | 2056511ns | base | --- | [2051169, 2063302] | --- | --- | --- | --- |
| abi_soa_win_wideselect_null_entry | 2526ns | -2053939.4ns (-99.9%) | [-2060791, -2048644]ns | [2462, 2621] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_wideselect_soa_payload | 916893ns | -1140798.0ns (-55.5%) | [-1146410, -1126981]ns | [915126, 924776] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_wideselect_scalar_payload | abi_soa_win_wideselect_null_entry | abi_soa_win_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2055802ns | -99.9% | -55.2% |
| 2 | 2047710ns | -99.9% | -54.6% |
| 3 | 2059844ns | -99.9% | -55.5% |
| 4 | 2054628ns | -99.9% | -55.5% |
| 5 | 2066761ns | -99.9% | -55.6% |
| 6 | 2057220ns | -99.9% | -55.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.001 | ok |
| abi_soa_win_wideselect_scalar_payload | -0.219 | moderate- |
| abi_soa_win_wideselect_soa_payload | 0.073 | ok |

**Consistency summary:**

- **abi_soa_win_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 118426.0ns | 2536.3ns | 4669.2% | HIGH |
| abi_soa_win_wideselect_scalar_payload | 6212435.3ns | 2056994.3ns | 302.0% | HIGH |
| abi_soa_win_wideselect_soa_payload | 2790517.6ns | 918931.5ns | 303.7% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_wideselect_null_entry (n=6, range 2438.8-2621.1 ns)
   2438.8 |########################################
   2447.9 |
   2457.0 |
   2466.1 |
   2475.2 |
   2484.4 |########################################
   2493.5 |########################################
   2502.6 |
   2511.7 |
   2520.8 |
   2529.9 |
   2539.0 |
   2548.2 |########################################
   2557.3 |
   2566.4 |
   2575.5 |########################################
   2584.6 |
   2593.7 |
   2602.8 |
   2611.9 |
  (0 below, 1 above range)

abi_soa_win_wideselect_scalar_payload (n=6, range 2047710.4-2063302.4 ns)
  2047710.4 |########################################
  2048490.0 |
  2049269.6 |
  2050049.2 |
  2050828.8 |
  2051608.4 |
  2052388.0 |
  2053167.6 |
  2053947.2 |########################################
  2054726.8 |
  2055506.4 |########################################
  2056286.0 |
  2057065.6 |########################################
  2057845.2 |
  2058624.8 |
  2059404.4 |########################################
  2060184.0 |
  2060963.6 |
  2061743.2 |
  2062522.8 |
  (0 below, 1 above range)

abi_soa_win_wideselect_soa_payload (n=6, range 914701.2-924775.6 ns)
  914701.2 |########################################
  915204.9 |########################################
  915708.6 |
  916212.4 |########################################
  916716.1 |
  917219.8 |########################################
  917723.5 |
  918227.2 |
  918731.0 |
  919234.7 |
  919738.4 |
  920242.1 |########################################
  920745.8 |
  921249.6 |
  921753.3 |
  922257.0 |
  922760.7 |
  923264.4 |
  923768.2 |
  924271.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_wideselect_null_entry**: bridge=4678.0% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_scalar_payload**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_soa_payload**: bridge=303.7% of algo (FFI overhead may distort results)
