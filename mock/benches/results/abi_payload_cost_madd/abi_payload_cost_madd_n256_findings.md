# abi_payload_cost (madd)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_madd_scalar_payload has the worst median (2.69 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_madd_null_entry at 2.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_madd_null_entry dominates: 42435% faster than the next best (abi_payload_cost_madd_soa_payload)

abi_payload_cost_madd_null_entry (2.52 us) leads abi_payload_cost_madd_soa_payload (1.07 ms) by 42435%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_madd_null_entry beats baseline by 100% (significant)

abi_payload_cost_madd_null_entry is -2.69 ms (100%) faster than baseline abi_payload_cost_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_madd_scalar_payload is an outlier: 1067.7x slower than the field

abi_payload_cost_madd_scalar_payload (2.69 ms) is 1067.7x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_madd_soa_payload shows alternating (throttle bounce) (autocorr -0.58)

abi_payload_cost_madd_soa_payload's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 1067.7x the fastest

Fastest abi_payload_cost_madd_null_entry (2.52 us) to slowest abi_payload_cost_madd_scalar_payload (2.69 ms): 1067.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_madd_null_entry** at 2518.8 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1067.70x (fastest 2518.8 ns, slowest 2689280.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4824ns | 4772ns | 4562ns | 4728ns | 5098ns | -99.82% |
| abi_payload_cost_madd_scalar_payload | 2730720ns | 2691934ns | 2669555ns | 2685926ns | 2828495ns | base |
| abi_payload_cost_madd_soa_payload | 1077929ns | 1073904ns | 1073128ns | 1073805ns | 1086516ns | -60.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 2540ns | 2412ns | 2664ns | -99.91% | 0.101 |
| abi_payload_cost_madd_scalar_payload | 2727664ns | 2666888ns | 2824651ns | base | 0.000 |
| abi_payload_cost_madd_soa_payload | 1075361ns | 1070602ns | 1083787ns | -60.58% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 27892.3 | 2729.5 | 2539.9 | n/a |
| abi_payload_cost_madd_scalar_payload | 53632.8 | 2729654.2 | 2727664.0 | n/a |
| abi_payload_cost_madd_soa_payload | 36660.6 | 1075952.5 | 1075361.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.106 Gops/s** (abi_payload_cost_madd_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_madd_null_entry | 0.102 | 95.7% |
| abi_payload_cost_madd_scalar_payload | 0.000 | 0.1% |
| abi_payload_cost_madd_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4824ns | 4824ns | -99.82% |
| abi_payload_cost_madd_scalar_payload | 2730720ns | 2730720ns | base |
| abi_payload_cost_madd_soa_payload | 1077929ns | 1077929ns | -60.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_madd_scalar_payload | 2689281ns | base | --- | [2669060, 2824651] | --- | --- | --- | --- |
| abi_payload_cost_madd_null_entry | 2519ns | -2686816.5ns (-99.9%) | [-2822160, -2666396]ns | [2437, 2664] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_madd_soa_payload | 1071343ns | -1617955.7ns (-60.2%) | [-1746500, -1592453]ns | [1070954, 1083787] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_madd_scalar_payload | abi_payload_cost_madd_null_entry | abi_payload_cost_madd_soa_payload |
|---|---|---|---|
| 1 | 2697234ns | -99.9% | -60.3% |
| 2 | 2666888ns | -99.9% | -59.4% |
| 3 | 2671232ns | -99.9% | -59.9% |
| 4 | 2681327ns | -99.9% | -60.0% |
| 5 | 2937675ns | -99.9% | -63.0% |
| 6 | 2711628ns | -99.9% | -60.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_madd_null_entry | 0.015 | ok |
| abi_payload_cost_madd_scalar_payload | -0.096 | ok |
| abi_payload_cost_madd_soa_payload | -0.576 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_payload_cost_madd_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 113808.4ns | 2539.9ns | 4480.8% | HIGH |
| abi_payload_cost_madd_scalar_payload | 8231163.5ns | 2727664.0ns | 301.8% | HIGH |
| abi_payload_cost_madd_soa_payload | 3264975.9ns | 1075361.2ns | 303.6% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_madd_null_entry (n=6, range 2411.7-2664.4 ns)
   2411.7 |####################
   2424.3 |
   2437.0 |
   2449.6 |####################
   2462.2 |
   2474.9 |
   2487.5 |
   2500.1 |
   2512.8 |########################################
   2525.4 |
   2538.0 |
   2550.7 |
   2563.3 |
   2575.9 |
   2588.6 |####################
   2601.2 |
   2613.8 |
   2626.5 |
   2639.1 |
   2651.7 |
  (0 below, 1 above range)

abi_payload_cost_madd_scalar_payload (n=6, range 2666887.9-2824651.2 ns)
  2666887.9 |########################################
  2674776.1 |####################
  2682664.2 |
  2690552.4 |####################
  2698440.6 |
  2706328.7 |####################
  2714216.9 |
  2722105.1 |
  2729993.2 |
  2737881.4 |
  2745769.6 |
  2753657.7 |
  2761545.9 |
  2769434.1 |
  2777322.2 |
  2785210.4 |
  2793098.6 |
  2800986.7 |
  2808874.9 |
  2816763.1 |
  (0 below, 1 above range)

abi_payload_cost_madd_soa_payload (n=6, range 1070602.1-1083786.9 ns)
  1070602.1 |#############
  1071261.3 |########################################
  1071920.6 |
  1072579.8 |
  1073239.1 |
  1073898.3 |
  1074557.5 |
  1075216.8 |
  1075876.0 |
  1076535.2 |
  1077194.5 |
  1077853.7 |
  1078513.0 |
  1079172.2 |
  1079831.4 |
  1080490.7 |
  1081149.9 |
  1081809.1 |#############
  1082468.4 |
  1083127.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_madd_null_entry**: bridge=4517.7% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_scalar_payload**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_soa_payload**: bridge=303.8% of algo (FFI overhead may distort results)
