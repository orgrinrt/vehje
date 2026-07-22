# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (80.53 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 31.97 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 dominates: 28% faster than the next best (carrier_vert_tight_vert4)

carrier_vert_tight_vert8 (31.97 us) leads carrier_vert_tight_vert4 (40.84 us) by 28%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_tight_vert8 beats baseline by 61% (significant)

carrier_vert_tight_vert8 is -49.16 us (61%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 2.5x slower than the field

carrier_vert_tight_scalar (80.53 us) is 2.5x the fastest (31.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_tight_vert8 shows warm-up / thermal drift (autocorr +0.59)

carrier_vert_tight_vert8's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 31968.8 ns median (-60.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.52x (fastest 31968.8 ns, slowest 80526.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 83348ns | 82920ns | 80572ns | 82528ns | 85966ns | base |
| carrier_vert_tight_vert4 | 43745ns | 43399ns | 41943ns | 43095ns | 45623ns | -47.51% |
| carrier_vert_tight_vert8 | 34376ns | 34428ns | 32920ns | 33932ns | 35771ns | -58.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 80871ns | 78131ns | 83270ns | base | 0.003 |
| carrier_vert_tight_vert4 | 41259ns | 39568ns | 43087ns | -48.98% | 0.006 |
| carrier_vert_tight_vert8 | 31893ns | 30583ns | 33112ns | -60.56% | 0.008 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.003 | 38.0% |
| carrier_vert_tight_vert4 | 0.006 | 74.9% |
| carrier_vert_tight_vert8 | 0.008 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 83348ns | 83348ns | base |
| carrier_vert_tight_vert4 | 43745ns | 43745ns | -47.51% |
| carrier_vert_tight_vert8 | 34376ns | 34376ns | -58.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 80526ns | base | --- | [78817, 83270] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 40844ns | -39499.4ns (-49.1%) | [-42426, -36911]ns | [39845, 43087] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 31969ns | -49164.4ns (-61.1%) | [-50713, -47057]ns | [30597, 33112] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 80495ns | -46.9% | -62.0% |
| 2 | 79502ns | -45.4% | -61.5% |
| 3 | 80557ns | -50.2% | -61.4% |
| 4 | 78131ns | -49.4% | -58.0% |
| 5 | 81978ns | -50.9% | -59.5% |
| 6 | 84561ns | -51.0% | -60.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | 0.117 | ok |
| carrier_vert_tight_vert4 | 0.346 | moderate+ |
| carrier_vert_tight_vert8 | 0.593 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 151056.6ns | 80870.9ns | 186.8% | HIGH |
| carrier_vert_tight_vert4 | 118708.9ns | 41258.6ns | 287.7% | HIGH |
| carrier_vert_tight_vert8 | 95943.6ns | 31892.8ns | 300.8% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 78131.2-83269.8 ns)
  78131.2 |####################
  78388.1 |
  78645.1 |
  78902.0 |
  79158.9 |
  79415.8 |####################
  79672.8 |
  79929.7 |
  80186.6 |
  80443.5 |########################################
  80700.5 |
  80957.4 |
  81214.3 |
  81471.3 |
  81728.2 |####################
  81985.1 |
  82242.0 |
  82499.0 |
  82755.9 |
  83012.8 |
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 39567.9-43087.3 ns)
  39567.9 |####################
  39743.9 |
  39919.8 |
  40095.8 |########################################
  40271.8 |
  40447.8 |
  40623.7 |
  40799.7 |
  40975.7 |
  41151.6 |
  41327.6 |####################
  41503.6 |
  41679.5 |
  41855.5 |
  42031.5 |
  42207.5 |
  42383.4 |
  42559.4 |
  42735.4 |####################
  42911.3 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 30582.9-33112.5 ns)
  30582.9 |########################################
  30709.4 |
  30835.9 |
  30962.3 |
  31088.8 |####################
  31215.3 |
  31341.8 |
  31468.3 |
  31594.7 |
  31721.2 |
  31847.7 |
  31974.2 |
  32100.7 |
  32227.1 |
  32353.6 |
  32480.1 |
  32606.6 |
  32733.1 |####################
  32859.5 |
  32986.0 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=193.9% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=292.7% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **carrier_vert_tight_vert8**: bridge=301.1% of algo (FFI overhead may distort results)
