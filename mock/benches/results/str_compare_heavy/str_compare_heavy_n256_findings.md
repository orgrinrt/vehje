# Runtime string interning, 8 compares per build: interning's best case

3 variants, 6 samples per variant.
Baseline: **str_h_plain**

## Highlights

Baseline for all deltas below: **str_h_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_h_plain dominates: 265% faster than the next best (str_h_eager)

str_h_plain (1.84 us) leads str_h_eager (6.74 us) by 265%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_h_lazy is an outlier: 4.6x slower than the field

str_h_lazy (8.49 us) is 4.6x the fastest (1.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### str_h_plain is fastest but the noisiest (CV 7.0%)

str_h_plain wins on median (1.84 us) yet has the highest variance (CV 7.0%), while str_h_eager is the steadiest (CV 5.0%, 6.74 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (str_h_plain)

The baseline str_h_plain is the fastest (1.84 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.6x the fastest

Fastest str_h_plain (1.84 us) to slowest str_h_lazy (8.49 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_h_plain) is the fastest** at 1844.8 ns median
- 2 variants significantly slower than baseline
- Spread: 4.60x (fastest 1844.8 ns, slowest 8489.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_h_eager | 9048ns | 8983ns | 8431ns | 8893ns | 9589ns | +115.30% |
| str_h_lazy | 10801ns | 10782ns | 10055ns | 10590ns | 11488ns | +157.01% |
| str_h_plain | 4202ns | 4091ns | 3890ns | 4060ns | 4573ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_h_eager | 6782ns | 6305ns | 7183ns | +256.59% | 0.038 |
| str_h_lazy | 8509ns | 7880ns | 9075ns | +347.43% | 0.030 |
| str_h_plain | 1902ns | 1762ns | 2079ns | base | 0.135 |

## Performance model

- Peak throughput: **0.145 Gops/s** (str_h_plain; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_h_eager | 0.038 | 26.1% |
| str_h_lazy | 0.030 | 20.8% |
| str_h_plain | 0.139 | 95.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_h_eager | 9048ns | 9048ns | +115.30% |
| str_h_lazy | 10801ns | 10801ns | +157.01% |
| str_h_plain | 4202ns | 4202ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_h_plain | 1845ns | base | --- | [1782, 2079] | --- | --- | --- | --- |
| str_h_eager | 6741ns | +4795.0ns (+259.9%) | [+4614, +5231]ns | [6422, 7183] | YES | 0.0313 | 0.0313 | 0 |
| str_h_lazy | 8489ns | +6609.1ns (+358.3%) | [+6140, +7074]ns | [7964, 9075] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_h_plain | str_h_eager | str_h_lazy |
|---|---|---|---|
| 1 | 1801ns | +265.8% | +337.5% |
| 2 | 1852ns | +253.0% | +334.8% |
| 3 | 1762ns | +257.7% | +356.7% |
| 4 | 2068ns | +242.7% | +335.2% |
| 5 | 1838ns | +296.2% | +385.8% |
| 6 | 2090ns | +229.8% | +337.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_h_eager | 0.333 | moderate+ |
| str_h_lazy | 0.453 | moderate+ |
| str_h_plain | -0.343 | moderate- |

**Consistency summary:**

- **str_h_eager**: won 0/6, lost 6/6
- **str_h_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_h_eager | 3645.3ns | 6781.8ns | 53.8% | HIGH |
| str_h_lazy | 3592.8ns | 8509.4ns | 42.2% | HIGH |
| str_h_plain | 4084.4ns | 1901.9ns | 214.8% | HIGH |

## Distribution (algo ns)

```
str_h_eager (n=6, range 6305.0-7183.3 ns)
   6305.0 |########################################
   6348.9 |
   6392.8 |
   6436.7 |
   6480.7 |
   6524.6 |########################################
   6568.5 |########################################
   6612.4 |
   6656.3 |
   6700.2 |
   6744.1 |
   6788.1 |
   6832.0 |
   6875.9 |########################################
   6919.8 |
   6963.7 |
   7007.6 |
   7051.6 |########################################
   7095.5 |
   7139.4 |
  (0 below, 1 above range)

str_h_lazy (n=6, range 7880.0-9074.6 ns)
   7880.0 |####################
   7939.7 |
   7999.5 |########################################
   8059.2 |
   8118.9 |
   8178.6 |
   8238.4 |
   8298.1 |
   8357.8 |
   8417.6 |
   8477.3 |
   8537.0 |
   8596.8 |
   8656.5 |
   8716.2 |
   8776.0 |
   8835.7 |
   8895.4 |####################
   8955.1 |####################
   9014.9 |
  (0 below, 1 above range)

str_h_plain (n=6, range 1762.5-2078.9 ns)
   1762.5 |########################################
   1778.3 |
   1794.1 |########################################
   1810.0 |
   1825.8 |########################################
   1841.6 |########################################
   1857.4 |
   1873.3 |
   1889.1 |
   1904.9 |
   1920.7 |
   1936.5 |
   1952.4 |
   1968.2 |
   1984.0 |
   1999.8 |
   2015.7 |
   2031.5 |
   2047.3 |
   2063.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **str_h_eager**: bridge=50.7% of algo (FFI overhead may distort results)
- **str_h_lazy**: bridge=40.5% of algo (FFI overhead may distort results)
- **str_h_plain**: bridge=219.1% of algo (FFI overhead may distort results)
