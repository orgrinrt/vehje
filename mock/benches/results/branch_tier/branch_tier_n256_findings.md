# Branch tier: interpreter dispatch vs native-direct vs copy-and-patch stencil JIT

3 variants, 6 samples per variant.
Baseline: **bt_interp_dispatch**

## Key findings

- **Fastest: bt_stencil_jit** at 302.0 ns median (-90.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.78x (fastest 302.0 ns, slowest 3256.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bt_interp_dispatch | 5722ns | 5831ns | 4870ns | 5721ns | 6150ns | base |
| bt_native_direct | 2856ns | 2978ns | 2507ns | 2851ns | 3039ns | -50.09% |
| bt_stencil_jit | 2701ns | 2798ns | 2335ns | 2681ns | 2914ns | -52.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bt_interp_dispatch | 3195ns | 2698ns | 3444ns | base | 0.080 |
| bt_native_direct | 371ns | 327ns | 391ns | -88.39% | 0.690 |
| bt_stencil_jit | 298ns | 246ns | 332ns | -90.67% | 0.859 |

## Performance model

- Peak throughput: **1.040 Gops/s** (bt_stencil_jit; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bt_interp_dispatch | 0.079 | 7.6% |
| bt_native_direct | 0.657 | 63.2% |
| bt_stencil_jit | 0.848 | 81.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bt_interp_dispatch | 5722ns | 5722ns | base |
| bt_native_direct | 2856ns | 2856ns | -50.09% |
| bt_stencil_jit | 2701ns | 2701ns | -52.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bt_interp_dispatch | 3257ns | base | --- | [2884, 3444] | --- | --- | --- | --- |
| bt_native_direct | 390ns | -2866.9ns (-88.0%) | [-3054, -2551]ns | [332, 391] | YES | 0.0313 | 0.0313 | 0 |
| bt_stencil_jit | 302ns | -2924.4ns (-89.8%) | [-3142, -2623]ns | [260, 332] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bt_interp_dispatch | bt_native_direct | bt_stencil_jit |
|---|---|---|---|
| 1 | 3070ns | -89.0% | -92.0% |
| 2 | 3248ns | -88.0% | -89.8% |
| 3 | 3317ns | -88.3% | -91.2% |
| 4 | 3571ns | -89.0% | -91.2% |
| 5 | 3265ns | -88.1% | -89.8% |
| 6 | 2698ns | -87.9% | -89.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bt_interp_dispatch | 0.087 | ok |
| bt_native_direct | -0.074 | ok |
| bt_stencil_jit | -0.397 | moderate- |

**Consistency summary:**

- **bt_native_direct**: won 6/6, lost 0/6
- **bt_stencil_jit**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bt_interp_dispatch | 3.5ns | 3194.7ns | 0.1% |  |
| bt_native_direct | 4.8ns | 370.8ns | 1.3% |  |
| bt_stencil_jit | 5.0ns | 298.2ns | 1.7% |  |

## Distribution (algo ns)

```
bt_interp_dispatch (n=6, range 2697.5-3443.9 ns)
   2697.5 |########################################
   2734.8 |
   2772.1 |
   2809.5 |
   2846.8 |
   2884.1 |
   2921.4 |
   2958.8 |
   2996.1 |
   3033.4 |########################################
   3070.7 |
   3108.0 |
   3145.4 |
   3182.7 |
   3220.0 |########################################
   3257.3 |########################################
   3294.7 |########################################
   3332.0 |
   3369.3 |
   3406.6 |
  (0 below, 1 above range)

bt_native_direct (n=6, range 327.1-390.6 ns)
    327.1 |#############
    330.3 |
    333.5 |
    336.6 |#############
    339.8 |
    343.0 |
    346.2 |
    349.3 |
    352.5 |
    355.7 |
    358.9 |
    362.0 |
    365.2 |
    368.4 |
    371.6 |
    374.7 |
    377.9 |
    381.1 |
    384.2 |
    387.4 |########################################
  (0 below, 1 above range)

bt_stencil_jit (n=6, range 246.2-332.3 ns)
    246.2 |########################################
    250.5 |
    254.8 |
    259.1 |
    263.4 |
    267.7 |
    272.0 |########################################
    276.3 |
    280.6 |
    284.9 |
    289.2 |########################################
    293.6 |
    297.9 |
    302.2 |
    306.5 |
    310.8 |########################################
    315.1 |
    319.4 |
    323.7 |
    328.0 |########################################
  (0 below, 1 above range)

```
