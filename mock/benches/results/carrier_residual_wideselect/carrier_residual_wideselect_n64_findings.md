# Residual encoding: predecoded register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 216% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (1.89 us) leads carrier_res_wideselect_stack (5.97 us) by 216%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (1.89 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.2x the fastest

Fastest carrier_res_wideselect_register (1.89 us) to slowest carrier_res_wideselect_stack (5.97 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 1886.8 ns median
- 1 variant significantly slower than baseline
- Spread: 3.16x (fastest 1886.8 ns, slowest 5971.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 4329ns | 4350ns | 3988ns | 4307ns | 4532ns | base |
| carrier_res_wideselect_stack | 8300ns | 8355ns | 7962ns | 8269ns | 8516ns | +91.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 1884ns | 1731ns | 1990ns | base | 0.034 |
| carrier_res_wideselect_stack | 5917ns | 5663ns | 6080ns | +214.02% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_wideselect_register | 272845 | 923622 | 0.295 | 1.00× |
| carrier_res_wideselect_stack | 277753 | 1101353 | 0.252 | 1.02× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.034 | 91.8% |
| carrier_res_wideselect_stack | 0.011 | 29.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 4329ns | 4329ns | base |
| carrier_res_wideselect_stack | 8300ns | 8300ns | +91.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 1887ns | base | --- | [1776, 1990] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 5971ns | +3987.5ns (+211.3%) | [+3838, +4273]ns | [5700, 6080] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 1870ns | +227.5% |
| 2 | 2010ns | +200.2% |
| 3 | 1903ns | +197.5% |
| 4 | 1731ns | +247.8% |
| 5 | 1820ns | +215.2% |
| 6 | 1971ns | +200.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | 0.039 | ok |
| carrier_res_wideselect_stack | -0.313 | moderate- |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 89808.3ns | 1884.3ns | 4766.2% | HIGH |
| carrier_res_wideselect_stack | 86455.1ns | 5916.9ns | 1461.1% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 1731.2-1990.4 ns)
   1731.2 |########################################
   1744.2 |
   1757.1 |
   1770.1 |
   1783.0 |
   1796.0 |
   1809.0 |########################################
   1821.9 |
   1834.9 |
   1847.8 |
   1860.8 |########################################
   1873.8 |
   1886.7 |
   1899.7 |########################################
   1912.6 |
   1925.6 |
   1938.6 |
   1951.5 |
   1964.5 |########################################
   1977.4 |
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 5662.9-6079.8 ns)
   5662.9 |####################
   5683.7 |
   5704.6 |
   5725.4 |####################
   5746.3 |
   5767.1 |
   5788.0 |
   5808.8 |
   5829.7 |
   5850.5 |
   5871.4 |
   5892.2 |
   5913.0 |####################
   5933.9 |
   5954.7 |
   5975.6 |
   5996.4 |
   6017.3 |########################################
   6038.1 |
   6059.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=4755.4% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=1449.6% of algo (FFI overhead may distort results)
