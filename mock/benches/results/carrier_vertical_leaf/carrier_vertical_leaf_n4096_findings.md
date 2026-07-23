# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (1.11 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 495.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 dominates: 37% faster than the next best (carrier_vert_leaf_vert4)

carrier_vert_leaf_vert8 (495.42 us) leads carrier_vert_leaf_vert4 (678.09 us) by 37%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_leaf_vert8 beats baseline by 56% (significant)

carrier_vert_leaf_vert8 is -621.18 us (56%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_leaf_scalar is an outlier: 2.3x slower than the field

carrier_vert_leaf_scalar (1.11 ms) is 2.3x the fastest (495.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 495418.9 ns median (-55.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.25x (fastest 495418.9 ns, slowest 1114744.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 1117980ns | 1117302ns | 1112758ns | 1116946ns | 1122141ns | base |
| carrier_vert_leaf_vert4 | 683622ns | 681480ns | 678357ns | 680674ns | 690677ns | -38.85% |
| carrier_vert_leaf_vert8 | 496729ns | 498109ns | 490321ns | 496078ns | 500909ns | -55.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 1115347ns | 1110181ns | 1119379ns | base | 0.004 |
| carrier_vert_leaf_vert4 | 680455ns | 675052ns | 687851ns | -38.99% | 0.006 |
| carrier_vert_leaf_vert8 | 493815ns | 486959ns | 498195ns | -55.73% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 6911103 | 19501924 | 0.354 | 1.00× |
| carrier_vert_leaf_vert4 | 4231731 | 7647469 | 0.553 | 0.61× |
| carrier_vert_leaf_vert8 | 3069373 | 5190930 | 0.591 | 0.44× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.004 | 43.7% |
| carrier_vert_leaf_vert4 | 0.006 | 71.8% |
| carrier_vert_leaf_vert8 | 0.008 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 1117980ns | 1117980ns | base |
| carrier_vert_leaf_vert4 | 683622ns | 683622ns | -38.85% |
| carrier_vert_leaf_vert8 | 496729ns | 496729ns | -55.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 1114745ns | base | --- | [1111918, 1119379] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 678091ns | -435992.1ns (-39.1%) | [-441792, -426893]ns | [675422, 687851] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 495419ns | -621183.9ns (-55.7%) | [-624502, -618913]ns | [487829, 498195] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 1115008ns | -38.9% | -55.6% |
| 2 | 1114482ns | -37.7% | -56.2% |
| 3 | 1110181ns | -39.1% | -56.1% |
| 4 | 1117986ns | -39.1% | -55.5% |
| 5 | 1113656ns | -39.3% | -55.5% |
| 6 | 1120772ns | -39.8% | -55.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | -0.337 | moderate- |
| carrier_vert_leaf_vert4 | -0.071 | ok |
| carrier_vert_leaf_vert8 | 0.165 | ok |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 1118816.9ns | 1115347.5ns | 100.3% | HIGH |
| carrier_vert_leaf_vert4 | 680397.4ns | 680455.0ns | 100.0% | HIGH |
| carrier_vert_leaf_vert8 | 500999.9ns | 493814.6ns | 101.5% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 1110180.8-1119379.4 ns)
  1110180.8 |########################################
  1110640.7 |
  1111100.7 |
  1111560.6 |
  1112020.5 |
  1112480.4 |
  1112940.4 |
  1113400.3 |########################################
  1113860.2 |
  1114320.1 |########################################
  1114780.1 |########################################
  1115240.0 |
  1115699.9 |
  1116159.9 |
  1116619.8 |
  1117079.7 |
  1117539.6 |########################################
  1117999.6 |
  1118459.5 |
  1118919.4 |
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 675051.7-687851.4 ns)
  675051.7 |####################
  675691.7 |########################################
  676331.7 |
  676971.7 |
  677611.6 |
  678251.6 |
  678891.6 |
  679531.6 |
  680171.6 |####################
  680811.6 |
  681451.6 |####################
  682091.6 |
  682731.5 |
  683371.5 |
  684011.5 |
  684651.5 |
  685291.5 |
  685931.5 |
  686571.5 |
  687211.5 |
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 486958.8-498195.4 ns)
  486958.8 |########################################
  487520.6 |
  488082.5 |
  488644.3 |########################################
  489206.1 |
  489768.0 |
  490329.8 |
  490891.6 |
  491453.4 |
  492015.3 |
  492577.1 |
  493138.9 |
  493700.8 |
  494262.6 |
  494824.4 |########################################
  495386.2 |########################################
  495948.1 |
  496509.9 |
  497071.7 |########################################
  497633.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=101.8% of algo (FFI overhead may distort results)
