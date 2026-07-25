# Runtime string interning, 8 compares of EQUAL strings: byte compare cannot exit early

3 variants, 6 samples per variant.
Baseline: **str_e_plain**

## Highlights

Baseline for all deltas below: **str_e_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_e_plain dominates: 220% faster than the next best (str_e_eager)

str_e_plain (437 ns) leads str_e_eager (1.40 us) by 220%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_e_lazy is an outlier: 4.1x slower than the field

str_e_lazy (1.81 us) is 4.1x the fastest (437 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### str_e_plain is fastest but the noisiest (CV 5.7%)

str_e_plain wins on median (437 ns) yet has the highest variance (CV 5.7%), while str_e_lazy is the steadiest (CV 4.7%, 1.81 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (str_e_plain)

The baseline str_e_plain is the fastest (437 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.1x the fastest

Fastest str_e_plain (437 ns) to slowest str_e_lazy (1.81 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_e_plain) is the fastest** at 436.9 ns median
- 2 variants significantly slower than baseline
- Spread: 4.14x (fastest 436.9 ns, slowest 1810.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_e_eager | 3736ns | 3650ns | 3582ns | 3632ns | 3970ns | +36.09% |
| str_e_lazy | 4071ns | 4038ns | 3872ns | 4012ns | 4258ns | +48.26% |
| str_e_plain | 2746ns | 2688ns | 2616ns | 2671ns | 2922ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_e_eager | 1441ns | 1372ns | 1550ns | +220.45% | 0.044 |
| str_e_lazy | 1820ns | 1697ns | 1923ns | +304.79% | 0.035 |
| str_e_plain | 450ns | 433ns | 478ns | base | 0.142 |

## Performance model

- Peak throughput: **0.148 Gops/s** (str_e_plain; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_e_eager | 0.046 | 31.0% |
| str_e_lazy | 0.035 | 23.9% |
| str_e_plain | 0.146 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_e_eager | 3736ns | 3736ns | +36.09% |
| str_e_lazy | 4071ns | 4071ns | +48.26% |
| str_e_plain | 2746ns | 2746ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_e_plain | 437ns | base | --- | [434, 478] | --- | --- | --- | --- |
| str_e_eager | 1398ns | +941.2ns (+215.4%) | [+918, +1115]ns | [1375, 1550] | YES | 0.0313 | 0.0313 | 0 |
| str_e_lazy | 1811ns | +1374.8ns (+314.7%) | [+1284, +1453]ns | [1727, 1923] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_e_plain | str_e_eager | str_e_lazy |
|---|---|---|---|
| 1 | 437ns | +255.2% | +343.4% |
| 2 | 503ns | +181.1% | +279.2% |
| 3 | 433ns | +257.1% | +305.4% |
| 4 | 435ns | +217.6% | +309.7% |
| 5 | 437ns | +214.3% | +321.2% |
| 6 | 452ns | +204.2% | +275.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_e_eager | -0.098 | ok |
| str_e_lazy | 0.097 | ok |
| str_e_plain | -0.314 | moderate- |

**Consistency summary:**

- **str_e_eager**: won 0/6, lost 6/6
- **str_e_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_e_eager | 4051.9ns | 1440.9ns | 281.2% | HIGH |
| str_e_lazy | 4255.5ns | 1820.1ns | 233.8% | HIGH |
| str_e_plain | 4182.1ns | 449.7ns | 930.1% | HIGH |

## Distribution (algo ns)

```
str_e_eager (n=6, range 1372.5-1550.0 ns)
   1372.5 |########################################
   1381.4 |####################
   1390.2 |
   1399.1 |
   1408.0 |####################
   1416.9 |
   1425.8 |
   1434.6 |
   1443.5 |
   1452.4 |
   1461.2 |
   1470.1 |
   1479.0 |
   1487.9 |
   1496.8 |
   1505.6 |
   1514.5 |
   1523.4 |
   1532.2 |
   1541.1 |####################
  (0 below, 1 above range)

str_e_lazy (n=6, range 1696.7-1923.1 ns)
   1696.7 |########################################
   1708.0 |
   1719.3 |
   1730.7 |
   1742.0 |
   1753.3 |########################################
   1764.6 |
   1775.9 |########################################
   1787.3 |
   1798.6 |
   1809.9 |
   1821.2 |
   1832.5 |########################################
   1843.9 |
   1855.2 |
   1866.5 |
   1877.8 |
   1889.1 |
   1900.5 |########################################
   1911.8 |
  (0 below, 1 above range)

str_e_plain (n=6, range 433.3-477.9 ns)
    433.3 |########################################
    435.5 |########################################
    437.8 |
    440.0 |
    442.2 |
    444.4 |
    446.7 |
    448.9 |
    451.1 |####################
    453.4 |
    455.6 |
    457.8 |
    460.1 |
    462.3 |
    464.5 |
    466.8 |
    469.0 |
    471.2 |
    473.4 |
    475.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **str_e_eager**: bridge=285.9% of algo (FFI overhead may distort results)
- **str_e_lazy**: bridge=228.6% of algo (FFI overhead may distort results)
- **str_e_plain**: bridge=961.8% of algo (FFI overhead may distort results)
