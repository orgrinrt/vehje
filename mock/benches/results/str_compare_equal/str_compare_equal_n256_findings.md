# Runtime string interning, 8 compares of EQUAL strings: byte compare cannot exit early

3 variants, 6 samples per variant.
Baseline: **str_e_plain**

## Highlights

Baseline for all deltas below: **str_e_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_e_plain dominates: 240% faster than the next best (str_e_eager)

str_e_plain (1.81 us) leads str_e_eager (6.14 us) by 240%, a clear separation rather than a photo finish. CV 6.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_e_lazy is an outlier: 4.1x slower than the field

str_e_lazy (7.37 us) is 4.1x the fastest (1.81 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### str_e_plain is fastest but the noisiest (CV 6.1%)

str_e_plain wins on median (1.81 us) yet has the highest variance (CV 6.1%), while str_e_lazy is the steadiest (CV 4.4%, 7.37 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (str_e_plain)

The baseline str_e_plain is the fastest (1.81 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.1x the fastest

Fastest str_e_plain (1.81 us) to slowest str_e_lazy (7.37 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_e_plain) is the fastest** at 1806.9 ns median
- 2 variants significantly slower than baseline
- Spread: 4.08x (fastest 1806.9 ns, slowest 7374.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_e_eager | 8450ns | 8450ns | 7907ns | 8289ns | 8964ns | +104.58% |
| str_e_lazy | 9715ns | 9806ns | 9082ns | 9629ns | 10162ns | +135.20% |
| str_e_plain | 4131ns | 4139ns | 3862ns | 4054ns | 4380ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_e_eager | 6119ns | 5748ns | 6449ns | +238.82% | 0.042 |
| str_e_lazy | 7351ns | 6907ns | 7706ns | +306.99% | 0.035 |
| str_e_plain | 1806ns | 1674ns | 1930ns | base | 0.142 |

## Performance model

- Peak throughput: **0.153 Gops/s** (str_e_plain; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_e_eager | 0.042 | 27.3% |
| str_e_lazy | 0.035 | 22.7% |
| str_e_plain | 0.142 | 92.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_e_eager | 8450ns | 8450ns | +104.58% |
| str_e_lazy | 9715ns | 9715ns | +135.20% |
| str_e_plain | 4131ns | 4131ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_e_plain | 1807ns | base | --- | [1681, 1930] | --- | --- | --- | --- |
| str_e_eager | 6136ns | +4288.8ns (+237.4%) | [+4092, +4559]ns | [5773, 6449] | YES | 0.0313 | 0.0313 | 0 |
| str_e_lazy | 7374ns | +5582.0ns (+308.9%) | [+5264, +5787]ns | [6971, 7706] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_e_plain | str_e_eager | str_e_lazy |
|---|---|---|---|
| 1 | 1674ns | +243.3% | +312.5% |
| 2 | 1688ns | +243.7% | +325.5% |
| 3 | 1875ns | +242.6% | +314.5% |
| 4 | 1897ns | +241.0% | +299.0% |
| 5 | 1964ns | +227.4% | +289.1% |
| 6 | 1739ns | +236.4% | +304.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_e_eager | 0.244 | moderate+ |
| str_e_lazy | 0.105 | ok |
| str_e_plain | 0.237 | moderate+ |

**Consistency summary:**

- **str_e_eager**: won 0/6, lost 6/6
- **str_e_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_e_eager | 3824.8ns | 6119.3ns | 62.5% | HIGH |
| str_e_lazy | 3858.5ns | 7350.6ns | 52.5% | HIGH |
| str_e_plain | 4092.3ns | 1806.1ns | 226.6% | HIGH |

## Distribution (algo ns)

```
str_e_eager (n=6, range 5747.5-6448.6 ns)
   5747.5 |####################
   5782.6 |####################
   5817.6 |####################
   5852.7 |
   5887.7 |
   5922.8 |
   5957.8 |
   5992.9 |
   6027.9 |
   6063.0 |
   6098.0 |
   6133.1 |
   6168.1 |
   6203.2 |
   6238.2 |
   6273.3 |
   6308.3 |
   6343.4 |
   6378.4 |
   6413.5 |########################################
  (0 below, 1 above range)

str_e_lazy (n=6, range 6906.7-7706.4 ns)
   6906.7 |########################################
   6946.7 |
   6986.7 |
   7026.7 |########################################
   7066.6 |
   7106.6 |
   7146.6 |########################################
   7186.6 |
   7226.6 |
   7266.6 |
   7306.6 |
   7346.6 |
   7386.6 |
   7426.5 |
   7466.5 |
   7506.5 |
   7546.5 |########################################
   7586.5 |
   7626.5 |########################################
   7666.5 |
  (0 below, 1 above range)

str_e_plain (n=6, range 1674.2-1930.5 ns)
   1674.2 |########################################
   1687.0 |########################################
   1699.8 |
   1712.6 |
   1725.5 |
   1738.3 |########################################
   1751.1 |
   1763.9 |
   1776.7 |
   1789.5 |
   1802.3 |
   1815.1 |
   1828.0 |
   1840.8 |
   1853.6 |
   1866.4 |########################################
   1879.2 |
   1892.0 |########################################
   1904.8 |
   1917.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **str_e_eager**: bridge=62.5% of algo (FFI overhead may distort results)
- **str_e_lazy**: bridge=51.8% of algo (FFI overhead may distort results)
- **str_e_plain**: bridge=222.9% of algo (FFI overhead may distort results)
