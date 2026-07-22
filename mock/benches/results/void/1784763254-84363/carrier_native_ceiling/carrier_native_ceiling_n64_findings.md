# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (191.19 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 109.91 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 74% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (109.91 us) leads carrier_ceil_interp (191.19 us) by 74%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 39% (significant)

carrier_ceil_native is -74.05 us (39%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 109914.0 ns median (-42.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.74x (fastest 109914.0 ns, slowest 191191.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 197390ns | 193877ns | 178774ns | 189083ns | 219159ns | base |
| carrier_ceil_native | 114881ns | 112443ns | 108820ns | 111876ns | 122419ns | -41.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 194512ns | 176279ns | 215804ns | base | 0.000 |
| carrier_ceil_native | 112182ns | 106498ns | 119299ns | -42.33% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 55.7% |
| carrier_ceil_native | 0.001 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 197390ns | 197390ns | base |
| carrier_ceil_native | 114881ns | 114881ns | -41.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 191192ns | base | --- | [176539, 215804] | --- | --- | --- | --- |
| carrier_ceil_native | 109914ns | -74051.0ns (-38.7%) | [-103731, -69207]ns | [107332, 119299] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 176800ns | -39.8% |
| 2 | 176279ns | -38.6% |
| 3 | 220549ns | -50.5% |
| 4 | 198753ns | -37.8% |
| 5 | 211059ns | -45.5% |
| 6 | 183631ns | -39.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.087 | ok |
| carrier_ceil_native | 0.143 | ok |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 193751.6ns | 194511.7ns | 99.6% | HIGH |
| carrier_ceil_native | 112524.7ns | 112181.9ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 176278.7-215804.0 ns)
  176278.7 |########################################
  178255.0 |
  180231.2 |
  182207.5 |####################
  184183.8 |
  186160.0 |
  188136.3 |
  190112.5 |
  192088.8 |
  194065.1 |
  196041.3 |
  198017.6 |####################
  199993.9 |
  201970.1 |
  203946.4 |
  205922.6 |
  207898.9 |
  209875.2 |####################
  211851.4 |
  213827.7 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 106497.5-119299.4 ns)
  106497.5 |########################################
  107137.6 |
  107777.7 |########################################
  108417.8 |
  109057.9 |########################################
  109698.0 |
  110338.1 |########################################
  110978.2 |
  111618.3 |
  112258.4 |
  112898.4 |
  113538.5 |
  114178.6 |
  114818.7 |########################################
  115458.8 |
  116098.9 |
  116739.0 |
  117379.1 |
  118019.2 |
  118659.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=97.7% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.2% of algo (FFI overhead may distort results)
