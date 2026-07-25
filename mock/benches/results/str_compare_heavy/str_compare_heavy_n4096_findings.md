# Runtime string interning, 8 compares per build: interning's best case

3 variants, 6 samples per variant.
Baseline: **str_h_plain**

## Highlights

Baseline for all deltas below: **str_h_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_h_plain dominates: 165% faster than the next best (str_h_eager)

str_h_plain (33.97 us) leads str_h_eager (89.97 us) by 165%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_h_lazy is an outlier: 3.4x slower than the field

str_h_lazy (114.83 us) is 3.4x the fastest (33.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (str_h_plain)

The baseline str_h_plain is the fastest (33.97 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest str_h_plain (33.97 us) to slowest str_h_lazy (114.83 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_h_plain) is the fastest** at 33971.2 ns median
- 2 variants significantly slower than baseline
- Spread: 3.38x (fastest 33971.2 ns, slowest 114825.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_h_eager | 94167ns | 92263ns | 90102ns | 91889ns | 99618ns | +153.57% |
| str_h_lazy | 118617ns | 117130ns | 115140ns | 117038ns | 122723ns | +219.41% |
| str_h_plain | 37136ns | 36328ns | 35503ns | 36255ns | 39275ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_h_eager | 91718ns | 87401ns | 97039ns | +163.57% | 0.045 |
| str_h_lazy | 116284ns | 112884ns | 120317ns | +234.16% | 0.035 |
| str_h_plain | 34799ns | 33268ns | 36936ns | base | 0.118 |

## Performance model

- Peak throughput: **0.123 Gops/s** (str_h_plain; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_h_eager | 0.046 | 37.0% |
| str_h_lazy | 0.036 | 29.0% |
| str_h_plain | 0.121 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_h_eager | 94167ns | 94167ns | +153.57% |
| str_h_lazy | 118617ns | 118617ns | +219.41% |
| str_h_plain | 37136ns | 37136ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_h_plain | 33971ns | base | --- | [33489, 36936] | --- | --- | --- | --- |
| str_h_eager | 89973ns | +55461.1ns (+163.3%) | [+52993, +62305]ns | [88143, 97039] | YES | 0.0313 | 0.0313 | 0 |
| str_h_lazy | 114825ns | +80917.0ns (+238.2%) | [+78146, +85393]ns | [113710, 120317] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_h_plain | str_h_eager | str_h_lazy |
|---|---|---|---|
| 1 | 33852ns | +162.6% | +239.3% |
| 2 | 33710ns | +169.4% | +239.8% |
| 3 | 34090ns | +163.9% | +248.6% |
| 4 | 33268ns | +162.7% | +239.3% |
| 5 | 38114ns | +136.0% | +201.2% |
| 6 | 35759ns | +188.8% | +240.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_h_eager | -0.006 | ok |
| str_h_lazy | -0.247 | moderate- |
| str_h_plain | 0.059 | ok |

**Consistency summary:**

- **str_h_eager**: won 0/6, lost 6/6
- **str_h_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_h_eager | 4274.4ns | 91718.3ns | 4.7% |  |
| str_h_lazy | 4060.3ns | 116283.9ns | 3.5% |  |
| str_h_plain | 4381.0ns | 34798.7ns | 12.6% | HIGH |

## Distribution (algo ns)

```
str_h_eager (n=6, range 87401.2-97039.2 ns)
  87401.2 |####################
  87883.1 |
  88365.0 |
  88846.9 |####################
  89328.8 |
  89810.7 |########################################
  90292.6 |
  90774.5 |####################
  91256.4 |
  91738.3 |
  92220.2 |
  92702.1 |
  93184.0 |
  93665.9 |
  94147.8 |
  94629.7 |
  95111.6 |
  95593.5 |
  96075.4 |
  96557.3 |
  (0 below, 1 above range)

str_h_lazy (n=6, range 112883.8-120317.1 ns)
  112883.8 |####################
  113255.5 |
  113627.1 |
  113998.8 |
  114370.5 |####################
  114742.1 |########################################
  115113.8 |
  115485.5 |
  115857.1 |
  116228.8 |
  116600.5 |
  116972.1 |
  117343.8 |
  117715.4 |
  118087.1 |
  118458.8 |
  118830.4 |####################
  119202.1 |
  119573.8 |
  119945.4 |
  (0 below, 1 above range)

str_h_plain (n=6, range 33267.5-36936.3 ns)
  33267.5 |########################################
  33450.9 |
  33634.4 |########################################
  33817.8 |########################################
  34001.3 |########################################
  34184.7 |
  34368.1 |
  34551.6 |
  34735.0 |
  34918.5 |
  35101.9 |
  35285.3 |
  35468.8 |
  35652.2 |########################################
  35835.7 |
  36019.1 |
  36202.5 |
  36386.0 |
  36569.4 |
  36752.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **str_h_plain**: bridge=12.8% of algo (FFI overhead may distort results)
