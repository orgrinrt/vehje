# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, tight profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_tight_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_tight_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_tight_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_tight_interp has the worst median (45.66 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_tight_stencil at 22.84 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_tight_stencil beats baseline by 50% (significant)

carrier_nat_tight_stencil is -22.90 us (50%) faster than baseline carrier_nat_tight_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (carrier_nat_tight_stencil, carrier_nat_tight_copypatch) are a dead heat (<1%)

carrier_nat_tight_stencil (22.84 us) and carrier_nat_tight_copypatch (22.98 us) differ by 0.58%, inside the noise, even though the wider field spreads 99.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_nat_tight_stencil** at 22842.9 ns median (-50.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.00x (fastest 22842.9 ns, slowest 45660.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 25113ns | 25384ns | 23297ns | 25304ns | 25734ns | -47.83% |
| carrier_nat_tight_interp | 48133ns | 48009ns | 47100ns | 47810ns | 49132ns | base |
| carrier_nat_tight_stencil | 25143ns | 25207ns | 24536ns | 25167ns | 25412ns | -47.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 22704ns | 20993ns | 23245ns | -50.33% | 0.045 |
| carrier_nat_tight_interp | 45707ns | 44662ns | 46604ns | base | 0.022 |
| carrier_nat_tight_stencil | 22785ns | 22261ns | 23021ns | -50.15% | 0.045 |

## Performance model

- Peak throughput: **0.049 Gops/s** (carrier_nat_tight_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_tight_copypatch | 0.045 | 91.4% |
| carrier_nat_tight_interp | 0.022 | 46.0% |
| carrier_nat_tight_stencil | 0.045 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_tight_copypatch | 25113ns | 25113ns | -47.83% |
| carrier_nat_tight_interp | 48133ns | 48133ns | base |
| carrier_nat_tight_stencil | 25143ns | 25143ns | -47.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_tight_interp | 45660ns | base | --- | [44858, 46604] | --- | --- | --- | --- |
| carrier_nat_tight_copypatch | 22976ns | -22674.8ns (-49.7%) | [-24545, -21789]ns | [21892, 23245] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_tight_stencil | 22843ns | -22904.8ns (-50.2%) | [-23942, -21920]ns | [22491, 23021] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_tight_interp | carrier_nat_tight_copypatch | carrier_nat_tight_stencil |
|---|---|---|---|
| 1 | 45718ns | -54.1% | -49.9% |
| 2 | 44662ns | -47.7% | -49.0% |
| 3 | 45612ns | -50.0% | -51.2% |
| 4 | 45709ns | -49.4% | -50.3% |
| 5 | 47491ns | -51.3% | -51.7% |
| 6 | 45053ns | -49.4% | -48.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_tight_copypatch | -0.199 | ok |
| carrier_nat_tight_interp | -0.228 | moderate- |
| carrier_nat_tight_stencil | 0.174 | ok |

**Consistency summary:**

- **carrier_nat_tight_copypatch**: won 6/6, lost 0/6
- **carrier_nat_tight_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 104538.3ns | 22704.4ns | 460.4% | HIGH |
| carrier_nat_tight_interp | 91635.5ns | 45707.4ns | 200.5% | HIGH |
| carrier_nat_tight_stencil | 109065.7ns | 22785.0ns | 478.7% | HIGH |

## Distribution (algo ns)

```
carrier_nat_tight_copypatch (n=6, range 20993.3-23244.8 ns)
  20993.3 |########################################
  21105.9 |
  21218.5 |
  21331.0 |
  21443.6 |
  21556.2 |
  21668.8 |
  21781.3 |
  21893.9 |
  22006.5 |
  22119.0 |
  22231.6 |
  22344.2 |
  22456.8 |
  22569.3 |
  22681.9 |########################################
  22794.5 |########################################
  22907.1 |
  23019.6 |########################################
  23132.2 |########################################
  (0 below, 1 above range)

carrier_nat_tight_interp (n=6, range 44661.7-46604.3 ns)
  44661.7 |####################
  44758.8 |
  44856.0 |
  44953.1 |
  45050.2 |####################
  45147.4 |
  45244.5 |
  45341.6 |
  45438.8 |
  45535.9 |####################
  45633.0 |########################################
  45730.2 |
  45827.3 |
  45924.4 |
  46021.6 |
  46118.7 |
  46215.8 |
  46313.0 |
  46410.1 |
  46507.2 |
  (0 below, 1 above range)

carrier_nat_tight_stencil (n=6, range 22261.2-23020.6 ns)
  22261.2 |########################################
  22299.2 |
  22337.1 |
  22375.1 |
  22413.1 |
  22451.0 |
  22489.0 |
  22527.0 |
  22565.0 |
  22602.9 |
  22640.9 |
  22678.9 |
  22716.8 |########################################
  22754.8 |########################################
  22792.8 |
  22830.8 |
  22868.7 |########################################
  22906.7 |
  22944.7 |########################################
  22982.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_tight_copypatch**: bridge=460.5% of algo (FFI overhead may distort results)
- **carrier_nat_tight_interp**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_nat_tight_stencil**: bridge=478.5% of algo (FFI overhead may distort results)
