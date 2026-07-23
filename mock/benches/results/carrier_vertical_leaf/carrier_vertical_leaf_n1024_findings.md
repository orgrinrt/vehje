# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (194.37 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 107.04 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 dominates: 12% faster than the next best (carrier_vert_leaf_vert4)

carrier_vert_leaf_vert8 (107.04 us) leads carrier_vert_leaf_vert4 (119.41 us) by 12%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_leaf_vert8 beats baseline by 45% (significant)

carrier_vert_leaf_vert8 is -86.68 us (45%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 107042.5 ns median (-44.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.82x (fastest 107042.5 ns, slowest 194366.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 197484ns | 196599ns | 194128ns | 196388ns | 200806ns | base |
| carrier_vert_leaf_vert4 | 121529ns | 121848ns | 119106ns | 121365ns | 122986ns | -38.46% |
| carrier_vert_leaf_vert8 | 110414ns | 109394ns | 107675ns | 109178ns | 113638ns | -44.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 195235ns | 191944ns | 198499ns | base | 0.005 |
| carrier_vert_leaf_vert4 | 119149ns | 116964ns | 120492ns | -38.97% | 0.009 |
| carrier_vert_leaf_vert8 | 107933ns | 105078ns | 111100ns | -44.72% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 1230404 | 4882742 | 0.252 | 1.00× |
| carrier_vert_leaf_vert4 | 748264 | 1917189 | 0.390 | 0.61× |
| carrier_vert_leaf_vert8 | 669764 | 1304173 | 0.514 | 0.54× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.005 | 54.1% |
| carrier_vert_leaf_vert4 | 0.009 | 88.0% |
| carrier_vert_leaf_vert8 | 0.010 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 197484ns | 197484ns | base |
| carrier_vert_leaf_vert4 | 121529ns | 121529ns | -38.46% |
| carrier_vert_leaf_vert8 | 110414ns | 110414ns | -44.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 194366ns | base | --- | [192840, 198499] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 119406ns | -75587.8ns (-38.9%) | [-80302, -72368]ns | [117551, 120492] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 107042ns | -86677.7ns (-44.6%) | [-89578, -85651]ns | [105656, 111100] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 198544ns | -41.1% | -43.0% |
| 2 | 193737ns | -38.4% | -44.6% |
| 3 | 193775ns | -37.5% | -45.2% |
| 4 | 198454ns | -39.8% | -46.2% |
| 5 | 191944ns | -37.5% | -45.3% |
| 6 | 194957ns | -39.4% | -44.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | -0.469 | moderate- |
| carrier_vert_leaf_vert4 | -0.013 | ok |
| carrier_vert_leaf_vert8 | -0.018 | ok |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 202981.2ns | 195235.2ns | 104.0% | HIGH |
| carrier_vert_leaf_vert4 | 119869.4ns | 119149.4ns | 100.6% | HIGH |
| carrier_vert_leaf_vert8 | 108691.7ns | 107932.9ns | 100.7% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 191943.7-198499.0 ns)
  191943.7 |####################
  192271.5 |
  192599.2 |
  192927.0 |
  193254.8 |
  193582.5 |########################################
  193910.3 |
  194238.1 |
  194565.8 |
  194893.6 |####################
  195221.4 |
  195549.1 |
  195876.9 |
  196204.6 |
  196532.4 |
  196860.2 |
  197187.9 |
  197515.7 |
  197843.5 |
  198171.2 |####################
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 116964.2-120491.5 ns)
  116964.2 |####################
  117140.6 |
  117316.9 |
  117493.3 |
  117669.7 |
  117846.0 |
  118022.4 |####################
  118198.8 |
  118375.1 |
  118551.5 |
  118727.9 |
  118904.2 |
  119080.6 |
  119256.9 |########################################
  119433.3 |
  119609.7 |
  119786.0 |####################
  119962.4 |
  120138.8 |
  120315.1 |
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 105078.3-111099.6 ns)
  105078.3 |########################################
  105379.4 |
  105680.4 |
  105981.5 |########################################
  106282.6 |
  106583.6 |########################################
  106884.7 |
  107185.8 |########################################
  107486.8 |
  107787.9 |
  108089.0 |
  108390.0 |
  108691.1 |
  108992.1 |########################################
  109293.2 |
  109594.3 |
  109895.3 |
  110196.4 |
  110497.5 |
  110798.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=103.6% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=101.0% of algo (FFI overhead may distort results)
