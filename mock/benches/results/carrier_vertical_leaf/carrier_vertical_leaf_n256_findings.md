# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (45.94 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 26.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 beats baseline by 43% (significant)

carrier_vert_leaf_vert8 is -19.61 us (43%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 26285.2 ns median (-42.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.75x (fastest 26285.2 ns, slowest 45942.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 48280ns | 48179ns | 47820ns | 48072ns | 48821ns | base |
| carrier_vert_leaf_vert4 | 31091ns | 31098ns | 30413ns | 31028ns | 31524ns | -35.60% |
| carrier_vert_leaf_vert8 | 28435ns | 28525ns | 27958ns | 28419ns | 28696ns | -41.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 45912ns | 45325ns | 46391ns | base | 0.006 |
| carrier_vert_leaf_vert4 | 28745ns | 28036ns | 29225ns | -37.39% | 0.009 |
| carrier_vert_leaf_vert8 | 26195ns | 25743ns | 26450ns | -42.95% | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 437246 | 1842182 | 0.237 | 1.00× |
| carrier_vert_leaf_vert4 | 403712 | 1075268 | 0.375 | 0.92× |
| carrier_vert_leaf_vert8 | 411394 | 821053 | 0.501 | 0.94× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.006 | 56.0% |
| carrier_vert_leaf_vert4 | 0.009 | 89.6% |
| carrier_vert_leaf_vert8 | 0.010 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 48280ns | 48280ns | base |
| carrier_vert_leaf_vert4 | 31091ns | 31091ns | -35.60% |
| carrier_vert_leaf_vert8 | 28435ns | 28435ns | -41.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 45942ns | base | --- | [45402, 46391] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 28719ns | -17125.2ns (-37.3%) | [-17762, -16612]ns | [28292, 29225] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 26285ns | -19607.5ns (-42.7%) | [-20245, -19299]ns | [25849, 26450] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 45325ns | -36.6% | -43.2% |
| 2 | 45919ns | -36.3% | -42.2% |
| 3 | 46015ns | -36.6% | -42.8% |
| 4 | 45479ns | -38.4% | -42.3% |
| 5 | 46768ns | -38.7% | -44.5% |
| 6 | 45965ns | -37.9% | -42.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | -0.291 | moderate- |
| carrier_vert_leaf_vert4 | -0.025 | ok |
| carrier_vert_leaf_vert8 | -0.345 | moderate- |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 92512.0ns | 45911.8ns | 201.5% | HIGH |
| carrier_vert_leaf_vert4 | 99086.3ns | 28745.5ns | 344.7% | HIGH |
| carrier_vert_leaf_vert8 | 104593.7ns | 26194.5ns | 399.3% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 45325.4-46391.1 ns)
  45325.4 |####################
  45378.7 |
  45432.0 |####################
  45485.2 |
  45538.5 |
  45591.8 |
  45645.1 |
  45698.4 |
  45751.7 |
  45804.9 |
  45858.2 |
  45911.5 |####################
  45964.8 |########################################
  46018.1 |
  46071.4 |
  46124.6 |
  46177.9 |
  46231.2 |
  46284.5 |
  46337.8 |
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 28035.8-29224.8 ns)
  28035.8 |########################################
  28095.2 |
  28154.7 |
  28214.1 |
  28273.6 |
  28333.0 |
  28392.5 |
  28452.0 |
  28511.4 |########################################
  28570.8 |
  28630.3 |########################################
  28689.8 |
  28749.2 |########################################
  28808.6 |
  28868.1 |
  28927.5 |
  28987.0 |
  29046.5 |
  29105.9 |
  29165.3 |########################################
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 25742.9-26449.6 ns)
  25742.9 |####################
  25778.2 |
  25813.6 |
  25848.9 |
  25884.2 |
  25919.6 |####################
  25954.9 |
  25990.2 |
  26025.6 |
  26060.9 |
  26096.2 |
  26131.6 |
  26166.9 |
  26202.3 |####################
  26237.6 |
  26272.9 |
  26308.3 |########################################
  26343.6 |
  26378.9 |
  26414.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=201.6% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=334.1% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=398.1% of algo (FFI overhead may distort results)
