# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (1.47 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 512.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 dominates: 24% faster than the next best (carrier_vert_tight_vert4)

carrier_vert_tight_vert8 (512.34 us) leads carrier_vert_tight_vert4 (637.60 us) by 24%, a clear separation rather than a photo finish. CV 0.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_tight_vert8 beats baseline by 65% (significant)

carrier_vert_tight_vert8 is -962.14 us (65%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 2.9x slower than the field

carrier_vert_tight_scalar (1.47 ms) is 2.9x the fastest (512.34 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 512337.1 ns median (-65.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.88x (fastest 512337.1 ns, slowest 1473656.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 1477145ns | 1477450ns | 1461745ns | 1475770ns | 1486909ns | base |
| carrier_vert_tight_vert4 | 642305ns | 640922ns | 639034ns | 640454ns | 646718ns | -56.52% |
| carrier_vert_tight_vert8 | 515610ns | 515613ns | 514980ns | 515450ns | 516165ns | -65.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 1473505ns | 1457814ns | 1483328ns | base | 0.003 |
| carrier_vert_tight_vert4 | 639140ns | 635595ns | 643444ns | -56.62% | 0.006 |
| carrier_vert_tight_vert8 | 512219ns | 511435ns | 512806ns | -65.24% | 0.008 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.003 | 34.7% |
| carrier_vert_tight_vert4 | 0.006 | 80.2% |
| carrier_vert_tight_vert8 | 0.008 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 1477145ns | 1477145ns | base |
| carrier_vert_tight_vert4 | 642305ns | 642305ns | -56.52% |
| carrier_vert_tight_vert8 | 515610ns | 515610ns | -65.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 1473656ns | base | --- | [1463531, 1483328] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 637597ns | -836873.5ns (-56.8%) | [-845483, -820738]ns | [636380, 643444] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 512337ns | -962143.1ns (-65.3%) | [-970647, -951068]ns | [511513, 512806] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 1473454ns | -56.7% | -65.3% |
| 2 | 1457814ns | -55.5% | -64.9% |
| 3 | 1473858ns | -56.9% | -65.3% |
| 4 | 1469248ns | -56.6% | -65.1% |
| 5 | 1476645ns | -56.8% | -65.3% |
| 6 | 1490011ns | -57.2% | -65.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | 0.059 | ok |
| carrier_vert_tight_vert4 | -0.293 | moderate- |
| carrier_vert_tight_vert8 | 0.069 | ok |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 1472308.9ns | 1473504.9ns | 99.9% | HIGH |
| carrier_vert_tight_vert4 | 645712.5ns | 639140.1ns | 101.0% | HIGH |
| carrier_vert_tight_vert8 | 518170.7ns | 512218.8ns | 101.2% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 1457813.7-1483327.7 ns)
  1457813.7 |####################
  1459089.4 |
  1460365.1 |
  1461640.8 |
  1462916.5 |
  1464192.2 |
  1465467.9 |
  1466743.6 |
  1468019.3 |####################
  1469295.0 |
  1470570.7 |
  1471846.4 |
  1473122.1 |########################################
  1474397.8 |
  1475673.5 |####################
  1476949.2 |
  1478224.9 |
  1479500.6 |
  1480776.3 |
  1482052.0 |
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 635594.6-643443.7 ns)
  635594.6 |####################
  635987.1 |
  636379.5 |
  636772.0 |
  637164.4 |########################################
  637556.9 |
  637949.3 |####################
  638341.8 |####################
  638734.2 |
  639126.7 |
  639519.1 |
  639911.6 |
  640304.1 |
  640696.5 |
  641089.0 |
  641481.4 |
  641873.9 |
  642266.3 |
  642658.8 |
  643051.2 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 511435.0-512806.2 ns)
  511435.0 |########################################
  511503.6 |
  511572.1 |########################################
  511640.7 |
  511709.2 |
  511777.8 |
  511846.4 |
  511914.9 |
  511983.5 |
  512052.1 |########################################
  512120.6 |
  512189.2 |
  512257.8 |
  512326.3 |
  512394.9 |
  512463.4 |
  512532.0 |########################################
  512600.6 |
  512669.1 |
  512737.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=101.1% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: bridge=101.1% of algo (FFI overhead may distort results)
