# Effect inclusion gate: thermometer subset test vs naive per-family compare

2 variants, 6 samples per variant.
Baseline: **eg_thermo**

## Highlights

Baseline for all deltas below: **eg_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### eg_thermo dominates: 3456% faster than the next best (eg_branchmax)

eg_thermo (457 ns) leads eg_branchmax (16.24 us) by 3456%, a clear separation rather than a photo finish. CV 6.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### eg_thermo is fastest but the noisiest (CV 6.1%)

eg_thermo wins on median (457 ns) yet has the highest variance (CV 6.1%), while eg_branchmax is the steadiest (CV 1.9%, 16.24 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (eg_thermo)

The baseline eg_thermo is the fastest (457 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 35.6x the fastest

Fastest eg_thermo (457 ns) to slowest eg_branchmax (16.24 us): 35.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (eg_thermo) is the fastest** at 456.7 ns median
- 1 variant significantly slower than baseline
- Spread: 35.56x (fastest 456.7 ns, slowest 16241.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| eg_branchmax | 18642ns | 18834ns | 18048ns | 18653ns | 18922ns | +541.35% |
| eg_thermo | 2907ns | 3025ns | 2488ns | 2963ns | 3032ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| eg_branchmax | 16071ns | 15510ns | 16326ns | +3547.28% | 0.016 |
| eg_thermo | 441ns | 382ns | 459ns | base | 0.581 |

## Performance model

- Peak throughput: **0.669 Gops/s** (eg_thermo; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| eg_branchmax | 0.016 | 2.4% |
| eg_thermo | 0.561 | 83.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| eg_branchmax | 18642ns | 18642ns | +541.35% |
| eg_thermo | 2907ns | 2907ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| eg_thermo | 457ns | base | --- | [406, 459] | --- | --- | --- | --- |
| eg_branchmax | 16242ns | +15784.4ns (+3456.2%) | [+15224, +15883]ns | [15645, 16326] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | eg_thermo | eg_branchmax |
|---|---|---|
| 1 | 382ns | +3954.9% |
| 2 | 430ns | +3703.6% |
| 3 | 460ns | +3330.6% |
| 4 | 457ns | +3471.6% |
| 5 | 457ns | +3455.8% |
| 6 | 458ns | +3444.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| eg_branchmax | -0.390 | moderate- |
| eg_thermo | 0.272 | moderate+ |

**Consistency summary:**

- **eg_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| eg_branchmax | 3.0ns | 16071.1ns | 0.0% |  |
| eg_thermo | 3.7ns | 440.6ns | 0.8% |  |

## Distribution (algo ns)

```
eg_branchmax (n=6, range 15510.0-16326.0 ns)
  15510.0 |########################################
  15550.8 |
  15591.6 |
  15632.4 |
  15673.2 |
  15714.0 |
  15754.8 |########################################
  15795.6 |
  15836.4 |
  15877.2 |
  15918.0 |
  15958.8 |
  15999.6 |
  16040.4 |
  16081.2 |
  16122.0 |
  16162.8 |
  16203.6 |########################################
  16244.4 |########################################
  16285.2 |########################################
  (0 below, 1 above range)

eg_thermo (n=6, range 382.5-459.1 ns)
    382.5 |#############
    386.3 |
    390.2 |
    394.0 |
    397.8 |
    401.7 |
    405.5 |
    409.3 |
    413.2 |
    417.0 |
    420.8 |
    424.7 |
    428.5 |#############
    432.3 |
    436.2 |
    440.0 |
    443.8 |
    447.7 |
    451.5 |
    455.3 |########################################
  (0 below, 1 above range)

```
