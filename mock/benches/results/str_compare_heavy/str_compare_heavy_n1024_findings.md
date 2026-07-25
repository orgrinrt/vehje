# Runtime string interning, 8 compares per build: interning's best case

3 variants, 6 samples per variant.
Baseline: **str_h_plain**

## Highlights

Baseline for all deltas below: **str_h_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_h_plain dominates: 185% faster than the next best (str_h_eager)

str_h_plain (7.95 us) leads str_h_eager (22.70 us) by 185%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_h_lazy is an outlier: 3.7x slower than the field

str_h_lazy (29.76 us) is 3.7x the fastest (7.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### str_h_lazy shows alternating (throttle bounce) (autocorr -0.51)

str_h_lazy's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (str_h_plain)

The baseline str_h_plain is the fastest (7.95 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.7x the fastest

Fastest str_h_plain (7.95 us) to slowest str_h_lazy (29.76 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_h_plain) is the fastest** at 7953.8 ns median
- 2 variants significantly slower than baseline
- Spread: 3.74x (fastest 7953.8 ns, slowest 29755.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_h_eager | 25256ns | 24931ns | 24450ns | 24825ns | 26307ns | +147.32% |
| str_h_lazy | 31774ns | 32036ns | 31106ns | 31738ns | 32163ns | +211.14% |
| str_h_plain | 10212ns | 10172ns | 9959ns | 10146ns | 10438ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_h_eager | 22954ns | 22257ns | 23885ns | +187.94% | 0.045 |
| str_h_lazy | 29503ns | 28892ns | 29824ns | +270.08% | 0.035 |
| str_h_plain | 7972ns | 7757ns | 8185ns | base | 0.128 |

## Performance model

- Peak throughput: **0.132 Gops/s** (str_h_plain; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_h_eager | 0.045 | 34.2% |
| str_h_lazy | 0.034 | 26.1% |
| str_h_plain | 0.129 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_h_eager | 25256ns | 25256ns | +147.32% |
| str_h_lazy | 31774ns | 31774ns | +211.14% |
| str_h_plain | 10212ns | 10212ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_h_plain | 7954ns | base | --- | [7777, 8185] | --- | --- | --- | --- |
| str_h_eager | 22699ns | +14834.4ns (+186.5%) | [+14299, +15814]ns | [22279, 23885] | YES | 0.0313 | 0.0313 | 0 |
| str_h_lazy | 29756ns | +21605.8ns (+271.6%) | [+21083, +21903]ns | [28928, 29824] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_h_plain | str_h_eager | str_h_lazy |
|---|---|---|---|
| 1 | 7797ns | +193.3% | +281.7% |
| 2 | 7933ns | +184.0% | +264.2% |
| 3 | 8203ns | +171.3% | +262.7% |
| 4 | 7757ns | +187.5% | +273.4% |
| 5 | 7974ns | +195.3% | +273.9% |
| 6 | 8168ns | +196.5% | +265.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_h_eager | 0.377 | moderate+ |
| str_h_lazy | -0.509 | HIGH- (thermal bounce) |
| str_h_plain | -0.305 | moderate- |

**Consistency summary:**

- **str_h_eager**: won 0/6, lost 6/6
- **str_h_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_h_eager | 3912.4ns | 22954.4ns | 17.0% | HIGH |
| str_h_lazy | 3796.7ns | 29502.6ns | 12.9% | HIGH |
| str_h_plain | 4219.8ns | 7972.0ns | 52.9% | HIGH |

## Distribution (algo ns)

```
str_h_eager (n=6, range 22257.1-23884.6 ns)
  22257.1 |########################################
  22338.5 |
  22419.8 |
  22501.2 |####################
  22582.6 |
  22664.0 |
  22745.3 |
  22826.7 |####################
  22908.1 |
  22989.5 |
  23070.8 |
  23152.2 |
  23233.6 |
  23314.9 |
  23396.3 |
  23477.7 |####################
  23559.1 |
  23640.4 |
  23721.8 |
  23803.2 |
  (0 below, 1 above range)

str_h_lazy (n=6, range 28891.7-29823.8 ns)
  28891.7 |####################
  28938.3 |####################
  28984.9 |
  29031.5 |
  29078.1 |
  29124.7 |
  29171.3 |
  29217.9 |
  29264.5 |
  29311.1 |
  29357.7 |
  29404.3 |
  29450.9 |
  29497.5 |
  29544.1 |
  29590.7 |
  29637.3 |
  29683.9 |
  29730.5 |########################################
  29777.1 |####################
  (0 below, 1 above range)

str_h_plain (n=6, range 7757.1-8185.4 ns)
   7757.1 |########################################
   7778.5 |########################################
   7799.9 |
   7821.3 |
   7842.8 |
   7864.2 |
   7885.6 |
   7907.0 |
   7928.4 |########################################
   7949.8 |
   7971.2 |########################################
   7992.7 |
   8014.1 |
   8035.5 |
   8056.9 |
   8078.3 |
   8099.7 |
   8121.2 |
   8142.6 |
   8164.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **str_h_eager**: bridge=17.4% of algo (FFI overhead may distort results)
- **str_h_lazy**: bridge=13.0% of algo (FFI overhead may distort results)
- **str_h_plain**: bridge=52.6% of algo (FFI overhead may distort results)
