# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (2.57 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 1.79 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 44% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (1.79 ms) leads carrier_ceil_interp (2.57 ms) by 44%, a clear separation rather than a photo finish. CV 0.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 30% (significant)

carrier_ceil_native is -784.47 us (30%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 1786543.1 ns median (-30.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.44x (fastest 1786543.1 ns, slowest 2572616.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2577058ns | 2574998ns | 2569320ns | 2574654ns | 2584535ns | base |
| carrier_ceil_native | 1791756ns | 1789701ns | 1786902ns | 1788776ns | 1798653ns | -30.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 2574403ns | 2566960ns | 2581281ns | base | 0.000 |
| carrier_ceil_native | 1788836ns | 1784498ns | 1795439ns | -30.51% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 69.4% |
| carrier_ceil_native | 0.000 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 2577058ns | 2577058ns | base |
| carrier_ceil_native | 1791756ns | 1791756ns | -30.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2572616ns | base | --- | [2569313, 2581281] | --- | --- | --- | --- |
| carrier_ceil_native | 1786543ns | -784467.8ns (-30.5%) | [-793231, -779005]ns | [1784524, 1795439] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 2585502ns | -30.8% |
| 2 | 2577060ns | -30.3% |
| 3 | 2571828ns | -30.3% |
| 4 | 2571667ns | -30.6% |
| 5 | 2573405ns | -30.7% |
| 6 | 2566960ns | -30.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | 0.199 | ok |
| carrier_ceil_native | 0.344 | moderate+ |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 2570356.5ns | 2574403.5ns | 99.8% | HIGH |
| carrier_ceil_native | 1788956.8ns | 1788835.5ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 2566960.0-2581280.9 ns)
  2566960.0 |####################
  2567676.0 |
  2568392.1 |
  2569108.1 |
  2569824.2 |
  2570540.2 |
  2571256.3 |########################################
  2571972.3 |
  2572688.3 |
  2573404.4 |####################
  2574120.4 |
  2574836.5 |
  2575552.5 |
  2576268.6 |
  2576984.6 |####################
  2577700.6 |
  2578416.7 |
  2579132.7 |
  2579848.8 |
  2580564.8 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 1784497.5-1795439.2 ns)
  1784497.5 |########################################
  1785044.6 |####################
  1785591.7 |
  1786138.8 |
  1786685.8 |
  1787232.9 |
  1787780.0 |####################
  1788327.1 |
  1788874.2 |
  1789421.3 |
  1789968.4 |
  1790515.4 |
  1791062.5 |
  1791609.6 |
  1792156.7 |
  1792703.8 |
  1793250.9 |####################
  1793797.9 |
  1794345.0 |
  1794892.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.0% of algo (FFI overhead may distort results)
