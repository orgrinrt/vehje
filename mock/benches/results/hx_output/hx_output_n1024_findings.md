# Interpolation output: format-to-temp+copy vs format-in-place

2 variants, 6 samples per variant.
Baseline: **hx_output__inplace**

## Highlights

Baseline for all deltas below: **hx_output__inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_output__inplace) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_output__inplace has the worst median (1.99 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_output__temp at 1.91 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (79 ns) is smaller than the fastest variant's own run-to-run std-dev (142 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: hx_output__temp** at 1908.3 ns median (-4.0% vs baseline)
- Spread: 1.04x (fastest 1908.3 ns, slowest 1987.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_output__inplace | 4564ns | 4680ns | 3735ns | 4623ns | 4891ns | base |
| hx_output__temp | 4464ns | 4495ns | 3916ns | 4395ns | 4840ns | -2.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_output__inplace | 1940ns | 1595ns | 2082ns | base | 0.528 |
| hx_output__temp | 1897ns | 1661ns | 2059ns | -2.22% | 0.540 |

## Performance model

- Peak throughput: **0.642 Gops/s** (hx_output__inplace; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_output__inplace | 0.515 | 80.2% |
| hx_output__temp | 0.537 | 83.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_output__inplace | 4564ns | 4564ns | base |
| hx_output__temp | 4464ns | 4464ns | -2.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_output__inplace | 1987ns | base | --- | [1750, 2082] | --- | --- | --- | --- |
| hx_output__temp | 1908ns | no significant difference | [-159, +96]ns | [1724, 2059] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_output__inplace | hx_output__temp |
|---|---|---|
| 1 | 1595ns | +4.2% |
| 2 | 2053ns | -6.7% |
| 3 | 2081ns | -8.6% |
| 4 | 2084ns | -0.7% |
| 5 | 1921ns | +6.5% |
| 6 | 1905ns | -6.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_output__inplace | -0.028 | ok |
| hx_output__temp | 0.049 | ok |

**Consistency summary:**

- **hx_output__temp**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_output__inplace | 2.3ns | 1939.9ns | 0.1% |  |
| hx_output__temp | 3.1ns | 1896.8ns | 0.2% |  |

## Distribution (algo ns)

```
hx_output__inplace (n=6, range 1594.6-2082.5 ns)
   1594.6 |########################################
   1619.0 |
   1643.4 |
   1667.8 |
   1692.2 |
   1716.6 |
   1741.0 |
   1765.4 |
   1789.8 |
   1814.2 |
   1838.5 |
   1862.9 |
   1887.3 |########################################
   1911.7 |########################################
   1936.1 |
   1960.5 |
   1984.9 |
   2009.3 |
   2033.7 |########################################
   2058.1 |########################################
  (0 below, 1 above range)

hx_output__temp (n=6, range 1660.8-2058.6 ns)
   1660.8 |####################
   1680.7 |
   1700.6 |
   1720.5 |
   1740.3 |
   1760.2 |
   1780.1 |####################
   1800.0 |
   1819.9 |
   1839.8 |
   1859.7 |
   1879.6 |
   1899.5 |########################################
   1919.3 |
   1939.2 |
   1959.1 |
   1979.0 |
   1998.9 |
   2018.8 |
   2038.7 |####################
  (0 below, 1 above range)

```
