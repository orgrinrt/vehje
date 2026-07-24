# abi_residency (madd)

3 variants, 6 samples per variant.
Baseline: **abi_residency_madd_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_madd_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_residency_madd_reused_buffer) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_residency_madd_reused_buffer has the worst median (2.68 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_residency_madd_null_entry at 3.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_residency_madd_null_entry dominates: 84981% faster than the next best (abi_residency_madd_fresh_alloc)

abi_residency_madd_null_entry (3.16 us) leads abi_residency_madd_fresh_alloc (2.68 ms) by 84981%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_madd_null_entry beats baseline by 100% (significant)

abi_residency_madd_null_entry is -2.68 ms (100%) faster than baseline abi_residency_madd_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_madd_reused_buffer is an outlier: 850.8x slower than the field

abi_residency_madd_reused_buffer (2.68 ms) is 850.8x the fastest (3.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 850.8x the fastest

Fastest abi_residency_madd_null_entry (3.16 us) to slowest abi_residency_madd_reused_buffer (2.68 ms): 850.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_madd_null_entry** at 3155.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 850.84x (fastest 3155.4 ns, slowest 2684735.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2719730ns | 2687154ns | 2677299ns | 2684150ns | 2794315ns | +0.56% |
| abi_residency_madd_null_entry | 5466ns | 5505ns | 5166ns | 5441ns | 5655ns | -99.80% |
| abi_residency_madd_reused_buffer | 2704658ns | 2687291ns | 2672780ns | 2684591ns | 2750696ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2716987ns | 2674252ns | 2791401ns | +0.56% | 0.000 |
| abi_residency_madd_null_entry | 3129ns | 2953ns | 3226ns | -99.88% | 0.003 |
| abi_residency_madd_reused_buffer | 2701956ns | 2670284ns | 2747652ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 47928.8 | 2715533.9 | 2716987.1 | n/a |
| abi_residency_madd_null_entry | 27895.0 | 3221.9 | 3129.1 | n/a |
| abi_residency_madd_reused_buffer | 46112.4 | 2705366.0 | 2701955.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_residency_madd_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.000 | 0.1% |
| abi_residency_madd_null_entry | 0.003 | 93.6% |
| abi_residency_madd_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2719730ns | 2719730ns | +0.56% |
| abi_residency_madd_null_entry | 5466ns | 5466ns | -99.80% |
| abi_residency_madd_reused_buffer | 2704658ns | 2704658ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_madd_reused_buffer | 2684736ns | base | --- | [2673480, 2747652] | --- | --- | --- | --- |
| abi_residency_madd_fresh_alloc | 2684651ns | no significant difference | [-14977, +56031]ns | [2674909, 2791401] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_madd_null_entry | 3155ns | -2681520.8ns (-99.9%) | [-2744621, -2670338]ns | [3006, 3226] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_madd_reused_buffer | abi_residency_madd_fresh_alloc | abi_residency_madd_null_entry |
|---|---|---|---|
| 1 | 2670284ns | +0.2% | -99.9% |
| 2 | 2676676ns | -0.0% | -99.9% |
| 3 | 2792208ns | +3.3% | -99.9% |
| 4 | 2703096ns | -1.1% | -99.9% |
| 5 | 2690941ns | +0.1% | -99.9% |
| 6 | 2678530ns | +0.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_madd_fresh_alloc | -0.322 | moderate- |
| abi_residency_madd_null_entry | 0.248 | moderate+ |
| abi_residency_madd_reused_buffer | -0.108 | ok |

**Consistency summary:**

- **abi_residency_madd_fresh_alloc**: won 1/6, lost 3/6
- **abi_residency_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 8196686.1ns | 2716987.1ns | 301.7% | HIGH |
| abi_residency_madd_null_entry | 120097.1ns | 3129.1ns | 3838.1% | HIGH |
| abi_residency_madd_reused_buffer | 8159278.5ns | 2701955.8ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_residency_madd_fresh_alloc (n=6, range 2674252.1-2791400.6 ns)
  2674252.1 |########################################
  2680109.5 |
  2685967.0 |
  2691824.4 |#############
  2697681.8 |#############
  2703539.2 |
  2709396.6 |
  2715254.1 |
  2721111.5 |
  2726968.9 |
  2732826.3 |
  2738683.8 |
  2744541.2 |
  2750398.6 |
  2756256.0 |
  2762113.5 |
  2767970.9 |
  2773828.3 |
  2779685.7 |
  2785543.2 |
  (0 below, 1 above range)

abi_residency_madd_null_entry (n=6, range 2953.3-3225.8 ns)
   2953.3 |########################################
   2966.9 |
   2980.6 |
   2994.2 |
   3007.8 |
   3021.4 |
   3035.1 |
   3048.7 |########################################
   3062.3 |
   3075.9 |
   3089.6 |
   3103.2 |########################################
   3116.8 |
   3130.5 |
   3144.1 |
   3157.7 |
   3171.3 |
   3185.0 |
   3198.6 |########################################
   3212.2 |########################################
  (0 below, 1 above range)

abi_residency_madd_reused_buffer (n=6, range 2670283.7-2747652.0 ns)
  2670283.7 |########################################
  2674152.1 |########################################
  2678020.5 |########################################
  2681889.0 |
  2685757.4 |
  2689625.8 |########################################
  2693494.2 |
  2697362.6 |
  2701231.0 |########################################
  2705099.5 |
  2708967.9 |
  2712836.3 |
  2716704.7 |
  2720573.1 |
  2724441.5 |
  2728310.0 |
  2732178.4 |
  2736046.8 |
  2739915.2 |
  2743783.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_madd_fresh_alloc**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_residency_madd_null_entry**: bridge=3783.8% of algo (FFI overhead may distort results)
- **abi_residency_madd_reused_buffer**: bridge=302.1% of algo (FFI overhead may distort results)
