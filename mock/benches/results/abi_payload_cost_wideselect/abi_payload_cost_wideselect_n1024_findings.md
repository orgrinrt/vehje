# abi_payload_cost (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_wideselect_scalar_payload has the worst median (8.73 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_wideselect_null_entry at 2.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_wideselect_null_entry dominates: 149004% faster than the next best (abi_payload_cost_wideselect_soa_payload)

abi_payload_cost_wideselect_null_entry (2.48 us) leads abi_payload_cost_wideselect_soa_payload (3.70 ms) by 149004%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_wideselect_null_entry beats baseline by 100% (significant)

abi_payload_cost_wideselect_null_entry is -8.73 ms (100%) faster than baseline abi_payload_cost_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_wideselect_scalar_payload is an outlier: 3519.5x slower than the field

abi_payload_cost_wideselect_scalar_payload (8.73 ms) is 3519.5x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_wideselect_null_entry shows alternating (throttle bounce) (autocorr -0.50)

abi_payload_cost_wideselect_null_entry's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3519.5x the fastest

Fastest abi_payload_cost_wideselect_null_entry (2.48 us) to slowest abi_payload_cost_wideselect_scalar_payload (8.73 ms): 3519.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_wideselect_null_entry** at 2481.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3519.51x (fastest 2481.4 ns, slowest 8733484.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4748ns | 4710ns | 4520ns | 4685ns | 4957ns | -99.95% |
| abi_payload_cost_wideselect_scalar_payload | 8852235ns | 8736484ns | 8710174ns | 8729755ns | 9106985ns | base |
| abi_payload_cost_wideselect_soa_payload | 3817328ns | 3702590ns | 3684872ns | 3698632ns | 4061599ns | -56.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 2497ns | 2382ns | 2599ns | -99.97% | 0.410 |
| abi_payload_cost_wideselect_scalar_payload | 8849197ns | 8707376ns | 9103754ns | base | 0.000 |
| abi_payload_cost_wideselect_soa_payload | 3814445ns | 3682149ns | 4058269ns | -56.90% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 51909.0 | 2712.9 | 2496.5 | n/a |
| abi_payload_cost_wideselect_scalar_payload | 94459.3 | 8752782.1 | 8849197.0 | n/a |
| abi_payload_cost_wideselect_soa_payload | 81574.4 | 3797171.1 | 3814445.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.430 Gops/s** (abi_payload_cost_wideselect_null_entry; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | 0.413 | 96.0% |
| abi_payload_cost_wideselect_scalar_payload | 0.000 | 0.0% |
| abi_payload_cost_wideselect_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4748ns | 4748ns | -99.95% |
| abi_payload_cost_wideselect_scalar_payload | 8852235ns | 8852235ns | base |
| abi_payload_cost_wideselect_soa_payload | 3817328ns | 3817328ns | -56.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_scalar_payload | 8733484ns | base | --- | [8710353, 9103754] | --- | --- | --- | --- |
| abi_payload_cost_wideselect_null_entry | 2481ns | -8731036.2ns (-100.0%) | [-9101312, -8707754]ns | [2409, 2599] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_wideselect_soa_payload | 3699951ns | -5032524.2ns (-57.6%) | [-5119099, -4952631]ns | [3685116, 4058269] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_wideselect_scalar_payload | abi_payload_cost_wideselect_null_entry | abi_payload_cost_wideselect_soa_payload |
|---|---|---|---|
| 1 | 8738654ns | -100.0% | -56.1% |
| 2 | 8713329ns | -100.0% | -57.7% |
| 3 | 9468854ns | -100.0% | -54.8% |
| 4 | 8733883ns | -100.0% | -57.8% |
| 5 | 8707376ns | -100.0% | -57.5% |
| 6 | 8733085ns | -100.0% | -57.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | -0.504 | HIGH- (thermal bounce) |
| abi_payload_cost_wideselect_scalar_payload | -0.234 | moderate- |
| abi_payload_cost_wideselect_soa_payload | -0.348 | moderate- |

**Consistency summary:**

- **abi_payload_cost_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 137587.4ns | 2496.5ns | 5511.2% | HIGH |
| abi_payload_cost_wideselect_scalar_payload | 26360816.2ns | 8849197.0ns | 297.9% | HIGH |
| abi_payload_cost_wideselect_soa_payload | 11426929.9ns | 3814445.4ns | 299.6% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_wideselect_null_entry (n=6, range 2382.5-2599.2 ns)
   2382.5 |########################################
   2393.3 |
   2404.2 |
   2415.0 |
   2425.8 |########################################
   2436.7 |
   2447.5 |
   2458.3 |########################################
   2469.2 |
   2480.0 |
   2490.8 |
   2501.7 |########################################
   2512.5 |
   2523.3 |
   2534.2 |
   2545.0 |########################################
   2555.8 |
   2566.7 |
   2577.5 |
   2588.3 |
  (0 below, 1 above range)

abi_payload_cost_wideselect_scalar_payload (n=6, range 8707376.2-9103754.0 ns)
  8707376.2 |##########################
  8727195.1 |########################################
  8747014.0 |
  8766832.9 |
  8786651.8 |
  8806470.6 |
  8826289.5 |
  8846108.4 |
  8865927.3 |
  8885746.2 |
  8905565.1 |
  8925384.0 |
  8945202.9 |
  8965021.8 |
  8984840.7 |
  9004659.6 |
  9024478.4 |
  9044297.3 |
  9064116.2 |
  9083935.1 |
  (0 below, 1 above range)

abi_payload_cost_wideselect_soa_payload (n=6, range 3682148.8-4058269.0 ns)
  3682148.8 |########################################
  3700954.8 |
  3719760.8 |
  3738566.8 |
  3757372.8 |
  3776178.8 |
  3794984.8 |
  3813790.9 |
  3832596.9 |##########
  3851402.9 |
  3870208.9 |
  3889014.9 |
  3907820.9 |
  3926626.9 |
  3945432.9 |
  3964238.9 |
  3983044.9 |
  4001850.9 |
  4020656.9 |
  4039462.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_wideselect_null_entry**: bridge=5564.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_scalar_payload**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_soa_payload**: bridge=301.7% of algo (FFI overhead may distort results)
