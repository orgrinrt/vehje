# Residual encoding: predecoded register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 207% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (502.47 us) leads carrier_res_tight_stack (1.54 ms) by 207%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (502.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_res_tight_register (502.47 us) to slowest carrier_res_tight_stack (1.54 ms): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 502471.0 ns median
- 1 variant significantly slower than baseline
- Spread: 3.07x (fastest 502471.0 ns, slowest 1543047.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 505872ns | 505773ns | 504391ns | 505668ns | 506919ns | base |
| carrier_res_tight_stack | 1546801ns | 1545810ns | 1534809ns | 1543273ns | 1558088ns | +205.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 502568ns | 501242ns | 503701ns | base | 0.033 |
| carrier_res_tight_stack | 1544193ns | 1532441ns | 1555379ns | +207.26% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_tight_register | 3216613 | 8846555 | 0.364 | 1.00× |
| carrier_res_tight_stack | 9684124 | 37852890 | 0.256 | 3.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.033 | 99.8% |
| carrier_res_tight_stack | 0.011 | 32.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 505872ns | 505872ns | base |
| carrier_res_tight_stack | 1546801ns | 1546801ns | +205.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 502471ns | base | --- | [501531, 503701] | --- | --- | --- | --- |
| carrier_res_tight_stack | 1543048ns | +1039346.5ns (+206.8%) | [+1032622, +1052908]ns | [1534153, 1555379] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 501820ns | +206.1% |
| 2 | 501242ns | +205.7% |
| 3 | 502791ns | +209.9% |
| 4 | 502151ns | +209.2% |
| 5 | 503798ns | +206.9% |
| 6 | 503605ns | +205.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | 0.266 | moderate+ |
| carrier_res_tight_stack | 0.127 | ok |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 519808.0ns | 502567.8ns | 103.4% | HIGH |
| carrier_res_tight_stack | 1556062.4ns | 1544193.5ns | 100.8% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 501242.5-503701.4 ns)
  501242.5 |########################################
  501365.4 |
  501488.4 |
  501611.3 |
  501734.3 |########################################
  501857.2 |
  501980.2 |
  502103.1 |########################################
  502226.1 |
  502349.0 |
  502472.0 |
  502594.9 |
  502717.9 |########################################
  502840.8 |
  502963.8 |
  503086.7 |
  503209.7 |
  503332.6 |
  503455.6 |
  503578.5 |########################################
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 1532441.2-1555379.1 ns)
  1532441.2 |########################################
  1533588.1 |
  1534735.0 |########################################
  1535881.9 |
  1537028.8 |
  1538175.7 |
  1539322.6 |########################################
  1540469.5 |
  1541616.4 |
  1542763.3 |
  1543910.2 |
  1545057.1 |
  1546204.0 |########################################
  1547350.9 |
  1548497.8 |
  1549644.7 |
  1550791.6 |
  1551938.5 |########################################
  1553085.4 |
  1554232.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=103.4% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=100.8% of algo (FFI overhead may distort results)
