# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vr_static) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vr_static has the worst median (10.67 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vr_nanbox at 9.00 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Speed leader carrier_vr_nanbox vs stability leader carrier_vr_tagged (+4% speed for 1.5x steadier)

carrier_vr_nanbox is fastest (9.00 us, CV 4.5%); carrier_vr_tagged gives up 4.4% median for 1.5x lower variance (CV 2.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_vr_nanbox** at 9001.7 ns median (-15.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.19x (fastest 9001.7 ns, slowest 10667.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 11380ns | 11539ns | 10392ns | 11345ns | 11926ns | -15.56% |
| carrier_vr_static | 13477ns | 13191ns | 12985ns | 13169ns | 14185ns | base |
| carrier_vr_tagged | 11764ns | 11909ns | 11084ns | 11774ns | 12088ns | -12.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 8859ns | 8157ns | 9233ns | -18.59% | 0.029 |
| carrier_vr_static | 10882ns | 10569ns | 11407ns | base | 0.024 |
| carrier_vr_tagged | 9253ns | 8742ns | 9472ns | -14.97% | 0.028 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_vr_nanbox; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.028 | 90.6% |
| carrier_vr_static | 0.024 | 76.5% |
| carrier_vr_tagged | 0.027 | 86.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 11380ns | 11380ns | -15.56% |
| carrier_vr_static | 13477ns | 13477ns | base |
| carrier_vr_tagged | 11764ns | 11764ns | -12.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 10667ns | base | --- | [10572, 11407] | --- | --- | --- | --- |
| carrier_vr_nanbox | 9002ns | -1601.9ns (-15.0%) | [-3065, -1402]ns | [8342, 9233] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vr_tagged | 9402ns | -1214.6ns (-11.4%) | [-2521, -1151]ns | [8886, 9472] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 12113ns | -32.7% | -25.5% |
| 2 | 10700ns | -20.3% | -18.3% |
| 3 | 10569ns | -14.6% | -10.4% |
| 4 | 10685ns | -15.0% | -11.4% |
| 5 | 10649ns | -11.9% | -11.4% |
| 6 | 10575ns | -15.1% | -11.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | 0.404 | moderate+ |
| carrier_vr_static | 0.006 | ok |
| carrier_vr_tagged | 0.244 | moderate+ |

**Consistency summary:**

- **carrier_vr_nanbox**: won 6/6, lost 0/6
- **carrier_vr_tagged**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 90331.0ns | 8858.9ns | 1019.7% | HIGH |
| carrier_vr_static | 94766.5ns | 10881.8ns | 870.9% | HIGH |
| carrier_vr_tagged | 91757.7ns | 9253.2ns | 991.6% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 8156.7-9233.4 ns)
   8156.7 |########################################
   8210.5 |
   8264.4 |
   8318.2 |
   8372.0 |
   8425.9 |
   8479.7 |########################################
   8533.5 |
   8587.4 |
   8641.2 |
   8695.0 |
   8748.9 |
   8802.7 |
   8856.5 |
   8910.4 |
   8964.2 |########################################
   9018.0 |########################################
   9071.9 |########################################
   9125.7 |
   9179.5 |
  (0 below, 1 above range)

carrier_vr_static (n=6, range 10568.8-11406.6 ns)
  10568.8 |########################################
  10610.7 |####################
  10652.6 |####################
  10694.5 |####################
  10736.4 |
  10778.3 |
  10820.2 |
  10862.0 |
  10903.9 |
  10945.8 |
  10987.7 |
  11029.6 |
  11071.5 |
  11113.4 |
  11155.3 |
  11197.2 |
  11239.1 |
  11281.0 |
  11322.9 |
  11364.8 |
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 8742.5-9471.7 ns)
   8742.5 |########################################
   8779.0 |
   8815.4 |
   8851.9 |
   8888.3 |
   8924.8 |
   8961.2 |
   8997.7 |########################################
   9034.2 |
   9070.6 |
   9107.1 |
   9143.5 |
   9180.0 |
   9216.4 |
   9252.9 |
   9289.4 |
   9325.8 |
   9362.3 |########################################
   9398.7 |########################################
   9435.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: bridge=998.7% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=886.3% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: bridge=984.7% of algo (FFI overhead may distort results)
