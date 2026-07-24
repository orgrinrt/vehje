# abi_native_cross (madd)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_madd_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_madd_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_madd_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_madd_native_ffi_w has the worst median (11.97 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_madd_null_entry at 2.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_madd_null_entry dominates: 378% faster than the next best (abi_native_cross_madd_inproc_native)

abi_native_cross_madd_null_entry (2.46 us) leads abi_native_cross_madd_inproc_native (11.72 us) by 378%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_madd_null_entry beats baseline by 80% (significant)

abi_native_cross_madd_null_entry is -9.52 us (80%) faster than baseline abi_native_cross_madd_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_madd_native_ffi_w is an outlier: 4.9x slower than the field

abi_native_cross_madd_native_ffi_w (11.97 us) is 4.9x the fastest (2.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.9x the fastest

Fastest abi_native_cross_madd_null_entry (2.46 us) to slowest abi_native_cross_madd_native_ffi_w (11.97 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_madd_null_entry** at 2455.0 ns median (-79.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.87x (fastest 2455.0 ns, slowest 11967.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 14060ns | 14069ns | 13631ns | 13967ns | 14414ns | -1.22% |
| abi_native_cross_madd_native_ffi_w | 14233ns | 14225ns | 13737ns | 14159ns | 14592ns | base |
| abi_native_cross_madd_null_entry | 4681ns | 4640ns | 4572ns | 4624ns | 4819ns | -67.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 11745ns | 11383ns | 12079ns | -1.74% | 0.005 |
| abi_native_cross_madd_native_ffi_w | 11953ns | 11525ns | 12268ns | base | 0.005 |
| abi_native_cross_madd_null_entry | 2453ns | 2371ns | 2521ns | -79.48% | 0.026 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 5283.0 | 11801.8 | 11745.2 | n/a |
| abi_native_cross_madd_native_ffi_w | 24989.2 | 12089.0 | 11953.2 | n/a |
| abi_native_cross_madd_null_entry | 26165.1 | 2706.6 | 2453.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_native_cross_madd_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_madd_inproc_native | 0.005 | 20.2% |
| abi_native_cross_madd_native_ffi_w | 0.005 | 19.8% |
| abi_native_cross_madd_null_entry | 0.026 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_madd_inproc_native | 14060ns | 14060ns | -1.22% |
| abi_native_cross_madd_native_ffi_w | 14233ns | 14233ns | base |
| abi_native_cross_madd_null_entry | 4681ns | 4681ns | -67.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_madd_native_ffi_w | 11967ns | base | --- | [11625, 12268] | --- | --- | --- | --- |
| abi_native_cross_madd_inproc_native | 11725ns | no significant difference | [-732, +291]ns | [11432, 12079] | no | 1.0000 | 1.0000 | 0 |
| abi_native_cross_madd_null_entry | 2455ns | -9520.0ns (-79.6%) | [-9813, -9167]ns | [2384, 2521] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_madd_native_ffi_w | abi_native_cross_madd_inproc_native | abi_native_cross_madd_null_entry |
|---|---|---|---|
| 1 | 11525ns | +3.4% | -79.2% |
| 2 | 12351ns | -7.8% | -79.9% |
| 3 | 12051ns | +1.6% | -79.1% |
| 4 | 11724ns | +0.3% | -78.5% |
| 5 | 11883ns | -3.4% | -80.0% |
| 6 | 12185ns | -4.1% | -80.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_madd_inproc_native | -0.461 | moderate- |
| abi_native_cross_madd_native_ffi_w | -0.333 | moderate- |
| abi_native_cross_madd_null_entry | 0.062 | ok |

**Consistency summary:**

- **abi_native_cross_madd_inproc_native**: won 3/6, lost 3/6
- **abi_native_cross_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 122893.6ns | 11745.2ns | 1046.3% | HIGH |
| abi_native_cross_madd_native_ffi_w | 140226.7ns | 11953.2ns | 1173.1% | HIGH |
| abi_native_cross_madd_null_entry | 111703.2ns | 2453.3ns | 4553.1% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_madd_inproc_native (n=6, range 11383.3-12079.0 ns)
  11383.3 |########################################
  11418.1 |
  11452.9 |########################################
  11487.6 |
  11522.4 |
  11557.2 |
  11592.0 |
  11626.8 |
  11661.6 |########################################
  11696.3 |
  11731.1 |########################################
  11765.9 |
  11800.7 |
  11835.5 |
  11870.3 |
  11905.0 |########################################
  11939.8 |
  11974.6 |
  12009.4 |
  12044.2 |
  (0 below, 1 above range)

abi_native_cross_madd_native_ffi_w (n=6, range 11525.4-12268.1 ns)
  11525.4 |########################################
  11562.5 |
  11599.7 |
  11636.8 |
  11673.9 |
  11711.1 |########################################
  11748.2 |
  11785.3 |
  11822.5 |
  11859.6 |########################################
  11896.8 |
  11933.9 |
  11971.0 |
  12008.2 |
  12045.3 |########################################
  12082.4 |
  12119.6 |
  12156.7 |########################################
  12193.8 |
  12231.0 |
  (0 below, 1 above range)

abi_native_cross_madd_null_entry (n=6, range 2370.8-2521.4 ns)
   2370.8 |########################################
   2378.3 |
   2385.9 |
   2393.4 |########################################
   2400.9 |
   2408.5 |
   2416.0 |
   2423.5 |
   2431.1 |########################################
   2438.6 |
   2446.1 |
   2453.7 |
   2461.2 |
   2468.7 |
   2476.3 |########################################
   2483.8 |
   2491.3 |
   2498.9 |
   2506.4 |
   2513.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_madd_inproc_native**: bridge=1051.0% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_native_ffi_w**: bridge=1172.8% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_null_entry**: bridge=4539.7% of algo (FFI overhead may distort results)
