# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (173.74 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 108.65 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 60% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (108.65 us) leads carrier_ceil_interp (173.74 us) by 60%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 38% (significant)

carrier_ceil_native is -65.76 us (38%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 108646.5 ns median (-37.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.60x (fastest 108646.5 ns, slowest 173739.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 179378ns | 176083ns | 169503ns | 175302ns | 190429ns | base |
| carrier_ceil_native | 110718ns | 111014ns | 103856ns | 110566ns | 114377ns | -38.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 176936ns | 167170ns | 187819ns | base | 0.000 |
| carrier_ceil_native | 108240ns | 101680ns | 111565ns | -38.83% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 58.5% |
| carrier_ceil_native | 0.001 | 93.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 179378ns | 179378ns | base |
| carrier_ceil_native | 110718ns | 110718ns | -38.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 173739ns | base | --- | [169249, 187819] | --- | --- | --- | --- |
| carrier_ceil_native | 108646ns | -65764.8ns (-37.9%) | [-79173, -61151]ns | [104508, 111565] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 167170ns | -39.2% |
| 2 | 175692ns | -37.6% |
| 3 | 198260ns | -45.7% |
| 4 | 171328ns | -33.8% |
| 5 | 177379ns | -38.2% |
| 6 | 171787ns | -37.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.228 | moderate- |
| carrier_ceil_native | -0.094 | ok |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 174899.4ns | 176935.9ns | 98.8% | HIGH |
| carrier_ceil_native | 108046.6ns | 108239.8ns | 99.8% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 167170.4-187819.4 ns)
  167170.4 |####################
  168202.9 |
  169235.3 |
  170267.8 |
  171300.2 |########################################
  172332.6 |
  173365.1 |
  174397.5 |
  175430.0 |####################
  176462.4 |####################
  177494.9 |
  178527.4 |
  179559.8 |
  180592.2 |
  181624.7 |
  182657.1 |
  183689.6 |
  184722.0 |
  185754.5 |
  186786.9 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 101679.6-111564.8 ns)
  101679.6 |####################
  102173.9 |
  102668.1 |
  103162.4 |
  103656.6 |
  104150.9 |
  104645.2 |
  105139.4 |
  105633.7 |
  106127.9 |
  106622.2 |
  107116.5 |####################
  107610.7 |####################
  108105.0 |
  108599.2 |
  109093.5 |
  109587.8 |########################################
  110082.0 |
  110576.3 |
  111070.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.6% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=99.3% of algo (FFI overhead may distort results)
