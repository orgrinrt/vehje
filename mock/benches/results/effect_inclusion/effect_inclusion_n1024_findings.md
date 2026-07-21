# Effect inclusion gate: thermometer subset test vs naive per-family compare

2 variants, 6 samples per variant.
Baseline: **eg_thermo**

## Highlights

Baseline for all deltas below: **eg_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### eg_thermo dominates: 4034% faster than the next best (eg_branchmax)

eg_thermo (1.48 us) leads eg_branchmax (61.15 us) by 4034%, a clear separation rather than a photo finish. CV 8.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### eg_thermo is fastest but the noisiest (CV 8.4%)

eg_thermo wins on median (1.48 us) yet has the highest variance (CV 8.4%), while eg_branchmax is the steadiest (CV 6.7%, 61.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (eg_thermo)

The baseline eg_thermo is the fastest (1.48 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 41.3x the fastest

Fastest eg_thermo (1.48 us) to slowest eg_branchmax (61.15 us): 41.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (eg_thermo) is the fastest** at 1479.2 ns median
- 1 variant significantly slower than baseline
- Spread: 41.34x (fastest 1479.2 ns, slowest 61145.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| eg_branchmax | 63816ns | 63574ns | 58505ns | 62411ns | 68579ns | +1552.82% |
| eg_thermo | 3861ns | 3660ns | 3616ns | 3647ns | 4305ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| eg_branchmax | 61404ns | 56239ns | 66039ns | +3853.56% | 0.017 |
| eg_thermo | 1553ns | 1453ns | 1727ns | base | 0.659 |

## Performance model

- Peak throughput: **0.705 Gops/s** (eg_thermo; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| eg_branchmax | 0.017 | 2.4% |
| eg_thermo | 0.692 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| eg_branchmax | 63816ns | 63816ns | +1552.82% |
| eg_thermo | 3861ns | 3861ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| eg_thermo | 1479ns | base | --- | [1453, 1727] | --- | --- | --- | --- |
| eg_branchmax | 61145ns | +59692.3ns (+4035.4%) | [+55549, +64312]ns | [57028, 66039] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | eg_thermo | eg_branchmax |
|---|---|---|
| 1 | 1453ns | +3907.8% |
| 2 | 1725ns | +3750.0% |
| 3 | 1729ns | +3697.4% |
| 4 | 1453ns | +4308.1% |
| 5 | 1495ns | +3662.8% |
| 6 | 1464ns | +3849.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| eg_branchmax | 0.217 | moderate+ |
| eg_thermo | 0.071 | ok |

**Consistency summary:**

- **eg_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| eg_branchmax | 4.2ns | 61404.0ns | 0.0% |  |
| eg_thermo | 3.3ns | 1553.1ns | 0.2% |  |

## Distribution (algo ns)

```
eg_branchmax (n=6, range 56239.2-66038.8 ns)
  56239.2 |########################################
  56729.2 |
  57219.2 |
  57709.1 |########################################
  58199.1 |########################################
  58689.1 |
  59179.1 |
  59669.0 |
  60159.0 |
  60649.0 |
  61139.0 |
  61629.0 |
  62118.9 |
  62608.9 |
  63098.9 |
  63588.9 |########################################
  64078.8 |
  64568.8 |
  65058.8 |
  65548.8 |########################################
  (0 below, 1 above range)

eg_thermo (n=6, range 1452.9-1727.1 ns)
   1452.9 |########################################
   1466.6 |
   1480.3 |
   1494.0 |#############
   1507.7 |
   1521.5 |
   1535.2 |
   1548.9 |
   1562.6 |
   1576.3 |
   1590.0 |
   1603.7 |
   1617.4 |
   1631.1 |
   1644.8 |
   1658.5 |
   1672.3 |
   1686.0 |
   1699.7 |
   1713.4 |#############
  (0 below, 1 above range)

```
