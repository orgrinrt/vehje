# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (2.60 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 1.82 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 43% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (1.82 ms) leads carrier_ceil_interp (2.60 ms) by 43%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 30% (significant)

carrier_ceil_native is -770.91 us (30%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_ceil_interp shows alternating (throttle bounce) (autocorr -0.57)

carrier_ceil_interp's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_ceil_native** at 1821730.6 ns median (-29.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.43x (fastest 1821730.6 ns, slowest 2596965.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2601992ns | 2600207ns | 2591848ns | 2598978ns | 2611585ns | base |
| carrier_ceil_native | 1846494ns | 1824783ns | 1814800ns | 1823680ns | 1896562ns | -29.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 2598803ns | 2588735ns | 2608397ns | base | 0.000 |
| carrier_ceil_native | 1843426ns | 1812296ns | 1893233ns | -29.07% | 0.000 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_ceil_interp | 16199768 | 73814122 | 0.219 | 1.00× |
| carrier_ceil_native | 11307460 | 54707598 | 0.207 | 0.70× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 69.8% |
| carrier_ceil_native | 0.000 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 2601992ns | 2601992ns | base |
| carrier_ceil_native | 1846494ns | 1846494ns | -29.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2596965ns | base | --- | [2591046, 2608397] | --- | --- | --- | --- |
| carrier_ceil_native | 1821731ns | -770907.7ns (-29.7%) | [-781477, -713746]ns | [1815315, 1893233] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 2600226ns | -30.3% |
| 2 | 2596542ns | -29.8% |
| 3 | 2597389ns | -29.5% |
| 4 | 2588735ns | -29.7% |
| 5 | 2616568ns | -25.3% |
| 6 | 2593358ns | -29.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.574 | HIGH- (thermal bounce) |
| carrier_ceil_native | -0.279 | moderate- |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 2596866.3ns | 2598802.9ns | 99.9% | HIGH |
| carrier_ceil_native | 1838855.9ns | 1843426.2ns | 99.8% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 2588735.0-2608396.6 ns)
  2588735.0 |########################################
  2589718.1 |
  2590701.2 |
  2591684.2 |
  2592667.3 |########################################
  2593650.4 |
  2594633.5 |
  2595616.6 |########################################
  2596599.7 |########################################
  2597582.7 |
  2598565.8 |
  2599548.9 |########################################
  2600532.0 |
  2601515.1 |
  2602498.2 |
  2603481.2 |
  2604464.3 |
  2605447.4 |
  2606430.5 |
  2607413.6 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 1812296.2-1893232.7 ns)
  1812296.2 |####################
  1816343.0 |########################################
  1820389.8 |####################
  1824436.7 |
  1828483.5 |####################
  1832530.3 |
  1836577.1 |
  1840624.0 |
  1844670.8 |
  1848717.6 |
  1852764.4 |
  1856811.3 |
  1860858.1 |
  1864904.9 |
  1868951.8 |
  1872998.6 |
  1877045.4 |
  1881092.2 |
  1885139.1 |
  1889185.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=99.9% of algo (FFI overhead may distort results)
