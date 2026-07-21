# Branch tier: interpreter dispatch vs native-direct vs copy-and-patch stencil JIT

3 variants, 6 samples per variant.
Baseline: **bt_interp_dispatch**

## Key findings

- **Fastest: bt_stencil_jit** at 1156.8 ns median (-91.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 11.77x (fastest 1156.8 ns, slowest 13610.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bt_interp_dispatch | 16275ns | 16121ns | 15432ns | 15925ns | 17220ns | base |
| bt_native_direct | 4248ns | 4220ns | 3960ns | 4218ns | 4437ns | -73.90% |
| bt_stencil_jit | 3588ns | 3584ns | 3112ns | 3581ns | 3837ns | -77.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bt_interp_dispatch | 13679ns | 13002ns | 14376ns | base | 0.075 |
| bt_native_direct | 1629ns | 1520ns | 1691ns | -88.09% | 0.629 |
| bt_stencil_jit | 1152ns | 980ns | 1231ns | -91.58% | 0.889 |

## Performance model

- Peak throughput: **1.045 Gops/s** (bt_stencil_jit; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bt_interp_dispatch | 0.075 | 7.2% |
| bt_native_direct | 0.630 | 60.3% |
| bt_stencil_jit | 0.885 | 84.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bt_interp_dispatch | 16275ns | 16275ns | base |
| bt_native_direct | 4248ns | 4248ns | -73.90% |
| bt_stencil_jit | 3588ns | 3588ns | -77.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bt_interp_dispatch | 13611ns | base | --- | [13050, 14376] | --- | --- | --- | --- |
| bt_native_direct | 1624ns | -11986.9ns (-88.1%) | [-12803, -11360]ns | [1572, 1691] | YES | 0.0313 | 0.0313 | 0 |
| bt_stencil_jit | 1157ns | -12414.4ns (-91.2%) | [-13308, -11861]ns | [1068, 1231] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bt_interp_dispatch | bt_native_direct | bt_stencil_jit |
|---|---|---|---|
| 1 | 14126ns | -89.2% | -93.1% |
| 2 | 13098ns | -86.6% | -91.2% |
| 3 | 13146ns | -87.7% | -91.2% |
| 4 | 14075ns | -88.5% | -91.2% |
| 5 | 13002ns | -87.5% | -90.6% |
| 6 | 14627ns | -88.9% | -92.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bt_interp_dispatch | -0.458 | moderate- |
| bt_native_direct | -0.517 | HIGH- (thermal bounce) |
| bt_stencil_jit | 0.153 | ok |

**Consistency summary:**

- **bt_native_direct**: won 6/6, lost 0/6
- **bt_stencil_jit**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bt_interp_dispatch | 3.5ns | 13679.2ns | 0.0% |  |
| bt_native_direct | 3.2ns | 1629.2ns | 0.2% |  |
| bt_stencil_jit | 5.8ns | 1151.7ns | 0.5% |  |

## Distribution (algo ns)

```
bt_interp_dispatch (n=6, range 13002.5-14376.5 ns)
  13002.5 |########################################
  13071.2 |########################################
  13139.9 |########################################
  13208.6 |
  13277.3 |
  13346.0 |
  13414.7 |
  13483.4 |
  13552.1 |
  13620.8 |
  13689.5 |
  13758.2 |
  13826.9 |
  13895.6 |
  13964.3 |
  14033.0 |########################################
  14101.7 |########################################
  14170.4 |
  14239.1 |
  14307.8 |
  (0 below, 1 above range)

bt_native_direct (n=6, range 1520.0-1691.5 ns)
   1520.0 |##########
   1528.6 |
   1537.1 |
   1545.7 |
   1554.3 |
   1562.9 |
   1571.4 |
   1580.0 |
   1588.6 |
   1597.2 |
   1605.7 |
   1614.3 |
   1622.9 |########################################
   1631.4 |
   1640.0 |
   1648.6 |
   1657.2 |
   1665.7 |
   1674.3 |
   1682.9 |
  (0 below, 1 above range)

bt_stencil_jit (n=6, range 980.0-1230.7 ns)
    980.0 |####################
    992.5 |
   1005.1 |
   1017.6 |
   1030.1 |
   1042.7 |
   1055.2 |
   1067.7 |
   1080.3 |
   1092.8 |
   1105.3 |
   1117.9 |
   1130.4 |
   1142.9 |####################
   1155.5 |########################################
   1168.0 |
   1180.5 |
   1193.1 |
   1205.6 |
   1218.1 |####################
  (0 below, 1 above range)

```
