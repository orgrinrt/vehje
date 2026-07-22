# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (6.66 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 2.19 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 dominates: 25% faster than the next best (carrier_vert_tight_vert4)

carrier_vert_tight_vert8 (2.19 ms) leads carrier_vert_tight_vert4 (2.73 ms) by 25%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_tight_vert8 beats baseline by 67% (significant)

carrier_vert_tight_vert8 is -4.46 ms (67%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 3.0x slower than the field

carrier_vert_tight_scalar (6.66 ms) is 3.0x the fastest (2.19 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_tight_vert8 is fastest but the noisiest (CV 6.4%)

carrier_vert_tight_vert8 wins on median (2.19 ms) yet has the highest variance (CV 6.4%), while carrier_vert_tight_scalar is the steadiest (CV 0.2%, 6.66 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.0x the fastest

Fastest carrier_vert_tight_vert8 (2.19 ms) to slowest carrier_vert_tight_scalar (6.66 ms): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 2188596.0 ns median (-67.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.04x (fastest 2188596.0 ns, slowest 6656467.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 6655658ns | 6660072ns | 6641588ns | 6654482ns | 6664458ns | base |
| carrier_vert_tight_vert4 | 2732791ns | 2733871ns | 2722879ns | 2731942ns | 2739020ns | -58.94% |
| carrier_vert_tight_vert8 | 2252723ns | 2191997ns | 2184982ns | 2190380ns | 2380109ns | -66.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 6651964ns | 6637213ns | 6661296ns | base | 0.002 |
| carrier_vert_tight_vert4 | 2729441ns | 2718803ns | 2735767ns | -58.97% | 0.006 |
| carrier_vert_tight_vert8 | 2249452ns | 2181826ns | 2376885ns | -66.18% | 0.007 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.002 | 32.8% |
| carrier_vert_tight_vert4 | 0.006 | 79.9% |
| carrier_vert_tight_vert8 | 0.007 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 6655658ns | 6655658ns | base |
| carrier_vert_tight_vert4 | 2732791ns | 2732791ns | -58.94% |
| carrier_vert_tight_vert8 | 2252723ns | 2252723ns | -66.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 6656468ns | base | --- | [6638128, 6661296] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 2730712ns | -3923035.0ns (-58.9%) | [-3934546, -3909989]ns | [2721842, 2735767] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 2188596ns | -4460296.1ns (-67.0%) | [-4471776, -4275464]ns | [2182876, 2376885] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 6637213ns | -58.8% | -67.0% |
| 2 | 6639044ns | -59.0% | -67.1% |
| 3 | 6663471ns | -59.1% | -61.6% |
| 4 | 6656632ns | -58.9% | -67.2% |
| 5 | 6659122ns | -59.1% | -67.1% |
| 6 | 6656303ns | -58.9% | -67.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | 0.263 | moderate+ |
| carrier_vert_tight_vert4 | -0.500 | moderate- |
| carrier_vert_tight_vert8 | -0.261 | moderate- |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 6650847.8ns | 6651964.1ns | 100.0% | HIGH |
| carrier_vert_tight_vert4 | 2786349.2ns | 2729440.5ns | 102.1% | HIGH |
| carrier_vert_tight_vert8 | 2283410.5ns | 2249452.1ns | 101.5% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 6637212.9-6661296.2 ns)
  6637212.9 |########################################
  6638417.1 |########################################
  6639621.2 |
  6640825.4 |
  6642029.6 |
  6643233.7 |
  6644437.9 |
  6645642.1 |
  6646846.2 |
  6648050.4 |
  6649254.6 |
  6650458.7 |
  6651662.9 |
  6652867.1 |
  6654071.2 |
  6655275.4 |########################################
  6656479.6 |########################################
  6657683.7 |
  6658887.9 |########################################
  6660092.1 |
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 2718803.3-2735767.3 ns)
  2718803.3 |########################################
  2719651.5 |
  2720499.7 |
  2721347.9 |
  2722196.1 |
  2723044.3 |
  2723892.5 |
  2724740.7 |########################################
  2725588.9 |
  2726437.1 |
  2727285.3 |
  2728133.5 |########################################
  2728981.7 |
  2729829.9 |
  2730678.1 |
  2731526.3 |
  2732374.5 |########################################
  2733222.7 |########################################
  2734070.9 |
  2734919.1 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 2181826.2-2376884.5 ns)
  2181826.2 |########################################
  2191579.1 |##########
  2201332.0 |
  2211085.0 |
  2220837.9 |
  2230590.8 |
  2240343.7 |
  2250096.6 |
  2259849.5 |
  2269602.5 |
  2279355.4 |
  2289108.3 |
  2298861.2 |
  2308614.1 |
  2318367.0 |
  2328120.0 |
  2337872.9 |
  2347625.8 |
  2357378.7 |
  2367131.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=102.1% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: bridge=101.3% of algo (FFI overhead may distort results)
