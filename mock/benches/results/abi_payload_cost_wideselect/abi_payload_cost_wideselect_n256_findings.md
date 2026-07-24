# abi_payload_cost (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_wideselect_scalar_payload has the worst median (2.05 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_wideselect_null_entry at 2.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_wideselect_null_entry dominates: 35965% faster than the next best (abi_payload_cost_wideselect_soa_payload)

abi_payload_cost_wideselect_null_entry (2.54 us) leads abi_payload_cost_wideselect_soa_payload (915.30 us) by 35965%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_wideselect_null_entry beats baseline by 100% (significant)

abi_payload_cost_wideselect_null_entry is -2.04 ms (100%) faster than baseline abi_payload_cost_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_wideselect_scalar_payload is an outlier: 806.2x slower than the field

abi_payload_cost_wideselect_scalar_payload (2.05 ms) is 806.2x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_wideselect_scalar_payload shows alternating (throttle bounce) (autocorr -0.79)

abi_payload_cost_wideselect_scalar_payload's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 806.2x the fastest

Fastest abi_payload_cost_wideselect_null_entry (2.54 us) to slowest abi_payload_cost_wideselect_scalar_payload (2.05 ms): 806.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_wideselect_null_entry** at 2537.9 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 806.18x (fastest 2537.9 ns, slowest 2045992.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4799ns | 4851ns | 4595ns | 4822ns | 4866ns | -99.77% |
| abi_payload_cost_wideselect_scalar_payload | 2049596ns | 2048516ns | 2042533ns | 2046827ns | 2057280ns | base |
| abi_payload_cost_wideselect_soa_payload | 928817ns | 917793ns | 909624ns | 917173ns | 955879ns | -54.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 2518ns | 2379ns | 2583ns | -99.88% | 0.102 |
| abi_payload_cost_wideselect_scalar_payload | 2047031ns | 2039814ns | 2054710ns | base | 0.000 |
| abi_payload_cost_wideselect_soa_payload | 926286ns | 907262ns | 953173ns | -54.75% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 28282.3 | 2694.2 | 2517.9 | n/a |
| abi_payload_cost_wideselect_scalar_payload | 40328.7 | 2046014.5 | 2047030.6 | n/a |
| abi_payload_cost_wideselect_soa_payload | 36875.2 | 1000670.8 | 926285.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.108 Gops/s** (abi_payload_cost_wideselect_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | 0.101 | 93.7% |
| abi_payload_cost_wideselect_scalar_payload | 0.000 | 0.1% |
| abi_payload_cost_wideselect_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4799ns | 4799ns | -99.77% |
| abi_payload_cost_wideselect_scalar_payload | 2049596ns | 2049596ns | base |
| abi_payload_cost_wideselect_soa_payload | 928817ns | 928817ns | -54.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_scalar_payload | 2045992ns | base | --- | [2040390, 2054710] | --- | --- | --- | --- |
| abi_payload_cost_wideselect_null_entry | 2538ns | -2043426.2ns (-99.9%) | [-2052231, -2037881]ns | [2432, 2583] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_wideselect_soa_payload | 915302ns | -1130690.2ns (-55.3%) | [-1135936, -1095608]ns | [910383, 953173] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_wideselect_scalar_payload | abi_payload_cost_wideselect_null_entry | abi_payload_cost_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2051412ns | -99.9% | -51.9% |
| 2 | 2039814ns | -99.9% | -55.5% |
| 3 | 2058009ns | -99.9% | -55.4% |
| 4 | 2040965ns | -99.9% | -55.2% |
| 5 | 2048733ns | -99.9% | -55.3% |
| 6 | 2043251ns | -99.9% | -55.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | 0.145 | ok |
| abi_payload_cost_wideselect_scalar_payload | -0.790 | HIGH- (thermal bounce) |
| abi_payload_cost_wideselect_soa_payload | -0.148 | ok |

**Consistency summary:**

- **abi_payload_cost_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 114449.3ns | 2517.9ns | 4545.4% | HIGH |
| abi_payload_cost_wideselect_scalar_payload | 6184083.7ns | 2047030.6ns | 302.1% | HIGH |
| abi_payload_cost_wideselect_soa_payload | 3041162.5ns | 926285.9ns | 328.3% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_wideselect_null_entry (n=6, range 2379.2-2583.4 ns)
   2379.2 |########################################
   2389.4 |
   2399.6 |
   2409.8 |
   2420.0 |
   2430.2 |
   2440.4 |
   2450.7 |
   2460.9 |
   2471.1 |
   2481.3 |########################################
   2491.5 |
   2501.7 |
   2511.9 |
   2522.1 |########################################
   2532.3 |
   2542.5 |########################################
   2552.7 |
   2562.9 |
   2573.1 |########################################
  (0 below, 1 above range)

abi_payload_cost_wideselect_scalar_payload (n=6, range 2039814.2-2054710.2 ns)
  2039814.2 |########################################
  2040559.0 |########################################
  2041303.8 |
  2042048.6 |
  2042793.4 |########################################
  2043538.2 |
  2044283.0 |
  2045027.8 |
  2045772.6 |
  2046517.4 |
  2047262.2 |
  2048007.0 |########################################
  2048751.8 |
  2049496.6 |
  2050241.4 |
  2050986.2 |########################################
  2051731.0 |
  2052475.8 |
  2053220.6 |
  2053965.4 |
  (0 below, 1 above range)

abi_payload_cost_wideselect_soa_payload (n=6, range 907261.7-953172.9 ns)
  907261.7 |####################
  909557.3 |
  911852.8 |####################
  914148.4 |########################################
  916443.9 |####################
  918739.5 |
  921035.1 |
  923330.6 |
  925626.2 |
  927921.7 |
  930217.3 |
  932512.9 |
  934808.4 |
  937104.0 |
  939399.5 |
  941695.1 |
  943990.7 |
  946286.2 |
  948581.8 |
  950877.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_wideselect_null_entry**: bridge=4501.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_scalar_payload**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_soa_payload**: bridge=304.2% of algo (FFI overhead may distort results)
