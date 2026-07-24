# abi_residency (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_residency_leaf_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_leaf_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_residency_leaf_reused_buffer) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_residency_leaf_reused_buffer has the worst median (1.45 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_residency_leaf_null_entry at 3.45 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_residency_leaf_null_entry dominates: 42037% faster than the next best (abi_residency_leaf_fresh_alloc)

abi_residency_leaf_null_entry (3.45 us) leads abi_residency_leaf_fresh_alloc (1.45 ms) by 42037%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_leaf_null_entry beats baseline by 100% (significant)

abi_residency_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_residency_leaf_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_leaf_reused_buffer is an outlier: 422.0x slower than the field

abi_residency_leaf_reused_buffer (1.45 ms) is 422.0x the fastest (3.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 422.0x the fastest

Fastest abi_residency_leaf_null_entry (3.45 us) to slowest abi_residency_leaf_reused_buffer (1.45 ms): 422.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_leaf_null_entry** at 3445.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 421.96x (fastest 3445.2 ns, slowest 1453722.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1454416ns | 1454237ns | 1450090ns | 1453409ns | 1458090ns | +0.06% |
| abi_residency_leaf_null_entry | 5757ns | 5690ns | 5560ns | 5662ns | 5997ns | -99.60% |
| abi_residency_leaf_reused_buffer | 1453560ns | 1456237ns | 1439701ns | 1454008ns | 1459816ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1451946ns | 1447702ns | 1455599ns | +0.06% | 0.000 |
| abi_residency_leaf_null_entry | 3482ns | 3360ns | 3624ns | -99.76% | 0.001 |
| abi_residency_leaf_reused_buffer | 1451072ns | 1437285ns | 1457299ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 37488.0 | 1453185.2 | 1451945.7 | n/a |
| abi_residency_leaf_null_entry | 27307.4 | 3543.2 | 3482.2 | n/a |
| abi_residency_leaf_reused_buffer | 37111.8 | 1451542.8 | 1451072.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_leaf_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.000 | 0.2% |
| abi_residency_leaf_null_entry | 0.001 | 97.5% |
| abi_residency_leaf_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1454416ns | 1454416ns | +0.06% |
| abi_residency_leaf_null_entry | 5757ns | 5757ns | -99.60% |
| abi_residency_leaf_reused_buffer | 1453560ns | 1453560ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_leaf_reused_buffer | 1453722ns | base | --- | [1442195, 1457299] | --- | --- | --- | --- |
| abi_residency_leaf_fresh_alloc | 1451705ns | no significant difference | [-8767, +13404]ns | [1448533, 1455599] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_leaf_null_entry | 3445ns | -1450300.9ns (-99.8%) | [-1453795, -1438674]ns | [3378, 3624] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_leaf_reused_buffer | abi_residency_leaf_fresh_alloc | abi_residency_leaf_null_entry |
|---|---|---|---|
| 1 | 1453419ns | +0.0% | -99.8% |
| 2 | 1454794ns | -0.4% | -99.8% |
| 3 | 1454026ns | -0.3% | -99.8% |
| 4 | 1459805ns | -0.8% | -99.8% |
| 5 | 1437285ns | +1.3% | -99.8% |
| 6 | 1447105ns | +0.6% | -99.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.132 | ok |
| abi_residency_leaf_null_entry | -0.221 | moderate- |
| abi_residency_leaf_reused_buffer | -0.065 | ok |

**Consistency summary:**

- **abi_residency_leaf_fresh_alloc**: won 3/6, lost 2/6
- **abi_residency_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 4397723.8ns | 1451945.7ns | 302.9% | HIGH |
| abi_residency_leaf_null_entry | 120499.8ns | 3482.2ns | 3460.4% | HIGH |
| abi_residency_leaf_reused_buffer | 4393399.1ns | 1451072.3ns | 302.8% | HIGH |

## Distribution (algo ns)

```
abi_residency_leaf_fresh_alloc (n=6, range 1447702.5-1455599.2 ns)
  1447702.5 |####################
  1448097.3 |
  1448492.2 |
  1448887.0 |
  1449281.8 |########################################
  1449676.7 |
  1450071.5 |
  1450466.3 |
  1450861.2 |
  1451256.0 |
  1451650.9 |
  1452045.7 |
  1452440.5 |
  1452835.4 |
  1453230.2 |
  1453625.0 |####################
  1454019.9 |
  1454414.7 |
  1454809.5 |
  1455204.4 |####################
  (0 below, 1 above range)

abi_residency_leaf_null_entry (n=6, range 3360.0-3623.6 ns)
   3360.0 |########################################
   3373.2 |
   3386.4 |########################################
   3399.5 |########################################
   3412.7 |
   3425.9 |
   3439.1 |
   3452.2 |
   3465.4 |
   3478.6 |########################################
   3491.8 |
   3505.0 |
   3518.1 |
   3531.3 |
   3544.5 |
   3557.7 |
   3570.8 |
   3584.0 |
   3597.2 |########################################
   3610.4 |
  (0 below, 1 above range)

abi_residency_leaf_reused_buffer (n=6, range 1437285.4-1457299.4 ns)
  1437285.4 |####################
  1438286.1 |
  1439286.8 |
  1440287.5 |
  1441288.2 |
  1442288.9 |
  1443289.6 |
  1444290.3 |
  1445291.0 |
  1446291.7 |####################
  1447292.4 |
  1448293.1 |
  1449293.8 |
  1450294.5 |
  1451295.2 |
  1452295.9 |
  1453296.6 |########################################
  1454297.3 |####################
  1455298.0 |
  1456298.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_leaf_fresh_alloc**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_residency_leaf_null_entry**: bridge=3497.3% of algo (FFI overhead may distort results)
- **abi_residency_leaf_reused_buffer**: bridge=302.6% of algo (FFI overhead may distort results)
