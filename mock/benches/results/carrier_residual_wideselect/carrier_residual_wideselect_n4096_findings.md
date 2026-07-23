# Residual encoding: predecoded register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 68% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (345.21 us) leads carrier_res_wideselect_stack (578.74 us) by 68%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_wideselect_register is fastest but the noisiest (CV 5.2%)

carrier_res_wideselect_register wins on median (345.21 us) yet has the highest variance (CV 5.2%), while carrier_res_wideselect_stack is the steadiest (CV 0.4%, 578.74 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (345.21 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 345208.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.68x (fastest 345208.3 ns, slowest 578736.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 342427ns | 348082ns | 312659ns | 340699ns | 359902ns | base |
| carrier_res_wideselect_stack | 582821ns | 581950ns | 580346ns | 581481ns | 586069ns | +70.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 339632ns | 309828ns | 357232ns | base | 0.012 |
| carrier_res_wideselect_stack | 579658ns | 577024ns | 582922ns | +70.67% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_wideselect_register | 2180668 | 2226671 | 0.979 | 1.00× |
| carrier_res_wideselect_stack | 3620370 | 9228198 | 0.392 | 1.66× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.013 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.012 | 89.8% |
| carrier_res_wideselect_stack | 0.007 | 53.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 342427ns | 342427ns | base |
| carrier_res_wideselect_stack | 582821ns | 582821ns | +70.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 345208ns | base | --- | [316457, 357232] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 578736ns | +234742.5ns (+68.0%) | [+220083, +265250]ns | [577315, 582922] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 349086ns | +65.6% |
| 2 | 341331ns | +70.5% |
| 3 | 309828ns | +88.4% |
| 4 | 323086ns | +79.4% |
| 5 | 352096ns | +64.0% |
| 6 | 362368ns | +59.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | 0.278 | moderate+ |
| carrier_res_wideselect_stack | 0.293 | moderate+ |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 352712.3ns | 339632.3ns | 103.9% | HIGH |
| carrier_res_wideselect_stack | 572352.4ns | 579657.5ns | 98.7% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 309827.9-357231.7 ns)
  309827.9 |########################################
  312198.1 |
  314568.3 |
  316938.5 |
  319308.7 |
  321678.8 |########################################
  324049.0 |
  326419.2 |
  328789.4 |
  331159.6 |
  333529.8 |
  335900.0 |
  338270.2 |
  340640.3 |########################################
  343010.5 |
  345380.7 |
  347750.9 |########################################
  350121.1 |########################################
  352491.3 |
  354861.5 |
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 577024.2-582921.5 ns)
  577024.2 |########################################
  577319.1 |########################################
  577613.9 |
  577908.8 |########################################
  578203.7 |
  578498.5 |
  578793.4 |
  579088.3 |
  579383.1 |########################################
  579678.0 |
  579972.8 |
  580267.7 |
  580562.6 |
  580857.4 |
  581152.3 |
  581447.2 |
  581742.0 |########################################
  582036.9 |
  582331.8 |
  582626.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=101.7% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=99.1% of algo (FFI overhead may distort results)
