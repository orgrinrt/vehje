# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_direct beats baseline by 24% (significant)

carrier_pre_leaf_direct is -1.54 us (24%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_leaf_direct shows alternating (throttle bounce) (autocorr -0.66)

carrier_pre_leaf_direct's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} vs {carrier_pre_leaf_fntable} (29% apart)

The field splits into a fast tier {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} and a slow tier {carrier_pre_leaf_fntable} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_leaf_direct** at 4898.3 ns median (-24.2% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.78x (fastest 4898.3 ns, slowest 8706.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 7481ns | 7446ns | 7370ns | 7426ns | 7618ns | -17.57% |
| carrier_pre_leaf_fntable | 11162ns | 11352ns | 9737ns | 11180ns | 11849ns | +23.01% |
| carrier_pre_leaf_null | 7693ns | 7674ns | 7420ns | 7645ns | 7901ns | -15.23% |
| carrier_pre_leaf_regcache | 9331ns | 9316ns | 9171ns | 9285ns | 9480ns | +2.82% |
| carrier_pre_leaf_switch | 9075ns | 9038ns | 8903ns | 9013ns | 9253ns | base |
| carrier_pre_leaf_threaded | 8343ns | 8374ns | 8205ns | 8330ns | 8432ns | -8.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 4914ns | 4878ns | 4961ns | -24.30% | 0.052 |
| carrier_pre_leaf_fntable | 8539ns | 7438ns | 9009ns | +31.53% | 0.030 |
| carrier_pre_leaf_null | 5097ns | 4983ns | 5165ns | -21.48% | 0.050 |
| carrier_pre_leaf_regcache | 6740ns | 6705ns | 6777ns | +3.83% | 0.038 |
| carrier_pre_leaf_switch | 6492ns | 6396ns | 6592ns | base | 0.039 |
| carrier_pre_leaf_threaded | 5748ns | 5715ns | 5777ns | -11.45% | 0.045 |

## Performance model

- Peak throughput: **0.052 Gops/s** (carrier_pre_leaf_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.052 | 99.6% |
| carrier_pre_leaf_fntable | 0.029 | 56.0% |
| carrier_pre_leaf_null | 0.050 | 95.5% |
| carrier_pre_leaf_regcache | 0.038 | 72.4% |
| carrier_pre_leaf_switch | 0.040 | 75.4% |
| carrier_pre_leaf_threaded | 0.045 | 84.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 7481ns | 7481ns | -17.57% |
| carrier_pre_leaf_fntable | 11162ns | 11162ns | +23.01% |
| carrier_pre_leaf_null | 7693ns | 7693ns | -15.23% |
| carrier_pre_leaf_regcache | 9331ns | 9331ns | +2.82% |
| carrier_pre_leaf_switch | 9075ns | 9075ns | base |
| carrier_pre_leaf_threaded | 8343ns | 8343ns | -8.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 6466ns | base | --- | [6417, 6592] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 4898ns | -1537.5ns (-23.8%) | [-1701, -1494]ns | [4884, 4961] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 8707ns | +2245.2ns (+34.7%) | [+1327, +2569]ns | [7900, 9009] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_leaf_null | 5105ns | -1358.5ns (-21.0%) | [-1527, -1298]ns | [5021, 5165] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 6734ns | +297.5ns (+4.6%) | [+118, +330]ns | [6710, 6777] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_leaf_threaded | 5742ns | -705.6ns (-10.9%) | [-844, -681]ns | [5725, 5777] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 6699ns | -27.0% | +11.0% | -24.5% | +0.2% | -14.2% |
| 2 | 6485ns | -23.2% | +34.8% | -21.8% | +4.7% | -10.5% |
| 3 | 6448ns | -24.4% | +29.7% | -20.2% | +4.9% | -11.1% |
| 4 | 6438ns | -23.3% | +34.7% | -20.2% | +4.5% | -10.8% |
| 5 | 6396ns | -23.3% | +44.3% | -22.1% | +5.4% | -10.6% |
| 6 | 6484ns | -24.6% | +35.5% | -20.0% | +3.4% | -11.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | -0.659 | HIGH- (thermal bounce) |
| carrier_pre_leaf_fntable | -0.012 | ok |
| carrier_pre_leaf_null | -0.488 | moderate- |
| carrier_pre_leaf_regcache | -0.122 | ok |
| carrier_pre_leaf_switch | 0.127 | ok |
| carrier_pre_leaf_threaded | -0.128 | ok |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 0/6, lost 6/6
- **carrier_pre_leaf_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 88813.8ns | 4914.2ns | 1807.3% | HIGH |
| carrier_pre_leaf_fntable | 88579.1ns | 8538.6ns | 1037.4% | HIGH |
| carrier_pre_leaf_null | 87706.6ns | 5097.1ns | 1720.7% | HIGH |
| carrier_pre_leaf_regcache | 88387.4ns | 6740.2ns | 1311.4% | HIGH |
| carrier_pre_leaf_switch | 90333.3ns | 6491.6ns | 1391.5% | HIGH |
| carrier_pre_leaf_threaded | 90915.0ns | 5748.0ns | 1581.7% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 4877.5-4960.6 ns)
   4877.5 |####################
   4881.7 |
   4885.8 |
   4890.0 |########################################
   4894.1 |
   4898.3 |
   4902.4 |####################
   4906.6 |
   4910.7 |
   4914.9 |
   4919.1 |
   4923.2 |
   4927.4 |
   4931.5 |
   4935.7 |
   4939.8 |####################
   4944.0 |
   4948.1 |
   4952.3 |
   4956.4 |
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 7437.9-9009.0 ns)
   7437.9 |########################################
   7516.5 |
   7595.0 |
   7673.6 |
   7752.1 |
   7830.7 |
   7909.2 |
   7987.8 |
   8066.3 |
   8144.9 |
   8223.4 |
   8302.0 |########################################
   8380.5 |
   8459.1 |
   8537.6 |
   8616.2 |########################################
   8694.7 |########################################
   8773.3 |########################################
   8851.8 |
   8930.4 |
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 4983.3-5165.0 ns)
   4983.3 |####################
   4992.4 |
   5001.5 |
   5010.6 |
   5019.6 |
   5028.7 |
   5037.8 |
   5046.9 |
   5056.0 |####################
   5065.1 |####################
   5074.1 |
   5083.2 |
   5092.3 |
   5101.4 |
   5110.5 |
   5119.6 |
   5128.7 |
   5137.7 |########################################
   5146.8 |
   5155.9 |
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 6705.0-6777.3 ns)
   6705.0 |########################################
   6708.6 |
   6712.2 |########################################
   6715.8 |
   6719.5 |
   6723.1 |########################################
   6726.7 |
   6730.3 |
   6733.9 |
   6737.5 |
   6741.1 |########################################
   6744.8 |
   6748.4 |
   6752.0 |
   6755.6 |
   6759.2 |########################################
   6762.8 |
   6766.5 |
   6770.1 |
   6773.7 |
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 6396.2-6592.1 ns)
   6396.2 |########################################
   6406.0 |
   6415.8 |
   6425.6 |
   6435.4 |########################################
   6445.2 |########################################
   6455.0 |
   6464.8 |
   6474.6 |########################################
   6484.4 |########################################
   6494.1 |
   6503.9 |
   6513.7 |
   6523.5 |
   6533.3 |
   6543.1 |
   6552.9 |
   6562.7 |
   6572.5 |
   6582.3 |
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 5715.4-5776.7 ns)
   5715.4 |########################################
   5718.5 |
   5721.5 |
   5724.6 |
   5727.7 |
   5730.7 |
   5733.8 |########################################
   5736.9 |########################################
   5739.9 |
   5743.0 |########################################
   5746.0 |
   5749.1 |########################################
   5752.2 |
   5755.2 |
   5758.3 |
   5761.4 |
   5764.4 |
   5767.5 |
   5770.6 |
   5773.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=1810.6% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=1005.0% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=1718.7% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=1312.5% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=1387.8% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=1582.4% of algo (FFI overhead may distort results)
