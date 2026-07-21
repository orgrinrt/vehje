# Branch tier: interpreter dispatch vs native-direct vs copy-and-patch stencil JIT

3 variants, 6 samples per variant.
Baseline: **bt_interp_dispatch**

## Key findings

- **Fastest: bt_stencil_jit** at 85.6 ns median (-88.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 8.33x (fastest 85.6 ns, slowest 713.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bt_interp_dispatch | 3313ns | 3053ns | 2834ns | 3008ns | 4012ns | base |
| bt_native_direct | 2915ns | 2708ns | 2244ns | 2562ns | 3780ns | -12.02% |
| bt_stencil_jit | 2437ns | 2325ns | 2248ns | 2322ns | 2704ns | -26.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bt_interp_dispatch | 771ns | 654ns | 937ns | base | 0.083 |
| bt_native_direct | 94ns | 70ns | 124ns | -87.80% | 0.680 |
| bt_stencil_jit | 89ns | 82ns | 98ns | -88.49% | 0.721 |

## Performance model

- Peak throughput: **0.920 Gops/s** (bt_native_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bt_interp_dispatch | 0.090 | 9.8% |
| bt_native_direct | 0.740 | 80.5% |
| bt_stencil_jit | 0.748 | 81.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bt_interp_dispatch | 3313ns | 3313ns | base |
| bt_native_direct | 2915ns | 2915ns | -12.02% |
| bt_stencil_jit | 2437ns | 2437ns | -26.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bt_interp_dispatch | 713ns | base | --- | [663, 937] | --- | --- | --- | --- |
| bt_native_direct | 86ns | -637.1ns (-89.3%) | [-813, -581]ns | [72, 124] | YES | 0.0313 | 0.0313 | 0 |
| bt_stencil_jit | 86ns | -629.9ns (-88.3%) | [-840, -577]ns | [83, 98] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bt_interp_dispatch | bt_native_direct | bt_stencil_jit |
|---|---|---|---|
| 1 | 1002ns | -86.7% | -91.2% |
| 2 | 872ns | -86.9% | -87.8% |
| 3 | 672ns | -86.5% | -86.7% |
| 4 | 672ns | -89.6% | -87.6% |
| 5 | 654ns | -88.6% | -87.4% |
| 6 | 755ns | -89.1% | -89.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bt_interp_dispatch | 0.378 | moderate+ |
| bt_native_direct | 0.496 | moderate+ |
| bt_stencil_jit | 0.166 | ok |

**Consistency summary:**

- **bt_native_direct**: won 6/6, lost 0/6
- **bt_stencil_jit**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bt_interp_dispatch | 3.8ns | 771.1ns | 0.5% |  |
| bt_native_direct | 3.7ns | 94.1ns | 3.9% |  |
| bt_stencil_jit | 5.2ns | 88.7ns | 5.8% | HIGH |

## Distribution (algo ns)

```
bt_interp_dispatch (n=6, range 653.7-937.3 ns)
    653.7 |####################
    667.9 |########################################
    682.1 |
    696.2 |
    710.4 |
    724.6 |
    738.8 |
    753.0 |####################
    767.1 |
    781.3 |
    795.5 |
    809.7 |
    823.9 |
    838.0 |
    852.2 |
    866.4 |####################
    880.6 |
    894.8 |
    908.9 |
    923.1 |
  (0 below, 1 above range)

bt_native_direct (n=6, range 69.6-124.0 ns)
     69.6 |########################################
     72.3 |########################################
     75.0 |
     77.8 |
     80.5 |########################################
     83.2 |
     85.9 |
     88.6 |########################################
     91.3 |
     94.1 |
     96.8 |
     99.5 |
    102.2 |
    104.9 |
    107.6 |
    110.4 |
    113.1 |########################################
    115.8 |
    118.5 |
    121.2 |
  (0 below, 1 above range)

bt_stencil_jit (n=6, range 82.1-97.9 ns)
     82.1 |####################
     82.9 |########################################
     83.7 |
     84.5 |
     85.3 |
     86.0 |
     86.8 |
     87.6 |####################
     88.4 |
     89.2 |####################
     90.0 |
     90.8 |
     91.6 |
     92.4 |
     93.2 |
     94.0 |
     94.7 |
     95.5 |
     96.3 |
     97.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **bt_native_direct**: CV=24.2% (high variance, measurements may be unstable)
