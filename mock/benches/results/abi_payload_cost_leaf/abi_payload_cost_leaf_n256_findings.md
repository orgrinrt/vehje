# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 32665% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.51 us) leads abi_payload_cost_leaf_soa_payload (821.37 us) by 32665%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 100% (significant)

abi_payload_cost_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 580.5x slower than the field

abi_payload_cost_leaf_scalar_payload (1.46 ms) is 580.5x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_leaf_null_entry shows alternating (throttle bounce) (autocorr -0.66)

abi_payload_cost_leaf_null_entry's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 580.5x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.51 us) to slowest abi_payload_cost_leaf_scalar_payload (1.46 ms): 580.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2506.8 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 580.53x (fastest 2506.8 ns, slowest 1455295.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4769ns | 4748ns | 4600ns | 4723ns | 4922ns | -99.67% |
| abi_payload_cost_leaf_scalar_payload | 1459085ns | 1457882ns | 1454836ns | 1456872ns | 1464529ns | base |
| abi_payload_cost_leaf_soa_payload | 823526ns | 823939ns | 820554ns | 823832ns | 824552ns | -43.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2514ns | 2432ns | 2592ns | -99.83% | 0.102 |
| abi_payload_cost_leaf_scalar_payload | 1456482ns | 1452260ns | 1461836ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 820966ns | 818155ns | 821928ns | -43.63% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 27273.3 | 2729.8 | 2514.4 | n/a |
| abi_payload_cost_leaf_scalar_payload | 42125.3 | 1456874.1 | 1456481.6 | n/a |
| abi_payload_cost_leaf_soa_payload | 38877.8 | 821110.8 | 820966.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.105 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.102 | 97.0% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 0.2% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4769ns | 4769ns | -99.67% |
| abi_payload_cost_leaf_scalar_payload | 1459085ns | 1459085ns | base |
| abi_payload_cost_leaf_soa_payload | 823526ns | 823526ns | -43.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 1455296ns | base | --- | [1452313, 1461836] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2507ns | -1452846.6ns (-99.8%) | [-1459292, -1449763]ns | [2445, 2592] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 821370ns | -633936.5ns (-43.6%) | [-642167, -630444]ns | [819601, 821928] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 1453906ns | -99.8% | -43.5% |
| 2 | 1452366ns | -99.8% | -43.4% |
| 3 | 1463909ns | -99.8% | -44.1% |
| 4 | 1459762ns | -99.8% | -43.7% |
| 5 | 1456686ns | -99.8% | -43.6% |
| 6 | 1452260ns | -99.8% | -43.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | -0.658 | HIGH- (thermal bounce) |
| abi_payload_cost_leaf_scalar_payload | 0.039 | ok |
| abi_payload_cost_leaf_soa_payload | -0.169 | ok |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 112769.0ns | 2514.4ns | 4484.9% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 4413410.5ns | 1456481.6ns | 303.0% | HIGH |
| abi_payload_cost_leaf_soa_payload | 2502795.8ns | 820966.0ns | 304.9% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2432.1-2591.9 ns)
   2432.1 |########################################
   2440.1 |
   2448.1 |
   2456.1 |########################################
   2464.1 |########################################
   2472.0 |
   2480.0 |
   2488.0 |
   2496.0 |
   2504.0 |
   2512.0 |
   2520.0 |
   2528.0 |
   2535.9 |
   2543.9 |########################################
   2551.9 |########################################
   2559.9 |
   2567.9 |
   2575.9 |
   2583.9 |
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 1452260.4-1461835.9 ns)
  1452260.4 |########################################
  1452739.2 |
  1453217.9 |
  1453696.7 |####################
  1454175.5 |
  1454654.3 |
  1455133.0 |
  1455611.8 |
  1456090.6 |
  1456569.4 |####################
  1457048.1 |
  1457526.9 |
  1458005.7 |
  1458484.4 |
  1458963.2 |
  1459442.0 |####################
  1459920.8 |
  1460399.5 |
  1460878.3 |
  1461357.1 |
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 818155.0-821927.5 ns)
  818155.0 |####################
  818343.6 |
  818532.2 |
  818720.9 |
  818909.5 |
  819098.1 |
  819286.8 |
  819475.4 |
  819664.0 |
  819852.6 |
  820041.2 |
  820229.9 |
  820418.5 |
  820607.1 |
  820795.8 |
  820984.4 |####################
  821173.0 |####################
  821361.6 |
  821550.2 |########################################
  821738.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4479.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=304.6% of algo (FFI overhead may distort results)
