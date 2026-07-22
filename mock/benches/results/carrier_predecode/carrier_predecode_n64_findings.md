# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (2.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flatthread at 1.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_predec_flatthread dominates: 27% faster than the next best (carrier_predec_flat)

carrier_predec_flatthread (1.47 us) leads carrier_predec_flat (1.87 us) by 27%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_predec_flatthread beats baseline by 35% (significant)

carrier_predec_flatthread is -760 ns (35%) faster than baseline carrier_predec_wire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_predec_flatthread** at 1473.9 ns median (-33.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.49x (fastest 1473.9 ns, slowest 2200.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 4192ns | 4109ns | 3939ns | 4081ns | 4485ns | -9.71% |
| carrier_predec_flatthread | 3798ns | 3726ns | 3590ns | 3721ns | 4019ns | -18.19% |
| carrier_predec_wire | 4643ns | 4446ns | 4304ns | 4442ns | 5114ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 1924ns | 1819ns | 2060ns | -16.30% | 0.033 |
| carrier_predec_flatthread | 1499ns | 1423ns | 1583ns | -34.80% | 0.043 |
| carrier_predec_wire | 2299ns | 2128ns | 2533ns | base | 0.028 |

## Performance model

- Peak throughput: **0.045 Gops/s** (carrier_predec_flatthread; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.034 | 76.0% |
| carrier_predec_flatthread | 0.043 | 96.6% |
| carrier_predec_wire | 0.029 | 64.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 4192ns | 4192ns | -9.71% |
| carrier_predec_flatthread | 3798ns | 3798ns | -18.19% |
| carrier_predec_wire | 4643ns | 4643ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 2201ns | base | --- | [2163, 2533] | --- | --- | --- | --- |
| carrier_predec_flat | 1873ns | -326.4ns (-14.8%) | [-525, -273]ns | [1839, 2060] | YES | 0.0313 | 0.0313 | 0 |
| carrier_predec_flatthread | 1474ns | -760.0ns (-34.5%) | [-959, -681]ns | [1440, 1583] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat | carrier_predec_flatthread |
|---|---|---|---|
| 1 | 2534ns | -28.2% | -42.1% |
| 2 | 2200ns | -12.6% | -32.5% |
| 3 | 2202ns | -14.4% | -33.9% |
| 4 | 2198ns | -15.2% | -35.2% |
| 5 | 2128ns | -12.6% | -30.5% |
| 6 | 2532ns | -13.2% | -33.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | -0.116 | ok |
| carrier_predec_flatthread | 0.051 | ok |
| carrier_predec_wire | -0.157 | ok |

**Consistency summary:**

- **carrier_predec_flat**: won 6/6, lost 0/6
- **carrier_predec_flatthread**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 294.0ns | 1924.3ns | 15.3% | HIGH |
| carrier_predec_flatthread | 295.3ns | 1499.0ns | 19.7% | HIGH |
| carrier_predec_wire | 56.3ns | 2299.0ns | 2.4% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 1819.2-2060.2 ns)
   1819.2 |####################
   1831.2 |
   1843.3 |
   1855.3 |########################################
   1867.4 |
   1879.5 |####################
   1891.5 |
   1903.5 |
   1915.6 |####################
   1927.6 |
   1939.7 |
   1951.8 |
   1963.8 |
   1975.8 |
   1987.9 |
   1999.9 |
   2012.0 |
   2024.0 |
   2036.1 |
   2048.1 |
  (0 below, 1 above range)

carrier_predec_flatthread (n=6, range 1423.3-1583.3 ns)
   1423.3 |####################
   1431.3 |
   1439.3 |
   1447.3 |
   1455.3 |####################
   1463.3 |####################
   1471.3 |
   1479.3 |########################################
   1487.3 |
   1495.3 |
   1503.3 |
   1511.3 |
   1519.3 |
   1527.3 |
   1535.3 |
   1543.3 |
   1551.3 |
   1559.3 |
   1567.3 |
   1575.3 |
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 2127.9-2533.3 ns)
   2127.9 |#############
   2148.2 |
   2168.4 |
   2188.7 |########################################
   2209.0 |
   2229.3 |
   2249.5 |
   2269.8 |
   2290.1 |
   2310.4 |
   2330.6 |
   2350.9 |
   2371.2 |
   2391.4 |
   2411.7 |
   2432.0 |
   2452.3 |
   2472.5 |
   2492.8 |
   2513.1 |#############
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_predec_flat**: bridge=15.3% of algo (FFI overhead may distort results)
- **carrier_predec_flatthread**: bridge=19.7% of algo (FFI overhead may distort results)
