# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vr_static) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vr_static has the worst median (40.39 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vr_nanbox at 33.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: carrier_vr_nanbox** at 33228.3 ns median (-17.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.22x (fastest 33228.3 ns, slowest 40393.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 35685ns | 35763ns | 33795ns | 35625ns | 36720ns | -16.30% |
| carrier_vr_static | 42634ns | 42919ns | 40943ns | 42811ns | 43213ns | base |
| carrier_vr_tagged | 38340ns | 38565ns | 36378ns | 38300ns | 39382ns | -10.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 33173ns | 31445ns | 34115ns | -17.33% | 0.031 |
| carrier_vr_static | 40128ns | 38589ns | 40687ns | base | 0.026 |
| carrier_vr_tagged | 36029ns | 34166ns | 37036ns | -10.21% | 0.028 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_vr_nanbox; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.031 | 94.6% |
| carrier_vr_static | 0.025 | 77.8% |
| carrier_vr_tagged | 0.028 | 86.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 35685ns | 35685ns | -16.30% |
| carrier_vr_static | 42634ns | 42634ns | base |
| carrier_vr_tagged | 38340ns | 38340ns | -10.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 40394ns | base | --- | [39302, 40687] | --- | --- | --- | --- |
| carrier_vr_nanbox | 33228ns | -7265.4ns (-18.0%) | [-8217, -5380]ns | [32177, 34115] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vr_tagged | 36227ns | -3986.5ns (-9.9%) | [-4909, -3400]ns | [34825, 37036] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 38589ns | -13.7% | -11.5% |
| 2 | 40495ns | -18.1% | -9.2% |
| 3 | 40878ns | -17.6% | -13.2% |
| 4 | 40378ns | -22.1% | -7.6% |
| 5 | 40410ns | -18.6% | -10.5% |
| 6 | 40015ns | -13.7% | -9.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | -0.153 | ok |
| carrier_vr_static | -0.020 | ok |
| carrier_vr_tagged | -0.378 | moderate- |

**Consistency summary:**

- **carrier_vr_nanbox**: won 6/6, lost 0/6
- **carrier_vr_tagged**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 99707.7ns | 33173.3ns | 300.6% | HIGH |
| carrier_vr_static | 120265.1ns | 40127.6ns | 299.7% | HIGH |
| carrier_vr_tagged | 108259.3ns | 36029.3ns | 300.5% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 31445.4-34115.0 ns)
  31445.4 |########################################
  31578.9 |
  31712.4 |
  31845.8 |
  31979.3 |
  32112.8 |
  32246.3 |
  32379.8 |
  32513.2 |
  32646.7 |
  32780.2 |########################################
  32913.7 |
  33047.2 |########################################
  33180.6 |########################################
  33314.1 |
  33447.6 |
  33581.1 |########################################
  33714.6 |
  33848.0 |
  33981.5 |
  (0 below, 1 above range)

carrier_vr_static (n=6, range 38589.2-40686.7 ns)
  38589.2 |####################
  38694.1 |
  38798.9 |
  38903.8 |
  39008.7 |
  39113.6 |
  39218.4 |
  39323.3 |
  39428.2 |
  39533.1 |
  39637.9 |
  39742.8 |
  39847.7 |
  39952.5 |####################
  40057.4 |
  40162.3 |
  40267.2 |
  40372.0 |########################################
  40476.9 |####################
  40581.8 |
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 34165.8-37036.1 ns)
  34165.8 |########################################
  34309.3 |
  34452.8 |
  34596.3 |
  34739.9 |
  34883.4 |
  35026.9 |
  35170.4 |
  35313.9 |
  35457.4 |########################################
  35600.9 |
  35744.4 |
  35888.0 |
  36031.5 |########################################
  36175.0 |########################################
  36318.5 |
  36462.0 |
  36605.5 |
  36749.0 |########################################
  36892.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: bridge=300.7% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=299.4% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: bridge=300.8% of algo (FFI overhead may distort results)
