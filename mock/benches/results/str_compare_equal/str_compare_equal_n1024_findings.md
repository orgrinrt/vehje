# Runtime string interning, 8 compares of EQUAL strings: byte compare cannot exit early

3 variants, 6 samples per variant.
Baseline: **str_e_plain**

## Highlights

Baseline for all deltas below: **str_e_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_e_plain dominates: 225% faster than the next best (str_e_eager)

str_e_plain (7.02 us) leads str_e_eager (22.80 us) by 225%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_e_lazy is an outlier: 4.2x slower than the field

str_e_lazy (29.33 us) is 4.2x the fastest (7.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (str_e_plain)

The baseline str_e_plain is the fastest (7.02 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.2x the fastest

Fastest str_e_plain (7.02 us) to slowest str_e_lazy (29.33 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_e_plain) is the fastest** at 7021.6 ns median
- 2 variants significantly slower than baseline
- Spread: 4.18x (fastest 7021.6 ns, slowest 29330.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_e_eager | 24937ns | 25059ns | 24256ns | 24853ns | 25403ns | +165.42% |
| str_e_lazy | 31536ns | 31701ns | 29759ns | 31428ns | 32585ns | +235.66% |
| str_e_plain | 9395ns | 9316ns | 8862ns | 9297ns | 9809ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_e_eager | 22668ns | 22058ns | 23090ns | +218.88% | 0.045 |
| str_e_lazy | 29158ns | 27572ns | 30052ns | +310.18% | 0.035 |
| str_e_plain | 7109ns | 6689ns | 7467ns | base | 0.144 |

## Performance model

- Peak throughput: **0.153 Gops/s** (str_e_plain; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_e_eager | 0.045 | 29.3% |
| str_e_lazy | 0.035 | 22.8% |
| str_e_plain | 0.146 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_e_eager | 24937ns | 24937ns | +165.42% |
| str_e_lazy | 31536ns | 31536ns | +235.66% |
| str_e_plain | 9395ns | 9395ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_e_plain | 7022ns | base | --- | [6837, 7467] | --- | --- | --- | --- |
| str_e_eager | 22798ns | +15767.9ns (+224.6%) | [+15071, +15838]ns | [22116, 23090] | YES | 0.0313 | 0.0313 | 0 |
| str_e_lazy | 29330ns | +21893.5ns (+311.8%) | [+21238, +23017]ns | [28092, 30052] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_e_plain | str_e_eager | str_e_lazy |
|---|---|---|---|
| 1 | 6689ns | +229.7% | +312.2% |
| 2 | 7400ns | +199.7% | +291.9% |
| 3 | 7018ns | +225.7% | +307.7% |
| 4 | 6985ns | +225.6% | +324.7% |
| 5 | 7025ns | +225.5% | +332.5% |
| 6 | 7535ns | +209.5% | +294.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_e_eager | 0.325 | moderate+ |
| str_e_lazy | 0.279 | moderate+ |
| str_e_plain | -0.344 | moderate- |

**Consistency summary:**

- **str_e_eager**: won 0/6, lost 6/6
- **str_e_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_e_eager | 3630.9ns | 22667.8ns | 16.0% | HIGH |
| str_e_lazy | 3827.4ns | 29158.3ns | 13.1% | HIGH |
| str_e_plain | 4101.8ns | 7108.6ns | 57.7% | HIGH |

## Distribution (algo ns)

```
str_e_eager (n=6, range 22057.5-23089.8 ns)
  22057.5 |####################
  22109.1 |
  22160.7 |####################
  22212.3 |
  22264.0 |
  22315.6 |
  22367.2 |
  22418.8 |
  22470.4 |
  22522.0 |
  22573.6 |
  22625.2 |
  22676.8 |
  22728.5 |####################
  22780.1 |
  22831.7 |########################################
  22883.3 |
  22934.9 |
  22986.5 |
  23038.1 |
  (0 below, 1 above range)

str_e_lazy (n=6, range 27572.5-30052.3 ns)
  27572.5 |########################################
  27696.5 |
  27820.5 |
  27944.5 |
  28068.5 |
  28192.5 |
  28316.4 |
  28440.4 |
  28564.4 |########################################
  28688.4 |
  28812.4 |
  28936.4 |########################################
  29060.4 |
  29184.4 |
  29308.4 |
  29432.4 |
  29556.3 |########################################
  29680.3 |########################################
  29804.3 |
  29928.3 |
  (0 below, 1 above range)

str_e_plain (n=6, range 6689.2-7467.3 ns)
   6689.2 |####################
   6728.1 |
   6767.0 |
   6805.9 |
   6844.8 |
   6883.7 |
   6922.6 |
   6961.5 |####################
   7000.4 |########################################
   7039.3 |
   7078.2 |
   7117.2 |
   7156.1 |
   7195.0 |
   7233.9 |
   7272.8 |
   7311.7 |
   7350.6 |
   7389.5 |####################
   7428.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **str_e_eager**: bridge=15.7% of algo (FFI overhead may distort results)
- **str_e_lazy**: bridge=13.0% of algo (FFI overhead may distort results)
- **str_e_plain**: bridge=58.1% of algo (FFI overhead may distort results)
