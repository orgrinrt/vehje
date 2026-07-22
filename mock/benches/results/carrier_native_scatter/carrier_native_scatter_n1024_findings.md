# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, scatter profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_scatter_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_scatter_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_scatter_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_scatter_interp has the worst median (45.01 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_scatter_copypatch at 15.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_scatter_copypatch beats baseline by 66% (significant)

carrier_nat_scatter_copypatch is -29.60 us (66%) faster than baseline carrier_nat_scatter_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_scatter_interp is an outlier: 2.9x slower than the field

carrier_nat_scatter_interp (45.01 us) is 2.9x the fastest (15.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_scatter_copypatch, carrier_nat_scatter_stencil) are a dead heat (<1%)

carrier_nat_scatter_copypatch (15.52 us) and carrier_nat_scatter_stencil (15.56 us) differ by 0.26%, inside the noise, even though the wider field spreads 190.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_nat_scatter_copypatch** at 15520.0 ns median (-65.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.90x (fastest 15520.0 ns, slowest 45006.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 17792ns | 17906ns | 16729ns | 17858ns | 18224ns | -62.44% |
| carrier_nat_scatter_interp | 47369ns | 47392ns | 46760ns | 47185ns | 47950ns | base |
| carrier_nat_scatter_stencil | 17968ns | 17966ns | 17714ns | 17935ns | 18144ns | -62.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 15427ns | 14498ns | 15808ns | -65.70% | 0.066 |
| carrier_nat_scatter_interp | 44981ns | 44397ns | 45535ns | base | 0.023 |
| carrier_nat_scatter_stencil | 15561ns | 15358ns | 15708ns | -65.41% | 0.066 |

## Performance model

- Peak throughput: **0.071 Gops/s** (carrier_nat_scatter_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.066 | 93.4% |
| carrier_nat_scatter_interp | 0.023 | 32.2% |
| carrier_nat_scatter_stencil | 0.066 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_scatter_copypatch | 17792ns | 17792ns | -62.44% |
| carrier_nat_scatter_interp | 47369ns | 47369ns | base |
| carrier_nat_scatter_stencil | 17968ns | 17968ns | -62.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_scatter_interp | 45006ns | base | --- | [44400, 45535] | --- | --- | --- | --- |
| carrier_nat_scatter_copypatch | 15520ns | -29602.1ns (-65.8%) | [-30044, -29015]ns | [14952, 15808] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_scatter_stencil | 15561ns | -29464.4ns (-65.5%) | [-29964, -28831]ns | [15413, 15708] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_scatter_interp | carrier_nat_scatter_copypatch | carrier_nat_scatter_stencil |
|---|---|---|---|
| 1 | 44397ns | -67.3% | -65.2% |
| 2 | 44621ns | -65.1% | -65.2% |
| 3 | 45642ns | -66.1% | -65.5% |
| 4 | 45429ns | -64.8% | -65.6% |
| 5 | 45392ns | -65.6% | -66.2% |
| 6 | 44403ns | -65.3% | -64.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_scatter_copypatch | -0.015 | ok |
| carrier_nat_scatter_interp | 0.134 | ok |
| carrier_nat_scatter_stencil | -0.267 | moderate- |

**Consistency summary:**

- **carrier_nat_scatter_copypatch**: won 6/6, lost 0/6
- **carrier_nat_scatter_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 96932.2ns | 15427.0ns | 628.3% | HIGH |
| carrier_nat_scatter_interp | 91272.0ns | 44980.6ns | 202.9% | HIGH |
| carrier_nat_scatter_stencil | 106266.5ns | 15560.6ns | 682.9% | HIGH |

## Distribution (algo ns)

```
carrier_nat_scatter_copypatch (n=6, range 14497.5-15808.5 ns)
  14497.5 |########################################
  14563.0 |
  14628.6 |
  14694.1 |
  14759.7 |
  14825.2 |
  14890.8 |
  14956.4 |
  15021.9 |
  15087.5 |
  15153.0 |
  15218.5 |
  15284.1 |
  15349.6 |########################################
  15415.2 |########################################
  15480.8 |
  15546.3 |########################################
  15611.9 |########################################
  15677.4 |
  15743.0 |
  (0 below, 1 above range)

carrier_nat_scatter_interp (n=6, range 44396.7-45535.4 ns)
  44396.7 |########################################
  44453.6 |
  44510.6 |
  44567.5 |####################
  44624.4 |
  44681.4 |
  44738.3 |
  44795.3 |
  44852.2 |
  44909.1 |
  44966.1 |
  45023.0 |
  45079.9 |
  45136.9 |
  45193.8 |
  45250.8 |
  45307.7 |
  45364.6 |####################
  45421.6 |####################
  45478.5 |
  (0 below, 1 above range)

carrier_nat_scatter_stencil (n=6, range 15357.5-15707.9 ns)
  15357.5 |########################################
  15375.0 |
  15392.5 |
  15410.1 |
  15427.6 |
  15445.1 |
  15462.6 |########################################
  15480.1 |
  15497.7 |
  15515.2 |########################################
  15532.7 |
  15550.2 |
  15567.7 |
  15585.3 |
  15602.8 |########################################
  15620.3 |
  15637.8 |
  15655.3 |########################################
  15672.9 |
  15690.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_scatter_copypatch**: bridge=627.4% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_interp**: bridge=202.1% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_stencil**: bridge=683.1% of algo (FFI overhead may distort results)
