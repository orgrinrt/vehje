# abi_native_cross (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_wideselect_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_wideselect_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_wideselect_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_wideselect_native_ffi_w has the worst median (12.39 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_wideselect_null_entry at 2.36 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_wideselect_null_entry dominates: 393% faster than the next best (abi_native_cross_wideselect_inproc_native)

abi_native_cross_wideselect_null_entry (2.36 us) leads abi_native_cross_wideselect_inproc_native (11.62 us) by 393%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_wideselect_null_entry beats baseline by 81% (significant)

abi_native_cross_wideselect_null_entry is -9.99 us (81%) faster than baseline abi_native_cross_wideselect_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_wideselect_native_ffi_w is an outlier: 5.3x slower than the field

abi_native_cross_wideselect_native_ffi_w (12.39 us) is 5.3x the fastest (2.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.3x the fastest

Fastest abi_native_cross_wideselect_null_entry (2.36 us) to slowest abi_native_cross_wideselect_native_ffi_w (12.39 us): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_wideselect_null_entry** at 2356.1 ns median (-81.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.26x (fastest 2356.1 ns, slowest 12389.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13935ns | 13930ns | 13463ns | 13817ns | 14347ns | -4.40% |
| abi_native_cross_wideselect_native_ffi_w | 14576ns | 14677ns | 14001ns | 14669ns | 14725ns | base |
| abi_native_cross_wideselect_null_entry | 4672ns | 4715ns | 4412ns | 4660ns | 4819ns | -67.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 11647ns | 11234ns | 12035ns | -5.35% | 0.003 |
| abi_native_cross_wideselect_native_ffi_w | 12305ns | 11841ns | 12413ns | base | 0.003 |
| abi_native_cross_wideselect_null_entry | 2354ns | 2235ns | 2438ns | -80.87% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 5278.1 | 11713.7 | 11646.7 | n/a |
| abi_native_cross_wideselect_native_ffi_w | 26605.3 | 12454.8 | 12304.9 | n/a |
| abi_native_cross_wideselect_null_entry | 28884.5 | 2451.9 | 2353.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_native_cross_wideselect_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.003 | 19.2% |
| abi_native_cross_wideselect_native_ffi_w | 0.003 | 18.0% |
| abi_native_cross_wideselect_null_entry | 0.014 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13935ns | 13935ns | -4.40% |
| abi_native_cross_wideselect_native_ffi_w | 14576ns | 14576ns | base |
| abi_native_cross_wideselect_null_entry | 4672ns | 4672ns | -67.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_native_ffi_w | 12390ns | base | --- | [12112, 12413] | --- | --- | --- | --- |
| abi_native_cross_wideselect_inproc_native | 11622ns | -690.8ns (-5.6%) | [-1108, -175]ns | [11284, 12035] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_wideselect_null_entry | 2356ns | -9994.9ns (-80.7%) | [-10095, -9763]ns | [2268, 2438] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_wideselect_native_ffi_w | abi_native_cross_wideselect_inproc_native | abi_native_cross_wideselect_null_entry |
|---|---|---|---|
| 1 | 11841ns | -1.2% | -81.1% |
| 2 | 12386ns | -9.3% | -80.8% |
| 3 | 12394ns | -6.8% | -81.2% |
| 4 | 12428ns | -4.3% | -81.5% |
| 5 | 12382ns | -1.6% | -80.1% |
| 6 | 12398ns | -8.6% | -80.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | -0.064 | ok |
| abi_native_cross_wideselect_native_ffi_w | -0.010 | ok |
| abi_native_cross_wideselect_null_entry | -0.064 | ok |

**Consistency summary:**

- **abi_native_cross_wideselect_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 121541.8ns | 11646.7ns | 1043.6% | HIGH |
| abi_native_cross_wideselect_native_ffi_w | 142764.2ns | 12304.9ns | 1160.2% | HIGH |
| abi_native_cross_wideselect_null_entry | 118811.0ns | 2353.9ns | 5047.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_wideselect_inproc_native (n=6, range 11234.2-12034.5 ns)
  11234.2 |########################################
  11274.2 |
  11314.2 |########################################
  11354.3 |
  11394.3 |
  11434.3 |
  11474.3 |
  11514.3 |########################################
  11554.3 |
  11594.4 |
  11634.4 |
  11674.4 |########################################
  11714.4 |
  11754.4 |
  11794.4 |
  11834.5 |
  11874.5 |########################################
  11914.5 |
  11954.5 |
  11994.5 |
  (0 below, 1 above range)

abi_native_cross_wideselect_native_ffi_w (n=6, range 11841.2-12413.3 ns)
  11841.2 |#############
  11869.8 |
  11898.4 |
  11927.0 |
  11955.6 |
  11984.2 |
  12012.8 |
  12041.4 |
  12070.0 |
  12098.6 |
  12127.2 |
  12155.9 |
  12184.5 |
  12213.1 |
  12241.7 |
  12270.3 |
  12298.9 |
  12327.5 |
  12356.1 |#############
  12384.7 |########################################
  (0 below, 1 above range)

abi_native_cross_wideselect_null_entry (n=6, range 2234.6-2437.5 ns)
   2234.6 |########################################
   2244.7 |
   2254.9 |
   2265.0 |
   2275.2 |
   2285.3 |
   2295.5 |########################################
   2305.6 |
   2315.8 |
   2325.9 |########################################
   2336.1 |
   2346.2 |
   2356.3 |
   2366.5 |
   2376.6 |########################################
   2386.8 |
   2396.9 |
   2407.1 |########################################
   2417.2 |
   2427.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_wideselect_inproc_native**: bridge=1051.7% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_native_ffi_w**: bridge=1157.2% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_null_entry**: bridge=5048.5% of algo (FFI overhead may distort results)
