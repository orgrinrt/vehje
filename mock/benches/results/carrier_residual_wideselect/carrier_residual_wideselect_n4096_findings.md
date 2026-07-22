# Residual encoding: register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 23% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (463.59 us) leads carrier_res_wideselect_stack (572.22 us) by 23%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (463.59 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 463586.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 463586.5 ns, slowest 572223.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 468872ns | 466482ns | 448905ns | 461619ns | 489735ns | base |
| carrier_res_wideselect_stack | 573751ns | 574839ns | 558461ns | 573338ns | 582015ns | +22.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 466200ns | 446693ns | 486873ns | base | 0.009 |
| carrier_res_wideselect_stack | 571100ns | 555800ns | 579148ns | +22.50% | 0.007 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.009 | 96.4% |
| carrier_res_wideselect_stack | 0.007 | 78.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 468872ns | 468872ns | base |
| carrier_res_wideselect_stack | 573751ns | 573751ns | +22.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 463586ns | base | --- | [448141, 486873] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 572224ns | +101696.3ns (+21.9%) | [+81995, +131007]ns | [561927, 579148] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 489493ns | +16.2% |
| 2 | 470987ns | +18.0% |
| 3 | 446693ns | +29.2% |
| 4 | 456186ns | +24.5% |
| 5 | 449589ns | +29.2% |
| 6 | 484253ns | +18.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | 0.048 | ok |
| carrier_res_wideselect_stack | -0.146 | ok |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 466355.1ns | 466200.3ns | 100.0% | HIGH |
| carrier_res_wideselect_stack | 568704.6ns | 571099.9ns | 99.6% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 446692.9-486873.3 ns)
  446692.9 |########################################
  448701.9 |########################################
  450710.9 |
  452720.0 |
  454729.0 |########################################
  456738.0 |
  458747.0 |
  460756.0 |
  462765.1 |
  464774.1 |
  466783.1 |
  468792.1 |
  470801.1 |########################################
  472810.2 |
  474819.2 |
  476828.2 |
  478837.2 |
  480846.2 |
  482855.3 |########################################
  484864.3 |
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 555800.4-579148.3 ns)
  555800.4 |########################################
  556967.8 |
  558135.2 |
  559302.6 |
  560470.0 |
  561637.4 |
  562804.8 |
  563972.2 |
  565139.6 |
  566307.0 |
  567474.4 |########################################
  568641.8 |########################################
  569809.2 |
  570976.6 |
  572144.0 |
  573311.4 |
  574478.8 |
  575646.2 |########################################
  576813.6 |########################################
  577981.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=99.7% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=99.5% of algo (FFI overhead may distort results)
