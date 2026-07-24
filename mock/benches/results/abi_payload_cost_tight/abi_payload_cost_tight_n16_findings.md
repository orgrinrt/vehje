# abi_payload_cost (tight)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_tight_scalar_payload has the worst median (104.88 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_tight_null_entry at 2.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_tight_null_entry dominates: 1459% faster than the next best (abi_payload_cost_tight_soa_payload)

abi_payload_cost_tight_null_entry (2.47 us) leads abi_payload_cost_tight_soa_payload (38.54 us) by 1459%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_tight_null_entry beats baseline by 98% (significant)

abi_payload_cost_tight_null_entry is -102.36 us (98%) faster than baseline abi_payload_cost_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_tight_scalar_payload is an outlier: 42.4x slower than the field

abi_payload_cost_tight_scalar_payload (104.88 us) is 42.4x the fastest (2.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_tight_soa_payload shows alternating (throttle bounce) (autocorr -0.52)

abi_payload_cost_tight_soa_payload's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 42.4x the fastest

Fastest abi_payload_cost_tight_null_entry (2.47 us) to slowest abi_payload_cost_tight_scalar_payload (104.88 us): 42.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_tight_null_entry** at 2472.9 ns median (-97.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 42.41x (fastest 2472.9 ns, slowest 104883.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 4745ns | 4705ns | 4535ns | 4687ns | 4939ns | -95.57% |
| abi_payload_cost_tight_scalar_payload | 107068ns | 107121ns | 105937ns | 106891ns | 107898ns | base |
| abi_payload_cost_tight_soa_payload | 40738ns | 40761ns | 40311ns | 40622ns | 41125ns | -61.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 2493ns | 2401ns | 2589ns | -97.62% | 0.006 |
| abi_payload_cost_tight_scalar_payload | 104833ns | 103758ns | 105636ns | base | 0.000 |
| abi_payload_cost_tight_soa_payload | 38501ns | 38059ns | 38878ns | -63.27% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 20743.4 | 2704.0 | 2493.3 | 0 |
| abi_payload_cost_tight_scalar_payload | 20271.9 | 104817.3 | 104833.4 | n/a |
| abi_payload_cost_tight_soa_payload | 20462.2 | 38273.1 | 38500.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_payload_cost_tight_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_tight_null_entry | 0.006 | 97.1% |
| abi_payload_cost_tight_scalar_payload | 0.000 | 2.3% |
| abi_payload_cost_tight_soa_payload | 0.000 | 6.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_tight_null_entry | 4745ns | 4745ns | -95.57% |
| abi_payload_cost_tight_scalar_payload | 107068ns | 107068ns | base |
| abi_payload_cost_tight_soa_payload | 40738ns | 40738ns | -61.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_tight_scalar_payload | 104883ns | base | --- | [103981, 105636] | --- | --- | --- | --- |
| abi_payload_cost_tight_null_entry | 2473ns | -102359.0ns (-97.6%) | [-103153, -101508]ns | [2418, 2589] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_tight_soa_payload | 38542ns | -66601.5ns (-63.5%) | [-67294, -65103]ns | [38081, 38878] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_tight_scalar_payload | abi_payload_cost_tight_null_entry | abi_payload_cost_tight_soa_payload |
|---|---|---|---|
| 1 | 104204ns | -97.6% | -62.9% |
| 2 | 105654ns | -97.6% | -64.0% |
| 3 | 105617ns | -97.7% | -63.4% |
| 4 | 105142ns | -97.5% | -63.4% |
| 5 | 103758ns | -97.6% | -62.3% |
| 6 | 104624ns | -97.7% | -63.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_tight_null_entry | -0.352 | moderate- |
| abi_payload_cost_tight_scalar_payload | 0.088 | ok |
| abi_payload_cost_tight_soa_payload | -0.516 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_payload_cost_tight_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 106084.7ns | 2493.3ns | 4254.8% | HIGH |
| abi_payload_cost_tight_scalar_payload | 335662.8ns | 104833.4ns | 320.2% | HIGH |
| abi_payload_cost_tight_soa_payload | 212339.1ns | 38500.6ns | 551.5% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_tight_null_entry (n=6, range 2401.2-2588.8 ns)
   2401.2 |########################################
   2410.6 |
   2420.0 |
   2429.3 |########################################
   2438.7 |
   2448.1 |
   2457.5 |########################################
   2466.8 |
   2476.2 |########################################
   2485.6 |
   2495.0 |
   2504.4 |
   2513.7 |
   2523.1 |########################################
   2532.5 |
   2541.9 |
   2551.2 |
   2560.6 |
   2570.0 |
   2579.4 |
  (0 below, 1 above range)

abi_payload_cost_tight_scalar_payload (n=6, range 103758.3-105635.6 ns)
  103758.3 |########################################
  103852.2 |
  103946.0 |
  104039.9 |
  104133.8 |########################################
  104227.6 |
  104321.5 |
  104415.4 |
  104509.2 |
  104603.1 |########################################
  104697.0 |
  104790.8 |
  104884.7 |
  104978.6 |
  105072.4 |########################################
  105166.3 |
  105260.2 |
  105354.0 |
  105447.9 |
  105541.8 |########################################
  (0 below, 1 above range)

abi_payload_cost_tight_soa_payload (n=6, range 38059.2-38878.3 ns)
  38059.2 |########################################
  38100.2 |########################################
  38141.1 |
  38182.1 |
  38223.0 |
  38264.0 |
  38304.9 |
  38345.9 |
  38386.9 |
  38427.8 |########################################
  38468.8 |
  38509.7 |
  38550.7 |
  38591.6 |########################################
  38632.6 |########################################
  38673.6 |
  38714.5 |
  38755.5 |
  38796.4 |
  38837.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_tight_null_entry**: bridge=4324.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_scalar_payload**: bridge=320.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_soa_payload**: bridge=550.7% of algo (FFI overhead may distort results)
