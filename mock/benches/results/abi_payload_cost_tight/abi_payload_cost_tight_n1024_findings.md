# abi_payload_cost (tight)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_tight_scalar_payload has the worst median (9.97 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_tight_null_entry at 2.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_tight_null_entry dominates: 151374% faster than the next best (abi_payload_cost_tight_soa_payload)

abi_payload_cost_tight_null_entry (2.54 us) leads abi_payload_cost_tight_soa_payload (3.84 ms) by 151374%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_tight_null_entry beats baseline by 100% (significant)

abi_payload_cost_tight_null_entry is -9.97 ms (100%) faster than baseline abi_payload_cost_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_tight_scalar_payload is an outlier: 3933.0x slower than the field

abi_payload_cost_tight_scalar_payload (9.97 ms) is 3933.0x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3933.0x the fastest

Fastest abi_payload_cost_tight_null_entry (2.54 us) to slowest abi_payload_cost_tight_scalar_payload (9.97 ms): 3933.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_tight_null_entry** at 2535.2 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3932.95x (fastest 2535.2 ns, slowest 9970826.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 4804ns | 4832ns | 4573ns | 4777ns | 4958ns | -99.95% |
| abi_payload_cost_tight_scalar_payload | 10075512ns | 9974106ns | 9941051ns | 9966927ns | 10305620ns | base |
| abi_payload_cost_tight_soa_payload | 3851968ns | 3843031ns | 3827703ns | 3838076ns | 3884938ns | -61.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 2526ns | 2402ns | 2610ns | -99.97% | 0.405 |
| abi_payload_cost_tight_scalar_payload | 10072240ns | 9938305ns | 10302136ns | base | 0.000 |
| abi_payload_cost_tight_soa_payload | 3849076ns | 3825012ns | 3881970ns | -61.79% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 46459.0 | 2684.7 | 2525.7 | n/a |
| abi_payload_cost_tight_scalar_payload | 89311.5 | 10108087.4 | 10072239.7 | n/a |
| abi_payload_cost_tight_soa_payload | 69170.7 | 3852423.7 | 3849075.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.426 Gops/s** (abi_payload_cost_tight_null_entry; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_tight_null_entry | 0.404 | 94.7% |
| abi_payload_cost_tight_scalar_payload | 0.000 | 0.0% |
| abi_payload_cost_tight_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_tight_null_entry | 4804ns | 4804ns | -99.95% |
| abi_payload_cost_tight_scalar_payload | 10075512ns | 10075512ns | base |
| abi_payload_cost_tight_soa_payload | 3851968ns | 3851968ns | -61.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_tight_scalar_payload | 9970826ns | base | --- | [9943757, 10302136] | --- | --- | --- | --- |
| abi_payload_cost_tight_null_entry | 2535ns | -9968341.7ns (-100.0%) | [-10299575, -9941225]ns | [2432, 2610] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_tight_soa_payload | 3840172ns | -6128082.2ns (-61.5%) | [-6422738, -6118672]ns | [3825085, 3881970] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_tight_scalar_payload | abi_payload_cost_tight_null_entry | abi_payload_cost_tight_soa_payload |
|---|---|---|---|
| 1 | 9938305ns | -100.0% | -61.5% |
| 2 | 9977242ns | -100.0% | -61.5% |
| 3 | 10627029ns | -100.0% | -63.1% |
| 4 | 9973311ns | -100.0% | -61.5% |
| 5 | 9949210ns | -100.0% | -61.6% |
| 6 | 9968342ns | -100.0% | -61.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_tight_null_entry | -0.313 | moderate- |
| abi_payload_cost_tight_scalar_payload | -0.189 | ok |
| abi_payload_cost_tight_soa_payload | -0.081 | ok |

**Consistency summary:**

- **abi_payload_cost_tight_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 132967.1ns | 2525.7ns | 5264.6% | HIGH |
| abi_payload_cost_tight_scalar_payload | 30498196.5ns | 10072239.7ns | 302.8% | HIGH |
| abi_payload_cost_tight_soa_payload | 11624822.3ns | 3849075.8ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_tight_null_entry (n=6, range 2401.7-2609.6 ns)
   2401.7 |########################################
   2412.1 |
   2422.5 |
   2432.9 |
   2443.3 |
   2453.7 |########################################
   2464.1 |
   2474.5 |
   2484.9 |
   2495.3 |########################################
   2505.7 |
   2516.0 |
   2526.4 |
   2536.8 |
   2547.2 |
   2557.6 |########################################
   2568.0 |
   2578.4 |
   2588.8 |
   2599.2 |########################################
  (0 below, 1 above range)

abi_payload_cost_tight_scalar_payload (n=6, range 9938304.6-10302135.7 ns)
  9938304.6 |########################################
  9956496.2 |########################################
  9974687.7 |####################
  9992879.3 |
  10011070.8 |
  10029262.4 |
  10047453.9 |
  10065645.5 |
  10083837.0 |
  10102028.6 |
  10120220.1 |
  10138411.7 |
  10156603.2 |
  10174794.8 |
  10192986.3 |
  10211177.9 |
  10229369.4 |
  10247561.0 |
  10265752.5 |
  10283944.1 |
  (0 below, 1 above range)

abi_payload_cost_tight_soa_payload (n=6, range 3825012.5-3881970.0 ns)
  3825012.5 |########################################
  3827860.4 |
  3830708.2 |
  3833556.1 |
  3836404.0 |####################
  3839251.9 |####################
  3842099.8 |####################
  3844947.6 |
  3847795.5 |
  3850643.4 |
  3853491.2 |
  3856339.1 |
  3859187.0 |
  3862034.9 |
  3864882.8 |
  3867730.6 |
  3870578.5 |
  3873426.4 |
  3876274.2 |
  3879122.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_tight_null_entry**: bridge=5267.8% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_scalar_payload**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_soa_payload**: bridge=302.1% of algo (FFI overhead may distort results)
