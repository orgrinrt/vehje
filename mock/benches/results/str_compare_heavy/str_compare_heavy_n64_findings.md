# Runtime string interning, 8 compares per build: interning's best case

3 variants, 6 samples per variant.
Baseline: **str_h_plain**

## Highlights

Baseline for all deltas below: **str_h_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_h_plain dominates: 281% faster than the next best (str_h_eager)

str_h_plain (471 ns) leads str_h_eager (1.80 us) by 281%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_h_lazy is an outlier: 4.6x slower than the field

str_h_lazy (2.16 us) is 4.6x the fastest (471 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (str_h_plain)

The baseline str_h_plain is the fastest (471 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.6x the fastest

Fastest str_h_plain (471 ns) to slowest str_h_lazy (2.16 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_h_plain) is the fastest** at 471.5 ns median
- 2 variants significantly slower than baseline
- Spread: 4.59x (fastest 471.5 ns, slowest 2163.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_h_eager | 4107ns | 4146ns | 3761ns | 4039ns | 4383ns | +45.19% |
| str_h_lazy | 4508ns | 4498ns | 4268ns | 4460ns | 4699ns | +59.36% |
| str_h_plain | 2829ns | 2841ns | 2568ns | 2792ns | 3014ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_h_eager | 1779ns | 1608ns | 1923ns | +284.47% | 0.036 |
| str_h_lazy | 2172ns | 2025ns | 2279ns | +369.41% | 0.029 |
| str_h_plain | 463ns | 418ns | 491ns | base | 0.138 |

## Performance model

- Peak throughput: **0.153 Gops/s** (str_h_plain; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_h_eager | 0.036 | 23.2% |
| str_h_lazy | 0.030 | 19.3% |
| str_h_plain | 0.136 | 88.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_h_eager | 4107ns | 4107ns | +45.19% |
| str_h_lazy | 4508ns | 4508ns | +59.36% |
| str_h_plain | 2829ns | 2829ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_h_plain | 471ns | base | --- | [425, 491] | --- | --- | --- | --- |
| str_h_eager | 1799ns | +1309.2ns (+277.7%) | [+1179, +1460]ns | [1615, 1923] | YES | 0.0313 | 0.0313 | 0 |
| str_h_lazy | 2163ns | +1714.4ns (+363.6%) | [+1624, +1789]ns | [2073, 2279] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_h_plain | str_h_eager | str_h_lazy |
|---|---|---|---|
| 1 | 455ns | +253.6% | +366.4% |
| 2 | 418ns | +288.5% | +385.1% |
| 3 | 488ns | +273.6% | +364.7% |
| 4 | 433ns | +331.0% | +407.2% |
| 5 | 492ns | +302.3% | +365.1% |
| 6 | 490ns | +261.5% | +334.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_h_eager | 0.385 | moderate+ |
| str_h_lazy | -0.130 | ok |
| str_h_plain | -0.306 | moderate- |

**Consistency summary:**

- **str_h_eager**: won 0/6, lost 6/6
- **str_h_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_h_eager | 3780.5ns | 1778.7ns | 212.5% | HIGH |
| str_h_lazy | 3955.5ns | 2171.7ns | 182.1% | HIGH |
| str_h_plain | 4147.1ns | 462.6ns | 896.4% | HIGH |

## Distribution (algo ns)

```
str_h_eager (n=6, range 1607.5-1922.7 ns)
   1607.5 |########################################
   1623.3 |
   1639.0 |
   1654.8 |
   1670.5 |
   1686.3 |
   1702.1 |
   1717.8 |
   1733.6 |
   1749.3 |
   1765.1 |####################
   1780.9 |
   1796.6 |
   1812.4 |####################
   1828.1 |
   1843.9 |
   1859.7 |####################
   1875.4 |
   1891.2 |
   1906.9 |
  (0 below, 1 above range)

str_h_lazy (n=6, range 2025.4-2279.0 ns)
   2025.4 |########################################
   2038.1 |
   2050.8 |
   2063.4 |
   2076.1 |
   2088.8 |
   2101.5 |
   2114.2 |########################################
   2126.8 |########################################
   2139.5 |
   2152.2 |
   2164.9 |
   2177.6 |
   2190.2 |########################################
   2202.9 |
   2215.6 |
   2228.3 |
   2241.0 |
   2253.6 |
   2266.3 |########################################
  (0 below, 1 above range)

str_h_plain (n=6, range 417.5-491.2 ns)
    417.5 |####################
    421.2 |
    424.9 |
    428.6 |
    432.2 |####################
    435.9 |
    439.6 |
    443.3 |
    447.0 |
    450.7 |
    454.4 |####################
    458.1 |
    461.8 |
    465.4 |
    469.1 |
    472.8 |
    476.5 |
    480.2 |
    483.9 |
    487.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **str_h_eager**: bridge=203.0% of algo (FFI overhead may distort results)
- **str_h_lazy**: bridge=183.6% of algo (FFI overhead may distort results)
- **str_h_plain**: bridge=879.3% of algo (FFI overhead may distort results)
