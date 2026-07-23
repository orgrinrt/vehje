# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (17.06 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 3.56 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 dominates: 57% faster than the next best (carrier_vert_scatter_vert4)

carrier_vert_scatter_vert8 (3.56 ms) leads carrier_vert_scatter_vert4 (5.58 ms) by 57%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_scatter_vert8 beats baseline by 79% (significant)

carrier_vert_scatter_vert8 is -13.51 ms (79%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 4.8x slower than the field

carrier_vert_scatter_scalar (17.06 ms) is 4.8x the fastest (3.56 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest carrier_vert_scatter_vert8 (3.56 ms) to slowest carrier_vert_scatter_scalar (17.06 ms): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 3556316.0 ns median (-79.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.80x (fastest 3556316.0 ns, slowest 17062805.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 17158612ns | 17066976ns | 16990258ns | 17061233ns | 17388858ns | base |
| carrier_vert_scatter_vert4 | 5590475ns | 5581489ns | 5572656ns | 5579646ns | 5615628ns | -67.42% |
| carrier_vert_scatter_vert8 | 3558164ns | 3559921ns | 3541172ns | 3557947ns | 3566987ns | -79.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 17154363ns | 16986002ns | 17384452ns | base | 0.001 |
| carrier_vert_scatter_vert4 | 5586565ns | 5568892ns | 5611686ns | -67.43% | 0.003 |
| carrier_vert_scatter_vert8 | 3554637ns | 3538189ns | 3563315ns | -79.28% | 0.005 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 105748665 | 84847263 | 1.246 | 1.00× |
| carrier_vert_scatter_vert4 | 34610471 | 32143559 | 1.077 | 0.33× |
| carrier_vert_scatter_vert8 | 22368702 | 22597142 | 0.990 | 0.21× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.001 | 20.7% |
| carrier_vert_scatter_vert4 | 0.003 | 63.4% |
| carrier_vert_scatter_vert8 | 0.005 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 17158612ns | 17158612ns | base |
| carrier_vert_scatter_vert4 | 5590475ns | 5590475ns | -67.42% |
| carrier_vert_scatter_vert8 | 3558164ns | 3558164ns | -79.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 17062805ns | base | --- | [17015831, 17384452] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 5577486ns | -11490652.7ns (-67.3%) | [-11772767, -11439976]ns | [5570522, 5611686] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 3556316ns | -13508741.7ns (-79.2%) | [-13837919, -13452517]ns | [3544281, 3563315] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 17639444ns | -68.1% | -79.8% |
| 2 | 17068707ns | -67.4% | -79.2% |
| 3 | 17045660ns | -67.3% | -79.1% |
| 4 | 17056903ns | -67.3% | -79.1% |
| 5 | 16986002ns | -67.2% | -79.0% |
| 6 | 17129460ns | -67.3% | -79.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | -0.004 | ok |
| carrier_vert_scatter_vert4 | -0.108 | ok |
| carrier_vert_scatter_vert8 | -0.202 | moderate- |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 17155718.2ns | 17154362.9ns | 100.0% | HIGH |
| carrier_vert_scatter_vert4 | 5588862.1ns | 5586564.6ns | 100.0% | HIGH |
| carrier_vert_scatter_vert8 | 3646901.6ns | 3554637.3ns | 102.6% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 16986002.5-17384452.3 ns)
  16986002.5 |########################################
  17005925.0 |
  17025847.5 |########################################
  17045770.0 |########################################
  17065692.5 |########################################
  17085614.9 |
  17105537.4 |
  17125459.9 |########################################
  17145382.4 |
  17165304.9 |
  17185227.4 |
  17205149.9 |
  17225072.4 |
  17244994.9 |
  17264917.4 |
  17284839.8 |
  17304762.3 |
  17324684.8 |
  17344607.3 |
  17364529.8 |
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 5568892.1-5611685.6 ns)
  5568892.1 |########################################
  5571031.8 |########################################
  5573171.4 |
  5575311.1 |########################################
  5577450.8 |########################################
  5579590.5 |
  5581730.1 |
  5583869.8 |
  5586009.5 |
  5588149.2 |
  5590288.8 |
  5592428.5 |
  5594568.2 |
  5596707.9 |
  5598847.5 |
  5600987.2 |########################################
  5603126.9 |
  5605266.6 |
  5607406.2 |
  5609545.9 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 3538188.7-3563314.5 ns)
  3538188.7 |########################################
  3539445.0 |
  3540701.3 |
  3541957.6 |
  3543213.9 |
  3544470.2 |
  3545726.5 |
  3546982.7 |
  3548239.0 |
  3549495.3 |########################################
  3550751.6 |
  3552007.9 |
  3553264.2 |
  3554520.5 |########################################
  3555776.8 |
  3557033.1 |########################################
  3558289.4 |
  3559545.7 |########################################
  3560802.0 |
  3562058.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: bridge=102.6% of algo (FFI overhead may distort results)
