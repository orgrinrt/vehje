# Interp output-building: format-to-temp+copy vs in-place vs span-list

3 variants, 6 samples per variant.
Baseline: **interp_out_inplace**

## Highlights

Baseline for all deltas below: **interp_out_inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### interp_out_temp shows alternating (throttle bounce) (autocorr -0.55)

interp_out_temp's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (interp_out_inplace)

The baseline interp_out_inplace is the fastest (808.40 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (interp_out_inplace) is the fastest** at 808402.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.25x (fastest 808402.5 ns, slowest 1006823.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| interp_out_inplace | 814855ns | 811538ns | 809974ns | 811181ns | 822806ns | base |
| interp_out_spanlist | 859432ns | 861124ns | 846311ns | 858316ns | 867665ns | +5.47% |
| interp_out_temp | 1013164ns | 1010246ns | 1003882ns | 1008438ns | 1024893ns | +24.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| interp_out_inplace | 811989ns | 807193ns | 820332ns | base | 0.005 |
| interp_out_spanlist | 856701ns | 842926ns | 865186ns | +5.51% | 0.005 |
| interp_out_temp | 1009846ns | 1000524ns | 1021680ns | +24.37% | 0.004 |

## Performance model

- Peak throughput: **0.005 Gops/s** (interp_out_inplace; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| interp_out_inplace | 0.005 | 99.9% |
| interp_out_spanlist | 0.005 | 94.0% |
| interp_out_temp | 0.004 | 80.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| interp_out_inplace | 814855ns | 814855ns | base |
| interp_out_spanlist | 859432ns | 859432ns | +5.47% |
| interp_out_temp | 1013164ns | 1013164ns | +24.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| interp_out_inplace | 808402ns | base | --- | [807233, 820332] | --- | --- | --- | --- |
| interp_out_spanlist | 858737ns | +45850.4ns (+5.7%) | [+33567, +54718]ns | [846179, 865186] | YES | 0.0313 | 0.0313 | 0 |
| interp_out_temp | 1006823ns | +194091.9ns (+24.0%) | [+186630, +212850]ns | [1001036, 1021680] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | interp_out_inplace | interp_out_spanlist | interp_out_temp |
|---|---|---|---|
| 1 | 807738ns | +7.3% | +24.2% |
| 2 | 807193ns | +6.1% | +27.6% |
| 3 | 809067ns | +4.2% | +23.8% |
| 4 | 807274ns | +5.2% | +25.1% |
| 5 | 813199ns | +6.2% | +23.0% |
| 6 | 827464ns | +4.0% | +22.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| interp_out_inplace | 0.196 | ok |
| interp_out_spanlist | 0.190 | ok |
| interp_out_temp | -0.546 | HIGH- (thermal bounce) |

**Consistency summary:**

- **interp_out_spanlist**: won 0/6, lost 6/6
- **interp_out_temp**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| interp_out_inplace | 7.6ns | 811989.1ns | 0.0% |  |
| interp_out_spanlist | 20.1ns | 856700.8ns | 0.0% |  |
| interp_out_temp | 14.8ns | 1009846.4ns | 0.0% |  |

## Distribution (algo ns)

```
interp_out_inplace (n=6, range 807192.9-820331.5 ns)
  807192.9 |########################################
  807849.8 |
  808506.8 |#############
  809163.7 |
  809820.6 |
  810477.6 |
  811134.5 |
  811791.4 |
  812448.3 |
  813105.3 |#############
  813762.2 |
  814419.1 |
  815076.1 |
  815733.0 |
  816389.9 |
  817046.8 |
  817703.8 |
  818360.7 |
  819017.6 |
  819674.6 |
  (0 below, 1 above range)

interp_out_spanlist (n=6, range 842926.2-865186.2 ns)
  842926.2 |########################################
  844039.2 |
  845152.2 |
  846265.2 |
  847378.2 |
  848491.2 |########################################
  849604.2 |
  850717.2 |
  851830.2 |
  852943.2 |
  854056.2 |
  855169.2 |
  856282.2 |########################################
  857395.2 |
  858508.2 |
  859621.2 |
  860734.2 |########################################
  861847.2 |
  862960.2 |########################################
  864073.2 |
  (0 below, 1 above range)

interp_out_temp (n=6, range 1000523.7-1021680.4 ns)
  1000523.7 |########################################
  1001581.5 |
  1002639.4 |####################
  1003697.2 |
  1004755.0 |
  1005812.9 |
  1006870.7 |
  1007928.5 |
  1008986.4 |
  1010044.2 |####################
  1011102.0 |
  1012159.9 |
  1013217.7 |####################
  1014275.6 |
  1015333.4 |
  1016391.2 |
  1017449.1 |
  1018506.9 |
  1019564.7 |
  1020622.6 |
  (0 below, 1 above range)

```
