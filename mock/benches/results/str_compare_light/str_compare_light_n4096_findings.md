# Runtime string interning, 1 compare per build: the templating norm

3 variants, 6 samples per variant.
Baseline: **str_l_plain**

## Highlights

Baseline for all deltas below: **str_l_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_l_plain dominates: 132% faster than the next best (str_l_eager)

str_l_plain (38.18 us) leads str_l_eager (88.59 us) by 132%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_l_lazy is an outlier: 2.4x slower than the field

str_l_lazy (91.18 us) is 2.4x the fastest (38.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (str_l_plain)

The baseline str_l_plain is the fastest (38.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (str_l_plain) is the fastest** at 38176.0 ns median
- 2 variants significantly slower than baseline
- Spread: 2.39x (fastest 38176.0 ns, slowest 91176.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_l_eager | 90943ns | 90941ns | 87933ns | 90603ns | 92958ns | +120.46% |
| str_l_lazy | 94730ns | 93556ns | 92574ns | 93392ns | 97815ns | +129.64% |
| str_l_plain | 41252ns | 40355ns | 40123ns | 40340ns | 43184ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_l_eager | 88563ns | 85597ns | 90577ns | +127.19% | 0.046 |
| str_l_lazy | 92331ns | 90282ns | 95299ns | +136.85% | 0.044 |
| str_l_plain | 38982ns | 37958ns | 40725ns | base | 0.105 |

## Performance model

- Peak throughput: **0.108 Gops/s** (str_l_plain; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_l_eager | 0.046 | 42.8% |
| str_l_lazy | 0.045 | 41.6% |
| str_l_plain | 0.107 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_l_eager | 90943ns | 90943ns | +120.46% |
| str_l_lazy | 94730ns | 94730ns | +129.64% |
| str_l_plain | 41252ns | 41252ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_l_plain | 38176ns | base | --- | [38046, 40725] | --- | --- | --- | --- |
| str_l_eager | 88587ns | +48929.0ns (+128.2%) | [+47474, +52339]ns | [86526, 90577] | YES | 0.0313 | 0.0313 | 0 |
| str_l_lazy | 91177ns | +53119.7ns (+139.1%) | [+50477, +56449]ns | [90517, 95299] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_l_plain | str_l_eager | str_l_lazy |
|---|---|---|---|
| 1 | 37958ns | +141.8% | +140.6% |
| 2 | 38196ns | +133.1% | +137.6% |
| 3 | 38135ns | +124.5% | +155.1% |
| 4 | 39565ns | +122.7% | +135.9% |
| 5 | 41884ns | +113.4% | +115.6% |
| 6 | 38156ns | +129.2% | +138.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_l_eager | 0.008 | ok |
| str_l_lazy | -0.020 | ok |
| str_l_plain | 0.023 | ok |

**Consistency summary:**

- **str_l_eager**: won 0/6, lost 6/6
- **str_l_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_l_eager | 3852.1ns | 88563.2ns | 4.3% |  |
| str_l_lazy | 3867.4ns | 92330.8ns | 4.2% |  |
| str_l_plain | 4216.4ns | 38982.4ns | 10.8% | HIGH |

## Distribution (algo ns)

```
str_l_eager (n=6, range 85596.7-90576.9 ns)
  85596.7 |########################################
  85845.7 |
  86094.7 |
  86343.7 |
  86592.7 |
  86841.8 |
  87090.8 |
  87339.8 |########################################
  87588.8 |
  87837.8 |
  88086.8 |########################################
  88335.8 |
  88584.8 |
  88833.8 |########################################
  89082.8 |
  89331.8 |########################################
  89580.9 |
  89829.9 |
  90078.9 |
  90327.9 |
  (0 below, 1 above range)

str_l_lazy (n=6, range 90281.7-95299.0 ns)
  90281.7 |########################################
  90532.6 |########################################
  90783.4 |########################################
  91034.3 |
  91285.1 |########################################
  91536.0 |
  91786.9 |
  92037.7 |
  92288.6 |
  92539.5 |
  92790.3 |
  93041.2 |
  93292.1 |########################################
  93542.9 |
  93793.8 |
  94044.6 |
  94295.5 |
  94546.4 |
  94797.2 |
  95048.1 |
  (0 below, 1 above range)

str_l_plain (n=6, range 37958.3-40724.8 ns)
  37958.3 |#############
  38096.6 |########################################
  38235.0 |
  38373.3 |
  38511.6 |
  38649.9 |
  38788.2 |
  38926.6 |
  39064.9 |
  39203.2 |
  39341.6 |
  39479.9 |#############
  39618.2 |
  39756.5 |
  39894.9 |
  40033.2 |
  40171.5 |
  40309.8 |
  40448.2 |
  40586.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **str_l_plain**: bridge=11.0% of algo (FFI overhead may distort results)
