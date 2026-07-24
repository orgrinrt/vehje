# abi_soa_win (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_wideselect_scalar_payload has the worst median (2.05 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_wideselect_null_entry at 3.14 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_wideselect_null_entry dominates: 29185% faster than the next best (abi_soa_win_wideselect_soa_payload)

abi_soa_win_wideselect_null_entry (3.14 us) leads abi_soa_win_wideselect_soa_payload (919.35 us) by 29185%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_wideselect_null_entry beats baseline by 100% (significant)

abi_soa_win_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_soa_win_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_wideselect_scalar_payload is an outlier: 653.6x slower than the field

abi_soa_win_wideselect_scalar_payload (2.05 ms) is 653.6x the fastest (3.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 653.6x the fastest

Fastest abi_soa_win_wideselect_null_entry (3.14 us) to slowest abi_soa_win_wideselect_scalar_payload (2.05 ms): 653.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_wideselect_null_entry** at 3139.4 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 653.56x (fastest 3139.4 ns, slowest 2051745.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 5441ns | 5374ns | 5245ns | 5358ns | 5661ns | -99.74% |
| abi_soa_win_wideselect_scalar_payload | 2056325ns | 2054391ns | 2051688ns | 2053517ns | 2062855ns | base |
| abi_soa_win_wideselect_soa_payload | 919968ns | 921857ns | 915048ns | 920039ns | 922322ns | -55.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 3156ns | 3054ns | 3249ns | -99.85% | 0.081 |
| abi_soa_win_wideselect_scalar_payload | 2053771ns | 2049234ns | 2060299ns | base | 0.000 |
| abi_soa_win_wideselect_soa_payload | 917532ns | 912661ns | 919871ns | -55.32% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 28072.0 | 3216.6 | 3155.7 | n/a |
| abi_soa_win_wideselect_scalar_payload | 39537.6 | 2054333.7 | 2053770.6 | n/a |
| abi_soa_win_wideselect_soa_payload | 34599.2 | 917729.2 | 917532.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.084 Gops/s** (abi_soa_win_wideselect_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.082 | 97.3% |
| abi_soa_win_wideselect_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_wideselect_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 5441ns | 5441ns | -99.74% |
| abi_soa_win_wideselect_scalar_payload | 2056325ns | 2056325ns | base |
| abi_soa_win_wideselect_soa_payload | 919968ns | 919968ns | -55.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_scalar_payload | 2051745ns | base | --- | [2049268, 2060299] | --- | --- | --- | --- |
| abi_soa_win_wideselect_null_entry | 3139ns | -2048666.2ns (-99.8%) | [-2057070, -2046109]ns | [3079, 3249] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_wideselect_soa_payload | 919353ns | -1133726.2ns (-55.3%) | [-1145128, -1129861]ns | [913372, 919871] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_wideselect_scalar_payload | abi_soa_win_wideselect_null_entry | abi_soa_win_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2049302ns | -99.8% | -55.1% |
| 2 | 2049234ns | -99.8% | -55.1% |
| 3 | 2057057ns | -99.8% | -55.3% |
| 4 | 2063541ns | -99.8% | -55.7% |
| 5 | 2053460ns | -99.8% | -55.6% |
| 6 | 2050030ns | -99.9% | -55.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_wideselect_null_entry | -0.015 | ok |
| abi_soa_win_wideselect_scalar_payload | 0.221 | moderate+ |
| abi_soa_win_wideselect_soa_payload | 0.135 | ok |

**Consistency summary:**

- **abi_soa_win_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 121568.1ns | 3155.7ns | 3852.4% | HIGH |
| abi_soa_win_wideselect_scalar_payload | 6206595.9ns | 2053770.6ns | 302.2% | HIGH |
| abi_soa_win_wideselect_soa_payload | 2789866.9ns | 917532.1ns | 304.1% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_wideselect_null_entry (n=6, range 3053.8-3248.9 ns)
   3053.8 |########################################
   3063.6 |
   3073.3 |
   3083.1 |
   3092.8 |
   3102.6 |########################################
   3112.3 |
   3122.1 |########################################
   3131.9 |
   3141.6 |
   3151.4 |########################################
   3161.1 |
   3170.9 |
   3180.6 |
   3190.4 |########################################
   3200.2 |
   3209.9 |
   3219.7 |
   3229.4 |
   3239.2 |
  (0 below, 1 above range)

abi_soa_win_wideselect_scalar_payload (n=6, range 2049233.8-2060299.0 ns)
  2049233.8 |########################################
  2049787.1 |####################
  2050340.3 |
  2050893.6 |
  2051446.8 |
  2052000.1 |
  2052553.3 |
  2053106.6 |####################
  2053659.9 |
  2054213.1 |
  2054766.4 |
  2055319.6 |
  2055872.9 |
  2056426.1 |
  2056979.4 |####################
  2057532.7 |
  2058085.9 |
  2058639.2 |
  2059192.4 |
  2059745.7 |
  (0 below, 1 above range)

abi_soa_win_wideselect_soa_payload (n=6, range 912660.8-919871.4 ns)
  912660.8 |####################
  913021.3 |
  913381.9 |
  913742.4 |####################
  914102.9 |
  914463.5 |
  914824.0 |
  915184.5 |
  915545.1 |
  915905.6 |
  916266.1 |
  916626.7 |
  916987.2 |
  917347.7 |
  917708.3 |
  918068.8 |
  918429.3 |
  918789.9 |
  919150.4 |########################################
  919510.9 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_wideselect_null_entry**: bridge=3875.2% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_scalar_payload**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_soa_payload**: bridge=303.8% of algo (FFI overhead may distort results)
