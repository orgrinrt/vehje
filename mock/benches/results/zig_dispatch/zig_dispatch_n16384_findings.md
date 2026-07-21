# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 39% faster than the next best (zig_tail)

zig_switch (1.42 ms) leads zig_tail (1.96 ms) by 39%, a clear separation rather than a photo finish. CV 4.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (1.42 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 1415643.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.39x (fastest 1415643.6 ns, slowest 1963482.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 1409918ns | 1418388ns | 1315524ns | 1391210ns | 1485176ns | base |
| zig_tail | 1964349ns | 1966864ns | 1935891ns | 1964584ns | 1978224ns | +39.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 1406961ns | 1312295ns | 1482425ns | base | 0.012 |
| zig_tail | 1961088ns | 1932996ns | 1975146ns | +39.38% | 0.008 |

## Performance model

- Peak throughput: **0.012 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.012 | 92.7% |
| zig_tail | 0.008 | 66.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 1409918ns | 1409918ns | base |
| zig_tail | 1964349ns | 1964349ns | +39.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 1415644ns | base | --- | [1322813, 1482425] | --- | --- | --- | --- |
| zig_tail | 1963482ns | +547838.9ns (+38.7%) | [+462210, +652333]ns | [1944635, 1975146] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 1454228ns | +32.9% |
| 2 | 1432774ns | +37.3% |
| 3 | 1312295ns | +50.9% |
| 4 | 1398513ns | +40.2% |
| 5 | 1333330ns | +47.7% |
| 6 | 1510623ns | +29.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | -0.265 | moderate- |
| zig_tail | -0.087 | ok |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 27.8ns | 1406960.6ns | 0.0% |  |
| zig_tail | 10.6ns | 1961087.9ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 1312295.0-1482425.4 ns)
  1312295.0 |########################################
  1320801.5 |
  1329308.0 |########################################
  1337814.6 |
  1346321.1 |
  1354827.6 |
  1363334.1 |
  1371840.6 |
  1380347.2 |
  1388853.7 |
  1397360.2 |########################################
  1405866.7 |
  1414373.2 |
  1422879.8 |
  1431386.3 |########################################
  1439892.8 |
  1448399.3 |########################################
  1456905.8 |
  1465412.4 |
  1473918.9 |
  (0 below, 1 above range)

zig_tail (n=6, range 1932996.2-1975145.8 ns)
  1932996.2 |########################################
  1935103.7 |
  1937211.2 |
  1939318.6 |
  1941426.1 |
  1943533.6 |
  1945641.1 |
  1947748.6 |
  1949856.0 |
  1951963.5 |
  1954071.0 |
  1956178.5 |########################################
  1958286.0 |########################################
  1960393.4 |
  1962500.9 |
  1964608.4 |########################################
  1966715.9 |
  1968823.4 |########################################
  1970930.8 |
  1973038.3 |
  (0 below, 1 above range)

```
