# abi_residency (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_residency_leaf_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_leaf_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_leaf_null_entry dominates: 55543% faster than the next best (abi_residency_leaf_reused_buffer)

abi_residency_leaf_null_entry (2.60 us) leads abi_residency_leaf_reused_buffer (1.45 ms) by 55543%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_leaf_null_entry beats baseline by 100% (significant)

abi_residency_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_residency_leaf_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_leaf_fresh_alloc is an outlier: 557.8x slower than the field

abi_residency_leaf_fresh_alloc (1.45 ms) is 557.8x the fastest (2.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 557.8x the fastest

Fastest abi_residency_leaf_null_entry (2.60 us) to slowest abi_residency_leaf_fresh_alloc (1.45 ms): 557.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_leaf_null_entry** at 2602.3 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 557.76x (fastest 2602.3 ns, slowest 1451460.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1454392ns | 1453936ns | 1448861ns | 1453788ns | 1458062ns | +0.28% |
| abi_residency_leaf_null_entry | 4913ns | 4925ns | 4748ns | 4886ns | 5036ns | -99.66% |
| abi_residency_leaf_reused_buffer | 1450354ns | 1450400ns | 1446940ns | 1449294ns | 1453651ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1451883ns | 1446322ns | 1455503ns | +0.27% | 0.000 |
| abi_residency_leaf_null_entry | 2583ns | 2486ns | 2638ns | -99.82% | 0.006 |
| abi_residency_leaf_reused_buffer | 1447925ns | 1444600ns | 1451167ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 36085.0 | 1452303.9 | 1451883.4 | n/a |
| abi_residency_leaf_null_entry | 27493.4 | 2675.4 | 2583.4 | n/a |
| abi_residency_leaf_reused_buffer | 34441.0 | 1446864.0 | 1447924.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_residency_leaf_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.000 | 0.2% |
| abi_residency_leaf_null_entry | 0.006 | 95.5% |
| abi_residency_leaf_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1454392ns | 1454392ns | +0.28% |
| abi_residency_leaf_null_entry | 4913ns | 4913ns | -99.66% |
| abi_residency_leaf_reused_buffer | 1450354ns | 1450354ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_leaf_reused_buffer | 1447986ns | base | --- | [1444620, 1451167] | --- | --- | --- | --- |
| abi_residency_leaf_fresh_alloc | 1451460ns | no significant difference | [-146, +8549]ns | [1448687, 1455503] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_leaf_null_entry | 2602ns | -1445476.5ns (-99.8%) | [-1448529, -1442018]ns | [2510, 2638] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_leaf_reused_buffer | abi_residency_leaf_fresh_alloc | abi_residency_leaf_null_entry |
|---|---|---|---|
| 1 | 1444600ns | +0.8% | -99.8% |
| 2 | 1453027ns | -0.1% | -99.8% |
| 3 | 1449308ns | +0.4% | -99.8% |
| 4 | 1448153ns | +0.2% | -99.8% |
| 5 | 1444640ns | +0.1% | -99.8% |
| 6 | 1447820ns | +0.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_leaf_fresh_alloc | -0.048 | ok |
| abi_residency_leaf_null_entry | -0.069 | ok |
| abi_residency_leaf_reused_buffer | -0.201 | moderate- |

**Consistency summary:**

- **abi_residency_leaf_fresh_alloc**: won 1/6, lost 5/6
- **abi_residency_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 4394241.6ns | 1451883.4ns | 302.7% | HIGH |
| abi_residency_leaf_null_entry | 117875.4ns | 2583.4ns | 4562.8% | HIGH |
| abi_residency_leaf_reused_buffer | 4378412.1ns | 1447924.5ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_residency_leaf_fresh_alloc (n=6, range 1446322.5-1455502.7 ns)
  1446322.5 |####################
  1446781.5 |
  1447240.5 |
  1447699.5 |
  1448158.5 |
  1448617.6 |
  1449076.6 |
  1449535.6 |
  1449994.6 |
  1450453.6 |
  1450912.6 |########################################
  1451371.6 |####################
  1451830.6 |
  1452289.6 |
  1452748.6 |
  1453207.7 |
  1453666.7 |
  1454125.7 |
  1454584.7 |####################
  1455043.7 |
  (0 below, 1 above range)

abi_residency_leaf_null_entry (n=6, range 2485.8-2637.9 ns)
   2485.8 |########################################
   2493.4 |
   2501.0 |
   2508.6 |
   2516.2 |
   2523.8 |
   2531.4 |########################################
   2539.0 |
   2546.6 |
   2554.2 |
   2561.9 |
   2569.5 |
   2577.1 |
   2584.7 |########################################
   2592.3 |
   2599.9 |
   2607.5 |########################################
   2615.1 |########################################
   2622.7 |
   2630.3 |
  (0 below, 1 above range)

abi_residency_leaf_reused_buffer (n=6, range 1444600.0-1451167.1 ns)
  1444600.0 |########################################
  1444928.4 |
  1445256.7 |
  1445585.1 |
  1445913.4 |
  1446241.8 |
  1446570.1 |
  1446898.5 |
  1447226.8 |
  1447555.2 |####################
  1447883.6 |####################
  1448211.9 |
  1448540.3 |
  1448868.6 |
  1449197.0 |####################
  1449525.3 |
  1449853.7 |
  1450182.0 |
  1450510.4 |
  1450838.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_leaf_fresh_alloc**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_residency_leaf_null_entry**: bridge=4530.4% of algo (FFI overhead may distort results)
- **abi_residency_leaf_reused_buffer**: bridge=302.4% of algo (FFI overhead may distort results)
