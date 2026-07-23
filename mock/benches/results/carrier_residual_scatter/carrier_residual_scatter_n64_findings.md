# Residual encoding: predecoded register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_scatter_register dominates: 215% faster than the next best (carrier_res_scatter_stack)

carrier_res_scatter_register (1.89 us) leads carrier_res_scatter_stack (5.96 us) by 215%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_scatter_register is fastest but the noisiest (CV 6.2%)

carrier_res_scatter_register wins on median (1.89 us) yet has the highest variance (CV 6.2%), while carrier_res_scatter_stack is the steadiest (CV 2.7%, 5.96 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (1.89 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_res_scatter_register (1.89 us) to slowest carrier_res_scatter_stack (5.96 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 1892.7 ns median
- 1 variant significantly slower than baseline
- Spread: 3.15x (fastest 1892.7 ns, slowest 5957.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 4462ns | 4395ns | 4121ns | 4347ns | 4805ns | base |
| carrier_res_scatter_stack | 8343ns | 8396ns | 7949ns | 8326ns | 8566ns | +86.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 1925ns | 1754ns | 2061ns | base | 0.033 |
| carrier_res_scatter_stack | 5893ns | 5599ns | 6033ns | +206.09% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_scatter_register | 250818 | 877377 | 0.286 | 1.00× |
| carrier_res_scatter_stack | 231955 | 878236 | 0.264 | 0.92× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.034 | 92.7% |
| carrier_res_scatter_stack | 0.011 | 29.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 4462ns | 4462ns | base |
| carrier_res_scatter_stack | 8343ns | 8343ns | +86.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 1893ns | base | --- | [1821, 2061] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 5957ns | +3966.1ns (+209.5%) | [+3795, +4141]ns | [5687, 6033] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 2143ns | +183.4% |
| 2 | 1980ns | +202.1% |
| 3 | 1889ns | +196.5% |
| 4 | 1754ns | +238.3% |
| 5 | 1890ns | +217.1% |
| 6 | 1896ns | +204.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | 0.280 | moderate+ |
| carrier_res_scatter_stack | -0.195 | ok |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 85027.4ns | 1925.1ns | 4416.7% | HIGH |
| carrier_res_scatter_stack | 73139.4ns | 5892.6ns | 1241.2% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 1753.8-2061.4 ns)
   1753.8 |####################
   1769.2 |
   1784.6 |
   1799.9 |
   1815.3 |
   1830.7 |
   1846.1 |
   1861.5 |
   1876.9 |########################################
   1892.2 |####################
   1907.6 |
   1923.0 |
   1938.4 |
   1953.8 |
   1969.2 |####################
   1984.5 |
   1999.9 |
   2015.3 |
   2030.7 |
   2046.1 |
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 5599.2-6033.1 ns)
   5599.2 |########################################
   5620.9 |
   5642.6 |
   5664.3 |
   5686.0 |
   5707.7 |
   5729.4 |
   5751.1 |
   5772.8 |########################################
   5794.5 |
   5816.2 |
   5837.9 |
   5859.6 |
   5881.3 |
   5903.0 |
   5924.7 |########################################
   5946.4 |
   5968.1 |########################################
   5989.8 |########################################
   6011.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: bridge=4480.7% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=1227.5% of algo (FFI overhead may distort results)
