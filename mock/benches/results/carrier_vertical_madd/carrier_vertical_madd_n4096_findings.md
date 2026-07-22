# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (1.57 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 575.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 46% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (575.07 us) leads carrier_vert_madd_vert4 (838.25 us) by 46%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 63% (significant)

carrier_vert_madd_vert8 is -998.46 us (63%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.7x slower than the field

carrier_vert_madd_scalar (1.57 ms) is 2.7x the fastest (575.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 575070.0 ns median (-63.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.74x (fastest 575070.0 ns, slowest 1573591.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 1578510ns | 1577258ns | 1575040ns | 1576792ns | 1582822ns | base |
| carrier_vert_madd_vert4 | 841901ns | 841926ns | 840411ns | 841499ns | 843250ns | -46.66% |
| carrier_vert_madd_vert8 | 578589ns | 578208ns | 573890ns | 578029ns | 581778ns | -63.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 1574892ns | 1571299ns | 1579252ns | base | 0.003 |
| carrier_vert_madd_vert4 | 838405ns | 836685ns | 840114ns | -46.76% | 0.005 |
| carrier_vert_madd_vert8 | 575424ns | 570635ns | 578784ns | -63.46% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.003 | 36.3% |
| carrier_vert_madd_vert4 | 0.005 | 68.1% |
| carrier_vert_madd_vert8 | 0.007 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 1578510ns | 1578510ns | base |
| carrier_vert_madd_vert4 | 841901ns | 841901ns | -46.66% |
| carrier_vert_madd_vert8 | 578589ns | 578589ns | -63.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 1573592ns | base | --- | [1571833, 1579252] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 838253ns | -734635.0ns (-46.7%) | [-741589, -733237]ns | [836849, 840114] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 575070ns | -998457.6ns (-63.5%) | [-1006834, -993113]ns | [572418, 578784] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 1572366ns | -46.6% | -63.0% |
| 2 | 1571299ns | -46.7% | -63.4% |
| 3 | 1573769ns | -46.8% | -63.5% |
| 4 | 1583106ns | -47.1% | -64.0% |
| 5 | 1573415ns | -46.6% | -63.4% |
| 6 | 1575398ns | -46.7% | -63.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | -0.099 | ok |
| carrier_vert_madd_vert4 | 0.235 | moderate+ |
| carrier_vert_madd_vert8 | 0.009 | ok |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 1574631.3ns | 1574892.2ns | 100.0% | HIGH |
| carrier_vert_madd_vert4 | 838201.7ns | 838405.2ns | 100.0% | HIGH |
| carrier_vert_madd_vert8 | 578553.2ns | 575423.9ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 1571299.2-1579252.0 ns)
  1571299.2 |########################################
  1571696.8 |
  1572094.5 |########################################
  1572492.1 |
  1572889.8 |
  1573287.4 |########################################
  1573685.1 |########################################
  1574082.7 |
  1574480.3 |
  1574878.0 |
  1575275.6 |########################################
  1575673.3 |
  1576070.9 |
  1576468.6 |
  1576866.2 |
  1577263.8 |
  1577661.5 |
  1578059.1 |
  1578456.8 |
  1578854.4 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 836684.6-840113.8 ns)
  836684.6 |########################################
  836856.1 |########################################
  837027.5 |########################################
  837199.0 |
  837370.4 |
  837541.9 |
  837713.3 |
  837884.8 |
  838056.3 |
  838227.7 |
  838399.2 |
  838570.6 |
  838742.1 |
  838913.5 |
  839085.0 |
  839256.5 |
  839427.9 |########################################
  839599.4 |
  839770.8 |########################################
  839942.3 |
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 570635.0-578783.9 ns)
  570635.0 |####################
  571042.4 |
  571449.9 |
  571857.3 |
  572264.8 |
  572672.2 |
  573079.7 |
  573487.1 |
  573894.6 |####################
  574302.0 |
  574709.5 |########################################
  575116.9 |####################
  575524.4 |
  575931.8 |
  576339.3 |
  576746.7 |
  577154.2 |
  577561.6 |
  577969.1 |
  578376.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=100.6% of algo (FFI overhead may distort results)
