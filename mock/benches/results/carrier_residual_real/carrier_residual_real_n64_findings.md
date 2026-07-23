# Residual encoding: predecoded register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_real_register dominates: 208% faster than the next best (carrier_res_real_stack)

carrier_res_real_register (1.85 us) leads carrier_res_real_stack (5.71 us) by 208%, a clear separation rather than a photo finish. CV 6.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_real_register is fastest but the noisiest (CV 6.0%)

carrier_res_real_register wins on median (1.85 us) yet has the highest variance (CV 6.0%), while carrier_res_real_stack is the steadiest (CV 0.7%, 5.71 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_res_real_stack shows alternating (throttle bounce) (autocorr -0.53)

carrier_res_real_stack's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (1.85 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_res_real_register (1.85 us) to slowest carrier_res_real_stack (5.71 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 1851.0 ns median
- 1 variant significantly slower than baseline
- Spread: 3.08x (fastest 1851.0 ns, slowest 5708.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 4327ns | 4325ns | 4012ns | 4241ns | 4613ns | base |
| carrier_res_real_stack | 8223ns | 8222ns | 8114ns | 8194ns | 8321ns | +90.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 1841ns | 1710ns | 1960ns | base | 0.035 |
| carrier_res_real_stack | 5719ns | 5677ns | 5768ns | +210.55% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_real_register | 255060 | 905086 | 0.282 | 1.00× |
| carrier_res_real_stack | 268177 | 1051990 | 0.255 | 1.05× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.035 | 92.4% |
| carrier_res_real_stack | 0.011 | 30.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 4327ns | 4327ns | base |
| carrier_res_real_stack | 8223ns | 8223ns | +90.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 1851ns | base | --- | [1713, 1960] | --- | --- | --- | --- |
| carrier_res_real_stack | 5709ns | +3851.1ns (+208.0%) | [+3763, +4017]ns | [5680, 5768] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 2035ns | +182.6% |
| 2 | 1827ns | +211.0% |
| 3 | 1885ns | +202.1% |
| 4 | 1716ns | +237.0% |
| 5 | 1710ns | +232.0% |
| 6 | 1875ns | +205.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | 0.043 | ok |
| carrier_res_real_stack | -0.528 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_real_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 85076.4ns | 1841.4ns | 4620.1% | HIGH |
| carrier_res_real_stack | 86355.6ns | 5718.6ns | 1510.1% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 1710.0-1960.2 ns)
   1710.0 |########################################
   1722.5 |
   1735.0 |
   1747.5 |
   1760.0 |
   1772.5 |
   1785.1 |
   1797.6 |
   1810.1 |
   1822.6 |####################
   1835.1 |
   1847.6 |
   1860.1 |
   1872.6 |####################
   1885.1 |####################
   1897.7 |
   1910.2 |
   1922.7 |
   1935.2 |
   1947.7 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 5676.7-5767.5 ns)
   5676.7 |########################################
   5681.2 |########################################
   5685.8 |
   5690.3 |
   5694.9 |########################################
   5699.4 |
   5703.9 |
   5708.5 |
   5713.0 |
   5717.6 |########################################
   5722.1 |
   5726.6 |
   5731.2 |
   5735.7 |
   5740.3 |
   5744.8 |
   5749.3 |########################################
   5753.9 |
   5758.4 |
   5763.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=4592.1% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=1511.3% of algo (FFI overhead may distort results)
