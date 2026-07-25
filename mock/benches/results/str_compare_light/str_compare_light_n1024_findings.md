# Runtime string interning, 1 compare per build: the templating norm

3 variants, 6 samples per variant.
Baseline: **str_l_plain**

## Highlights

Baseline for all deltas below: **str_l_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_l_plain dominates: 142% faster than the next best (str_l_eager)

str_l_plain (9.64 us) leads str_l_eager (23.31 us) by 142%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_l_lazy is an outlier: 2.4x slower than the field

str_l_lazy (23.57 us) is 2.4x the fastest (9.64 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (str_l_plain)

The baseline str_l_plain is the fastest (9.64 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (str_l_plain) is the fastest** at 9637.5 ns median
- 2 variants significantly slower than baseline
- Spread: 2.45x (fastest 9637.5 ns, slowest 23573.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_l_eager | 25767ns | 25588ns | 24763ns | 25450ns | 26744ns | +115.79% |
| str_l_lazy | 25849ns | 25862ns | 25094ns | 25683ns | 26474ns | +116.47% |
| str_l_plain | 11941ns | 11881ns | 11378ns | 11818ns | 12407ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_l_eager | 23499ns | 22591ns | 24385ns | +142.55% | 0.044 |
| str_l_lazy | 23579ns | 22917ns | 24151ns | +143.37% | 0.043 |
| str_l_plain | 9688ns | 9198ns | 10080ns | base | 0.106 |

## Performance model

- Peak throughput: **0.111 Gops/s** (str_l_plain; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_l_eager | 0.044 | 39.5% |
| str_l_lazy | 0.043 | 39.0% |
| str_l_plain | 0.106 | 95.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_l_eager | 25767ns | 25767ns | +115.79% |
| str_l_lazy | 25849ns | 25849ns | +116.47% |
| str_l_plain | 11941ns | 11941ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_l_plain | 9638ns | base | --- | [9347, 10080] | --- | --- | --- | --- |
| str_l_eager | 23313ns | +13607.2ns (+141.2%) | [+13243, +14582]ns | [22800, 24385] | YES | 0.0313 | 0.0313 | 0 |
| str_l_lazy | 23574ns | +13927.9ns (+144.5%) | [+13259, +14485]ns | [23012, 24151] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_l_plain | str_l_eager | str_l_lazy |
|---|---|---|---|
| 1 | 9198ns | +167.7% | +151.2% |
| 2 | 9617ns | +139.3% | +145.8% |
| 3 | 10407ns | +132.0% | +125.9% |
| 4 | 9658ns | +141.2% | +154.7% |
| 5 | 9754ns | +139.2% | +143.0% |
| 6 | 9496ns | +137.9% | +141.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_l_eager | -0.288 | moderate- |
| str_l_lazy | -0.039 | ok |
| str_l_plain | -0.066 | ok |

**Consistency summary:**

- **str_l_eager**: won 0/6, lost 6/6
- **str_l_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_l_eager | 3794.8ns | 23499.3ns | 16.1% | HIGH |
| str_l_lazy | 3617.6ns | 23578.9ns | 15.3% | HIGH |
| str_l_plain | 4117.5ns | 9688.3ns | 42.5% | HIGH |

## Distribution (algo ns)

```
str_l_eager (n=6, range 22590.8-24384.6 ns)
  22590.8 |########################################
  22680.5 |
  22770.2 |
  22859.9 |
  22949.6 |########################################
  23039.2 |
  23128.9 |
  23218.6 |########################################
  23308.3 |########################################
  23398.0 |
  23487.7 |
  23577.4 |
  23667.1 |
  23756.8 |
  23846.5 |
  23936.1 |
  24025.8 |
  24115.5 |########################################
  24205.2 |
  24294.9 |
  (0 below, 1 above range)

str_l_lazy (n=6, range 22917.1-24150.8 ns)
  22917.1 |########################################
  22978.8 |
  23040.5 |
  23102.2 |########################################
  23163.8 |
  23225.5 |
  23287.2 |
  23348.9 |
  23410.6 |
  23472.3 |########################################
  23534.0 |
  23595.6 |########################################
  23657.3 |########################################
  23719.0 |
  23780.7 |
  23842.4 |
  23904.1 |
  23965.7 |
  24027.4 |
  24089.1 |
  (0 below, 1 above range)

str_l_plain (n=6, range 9197.9-10080.5 ns)
   9197.9 |########################################
   9242.0 |
   9286.2 |
   9330.3 |
   9374.4 |
   9418.5 |
   9462.7 |########################################
   9506.8 |
   9550.9 |
   9595.0 |########################################
   9639.2 |########################################
   9683.3 |
   9727.4 |########################################
   9771.6 |
   9815.7 |
   9859.8 |
   9903.9 |
   9948.1 |
   9992.2 |
  10036.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **str_l_eager**: bridge=16.2% of algo (FFI overhead may distort results)
- **str_l_lazy**: bridge=15.2% of algo (FFI overhead may distort results)
- **str_l_plain**: bridge=42.1% of algo (FFI overhead may distort results)
