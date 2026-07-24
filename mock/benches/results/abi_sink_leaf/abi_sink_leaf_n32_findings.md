# abi_sink (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_sink_leaf_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_leaf_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_leaf_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_leaf_null_sink has the worst median (1.45 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_leaf_batched_sink at 1.45 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 0.3% of the fastest

All 4 variants sit between 1.45 ms and 1.45 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_leaf_batched_sink** at 1445070.4 ns median (-0.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 1445070.4 ns, slowest 1449855.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1447609ns | 1447483ns | 1444660ns | 1446955ns | 1450064ns | -0.37% |
| abi_sink_leaf_batched_sink_decode | 1452190ns | 1451547ns | 1448159ns | 1450947ns | 1456068ns | -0.06% |
| abi_sink_leaf_null_sink | 1452995ns | 1452308ns | 1445617ns | 1451589ns | 1458793ns | base |
| abi_sink_leaf_per_record_sink | 1451754ns | 1451418ns | 1449910ns | 1451320ns | 1453328ns | -0.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 1445155ns | 1442205ns | 1447562ns | -0.37% | 0.000 |
| abi_sink_leaf_batched_sink_decode | 1449634ns | 1445714ns | 1453392ns | -0.06% | 0.000 |
| abi_sink_leaf_null_sink | 1450481ns | 1443269ns | 1456122ns | base | 0.000 |
| abi_sink_leaf_per_record_sink | 1449323ns | 1447466ns | 1450908ns | -0.08% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 35263.8 | 1445076.4 | 1445155.3 | n/a |
| abi_sink_leaf_batched_sink_decode | 37552.6 | 1450222.8 | 1449634.0 | 2 |
| abi_sink_leaf_null_sink | 36051.5 | 1450524.4 | 1450481.2 | n/a |
| abi_sink_leaf_per_record_sink | 35289.1 | 1449095.6 | 1449323.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_leaf_batched_sink; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.000 | 99.8% |
| abi_sink_leaf_batched_sink_decode | 0.000 | 99.5% |
| abi_sink_leaf_null_sink | 0.000 | 99.5% |
| abi_sink_leaf_per_record_sink | 0.000 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_leaf_batched_sink | 1447609ns | 1447609ns | -0.37% |
| abi_sink_leaf_batched_sink_decode | 1452190ns | 1452190ns | -0.06% |
| abi_sink_leaf_null_sink | 1452995ns | 1452995ns | base |
| abi_sink_leaf_per_record_sink | 1451754ns | 1451754ns | -0.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_leaf_null_sink | 1449856ns | base | --- | [1445465, 1456122] | --- | --- | --- | --- |
| abi_sink_leaf_batched_sink | 1445070ns | -3304.4ns (-0.2%) | [-12605, -69]ns | [1442834, 1447562] | YES (adj: no) | 0.6563 | 0.2188 | 0 |
| abi_sink_leaf_batched_sink_decode | 1448976ns | no significant difference | [-9151, +4188]ns | [1446534, 1453392] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_leaf_per_record_sink | 1448972ns | no significant difference | [-7905, +4120]ns | [1448089, 1450908] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_leaf_null_sink | abi_sink_leaf_batched_sink | abi_sink_leaf_batched_sink_decode | abi_sink_leaf_per_record_sink |
|---|---|---|---|---|
| 1 | 1443269ns | +0.0% | +0.3% | +0.4% |
| 2 | 1461268ns | -1.2% | -1.1% | -0.9% |
| 3 | 1450103ns | -0.3% | -0.2% | +0.1% |
| 4 | 1447662ns | -0.0% | +0.3% | +0.2% |
| 5 | 1450977ns | -0.2% | +0.3% | -0.1% |
| 6 | 1449608ns | -0.5% | +0.1% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_leaf_batched_sink | 0.004 | ok |
| abi_sink_leaf_batched_sink_decode | 0.485 | moderate+ |
| abi_sink_leaf_null_sink | -0.466 | moderate- |
| abi_sink_leaf_per_record_sink | -0.065 | ok |

**Consistency summary:**

- **abi_sink_leaf_batched_sink**: won 4/6, lost 0/6
- **abi_sink_leaf_batched_sink_decode**: won 2/6, lost 3/6
- **abi_sink_leaf_per_record_sink**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_leaf_batched_sink | 4372886.8ns | 1445155.3ns | 302.6% | HIGH |
| abi_sink_leaf_batched_sink_decode | 4390050.1ns | 1449634.0ns | 302.8% | HIGH |
| abi_sink_leaf_null_sink | 4390841.0ns | 1450481.2ns | 302.7% | HIGH |
| abi_sink_leaf_per_record_sink | 4384510.8ns | 1449323.3ns | 302.5% | HIGH |

## Distribution (algo ns)

```
abi_sink_leaf_batched_sink (n=6, range 1442205.0-1447561.9 ns)
  1442205.0 |########################################
  1442472.8 |
  1442740.7 |
  1443008.5 |
  1443276.4 |########################################
  1443544.2 |########################################
  1443812.1 |
  1444079.9 |
  1444347.7 |
  1444615.6 |
  1444883.4 |
  1445151.3 |
  1445419.1 |
  1445687.0 |
  1445954.8 |
  1446222.6 |########################################
  1446490.5 |
  1446758.3 |
  1447026.2 |########################################
  1447294.0 |
  (0 below, 1 above range)

abi_sink_leaf_batched_sink_decode (n=6, range 1445714.2-1453391.9 ns)
  1445714.2 |####################
  1446098.1 |
  1446482.0 |
  1446865.8 |
  1447249.7 |########################################
  1447633.6 |
  1448017.5 |
  1448401.4 |
  1448785.3 |
  1449169.1 |
  1449553.0 |
  1449936.9 |
  1450320.8 |####################
  1450704.7 |
  1451088.6 |
  1451472.4 |####################
  1451856.3 |
  1452240.2 |
  1452624.1 |
  1453008.0 |
  (0 below, 1 above range)

abi_sink_leaf_null_sink (n=6, range 1443268.7-1456122.3 ns)
  1443268.7 |########################################
  1443911.4 |
  1444554.1 |
  1445196.7 |
  1445839.4 |
  1446482.1 |
  1447124.8 |########################################
  1447767.5 |
  1448410.1 |
  1449052.8 |########################################
  1449695.5 |########################################
  1450338.2 |########################################
  1450980.9 |
  1451623.5 |
  1452266.2 |
  1452908.9 |
  1453551.6 |
  1454194.3 |
  1454836.9 |
  1455479.6 |
  (0 below, 1 above range)

abi_sink_leaf_per_record_sink (n=6, range 1447466.2-1450908.1 ns)
  1447466.2 |####################
  1447638.3 |
  1447810.4 |
  1447982.5 |
  1448154.6 |
  1448326.7 |
  1448498.8 |
  1448670.9 |####################
  1448843.0 |########################################
  1449015.1 |
  1449187.1 |
  1449359.2 |
  1449531.3 |
  1449703.4 |
  1449875.5 |
  1450047.6 |
  1450219.7 |
  1450391.8 |####################
  1450563.9 |
  1450736.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_leaf_batched_sink**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_leaf_batched_sink_decode**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_leaf_null_sink**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_sink_leaf_per_record_sink**: bridge=302.6% of algo (FFI overhead may distort results)
