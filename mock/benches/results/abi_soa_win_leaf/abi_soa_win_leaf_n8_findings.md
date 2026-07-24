# abi_soa_win (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_leaf_scalar_payload has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_leaf_null_entry at 3.05 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_leaf_null_entry dominates: 27103% faster than the next best (abi_soa_win_leaf_soa_payload)

abi_soa_win_leaf_null_entry (3.05 us) leads abi_soa_win_leaf_soa_payload (829.47 us) by 27103%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_leaf_null_entry beats baseline by 100% (significant)

abi_soa_win_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_soa_win_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_leaf_scalar_payload is an outlier: 478.6x slower than the field

abi_soa_win_leaf_scalar_payload (1.46 ms) is 478.6x the fastest (3.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_leaf_null_entry shows alternating (throttle bounce) (autocorr -0.57)

abi_soa_win_leaf_null_entry's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 478.6x the fastest

Fastest abi_soa_win_leaf_null_entry (3.05 us) to slowest abi_soa_win_leaf_scalar_payload (1.46 ms): 478.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_leaf_null_entry** at 3049.2 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 478.57x (fastest 3049.2 ns, slowest 1459227.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 5352ns | 5399ns | 5079ns | 5298ns | 5571ns | -99.63% |
| abi_soa_win_leaf_scalar_payload | 1462848ns | 1462312ns | 1452900ns | 1461212ns | 1470276ns | base |
| abi_soa_win_leaf_soa_payload | 831093ns | 832270ns | 827001ns | 831132ns | 833080ns | -43.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 3049ns | 2921ns | 3176ns | -99.79% | 0.003 |
| abi_soa_win_leaf_scalar_payload | 1459923ns | 1450407ns | 1467185ns | base | 0.000 |
| abi_soa_win_leaf_soa_payload | 828356ns | 824583ns | 830174ns | -43.26% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 27903.0 | 3136.6 | 3049.2 | n/a |
| abi_soa_win_leaf_scalar_payload | 48483.4 | 1460895.9 | 1459923.1 | n/a |
| abi_soa_win_leaf_soa_payload | 43222.0 | 826291.0 | 828356.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_soa_win_leaf_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.003 | 95.8% |
| abi_soa_win_leaf_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_leaf_soa_payload | 0.000 | 0.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_leaf_null_entry | 5352ns | 5352ns | -99.63% |
| abi_soa_win_leaf_scalar_payload | 1462848ns | 1462848ns | base |
| abi_soa_win_leaf_soa_payload | 831093ns | 831093ns | -43.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_leaf_scalar_payload | 1459227ns | base | --- | [1453357, 1467185] | --- | --- | --- | --- |
| abi_soa_win_leaf_null_entry | 3049ns | -1456083.6ns (-99.8%) | [-1464192, -1450346]ns | [2923, 3176] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_leaf_soa_payload | 829472ns | -629754.9ns (-43.2%) | [-638881, -626064]ns | [825423, 830174] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_leaf_scalar_payload | abi_soa_win_leaf_null_entry | abi_soa_win_leaf_soa_payload |
|---|---|---|---|
| 1 | 1456307ns | -99.8% | -43.0% |
| 2 | 1456528ns | -99.8% | -43.0% |
| 3 | 1468889ns | -99.8% | -43.7% |
| 4 | 1450407ns | -99.8% | -43.1% |
| 5 | 1461926ns | -99.8% | -43.3% |
| 6 | 1465481ns | -99.8% | -43.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_leaf_null_entry | -0.575 | HIGH- (thermal bounce) |
| abi_soa_win_leaf_scalar_payload | -0.483 | moderate- |
| abi_soa_win_leaf_soa_payload | 0.205 | moderate+ |

**Consistency summary:**

- **abi_soa_win_leaf_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 119295.4ns | 3049.2ns | 3912.4% | HIGH |
| abi_soa_win_leaf_scalar_payload | 4433133.7ns | 1459923.1ns | 303.7% | HIGH |
| abi_soa_win_leaf_soa_payload | 2521998.4ns | 828356.5ns | 304.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_leaf_null_entry (n=6, range 2921.2-3175.6 ns)
   2921.2 |########################################
   2933.9 |
   2946.6 |
   2959.4 |
   2972.1 |
   2984.8 |
   2997.5 |
   3010.2 |
   3023.0 |
   3035.7 |####################
   3048.4 |
   3061.1 |####################
   3073.8 |
   3086.6 |
   3099.3 |
   3112.0 |
   3124.7 |
   3137.4 |####################
   3150.2 |
   3162.9 |
  (0 below, 1 above range)

abi_soa_win_leaf_scalar_payload (n=6, range 1450407.1-1467185.0 ns)
  1450407.1 |####################
  1451246.0 |
  1452084.9 |
  1452923.8 |
  1453762.7 |
  1454601.6 |
  1455440.5 |
  1456279.4 |########################################
  1457118.3 |
  1457957.2 |
  1458796.1 |
  1459634.9 |
  1460473.8 |
  1461312.7 |####################
  1462151.6 |
  1462990.5 |
  1463829.4 |
  1464668.3 |####################
  1465507.2 |
  1466346.1 |
  (0 below, 1 above range)

abi_soa_win_leaf_soa_payload (n=6, range 824582.9-830174.4 ns)
  824582.9 |########################################
  824862.5 |
  825142.0 |
  825421.6 |
  825701.2 |
  825980.8 |
  826260.3 |########################################
  826539.9 |
  826819.5 |
  827099.1 |
  827378.6 |
  827658.2 |
  827937.8 |
  828217.3 |
  828496.9 |
  828776.5 |
  829056.1 |########################################
  829335.6 |
  829615.2 |########################################
  829894.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_leaf_null_entry**: bridge=3911.6% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_scalar_payload**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_soa_payload**: bridge=304.3% of algo (FFI overhead may distort results)
