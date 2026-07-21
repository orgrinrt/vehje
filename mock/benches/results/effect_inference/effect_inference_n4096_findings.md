# Effect inference: thermometer OR-join vs naive branch-max lattice join

2 variants, 6 samples per variant.
Baseline: **ei_thermo**

## Highlights

Baseline for all deltas below: **ei_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ei_thermo dominates: 360% faster than the next best (ei_branchmax)

ei_thermo (77.91 us) leads ei_branchmax (358.51 us) by 360%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (ei_thermo)

The baseline ei_thermo is the fastest (77.91 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.6x the fastest

Fastest ei_thermo (77.91 us) to slowest ei_branchmax (358.51 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ei_thermo) is the fastest** at 77911.0 ns median
- 1 variant significantly slower than baseline
- Spread: 4.60x (fastest 77911.0 ns, slowest 358513.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ei_branchmax | 360659ns | 361063ns | 350665ns | 359675ns | 367131ns | +348.24% |
| ei_thermo | 80461ns | 80285ns | 75654ns | 78752ns | 85430ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ei_branchmax | 358109ns | 348001ns | 364472ns | +358.74% | 0.011 |
| ei_thermo | 78063ns | 73432ns | 82842ns | base | 0.052 |

## Performance model

- Peak throughput: **0.056 Gops/s** (ei_thermo; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ei_branchmax | 0.011 | 20.5% |
| ei_thermo | 0.053 | 94.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ei_branchmax | 360659ns | 360659ns | +348.24% |
| ei_thermo | 80461ns | 80461ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ei_thermo | 77911ns | base | --- | [73436, 82842] | --- | --- | --- | --- |
| ei_branchmax | 358514ns | +280265.0ns (+359.7%) | [+273365, +286509]ns | [351342, 364472] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ei_thermo | ei_branchmax |
|---|---|---|
| 1 | 73440ns | +388.4% |
| 2 | 73432ns | +383.0% |
| 3 | 78038ns | +368.7% |
| 4 | 81804ns | +338.0% |
| 5 | 83880ns | +332.9% |
| 6 | 77784ns | +347.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ei_branchmax | -0.386 | moderate- |
| ei_thermo | 0.458 | moderate+ |

**Consistency summary:**

- **ei_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ei_branchmax | 454.6ns | 358109.4ns | 0.1% |  |
| ei_thermo | 435.9ns | 78063.0ns | 0.6% |  |

## Distribution (algo ns)

```
ei_branchmax (n=6, range 348001.2-364472.3 ns)
  348001.2 |########################################
  348824.8 |
  349648.3 |
  350471.9 |
  351295.4 |
  352119.0 |
  352942.5 |
  353766.1 |
  354589.6 |########################################
  355413.2 |
  356236.8 |
  357060.3 |
  357883.9 |########################################
  358707.4 |########################################
  359531.0 |
  360354.5 |
  361178.1 |
  362001.6 |
  362825.2 |########################################
  363648.7 |
  (0 below, 1 above range)

ei_thermo (n=6, range 73431.7-82842.1 ns)
  73431.7 |########################################
  73902.2 |
  74372.7 |
  74843.3 |
  75313.8 |
  75784.3 |
  76254.8 |
  76725.3 |
  77195.9 |
  77666.4 |########################################
  78136.9 |
  78607.4 |
  79077.9 |
  79548.5 |
  80019.0 |
  80489.5 |
  80960.0 |
  81430.5 |####################
  81901.1 |
  82371.6 |
  (0 below, 1 above range)

```
