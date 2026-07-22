# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, leaf profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_leaf_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_leaf_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_leaf_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_leaf_interp has the worst median (9.62 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_leaf_stencil at 3.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_leaf_copypatch beats baseline by 63% (significant)

carrier_nat_leaf_copypatch is -6.08 us (63%) faster than baseline carrier_nat_leaf_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_leaf_interp is an outlier: 2.7x slower than the field

carrier_nat_leaf_interp (9.62 us) is 2.7x the fastest (3.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_leaf_stencil, carrier_nat_leaf_copypatch) are a dead heat (<1%)

carrier_nat_leaf_stencil (3.57 us) and carrier_nat_leaf_copypatch (3.58 us) differ by 0.26%, inside the noise, even though the wider field spreads 169.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_nat_leaf_stencil** at 3566.9 ns median (-62.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.70x (fastest 3566.9 ns, slowest 9618.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 5931ns | 6047ns | 5257ns | 6023ns | 6129ns | -50.97% |
| carrier_nat_leaf_interp | 12098ns | 12156ns | 10465ns | 12115ns | 12888ns | base |
| carrier_nat_leaf_stencil | 6034ns | 6062ns | 5844ns | 6060ns | 6091ns | -50.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 3493ns | 3088ns | 3605ns | -63.74% | 0.073 |
| carrier_nat_leaf_interp | 9634ns | 8301ns | 10347ns | base | 0.027 |
| carrier_nat_leaf_stencil | 3554ns | 3435ns | 3596ns | -63.11% | 0.072 |

## Performance model

- Peak throughput: **0.083 Gops/s** (carrier_nat_leaf_copypatch; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.072 | 86.4% |
| carrier_nat_leaf_interp | 0.027 | 32.1% |
| carrier_nat_leaf_stencil | 0.072 | 86.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_leaf_copypatch | 5931ns | 5931ns | -50.97% |
| carrier_nat_leaf_interp | 12098ns | 12098ns | base |
| carrier_nat_leaf_stencil | 6034ns | 6034ns | -50.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_leaf_interp | 9619ns | base | --- | [8936, 10347] | --- | --- | --- | --- |
| carrier_nat_leaf_copypatch | 3576ns | -6077.3ns (-63.2%) | [-6742, -5604]ns | [3297, 3605] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_leaf_stencil | 3567ns | -6028.2ns (-62.7%) | [-6785, -5428]ns | [3498, 3596] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_leaf_interp | carrier_nat_leaf_copypatch | carrier_nat_leaf_stencil |
|---|---|---|---|
| 1 | 8301ns | -62.8% | -58.6% |
| 2 | 9640ns | -63.6% | -62.8% |
| 3 | 9572ns | -62.6% | -62.7% |
| 4 | 10364ns | -65.3% | -65.6% |
| 5 | 9598ns | -62.7% | -62.4% |
| 6 | 10330ns | -65.0% | -65.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.113 | ok |
| carrier_nat_leaf_interp | -0.038 | ok |
| carrier_nat_leaf_stencil | -0.115 | ok |

**Consistency summary:**

- **carrier_nat_leaf_copypatch**: won 6/6, lost 0/6
- **carrier_nat_leaf_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 84358.3ns | 3492.8ns | 2415.2% | HIGH |
| carrier_nat_leaf_interp | 92426.9ns | 9633.8ns | 959.4% | HIGH |
| carrier_nat_leaf_stencil | 85385.6ns | 3553.5ns | 2402.8% | HIGH |

## Distribution (algo ns)

```
carrier_nat_leaf_copypatch (n=6, range 3088.3-3605.0 ns)
   3088.3 |####################
   3114.1 |
   3140.0 |
   3165.8 |
   3191.6 |
   3217.5 |
   3243.3 |
   3269.1 |
   3295.0 |
   3320.8 |
   3346.7 |
   3372.5 |
   3398.3 |
   3424.2 |
   3450.0 |
   3475.8 |
   3501.7 |####################
   3527.5 |
   3553.3 |########################################
   3579.2 |####################
  (0 below, 1 above range)

carrier_nat_leaf_interp (n=6, range 8300.8-10346.7 ns)
   8300.8 |####################
   8403.1 |
   8505.4 |
   8607.7 |
   8710.0 |
   8812.3 |
   8914.6 |
   9016.8 |
   9119.1 |
   9221.4 |
   9323.7 |
   9426.0 |
   9528.3 |########################################
   9630.6 |####################
   9732.9 |
   9835.2 |
   9937.5 |
  10039.8 |
  10142.1 |
  10244.4 |####################
  (0 below, 1 above range)

carrier_nat_leaf_stencil (n=6, range 3435.0-3596.2 ns)
   3435.0 |####################
   3443.1 |
   3451.1 |
   3459.2 |
   3467.2 |
   3475.3 |
   3483.4 |
   3491.4 |
   3499.5 |
   3507.6 |
   3515.6 |
   3523.7 |
   3531.8 |
   3539.8 |
   3547.9 |
   3555.9 |########################################
   3564.0 |####################
   3572.1 |
   3580.1 |####################
   3588.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_leaf_copypatch**: bridge=2371.7% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_interp**: bridge=961.4% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_stencil**: bridge=2390.1% of algo (FFI overhead may distort results)
