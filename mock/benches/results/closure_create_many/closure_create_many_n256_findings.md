# Closure representation: create-many-call-once, flat vs linked (creation cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_create_many_flat**

## Highlights

Baseline for all deltas below: **closure_create_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (closure_create_many_flat) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline closure_create_many_flat has the worst median (39.72 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest closure_create_many_linked at 18.60 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### closure_create_many_linked dominates: 114% faster than the next best (closure_create_many_flat)

closure_create_many_linked (18.60 us) leads closure_create_many_flat (39.72 us) by 114%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### closure_create_many_linked beats baseline by 53% (significant)

closure_create_many_linked is -21.17 us (53%) faster than baseline closure_create_many_flat, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: closure_create_many_linked** at 18601.8 ns median (-53.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.14x (fastest 18601.8 ns, slowest 39720.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_create_many_flat | 42154ns | 42045ns | 40038ns | 41404ns | 44336ns | base |
| closure_create_many_linked | 20910ns | 20914ns | 19920ns | 20690ns | 21736ns | -50.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_create_many_flat | 39820ns | 37800ns | 41872ns | base | 0.006 |
| closure_create_many_linked | 18633ns | 17776ns | 19372ns | -53.21% | 0.014 |

## Performance model

- Peak throughput: **0.014 Gops/s** (closure_create_many_linked; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_create_many_flat | 0.006 | 44.8% |
| closure_create_many_linked | 0.014 | 95.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_create_many_flat | 42154ns | 42154ns | base |
| closure_create_many_linked | 20910ns | 20910ns | -50.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_create_many_flat | 39721ns | base | --- | [37868, 41872] | --- | --- | --- | --- |
| closure_create_many_linked | 18602ns | -21174.6ns (-53.3%) | [-22854, -19532]ns | [17925, 19372] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_create_many_flat | closure_create_many_linked |
|---|---|---|
| 1 | 40340ns | -55.9% |
| 2 | 41877ns | -55.3% |
| 3 | 39102ns | -52.4% |
| 4 | 41868ns | -52.2% |
| 5 | 37800ns | -52.2% |
| 6 | 37935ns | -51.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_create_many_flat | -0.131 | ok |
| closure_create_many_linked | -0.294 | moderate- |

**Consistency summary:**

- **closure_create_many_linked**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_create_many_flat | 5.1ns | 39820.3ns | 0.0% |  |
| closure_create_many_linked | 3.8ns | 18633.2ns | 0.0% |  |

## Distribution (algo ns)

```
closure_create_many_flat (n=6, range 37800.0-41872.5 ns)
  37800.0 |########################################
  38003.6 |
  38207.2 |
  38410.9 |
  38614.5 |
  38818.1 |
  39021.8 |####################
  39225.4 |
  39429.0 |
  39632.6 |
  39836.2 |
  40039.9 |
  40243.5 |####################
  40447.1 |
  40650.8 |
  40854.4 |
  41058.0 |
  41261.6 |
  41465.2 |
  41668.9 |####################
  (0 below, 1 above range)

closure_create_many_linked (n=6, range 17776.2-19372.3 ns)
  17776.2 |####################
  17856.0 |
  17935.8 |
  18015.6 |####################
  18095.4 |
  18175.2 |
  18255.0 |
  18334.8 |
  18414.6 |
  18494.4 |
  18574.2 |########################################
  18654.1 |####################
  18733.9 |
  18813.7 |
  18893.5 |
  18973.3 |
  19053.1 |
  19132.9 |
  19212.7 |
  19292.5 |
  (0 below, 1 above range)

```
