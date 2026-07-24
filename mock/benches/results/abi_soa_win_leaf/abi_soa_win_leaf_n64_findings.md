# abi_soa_win (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_leaf_scalar_payload has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_leaf_null_entry at 2.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_leaf_null_entry dominates: 32737% faster than the next best (abi_soa_win_leaf_soa_payload)

abi_soa_win_leaf_null_entry (2.50 us) leads abi_soa_win_leaf_soa_payload (821.14 us) by 32737%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_leaf_null_entry beats baseline by 100% (significant)

abi_soa_win_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_soa_win_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_leaf_scalar_payload is an outlier: 582.1x slower than the field

abi_soa_win_leaf_scalar_payload (1.46 ms) is 582.1x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_leaf_scalar_payload shows alternating (throttle bounce) (autocorr -0.68)

abi_soa_win_leaf_scalar_payload's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 582.1x the fastest

Fastest abi_soa_win_leaf_null_entry (2.50 us) to slowest abi_soa_win_leaf_scalar_payload (1.46 ms): 582.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_leaf_null_entry** at 2500.7 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 582.06x (fastest 2500.7 ns, slowest 1455529.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4791ns | 4713ns | 4626ns | 4692ns | 5022ns | -99.67% |
| abi_soa_win_leaf_scalar_payload | 1458160ns | 1458144ns | 1455440ns | 1457425ns | 1460622ns | base |
| abi_soa_win_leaf_soa_payload | 824007ns | 823806ns | 821874ns | 823546ns | 825764ns | -43.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 2532ns | 2463ns | 2629ns | -99.83% | 0.025 |
| abi_soa_win_leaf_scalar_payload | 1455439ns | 1452711ns | 1457796ns | base | 0.000 |
| abi_soa_win_leaf_soa_payload | 821337ns | 819271ns | 823061ns | -43.57% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 27056.2 | 2696.8 | 2531.9 | n/a |
| abi_soa_win_leaf_scalar_payload | 44412.0 | 1456266.8 | 1455438.7 | n/a |
| abi_soa_win_leaf_soa_payload | 41467.1 | 821697.7 | 821336.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_soa_win_leaf_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.026 | 98.5% |
| abi_soa_win_leaf_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_leaf_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4791ns | 4791ns | -99.67% |
| abi_soa_win_leaf_scalar_payload | 1458160ns | 1458160ns | base |
| abi_soa_win_leaf_soa_payload | 824007ns | 824007ns | -43.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_leaf_scalar_payload | 1455529ns | base | --- | [1452991, 1457796] | --- | --- | --- | --- |
| abi_soa_win_leaf_null_entry | 2501ns | -1453053.9ns (-99.8%) | [-1455304, -1450362]ns | [2466, 2629] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_leaf_soa_payload | 821140ns | -633666.8ns (-43.5%) | [-637506, -631133]ns | [819809, 823061] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_leaf_scalar_payload | abi_soa_win_leaf_null_entry | abi_soa_win_leaf_soa_payload |
|---|---|---|---|
| 1 | 1454483ns | -99.8% | -43.6% |
| 2 | 1456576ns | -99.8% | -43.5% |
| 3 | 1453271ns | -99.8% | -43.5% |
| 4 | 1456672ns | -99.8% | -43.8% |
| 5 | 1452711ns | -99.8% | -43.4% |
| 6 | 1458920ns | -99.8% | -43.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_leaf_null_entry | -0.588 | HIGH- (thermal bounce) |
| abi_soa_win_leaf_scalar_payload | -0.682 | HIGH- (thermal bounce) |
| abi_soa_win_leaf_soa_payload | -0.430 | moderate- |

**Consistency summary:**

- **abi_soa_win_leaf_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 114465.5ns | 2531.9ns | 4520.9% | HIGH |
| abi_soa_win_leaf_scalar_payload | 4414489.3ns | 1455438.7ns | 303.3% | HIGH |
| abi_soa_win_leaf_soa_payload | 2508988.5ns | 821336.7ns | 305.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_leaf_null_entry (n=6, range 2463.3-2628.7 ns)
   2463.3 |########################################
   2471.6 |
   2479.8 |####################
   2488.1 |
   2496.4 |
   2504.7 |
   2512.9 |####################
   2521.2 |
   2529.5 |
   2537.7 |
   2546.0 |
   2554.3 |
   2562.5 |
   2570.8 |
   2579.1 |####################
   2587.3 |
   2595.6 |
   2603.9 |
   2612.2 |
   2620.4 |
  (0 below, 1 above range)

abi_soa_win_leaf_scalar_payload (n=6, range 1452711.2-1457795.9 ns)
  1452711.2 |####################
  1452965.4 |
  1453219.7 |####################
  1453473.9 |
  1453728.1 |
  1453982.4 |
  1454236.6 |####################
  1454490.8 |
  1454745.1 |
  1454999.3 |
  1455253.5 |
  1455507.8 |
  1455762.0 |
  1456016.2 |
  1456270.5 |
  1456524.7 |########################################
  1456778.9 |
  1457033.2 |
  1457287.4 |
  1457541.6 |
  (0 below, 1 above range)

abi_soa_win_leaf_soa_payload (n=6, range 819271.2-823061.2 ns)
  819271.2 |########################################
  819460.7 |
  819650.2 |
  819839.7 |
  820029.2 |
  820218.7 |########################################
  820408.2 |
  820597.7 |
  820787.2 |########################################
  820976.7 |
  821166.2 |########################################
  821355.7 |
  821545.2 |
  821734.7 |
  821924.2 |
  822113.7 |
  822303.2 |
  822492.7 |
  822682.2 |########################################
  822871.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_leaf_null_entry**: bridge=4584.8% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_scalar_payload**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_soa_payload**: bridge=305.5% of algo (FFI overhead may distort results)
