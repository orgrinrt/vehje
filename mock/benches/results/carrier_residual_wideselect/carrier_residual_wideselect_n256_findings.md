# Residual encoding: predecoded register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 273% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (6.54 us) leads carrier_res_wideselect_stack (24.42 us) by 273%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (6.54 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.7x the fastest

Fastest carrier_res_wideselect_register (6.54 us) to slowest carrier_res_wideselect_stack (24.42 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 6540.5 ns median
- 1 variant significantly slower than baseline
- Spread: 3.73x (fastest 6540.5 ns, slowest 24421.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 8823ns | 8879ns | 8460ns | 8752ns | 9111ns | base |
| carrier_res_wideselect_stack | 26787ns | 26830ns | 25683ns | 26577ns | 27653ns | +203.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 6507ns | 6231ns | 6710ns | base | 0.039 |
| carrier_res_wideselect_stack | 24378ns | 23388ns | 25163ns | +274.67% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_wideselect_register | 294377 | 1042614 | 0.282 | 1.00× |
| carrier_res_wideselect_stack | 371898 | 1461129 | 0.255 | 1.26× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.041 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.039 | 95.3% |
| carrier_res_wideselect_stack | 0.010 | 25.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 8823ns | 8823ns | base |
| carrier_res_wideselect_stack | 26787ns | 26787ns | +203.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 6540ns | base | --- | [6270, 6710] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 24422ns | +17932.6ns (+274.2%) | [+17010, +18673]ns | [23550, 25163] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 6309ns | +301.1% |
| 2 | 6539ns | +262.6% |
| 3 | 6542ns | +257.5% |
| 4 | 6813ns | +267.3% |
| 5 | 6231ns | +283.3% |
| 6 | 6606ns | +277.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | -0.480 | moderate- |
| carrier_res_wideselect_stack | -0.366 | moderate- |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 87724.2ns | 6506.7ns | 1348.2% | HIGH |
| carrier_res_wideselect_stack | 96923.3ns | 24378.3ns | 397.6% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 6231.2-6709.6 ns)
   6231.2 |####################
   6255.1 |
   6279.0 |
   6303.0 |####################
   6326.9 |
   6350.8 |
   6374.7 |
   6398.6 |
   6422.5 |
   6446.5 |
   6470.4 |
   6494.3 |
   6518.2 |########################################
   6542.1 |
   6566.0 |
   6590.0 |####################
   6613.9 |
   6637.8 |
   6661.7 |
   6685.6 |
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 23388.3-25162.9 ns)
  23388.3 |########################################
  23477.0 |
  23565.8 |
  23654.5 |########################################
  23743.2 |
  23832.0 |########################################
  23920.7 |
  24009.4 |
  24098.1 |
  24186.9 |
  24275.6 |
  24364.3 |
  24453.1 |
  24541.8 |
  24630.5 |
  24719.2 |
  24808.0 |
  24896.7 |########################################
  24985.4 |########################################
  25074.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=1337.7% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=396.3% of algo (FFI overhead may distort results)
