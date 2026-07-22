# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, real profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_real_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_real_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_real_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_real_interp has the worst median (44.81 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_real_stencil at 15.60 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_real_stencil beats baseline by 65% (significant)

carrier_nat_real_stencil is -29.33 us (65%) faster than baseline carrier_nat_real_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_real_interp is an outlier: 2.9x slower than the field

carrier_nat_real_interp (44.81 us) is 2.9x the fastest (15.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_real_stencil, carrier_nat_real_copypatch) are a dead heat (<1%)

carrier_nat_real_stencil (15.60 us) and carrier_nat_real_copypatch (15.65 us) differ by 0.34%, inside the noise, even though the wider field spreads 187.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_nat_real_stencil** at 15598.5 ns median (-65.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.87x (fastest 15598.5 ns, slowest 44814.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 17819ns | 18118ns | 16386ns | 17828ns | 18522ns | -64.07% |
| carrier_nat_real_interp | 49596ns | 47304ns | 43470ns | 47165ns | 56306ns | base |
| carrier_nat_real_stencil | 18058ns | 18002ns | 17605ns | 17932ns | 18475ns | -63.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 15407ns | 14222ns | 16025ns | -67.24% | 0.066 |
| carrier_nat_real_interp | 47032ns | 41051ns | 53582ns | base | 0.022 |
| carrier_nat_real_stencil | 15623ns | 15234ns | 15966ns | -66.78% | 0.066 |

## Performance model

- Peak throughput: **0.072 Gops/s** (carrier_nat_real_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_real_copypatch | 0.065 | 90.9% |
| carrier_nat_real_interp | 0.023 | 31.7% |
| carrier_nat_real_stencil | 0.066 | 91.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_real_copypatch | 17819ns | 17819ns | -64.07% |
| carrier_nat_real_interp | 49596ns | 49596ns | base |
| carrier_nat_real_stencil | 18058ns | 18058ns | -63.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_real_interp | 44815ns | base | --- | [42700, 53582] | --- | --- | --- | --- |
| carrier_nat_real_copypatch | 15652ns | -28963.9ns (-64.6%) | [-39038, -26874]ns | [14544, 16025] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_real_stencil | 15599ns | -29333.3ns (-65.5%) | [-37878, -27017]ns | [15304, 15966] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_real_interp | carrier_nat_real_copypatch | carrier_nat_real_stencil |
|---|---|---|---|
| 1 | 45258ns | -68.6% | -65.5% |
| 2 | 44605ns | -64.1% | -65.0% |
| 3 | 44349ns | -65.3% | -63.9% |
| 4 | 41051ns | -61.3% | -62.5% |
| 5 | 45024ns | -64.4% | -66.2% |
| 6 | 61906ns | -76.0% | -74.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_real_copypatch | -0.278 | moderate- |
| carrier_nat_real_interp | 0.032 | ok |
| carrier_nat_real_stencil | -0.277 | moderate- |

**Consistency summary:**

- **carrier_nat_real_copypatch**: won 6/6, lost 0/6
- **carrier_nat_real_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 91525.3ns | 15406.9ns | 594.1% | HIGH |
| carrier_nat_real_interp | 107692.7ns | 47032.3ns | 229.0% | HIGH |
| carrier_nat_real_stencil | 101327.0ns | 15622.9ns | 648.6% | HIGH |

## Distribution (algo ns)

```
carrier_nat_real_copypatch (n=6, range 14222.5-16025.0 ns)
  14222.5 |########################################
  14312.6 |
  14402.8 |
  14492.9 |
  14583.0 |
  14673.1 |
  14763.2 |
  14853.4 |########################################
  14943.5 |
  15033.6 |
  15123.8 |
  15213.9 |
  15304.0 |
  15394.1 |########################################
  15484.2 |
  15574.4 |
  15664.5 |
  15754.6 |
  15844.8 |########################################
  15934.9 |########################################
  (0 below, 1 above range)

carrier_nat_real_interp (n=6, range 41050.8-53582.2 ns)
  41050.8 |####################
  41677.4 |
  42303.9 |
  42930.5 |
  43557.1 |
  44183.7 |########################################
  44810.2 |########################################
  45436.8 |
  46063.4 |
  46690.0 |
  47316.5 |
  47943.1 |
  48569.7 |
  49196.2 |
  49822.8 |
  50449.4 |
  51076.0 |
  51702.5 |
  52329.1 |
  52955.7 |
  (0 below, 1 above range)

carrier_nat_real_stencil (n=6, range 15233.8-15966.5 ns)
  15233.8 |########################################
  15270.4 |
  15307.1 |
  15343.7 |########################################
  15380.3 |
  15417.0 |
  15453.6 |
  15490.2 |
  15526.9 |
  15563.5 |########################################
  15600.1 |########################################
  15636.8 |
  15673.4 |
  15710.0 |
  15746.7 |
  15783.3 |
  15819.9 |
  15856.6 |
  15893.2 |
  15929.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_real_copypatch**: bridge=593.2% of algo (FFI overhead may distort results)
- **carrier_nat_real_interp**: bridge=232.1% of algo (FFI overhead may distort results)
- **carrier_nat_real_stencil**: bridge=652.0% of algo (FFI overhead may distort results)
