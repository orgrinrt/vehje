# Runtime string interning, 1 compare per build: the templating norm

3 variants, 6 samples per variant.
Baseline: **str_l_plain**

## Highlights

Baseline for all deltas below: **str_l_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_l_plain dominates: 215% faster than the next best (str_l_lazy)

str_l_plain (560 ns) leads str_l_lazy (1.76 us) by 215%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_l_eager is an outlier: 3.3x slower than the field

str_l_eager (1.84 us) is 3.3x the fastest (560 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### str_l_lazy shows alternating (throttle bounce) (autocorr -0.69)

str_l_lazy's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (str_l_plain)

The baseline str_l_plain is the fastest (560 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.3x the fastest

Fastest str_l_plain (560 ns) to slowest str_l_eager (1.84 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_l_plain) is the fastest** at 560.2 ns median
- 2 variants significantly slower than baseline
- Spread: 3.28x (fastest 560.2 ns, slowest 1835.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_l_eager | 4166ns | 4232ns | 3792ns | 4176ns | 4338ns | +41.10% |
| str_l_lazy | 4126ns | 4180ns | 3940ns | 4109ns | 4245ns | +39.76% |
| str_l_plain | 2952ns | 2986ns | 2764ns | 2982ns | 3002ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_l_eager | 1798ns | 1589ns | 1899ns | +225.56% | 0.036 |
| str_l_lazy | 1761ns | 1670ns | 1821ns | +218.75% | 0.036 |
| str_l_plain | 552ns | 503ns | 571ns | base | 0.116 |

## Performance model

- Peak throughput: **0.127 Gops/s** (str_l_plain; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_l_eager | 0.035 | 27.4% |
| str_l_lazy | 0.036 | 28.5% |
| str_l_plain | 0.114 | 89.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_l_eager | 4166ns | 4166ns | +41.10% |
| str_l_lazy | 4126ns | 4126ns | +39.76% |
| str_l_plain | 2952ns | 2952ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_l_plain | 560ns | base | --- | [526, 571] | --- | --- | --- | --- |
| str_l_eager | 1835ns | +1297.3ns (+231.6%) | [+1106, +1334]ns | [1661, 1899] | YES | 0.0313 | 0.0313 | 0 |
| str_l_lazy | 1762ns | +1230.7ns (+219.7%) | [+1140, +1255]ns | [1699, 1821] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_l_plain | str_l_eager | str_l_lazy |
|---|---|---|---|
| 1 | 561ns | +209.0% | +220.3% |
| 2 | 548ns | +189.8% | +204.7% |
| 3 | 560ns | +244.5% | +222.0% |
| 4 | 503ns | +258.5% | +243.8% |
| 5 | 573ns | +226.0% | +221.2% |
| 6 | 570ns | +228.1% | +203.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_l_eager | -0.101 | ok |
| str_l_lazy | -0.687 | HIGH- (thermal bounce) |
| str_l_plain | -0.324 | moderate- |

**Consistency summary:**

- **str_l_eager**: won 0/6, lost 6/6
- **str_l_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_l_eager | 3781.5ns | 1798.5ns | 210.3% | HIGH |
| str_l_lazy | 3684.0ns | 1760.8ns | 209.2% | HIGH |
| str_l_plain | 4206.7ns | 552.4ns | 761.5% | HIGH |

## Distribution (algo ns)

```
str_l_eager (n=6, range 1589.2-1899.2 ns)
   1589.2 |########################################
   1604.7 |
   1620.2 |
   1635.7 |
   1651.2 |
   1666.7 |
   1682.2 |
   1697.7 |
   1713.2 |
   1728.7 |########################################
   1744.2 |
   1759.7 |
   1775.2 |
   1790.7 |########################################
   1806.2 |
   1821.7 |
   1837.2 |
   1852.7 |########################################
   1868.2 |########################################
   1883.7 |
  (0 below, 1 above range)

str_l_lazy (n=6, range 1670.4-1821.0 ns)
   1670.4 |####################
   1677.9 |
   1685.5 |
   1693.0 |
   1700.5 |
   1708.1 |
   1715.6 |
   1723.1 |########################################
   1730.7 |
   1738.2 |
   1745.7 |
   1753.3 |
   1760.8 |
   1768.3 |
   1775.9 |
   1783.4 |
   1790.9 |####################
   1798.5 |####################
   1806.0 |
   1813.5 |
  (0 below, 1 above range)

str_l_plain (n=6, range 502.9-571.5 ns)
    502.9 |####################
    506.3 |
    509.8 |
    513.2 |
    516.6 |
    520.0 |
    523.5 |
    526.9 |
    530.3 |
    533.7 |
    537.2 |
    540.6 |
    544.0 |
    547.5 |####################
    550.9 |
    554.3 |
    557.7 |########################################
    561.2 |
    564.6 |
    568.0 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **str_l_eager**: bridge=201.1% of algo (FFI overhead may distort results)
- **str_l_lazy**: bridge=204.7% of algo (FFI overhead may distort results)
- **str_l_plain**: bridge=750.6% of algo (FFI overhead may distort results)
