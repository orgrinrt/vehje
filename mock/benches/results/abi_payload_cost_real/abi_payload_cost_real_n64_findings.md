# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (478.91 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 7936% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.55 us) leads abi_payload_cost_real_soa_payload (204.62 us) by 7936%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 99% (significant)

abi_payload_cost_real_null_entry is -476.43 us (99%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 188.1x slower than the field

abi_payload_cost_real_scalar_payload (478.91 us) is 188.1x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_real_scalar_payload shows alternating (throttle bounce) (autocorr -0.62)

abi_payload_cost_real_scalar_payload's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 188.1x the fastest

Fastest abi_payload_cost_real_null_entry (2.55 us) to slowest abi_payload_cost_real_scalar_payload (478.91 us): 188.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2546.2 ns median (-99.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 188.08x (fastest 2546.2 ns, slowest 478908.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4817ns | 4794ns | 4582ns | 4743ns | 5046ns | -99.00% |
| abi_payload_cost_real_scalar_payload | 480684ns | 481175ns | 475824ns | 480874ns | 482828ns | base |
| abi_payload_cost_real_soa_payload | 207415ns | 206833ns | 206319ns | 206737ns | 208980ns | -56.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2548ns | 2430ns | 2646ns | -99.47% | 0.025 |
| abi_payload_cost_real_scalar_payload | 478374ns | 473454ns | 480541ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 205192ns | 204116ns | 206725ns | -57.11% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 21654.2 | 2731.9 | 2547.6 | n/a |
| abi_payload_cost_real_scalar_payload | 23889.4 | 478521.0 | 478374.3 | n/a |
| abi_payload_cost_real_soa_payload | 21635.5 | 205090.8 | 205192.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.025 | 95.4% |
| abi_payload_cost_real_scalar_payload | 0.000 | 0.5% |
| abi_payload_cost_real_soa_payload | 0.000 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4817ns | 4817ns | -99.00% |
| abi_payload_cost_real_scalar_payload | 480684ns | 480684ns | base |
| abi_payload_cost_real_soa_payload | 207415ns | 207415ns | -56.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 478908ns | base | --- | [475674, 480541] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2546ns | -476433.2ns (-99.5%) | [-477903, -473144]ns | [2451, 2646] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 204617ns | -273993.6ns (-57.2%) | [-276305, -269247]ns | [204235, 206725] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 479328ns | -99.5% | -57.3% |
| 2 | 478489ns | -99.5% | -56.6% |
| 3 | 477894ns | -99.5% | -57.2% |
| 4 | 481469ns | -99.4% | -57.6% |
| 5 | 473454ns | -99.4% | -56.5% |
| 6 | 479612ns | -99.5% | -57.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.323 | moderate+ |
| abi_payload_cost_real_scalar_payload | -0.624 | HIGH- (thermal bounce) |
| abi_payload_cost_real_soa_payload | -0.443 | moderate- |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 108221.6ns | 2547.6ns | 4248.0% | HIGH |
| abi_payload_cost_real_scalar_payload | 1461368.2ns | 478374.3ns | 305.5% | HIGH |
| abi_payload_cost_real_soa_payload | 637883.7ns | 205192.4ns | 310.9% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2429.6-2645.6 ns)
   2429.6 |########################################
   2440.4 |
   2451.2 |
   2462.0 |########################################
   2472.8 |########################################
   2483.6 |
   2494.4 |
   2505.2 |
   2516.0 |
   2526.8 |
   2537.6 |
   2548.4 |
   2559.2 |
   2570.0 |
   2580.8 |
   2591.6 |
   2602.4 |
   2613.2 |########################################
   2624.0 |########################################
   2634.8 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 473453.7-480540.7 ns)
  473453.7 |########################################
  473808.0 |
  474162.4 |
  474516.7 |
  474871.1 |
  475225.4 |
  475579.8 |
  475934.1 |
  476288.5 |
  476642.8 |
  476997.2 |
  477351.5 |
  477705.9 |########################################
  478060.2 |
  478414.6 |########################################
  478768.9 |
  479123.3 |########################################
  479477.6 |########################################
  479832.0 |
  480186.3 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 204116.2-206724.5 ns)
  204116.2 |########################################
  204246.6 |########################################
  204377.0 |########################################
  204507.5 |
  204637.9 |
  204768.3 |########################################
  204898.7 |
  205029.1 |
  205159.5 |
  205290.0 |
  205420.4 |
  205550.8 |
  205681.2 |
  205811.6 |
  205942.0 |########################################
  206072.5 |
  206202.9 |
  206333.3 |
  206463.7 |
  206594.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4237.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=305.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=310.7% of algo (FFI overhead may distort results)
