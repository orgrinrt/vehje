# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (3.86 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 826.41 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 dominates: 62% faster than the next best (carrier_vert_scatter_vert4)

carrier_vert_scatter_vert8 (826.41 us) leads carrier_vert_scatter_vert4 (1.34 ms) by 62%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_scatter_vert8 beats baseline by 79% (significant)

carrier_vert_scatter_vert8 is -3.03 ms (79%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 4.7x slower than the field

carrier_vert_scatter_scalar (3.86 ms) is 4.7x the fastest (826.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.7x the fastest

Fastest carrier_vert_scatter_vert8 (826.41 us) to slowest carrier_vert_scatter_scalar (3.86 ms): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 826410.8 ns median (-78.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.67x (fastest 826410.8 ns, slowest 3855332.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 3848997ns | 3858276ns | 3788675ns | 3856352ns | 3868126ns | base |
| carrier_vert_scatter_vert4 | 1344087ns | 1342069ns | 1321077ns | 1341454ns | 1359542ns | -65.08% |
| carrier_vert_scatter_vert8 | 826573ns | 829607ns | 817023ns | 825502ns | 832953ns | -78.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 3846042ns | 3785835ns | 3865053ns | base | 0.001 |
| carrier_vert_scatter_vert4 | 1340971ns | 1317605ns | 1356540ns | -65.13% | 0.003 |
| carrier_vert_scatter_vert8 | 823227ns | 813775ns | 829450ns | -78.60% | 0.005 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 23968981 | 21211442 | 1.130 | 1.00× |
| carrier_vert_scatter_vert4 | 8326587 | 8046548 | 1.035 | 0.35× |
| carrier_vert_scatter_vert8 | 5220941 | 5660513 | 0.922 | 0.22× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.001 | 21.1% |
| carrier_vert_scatter_vert4 | 0.003 | 60.8% |
| carrier_vert_scatter_vert8 | 0.005 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 3848997ns | 3848997ns | base |
| carrier_vert_scatter_vert4 | 1344087ns | 1344087ns | -65.08% |
| carrier_vert_scatter_vert8 | 826573ns | 826573ns | -78.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 3855333ns | base | --- | [3817741, 3865053] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 1339310ns | -2506024.5ns (-65.0%) | [-2536569, -2472621]ns | [1327063, 1356540] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 826411ns | -3026730.0ns (-78.5%) | [-3051232, -2990484]ns | [813821, 829450] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 3861078ns | -65.3% | -78.9% |
| 2 | 3785835ns | -64.6% | -78.1% |
| 3 | 3852710ns | -64.9% | -78.6% |
| 4 | 3857956ns | -64.8% | -78.5% |
| 5 | 3849648ns | -65.3% | -78.5% |
| 6 | 3869028ns | -65.9% | -79.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | -0.241 | moderate- |
| carrier_vert_scatter_vert4 | 0.225 | moderate+ |
| carrier_vert_scatter_vert8 | -0.045 | ok |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 3873559.0ns | 3846042.4ns | 100.7% | HIGH |
| carrier_vert_scatter_vert4 | 1342032.7ns | 1340970.7ns | 100.1% | HIGH |
| carrier_vert_scatter_vert8 | 843076.0ns | 823227.2ns | 102.4% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 3785834.6-3865052.9 ns)
  3785834.6 |####################
  3789795.5 |
  3793756.4 |
  3797717.3 |
  3801678.3 |
  3805639.2 |
  3809600.1 |
  3813561.0 |
  3817521.9 |
  3821482.8 |
  3825443.8 |
  3829404.7 |
  3833365.6 |
  3837326.5 |
  3841287.4 |
  3845248.3 |
  3849209.2 |########################################
  3853170.2 |
  3857131.1 |########################################
  3861092.0 |
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 1317605.4-1356539.6 ns)
  1317605.4 |####################
  1319552.1 |
  1321498.8 |
  1323445.5 |
  1325392.2 |
  1327338.9 |
  1329285.7 |
  1331232.4 |
  1333179.1 |
  1335125.8 |####################
  1337072.5 |
  1339019.2 |########################################
  1340965.9 |
  1342912.6 |
  1344859.3 |
  1346806.1 |
  1348752.8 |
  1350699.5 |
  1352646.2 |####################
  1354592.9 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 813774.6-829449.6 ns)
  813774.6 |########################################
  814558.3 |
  815342.1 |
  816125.8 |
  816909.6 |
  817693.3 |
  818477.1 |
  819260.8 |
  820044.6 |
  820828.3 |
  821612.1 |
  822395.8 |
  823179.6 |
  823963.3 |
  824747.1 |
  825530.8 |####################
  826314.6 |####################
  827098.3 |####################
  827882.1 |
  828665.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=100.7% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: bridge=101.7% of algo (FFI overhead may distort results)
