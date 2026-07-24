# abi_native_cross (tight)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_tight_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_tight_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_tight_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_tight_native_ffi_w has the worst median (31.60 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_tight_null_entry at 4.98 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_tight_null_entry dominates: 134% faster than the next best (abi_native_cross_tight_inproc_native)

abi_native_cross_tight_null_entry (4.98 us) leads abi_native_cross_tight_inproc_native (11.65 us) by 134%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_tight_null_entry beats baseline by 84% (significant)

abi_native_cross_tight_null_entry is -26.66 us (84%) faster than baseline abi_native_cross_tight_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_tight_native_ffi_w is an outlier: 6.4x slower than the field

abi_native_cross_tight_native_ffi_w (31.60 us) is 6.4x the fastest (4.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 6.4x the fastest

Fastest abi_native_cross_tight_null_entry (4.98 us) to slowest abi_native_cross_tight_native_ffi_w (31.60 us): 6.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_tight_null_entry** at 4975.6 ns median (-84.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.35x (fastest 4975.6 ns, slowest 31598.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13973ns | 13948ns | 13384ns | 13932ns | 14331ns | -58.41% |
| abi_native_cross_tight_native_ffi_w | 33597ns | 33815ns | 28802ns | 32404ns | 37785ns | base |
| abi_native_cross_tight_null_entry | 7192ns | 7296ns | 6927ns | 7188ns | 7331ns | -78.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 11657ns | 11200ns | 11914ns | -62.84% | 0.000 |
| abi_native_cross_tight_native_ffi_w | 31368ns | 26592ns | 35544ns | base | 0.000 |
| abi_native_cross_tight_null_entry | 4911ns | 4748ns | 4999ns | -84.34% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 5312.8 | 11696.7 | 11656.6 | n/a |
| abi_native_cross_tight_native_ffi_w | 25104.1 | 31598.1 | 31368.0 | n/a |
| abi_native_cross_tight_null_entry | 26718.5 | 4962.8 | 4911.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_native_cross_tight_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_tight_inproc_native | 0.000 | 40.7% |
| abi_native_cross_tight_native_ffi_w | 0.000 | 15.0% |
| abi_native_cross_tight_null_entry | 0.000 | 95.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13973ns | 13973ns | -58.41% |
| abi_native_cross_tight_native_ffi_w | 33597ns | 33597ns | base |
| abi_native_cross_tight_null_entry | 7192ns | 7192ns | -78.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_tight_native_ffi_w | 31598ns | base | --- | [26962, 35544] | --- | --- | --- | --- |
| abi_native_cross_tight_inproc_native | 11655ns | -19965.4ns (-63.2%) | [-24121, -15048]ns | [11400, 11914] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_tight_null_entry | 4976ns | -26664.6ns (-84.4%) | [-30739, -21967]ns | [4759, 4999] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_tight_native_ffi_w | abi_native_cross_tight_inproc_native | abi_native_cross_tight_null_entry |
|---|---|---|---|
| 1 | 35401ns | -67.2% | -86.6% |
| 2 | 27796ns | -58.0% | -82.1% |
| 3 | 27332ns | -55.6% | -81.8% |
| 4 | 35595ns | -67.3% | -86.6% |
| 5 | 35492ns | -68.4% | -85.9% |
| 6 | 26592ns | -56.0% | -81.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_tight_inproc_native | -0.038 | ok |
| abi_native_cross_tight_native_ffi_w | -0.188 | ok |
| abi_native_cross_tight_null_entry | -0.271 | moderate- |

**Consistency summary:**

- **abi_native_cross_tight_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 122284.6ns | 11656.6ns | 1049.1% | HIGH |
| abi_native_cross_tight_native_ffi_w | 196272.9ns | 31368.0ns | 625.7% | HIGH |
| abi_native_cross_tight_null_entry | 122807.7ns | 4911.2ns | 2500.5% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_tight_inproc_native (n=6, range 11199.6-11914.3 ns)
  11199.6 |########################################
  11235.3 |
  11271.1 |
  11306.8 |
  11342.5 |
  11378.3 |
  11414.0 |
  11449.8 |
  11485.5 |
  11521.2 |
  11557.0 |
  11592.7 |########################################
  11628.4 |########################################
  11664.2 |########################################
  11699.9 |########################################
  11735.7 |
  11771.4 |
  11807.1 |
  11842.9 |
  11878.6 |
  (0 below, 1 above range)

abi_native_cross_tight_native_ffi_w (n=6, range 26591.7-35543.6 ns)
  26591.7 |####################
  27039.3 |####################
  27486.9 |####################
  27934.5 |
  28382.1 |
  28829.7 |
  29277.3 |
  29724.8 |
  30172.4 |
  30620.0 |
  31067.6 |
  31515.2 |
  31962.8 |
  32410.4 |
  32858.0 |
  33305.6 |
  33753.2 |
  34200.8 |
  34648.4 |
  35096.0 |########################################
  (0 below, 1 above range)

abi_native_cross_tight_null_entry (n=6, range 4747.9-4998.9 ns)
   4747.9 |########################################
   4760.5 |########################################
   4773.0 |
   4785.6 |
   4798.1 |
   4810.7 |
   4823.2 |
   4835.8 |
   4848.3 |
   4860.9 |
   4873.4 |
   4886.0 |
   4898.5 |
   4911.1 |
   4923.6 |
   4936.2 |
   4948.7 |
   4961.3 |########################################
   4973.8 |########################################
   4986.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_tight_inproc_native**: bridge=1052.9% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_native_ffi_w**: bridge=622.8% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_null_entry**: bridge=2474.6% of algo (FFI overhead may distort results)
