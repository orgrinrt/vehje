# abi_payload_cost (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_wideselect_scalar_payload has the worst median (26.93 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_wideselect_null_entry at 2.59 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_wideselect_null_entry dominates: 152% faster than the next best (abi_payload_cost_wideselect_soa_payload)

abi_payload_cost_wideselect_null_entry (2.59 us) leads abi_payload_cost_wideselect_soa_payload (6.52 us) by 152%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_wideselect_null_entry beats baseline by 90% (significant)

abi_payload_cost_wideselect_null_entry is -24.32 us (90%) faster than baseline abi_payload_cost_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_wideselect_scalar_payload is an outlier: 10.4x slower than the field

abi_payload_cost_wideselect_scalar_payload (26.93 us) is 10.4x the fastest (2.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 10.4x the fastest

Fastest abi_payload_cost_wideselect_null_entry (2.59 us) to slowest abi_payload_cost_wideselect_scalar_payload (26.93 us): 10.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_wideselect_null_entry** at 2585.4 ns median (-90.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.41x (fastest 2585.4 ns, slowest 26925.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4827ns | 4886ns | 4549ns | 4812ns | 4987ns | -83.28% |
| abi_payload_cost_wideselect_scalar_payload | 28874ns | 29197ns | 27808ns | 28888ns | 29387ns | base |
| abi_payload_cost_wideselect_soa_payload | 8857ns | 8856ns | 8685ns | 8821ns | 8995ns | -69.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 2564ns | 2411ns | 2667ns | -90.37% | 0.002 |
| abi_payload_cost_wideselect_scalar_payload | 26624ns | 25667ns | 27083ns | base | 0.000 |
| abi_payload_cost_wideselect_soa_payload | 6528ns | 6403ns | 6635ns | -75.48% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 19563.1 | 2684.9 | 2564.3 | n/a |
| abi_payload_cost_wideselect_scalar_payload | 19768.8 | 26773.5 | 26623.7 | n/a |
| abi_payload_cost_wideselect_soa_payload | 20567.0 | 6615.8 | 6528.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_payload_cost_wideselect_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | 0.002 | 93.2% |
| abi_payload_cost_wideselect_scalar_payload | 0.000 | 9.0% |
| abi_payload_cost_wideselect_soa_payload | 0.001 | 37.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4827ns | 4827ns | -83.28% |
| abi_payload_cost_wideselect_scalar_payload | 28874ns | 28874ns | base |
| abi_payload_cost_wideselect_soa_payload | 8857ns | 8857ns | -69.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_scalar_payload | 26925ns | base | --- | [25862, 27083] | --- | --- | --- | --- |
| abi_payload_cost_wideselect_null_entry | 2585ns | -24320.0ns (-90.3%) | [-24549, -23309]ns | [2441, 2667] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_wideselect_soa_payload | 6522ns | -20350.3ns (-75.6%) | [-20556, -19381]ns | [6428, 6635] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_wideselect_scalar_payload | abi_payload_cost_wideselect_null_entry | abi_payload_cost_wideselect_soa_payload |
|---|---|---|---|
| 1 | 25667ns | -90.6% | -74.4% |
| 2 | 27044ns | -90.4% | -75.3% |
| 3 | 26925ns | -90.4% | -76.0% |
| 4 | 26926ns | -90.2% | -75.6% |
| 5 | 26058ns | -89.7% | -75.4% |
| 6 | 27123ns | -90.9% | -76.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | -0.123 | ok |
| abi_payload_cost_wideselect_scalar_payload | -0.346 | moderate- |
| abi_payload_cost_wideselect_soa_payload | -0.223 | moderate- |

**Consistency summary:**

- **abi_payload_cost_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 107056.7ns | 2564.3ns | 4174.8% | HIGH |
| abi_payload_cost_wideselect_scalar_payload | 179317.1ns | 26623.7ns | 673.5% | HIGH |
| abi_payload_cost_wideselect_soa_payload | 121206.0ns | 6528.2ns | 1856.7% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_wideselect_null_entry (n=6, range 2410.8-2666.9 ns)
   2410.8 |########################################
   2423.6 |
   2436.4 |
   2449.2 |
   2462.0 |########################################
   2474.8 |
   2487.6 |
   2500.4 |
   2513.2 |
   2526.0 |
   2538.8 |
   2551.6 |
   2564.4 |########################################
   2577.2 |
   2590.0 |########################################
   2602.8 |
   2615.6 |
   2628.4 |########################################
   2641.2 |
   2654.0 |
  (0 below, 1 above range)

abi_payload_cost_wideselect_scalar_payload (n=6, range 25667.1-27083.3 ns)
  25667.1 |####################
  25737.9 |
  25808.7 |
  25879.5 |
  25950.3 |
  26021.2 |####################
  26092.0 |
  26162.8 |
  26233.6 |
  26304.4 |
  26375.2 |
  26446.0 |
  26516.8 |
  26587.6 |
  26658.4 |
  26729.2 |
  26800.1 |
  26870.9 |########################################
  26941.7 |
  27012.5 |####################
  (0 below, 1 above range)

abi_payload_cost_wideselect_soa_payload (n=6, range 6403.3-6634.6 ns)
   6403.3 |########################################
   6414.9 |
   6426.4 |
   6438.0 |
   6449.6 |########################################
   6461.1 |
   6472.7 |########################################
   6484.3 |
   6495.8 |
   6507.4 |
   6519.0 |
   6530.5 |
   6542.1 |
   6553.6 |########################################
   6565.2 |
   6576.8 |########################################
   6588.3 |
   6599.9 |
   6611.5 |
   6623.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_wideselect_null_entry**: bridge=4138.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_scalar_payload**: bridge=671.7% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_soa_payload**: bridge=1860.2% of algo (FFI overhead may distort results)
