# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (1.74 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 484.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 dominates: 38% faster than the next best (carrier_vert_leaf_vert4)

carrier_vert_leaf_vert8 (484.27 us) leads carrier_vert_leaf_vert4 (668.30 us) by 38%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_leaf_vert8 beats baseline by 72% (significant)

carrier_vert_leaf_vert8 is -1.26 ms (72%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_leaf_scalar is an outlier: 3.6x slower than the field

carrier_vert_leaf_scalar (1.74 ms) is 3.6x the fastest (484.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.6x the fastest

Fastest carrier_vert_leaf_vert8 (484.27 us) to slowest carrier_vert_leaf_scalar (1.74 ms): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 484267.2 ns median (-72.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.60x (fastest 484267.2 ns, slowest 1741696.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 1763719ns | 1745582ns | 1724639ns | 1742597ns | 1814942ns | base |
| carrier_vert_leaf_vert4 | 671989ns | 671696ns | 670555ns | 671428ns | 673549ns | -61.90% |
| carrier_vert_leaf_vert8 | 486931ns | 487743ns | 481706ns | 487159ns | 489200ns | -72.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 1759715ns | 1720649ns | 1810857ns | base | 0.002 |
| carrier_vert_leaf_vert4 | 668463ns | 666686ns | 670179ns | -62.01% | 0.006 |
| carrier_vert_leaf_vert8 | 483652ns | 478062ns | 486361ns | -72.52% | 0.008 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.002 | 27.4% |
| carrier_vert_leaf_vert4 | 0.006 | 71.5% |
| carrier_vert_leaf_vert8 | 0.008 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 1763719ns | 1763719ns | base |
| carrier_vert_leaf_vert4 | 671989ns | 671989ns | -61.90% |
| carrier_vert_leaf_vert8 | 486931ns | 486931ns | -72.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 1741697ns | base | --- | [1726592, 1810857] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 668302ns | -1073837.7ns (-61.7%) | [-1142105, -1057814]ns | [666908, 670179] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 484267ns | -1260588.8ns (-72.4%) | [-1325681, -1241922]ns | [480327, 486361] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 1750617ns | -61.8% | -72.7% |
| 2 | 1803433ns | -62.9% | -73.0% |
| 3 | 1732776ns | -61.3% | -72.1% |
| 4 | 1818282ns | -63.2% | -73.5% |
| 5 | 1732536ns | -61.5% | -72.0% |
| 6 | 1720649ns | -61.3% | -71.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | -0.438 | moderate- |
| carrier_vert_leaf_vert4 | 0.228 | moderate+ |
| carrier_vert_leaf_vert8 | -0.411 | moderate- |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 1758882.9ns | 1759715.5ns | 100.0% | HIGH |
| carrier_vert_leaf_vert4 | 667824.8ns | 668463.4ns | 99.9% | HIGH |
| carrier_vert_leaf_vert8 | 486757.9ns | 483651.6ns | 100.6% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 1720648.8-1810857.3 ns)
  1720648.8 |####################
  1725159.2 |
  1729669.6 |########################################
  1734180.1 |
  1738690.5 |
  1743200.9 |
  1747711.4 |####################
  1752221.8 |
  1756732.2 |
  1761242.6 |
  1765753.0 |
  1770263.5 |
  1774773.9 |
  1779284.3 |
  1783794.8 |
  1788305.2 |
  1792815.6 |
  1797326.0 |
  1801836.4 |####################
  1806346.9 |
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 666686.2-670179.3 ns)
  666686.2 |########################################
  666860.9 |
  667035.5 |########################################
  667210.2 |
  667384.8 |
  667559.5 |
  667734.1 |
  667908.8 |
  668083.5 |########################################
  668258.1 |########################################
  668432.8 |
  668607.4 |
  668782.1 |
  668956.7 |
  669131.4 |########################################
  669306.1 |
  669480.7 |
  669655.4 |
  669830.0 |
  670004.7 |
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 478062.5-486360.8 ns)
  478062.5 |########################################
  478477.4 |
  478892.3 |
  479307.2 |
  479722.2 |
  480137.1 |
  480552.0 |
  480966.9 |
  481381.8 |
  481796.7 |
  482211.7 |########################################
  482626.6 |
  483041.5 |
  483456.4 |
  483871.3 |########################################
  484286.2 |########################################
  484701.1 |########################################
  485116.1 |
  485531.0 |
  485945.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=100.5% of algo (FFI overhead may distort results)
