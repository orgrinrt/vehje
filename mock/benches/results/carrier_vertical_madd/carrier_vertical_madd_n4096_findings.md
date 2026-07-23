# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (1.53 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 578.40 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 46% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (578.40 us) leads carrier_vert_madd_vert4 (845.37 us) by 46%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 62% (significant)

carrier_vert_madd_vert8 is -949.81 us (62%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.6x slower than the field

carrier_vert_madd_scalar (1.53 ms) is 2.6x the fastest (578.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_madd_scalar shows alternating (throttle bounce) (autocorr -0.52)

carrier_vert_madd_scalar's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 578402.2 ns median (-62.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.64x (fastest 578402.2 ns, slowest 1527420.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 1528824ns | 1530013ns | 1516795ns | 1529525ns | 1533787ns | base |
| carrier_vert_madd_vert4 | 850533ns | 847854ns | 843650ns | 847110ns | 859109ns | -44.37% |
| carrier_vert_madd_vert8 | 582618ns | 580750ns | 579375ns | 580368ns | 587614ns | -61.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 1526272ns | 1514181ns | 1531364ns | base | 0.003 |
| carrier_vert_madd_vert4 | 848056ns | 841228ns | 856590ns | -44.44% | 0.005 |
| carrier_vert_madd_vert8 | 580297ns | 577133ns | 585299ns | -61.98% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 9488368 | 21173423 | 0.448 | 1.00× |
| carrier_vert_madd_vert4 | 5283198 | 8682376 | 0.608 | 0.56× |
| carrier_vert_madd_vert8 | 3633552 | 6328500 | 0.574 | 0.38× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.003 | 37.8% |
| carrier_vert_madd_vert4 | 0.005 | 68.3% |
| carrier_vert_madd_vert8 | 0.007 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 1528824ns | 1528824ns | base |
| carrier_vert_madd_vert4 | 850533ns | 850533ns | -44.37% |
| carrier_vert_madd_vert8 | 582618ns | 582618ns | -61.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 1527421ns | base | --- | [1520031, 1531364] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 845374ns | -678827.7ns (-44.4%) | [-688582, -667237]ns | [842204, 856590] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 578402ns | -949812.9ns (-62.2%) | [-952962, -935150]ns | [577189, 585299] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 1525880ns | -43.7% | -62.2% |
| 2 | 1526717ns | -44.1% | -61.3% |
| 3 | 1528125ns | -44.8% | -62.2% |
| 4 | 1533448ns | -45.1% | -62.2% |
| 5 | 1514181ns | -44.1% | -61.7% |
| 6 | 1529280ns | -44.8% | -62.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | -0.519 | HIGH- (thermal bounce) |
| carrier_vert_madd_vert4 | 0.357 | moderate+ |
| carrier_vert_madd_vert8 | -0.466 | moderate- |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 1538739.7ns | 1526271.7ns | 100.8% | HIGH |
| carrier_vert_madd_vert4 | 850406.3ns | 848056.3ns | 100.3% | HIGH |
| carrier_vert_madd_vert8 | 584542.5ns | 580296.8ns | 100.7% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 1514181.2-1531363.9 ns)
  1514181.2 |########################################
  1515040.3 |
  1515899.5 |
  1516758.6 |
  1517617.8 |
  1518476.9 |
  1519336.0 |
  1520195.2 |
  1521054.3 |
  1521913.4 |
  1522772.6 |
  1523631.7 |
  1524490.8 |
  1525350.0 |########################################
  1526209.1 |########################################
  1527068.3 |
  1527927.4 |########################################
  1528786.5 |########################################
  1529645.7 |
  1530504.8 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 841228.3-856590.2 ns)
  841228.3 |########################################
  841996.4 |
  842764.5 |########################################
  843532.6 |
  844300.7 |########################################
  845068.8 |
  845836.9 |########################################
  846605.0 |
  847373.1 |
  848141.2 |
  848909.2 |
  849677.3 |
  850445.4 |
  851213.5 |
  851981.6 |
  852749.7 |
  853517.8 |########################################
  854285.9 |
  855054.0 |
  855822.1 |
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 577132.9-585298.9 ns)
  577132.9 |########################################
  577541.2 |####################
  577949.5 |
  578357.8 |
  578766.1 |
  579174.4 |####################
  579582.7 |
  579991.0 |
  580399.3 |####################
  580807.6 |
  581215.9 |
  581624.2 |
  582032.5 |
  582440.8 |
  582849.1 |
  583257.4 |
  583665.7 |
  584074.0 |
  584482.3 |
  584890.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=100.9% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=100.7% of algo (FFI overhead may distort results)
