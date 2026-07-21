# tnum multiply: always-loop vs known-operand fast path (abstract arith)

2 variants, 6 samples per variant.
Baseline: **tm_fastpath**

## Highlights

Baseline for all deltas below: **tm_fastpath**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tm_fastpath dominates: 156% faster than the next best (tm_loop)

tm_fastpath (1.98 ms) leads tm_loop (5.07 ms) by 156%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (tm_fastpath)

The baseline tm_fastpath is the fastest (1.98 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (tm_fastpath) is the fastest** at 1977594.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.56x (fastest 1977594.6 ns, slowest 5070754.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tm_fastpath | 1983168ns | 1981004ns | 1955886ns | 1975032ns | 2009014ns | base |
| tm_loop | 5069540ns | 5074983ns | 5031870ns | 5064769ns | 5095532ns | +155.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tm_fastpath | 1979706ns | 1951992ns | 2005820ns | base | 0.008 |
| tm_loop | 5065631ns | 5027886ns | 5091848ns | +155.88% | 0.003 |

## Performance model

- Peak throughput: **0.008 Gops/s** (tm_fastpath; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tm_fastpath | 0.008 | 98.7% |
| tm_loop | 0.003 | 38.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tm_fastpath | 1983168ns | 1983168ns | base |
| tm_loop | 5069540ns | 5069540ns | +155.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tm_fastpath | 1977595ns | base | --- | [1955703, 2005820] | --- | --- | --- | --- |
| tm_loop | 5070754ns | +3088474.2ns (+156.2%) | [+3039626, +3129674]ns | [5034290, 5091848] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tm_fastpath | tm_loop |
|---|---|---|
| 1 | 1959415ns | +159.2% |
| 2 | 2005982ns | +152.4% |
| 3 | 1982833ns | +154.2% |
| 4 | 1972356ns | +158.2% |
| 5 | 1951992ns | +160.8% |
| 6 | 2005658ns | +150.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tm_fastpath | -0.379 | moderate- |
| tm_loop | -0.255 | moderate- |

**Consistency summary:**

- **tm_loop**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tm_fastpath | 59.0ns | 1979706.1ns | 0.0% |  |
| tm_loop | 115.1ns | 5065630.9ns | 0.0% |  |

## Distribution (algo ns)

```
tm_fastpath (n=6, range 1951992.1-2005820.4 ns)
  1951992.1 |########################################
  1954683.5 |
  1957374.9 |########################################
  1960066.3 |
  1962757.8 |
  1965449.2 |
  1968140.6 |
  1970832.0 |########################################
  1973523.4 |
  1976214.8 |
  1978906.2 |
  1981597.7 |########################################
  1984289.1 |
  1986980.5 |
  1989671.9 |
  1992363.3 |
  1995054.7 |
  1997746.2 |
  2000437.6 |
  2003129.0 |########################################
  (0 below, 1 above range)

tm_loop (n=6, range 5027886.2-5091848.0 ns)
  5027886.2 |########################################
  5031084.3 |
  5034282.4 |
  5037480.5 |
  5040678.5 |########################################
  5043876.6 |
  5047074.7 |
  5050272.8 |
  5053470.9 |
  5056669.0 |
  5059867.1 |########################################
  5063065.2 |
  5066263.2 |
  5069461.3 |
  5072659.4 |
  5075857.5 |########################################
  5079055.6 |
  5082253.7 |
  5085451.8 |
  5088649.9 |########################################
  (0 below, 1 above range)

```
