# Residual encoding: predecoded register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 62% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (1.58 ms) leads carrier_res_wideselect_stack (2.56 ms) by 62%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (1.58 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 1575280.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.62x (fastest 1575280.6 ns, slowest 2557024.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 1587629ns | 1578603ns | 1571290ns | 1576916ns | 1611868ns | base |
| carrier_res_wideselect_stack | 2559404ns | 2559687ns | 2549863ns | 2557222ns | 2567447ns | +61.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 1584066ns | 1567581ns | 1608242ns | base | 0.010 |
| carrier_res_wideselect_stack | 2556779ns | 2547365ns | 2564709ns | +61.41% | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_wideselect_register | 10056272 | 8851254 | 1.136 | 1.00× |
| carrier_res_wideselect_stack | 15994954 | 36918717 | 0.433 | 1.59× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.010 | 99.5% |
| carrier_res_wideselect_stack | 0.006 | 61.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 1587629ns | 1587629ns | base |
| carrier_res_wideselect_stack | 2559404ns | 2559404ns | +61.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 1575281ns | base | --- | [1568676, 1608242] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 2557025ns | +982298.8ns (+62.4%) | [+940360, +995479]ns | [2548602, 2564709] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 1569770ns | +63.8% |
| 2 | 1575473ns | +61.7% |
| 3 | 1641011ns | +55.4% |
| 4 | 1567581ns | +63.1% |
| 5 | 1575328ns | +62.4% |
| 6 | 1575233ns | +62.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | -0.275 | moderate- |
| carrier_res_wideselect_stack | -0.206 | moderate- |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 1648243.5ns | 1584066.0ns | 104.1% | HIGH |
| carrier_res_wideselect_stack | 2552888.8ns | 2556778.7ns | 99.8% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 1567581.2-1608241.9 ns)
  1567581.2 |#############
  1569614.2 |#############
  1571647.3 |
  1573680.3 |########################################
  1575713.3 |
  1577746.4 |
  1579779.4 |
  1581812.4 |
  1583845.5 |
  1585878.5 |
  1587911.5 |
  1589944.6 |
  1591977.6 |
  1594010.6 |
  1596043.7 |
  1598076.7 |
  1600109.7 |
  1602142.8 |
  1604175.8 |
  1606208.8 |
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 2547365.4-2564709.4 ns)
  2547365.4 |####################
  2548232.6 |
  2549099.8 |####################
  2549967.0 |
  2550834.2 |
  2551701.4 |
  2552568.6 |
  2553435.8 |
  2554303.0 |
  2555170.2 |
  2556037.4 |
  2556904.6 |########################################
  2557771.8 |####################
  2558639.0 |
  2559506.2 |
  2560373.4 |
  2561240.6 |
  2562107.8 |
  2562975.0 |
  2563842.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=103.9% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=99.7% of algo (FFI overhead may distort results)
