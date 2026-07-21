# Closure representation: create-once-call-many, flat vs linked (access cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_call_many_flat**

## Highlights

Baseline for all deltas below: **closure_call_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### closure_call_many_flat dominates: 198% faster than the next best (closure_call_many_linked)

closure_call_many_flat (3.19 us) leads closure_call_many_linked (9.49 us) by 198%, a clear separation rather than a photo finish. CV 12.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### closure_call_many_flat is fastest but the noisiest (CV 12.4%)

closure_call_many_flat wins on median (3.19 us) yet has the highest variance (CV 12.4%), while closure_call_many_linked is the steadiest (CV 10.4%, 9.49 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (closure_call_many_flat)

The baseline closure_call_many_flat is the fastest (3.19 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (closure_call_many_flat) is the fastest** at 3185.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.98x (fastest 3185.6 ns, slowest 9489.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_call_many_flat | 6121ns | 5775ns | 5510ns | 5773ns | 6949ns | base |
| closure_call_many_linked | 12607ns | 12070ns | 11297ns | 12054ns | 14091ns | +105.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_call_many_flat | 3378ns | 3038ns | 3837ns | base | 0.019 |
| closure_call_many_linked | 9926ns | 8883ns | 11121ns | +193.87% | 0.006 |

## Performance model

- Peak throughput: **0.021 Gops/s** (closure_call_many_flat; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_call_many_flat | 0.020 | 95.4% |
| closure_call_many_linked | 0.007 | 32.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_call_many_flat | 6121ns | 6121ns | base |
| closure_call_many_linked | 12607ns | 12607ns | +105.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_call_many_flat | 3186ns | base | --- | [3110, 3837] | --- | --- | --- | --- |
| closure_call_many_linked | 9489ns | +6361.0ns (+199.7%) | [+6000, +7284]ns | [9167, 11121] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_call_many_flat | closure_call_many_linked |
|---|---|---|
| 1 | 4219ns | +183.0% |
| 2 | 3455ns | +198.2% |
| 3 | 3182ns | +198.3% |
| 4 | 3038ns | +211.1% |
| 5 | 3185ns | +197.9% |
| 6 | 3186ns | +178.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_call_many_flat | 0.232 | moderate+ |
| closure_call_many_linked | 0.248 | moderate+ |

**Consistency summary:**

- **closure_call_many_linked**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_call_many_flat | 4.2ns | 3377.6ns | 0.1% |  |
| closure_call_many_linked | 60.3ns | 9925.8ns | 0.6% |  |

## Distribution (algo ns)

```
closure_call_many_flat (n=6, range 3038.3-3837.1 ns)
   3038.3 |#############
   3078.2 |
   3118.2 |
   3158.1 |########################################
   3198.1 |
   3238.0 |
   3277.9 |
   3317.9 |
   3357.8 |
   3397.8 |
   3437.7 |#############
   3477.6 |
   3517.6 |
   3557.5 |
   3597.5 |
   3637.4 |
   3677.3 |
   3717.3 |
   3757.2 |
   3797.2 |
  (0 below, 1 above range)

closure_call_many_linked (n=6, range 8882.9-11121.0 ns)
   8882.9 |#############
   8994.8 |
   9106.7 |
   9218.6 |
   9330.5 |
   9442.4 |########################################
   9554.3 |
   9666.2 |
   9778.1 |
   9890.0 |
  10002.0 |
  10113.9 |
  10225.8 |#############
  10337.7 |
  10449.6 |
  10561.5 |
  10673.4 |
  10785.3 |
  10897.2 |
  11009.1 |
  (0 below, 1 above range)

```
