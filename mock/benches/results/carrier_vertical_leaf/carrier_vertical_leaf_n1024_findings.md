# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (335.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 109.77 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 dominates: 11% faster than the next best (carrier_vert_leaf_vert4)

carrier_vert_leaf_vert8 (109.77 us) leads carrier_vert_leaf_vert4 (122.32 us) by 11%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_leaf_vert8 beats baseline by 68% (significant)

carrier_vert_leaf_vert8 is -228.10 us (68%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_leaf_scalar is an outlier: 3.1x slower than the field

carrier_vert_leaf_scalar (335.11 us) is 3.1x the fastest (109.77 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_vert_leaf_vert8 (109.77 us) to slowest carrier_vert_leaf_scalar (335.11 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 109767.7 ns median (-67.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.05x (fastest 109767.7 ns, slowest 335105.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 340686ns | 338201ns | 326429ns | 334363ns | 357299ns | base |
| carrier_vert_leaf_vert4 | 125140ns | 124996ns | 120727ns | 123913ns | 129188ns | -63.27% |
| carrier_vert_leaf_vert8 | 110948ns | 112304ns | 105917ns | 111276ns | 112973ns | -67.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 337689ns | 323145ns | 354257ns | base | 0.003 |
| carrier_vert_leaf_vert4 | 122519ns | 118558ns | 126465ns | -63.72% | 0.008 |
| carrier_vert_leaf_vert8 | 108482ns | 103666ns | 110417ns | -67.88% | 0.009 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.003 | 30.9% |
| carrier_vert_leaf_vert4 | 0.008 | 84.7% |
| carrier_vert_leaf_vert8 | 0.009 | 94.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 340686ns | 340686ns | base |
| carrier_vert_leaf_vert4 | 125140ns | 125140ns | -63.27% |
| carrier_vert_leaf_vert8 | 110948ns | 110948ns | -67.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 335106ns | base | --- | [323703, 354257] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 122325ns | -213443.8ns (-63.7%) | [-231716, -200348]ns | [118768, 126465] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 109768ns | -228097.9ns (-68.1%) | [-244317, -215206]ns | [105260, 110417] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 354525ns | -66.1% | -69.1% |
| 2 | 353989ns | -64.8% | -68.8% |
| 3 | 327723ns | -63.7% | -66.3% |
| 4 | 323145ns | -60.3% | -66.9% |
| 5 | 342488ns | -63.7% | -69.7% |
| 6 | 324261ns | -63.4% | -66.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | 0.115 | ok |
| carrier_vert_leaf_vert4 | -0.408 | moderate- |
| carrier_vert_leaf_vert8 | 0.065 | ok |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 337108.8ns | 337688.5ns | 99.8% | HIGH |
| carrier_vert_leaf_vert4 | 122071.7ns | 122519.2ns | 99.6% | HIGH |
| carrier_vert_leaf_vert8 | 109990.6ns | 108481.5ns | 101.4% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 323145.0-354256.9 ns)
  323145.0 |########################################
  324700.6 |
  326256.2 |####################
  327811.8 |
  329367.4 |
  330923.0 |
  332478.6 |
  334034.2 |
  335589.8 |
  337145.4 |
  338701.0 |
  340256.5 |
  341812.1 |####################
  343367.7 |
  344923.3 |
  346478.9 |
  348034.5 |
  349590.1 |
  351145.7 |
  352701.3 |####################
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 118558.3-126464.6 ns)
  118558.3 |########################################
  118953.6 |########################################
  119348.9 |
  119744.2 |
  120139.6 |########################################
  120534.9 |
  120930.2 |
  121325.5 |
  121720.8 |
  122116.1 |
  122511.5 |
  122906.8 |
  123302.1 |
  123697.4 |
  124092.7 |########################################
  124488.0 |########################################
  124883.3 |
  125278.7 |
  125674.0 |
  126069.3 |
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 103665.8-110417.3 ns)
  103665.8 |####################
  104003.4 |
  104340.9 |
  104678.5 |
  105016.1 |
  105353.7 |
  105691.2 |
  106028.8 |
  106366.4 |
  106704.0 |####################
  107041.6 |
  107379.1 |
  107716.7 |
  108054.3 |
  108391.9 |
  108729.4 |
  109067.0 |####################
  109404.6 |
  109742.2 |
  110079.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=101.1% of algo (FFI overhead may distort results)
