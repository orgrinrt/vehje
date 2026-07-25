# Runtime string interning, 1 compare per build: the templating norm

3 variants, 6 samples per variant.
Baseline: **str_l_plain**

## Highlights

Baseline for all deltas below: **str_l_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_l_plain dominates: 107% faster than the next best (str_l_eager)

str_l_plain (166.50 us) leads str_l_eager (344.41 us) by 107%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_l_lazy is an outlier: 2.1x slower than the field

str_l_lazy (350.63 us) is 2.1x the fastest (166.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (str_l_plain)

The baseline str_l_plain is the fastest (166.50 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (str_l_plain) is the fastest** at 166502.9 ns median
- 2 variants significantly slower than baseline
- Spread: 2.11x (fastest 166502.9 ns, slowest 350629.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_l_eager | 346828ns | 346963ns | 342438ns | 346696ns | 349219ns | +105.18% |
| str_l_lazy | 356035ns | 353177ns | 347323ns | 352439ns | 365786ns | +110.63% |
| str_l_plain | 169032ns | 169149ns | 162414ns | 168154ns | 173659ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_l_eager | 344227ns | 339708ns | 346481ns | +106.80% | 0.048 |
| str_l_lazy | 353470ns | 344743ns | 363085ns | +112.35% | 0.046 |
| str_l_plain | 166455ns | 160010ns | 171152ns | base | 0.098 |

## Performance model

- Peak throughput: **0.102 Gops/s** (str_l_plain; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_l_eager | 0.048 | 46.5% |
| str_l_lazy | 0.047 | 45.6% |
| str_l_plain | 0.098 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_l_eager | 346828ns | 346828ns | +105.18% |
| str_l_lazy | 356035ns | 356035ns | +110.63% |
| str_l_plain | 169032ns | 169032ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_l_plain | 166503ns | base | --- | [161708, 171152] | --- | --- | --- | --- |
| str_l_eager | 344406ns | +177862.7ns (+106.8%) | [+171986, +183469]ns | [341794, 346481] | YES | 0.0313 | 0.0313 | 0 |
| str_l_lazy | 350630ns | +186202.1ns (+111.8%) | [+179748, +195096]ns | [346695, 363085] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_l_plain | str_l_eager | str_l_lazy |
|---|---|---|---|
| 1 | 160010ns | +116.5% | +119.5% |
| 2 | 166340ns | +104.2% | +112.5% |
| 3 | 173688ns | +98.2% | +114.6% |
| 4 | 163406ns | +110.4% | +113.4% |
| 5 | 168617ns | +105.5% | +107.6% |
| 6 | 166666ns | +106.7% | +106.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_l_eager | -0.344 | moderate- |
| str_l_lazy | -0.094 | ok |
| str_l_plain | -0.262 | moderate- |

**Consistency summary:**

- **str_l_eager**: won 0/6, lost 6/6
- **str_l_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_l_eager | 3944.8ns | 344227.4ns | 1.1% |  |
| str_l_lazy | 4091.9ns | 353469.8ns | 1.2% |  |
| str_l_plain | 4107.8ns | 166454.6ns | 2.5% |  |

## Distribution (algo ns)

```
str_l_eager (n=6, range 339708.3-346481.2 ns)
  339708.3 |########################################
  340046.9 |
  340385.6 |
  340724.2 |
  341062.9 |
  341401.5 |
  341740.2 |
  342078.8 |
  342417.5 |
  342756.1 |
  343094.8 |
  343433.4 |
  343772.1 |########################################
  344110.7 |########################################
  344449.4 |########################################
  344788.0 |
  345126.7 |
  345465.3 |
  345804.0 |
  346142.6 |########################################
  (0 below, 1 above range)

str_l_lazy (n=6, range 344743.3-363084.8 ns)
  344743.3 |########################################
  345660.4 |
  346577.5 |
  347494.5 |
  348411.6 |########################################
  349328.7 |########################################
  350245.8 |
  351162.8 |########################################
  352079.9 |
  352997.0 |########################################
  353914.1 |
  354831.1 |
  355748.2 |
  356665.3 |
  357582.4 |
  358499.4 |
  359416.5 |
  360333.6 |
  361250.7 |
  362167.7 |
  (0 below, 1 above range)

str_l_plain (n=6, range 160010.4-171152.5 ns)
  160010.4 |####################
  160567.5 |
  161124.6 |
  161681.7 |
  162238.8 |
  162795.9 |
  163353.0 |####################
  163910.1 |
  164467.2 |
  165024.3 |
  165581.5 |
  166138.6 |########################################
  166695.7 |
  167252.8 |
  167809.9 |
  168367.0 |####################
  168924.1 |
  169481.2 |
  170038.3 |
  170595.4 |
  (0 below, 1 above range)

```
