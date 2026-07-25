# Runtime string interning, 1 compare per build: the templating norm

3 variants, 6 samples per variant.
Baseline: **str_l_plain**

## Highlights

Baseline for all deltas below: **str_l_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_l_plain dominates: 211% faster than the next best (str_l_lazy)

str_l_plain (2.21 us) leads str_l_lazy (6.87 us) by 211%, a clear separation rather than a photo finish. CV 5.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_l_eager is an outlier: 3.1x slower than the field

str_l_eager (6.94 us) is 3.1x the fastest (2.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (str_l_plain)

The baseline str_l_plain is the fastest (2.21 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest str_l_plain (2.21 us) to slowest str_l_eager (6.94 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_l_plain) is the fastest** at 2206.5 ns median
- 2 variants significantly slower than baseline
- Spread: 3.14x (fastest 2206.5 ns, slowest 6938.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_l_eager | 9304ns | 9250ns | 8761ns | 9095ns | 9889ns | +106.66% |
| str_l_lazy | 9243ns | 9098ns | 8750ns | 8993ns | 9866ns | +105.32% |
| str_l_plain | 4502ns | 4499ns | 4236ns | 4414ns | 4766ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_l_eager | 6993ns | 6527ns | 7475ns | +217.10% | 0.037 |
| str_l_lazy | 6963ns | 6578ns | 7410ns | +215.73% | 0.037 |
| str_l_plain | 2205ns | 2057ns | 2343ns | base | 0.116 |

## Performance model

- Peak throughput: **0.124 Gops/s** (str_l_plain; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_l_eager | 0.037 | 29.6% |
| str_l_lazy | 0.037 | 30.0% |
| str_l_plain | 0.116 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_l_eager | 9304ns | 9304ns | +106.66% |
| str_l_lazy | 9243ns | 9243ns | +105.32% |
| str_l_plain | 4502ns | 4502ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_l_plain | 2206ns | base | --- | [2066, 2343] | --- | --- | --- | --- |
| str_l_eager | 6938ns | +4780.2ns (+216.6%) | [+4452, +5131]ns | [6567, 7475] | YES | 0.0313 | 0.0313 | 0 |
| str_l_lazy | 6865ns | +4654.8ns (+211.0%) | [+4509, +5110]ns | [6614, 7410] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_l_plain | str_l_eager | str_l_lazy |
|---|---|---|---|
| 1 | 2057ns | +221.3% | +223.3% |
| 2 | 2259ns | +221.7% | +230.6% |
| 3 | 2341ns | +222.7% | +214.0% |
| 4 | 2345ns | +215.3% | +200.6% |
| 5 | 2076ns | +214.4% | +221.9% |
| 6 | 2154ns | +206.8% | +205.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_l_eager | 0.250 | moderate+ |
| str_l_lazy | 0.211 | moderate+ |
| str_l_plain | 0.084 | ok |

**Consistency summary:**

- **str_l_eager**: won 0/6, lost 6/6
- **str_l_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_l_eager | 3586.4ns | 6993.4ns | 51.3% | HIGH |
| str_l_lazy | 3484.9ns | 6963.1ns | 50.0% | HIGH |
| str_l_plain | 4061.5ns | 2205.4ns | 184.2% | HIGH |

## Distribution (algo ns)

```
str_l_eager (n=6, range 6527.1-7474.6 ns)
   6527.1 |####################
   6574.5 |########################################
   6621.9 |
   6669.2 |
   6716.6 |
   6764.0 |
   6811.4 |
   6858.7 |
   6906.1 |
   6953.5 |
   7000.9 |
   7048.2 |
   7095.6 |
   7143.0 |
   7190.4 |
   7237.7 |####################
   7285.1 |
   7332.5 |
   7379.9 |####################
   7427.2 |
  (0 below, 1 above range)

str_l_lazy (n=6, range 6577.5-7409.8 ns)
   6577.5 |########################################
   6619.1 |########################################
   6660.7 |########################################
   6702.3 |
   6743.9 |
   6785.6 |
   6827.2 |
   6868.8 |
   6910.4 |
   6952.0 |
   6993.6 |
   7035.2 |########################################
   7076.9 |
   7118.5 |
   7160.1 |
   7201.7 |
   7243.3 |
   7284.9 |
   7326.5 |########################################
   7368.1 |
  (0 below, 1 above range)

str_l_plain (n=6, range 2057.1-2343.3 ns)
   2057.1 |########################################
   2071.4 |########################################
   2085.7 |
   2100.0 |
   2114.3 |
   2128.7 |
   2143.0 |########################################
   2157.3 |
   2171.6 |
   2185.9 |
   2200.2 |
   2214.5 |
   2228.8 |
   2243.1 |
   2257.4 |########################################
   2271.8 |
   2286.1 |
   2300.4 |
   2314.7 |
   2329.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **str_l_eager**: bridge=50.2% of algo (FFI overhead may distort results)
- **str_l_lazy**: bridge=49.8% of algo (FFI overhead may distort results)
- **str_l_plain**: bridge=180.8% of algo (FFI overhead may distort results)
