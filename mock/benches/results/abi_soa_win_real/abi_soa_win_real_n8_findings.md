# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 3.03 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 29522% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (3.03 us) leads abi_soa_win_real_soa_payload (897.61 us) by 29522%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 711.6x slower than the field

abi_soa_win_real_scalar_payload (2.16 ms) is 711.6x the fastest (3.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 711.6x the fastest

Fastest abi_soa_win_real_null_entry (3.03 us) to slowest abi_soa_win_real_scalar_payload (2.16 ms): 711.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 3030.2 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 711.56x (fastest 3030.2 ns, slowest 2156209.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 5301ns | 5310ns | 5168ns | 5283ns | 5395ns | -99.75% |
| abi_soa_win_real_scalar_payload | 2162984ns | 2159282ns | 2153766ns | 2158655ns | 2174085ns | base |
| abi_soa_win_real_soa_payload | 915508ns | 900330ns | 893859ns | 898785ns | 951416ns | -57.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 3021ns | 2945ns | 3080ns | -99.86% | 0.003 |
| abi_soa_win_real_scalar_payload | 2159958ns | 2150807ns | 2170997ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 912452ns | 891278ns | 947596ns | -57.76% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 28124.2 | 3130.4 | 3021.3 | n/a |
| abi_soa_win_real_scalar_payload | 56781.5 | 2160653.3 | 2159958.3 | n/a |
| abi_soa_win_real_soa_payload | 48539.8 | 909448.4 | 912452.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.003 | 97.2% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 5301ns | 5301ns | -99.75% |
| abi_soa_win_real_scalar_payload | 2162984ns | 2162984ns | base |
| abi_soa_win_real_soa_payload | 915508ns | 915508ns | -57.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2156209ns | base | --- | [2152669, 2170997] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 3030ns | -2153241.9ns (-99.9%) | [-2167918, -2149651]ns | [2953, 3080] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 897613ns | -1258238.6ns (-58.4%) | [-1264280, -1220000]ns | [892147, 947596] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2184386ns | -99.9% | -55.4% |
| 2 | 2157174ns | -99.9% | -58.5% |
| 3 | 2154530ns | -99.9% | -58.2% |
| 4 | 2157608ns | -99.9% | -58.6% |
| 5 | 2155244ns | -99.9% | -58.6% |
| 6 | 2150807ns | -99.9% | -57.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.020 | ok |
| abi_soa_win_real_scalar_payload | 0.019 | ok |
| abi_soa_win_real_soa_payload | -0.066 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 119450.3ns | 3021.3ns | 3953.7% | HIGH |
| abi_soa_win_real_scalar_payload | 6543622.9ns | 2159958.3ns | 303.0% | HIGH |
| abi_soa_win_real_soa_payload | 2782745.9ns | 912452.1ns | 305.0% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 2945.4-3080.2 ns)
   2945.4 |########################################
   2952.1 |
   2958.9 |########################################
   2965.6 |
   2972.4 |
   2979.1 |
   2985.8 |########################################
   2992.6 |
   2999.3 |
   3006.1 |
   3012.8 |
   3019.5 |
   3026.3 |
   3033.0 |
   3039.8 |
   3046.5 |
   3053.2 |
   3060.0 |
   3066.7 |########################################
   3073.5 |########################################
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2150807.1-2170997.2 ns)
  2150807.1 |####################
  2151816.6 |
  2152826.1 |
  2153835.6 |####################
  2154845.1 |####################
  2155854.6 |
  2156864.1 |########################################
  2157873.7 |
  2158883.2 |
  2159892.7 |
  2160902.2 |
  2161911.7 |
  2162921.2 |
  2163930.7 |
  2164940.2 |
  2165949.7 |
  2166959.2 |
  2167968.7 |
  2168978.2 |
  2169987.7 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 891277.9-947596.4 ns)
  891277.9 |########################################
  894093.8 |####################
  896909.8 |####################
  899725.7 |
  902541.6 |
  905357.5 |
  908173.5 |
  910989.4 |
  913805.3 |
  916621.2 |
  919437.2 |####################
  922253.1 |
  925069.0 |
  927885.0 |
  930700.9 |
  933516.8 |
  936332.7 |
  939148.7 |
  941964.6 |
  944780.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=3928.7% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=304.4% of algo (FFI overhead may distort results)
