# abi_residency (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_residency_leaf_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_leaf_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_leaf_null_entry dominates: 62572% faster than the next best (abi_residency_leaf_reused_buffer)

abi_residency_leaf_null_entry (2.31 us) leads abi_residency_leaf_reused_buffer (1.45 ms) by 62572%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_leaf_null_entry beats baseline by 100% (significant)

abi_residency_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_residency_leaf_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_leaf_fresh_alloc is an outlier: 627.9x slower than the field

abi_residency_leaf_fresh_alloc (1.45 ms) is 627.9x the fastest (2.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 627.9x the fastest

Fastest abi_residency_leaf_null_entry (2.31 us) to slowest abi_residency_leaf_fresh_alloc (1.45 ms): 627.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_leaf_null_entry** at 2309.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 627.91x (fastest 2309.6 ns, slowest 1450216.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1454716ns | 1452752ns | 1447316ns | 1452079ns | 1462371ns | +0.30% |
| abi_residency_leaf_null_entry | 4622ns | 4595ns | 4422ns | 4552ns | 4825ns | -99.68% |
| abi_residency_leaf_reused_buffer | 1450436ns | 1449925ns | 1448374ns | 1449601ns | 1452720ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1452177ns | 1444817ns | 1459743ns | +0.29% | 0.000 |
| abi_residency_leaf_null_entry | 2326ns | 2230ns | 2425ns | -99.84% | 0.014 |
| abi_residency_leaf_reused_buffer | 1447972ns | 1445965ns | 1450229ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 36908.8 | 1451243.2 | 1452177.4 | n/a |
| abi_residency_leaf_null_entry | 27336.8 | 2440.8 | 2325.8 | n/a |
| abi_residency_leaf_reused_buffer | 36216.0 | 1448400.6 | 1447972.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_residency_leaf_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.000 | 0.2% |
| abi_residency_leaf_null_entry | 0.014 | 96.6% |
| abi_residency_leaf_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1454716ns | 1454716ns | +0.30% |
| abi_residency_leaf_null_entry | 4622ns | 4622ns | -99.68% |
| abi_residency_leaf_reused_buffer | 1450436ns | 1450436ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_leaf_reused_buffer | 1447483ns | base | --- | [1446205, 1450229] | --- | --- | --- | --- |
| abi_residency_leaf_fresh_alloc | 1450216ns | no significant difference | [-520, +10542]ns | [1446572, 1459743] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_leaf_null_entry | 2310ns | -1445215.6ns (-99.8%) | [-1447945, -1443780]ns | [2242, 2425] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_leaf_reused_buffer | abi_residency_leaf_fresh_alloc | abi_residency_leaf_null_entry |
|---|---|---|---|
| 1 | 1451658ns | +0.2% | -99.8% |
| 2 | 1446446ns | +0.1% | -99.8% |
| 3 | 1446746ns | +1.2% | -99.8% |
| 4 | 1448220ns | +0.0% | -99.8% |
| 5 | 1448800ns | +0.2% | -99.8% |
| 6 | 1445965ns | -0.1% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_leaf_fresh_alloc | -0.410 | moderate- |
| abi_residency_leaf_null_entry | -0.049 | ok |
| abi_residency_leaf_reused_buffer | -0.248 | moderate- |

**Consistency summary:**

- **abi_residency_leaf_fresh_alloc**: won 0/6, lost 4/6
- **abi_residency_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 4394194.0ns | 1452177.4ns | 302.6% | HIGH |
| abi_residency_leaf_null_entry | 115857.2ns | 2325.8ns | 4981.5% | HIGH |
| abi_residency_leaf_reused_buffer | 4382009.4ns | 1447972.5ns | 302.6% | HIGH |

## Distribution (algo ns)

```
abi_residency_leaf_fresh_alloc (n=6, range 1444817.1-1459743.4 ns)
  1444817.1 |########################################
  1445563.4 |
  1446309.7 |
  1447056.0 |
  1447802.4 |########################################
  1448548.7 |########################################
  1449295.0 |
  1450041.3 |
  1450787.6 |
  1451533.9 |########################################
  1452280.2 |
  1453026.5 |
  1453772.9 |
  1454519.2 |
  1455265.5 |########################################
  1456011.8 |
  1456758.1 |
  1457504.4 |
  1458250.7 |
  1458997.0 |
  (0 below, 1 above range)

abi_residency_leaf_null_entry (n=6, range 2230.4-2425.4 ns)
   2230.4 |########################################
   2240.2 |
   2249.9 |########################################
   2259.7 |
   2269.4 |
   2279.2 |
   2288.9 |
   2298.7 |########################################
   2308.4 |########################################
   2318.2 |
   2327.9 |
   2337.7 |
   2347.4 |
   2357.2 |
   2366.9 |
   2376.7 |
   2386.4 |
   2396.2 |
   2405.9 |
   2415.7 |########################################
  (0 below, 1 above range)

abi_residency_leaf_reused_buffer (n=6, range 1445964.6-1450228.9 ns)
  1445964.6 |########################################
  1446177.8 |
  1446391.0 |########################################
  1446604.3 |########################################
  1446817.5 |
  1447030.7 |
  1447243.9 |
  1447457.1 |
  1447670.3 |
  1447883.6 |
  1448096.8 |########################################
  1448310.0 |
  1448523.2 |
  1448736.4 |########################################
  1448949.6 |
  1449162.9 |
  1449376.1 |
  1449589.3 |
  1449802.5 |
  1450015.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_leaf_fresh_alloc**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_residency_leaf_null_entry**: bridge=5009.6% of algo (FFI overhead may distort results)
- **abi_residency_leaf_reused_buffer**: bridge=302.7% of algo (FFI overhead may distort results)
