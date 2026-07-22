# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (2.61 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 1.79 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 45% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (1.79 ms) leads carrier_ceil_interp (2.61 ms) by 45%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 31% (significant)

carrier_ceil_native is -817.55 us (31%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 1794662.7 ns median (-31.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.45x (fastest 1794662.7 ns, slowest 2610650.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2634578ns | 2613844ns | 2587119ns | 2612082ns | 2692051ns | base |
| carrier_ceil_native | 1825020ns | 1798250ns | 1787720ns | 1796000ns | 1887201ns | -30.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 2631130ns | 2583423ns | 2688389ns | base | 0.000 |
| carrier_ceil_native | 1821286ns | 1784020ns | 1883245ns | -30.78% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 68.3% |
| carrier_ceil_native | 0.000 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 2634578ns | 2634578ns | base |
| carrier_ceil_native | 1825020ns | 1825020ns | -30.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2610651ns | base | --- | [2594352, 2688389] | --- | --- | --- | --- |
| carrier_ceil_native | 1794663ns | -817549.2ns (-31.3%) | [-832100, -779883]ns | [1785951, 1883245] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 2749867ns | -30.4% |
| 2 | 2626911ns | -29.5% |
| 3 | 2583423ns | -30.4% |
| 4 | 2605280ns | -31.4% |
| 5 | 2608401ns | -31.3% |
| 6 | 2612900ns | -31.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | 0.108 | ok |
| carrier_ceil_native | 0.374 | moderate+ |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 2634666.2ns | 2631130.4ns | 100.1% | HIGH |
| carrier_ceil_native | 1824768.5ns | 1821286.4ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 2583423.3-2688388.8 ns)
  2583423.3 |####################
  2588671.6 |
  2593919.8 |
  2599168.1 |
  2604416.4 |########################################
  2609664.7 |####################
  2614912.9 |
  2620161.2 |
  2625409.5 |####################
  2630657.8 |
  2635906.0 |
  2641154.3 |
  2646402.6 |
  2651650.8 |
  2656899.1 |
  2662147.4 |
  2667395.7 |
  2672643.9 |
  2677892.2 |
  2683140.5 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 1784020.4-1883245.0 ns)
  1784020.4 |########################################
  1788981.6 |####################
  1793942.9 |####################
  1798904.1 |
  1803865.3 |
  1808826.5 |
  1813787.8 |
  1818749.0 |
  1823710.2 |
  1828671.5 |
  1833632.7 |
  1838593.9 |
  1843555.2 |
  1848516.4 |####################
  1853477.6 |
  1858438.9 |
  1863400.1 |
  1868361.3 |
  1873322.5 |
  1878283.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.2% of algo (FFI overhead may distort results)
