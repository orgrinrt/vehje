# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_scatter_dce's edge over baseline is significant but tiny (-7 ns, 0.08%)

carrier_opt_scatter_dce differs from baseline carrier_opt_scatter_none by -7 ns (0.08%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_scatter_all** at 8673.8 ns median (-6.8% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 8673.8 ns, slowest 9347.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 11263ns | 11229ns | 11165ns | 11211ns | 11389ns | -5.89% |
| carrier_opt_scatter_cse | 11267ns | 11468ns | 10035ns | 11435ns | 11632ns | -5.85% |
| carrier_opt_scatter_cseeqsat | 11474ns | 11502ns | 11312ns | 11479ns | 11547ns | -4.12% |
| carrier_opt_scatter_dce | 11956ns | 11913ns | 11671ns | 11889ns | 12198ns | -0.10% |
| carrier_opt_scatter_eqsat | 11550ns | 11485ns | 11424ns | 11473ns | 11730ns | -3.48% |
| carrier_opt_scatter_fold | 11940ns | 11840ns | 11802ns | 11828ns | 12176ns | -0.23% |
| carrier_opt_scatter_none | 11967ns | 11868ns | 11823ns | 11853ns | 12211ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 8711ns | 8659ns | 8799ns | -6.72% | 0.029 |
| carrier_opt_scatter_cse | 8733ns | 7765ns | 8988ns | -6.49% | 0.029 |
| carrier_opt_scatter_cseeqsat | 8876ns | 8840ns | 8915ns | -4.95% | 0.029 |
| carrier_opt_scatter_dce | 9328ns | 9099ns | 9505ns | -0.11% | 0.027 |
| carrier_opt_scatter_eqsat | 8886ns | 8822ns | 8936ns | -4.85% | 0.029 |
| carrier_opt_scatter_fold | 9391ns | 9318ns | 9506ns | +0.57% | 0.027 |
| carrier_opt_scatter_none | 9338ns | 9214ns | 9468ns | base | 0.027 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_opt_scatter_cse; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.030 | 89.5% |
| carrier_opt_scatter_cse | 0.029 | 87.4% |
| carrier_opt_scatter_cseeqsat | 0.029 | 87.5% |
| carrier_opt_scatter_dce | 0.028 | 83.5% |
| carrier_opt_scatter_eqsat | 0.029 | 87.3% |
| carrier_opt_scatter_fold | 0.027 | 83.1% |
| carrier_opt_scatter_none | 0.028 | 83.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 11263ns | 11263ns | -5.89% |
| carrier_opt_scatter_cse | 11267ns | 11267ns | -5.85% |
| carrier_opt_scatter_cseeqsat | 11474ns | 11474ns | -4.12% |
| carrier_opt_scatter_dce | 11956ns | 11956ns | -0.10% |
| carrier_opt_scatter_eqsat | 11550ns | 11550ns | -3.48% |
| carrier_opt_scatter_fold | 11940ns | 11940ns | -0.23% |
| carrier_opt_scatter_none | 11967ns | 11967ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 9308ns | base | --- | [9239, 9468] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 8674ns | -641.6ns (-6.9%) | [-694, -547]ns | [8660, 8799] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_cse | 8888ns | -422.1ns (-4.5%) | [-1103, -292]ns | [8323, 8988] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_cseeqsat | 8874ns | -435.8ns (-4.7%) | [-581, -368]ns | [8841, 8915] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_dce | 9301ns | no significant difference | [-99, +75]ns | [9178, 9505] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_scatter_eqsat | 8899ns | -453.5ns (-4.9%) | [-601, -303]ns | [8823, 8936] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_fold | 9347ns | +54.2ns (+0.6%) | [+6, +99]ns | [9320, 9506] | YES (adj: no) | 0.2625 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_cse | carrier_opt_scatter_cseeqsat | carrier_opt_scatter_dce | carrier_opt_scatter_eqsat | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|---|
| 1 | 9264ns | -5.9% | -16.2% | -3.9% | -0.1% | -3.7% | +1.0% |
| 2 | 9588ns | -7.3% | -7.4% | -6.9% | +0.3% | -7.1% | +0.2% |
| 3 | 9271ns | -6.6% | -4.2% | -4.5% | +1.3% | -4.8% | +0.5% |
| 4 | 9348ns | -7.2% | -4.9% | -4.9% | -0.9% | -5.6% | +0.7% |
| 5 | 9345ns | -7.3% | -3.4% | -5.4% | -0.1% | -4.9% | -0.0% |
| 6 | 9214ns | -5.9% | -2.9% | -4.1% | -1.2% | -2.8% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | -0.081 | ok |
| carrier_opt_scatter_cse | 0.012 | ok |
| carrier_opt_scatter_cseeqsat | 0.079 | ok |
| carrier_opt_scatter_dce | -0.053 | ok |
| carrier_opt_scatter_eqsat | 0.228 | moderate+ |
| carrier_opt_scatter_fold | -0.382 | moderate- |
| carrier_opt_scatter_none | -0.418 | moderate- |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 6/6, lost 0/6
- **carrier_opt_scatter_cse**: won 6/6, lost 0/6
- **carrier_opt_scatter_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_scatter_dce**: won 2/6, lost 2/6
- **carrier_opt_scatter_eqsat**: won 6/6, lost 0/6
- **carrier_opt_scatter_fold**: won 0/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 87339.9ns | 8710.9ns | 1002.7% | HIGH |
| carrier_opt_scatter_cse | 89243.5ns | 8732.6ns | 1022.0% | HIGH |
| carrier_opt_scatter_cseeqsat | 89037.9ns | 8876.5ns | 1003.1% | HIGH |
| carrier_opt_scatter_dce | 91356.6ns | 9327.8ns | 979.4% | HIGH |
| carrier_opt_scatter_eqsat | 88640.1ns | 8885.8ns | 997.6% | HIGH |
| carrier_opt_scatter_fold | 91922.3ns | 9391.3ns | 978.8% | HIGH |
| carrier_opt_scatter_none | 91935.1ns | 9338.3ns | 984.5% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 8658.8-8798.8 ns)
   8658.8 |########################################
   8665.8 |####################
   8672.8 |####################
   8679.8 |
   8686.8 |
   8693.8 |
   8700.8 |
   8707.8 |####################
   8714.8 |
   8721.8 |
   8728.8 |
   8735.8 |
   8742.8 |
   8749.8 |
   8756.8 |
   8763.8 |
   8770.8 |
   8777.8 |
   8784.8 |
   8791.8 |
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 7765.4-8987.7 ns)
   7765.4 |#############
   7826.5 |
   7887.6 |
   7948.7 |
   8009.9 |
   8071.0 |
   8132.1 |
   8193.2 |
   8254.3 |
   8315.4 |
   8376.5 |
   8437.7 |
   8498.8 |
   8559.9 |
   8621.0 |
   8682.1 |
   8743.2 |
   8804.4 |
   8865.5 |########################################
   8926.6 |#############
  (0 below, 1 above range)

carrier_opt_scatter_cseeqsat (n=6, range 8839.6-8915.0 ns)
   8839.6 |########################################
   8843.4 |
   8847.1 |
   8850.9 |####################
   8854.7 |
   8858.5 |
   8862.2 |
   8866.0 |
   8869.8 |
   8873.5 |
   8877.3 |
   8881.1 |
   8884.8 |
   8888.6 |
   8892.4 |####################
   8896.1 |
   8899.9 |####################
   8903.7 |
   8907.5 |
   8911.2 |
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 9098.8-9504.5 ns)
   9098.8 |########################################
   9119.1 |
   9139.4 |
   9159.7 |
   9179.9 |
   9200.2 |
   9220.5 |
   9240.8 |########################################
   9261.1 |########################################
   9281.4 |
   9301.7 |
   9322.0 |########################################
   9342.2 |
   9362.5 |
   9382.8 |########################################
   9403.1 |
   9423.4 |
   9443.7 |
   9464.0 |
   9484.3 |
  (0 below, 1 above range)

carrier_opt_scatter_eqsat (n=6, range 8822.5-8935.7 ns)
   8822.5 |########################################
   8828.2 |
   8833.8 |
   8839.5 |
   8845.1 |
   8850.8 |
   8856.4 |
   8862.1 |
   8867.8 |
   8873.4 |
   8879.1 |
   8884.7 |####################
   8890.4 |
   8896.0 |
   8901.7 |
   8907.4 |####################
   8913.0 |
   8918.7 |####################
   8924.3 |
   8930.0 |
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 9317.9-9506.5 ns)
   9317.9 |########################################
   9327.3 |
   9336.8 |####################
   9346.2 |####################
   9355.6 |
   9365.0 |
   9374.5 |
   9383.9 |
   9393.3 |
   9402.7 |####################
   9412.2 |
   9421.6 |
   9431.0 |
   9440.5 |
   9449.9 |
   9459.3 |
   9468.7 |
   9478.2 |
   9487.6 |
   9497.0 |
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 9213.8-9467.9 ns)
   9213.8 |####################
   9226.5 |
   9239.2 |
   9251.9 |####################
   9264.6 |####################
   9277.3 |
   9290.0 |
   9302.7 |
   9315.4 |
   9328.1 |
   9340.8 |########################################
   9353.6 |
   9366.3 |
   9379.0 |
   9391.7 |
   9404.4 |
   9417.1 |
   9429.8 |
   9442.5 |
   9455.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=1002.8% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=1002.5% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cseeqsat**: bridge=1003.0% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=983.5% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_eqsat**: bridge=996.4% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=985.5% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=987.6% of algo (FFI overhead may distort results)
