# abi_native_cross (tight)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_tight_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_tight_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_tight_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_tight_native_ffi_w has the worst median (12.03 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_tight_null_entry at 2.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_tight_null_entry dominates: 376% faster than the next best (abi_native_cross_tight_inproc_native)

abi_native_cross_tight_null_entry (2.46 us) leads abi_native_cross_tight_inproc_native (11.71 us) by 376%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_tight_null_entry beats baseline by 79% (significant)

abi_native_cross_tight_null_entry is -9.54 us (79%) faster than baseline abi_native_cross_tight_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_tight_native_ffi_w is an outlier: 4.9x slower than the field

abi_native_cross_tight_native_ffi_w (12.03 us) is 4.9x the fastest (2.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.9x the fastest

Fastest abi_native_cross_tight_null_entry (2.46 us) to slowest abi_native_cross_tight_native_ffi_w (12.03 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_tight_null_entry** at 2462.1 ns median (-79.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.89x (fastest 2462.1 ns, slowest 12030.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 14012ns | 14076ns | 13374ns | 13966ns | 14400ns | -2.22% |
| abi_native_cross_tight_native_ffi_w | 14329ns | 14312ns | 13887ns | 14291ns | 14608ns | base |
| abi_native_cross_tight_null_entry | 4768ns | 4674ns | 4569ns | 4670ns | 5013ns | -66.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 11652ns | 11143ns | 11972ns | -3.33% | 0.005 |
| abi_native_cross_tight_native_ffi_w | 12054ns | 11661ns | 12317ns | base | 0.005 |
| abi_native_cross_tight_null_entry | 2481ns | 2405ns | 2566ns | -79.42% | 0.026 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 5219.4 | 11689.7 | 11652.5 | n/a |
| abi_native_cross_tight_native_ffi_w | 25187.7 | 12213.2 | 12053.9 | n/a |
| abi_native_cross_tight_null_entry | 26793.2 | 2716.7 | 2480.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_native_cross_tight_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_tight_inproc_native | 0.005 | 20.5% |
| abi_native_cross_tight_native_ffi_w | 0.005 | 20.0% |
| abi_native_cross_tight_null_entry | 0.026 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_tight_inproc_native | 14012ns | 14012ns | -2.22% |
| abi_native_cross_tight_native_ffi_w | 14329ns | 14329ns | base |
| abi_native_cross_tight_null_entry | 4768ns | 4768ns | -66.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_tight_native_ffi_w | 12031ns | base | --- | [11814, 12317] | --- | --- | --- | --- |
| abi_native_cross_tight_inproc_native | 11713ns | -411.3ns (-3.4%) | [-725, -68]ns | [11272, 11972] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_native_cross_tight_null_entry | 2462ns | -9544.1ns (-79.3%) | [-9828, -9348]ns | [2414, 2566] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_tight_native_ffi_w | abi_native_cross_tight_inproc_native | abi_native_cross_tight_null_entry |
|---|---|---|---|
| 1 | 11996ns | -1.9% | -80.0% |
| 2 | 12066ns | -5.5% | -79.7% |
| 3 | 12066ns | +0.8% | -78.4% |
| 4 | 12567ns | -6.2% | -79.9% |
| 5 | 11661ns | -4.4% | -79.2% |
| 6 | 11967ns | -2.5% | -79.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_tight_inproc_native | -0.263 | moderate- |
| abi_native_cross_tight_native_ffi_w | -0.378 | moderate- |
| abi_native_cross_tight_null_entry | 0.079 | ok |

**Consistency summary:**

- **abi_native_cross_tight_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 120721.0ns | 11652.5ns | 1036.0% | HIGH |
| abi_native_cross_tight_native_ffi_w | 140142.2ns | 12053.9ns | 1162.6% | HIGH |
| abi_native_cross_tight_null_entry | 112610.8ns | 2480.7ns | 4539.5% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_tight_inproc_native (n=6, range 11143.3-11972.1 ns)
  11143.3 |########################################
  11184.7 |
  11226.2 |
  11267.6 |
  11309.1 |
  11350.5 |
  11391.9 |########################################
  11433.4 |
  11474.8 |
  11516.3 |
  11557.7 |
  11599.1 |
  11640.6 |########################################
  11682.0 |
  11723.5 |########################################
  11764.9 |########################################
  11806.3 |
  11847.8 |
  11889.2 |
  11930.7 |
  (0 below, 1 above range)

abi_native_cross_tight_native_ffi_w (n=6, range 11661.2-12316.7 ns)
  11661.2 |####################
  11694.0 |
  11726.7 |
  11759.5 |
  11792.3 |
  11825.1 |
  11857.8 |
  11890.6 |
  11923.4 |
  11956.2 |####################
  11988.9 |####################
  12021.7 |
  12054.5 |########################################
  12087.2 |
  12120.0 |
  12152.8 |
  12185.6 |
  12218.3 |
  12251.1 |
  12283.9 |
  (0 below, 1 above range)

abi_native_cross_tight_null_entry (n=6, range 2405.0-2566.4 ns)
   2405.0 |########################################
   2413.1 |
   2421.1 |########################################
   2429.2 |
   2437.3 |
   2445.4 |
   2453.4 |########################################
   2461.5 |
   2469.6 |########################################
   2477.7 |
   2485.7 |
   2493.8 |
   2501.9 |
   2509.9 |
   2518.0 |########################################
   2526.1 |
   2534.2 |
   2542.2 |
   2550.3 |
   2558.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_tight_inproc_native**: bridge=1033.3% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_native_ffi_w**: bridge=1155.5% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_null_entry**: bridge=4557.1% of algo (FFI overhead may distort results)
