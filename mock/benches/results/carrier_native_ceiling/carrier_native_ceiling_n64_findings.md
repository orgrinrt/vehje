# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (171.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 105.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 62% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (105.52 us) leads carrier_ceil_interp (171.41 us) by 62%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 38% (significant)

carrier_ceil_native is -65.25 us (38%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 105515.2 ns median (-38.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.62x (fastest 105515.2 ns, slowest 171405.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 173502ns | 173699ns | 170255ns | 173391ns | 175292ns | base |
| carrier_ceil_native | 108032ns | 107832ns | 104407ns | 107172ns | 111134ns | -37.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 171140ns | 167705ns | 172954ns | base | 0.000 |
| carrier_ceil_native | 105610ns | 101749ns | 108628ns | -38.29% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 59.4% |
| carrier_ceil_native | 0.001 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 173502ns | 173502ns | base |
| carrier_ceil_native | 108032ns | 108032ns | -37.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 171405ns | base | --- | [169061, 172954] | --- | --- | --- | --- |
| carrier_ceil_native | 105515ns | -65249.6ns (-38.1%) | [-69443, -61899]ns | [102686, 108628] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 171239ns | -39.5% |
| 2 | 171572ns | -37.1% |
| 3 | 170417ns | -35.9% |
| 4 | 173018ns | -41.2% |
| 5 | 172890ns | -38.7% |
| 6 | 167705ns | -37.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.228 | moderate- |
| carrier_ceil_native | -0.312 | moderate- |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 169172.3ns | 171140.0ns | 98.9% | HIGH |
| carrier_ceil_native | 105579.2ns | 105609.7ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 167705.4-172953.5 ns)
  167705.4 |########################################
  167967.8 |
  168230.2 |
  168492.6 |
  168755.0 |
  169017.4 |
  169279.8 |
  169542.3 |
  169804.7 |
  170067.1 |
  170329.5 |########################################
  170591.9 |
  170854.3 |
  171116.7 |########################################
  171379.1 |########################################
  171641.5 |
  171903.9 |
  172166.3 |
  172428.7 |
  172691.1 |########################################
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 101749.2-108628.4 ns)
  101749.2 |########################################
  102093.2 |
  102437.1 |
  102781.1 |
  103125.0 |
  103469.0 |########################################
  103812.9 |
  104156.9 |
  104500.9 |
  104844.8 |########################################
  105188.8 |
  105532.7 |
  105876.7 |########################################
  106220.6 |
  106564.6 |
  106908.6 |
  107252.5 |
  107596.5 |
  107940.4 |########################################
  108284.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=98.8% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=99.9% of algo (FFI overhead may distort results)
