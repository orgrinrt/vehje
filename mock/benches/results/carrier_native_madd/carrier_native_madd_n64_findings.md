# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, madd profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_madd_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_madd_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_madd_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_madd_interp has the worst median (2.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_madd_copypatch at 1.01 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_madd_copypatch beats baseline by 58% (significant)

carrier_nat_madd_copypatch is -1.41 us (58%) faster than baseline carrier_nat_madd_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_madd_interp is an outlier: 2.4x slower than the field

carrier_nat_madd_interp (2.41 us) is 2.4x the fastest (1.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_nat_madd_copypatch** at 1007.9 ns median (-58.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.39x (fastest 1007.9 ns, slowest 2412.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 3289ns | 3251ns | 3211ns | 3241ns | 3399ns | -33.47% |
| carrier_nat_madd_interp | 4943ns | 4758ns | 4614ns | 4744ns | 5407ns | base |
| carrier_nat_madd_stencil | 3563ns | 3422ns | 3307ns | 3384ns | 3961ns | -27.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 1021ns | 999ns | 1056ns | -59.30% | 0.063 |
| carrier_nat_madd_interp | 2510ns | 2338ns | 2774ns | base | 0.026 |
| carrier_nat_madd_stencil | 1079ns | 1014ns | 1184ns | -57.01% | 0.059 |

## Performance model

- Peak throughput: **0.064 Gops/s** (carrier_nat_madd_copypatch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_madd_copypatch | 0.063 | 99.1% |
| carrier_nat_madd_interp | 0.027 | 41.4% |
| carrier_nat_madd_stencil | 0.062 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_madd_copypatch | 3289ns | 3289ns | -33.47% |
| carrier_nat_madd_interp | 4943ns | 4943ns | base |
| carrier_nat_madd_stencil | 3563ns | 3563ns | -27.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_madd_interp | 2413ns | base | --- | [2341, 2774] | --- | --- | --- | --- |
| carrier_nat_madd_copypatch | 1008ns | -1410.6ns (-58.5%) | [-1718, -1335]ns | [1000, 1056] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_madd_stencil | 1031ns | -1345.8ns (-55.8%) | [-1648, -1299]ns | [1021, 1184] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_madd_interp | carrier_nat_madd_copypatch | carrier_nat_madd_stencil |
|---|---|---|---|
| 1 | 2338ns | -56.8% | -56.0% |
| 2 | 2393ns | -58.0% | -56.9% |
| 3 | 2345ns | -57.3% | -56.8% |
| 4 | 2987ns | -65.0% | -63.4% |
| 5 | 2562ns | -58.3% | -50.2% |
| 6 | 2432ns | -58.9% | -57.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_madd_copypatch | 0.021 | ok |
| carrier_nat_madd_interp | -0.061 | ok |
| carrier_nat_madd_stencil | -0.046 | ok |

**Consistency summary:**

- **carrier_nat_madd_copypatch**: won 6/6, lost 0/6
- **carrier_nat_madd_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 77126.0ns | 1021.5ns | 7550.5% | HIGH |
| carrier_nat_madd_interp | 87547.8ns | 2509.5ns | 3488.6% | HIGH |
| carrier_nat_madd_stencil | 79939.0ns | 1078.7ns | 7410.4% | HIGH |

## Distribution (algo ns)

```
carrier_nat_madd_copypatch (n=6, range 999.2-1056.0 ns)
    999.2 |########################################
   1002.0 |
   1004.9 |####################
   1007.7 |
   1010.6 |####################
   1013.4 |
   1016.3 |
   1019.1 |
   1021.9 |
   1024.8 |
   1027.6 |
   1030.5 |
   1033.3 |
   1036.2 |
   1039.0 |
   1041.8 |####################
   1044.7 |
   1047.5 |
   1050.4 |
   1053.2 |
  (0 below, 1 above range)

carrier_nat_madd_interp (n=6, range 2338.3-2774.4 ns)
   2338.3 |########################################
   2360.1 |
   2381.9 |####################
   2403.7 |
   2425.5 |####################
   2447.3 |
   2469.1 |
   2490.9 |
   2512.7 |
   2534.5 |
   2556.3 |####################
   2578.2 |
   2600.0 |
   2621.8 |
   2643.6 |
   2665.4 |
   2687.2 |
   2709.0 |
   2730.8 |
   2752.6 |
  (0 below, 1 above range)

carrier_nat_madd_stencil (n=6, range 1013.7-1183.8 ns)
   1013.7 |####################
   1022.2 |########################################
   1030.7 |####################
   1039.2 |
   1047.7 |
   1056.2 |
   1064.7 |
   1073.2 |
   1081.7 |
   1090.2 |####################
   1098.7 |
   1107.2 |
   1115.7 |
   1124.2 |
   1132.7 |
   1141.2 |
   1149.7 |
   1158.2 |
   1166.7 |
   1175.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_madd_copypatch**: bridge=7656.0% of algo (FFI overhead may distort results)
- **carrier_nat_madd_interp**: bridge=3579.6% of algo (FFI overhead may distort results)
- **carrier_nat_madd_stencil**: bridge=7730.5% of algo (FFI overhead may distort results)
