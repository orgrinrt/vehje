# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (81.52 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 30.10 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 dominates: 15% faster than the next best (carrier_vert_scatter_vert4)

carrier_vert_scatter_vert8 (30.10 us) leads carrier_vert_scatter_vert4 (34.58 us) by 15%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_scatter_vert8 beats baseline by 63% (significant)

carrier_vert_scatter_vert8 is -51.63 us (63%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 2.7x slower than the field

carrier_vert_scatter_scalar (81.52 us) is 2.7x the fastest (30.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 30099.8 ns median (-63.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.71x (fastest 30099.8 ns, slowest 81518.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 84171ns | 84039ns | 81560ns | 83546ns | 86413ns | base |
| carrier_vert_scatter_vert4 | 37477ns | 36949ns | 35600ns | 36545ns | 39813ns | -55.47% |
| carrier_vert_scatter_vert8 | 32570ns | 32557ns | 31732ns | 32293ns | 33404ns | -61.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 81709ns | 79228ns | 83826ns | base | 0.003 |
| carrier_vert_scatter_vert4 | 35010ns | 33252ns | 37178ns | -57.15% | 0.007 |
| carrier_vert_scatter_vert8 | 30056ns | 29247ns | 30809ns | -63.22% | 0.009 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.003 | 35.9% |
| carrier_vert_scatter_vert4 | 0.007 | 84.6% |
| carrier_vert_scatter_vert8 | 0.009 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 84171ns | 84171ns | base |
| carrier_vert_scatter_vert4 | 37477ns | 37477ns | -55.47% |
| carrier_vert_scatter_vert8 | 32570ns | 32570ns | -61.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 81519ns | base | --- | [79783, 83826] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 34582ns | -47811.9ns (-58.7%) | [-48767, -43520]ns | [33269, 37178] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 30100ns | -51628.3ns (-63.3%) | [-53329, -50002]ns | [29260, 30809] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 82563ns | -58.7% | -63.0% |
| 2 | 80338ns | -58.6% | -63.6% |
| 3 | 80474ns | -58.6% | -63.7% |
| 4 | 79228ns | -50.4% | -61.8% |
| 5 | 83605ns | -58.0% | -62.8% |
| 6 | 84048ns | -58.3% | -64.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | 0.171 | ok |
| carrier_vert_scatter_vert4 | -0.098 | ok |
| carrier_vert_scatter_vert8 | 0.061 | ok |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 151806.5ns | 81709.4ns | 185.8% | HIGH |
| carrier_vert_scatter_vert4 | 103848.4ns | 35009.7ns | 296.6% | HIGH |
| carrier_vert_scatter_vert8 | 92049.6ns | 30056.2ns | 306.3% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 79227.9-83826.4 ns)
  79227.9 |########################################
  79457.8 |
  79687.8 |
  79917.7 |
  80147.6 |########################################
  80377.5 |########################################
  80607.5 |
  80837.4 |
  81067.3 |
  81297.2 |
  81527.2 |
  81757.1 |
  81987.0 |
  82217.0 |
  82446.9 |########################################
  82676.8 |
  82906.7 |
  83136.7 |
  83366.6 |
  83596.5 |########################################
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 33252.5-37177.9 ns)
  33252.5 |########################################
  33448.8 |
  33645.0 |
  33841.3 |
  34037.6 |####################
  34233.9 |
  34430.1 |
  34626.4 |
  34822.7 |
  35019.0 |########################################
  35215.2 |
  35411.5 |
  35607.8 |
  35804.0 |
  36000.3 |
  36196.6 |
  36392.9 |
  36589.1 |
  36785.4 |
  36981.7 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 29246.7-30808.7 ns)
  29246.7 |########################################
  29324.8 |
  29402.9 |
  29481.0 |
  29559.1 |
  29637.2 |
  29715.3 |
  29793.4 |
  29871.5 |####################
  29949.6 |
  30027.7 |
  30105.8 |
  30183.9 |
  30262.0 |####################
  30340.1 |
  30418.2 |
  30496.3 |####################
  30574.4 |
  30652.5 |
  30730.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=192.3% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=300.2% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: bridge=304.0% of algo (FFI overhead may distort results)
