# abi_residency (madd)

3 variants, 6 samples per variant.
Baseline: **abi_residency_madd_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_madd_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_residency_madd_reused_buffer) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_residency_madd_reused_buffer has the worst median (2.68 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_residency_madd_null_entry at 2.64 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_residency_madd_null_entry dominates: 101244% faster than the next best (abi_residency_madd_fresh_alloc)

abi_residency_madd_null_entry (2.64 us) leads abi_residency_madd_fresh_alloc (2.68 ms) by 101244%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_madd_null_entry beats baseline by 100% (significant)

abi_residency_madd_null_entry is -2.68 ms (100%) faster than baseline abi_residency_madd_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_madd_reused_buffer is an outlier: 1014.7x slower than the field

abi_residency_madd_reused_buffer (2.68 ms) is 1014.7x the fastest (2.64 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 1014.7x the fastest

Fastest abi_residency_madd_null_entry (2.64 us) to slowest abi_residency_madd_reused_buffer (2.68 ms): 1014.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_madd_null_entry** at 2642.5 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1014.72x (fastest 2642.5 ns, slowest 2681385.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2684237ns | 2680717ns | 2674730ns | 2679315ns | 2696375ns | -0.06% |
| abi_residency_madd_null_entry | 5014ns | 4991ns | 4708ns | 4986ns | 5210ns | -99.81% |
| abi_residency_madd_reused_buffer | 2685802ns | 2683925ns | 2676676ns | 2682310ns | 2695605ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2681550ns | 2672141ns | 2693575ns | -0.06% | 0.000 |
| abi_residency_madd_null_entry | 2625ns | 2465ns | 2690ns | -99.90% | 0.006 |
| abi_residency_madd_reused_buffer | 2683205ns | 2674203ns | 2692954ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 43914.1 | 2685726.1 | 2681550.4 | 1 |
| abi_residency_madd_null_entry | 28757.5 | 2744.9 | 2624.7 | n/a |
| abi_residency_madd_reused_buffer | 42745.2 | 2683237.1 | 2683205.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_residency_madd_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.000 | 0.1% |
| abi_residency_madd_null_entry | 0.006 | 93.3% |
| abi_residency_madd_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2684237ns | 2684237ns | -0.06% |
| abi_residency_madd_null_entry | 5014ns | 5014ns | -99.81% |
| abi_residency_madd_reused_buffer | 2685802ns | 2685802ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_madd_reused_buffer | 2681385ns | base | --- | [2675276, 2692954] | --- | --- | --- | --- |
| abi_residency_madd_fresh_alloc | 2678020ns | no significant difference | [-17242, +13328]ns | [2673056, 2693575] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_madd_null_entry | 2642ns | -2678822.9ns (-99.9%) | [-2690272, -2672646]ns | [2542, 2690] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_madd_reused_buffer | abi_residency_madd_fresh_alloc | abi_residency_madd_null_entry |
|---|---|---|---|
| 1 | 2678627ns | -0.2% | -99.9% |
| 2 | 2676350ns | +0.6% | -99.9% |
| 3 | 2693342ns | -0.6% | -99.9% |
| 4 | 2684143ns | +0.4% | -99.9% |
| 5 | 2674203ns | +0.2% | -99.9% |
| 6 | 2692566ns | -0.7% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_madd_fresh_alloc | -0.467 | moderate- |
| abi_residency_madd_null_entry | -0.118 | ok |
| abi_residency_madd_reused_buffer | -0.357 | moderate- |

**Consistency summary:**

- **abi_residency_madd_fresh_alloc**: won 3/6, lost 3/6
- **abi_residency_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 8098117.6ns | 2681550.4ns | 302.0% | HIGH |
| abi_residency_madd_null_entry | 118809.5ns | 2624.7ns | 4526.5% | HIGH |
| abi_residency_madd_reused_buffer | 8090897.9ns | 2683205.2ns | 301.5% | HIGH |

## Distribution (algo ns)

```
abi_residency_madd_fresh_alloc (n=6, range 2672140.8-2693574.8 ns)
  2672140.8 |########################################
  2673212.5 |########################################
  2674284.2 |
  2675355.9 |
  2676427.6 |########################################
  2677499.3 |
  2678571.0 |########################################
  2679642.7 |
  2680714.4 |
  2681786.1 |
  2682857.8 |
  2683929.5 |
  2685001.2 |
  2686072.9 |
  2687144.6 |
  2688216.3 |
  2689288.0 |
  2690359.7 |
  2691431.4 |########################################
  2692503.1 |
  (0 below, 1 above range)

abi_residency_madd_null_entry (n=6, range 2464.6-2690.0 ns)
   2464.6 |####################
   2475.9 |
   2487.1 |
   2498.4 |
   2509.7 |
   2520.9 |
   2532.2 |
   2543.5 |
   2554.8 |
   2566.0 |
   2577.3 |
   2588.6 |
   2599.8 |
   2611.1 |####################
   2622.4 |
   2633.7 |########################################
   2644.9 |
   2656.2 |####################
   2667.5 |
   2678.7 |
  (0 below, 1 above range)

abi_residency_madd_reused_buffer (n=6, range 2674202.9-2692954.1 ns)
  2674202.9 |########################################
  2675140.5 |
  2676078.0 |########################################
  2677015.6 |
  2677953.1 |########################################
  2678890.7 |
  2679828.3 |
  2680765.8 |
  2681703.4 |
  2682641.0 |
  2683578.5 |########################################
  2684516.1 |
  2685453.6 |
  2686391.2 |
  2687328.8 |
  2688266.3 |
  2689203.9 |
  2690141.5 |
  2691079.0 |
  2692016.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_madd_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_madd_null_entry**: bridge=4495.1% of algo (FFI overhead may distort results)
- **abi_residency_madd_reused_buffer**: bridge=301.3% of algo (FFI overhead may distort results)
