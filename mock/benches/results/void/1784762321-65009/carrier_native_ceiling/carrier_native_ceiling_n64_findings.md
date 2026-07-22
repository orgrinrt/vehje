# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (185.08 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 119.85 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 54% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (119.85 us) leads carrier_ceil_interp (185.08 us) by 54%, a clear separation rather than a photo finish. CV 7.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 37% (significant)

carrier_ceil_native is -68.75 us (37%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_ceil_native is fastest but the noisiest (CV 7.7%)

carrier_ceil_native wins on median (119.85 us) yet has the highest variance (CV 7.7%), while carrier_ceil_interp is the steadiest (CV 5.8%, 185.08 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_ceil_native** at 119848.9 ns median (-35.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.54x (fastest 119848.9 ns, slowest 185075.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 188540ns | 187629ns | 176130ns | 184121ns | 201372ns | base |
| carrier_ceil_native | 121268ns | 122893ns | 108447ns | 118709ns | 131518ns | -35.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 185770ns | 173638ns | 198175ns | base | 0.000 |
| carrier_ceil_native | 118460ns | 106064ns | 128516ns | -36.23% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 57.3% |
| carrier_ceil_native | 0.001 | 88.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 188540ns | 188540ns | base |
| carrier_ceil_native | 121268ns | 121268ns | -35.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 185075ns | base | --- | [174060, 198175] | --- | --- | --- | --- |
| carrier_ceil_native | 119849ns | -68749.8ns (-37.1%) | [-76721, -56459]ns | [107015, 128516] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 173638ns | -38.9% |
| 2 | 192259ns | -36.8% |
| 3 | 177892ns | -39.3% |
| 4 | 174481ns | -28.9% |
| 5 | 200880ns | -41.2% |
| 6 | 195469ns | -32.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.093 | ok |
| carrier_ceil_native | -0.261 | moderate- |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 184029.8ns | 185769.9ns | 99.1% | HIGH |
| carrier_ceil_native | 119490.8ns | 118460.1ns | 100.9% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 173638.3-198174.6 ns)
  173638.3 |########################################
  174865.1 |
  176091.9 |
  177318.7 |####################
  178545.6 |
  179772.4 |
  180999.2 |
  182226.0 |
  183452.8 |
  184679.6 |
  185906.4 |
  187133.3 |
  188360.1 |
  189586.9 |
  190813.7 |
  192040.5 |####################
  193267.3 |
  194494.2 |####################
  195721.0 |
  196947.8 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 106063.8-128516.0 ns)
  106063.8 |########################################
  107186.4 |########################################
  108309.0 |
  109431.6 |
  110554.2 |
  111676.9 |
  112799.5 |
  113922.1 |
  115044.7 |
  116167.3 |
  117289.9 |########################################
  118412.5 |
  119535.1 |
  120657.7 |########################################
  121780.3 |
  122902.9 |
  124025.6 |########################################
  125148.2 |
  126270.8 |
  127393.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.7% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=98.0% of algo (FFI overhead may distort results)
