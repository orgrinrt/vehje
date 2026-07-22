# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 49% faster than the next best (zig_tail)

zig_switch (1.31 ms) leads zig_tail (1.96 ms) by 49%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (1.31 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 1311946.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.49x (fastest 1311946.5 ns, slowest 1957140.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 1327381ns | 1315338ns | 1269119ns | 1305573ns | 1389223ns | base |
| zig_tail | 1964674ns | 1960607ns | 1952915ns | 1959735ns | 1977961ns | +48.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 1324031ns | 1265554ns | 1386202ns | base | 0.012 |
| zig_tail | 1961336ns | 1949927ns | 1974574ns | +48.13% | 0.008 |

## Performance model

- Peak throughput: **0.013 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.012 | 96.5% |
| zig_tail | 0.008 | 64.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 1327381ns | 1327381ns | base |
| zig_tail | 1964674ns | 1964674ns | +48.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 1311946ns | base | --- | [1273945, 1386202] | --- | --- | --- | --- |
| zig_tail | 1957141ns | +645722.0ns (+49.2%) | [+583607, +682587]ns | [1952295, 1974574] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 1346010ns | +44.9% |
| 2 | 1426393ns | +39.5% |
| 3 | 1265554ns | +54.5% |
| 4 | 1283553ns | +52.6% |
| 5 | 1282335ns | +52.4% |
| 6 | 1340340ns | +46.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | -0.020 | ok |
| zig_tail | -0.440 | moderate- |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 27.1ns | 1324030.9ns | 0.0% |  |
| zig_tail | 9.3ns | 1961336.3ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 1265554.2-1386201.6 ns)
  1265554.2 |####################
  1271586.6 |
  1277618.9 |########################################
  1283651.3 |
  1289683.7 |
  1295716.1 |
  1301748.4 |
  1307780.8 |
  1313813.2 |
  1319845.6 |
  1325877.9 |
  1331910.3 |
  1337942.7 |####################
  1343975.0 |####################
  1350007.4 |
  1356039.8 |
  1362072.2 |
  1368104.5 |
  1374136.9 |
  1380169.3 |
  (0 below, 1 above range)

zig_tail (n=6, range 1949927.1-1974573.5 ns)
  1949927.1 |####################
  1951159.4 |
  1952391.7 |
  1953624.1 |####################
  1954856.4 |####################
  1956088.7 |
  1957321.0 |
  1958553.3 |########################################
  1959785.7 |
  1961018.0 |
  1962250.3 |
  1963482.6 |
  1964714.9 |
  1965947.3 |
  1967179.6 |
  1968411.9 |
  1969644.2 |
  1970876.5 |
  1972108.9 |
  1973341.2 |
  (0 below, 1 above range)

```
