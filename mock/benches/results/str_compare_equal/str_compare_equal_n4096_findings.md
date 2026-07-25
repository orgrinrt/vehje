# Runtime string interning, 8 compares of EQUAL strings: byte compare cannot exit early

3 variants, 6 samples per variant.
Baseline: **str_e_plain**

## Highlights

Baseline for all deltas below: **str_e_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_e_plain dominates: 201% faster than the next best (str_e_eager)

str_e_plain (28.59 us) leads str_e_eager (86.05 us) by 201%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_e_lazy is an outlier: 3.8x slower than the field

str_e_lazy (109.95 us) is 3.8x the fastest (28.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (str_e_plain)

The baseline str_e_plain is the fastest (28.59 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.8x the fastest

Fastest str_e_plain (28.59 us) to slowest str_e_lazy (109.95 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_e_plain) is the fastest** at 28591.5 ns median
- 2 variants significantly slower than baseline
- Spread: 3.85x (fastest 28591.5 ns, slowest 109945.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_e_eager | 88443ns | 88366ns | 86581ns | 88165ns | 89792ns | +182.60% |
| str_e_lazy | 112173ns | 112586ns | 108888ns | 111631ns | 114628ns | +258.43% |
| str_e_plain | 31296ns | 30997ns | 30183ns | 30836ns | 32542ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_e_eager | 86075ns | 84212ns | 87356ns | +197.87% | 0.048 |
| str_e_lazy | 109712ns | 106577ns | 112140ns | +279.67% | 0.037 |
| str_e_plain | 28897ns | 27885ns | 30090ns | base | 0.142 |

## Performance model

- Peak throughput: **0.147 Gops/s** (str_e_plain; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_e_eager | 0.048 | 32.4% |
| str_e_lazy | 0.037 | 25.4% |
| str_e_plain | 0.143 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_e_eager | 88443ns | 88443ns | +182.60% |
| str_e_lazy | 112173ns | 112173ns | +258.43% |
| str_e_plain | 31296ns | 31296ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_e_plain | 28591ns | base | --- | [28009, 30090] | --- | --- | --- | --- |
| str_e_eager | 86052ns | +57531.9ns (+201.2%) | [+55708, +58295]ns | [84816, 87356] | YES | 0.0313 | 0.0313 | 0 |
| str_e_lazy | 109945ns | +79854.8ns (+279.3%) | [+78467, +84123]ns | [107050, 112140] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_e_plain | str_e_eager | str_e_lazy |
|---|---|---|---|
| 1 | 28148ns | +205.3% | +297.8% |
| 2 | 27885ns | +202.0% | +302.8% |
| 3 | 29035ns | +201.7% | +270.3% |
| 4 | 29848ns | +191.9% | +271.8% |
| 5 | 30332ns | +181.6% | +259.0% |
| 6 | 28132ns | +206.3% | +278.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_e_eager | -0.234 | moderate- |
| str_e_lazy | -0.039 | ok |
| str_e_plain | 0.197 | ok |

**Consistency summary:**

- **str_e_eager**: won 0/6, lost 6/6
- **str_e_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_e_eager | 3805.9ns | 86074.8ns | 4.4% |  |
| str_e_lazy | 3803.9ns | 109711.6ns | 3.5% |  |
| str_e_plain | 4075.8ns | 28896.8ns | 14.1% | HIGH |

## Distribution (algo ns)

```
str_e_eager (n=6, range 84211.7-87356.1 ns)
  84211.7 |########################################
  84368.9 |
  84526.1 |
  84683.4 |
  84840.6 |
  84997.8 |
  85155.0 |
  85312.2 |########################################
  85469.4 |
  85626.7 |
  85783.9 |########################################
  85941.1 |
  86098.3 |########################################
  86255.5 |
  86412.7 |
  86570.0 |
  86727.2 |
  86884.4 |
  87041.6 |########################################
  87198.8 |
  (0 below, 1 above range)

str_e_lazy (n=6, range 106576.7-112139.6 ns)
  106576.7 |########################################
  106854.8 |
  107133.0 |
  107411.1 |########################################
  107689.3 |
  107967.4 |
  108245.6 |
  108523.7 |
  108801.9 |########################################
  109080.0 |
  109358.1 |
  109636.3 |
  109914.4 |
  110192.6 |
  110470.7 |
  110748.9 |########################################
  111027.0 |
  111305.2 |
  111583.3 |
  111861.5 |########################################
  (0 below, 1 above range)

str_e_plain (n=6, range 27885.4-30090.4 ns)
  27885.4 |####################
  27995.7 |
  28105.9 |########################################
  28216.2 |
  28326.4 |
  28436.7 |
  28546.9 |
  28657.2 |
  28767.4 |
  28877.7 |
  28987.9 |####################
  29098.2 |
  29208.4 |
  29318.7 |
  29428.9 |
  29539.2 |
  29649.4 |
  29759.7 |####################
  29869.9 |
  29980.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **str_e_plain**: bridge=14.3% of algo (FFI overhead may distort results)
