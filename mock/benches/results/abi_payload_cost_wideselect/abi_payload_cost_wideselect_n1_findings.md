# abi_payload_cost (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_wideselect_scalar_payload has the worst median (9.55 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_wideselect_null_entry at 2.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_wideselect_null_entry beats baseline by 73% (significant)

abi_payload_cost_wideselect_null_entry is -6.95 us (73%) faster than baseline abi_payload_cost_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_wideselect_scalar_payload is an outlier: 3.7x slower than the field

abi_payload_cost_wideselect_scalar_payload (9.55 us) is 3.7x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_wideselect_scalar_payload shows alternating (throttle bounce) (autocorr -0.69)

abi_payload_cost_wideselect_scalar_payload's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_payload_cost_wideselect_null_entry (2.56 us) to slowest abi_payload_cost_wideselect_scalar_payload (9.55 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader abi_payload_cost_wideselect_null_entry vs stability leader abi_payload_cost_wideselect_soa_payload (+3% speed for 2.5x steadier)

abi_payload_cost_wideselect_null_entry is fastest (2.56 us, CV 3.2%); abi_payload_cost_wideselect_soa_payload gives up 2.5% median for 2.5x lower variance (CV 1.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: abi_payload_cost_wideselect_null_entry** at 2555.4 ns median (-73.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.74x (fastest 2555.4 ns, slowest 9548.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4877ns | 4862ns | 4626ns | 4817ns | 5092ns | -58.82% |
| abi_payload_cost_wideselect_scalar_payload | 11844ns | 11839ns | 11527ns | 11776ns | 12104ns | base |
| abi_payload_cost_wideselect_soa_payload | 4938ns | 4938ns | 4848ns | 4916ns | 5017ns | -58.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 2555ns | 2417ns | 2651ns | -73.21% | 0.000 |
| abi_payload_cost_wideselect_scalar_payload | 9537ns | 9302ns | 9734ns | base | 0.000 |
| abi_payload_cost_wideselect_soa_payload | 2614ns | 2558ns | 2648ns | -72.59% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 19987.8 | 2745.5 | 2554.6 | n/a |
| abi_payload_cost_wideselect_scalar_payload | 20497.2 | 9691.4 | 9536.6 | n/a |
| abi_payload_cost_wideselect_soa_payload | 20777.8 | 2977.6 | 2613.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_payload_cost_wideselect_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | 0.000 | 94.6% |
| abi_payload_cost_wideselect_scalar_payload | 0.000 | 25.3% |
| abi_payload_cost_wideselect_soa_payload | 0.000 | 92.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4877ns | 4877ns | -58.82% |
| abi_payload_cost_wideselect_scalar_payload | 11844ns | 11844ns | base |
| abi_payload_cost_wideselect_soa_payload | 4938ns | 4938ns | -58.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_scalar_payload | 9549ns | base | --- | [9327, 9734] | --- | --- | --- | --- |
| abi_payload_cost_wideselect_null_entry | 2555ns | -6945.4ns (-72.7%) | [-7277, -6724]ns | [2457, 2651] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_wideselect_soa_payload | 2620ns | -6944.3ns (-72.7%) | [-7109, -6715]ns | [2574, 2648] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_wideselect_scalar_payload | abi_payload_cost_wideselect_null_entry | abi_payload_cost_wideselect_soa_payload |
|---|---|---|---|
| 1 | 9810ns | -75.4% | -73.2% |
| 2 | 9302ns | -71.5% | -72.5% |
| 3 | 9623ns | -73.4% | -73.1% |
| 4 | 9353ns | -72.7% | -71.5% |
| 5 | 9658ns | -74.1% | -72.9% |
| 6 | 9474ns | -72.0% | -72.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | -0.461 | moderate- |
| abi_payload_cost_wideselect_scalar_payload | -0.687 | HIGH- (thermal bounce) |
| abi_payload_cost_wideselect_soa_payload | -0.053 | ok |

**Consistency summary:**

- **abi_payload_cost_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 106092.2ns | 2554.6ns | 4153.0% | HIGH |
| abi_payload_cost_wideselect_scalar_payload | 127834.6ns | 9536.6ns | 1340.5% | HIGH |
| abi_payload_cost_wideselect_soa_payload | 103104.9ns | 2613.8ns | 3944.6% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_wideselect_null_entry (n=6, range 2417.1-2651.0 ns)
   2417.1 |####################
   2428.8 |
   2440.5 |
   2452.2 |
   2463.9 |
   2475.6 |
   2487.3 |####################
   2499.0 |
   2510.7 |
   2522.4 |
   2534.1 |
   2545.7 |########################################
   2557.4 |
   2569.1 |
   2580.8 |
   2592.5 |
   2604.2 |
   2615.9 |
   2627.6 |
   2639.3 |####################
  (0 below, 1 above range)

abi_payload_cost_wideselect_scalar_payload (n=6, range 9301.7-9734.0 ns)
   9301.7 |########################################
   9323.3 |
   9344.9 |########################################
   9366.5 |
   9388.2 |
   9409.8 |
   9431.4 |
   9453.0 |########################################
   9474.6 |
   9496.2 |
   9517.8 |
   9539.4 |
   9561.1 |
   9582.7 |
   9604.3 |########################################
   9625.9 |
   9647.5 |########################################
   9669.1 |
   9690.7 |
   9712.3 |
  (0 below, 1 above range)

abi_payload_cost_wideselect_soa_payload (n=6, range 2558.3-2647.8 ns)
   2558.3 |####################
   2562.8 |
   2567.2 |
   2571.7 |
   2576.2 |
   2580.7 |
   2585.1 |####################
   2589.6 |
   2594.1 |
   2598.6 |
   2603.0 |
   2607.5 |
   2612.0 |
   2616.4 |########################################
   2620.9 |
   2625.4 |####################
   2629.9 |
   2634.3 |
   2638.8 |
   2643.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_wideselect_null_entry**: bridge=4146.7% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_scalar_payload**: bridge=1334.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_soa_payload**: bridge=3930.2% of algo (FFI overhead may distort results)
