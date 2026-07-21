# Branch tier: interpreter dispatch vs native-direct vs copy-and-patch stencil JIT

3 variants, 6 samples per variant.
Baseline: **bt_interp_dispatch**

## Key findings

- **Fastest: bt_native_direct** at 6131.4 ns median (-88.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.01x (fastest 6131.4 ns, slowest 55248.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bt_interp_dispatch | 57803ns | 57711ns | 55416ns | 57140ns | 59990ns | base |
| bt_native_direct | 8705ns | 8564ns | 8558ns | 8562ns | 8993ns | -84.94% |
| bt_stencil_jit | 11291ns | 11191ns | 9978ns | 10861ns | 12593ns | -80.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bt_interp_dispatch | 55298ns | 53042ns | 57299ns | base | 0.074 |
| bt_native_direct | 6239ns | 6129ns | 6455ns | -88.72% | 0.657 |
| bt_stencil_jit | 8842ns | 7724ns | 9986ns | -84.01% | 0.463 |

## Performance model

- Peak throughput: **0.668 Gops/s** (bt_native_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bt_interp_dispatch | 0.074 | 11.1% |
| bt_native_direct | 0.668 | 100.0% |
| bt_stencil_jit | 0.473 | 70.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bt_interp_dispatch | 57803ns | 57803ns | base |
| bt_native_direct | 8705ns | 8705ns | -84.94% |
| bt_stencil_jit | 11291ns | 11291ns | -80.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bt_interp_dispatch | 55249ns | base | --- | [53345, 57299] | --- | --- | --- | --- |
| bt_native_direct | 6131ns | -49005.8ns (-88.7%) | [-51169, -47001]ns | [6130, 6455] | YES | 0.0313 | 0.0313 | 0 |
| bt_stencil_jit | 8651ns | -46321.6ns (-83.8%) | [-49127, -43919]ns | [7887, 9986] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bt_interp_dispatch | bt_native_direct | bt_stencil_jit |
|---|---|---|---|
| 1 | 58165ns | -89.5% | -86.2% |
| 2 | 54293ns | -88.7% | -85.8% |
| 3 | 56204ns | -88.7% | -82.0% |
| 4 | 53042ns | -88.4% | -83.0% |
| 5 | 56433ns | -89.1% | -85.3% |
| 6 | 53649ns | -87.8% | -81.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bt_interp_dispatch | -0.536 | HIGH- (thermal bounce) |
| bt_native_direct | -0.221 | moderate- |
| bt_stencil_jit | -0.202 | moderate- |

**Consistency summary:**

- **bt_native_direct**: won 6/6, lost 0/6
- **bt_stencil_jit**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bt_interp_dispatch | 4.5ns | 55297.6ns | 0.0% |  |
| bt_native_direct | 3.2ns | 6238.9ns | 0.1% |  |
| bt_stencil_jit | 5.1ns | 8841.7ns | 0.1% |  |

## Distribution (algo ns)

```
bt_interp_dispatch (n=6, range 53041.7-57298.9 ns)
  53041.7 |########################################
  53254.6 |
  53467.4 |########################################
  53680.3 |
  53893.1 |
  54106.0 |########################################
  54318.9 |
  54531.7 |
  54744.6 |
  54957.5 |
  55170.3 |
  55383.2 |
  55596.0 |
  55808.9 |
  56021.8 |########################################
  56234.6 |########################################
  56447.5 |
  56660.4 |
  56873.2 |
  57086.1 |
  (0 below, 1 above range)

bt_native_direct (n=6, range 6128.8-6455.2 ns)
   6128.8 |########################################
   6145.1 |
   6161.4 |
   6177.8 |
   6194.1 |
   6210.4 |
   6226.7 |
   6243.1 |
   6259.4 |
   6275.7 |
   6292.0 |
   6308.3 |
   6324.7 |
   6341.0 |##########
   6357.3 |
   6373.6 |
   6390.0 |
   6406.3 |
   6422.6 |
   6438.9 |
  (0 below, 1 above range)

bt_stencil_jit (n=6, range 7723.8-9986.5 ns)
   7723.8 |########################################
   7836.9 |
   7950.1 |########################################
   8063.2 |
   8176.3 |
   8289.5 |########################################
   8402.6 |
   8515.7 |
   8628.9 |
   8742.0 |
   8855.1 |
   8968.3 |########################################
   9081.4 |
   9194.5 |
   9307.7 |
   9420.8 |
   9533.9 |
   9647.1 |
   9760.2 |########################################
   9873.3 |
  (0 below, 1 above range)

```
