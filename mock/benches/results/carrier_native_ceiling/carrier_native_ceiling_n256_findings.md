# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (2.62 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 1.81 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 45% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (1.81 ms) leads carrier_ceil_interp (2.62 ms) by 45%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 31% (significant)

carrier_ceil_native is -804.69 us (31%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_ceil_interp shows alternating (throttle bounce) (autocorr -0.58)

carrier_ceil_interp's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_ceil_native** at 1808865.4 ns median (-31.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.45x (fastest 1808865.4 ns, slowest 2622928.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2621476ns | 2625809ns | 2608030ns | 2620592ns | 2629524ns | base |
| carrier_ceil_native | 1815147ns | 1812262ns | 1798646ns | 1810777ns | 1829952ns | -30.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 2618559ns | 2604878ns | 2626550ns | base | 0.000 |
| carrier_ceil_native | 1811865ns | 1795038ns | 1826878ns | -30.81% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 68.4% |
| carrier_ceil_native | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 2621476ns | 2621476ns | base |
| carrier_ceil_native | 1815147ns | 1815147ns | -30.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 2622929ns | base | --- | [2606198, 2626550] | --- | --- | --- | --- |
| carrier_ceil_native | 1808865ns | -804688.2ns (-30.7%) | [-826459, -788935]ns | [1799850, 1826878] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 2626749ns | -30.0% |
| 2 | 2607518ns | -30.6% |
| 3 | 2626351ns | -31.7% |
| 4 | 2619590ns | -31.0% |
| 5 | 2604878ns | -30.3% |
| 6 | 2626268ns | -31.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.579 | HIGH- (thermal bounce) |
| carrier_ceil_native | 0.022 | ok |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 2615609.8ns | 2618558.7ns | 99.9% | HIGH |
| carrier_ceil_native | 1814733.1ns | 1811864.6ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 2604877.5-2626550.0 ns)
  2604877.5 |####################
  2605961.1 |
  2607044.8 |####################
  2608128.4 |
  2609212.0 |
  2610295.6 |
  2611379.2 |
  2612462.9 |
  2613546.5 |
  2614630.1 |
  2615713.8 |
  2616797.4 |
  2617881.0 |
  2618964.6 |####################
  2620048.2 |
  2621131.9 |
  2622215.5 |
  2623299.1 |
  2624382.8 |
  2625466.4 |########################################
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 1795037.5-1826877.9 ns)
  1795037.5 |########################################
  1796629.5 |
  1798221.5 |
  1799813.6 |
  1801405.6 |
  1802997.6 |
  1804589.6 |########################################
  1806181.6 |########################################
  1807773.7 |
  1809365.7 |########################################
  1810957.7 |
  1812549.7 |
  1814141.7 |########################################
  1815733.8 |
  1817325.8 |
  1818917.8 |
  1820509.8 |
  1822101.8 |
  1823693.9 |
  1825285.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.1% of algo (FFI overhead may distort results)
