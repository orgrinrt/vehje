# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (12.36 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 408% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.27 us) leads abi_native_cross_real_inproc_native (11.52 us) by 408%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 82% (significant)

abi_native_cross_real_null_entry is -10.09 us (82%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 5.4x slower than the field

abi_native_cross_real_native_ffi_w (12.36 us) is 5.4x the fastest (2.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.4x the fastest

Fastest abi_native_cross_real_null_entry (2.27 us) to slowest abi_native_cross_real_native_ffi_w (12.36 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2269.8 ns median (-81.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.44x (fastest 2269.8 ns, slowest 12357.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13874ns | 13815ns | 13556ns | 13750ns | 14218ns | -5.63% |
| abi_native_cross_real_native_ffi_w | 14701ns | 14680ns | 14324ns | 14588ns | 15060ns | base |
| abi_native_cross_real_null_entry | 4514ns | 4524ns | 4392ns | 4500ns | 4596ns | -69.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11554ns | 11286ns | 11843ns | -6.87% | 0.003 |
| abi_native_cross_real_native_ffi_w | 12405ns | 12091ns | 12741ns | base | 0.003 |
| abi_native_cross_real_null_entry | 2270ns | 2218ns | 2313ns | -81.70% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5364.3 | 11590.4 | 11553.7 | n/a |
| abi_native_cross_real_native_ffi_w | 25304.2 | 12578.7 | 12405.5 | n/a |
| abi_native_cross_real_null_entry | 27013.0 | 2402.6 | 2269.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.003 | 19.3% |
| abi_native_cross_real_native_ffi_w | 0.003 | 18.0% |
| abi_native_cross_real_null_entry | 0.014 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13874ns | 13874ns | -5.63% |
| abi_native_cross_real_native_ffi_w | 14701ns | 14701ns | base |
| abi_native_cross_real_null_entry | 4514ns | 4514ns | -69.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 12357ns | base | --- | [12118, 12741] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11522ns | -742.1ns (-6.0%) | [-1445, -368]ns | [11296, 11843] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 2270ns | -10087.5ns (-81.6%) | [-10429, -9891]ns | [2226, 2313] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 12145ns | -2.3% | -81.7% |
| 2 | 12501ns | -9.7% | -81.7% |
| 3 | 12982ns | -12.9% | -82.0% |
| 4 | 12277ns | -3.7% | -81.4% |
| 5 | 12091ns | -5.9% | -81.5% |
| 6 | 12438ns | -6.2% | -81.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.442 | moderate- |
| abi_native_cross_real_native_ffi_w | -0.026 | ok |
| abi_native_cross_real_null_entry | 0.170 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 121834.7ns | 11553.7ns | 1054.5% | HIGH |
| abi_native_cross_real_native_ffi_w | 142760.9ns | 12405.5ns | 1150.8% | HIGH |
| abi_native_cross_real_null_entry | 115285.5ns | 2269.6ns | 5079.6% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11286.2-11842.7 ns)
  11286.2 |########################################
  11314.0 |
  11341.9 |
  11369.7 |####################
  11397.5 |
  11425.3 |
  11453.2 |
  11481.0 |
  11508.8 |
  11536.6 |
  11564.5 |
  11592.3 |
  11620.1 |
  11647.9 |####################
  11675.8 |
  11703.6 |
  11731.4 |
  11759.2 |
  11787.1 |
  11814.9 |####################
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 12090.8-12741.5 ns)
  12090.8 |########################################
  12123.3 |########################################
  12155.9 |
  12188.4 |
  12220.9 |
  12253.5 |########################################
  12286.0 |
  12318.5 |
  12351.1 |
  12383.6 |
  12416.1 |########################################
  12448.7 |
  12481.2 |########################################
  12513.7 |
  12546.3 |
  12578.8 |
  12611.3 |
  12643.9 |
  12676.4 |
  12708.9 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2218.3-2312.7 ns)
   2218.3 |####################
   2223.0 |
   2227.7 |
   2232.5 |####################
   2237.2 |
   2241.9 |
   2246.6 |####################
   2251.3 |
   2256.1 |
   2260.8 |
   2265.5 |
   2270.2 |
   2274.9 |
   2279.7 |
   2284.4 |
   2289.1 |########################################
   2293.8 |
   2298.5 |
   2303.3 |
   2308.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1055.4% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1161.9% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=5080.9% of algo (FFI overhead may distort results)
