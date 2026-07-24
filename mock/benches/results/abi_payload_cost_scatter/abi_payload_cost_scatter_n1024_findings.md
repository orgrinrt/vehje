# abi_payload_cost (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_scatter_scalar_payload has the worst median (9.01 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_scatter_null_entry at 2.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_scatter_null_entry dominates: 141210% faster than the next best (abi_payload_cost_scatter_soa_payload)

abi_payload_cost_scatter_null_entry (2.49 us) leads abi_payload_cost_scatter_soa_payload (3.52 ms) by 141210%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_scatter_null_entry beats baseline by 100% (significant)

abi_payload_cost_scatter_null_entry is -9.01 ms (100%) faster than baseline abi_payload_cost_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_scatter_scalar_payload is an outlier: 3615.0x slower than the field

abi_payload_cost_scatter_scalar_payload (9.01 ms) is 3615.0x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3615.0x the fastest

Fastest abi_payload_cost_scatter_null_entry (2.49 us) to slowest abi_payload_cost_scatter_scalar_payload (9.01 ms): 3615.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_scatter_null_entry** at 2491.9 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3614.95x (fastest 2491.9 ns, slowest 9008104.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4768ns | 4752ns | 4518ns | 4729ns | 4952ns | -99.95% |
| abi_payload_cost_scatter_scalar_payload | 9021197ns | 9011173ns | 8970273ns | 9010494ns | 9062713ns | base |
| abi_payload_cost_scatter_soa_payload | 3519911ns | 3523961ns | 3507018ns | 3520753ns | 3525095ns | -60.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 2488ns | 2359ns | 2594ns | -99.97% | 0.411 |
| abi_payload_cost_scatter_scalar_payload | 9018139ns | 8966993ns | 9059844ns | base | 0.000 |
| abi_payload_cost_scatter_soa_payload | 3517257ns | 3504631ns | 3522370ns | -61.00% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 46111.2 | 2727.6 | 2488.5 | n/a |
| abi_payload_cost_scatter_scalar_payload | 82885.1 | 9049658.7 | 9018139.1 | n/a |
| abi_payload_cost_scatter_soa_payload | 60997.1 | 3517395.9 | 3517257.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.434 Gops/s** (abi_payload_cost_scatter_null_entry; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_scatter_null_entry | 0.411 | 94.7% |
| abi_payload_cost_scatter_scalar_payload | 0.000 | 0.0% |
| abi_payload_cost_scatter_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4768ns | 4768ns | -99.95% |
| abi_payload_cost_scatter_scalar_payload | 9021197ns | 9021197ns | base |
| abi_payload_cost_scatter_soa_payload | 3519911ns | 3519911ns | -60.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_scalar_payload | 9008105ns | base | --- | [8986469, 9059844] | --- | --- | --- | --- |
| abi_payload_cost_scatter_null_entry | 2492ns | -9005572.5ns (-100.0%) | [-9057429, -8983950]ns | [2380, 2594] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_scatter_soa_payload | 3521300ns | -5486804.8ns (-60.9%) | [-5546469, -5469371]ns | [3508103, 3522370] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_scatter_scalar_payload | abi_payload_cost_scatter_null_entry | abi_payload_cost_scatter_soa_payload |
|---|---|---|---|
| 1 | 9019910ns | -100.0% | -61.0% |
| 2 | 9007183ns | -100.0% | -60.9% |
| 3 | 9099778ns | -100.0% | -61.5% |
| 4 | 9009026ns | -100.0% | -60.9% |
| 5 | 9005945ns | -100.0% | -60.9% |
| 6 | 8966993ns | -100.0% | -60.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_scatter_null_entry | -0.073 | ok |
| abi_payload_cost_scatter_scalar_payload | -0.096 | ok |
| abi_payload_cost_scatter_soa_payload | -0.327 | moderate- |

**Consistency summary:**

- **abi_payload_cost_scatter_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 132061.9ns | 2488.5ns | 5306.9% | HIGH |
| abi_payload_cost_scatter_scalar_payload | 27171332.4ns | 9018139.1ns | 301.3% | HIGH |
| abi_payload_cost_scatter_soa_payload | 10616901.0ns | 3517257.4ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_scatter_null_entry (n=6, range 2359.2-2593.6 ns)
   2359.2 |########################################
   2370.9 |
   2382.6 |
   2394.4 |########################################
   2406.1 |
   2417.8 |
   2429.5 |
   2441.2 |
   2452.9 |
   2464.7 |########################################
   2476.4 |
   2488.1 |
   2499.8 |
   2511.5 |########################################
   2523.2 |########################################
   2535.0 |
   2546.7 |
   2558.4 |
   2570.1 |
   2581.8 |
  (0 below, 1 above range)

abi_payload_cost_scatter_scalar_payload (n=6, range 8966992.9-9059843.8 ns)
  8966992.9 |####################
  8971635.4 |
  8976278.0 |
  8980920.5 |
  8985563.1 |
  8990205.6 |
  8994848.2 |
  8999490.7 |
  9004133.2 |########################################
  9008775.8 |####################
  9013418.3 |
  9018060.9 |####################
  9022703.4 |
  9027346.0 |
  9031988.5 |
  9036631.0 |
  9041273.6 |
  9045916.1 |
  9050558.7 |
  9055201.2 |
  (0 below, 1 above range)

abi_payload_cost_scatter_soa_payload (n=6, range 3504631.2-3522369.6 ns)
  3504631.2 |####################
  3505518.1 |
  3506405.0 |
  3507292.0 |
  3508178.9 |
  3509065.8 |
  3509952.7 |
  3510839.6 |####################
  3511726.6 |
  3512613.5 |
  3513500.4 |
  3514387.3 |
  3515274.2 |
  3516161.2 |
  3517048.1 |
  3517935.0 |
  3518821.9 |
  3519708.8 |####################
  3520595.8 |
  3521482.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_scatter_null_entry**: bridge=5305.1% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_scalar_payload**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_soa_payload**: bridge=301.7% of algo (FFI overhead may distort results)
