# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, madd profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_madd_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_madd_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_madd_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_madd_interp has the worst median (49.42 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_madd_copypatch at 34.65 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_madd_copypatch beats baseline by 30% (significant)

carrier_nat_madd_copypatch is -14.77 us (30%) faster than baseline carrier_nat_madd_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (carrier_nat_madd_copypatch, carrier_nat_madd_stencil) are a dead heat (<1%)

carrier_nat_madd_copypatch (34.65 us) and carrier_nat_madd_stencil (34.70 us) differ by 0.15%, inside the noise, even though the wider field spreads 42.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_nat_madd_copypatch** at 34647.3 ns median (-29.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.43x (fastest 34647.3 ns, slowest 49417.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 36637ns | 37022ns | 34351ns | 36834ns | 37486ns | -29.65% |
| carrier_nat_madd_interp | 52078ns | 52185ns | 51528ns | 52065ns | 52371ns | base |
| carrier_nat_madd_stencil | 37058ns | 37111ns | 36422ns | 37094ns | 37321ns | -28.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 34296ns | 32200ns | 35074ns | -30.68% | 0.030 |
| carrier_nat_madd_interp | 49475ns | 49221ns | 49722ns | base | 0.021 |
| carrier_nat_madd_stencil | 34669ns | 34082ns | 34928ns | -29.93% | 0.030 |

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_nat_madd_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_madd_copypatch | 0.030 | 92.9% |
| carrier_nat_madd_interp | 0.021 | 65.2% |
| carrier_nat_madd_stencil | 0.030 | 92.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_madd_copypatch | 36637ns | 36637ns | -29.65% |
| carrier_nat_madd_interp | 52078ns | 52078ns | base |
| carrier_nat_madd_stencil | 37058ns | 37058ns | -28.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_madd_interp | 49417ns | base | --- | [49285, 49722] | --- | --- | --- | --- |
| carrier_nat_madd_copypatch | 34647ns | -14769.8ns (-29.9%) | [-16555, -14211]ns | [33168, 35074] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_madd_stencil | 34699ns | -14739.8ns (-29.8%) | [-15124, -14554]ns | [34380, 34928] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_madd_interp | carrier_nat_madd_copypatch | carrier_nat_madd_stencil |
|---|---|---|---|
| 1 | 49762ns | -35.3% | -29.7% |
| 2 | 49354ns | -29.5% | -29.7% |
| 3 | 49350ns | -28.4% | -30.9% |
| 4 | 49682ns | -31.3% | -30.1% |
| 5 | 49480ns | -30.3% | -29.5% |
| 6 | 49221ns | -29.3% | -29.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_madd_copypatch | -0.105 | ok |
| carrier_nat_madd_interp | -0.208 | moderate- |
| carrier_nat_madd_stencil | -0.019 | ok |

**Consistency summary:**

- **carrier_nat_madd_copypatch**: won 6/6, lost 0/6
- **carrier_nat_madd_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 115558.8ns | 34296.3ns | 336.9% | HIGH |
| carrier_nat_madd_interp | 99259.5ns | 49475.0ns | 200.6% | HIGH |
| carrier_nat_madd_stencil | 124565.4ns | 34669.1ns | 359.3% | HIGH |

## Distribution (algo ns)

```
carrier_nat_madd_copypatch (n=6, range 32199.6-35073.9 ns)
  32199.6 |####################
  32343.3 |
  32487.0 |
  32630.8 |
  32774.5 |
  32918.2 |
  33061.9 |
  33205.6 |
  33349.3 |
  33493.1 |
  33636.8 |
  33780.5 |
  33924.2 |
  34067.9 |####################
  34211.6 |
  34355.4 |####################
  34499.1 |
  34642.8 |
  34786.5 |########################################
  34930.2 |
  (0 below, 1 above range)

carrier_nat_madd_interp (n=6, range 49221.2-49722.5 ns)
  49221.2 |####################
  49246.3 |
  49271.3 |
  49296.4 |
  49321.5 |
  49346.5 |########################################
  49371.6 |
  49396.7 |
  49421.7 |
  49446.8 |
  49471.8 |####################
  49496.9 |
  49522.0 |
  49547.0 |
  49572.1 |
  49597.2 |
  49622.2 |
  49647.3 |
  49672.4 |####################
  49697.4 |
  (0 below, 1 above range)

carrier_nat_madd_stencil (n=6, range 34081.7-34928.1 ns)
  34081.7 |#############
  34124.0 |
  34166.3 |
  34208.7 |
  34251.0 |
  34293.3 |
  34335.6 |
  34377.9 |
  34420.3 |
  34462.6 |
  34504.9 |
  34547.2 |
  34589.5 |
  34631.9 |
  34674.2 |########################################
  34716.5 |
  34758.8 |
  34801.1 |
  34843.5 |
  34885.8 |#############
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_madd_copypatch**: bridge=336.7% of algo (FFI overhead may distort results)
- **carrier_nat_madd_interp**: bridge=200.7% of algo (FFI overhead may distort results)
- **carrier_nat_madd_stencil**: bridge=348.6% of algo (FFI overhead may distort results)
