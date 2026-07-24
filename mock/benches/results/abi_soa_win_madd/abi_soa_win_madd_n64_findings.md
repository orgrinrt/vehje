# abi_soa_win (madd)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_madd_scalar_payload has the worst median (2.71 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_madd_null_entry at 2.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_madd_null_entry dominates: 42050% faster than the next best (abi_soa_win_madd_soa_payload)

abi_soa_win_madd_null_entry (2.57 us) leads abi_soa_win_madd_soa_payload (1.08 ms) by 42050%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_madd_null_entry beats baseline by 100% (significant)

abi_soa_win_madd_null_entry is -2.71 ms (100%) faster than baseline abi_soa_win_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_madd_scalar_payload is an outlier: 1055.1x slower than the field

abi_soa_win_madd_scalar_payload (2.71 ms) is 1055.1x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_madd_null_entry shows alternating (throttle bounce) (autocorr -0.50)

abi_soa_win_madd_null_entry's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 1055.1x the fastest

Fastest abi_soa_win_madd_null_entry (2.57 us) to slowest abi_soa_win_madd_scalar_payload (2.71 ms): 1055.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_madd_null_entry** at 2573.1 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1055.12x (fastest 2573.1 ns, slowest 2714986.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 4881ns | 4881ns | 4797ns | 4859ns | 4956ns | -99.82% |
| abi_soa_win_madd_scalar_payload | 2717356ns | 2718115ns | 2711759ns | 2717024ns | 2720653ns | base |
| abi_soa_win_madd_soa_payload | 1087131ns | 1087118ns | 1085080ns | 1086756ns | 1088720ns | -59.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 2565ns | 2521ns | 2600ns | -99.91% | 0.025 |
| abi_soa_win_madd_scalar_payload | 2714263ns | 2709002ns | 2717348ns | base | 0.000 |
| abi_soa_win_madd_soa_payload | 1084529ns | 1082392ns | 1086122ns | -60.04% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 27748.4 | 2755.9 | 2564.9 | n/a |
| abi_soa_win_madd_scalar_payload | 58734.5 | 2714831.1 | 2714263.0 | n/a |
| abi_soa_win_madd_soa_payload | 39382.4 | 1084070.8 | 1084529.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.025 Gops/s** (abi_soa_win_madd_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.025 | 98.0% |
| abi_soa_win_madd_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_madd_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_madd_null_entry | 4881ns | 4881ns | -99.82% |
| abi_soa_win_madd_scalar_payload | 2717356ns | 2717356ns | base |
| abi_soa_win_madd_soa_payload | 1087131ns | 1087131ns | -59.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_madd_scalar_payload | 2714986ns | base | --- | [2710455, 2717348] | --- | --- | --- | --- |
| abi_soa_win_madd_null_entry | 2573ns | -2712386.0ns (-99.9%) | [-2714799, -2707909]ns | [2522, 2600] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_madd_soa_payload | 1084581ns | -1630384.8ns (-60.1%) | [-1634321, -1624496]ns | [1082884, 1086122] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_madd_scalar_payload | abi_soa_win_madd_null_entry | abi_soa_win_madd_soa_payload |
|---|---|---|---|
| 1 | 2716846ns | -99.9% | -60.1% |
| 2 | 2717564ns | -99.9% | -60.2% |
| 3 | 2711908ns | -99.9% | -59.9% |
| 4 | 2713126ns | -99.9% | -60.0% |
| 5 | 2709002ns | -99.9% | -59.9% |
| 6 | 2717132ns | -99.9% | -60.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_madd_null_entry | -0.504 | HIGH- (thermal bounce) |
| abi_soa_win_madd_scalar_payload | -0.094 | ok |
| abi_soa_win_madd_soa_payload | 0.018 | ok |

**Consistency summary:**

- **abi_soa_win_madd_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 115080.9ns | 2564.9ns | 4486.7% | HIGH |
| abi_soa_win_madd_scalar_payload | 8204212.3ns | 2714263.0ns | 302.3% | HIGH |
| abi_soa_win_madd_soa_payload | 3294903.6ns | 1084529.0ns | 303.8% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_madd_null_entry (n=6, range 2520.8-2599.9 ns)
   2520.8 |########################################
   2524.8 |
   2528.7 |
   2532.7 |
   2536.6 |
   2540.6 |
   2544.5 |
   2548.5 |
   2552.5 |
   2556.4 |
   2560.4 |
   2564.3 |
   2568.3 |####################
   2572.2 |
   2576.2 |####################
   2580.2 |####################
   2584.1 |
   2588.1 |
   2592.0 |
   2596.0 |
  (0 below, 1 above range)

abi_soa_win_madd_scalar_payload (n=6, range 2709002.1-2717348.0 ns)
  2709002.1 |########################################
  2709419.4 |
  2709836.7 |
  2710254.0 |
  2710671.3 |
  2711088.6 |
  2711505.9 |########################################
  2711923.1 |
  2712340.4 |
  2712757.7 |########################################
  2713175.0 |
  2713592.3 |
  2714009.6 |
  2714426.9 |
  2714844.2 |
  2715261.5 |
  2715678.8 |
  2716096.1 |
  2716513.4 |########################################
  2716930.7 |########################################
  (0 below, 1 above range)

abi_soa_win_madd_soa_payload (n=6, range 1082392.1-1086121.9 ns)
  1082392.1 |########################################
  1082578.6 |
  1082765.1 |
  1082951.6 |
  1083138.1 |
  1083324.5 |########################################
  1083511.0 |
  1083697.5 |
  1083884.0 |########################################
  1084070.5 |
  1084257.0 |
  1084443.5 |
  1084630.0 |
  1084816.4 |
  1085002.9 |
  1085189.4 |########################################
  1085375.9 |########################################
  1085562.4 |
  1085748.9 |
  1085935.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_madd_null_entry**: bridge=4476.6% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_scalar_payload**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_soa_payload**: bridge=303.9% of algo (FFI overhead may distort results)
