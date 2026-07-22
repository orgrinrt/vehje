# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (2.68 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 1.82 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 47% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (1.82 ms) leads carrier_ceil_interp (2.68 ms) by 47%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 32% (significant)

carrier_ceil_native is -869.89 us (32%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 1820008.5 ns median (-32.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.47x (fastest 1820008.5 ns, slowest 2683608.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2740653ns | 2687520ns | 2599175ns | 2664950ns | 2924946ns | base |
| carrier_ceil_native | 1830114ns | 1823615ns | 1801633ns | 1822062ns | 1856433ns | -33.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 2736720ns | 2595715ns | 2920391ns | base | 0.000 |
| carrier_ceil_native | 1826702ns | 1798436ns | 1852794ns | -33.25% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 67.0% |
| carrier_ceil_native | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 2740653ns | 2740653ns | base |
| carrier_ceil_native | 1830114ns | 1830114ns | -33.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2683609ns | base | --- | [2606159, 2920391] | --- | --- | --- | --- |
| carrier_ceil_native | 1820009ns | -869889.2ns (-32.4%) | [-1074012, -786151]ns | [1807305, 1852794] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 2836419ns | -33.8% |
| 2 | 2595715ns | -29.7% |
| 3 | 2616604ns | -30.6% |
| 4 | 2720654ns | -33.9% |
| 5 | 3004362ns | -39.5% |
| 6 | 2646564ns | -30.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.190 | ok |
| carrier_ceil_native | 0.128 | ok |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 2743112.2ns | 2736719.5ns | 100.2% | HIGH |
| carrier_ceil_native | 1826131.0ns | 1826702.3ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 2595714.6-2920390.6 ns)
  2595714.6 |########################################
  2611948.4 |########################################
  2628182.2 |
  2644416.0 |########################################
  2660649.8 |
  2676883.6 |
  2693117.4 |
  2709351.2 |########################################
  2725585.0 |
  2741818.8 |
  2758052.6 |
  2774286.4 |
  2790520.2 |
  2806754.0 |
  2822987.8 |########################################
  2839221.6 |
  2855455.4 |
  2871689.2 |
  2887923.0 |
  2904156.8 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 1798436.2-1852793.5 ns)
  1798436.2 |####################
  1801154.1 |
  1803871.9 |
  1806589.8 |
  1809307.7 |
  1812025.5 |
  1814743.4 |########################################
  1817461.3 |
  1820179.1 |
  1822897.0 |####################
  1825614.9 |
  1828332.7 |####################
  1831050.6 |
  1833768.5 |
  1836486.3 |
  1839204.2 |
  1841922.1 |
  1844639.9 |
  1847357.8 |
  1850075.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.0% of algo (FFI overhead may distort results)
