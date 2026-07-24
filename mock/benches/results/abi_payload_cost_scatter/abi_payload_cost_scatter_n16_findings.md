# abi_payload_cost (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_scatter_scalar_payload has the worst median (110.27 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_scatter_null_entry at 2.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_scatter_null_entry dominates: 1195% faster than the next best (abi_payload_cost_scatter_soa_payload)

abi_payload_cost_scatter_null_entry (2.54 us) leads abi_payload_cost_scatter_soa_payload (32.94 us) by 1195%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_scatter_null_entry beats baseline by 98% (significant)

abi_payload_cost_scatter_null_entry is -107.74 us (98%) faster than baseline abi_payload_cost_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_scatter_scalar_payload is an outlier: 43.3x slower than the field

abi_payload_cost_scatter_scalar_payload (110.27 us) is 43.3x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 43.3x the fastest

Fastest abi_payload_cost_scatter_null_entry (2.54 us) to slowest abi_payload_cost_scatter_scalar_payload (110.27 us): 43.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_scatter_null_entry** at 2543.9 ns median (-97.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 43.35x (fastest 2543.9 ns, slowest 110273.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4828ns | 4834ns | 4758ns | 4827ns | 4866ns | -95.71% |
| abi_payload_cost_scatter_scalar_payload | 112491ns | 112495ns | 111677ns | 112398ns | 113038ns | base |
| abi_payload_cost_scatter_soa_payload | 35268ns | 35172ns | 34938ns | 35099ns | 35687ns | -68.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 2548ns | 2512ns | 2586ns | -97.69% | 0.006 |
| abi_payload_cost_scatter_scalar_payload | 110248ns | 109528ns | 110720ns | base | 0.000 |
| abi_payload_cost_scatter_soa_payload | 32996ns | 32600ns | 33411ns | -70.07% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 20637.4 | 2768.3 | 2548.5 | n/a |
| abi_payload_cost_scatter_scalar_payload | 20835.4 | 110313.0 | 110248.1 | n/a |
| abi_payload_cost_scatter_soa_payload | 20644.2 | 33118.4 | 32995.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_payload_cost_scatter_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_scatter_null_entry | 0.006 | 98.8% |
| abi_payload_cost_scatter_scalar_payload | 0.000 | 2.3% |
| abi_payload_cost_scatter_soa_payload | 0.000 | 7.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4828ns | 4828ns | -95.71% |
| abi_payload_cost_scatter_scalar_payload | 112491ns | 112491ns | base |
| abi_payload_cost_scatter_soa_payload | 35268ns | 35268ns | -68.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_scalar_payload | 110273ns | base | --- | [109751, 110720] | --- | --- | --- | --- |
| abi_payload_cost_scatter_null_entry | 2544ns | -107740.0ns (-97.7%) | [-108134, -107225]ns | [2515, 2586] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_scatter_soa_payload | 32935ns | -77255.2ns (-70.1%) | [-77879, -76623]ns | [32640, 33411] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_scatter_scalar_payload | abi_payload_cost_scatter_null_entry | abi_payload_cost_scatter_soa_payload |
|---|---|---|---|
| 1 | 110453ns | -97.7% | -70.2% |
| 2 | 109528ns | -97.7% | -69.9% |
| 3 | 110093ns | -97.7% | -70.1% |
| 4 | 109974ns | -97.7% | -70.4% |
| 5 | 110900ns | -97.7% | -70.5% |
| 6 | 110540ns | -97.7% | -69.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_scatter_null_entry | 0.012 | ok |
| abi_payload_cost_scatter_scalar_payload | 0.015 | ok |
| abi_payload_cost_scatter_soa_payload | -0.128 | ok |

**Consistency summary:**

- **abi_payload_cost_scatter_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 106979.4ns | 2548.5ns | 4197.8% | HIGH |
| abi_payload_cost_scatter_scalar_payload | 352490.3ns | 110248.1ns | 319.7% | HIGH |
| abi_payload_cost_scatter_soa_payload | 186318.5ns | 32995.5ns | 564.7% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_scatter_null_entry (n=6, range 2512.5-2586.2 ns)
   2512.5 |########################################
   2516.2 |########################################
   2519.9 |
   2523.6 |
   2527.2 |
   2530.9 |
   2534.6 |
   2538.3 |########################################
   2542.0 |
   2545.7 |########################################
   2549.4 |
   2553.1 |
   2556.8 |
   2560.4 |
   2564.1 |
   2567.8 |
   2571.5 |
   2575.2 |
   2578.9 |########################################
   2582.6 |
  (0 below, 1 above range)

abi_payload_cost_scatter_scalar_payload (n=6, range 109528.3-110719.8 ns)
  109528.3 |########################################
  109587.9 |
  109647.4 |
  109707.0 |
  109766.6 |
  109826.2 |
  109885.8 |
  109945.3 |########################################
  110004.9 |
  110064.5 |########################################
  110124.1 |
  110183.6 |
  110243.2 |
  110302.8 |
  110362.4 |
  110421.9 |########################################
  110481.5 |########################################
  110541.1 |
  110600.7 |
  110660.2 |
  (0 below, 1 above range)

abi_payload_cost_scatter_soa_payload (n=6, range 32600.0-33410.8 ns)
  32600.0 |########################################
  32640.5 |########################################
  32681.1 |
  32721.6 |
  32762.2 |
  32802.7 |
  32843.3 |
  32883.8 |########################################
  32924.3 |########################################
  32964.9 |########################################
  33005.4 |
  33046.0 |
  33086.5 |
  33127.1 |
  33167.6 |
  33208.1 |
  33248.7 |
  33289.2 |
  33329.8 |
  33370.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_scatter_null_entry**: bridge=4201.1% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_scalar_payload**: bridge=319.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_soa_payload**: bridge=564.0% of algo (FFI overhead may distort results)
