# abi_payload_cost (madd)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_madd_scalar_payload has the worst median (11.99 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_madd_null_entry at 2.61 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_madd_null_entry dominates: 169520% faster than the next best (abi_payload_cost_madd_soa_payload)

abi_payload_cost_madd_null_entry (2.61 us) leads abi_payload_cost_madd_soa_payload (4.42 ms) by 169520%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_madd_null_entry beats baseline by 100% (significant)

abi_payload_cost_madd_null_entry is -11.99 ms (100%) faster than baseline abi_payload_cost_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_madd_scalar_payload is an outlier: 4602.9x slower than the field

abi_payload_cost_madd_scalar_payload (11.99 ms) is 4602.9x the fastest (2.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_madd_scalar_payload shows alternating (throttle bounce) (autocorr -0.64)

abi_payload_cost_madd_scalar_payload's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4602.9x the fastest

Fastest abi_payload_cost_madd_null_entry (2.61 us) to slowest abi_payload_cost_madd_scalar_payload (11.99 ms): 4602.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_madd_null_entry** at 2605.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4602.88x (fastest 2605.4 ns, slowest 11992349.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4890ns | 4911ns | 4600ns | 4838ns | 5114ns | -99.96% |
| abi_payload_cost_madd_scalar_payload | 12027503ns | 11996236ns | 11906101ns | 11976880ns | 12164137ns | base |
| abi_payload_cost_madd_soa_payload | 4440088ns | 4422472ns | 4384795ns | 4418531ns | 4500072ns | -63.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 2565ns | 2407ns | 2666ns | -99.98% | 0.399 |
| abi_payload_cost_madd_scalar_payload | 12023704ns | 11902488ns | 12160122ns | base | 0.000 |
| abi_payload_cost_madd_soa_payload | 4437006ns | 4382007ns | 4496774ns | -63.10% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 46579.8 | 2757.2 | 2564.7 | n/a |
| abi_payload_cost_madd_scalar_payload | 108068.4 | 11997713.9 | 12023703.8 | n/a |
| abi_payload_cost_madd_soa_payload | 80937.6 | 4447239.7 | 4437006.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.425 Gops/s** (abi_payload_cost_madd_null_entry; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_madd_null_entry | 0.393 | 92.4% |
| abi_payload_cost_madd_scalar_payload | 0.000 | 0.0% |
| abi_payload_cost_madd_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4890ns | 4890ns | -99.96% |
| abi_payload_cost_madd_scalar_payload | 12027503ns | 12027503ns | base |
| abi_payload_cost_madd_soa_payload | 4440088ns | 4440088ns | -63.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_madd_scalar_payload | 11992350ns | base | --- | [11918640, 12160122] | --- | --- | --- | --- |
| abi_payload_cost_madd_null_entry | 2605ns | -11989927.3ns (-100.0%) | [-12157493, -11915998]ns | [2422, 2666] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_madd_soa_payload | 4419270ns | -7573834.4ns (-63.2%) | [-7663348, -7522910]ns | [4394976, 4496774] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_madd_scalar_payload | abi_payload_cost_madd_null_entry | abi_payload_cost_madd_soa_payload |
|---|---|---|---|
| 1 | 12006324ns | -100.0% | -63.1% |
| 2 | 12104314ns | -100.0% | -63.1% |
| 3 | 11934792ns | -100.0% | -63.3% |
| 4 | 12215929ns | -100.0% | -62.9% |
| 5 | 11902488ns | -100.0% | -63.0% |
| 6 | 11978376ns | -100.0% | -63.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_madd_null_entry | -0.258 | moderate- |
| abi_payload_cost_madd_scalar_payload | -0.635 | HIGH- (thermal bounce) |
| abi_payload_cost_madd_soa_payload | -0.619 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_payload_cost_madd_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 133484.7ns | 2564.7ns | 5204.8% | HIGH |
| abi_payload_cost_madd_scalar_payload | 36188717.7ns | 12023703.8ns | 301.0% | HIGH |
| abi_payload_cost_madd_soa_payload | 13479565.6ns | 4437006.3ns | 303.8% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_madd_null_entry (n=6, range 2406.7-2666.1 ns)
   2406.7 |########################################
   2419.7 |
   2432.6 |########################################
   2445.6 |
   2458.6 |
   2471.5 |
   2484.5 |
   2497.5 |
   2510.4 |
   2523.4 |
   2536.4 |
   2549.3 |
   2562.3 |
   2575.3 |
   2588.2 |########################################
   2601.2 |
   2614.2 |########################################
   2627.1 |########################################
   2640.1 |
   2653.1 |
  (0 below, 1 above range)

abi_payload_cost_madd_scalar_payload (n=6, range 11902488.3-12160121.5 ns)
  11902488.3 |########################################
  11915370.0 |
  11928251.6 |########################################
  11941133.3 |
  11954014.9 |
  11966896.6 |########################################
  11979778.3 |
  11992659.9 |
  12005541.6 |########################################
  12018423.2 |
  12031304.9 |
  12044186.6 |
  12057068.2 |
  12069949.9 |
  12082831.5 |
  12095713.2 |########################################
  12108594.9 |
  12121476.5 |
  12134358.2 |
  12147239.8 |
  (0 below, 1 above range)

abi_payload_cost_madd_soa_payload (n=6, range 4382007.1-4496773.8 ns)
  4382007.1 |####################
  4387745.4 |
  4393483.8 |
  4399222.1 |
  4404960.4 |########################################
  4410698.8 |
  4416437.1 |
  4422175.4 |
  4427913.8 |####################
  4433652.1 |
  4439390.4 |
  4445128.8 |
  4450867.1 |
  4456605.4 |
  4462343.8 |####################
  4468082.1 |
  4473820.4 |
  4479558.8 |
  4485297.1 |
  4491035.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_madd_null_entry**: bridge=5167.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_scalar_payload**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_soa_payload**: bridge=302.1% of algo (FFI overhead may distort results)
