# abi_soa_win (madd)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_madd_scalar_payload has the worst median (2.72 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_madd_null_entry at 2.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_madd_null_entry dominates: 47789% faster than the next best (abi_soa_win_madd_soa_payload)

abi_soa_win_madd_null_entry (2.27 us) leads abi_soa_win_madd_soa_payload (1.09 ms) by 47789%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_madd_null_entry beats baseline by 100% (significant)

abi_soa_win_madd_null_entry is -2.72 ms (100%) faster than baseline abi_soa_win_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_madd_scalar_payload is an outlier: 1195.5x slower than the field

abi_soa_win_madd_scalar_payload (2.72 ms) is 1195.5x the fastest (2.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 1195.5x the fastest

Fastest abi_soa_win_madd_null_entry (2.27 us) to slowest abi_soa_win_madd_scalar_payload (2.72 ms): 1195.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_madd_null_entry** at 2273.1 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1195.51x (fastest 2273.1 ns, slowest 2717504.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 4584ns | 4567ns | 4510ns | 4549ns | 4673ns | -99.83% |
| abi_soa_win_madd_scalar_payload | 2721925ns | 2720792ns | 2711705ns | 2720200ns | 2729622ns | base |
| abi_soa_win_madd_soa_payload | 1090334ns | 1091217ns | 1085321ns | 1089643ns | 1093876ns | -59.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 2283ns | 2247ns | 2323ns | -99.92% | 0.014 |
| abi_soa_win_madd_scalar_payload | 2718705ns | 2709000ns | 2726140ns | base | 0.000 |
| abi_soa_win_madd_soa_payload | 1087644ns | 1082768ns | 1090978ns | -59.99% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 27234.9 | 2352.4 | 2282.9 | n/a |
| abi_soa_win_madd_scalar_payload | 58927.8 | 2715126.3 | 2718704.6 | n/a |
| abi_soa_win_madd_soa_payload | 42092.5 | 1087710.8 | 1087644.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_soa_win_madd_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.014 | 98.8% |
| abi_soa_win_madd_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_madd_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_madd_null_entry | 4584ns | 4584ns | -99.83% |
| abi_soa_win_madd_scalar_payload | 2721925ns | 2721925ns | base |
| abi_soa_win_madd_soa_payload | 1090334ns | 1090334ns | -59.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_madd_scalar_payload | 2717504ns | base | --- | [2712470, 2726140] | --- | --- | --- | --- |
| abi_soa_win_madd_null_entry | 2273ns | -2715251.4ns (-99.9%) | [-2723817, -2710197]ns | [2253, 2323] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_madd_soa_payload | 1088555ns | -1629066.1ns (-59.9%) | [-1638270, -1625846]ns | [1083400, 1090978] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_madd_scalar_payload | abi_soa_win_madd_null_entry | abi_soa_win_madd_soa_payload |
|---|---|---|---|
| 1 | 2731037ns | -99.9% | -60.2% |
| 2 | 2721242ns | -99.9% | -59.9% |
| 3 | 2715941ns | -99.9% | -59.9% |
| 4 | 2718845ns | -99.9% | -59.9% |
| 5 | 2709000ns | -99.9% | -60.0% |
| 6 | 2716163ns | -99.9% | -60.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.129 | ok |
| abi_soa_win_madd_scalar_payload | 0.177 | ok |
| abi_soa_win_madd_soa_payload | 0.234 | moderate+ |

**Consistency summary:**

- **abi_soa_win_madd_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 117204.7ns | 2282.9ns | 5134.0% | HIGH |
| abi_soa_win_madd_scalar_payload | 8200679.9ns | 2718704.6ns | 301.6% | HIGH |
| abi_soa_win_madd_soa_payload | 3304998.2ns | 1087644.2ns | 303.9% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_madd_null_entry (n=6, range 2246.7-2322.9 ns)
   2246.7 |########################################
   2250.5 |
   2254.3 |
   2258.1 |########################################
   2261.9 |
   2265.8 |########################################
   2269.6 |
   2273.4 |
   2277.2 |########################################
   2281.0 |
   2284.8 |########################################
   2288.6 |
   2292.4 |
   2296.2 |
   2300.0 |
   2303.8 |
   2307.7 |
   2311.5 |
   2315.3 |
   2319.1 |
  (0 below, 1 above range)

abi_soa_win_madd_scalar_payload (n=6, range 2708999.6-2726139.6 ns)
  2708999.6 |####################
  2709856.6 |
  2710713.6 |
  2711570.6 |
  2712427.6 |
  2713284.6 |
  2714141.6 |
  2714998.6 |
  2715855.6 |########################################
  2716712.6 |
  2717569.6 |
  2718426.6 |####################
  2719283.6 |
  2720140.6 |
  2720997.6 |####################
  2721854.6 |
  2722711.6 |
  2723568.6 |
  2724425.6 |
  2725282.6 |
  (0 below, 1 above range)

abi_soa_win_madd_soa_payload (n=6, range 1082767.5-1090977.5 ns)
  1082767.5 |########################################
  1083178.0 |
  1083588.5 |
  1083999.0 |########################################
  1084409.5 |
  1084820.0 |
  1085230.5 |
  1085641.0 |
  1086051.5 |
  1086462.0 |
  1086872.5 |
  1087283.0 |
  1087693.5 |########################################
  1088104.0 |
  1088514.5 |
  1088925.0 |########################################
  1089335.5 |
  1089746.0 |
  1090156.5 |########################################
  1090567.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_madd_null_entry**: bridge=5137.2% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_scalar_payload**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_soa_payload**: bridge=304.0% of algo (FFI overhead may distort results)
