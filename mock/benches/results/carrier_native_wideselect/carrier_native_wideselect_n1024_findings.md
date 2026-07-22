# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, wideselect profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_wideselect_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_wideselect_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_wideselect_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_wideselect_interp has the worst median (43.52 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_wideselect_stencil at 16.38 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_wideselect_stencil beats baseline by 62% (significant)

carrier_nat_wideselect_stencil is -27.07 us (62%) faster than baseline carrier_nat_wideselect_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_wideselect_interp is an outlier: 2.7x slower than the field

carrier_nat_wideselect_interp (43.52 us) is 2.7x the fastest (16.38 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_nat_wideselect_stencil** at 16382.9 ns median (-62.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.66x (fastest 16382.9 ns, slowest 43517.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 18727ns | 18957ns | 17026ns | 18868ns | 19367ns | -59.25% |
| carrier_nat_wideselect_interp | 45952ns | 46021ns | 45024ns | 45945ns | 46428ns | base |
| carrier_nat_wideselect_stencil | 18681ns | 18771ns | 17623ns | 18664ns | 19238ns | -59.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 16340ns | 14852ns | 16878ns | -62.38% | 0.063 |
| carrier_nat_wideselect_interp | 43430ns | 42617ns | 43904ns | base | 0.024 |
| carrier_nat_wideselect_stencil | 16293ns | 15387ns | 16772ns | -62.49% | 0.063 |

## Performance model

- Peak throughput: **0.069 Gops/s** (carrier_nat_wideselect_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.062 | 89.7% |
| carrier_nat_wideselect_interp | 0.024 | 34.1% |
| carrier_nat_wideselect_stencil | 0.063 | 90.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_wideselect_copypatch | 18727ns | 18727ns | -59.25% |
| carrier_nat_wideselect_interp | 45952ns | 45952ns | base |
| carrier_nat_wideselect_stencil | 18681ns | 18681ns | -59.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_wideselect_interp | 43517ns | base | --- | [42871, 43904] | --- | --- | --- | --- |
| carrier_nat_wideselect_copypatch | 16551ns | -26987.3ns (-62.0%) | [-27525, -26760]ns | [15590, 16878] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_wideselect_stencil | 16383ns | -27065.2ns (-62.2%) | [-28042, -26306]ns | [15723, 16772] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_wideselect_interp | carrier_nat_wideselect_copypatch | carrier_nat_wideselect_stencil |
|---|---|---|---|
| 1 | 42617ns | -65.1% | -61.9% |
| 2 | 43724ns | -61.2% | -62.2% |
| 3 | 43124ns | -62.1% | -60.8% |
| 4 | 44083ns | -61.9% | -65.1% |
| 5 | 43448ns | -61.6% | -63.0% |
| 6 | 43587ns | -62.4% | -61.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_wideselect_copypatch | -0.253 | moderate- |
| carrier_nat_wideselect_interp | -0.399 | moderate- |
| carrier_nat_wideselect_stencil | -0.201 | moderate- |

**Consistency summary:**

- **carrier_nat_wideselect_copypatch**: won 6/6, lost 0/6
- **carrier_nat_wideselect_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 90040.1ns | 16339.9ns | 551.0% | HIGH |
| carrier_nat_wideselect_interp | 100344.5ns | 43430.5ns | 231.0% | HIGH |
| carrier_nat_wideselect_stencil | 101535.0ns | 16292.6ns | 623.2% | HIGH |

## Distribution (algo ns)

```
carrier_nat_wideselect_copypatch (n=6, range 14852.1-16877.9 ns)
  14852.1 |########################################
  14953.4 |
  15054.7 |
  15156.0 |
  15257.3 |
  15358.6 |
  15459.9 |
  15561.1 |
  15662.4 |
  15763.7 |
  15865.0 |
  15966.3 |
  16067.6 |
  16168.9 |
  16270.2 |########################################
  16371.5 |########################################
  16472.8 |
  16574.1 |
  16675.4 |########################################
  16776.7 |########################################
  (0 below, 1 above range)

carrier_nat_wideselect_interp (n=6, range 42617.1-43903.6 ns)
  42617.1 |########################################
  42681.4 |
  42745.7 |
  42810.1 |
  42874.4 |
  42938.7 |
  43003.0 |
  43067.4 |########################################
  43131.7 |
  43196.0 |
  43260.3 |
  43324.6 |
  43389.0 |########################################
  43453.3 |
  43517.6 |
  43581.9 |########################################
  43646.3 |
  43710.6 |########################################
  43774.9 |
  43839.2 |
  (0 below, 1 above range)

carrier_nat_wideselect_stencil (n=6, range 15386.7-16772.0 ns)
  15386.7 |########################################
  15456.0 |
  15525.2 |
  15594.5 |
  15663.8 |
  15733.0 |
  15802.3 |
  15871.6 |
  15940.8 |
  16010.1 |########################################
  16079.4 |
  16148.6 |
  16217.9 |########################################
  16287.2 |
  16356.4 |
  16425.7 |
  16495.0 |########################################
  16564.2 |
  16633.5 |########################################
  16702.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_wideselect_copypatch**: bridge=540.7% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_interp**: bridge=228.3% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_stencil**: bridge=605.7% of algo (FFI overhead may distort results)
