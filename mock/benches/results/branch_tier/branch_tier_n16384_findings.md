# Branch tier: interpreter dispatch vs native-direct vs copy-and-patch stencil JIT

3 variants, 6 samples per variant.
Baseline: **bt_interp_dispatch**

## Key findings

- **Fastest: bt_native_direct** at 22398.9 ns median (-88.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.04x (fastest 22398.9 ns, slowest 202499.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bt_interp_dispatch | 206769ns | 204819ns | 202588ns | 204670ns | 212009ns | base |
| bt_native_direct | 24949ns | 24619ns | 24190ns | 24479ns | 26032ns | -87.93% |
| bt_stencil_jit | 58766ns | 58423ns | 54460ns | 57344ns | 63053ns | -71.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bt_interp_dispatch | 204281ns | 200263ns | 209184ns | base | 0.080 |
| bt_native_direct | 22691ns | 22008ns | 23666ns | -88.89% | 0.722 |
| bt_stencil_jit | 56339ns | 52259ns | 60341ns | -72.42% | 0.291 |

## Performance model

- Peak throughput: **0.744 Gops/s** (bt_native_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bt_interp_dispatch | 0.081 | 10.9% |
| bt_native_direct | 0.731 | 98.3% |
| bt_stencil_jit | 0.292 | 39.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bt_interp_dispatch | 206769ns | 206769ns | base |
| bt_native_direct | 24949ns | 24949ns | -87.93% |
| bt_stencil_jit | 58766ns | 58766ns | -71.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bt_interp_dispatch | 202500ns | base | --- | [201160, 209184] | --- | --- | --- | --- |
| bt_native_direct | 22399ns | -180157.1ns (-89.0%) | [-185807, -178805]ns | [22009, 23666] | YES | 0.0313 | 0.0313 | 0 |
| bt_stencil_jit | 56061ns | -148162.2ns (-73.2%) | [-152214, -143450]ns | [52616, 60341] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bt_interp_dispatch | bt_native_direct | bt_stencil_jit |
|---|---|---|---|
| 1 | 202056ns | -88.8% | -73.4% |
| 2 | 212915ns | -89.6% | -72.6% |
| 3 | 202130ns | -88.7% | -70.7% |
| 4 | 205452ns | -88.0% | -70.1% |
| 5 | 202870ns | -89.2% | -73.9% |
| 6 | 200263ns | -89.0% | -73.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bt_interp_dispatch | -0.350 | moderate- |
| bt_native_direct | -0.138 | ok |
| bt_stencil_jit | 0.165 | ok |

**Consistency summary:**

- **bt_native_direct**: won 6/6, lost 0/6
- **bt_stencil_jit**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bt_interp_dispatch | 6.2ns | 204281.1ns | 0.0% |  |
| bt_native_direct | 3.7ns | 22691.4ns | 0.0% |  |
| bt_stencil_jit | 4.6ns | 56339.1ns | 0.0% |  |

## Distribution (algo ns)

```
bt_interp_dispatch (n=6, range 200263.3-209184.0 ns)
  200263.3 |####################
  200709.3 |
  201155.4 |
  201601.4 |
  202047.4 |########################################
  202493.5 |####################
  202939.5 |
  203385.5 |
  203831.6 |
  204277.6 |
  204723.6 |
  205169.7 |####################
  205615.7 |
  206061.7 |
  206507.8 |
  206953.8 |
  207399.8 |
  207845.9 |
  208291.9 |
  208737.9 |
  (0 below, 1 above range)

bt_native_direct (n=6, range 22007.9-23666.0 ns)
  22007.9 |########################################
  22090.8 |
  22173.7 |####################
  22256.6 |
  22339.5 |
  22422.4 |
  22505.3 |
  22588.2 |####################
  22671.1 |
  22754.0 |####################
  22837.0 |
  22919.9 |
  23002.8 |
  23085.7 |
  23168.6 |
  23251.5 |
  23334.4 |
  23417.3 |
  23500.2 |
  23583.1 |
  (0 below, 1 above range)

bt_stencil_jit (n=6, range 52258.8-60340.8 ns)
  52258.8 |########################################
  52662.9 |########################################
  53067.0 |
  53471.1 |########################################
  53875.2 |
  54279.3 |
  54683.4 |
  55087.5 |
  55491.6 |
  55895.7 |
  56299.8 |
  56703.9 |
  57108.0 |
  57512.1 |
  57916.2 |
  58320.3 |########################################
  58724.4 |
  59128.5 |########################################
  59532.6 |
  59936.7 |
  (0 below, 1 above range)

```
