# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (5.14 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 2.22 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 dominates: 25% faster than the next best (carrier_vert_tight_vert4)

carrier_vert_tight_vert8 (2.22 ms) leads carrier_vert_tight_vert4 (2.78 ms) by 25%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_tight_vert8 beats baseline by 57% (significant)

carrier_vert_tight_vert8 is -2.91 ms (57%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 2.3x slower than the field

carrier_vert_tight_scalar (5.14 ms) is 2.3x the fastest (2.22 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 2223242.9 ns median (-56.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.31x (fastest 2223242.9 ns, slowest 5136151.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 5138579ns | 5139411ns | 5120809ns | 5136095ns | 5151192ns | base |
| carrier_vert_tight_vert4 | 2793758ns | 2780836ns | 2762802ns | 2778335ns | 2832372ns | -45.63% |
| carrier_vert_tight_vert8 | 2245943ns | 2226995ns | 2207277ns | 2224961ns | 2296749ns | -56.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 5135308ns | 5117675ns | 5147692ns | base | 0.003 |
| carrier_vert_tight_vert4 | 2789802ns | 2759066ns | 2827677ns | -45.67% | 0.006 |
| carrier_vert_tight_vert8 | 2242332ns | 2203828ns | 2293048ns | -56.33% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 32086470 | 84610023 | 0.379 | 1.00× |
| carrier_vert_tight_vert4 | 17245268 | 31963194 | 0.540 | 0.54× |
| carrier_vert_tight_vert8 | 13916016 | 22550655 | 0.617 | 0.43× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.003 | 42.9% |
| carrier_vert_tight_vert4 | 0.006 | 79.4% |
| carrier_vert_tight_vert8 | 0.007 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 5138579ns | 5138579ns | base |
| carrier_vert_tight_vert4 | 2793758ns | 2793758ns | -45.63% |
| carrier_vert_tight_vert8 | 2245943ns | 2245943ns | -56.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 5136151ns | base | --- | [5122081, 5147692] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 2777280ns | -2353093.4ns (-45.8%) | [-2383242, -2300182]ns | [2764450, 2827677] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 2223243ns | -2907055.4ns (-56.6%) | [-2933009, -2838863]ns | [2210706, 2293048] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 5149237ns | -46.4% | -56.8% |
| 2 | 5146148ns | -46.2% | -54.7% |
| 3 | 5117675ns | -45.7% | -55.9% |
| 4 | 5126487ns | -44.2% | -56.7% |
| 5 | 5127393ns | -45.9% | -56.7% |
| 6 | 5144910ns | -45.7% | -57.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | 0.128 | ok |
| carrier_vert_tight_vert4 | -0.155 | ok |
| carrier_vert_tight_vert8 | 0.052 | ok |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 5211740.7ns | 5135308.2ns | 101.5% | HIGH |
| carrier_vert_tight_vert4 | 2786538.3ns | 2789802.5ns | 99.9% | HIGH |
| carrier_vert_tight_vert8 | 2266213.7ns | 2242332.5ns | 101.1% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 5117674.6-5147692.3 ns)
  5117674.6 |####################
  5119175.5 |
  5120676.4 |
  5122177.3 |
  5123678.1 |
  5125179.0 |####################
  5126679.9 |####################
  5128180.8 |
  5129681.7 |
  5131182.6 |
  5132683.4 |
  5134184.3 |
  5135685.2 |
  5137186.1 |
  5138687.0 |
  5140187.9 |
  5141688.8 |
  5143189.6 |
  5144690.5 |########################################
  5146191.4 |
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 2759065.8-2827677.0 ns)
  2759065.8 |########################################
  2762496.4 |
  2765926.9 |
  2769357.5 |########################################
  2772788.0 |########################################
  2776218.6 |
  2779649.2 |########################################
  2783079.7 |
  2786510.3 |
  2789940.9 |########################################
  2793371.4 |
  2796802.0 |
  2800232.5 |
  2803663.1 |
  2807093.7 |
  2810524.2 |
  2813954.8 |
  2817385.4 |
  2820815.9 |
  2824246.5 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 2203828.3-2293048.5 ns)
  2203828.3 |####################
  2208289.3 |
  2212750.3 |
  2217211.3 |####################
  2221672.3 |########################################
  2226133.3 |
  2230594.4 |
  2235055.4 |
  2239516.4 |
  2243977.4 |
  2248438.4 |
  2252899.4 |####################
  2257360.4 |
  2261821.4 |
  2266282.4 |
  2270743.5 |
  2275204.5 |
  2279665.5 |
  2284126.5 |
  2288587.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=101.5% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: bridge=101.0% of algo (FFI overhead may distort results)
