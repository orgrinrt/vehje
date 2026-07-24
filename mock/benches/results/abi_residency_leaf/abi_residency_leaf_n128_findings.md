# abi_residency (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_residency_leaf_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_leaf_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_leaf_null_entry dominates: 53279% faster than the next best (abi_residency_leaf_reused_buffer)

abi_residency_leaf_null_entry (2.71 us) leads abi_residency_leaf_reused_buffer (1.45 ms) by 53279%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_leaf_null_entry beats baseline by 100% (significant)

abi_residency_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_residency_leaf_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_leaf_fresh_alloc is an outlier: 536.5x slower than the field

abi_residency_leaf_fresh_alloc (1.46 ms) is 536.5x the fastest (2.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_leaf_fresh_alloc shows warm-up / thermal drift (autocorr +0.56)

abi_residency_leaf_fresh_alloc's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 536.5x the fastest

Fastest abi_residency_leaf_null_entry (2.71 us) to slowest abi_residency_leaf_fresh_alloc (1.46 ms): 536.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_leaf_null_entry** at 2712.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 536.50x (fastest 2712.5 ns, slowest 1455264.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1457065ns | 1457750ns | 1454360ns | 1456740ns | 1458904ns | +0.40% |
| abi_residency_leaf_null_entry | 4983ns | 4952ns | 4823ns | 4920ns | 5158ns | -99.66% |
| abi_residency_leaf_reused_buffer | 1451295ns | 1450488ns | 1448508ns | 1449868ns | 1454829ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1454553ns | 1451697ns | 1456411ns | +0.40% | 0.000 |
| abi_residency_leaf_null_entry | 2722ns | 2648ns | 2806ns | -99.81% | 0.047 |
| abi_residency_leaf_reused_buffer | 1448784ns | 1446149ns | 1452259ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 36828.8 | 1454315.6 | 1454552.6 | n/a |
| abi_residency_leaf_null_entry | 26890.3 | 2735.3 | 2722.4 | n/a |
| abi_residency_leaf_reused_buffer | 35752.8 | 1450428.0 | 1448783.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_residency_leaf_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.000 | 0.2% |
| abi_residency_leaf_null_entry | 0.047 | 97.6% |
| abi_residency_leaf_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1457065ns | 1457065ns | +0.40% |
| abi_residency_leaf_null_entry | 4983ns | 4983ns | -99.66% |
| abi_residency_leaf_reused_buffer | 1451295ns | 1451295ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_leaf_reused_buffer | 1447908ns | base | --- | [1446184, 1452259] | --- | --- | --- | --- |
| abi_residency_leaf_fresh_alloc | 1455264ns | +7383.3ns (+0.5%) | [+843, +9080]ns | [1451982, 1456411] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_residency_leaf_null_entry | 2712ns | -1445201.2ns (-99.8%) | [-1449546, -1443438]ns | [2649, 2806] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_leaf_reused_buffer | abi_residency_leaf_fresh_alloc | abi_residency_leaf_null_entry |
|---|---|---|---|
| 1 | 1448711ns | +0.5% | -99.8% |
| 2 | 1449345ns | +0.5% | -99.8% |
| 3 | 1446220ns | +0.7% | -99.8% |
| 4 | 1446149ns | +0.6% | -99.8% |
| 5 | 1455174ns | -0.2% | -99.8% |
| 6 | 1447106ns | +0.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.559 | HIGH+ (drift/warm-up) |
| abi_residency_leaf_null_entry | -0.146 | ok |
| abi_residency_leaf_reused_buffer | -0.388 | moderate- |

**Consistency summary:**

- **abi_residency_leaf_fresh_alloc**: won 1/6, lost 5/6
- **abi_residency_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 4399765.3ns | 1454552.6ns | 302.5% | HIGH |
| abi_residency_leaf_null_entry | 119344.2ns | 2722.4ns | 4383.8% | HIGH |
| abi_residency_leaf_reused_buffer | 4387963.1ns | 1448783.9ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_residency_leaf_fresh_alloc (n=6, range 1451697.1-1456411.2 ns)
  1451697.1 |########################################
  1451932.8 |
  1452168.5 |########################################
  1452404.2 |
  1452639.9 |
  1452875.6 |
  1453111.3 |
  1453347.1 |
  1453582.8 |
  1453818.5 |
  1454054.2 |
  1454289.9 |
  1454525.6 |########################################
  1454761.3 |
  1454997.0 |
  1455232.7 |
  1455468.4 |
  1455704.1 |########################################
  1455939.8 |
  1456175.5 |########################################
  (0 below, 1 above range)

abi_residency_leaf_null_entry (n=6, range 2648.3-2805.8 ns)
   2648.3 |########################################
   2656.2 |####################
   2664.1 |
   2671.9 |
   2679.8 |
   2687.7 |
   2695.6 |
   2703.4 |
   2711.3 |
   2719.2 |
   2727.1 |
   2735.0 |
   2742.8 |
   2750.7 |
   2758.6 |####################
   2766.5 |####################
   2774.3 |
   2782.2 |
   2790.1 |
   2798.0 |
  (0 below, 1 above range)

abi_residency_leaf_reused_buffer (n=6, range 1446148.8-1452259.4 ns)
  1446148.8 |########################################
  1446454.3 |
  1446759.9 |
  1447065.4 |####################
  1447370.9 |
  1447676.4 |
  1447982.0 |
  1448287.5 |
  1448593.0 |####################
  1448898.5 |
  1449204.1 |####################
  1449509.6 |
  1449815.1 |
  1450120.7 |
  1450426.2 |
  1450731.7 |
  1451037.2 |
  1451342.8 |
  1451648.3 |
  1451953.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_leaf_fresh_alloc**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **abi_residency_leaf_fresh_alloc**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_residency_leaf_null_entry**: bridge=4379.7% of algo (FFI overhead may distort results)
- **abi_residency_leaf_reused_buffer**: bridge=302.8% of algo (FFI overhead may distort results)
