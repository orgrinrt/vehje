# Residual encoding: register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 125% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (2.48 us) leads carrier_res_madd_stack (5.58 us) by 125%, a clear separation rather than a photo finish. CV 7.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_madd_register is fastest but the noisiest (CV 7.2%)

carrier_res_madd_register wins on median (2.48 us) yet has the highest variance (CV 7.2%), while carrier_res_madd_stack is the steadiest (CV 6.8%, 5.58 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (2.48 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 2478.3 ns median
- 1 variant significantly slower than baseline
- Spread: 2.25x (fastest 2478.3 ns, slowest 5577.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 5022ns | 4809ns | 4756ns | 4801ns | 5486ns | base |
| carrier_res_madd_stack | 8091ns | 7831ns | 7742ns | 7804ns | 8696ns | +61.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 2580ns | 2460ns | 2800ns | base | 0.025 |
| carrier_res_madd_stack | 5759ns | 5510ns | 6187ns | +123.25% | 0.011 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.026 | 99.2% |
| carrier_res_madd_stack | 0.011 | 44.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 5022ns | 5022ns | base |
| carrier_res_madd_stack | 8091ns | 8091ns | +61.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 2478ns | base | --- | [2460, 2800] | --- | --- | --- | --- |
| carrier_res_madd_stack | 5578ns | +3111.9ns (+125.6%) | [+2956, +3470]ns | [5512, 6187] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 2949ns | +123.1% |
| 2 | 2472ns | +127.9% |
| 3 | 2652ns | +107.7% |
| 4 | 2460ns | +124.5% |
| 5 | 2460ns | +124.2% |
| 6 | 2485ns | +133.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | -0.161 | ok |
| carrier_res_madd_stack | 0.043 | ok |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 86787.9ns | 2579.5ns | 3364.5% | HIGH |
| carrier_res_madd_stack | 87462.0ns | 5758.8ns | 1518.8% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 2459.6-2800.4 ns)
   2459.6 |########################################
   2476.6 |#############
   2493.7 |
   2510.7 |
   2527.8 |
   2544.8 |
   2561.8 |
   2578.9 |
   2595.9 |
   2613.0 |
   2630.0 |
   2647.0 |#############
   2664.1 |
   2681.1 |
   2698.2 |
   2715.2 |
   2732.2 |
   2749.3 |
   2766.3 |
   2783.4 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 5509.6-6186.9 ns)
   5509.6 |########################################
   5543.5 |
   5577.3 |
   5611.2 |#############
   5645.1 |
   5678.9 |
   5712.8 |
   5746.7 |
   5780.5 |#############
   5814.4 |
   5848.2 |
   5882.1 |
   5916.0 |
   5949.8 |
   5983.7 |
   6017.6 |
   6051.4 |
   6085.3 |
   6119.2 |
   6153.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=3499.3% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=1572.0% of algo (FFI overhead may distort results)
