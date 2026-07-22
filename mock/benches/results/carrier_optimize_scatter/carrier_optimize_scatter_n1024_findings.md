# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_scatter_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_scatter_none has the worst median (37.33 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_scatter_all at 34.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: carrier_opt_scatter_all** at 34294.4 ns median (-8.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.09x (fastest 34294.4 ns, slowest 37332.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 36841ns | 36782ns | 36169ns | 36731ns | 37342ns | -14.74% |
| carrier_opt_scatter_cse | 37029ns | 37330ns | 35078ns | 37052ns | 37969ns | -14.31% |
| carrier_opt_scatter_cseeqsat | 41719ns | 37067ns | 36718ns | 37030ns | 51252ns | -3.45% |
| carrier_opt_scatter_dce | 39703ns | 39535ns | 39150ns | 39468ns | 40332ns | -8.12% |
| carrier_opt_scatter_eqsat | 37384ns | 37650ns | 36119ns | 37371ns | 38035ns | -13.49% |
| carrier_opt_scatter_fold | 39237ns | 39661ns | 36544ns | 39496ns | 40196ns | -9.20% |
| carrier_opt_scatter_none | 43211ns | 40084ns | 39678ns | 40025ns | 49756ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 34289ns | 33870ns | 34501ns | -15.41% | 0.030 |
| carrier_opt_scatter_cse | 34484ns | 32497ns | 35373ns | -14.92% | 0.030 |
| carrier_opt_scatter_cseeqsat | 39265ns | 34423ns | 48658ns | -3.13% | 0.026 |
| carrier_opt_scatter_dce | 37043ns | 36785ns | 37480ns | -8.61% | 0.028 |
| carrier_opt_scatter_eqsat | 34614ns | 33098ns | 35113ns | -14.60% | 0.030 |
| carrier_opt_scatter_fold | 36643ns | 34207ns | 37415ns | -9.60% | 0.028 |
| carrier_opt_scatter_none | 40534ns | 37132ns | 47130ns | base | 0.025 |

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_opt_scatter_cse; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.030 | 94.8% |
| carrier_opt_scatter_cse | 0.029 | 93.2% |
| carrier_opt_scatter_cseeqsat | 0.030 | 93.8% |
| carrier_opt_scatter_dce | 0.028 | 88.2% |
| carrier_opt_scatter_eqsat | 0.029 | 93.0% |
| carrier_opt_scatter_fold | 0.028 | 87.7% |
| carrier_opt_scatter_none | 0.027 | 87.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 36841ns | 36841ns | -14.74% |
| carrier_opt_scatter_cse | 37029ns | 37029ns | -14.31% |
| carrier_opt_scatter_cseeqsat | 41719ns | 41719ns | -3.45% |
| carrier_opt_scatter_dce | 39703ns | 39703ns | -8.12% |
| carrier_opt_scatter_eqsat | 37384ns | 37384ns | -13.49% |
| carrier_opt_scatter_fold | 39237ns | 39237ns | -9.20% |
| carrier_opt_scatter_none | 43211ns | 43211ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 37332ns | base | --- | [37140, 47130] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 34294ns | -3156.7ns (-8.5%) | [-12845, -2733]ns | [34072, 34501] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_scatter_cse | 34879ns | -2686.4ns (-7.2%) | [-13504, -1959]ns | [33200, 35373] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_scatter_cseeqsat | 34653ns | no significant difference | [-2878, +1766]ns | [34486, 48658] | no | 0.2625 | 0.2188 | 0 |
| carrier_opt_scatter_dce | 36860ns | no significant difference | [-10276, +340]ns | [36788, 37480] | no | 0.2625 | 0.2188 | 0 |
| carrier_opt_scatter_eqsat | 34932ns | -2665.9ns (-7.1%) | [-12800, -2294]ns | [33797, 35113] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_scatter_fold | 37045ns | no significant difference | [-11661, +203]ns | [35469, 37415] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_cse | carrier_opt_scatter_cseeqsat | carrier_opt_scatter_dce | carrier_opt_scatter_eqsat | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|---|
| 1 | 37609ns | -8.4% | -13.6% | -7.9% | -1.8% | -7.0% | -2.3% |
| 2 | 37132ns | -7.0% | -5.7% | -5.4% | +2.3% | -10.9% | -1.0% |
| 3 | 37147ns | -7.7% | -8.7% | -7.0% | -0.5% | -5.4% | +0.8% |
| 4 | 37454ns | -8.5% | -5.2% | -7.5% | -1.8% | -6.9% | -0.2% |
| 5 | 56651ns | -39.5% | -38.7% | +9.8% | -35.1% | -38.1% | -39.6% |
| 6 | 37210ns | -9.0% | -5.3% | -7.5% | -1.1% | -7.3% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | 0.127 | ok |
| carrier_opt_scatter_cse | -0.232 | moderate- |
| carrier_opt_scatter_cseeqsat | -0.248 | moderate- |
| carrier_opt_scatter_dce | -0.047 | ok |
| carrier_opt_scatter_eqsat | -0.386 | moderate- |
| carrier_opt_scatter_fold | -0.364 | moderate- |
| carrier_opt_scatter_none | -0.229 | moderate- |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 6/6, lost 0/6
- **carrier_opt_scatter_cse**: won 6/6, lost 0/6
- **carrier_opt_scatter_cseeqsat**: won 5/6, lost 1/6
- **carrier_opt_scatter_dce**: won 5/6, lost 1/6
- **carrier_opt_scatter_eqsat**: won 6/6, lost 0/6
- **carrier_opt_scatter_fold**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 103054.6ns | 34289.0ns | 300.5% | HIGH |
| carrier_opt_scatter_cse | 103615.9ns | 34484.3ns | 300.5% | HIGH |
| carrier_opt_scatter_cseeqsat | 107513.6ns | 39265.4ns | 273.8% | HIGH |
| carrier_opt_scatter_dce | 110233.8ns | 37042.9ns | 297.6% | HIGH |
| carrier_opt_scatter_eqsat | 104049.1ns | 34614.3ns | 300.6% | HIGH |
| carrier_opt_scatter_fold | 109840.6ns | 36642.9ns | 299.8% | HIGH |
| carrier_opt_scatter_none | 111701.5ns | 40534.0ns | 275.6% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 33870.0-34501.1 ns)
  33870.0 |####################
  33901.6 |
  33933.1 |
  33964.7 |
  33996.2 |
  34027.8 |
  34059.3 |
  34090.9 |
  34122.4 |
  34154.0 |
  34185.5 |
  34217.1 |
  34248.6 |####################
  34280.2 |########################################
  34311.7 |
  34343.3 |
  34374.8 |
  34406.4 |
  34437.9 |####################
  34469.5 |
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 32497.1-35373.3 ns)
  32497.1 |########################################
  32640.9 |
  32784.7 |
  32928.5 |
  33072.3 |
  33216.2 |
  33360.0 |
  33503.8 |
  33647.6 |
  33791.4 |########################################
  33935.2 |
  34079.0 |
  34222.8 |
  34366.7 |
  34510.5 |
  34654.3 |########################################
  34798.1 |
  34941.9 |########################################
  35085.7 |
  35229.5 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_cseeqsat (n=6, range 34423.3-48657.7 ns)
  34423.3 |########################################
  35135.0 |
  35846.7 |
  36558.5 |
  37270.2 |
  37981.9 |
  38693.6 |
  39405.3 |
  40117.1 |
  40828.8 |
  41540.5 |
  42252.2 |
  42963.9 |
  43675.7 |
  44387.4 |
  45099.1 |
  45810.8 |
  46522.5 |
  47234.3 |
  47946.0 |
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 36785.4-37480.2 ns)
  36785.4 |########################################
  36820.1 |
  36854.9 |
  36889.6 |#############
  36924.4 |#############
  36959.1 |
  36993.9 |
  37028.6 |
  37063.3 |
  37098.1 |
  37132.8 |
  37167.6 |
  37202.3 |
  37237.1 |
  37271.8 |
  37306.5 |
  37341.3 |
  37376.0 |
  37410.8 |
  37445.5 |
  (0 below, 1 above range)

carrier_opt_scatter_eqsat (n=6, range 33097.9-35113.3 ns)
  33097.9 |########################################
  33198.7 |
  33299.4 |
  33400.2 |
  33501.0 |
  33601.8 |
  33702.5 |
  33803.3 |
  33904.1 |
  34004.9 |
  34105.6 |
  34206.4 |
  34307.2 |
  34407.9 |########################################
  34508.7 |
  34609.5 |
  34710.3 |
  34811.0 |########################################
  34911.8 |########################################
  35012.6 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 34206.7-37414.6 ns)
  34206.7 |####################
  34367.1 |
  34527.5 |
  34687.9 |
  34848.3 |
  35008.7 |
  35169.1 |
  35329.5 |
  35489.9 |
  35650.3 |
  35810.6 |
  35971.0 |
  36131.4 |
  36291.8 |
  36452.2 |
  36612.6 |########################################
  36773.0 |
  36933.4 |
  37093.8 |
  37254.2 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 37132.5-47130.0 ns)
  37132.5 |########################################
  37632.4 |
  38132.2 |
  38632.1 |
  39132.0 |
  39631.9 |
  40131.8 |
  40631.6 |
  41131.5 |
  41631.4 |
  42131.2 |
  42631.1 |
  43131.0 |
  43630.9 |
  44130.8 |
  44630.6 |
  45130.5 |
  45630.4 |
  46130.2 |
  46630.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=300.4% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cseeqsat**: CV=26.1% (high variance, measurements may be unstable)
- **carrier_opt_scatter_cseeqsat**: bridge=300.7% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=299.6% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_eqsat**: bridge=300.4% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=300.3% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=298.3% of algo (FFI overhead may distort results)
