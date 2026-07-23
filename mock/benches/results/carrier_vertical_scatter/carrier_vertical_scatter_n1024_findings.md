# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (294.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 117.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 dominates: 133% faster than the next best (carrier_vert_scatter_vert4)

carrier_vert_scatter_vert8 (117.42 us) leads carrier_vert_scatter_vert4 (273.24 us) by 133%, a clear separation rather than a photo finish. CV 18.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_scatter_vert8 beats baseline by 57% (significant)

carrier_vert_scatter_vert8 is -168.61 us (57%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 2.5x slower than the field

carrier_vert_scatter_scalar (294.51 us) is 2.5x the fastest (117.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_scatter_vert8 is fastest but the noisiest (CV 18.5%)

carrier_vert_scatter_vert8 wins on median (117.42 us) yet has the highest variance (CV 18.5%), while carrier_vert_scatter_vert4 is the steadiest (CV 2.2%, 273.24 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 117421.2 ns median (-60.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.51x (fastest 117421.2 ns, slowest 294511.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 296310ns | 296774ns | 286330ns | 293866ns | 304967ns | base |
| carrier_vert_scatter_vert4 | 277547ns | 275910ns | 270892ns | 274602ns | 285292ns | -6.33% |
| carrier_vert_scatter_vert8 | 131977ns | 119879ns | 115112ns | 118661ns | 160383ns | -55.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 293986ns | 284214ns | 302710ns | base | 0.003 |
| carrier_vert_scatter_vert4 | 274966ns | 268330ns | 282636ns | -6.47% | 0.004 |
| carrier_vert_scatter_vert8 | 129408ns | 112494ns | 157641ns | -55.98% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 1844929 | 5309015 | 0.348 | 1.00× |
| carrier_vert_scatter_vert4 | 1662305 | 2032632 | 0.818 | 0.90× |
| carrier_vert_scatter_vert8 | 784914 | 1437578 | 0.546 | 0.43× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.003 | 38.2% |
| carrier_vert_scatter_vert4 | 0.004 | 41.2% |
| carrier_vert_scatter_vert8 | 0.009 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 296310ns | 296310ns | base |
| carrier_vert_scatter_vert4 | 277547ns | 277547ns | -6.33% |
| carrier_vert_scatter_vert8 | 131977ns | 131977ns | -55.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 294511ns | base | --- | [284735, 302710] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 273235ns | -16027.3ns (-5.4%) | [-33682, -7348]ns | [269028, 282636] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 117421ns | -168607.5ns (-57.2%) | [-183684, -141440]ns | [113163, 157641] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 296970ns | -3.7% | -51.8% |
| 2 | 292053ns | -7.2% | -60.6% |
| 3 | 284214ns | -1.7% | -60.4% |
| 4 | 285257ns | -3.5% | -58.0% |
| 5 | 304227ns | -11.8% | -62.6% |
| 6 | 301193ns | -10.4% | -42.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | 0.243 | moderate+ |
| carrier_vert_scatter_vert4 | -0.113 | ok |
| carrier_vert_scatter_vert8 | -0.108 | ok |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 296472.8ns | 293985.6ns | 100.8% | HIGH |
| carrier_vert_scatter_vert4 | 257924.9ns | 274966.4ns | 93.8% | HIGH |
| carrier_vert_scatter_vert8 | 127306.4ns | 129408.3ns | 98.4% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 284214.2-302710.0 ns)
  284214.2 |########################################
  285139.0 |########################################
  286063.8 |
  286988.6 |
  287913.4 |
  288838.2 |
  289762.9 |
  290687.7 |
  291612.5 |########################################
  292537.3 |
  293462.1 |
  294386.9 |
  295311.7 |
  296236.5 |########################################
  297161.3 |
  298086.0 |
  299010.8 |
  299935.6 |
  300860.4 |########################################
  301785.2 |
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 268330.0-282635.8 ns)
  268330.0 |########################################
  269045.3 |########################################
  269760.6 |
  270475.9 |########################################
  271191.2 |
  271906.5 |
  272621.8 |
  273337.0 |
  274052.3 |
  274767.6 |########################################
  275482.9 |
  276198.2 |
  276913.5 |
  277628.8 |
  278344.1 |
  279059.4 |########################################
  279774.7 |
  280490.0 |
  281205.3 |
  281920.6 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 112494.2-157641.0 ns)
  112494.2 |########################################
  114751.5 |####################
  117008.9 |
  119266.2 |####################
  121523.6 |
  123780.9 |
  126038.3 |
  128295.6 |
  130552.9 |
  132810.3 |
  135067.6 |
  137325.0 |
  139582.3 |
  141839.7 |####################
  144097.0 |
  146354.3 |
  148611.7 |
  150869.0 |
  153126.4 |
  155383.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=94.6% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: bridge=105.2% of algo (FFI overhead may distort results)
