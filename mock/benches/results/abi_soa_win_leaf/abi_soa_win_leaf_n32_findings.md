# abi_soa_win (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_leaf_scalar_payload has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_leaf_null_entry at 2.31 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_leaf_null_entry dominates: 35508% faster than the next best (abi_soa_win_leaf_soa_payload)

abi_soa_win_leaf_null_entry (2.31 us) leads abi_soa_win_leaf_soa_payload (821.88 us) by 35508%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_leaf_null_entry beats baseline by 100% (significant)

abi_soa_win_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_soa_win_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_leaf_scalar_payload is an outlier: 631.3x slower than the field

abi_soa_win_leaf_scalar_payload (1.46 ms) is 631.3x the fastest (2.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 631.3x the fastest

Fastest abi_soa_win_leaf_null_entry (2.31 us) to slowest abi_soa_win_leaf_scalar_payload (1.46 ms): 631.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_leaf_null_entry** at 2308.1 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 631.27x (fastest 2308.1 ns, slowest 1457074.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4598ns | 4590ns | 4400ns | 4542ns | 4781ns | -99.69% |
| abi_soa_win_leaf_scalar_payload | 1460720ns | 1459899ns | 1452413ns | 1458245ns | 1468585ns | base |
| abi_soa_win_leaf_soa_payload | 824940ns | 824461ns | 821581ns | 824113ns | 827860ns | -43.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 2310ns | 2211ns | 2386ns | -99.84% | 0.014 |
| abi_soa_win_leaf_scalar_payload | 1457940ns | 1449858ns | 1465633ns | base | 0.000 |
| abi_soa_win_leaf_soa_payload | 822317ns | 819137ns | 825028ns | -43.60% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 27503.5 | 2406.7 | 2310.1 | n/a |
| abi_soa_win_leaf_scalar_payload | 46738.7 | 1458815.8 | 1457939.8 | n/a |
| abi_soa_win_leaf_soa_payload | 41650.8 | 822256.3 | 822316.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_soa_win_leaf_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.014 | 95.8% |
| abi_soa_win_leaf_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_leaf_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4598ns | 4598ns | -99.69% |
| abi_soa_win_leaf_scalar_payload | 1460720ns | 1460720ns | base |
| abi_soa_win_leaf_soa_payload | 824940ns | 824940ns | -43.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_leaf_scalar_payload | 1457074ns | base | --- | [1451112, 1465633] | --- | --- | --- | --- |
| abi_soa_win_leaf_null_entry | 2308ns | -1454799.1ns (-99.8%) | [-1463311, -1448779]ns | [2236, 2386] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_leaf_soa_payload | 821878ns | -634747.5ns (-43.6%) | [-643951, -628171]ns | [820044, 825028] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_leaf_scalar_payload | abi_soa_win_leaf_null_entry | abi_soa_win_leaf_soa_payload |
|---|---|---|---|
| 1 | 1466466ns | -99.8% | -44.0% |
| 2 | 1449858ns | -99.8% | -43.5% |
| 3 | 1455660ns | -99.8% | -43.4% |
| 4 | 1464800ns | -99.8% | -43.9% |
| 5 | 1452366ns | -99.8% | -43.1% |
| 6 | 1458488ns | -99.8% | -43.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_leaf_null_entry | -0.194 | ok |
| abi_soa_win_leaf_scalar_payload | -0.485 | moderate- |
| abi_soa_win_leaf_soa_payload | -0.078 | ok |

**Consistency summary:**

- **abi_soa_win_leaf_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 116442.1ns | 2310.1ns | 5040.5% | HIGH |
| abi_soa_win_leaf_scalar_payload | 4424156.5ns | 1457939.8ns | 303.5% | HIGH |
| abi_soa_win_leaf_soa_payload | 2510667.2ns | 822316.9ns | 305.3% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_leaf_null_entry (n=6, range 2211.2-2386.2 ns)
   2211.2 |########################################
   2220.0 |
   2228.7 |
   2237.5 |
   2246.2 |
   2255.0 |########################################
   2263.7 |
   2272.5 |
   2281.2 |########################################
   2290.0 |
   2298.7 |
   2307.5 |
   2316.2 |
   2325.0 |########################################
   2333.7 |########################################
   2342.5 |
   2351.2 |
   2360.0 |
   2368.7 |
   2377.5 |
  (0 below, 1 above range)

abi_soa_win_leaf_scalar_payload (n=6, range 1449857.9-1465633.3 ns)
  1449857.9 |########################################
  1450646.7 |
  1451435.4 |
  1452224.2 |########################################
  1453013.0 |
  1453801.8 |
  1454590.5 |
  1455379.3 |########################################
  1456168.1 |
  1456956.8 |
  1457745.6 |########################################
  1458534.4 |
  1459323.1 |
  1460111.9 |
  1460900.7 |
  1461689.4 |
  1462478.2 |
  1463267.0 |
  1464055.8 |########################################
  1464844.5 |
  (0 below, 1 above range)

abi_soa_win_leaf_soa_payload (n=6, range 819137.1-825028.1 ns)
  819137.1 |########################################
  819431.7 |
  819726.2 |
  820020.8 |
  820315.3 |
  820609.8 |
  820904.4 |########################################
  821199.0 |########################################
  821493.5 |
  821788.1 |
  822082.6 |
  822377.2 |########################################
  822671.7 |
  822966.2 |
  823260.8 |########################################
  823555.4 |
  823849.9 |
  824144.5 |
  824439.0 |
  824733.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_leaf_null_entry**: bridge=5063.8% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_scalar_payload**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_soa_payload**: bridge=305.2% of algo (FFI overhead may distort results)
