# Residual encoding: register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 206% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (2.11 us) leads carrier_res_tight_stack (6.44 us) by 206%, a clear separation rather than a photo finish. CV 12.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_tight_register is fastest but the noisiest (CV 12.0%)

carrier_res_tight_register wins on median (2.11 us) yet has the highest variance (CV 12.0%), while carrier_res_tight_stack is the steadiest (CV 11.9%, 6.44 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (2.11 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_res_tight_register (2.11 us) to slowest carrier_res_tight_stack (6.44 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 2105.4 ns median
- 1 variant significantly slower than baseline
- Spread: 3.06x (fastest 2105.4 ns, slowest 6437.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 4876ns | 4522ns | 4282ns | 4464ns | 5790ns | base |
| carrier_res_tight_stack | 9327ns | 8966ns | 7927ns | 8680ns | 10997ns | +91.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 2262ns | 2032ns | 2615ns | base | 0.028 |
| carrier_res_tight_stack | 6456ns | 5611ns | 7244ns | +185.46% | 0.010 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.030 | 96.5% |
| carrier_res_tight_stack | 0.010 | 31.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 4876ns | 4876ns | base |
| carrier_res_tight_stack | 9327ns | 9327ns | +91.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 2105ns | base | --- | [2064, 2615] | --- | --- | --- | --- |
| carrier_res_tight_stack | 6437ns | +3835.3ns (+182.2%) | [+3617, +5131]ns | [5686, 7244] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 2639ns | +146.7% |
| 2 | 2106ns | +173.5% |
| 3 | 2592ns | +146.6% |
| 4 | 2032ns | +176.1% |
| 5 | 2105ns | +208.1% |
| 6 | 2096ns | +280.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | -0.325 | moderate- |
| carrier_res_tight_stack | 0.023 | ok |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 87607.9ns | 2261.5ns | 3873.8% | HIGH |
| carrier_res_tight_stack | 90983.1ns | 6455.9ns | 1409.3% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 2032.1-2615.2 ns)
   2032.1 |#############
   2061.3 |
   2090.4 |########################################
   2119.6 |
   2148.7 |
   2177.9 |
   2207.0 |
   2236.2 |
   2265.4 |
   2294.5 |
   2323.7 |
   2352.8 |
   2382.0 |
   2411.1 |
   2440.3 |
   2469.5 |
   2498.6 |
   2527.8 |
   2556.9 |
   2586.1 |#############
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 5610.8-7244.4 ns)
   5610.8 |########################################
   5692.5 |########################################
   5774.2 |
   5855.8 |
   5937.5 |
   6019.2 |
   6100.9 |
   6182.5 |
   6264.2 |
   6345.9 |########################################
   6427.6 |########################################
   6509.3 |########################################
   6590.9 |
   6672.6 |
   6754.3 |
   6836.0 |
   6917.6 |
   6999.3 |
   7081.0 |
   7162.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=4111.2% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=1413.4% of algo (FFI overhead may distort results)
