# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_tight_null dominates: 10% faster than the next best (carrier_pre_tight_regcache)

carrier_pre_tight_null (8.08 us) leads carrier_pre_tight_regcache (8.93 us) by 10%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_tight_direct shows alternating (throttle bounce) (autocorr -0.74)

carrier_pre_tight_direct's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_pre_tight_regcache's edge over baseline is significant but tiny (-46 ns, 0.51%)

carrier_pre_tight_regcache differs from baseline carrier_pre_tight_switch by -46 ns (0.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_pre_tight_null** at 8080.4 ns median (-9.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.23x (fastest 8080.4 ns, slowest 9921.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 12515ns | 12555ns | 12315ns | 12500ns | 12638ns | +9.18% |
| carrier_pre_tight_fntable | 11849ns | 12066ns | 10522ns | 12019ns | 12259ns | +3.38% |
| carrier_pre_tight_null | 10617ns | 10622ns | 10458ns | 10590ns | 10736ns | -7.38% |
| carrier_pre_tight_regcache | 11546ns | 11526ns | 11358ns | 11511ns | 11693ns | +0.73% |
| carrier_pre_tight_switch | 11462ns | 11419ns | 11345ns | 11413ns | 11595ns | base |
| carrier_pre_tight_threaded | 12103ns | 12101ns | 11960ns | 12070ns | 12225ns | +5.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 9925ns | 9859ns | 9975ns | +11.06% | 0.026 |
| carrier_pre_tight_fntable | 9363ns | 8319ns | 9668ns | +4.77% | 0.027 |
| carrier_pre_tight_null | 8047ns | 7867ns | 8097ns | -9.96% | 0.032 |
| carrier_pre_tight_regcache | 8939ns | 8798ns | 9060ns | +0.02% | 0.029 |
| carrier_pre_tight_switch | 8937ns | 8771ns | 9041ns | base | 0.029 |
| carrier_pre_tight_threaded | 9515ns | 9480ns | 9567ns | +6.47% | 0.027 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.026 | 79.3% |
| carrier_pre_tight_fntable | 0.027 | 82.5% |
| carrier_pre_tight_null | 0.032 | 97.4% |
| carrier_pre_tight_regcache | 0.029 | 88.1% |
| carrier_pre_tight_switch | 0.029 | 88.1% |
| carrier_pre_tight_threaded | 0.027 | 82.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 12515ns | 12515ns | +9.18% |
| carrier_pre_tight_fntable | 11849ns | 11849ns | +3.38% |
| carrier_pre_tight_null | 10617ns | 10617ns | -7.38% |
| carrier_pre_tight_regcache | 11546ns | 11546ns | +0.73% |
| carrier_pre_tight_switch | 11462ns | 11462ns | base |
| carrier_pre_tight_threaded | 12103ns | 12103ns | +5.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 8934ns | base | --- | [8836, 9041] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 9921ns | +997.9ns (+11.2%) | [+888, +1078]ns | [9879, 9975] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_tight_fntable | 9533ns | no significant difference | [-59, +781]ns | [8890, 9668] | no | 0.2734 | 0.2188 | 0 |
| carrier_pre_tight_null | 8080ns | -860.0ns (-9.6%) | [-1056, -754]ns | [7964, 8097] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 8928ns | no significant difference | [-131, +182]ns | [8829, 9060] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_tight_threaded | 9496ns | +583.1ns (+6.5%) | [+486, +665]ns | [9482, 9567] | YES (adj: no) | 0.0521 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 8901ns | +11.2% | -6.5% | -9.1% | +2.2% | +6.5% |
| 2 | 9002ns | +11.1% | +8.1% | -12.6% | -1.6% | +6.3% |
| 3 | 9080ns | +8.6% | +5.1% | -10.8% | -0.7% | +4.4% |
| 4 | 8950ns | +11.1% | +6.4% | -9.9% | -0.4% | +6.8% |
| 5 | 8771ns | +13.2% | +9.5% | -7.9% | +1.9% | +8.2% |
| 6 | 8917ns | +11.2% | +6.1% | -9.3% | -1.3% | +6.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | -0.742 | HIGH- (thermal bounce) |
| carrier_pre_tight_fntable | -0.166 | ok |
| carrier_pre_tight_null | -0.394 | moderate- |
| carrier_pre_tight_regcache | -0.346 | moderate- |
| carrier_pre_tight_switch | 0.186 | ok |
| carrier_pre_tight_threaded | -0.732 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 0/6, lost 6/6
- **carrier_pre_tight_fntable**: won 1/6, lost 5/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 4/6, lost 2/6
- **carrier_pre_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 90128.3ns | 9924.9ns | 908.1% | HIGH |
| carrier_pre_tight_fntable | 92397.2ns | 9363.5ns | 986.8% | HIGH |
| carrier_pre_tight_null | 89358.6ns | 8046.9ns | 1110.5% | HIGH |
| carrier_pre_tight_regcache | 90138.3ns | 8938.8ns | 1008.4% | HIGH |
| carrier_pre_tight_switch | 89966.4ns | 8936.9ns | 1006.7% | HIGH |
| carrier_pre_tight_threaded | 91734.8ns | 9514.8ns | 964.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 9859.2-9974.5 ns)
   9859.2 |########################################
   9865.0 |
   9870.7 |
   9876.5 |
   9882.3 |
   9888.0 |
   9893.8 |########################################
   9899.6 |
   9905.3 |
   9911.1 |########################################
   9916.9 |
   9922.6 |########################################
   9928.4 |
   9934.2 |
   9939.9 |
   9945.7 |########################################
   9951.5 |
   9957.2 |
   9963.0 |
   9968.8 |
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 8319.2-9667.9 ns)
   8319.2 |########################################
   8386.6 |
   8454.1 |
   8521.5 |
   8588.9 |
   8656.4 |
   8723.8 |
   8791.2 |
   8858.7 |
   8926.1 |
   8993.5 |
   9061.0 |
   9128.4 |
   9195.9 |
   9263.3 |
   9330.7 |
   9398.2 |########################################
   9465.6 |########################################
   9533.0 |########################################
   9600.5 |########################################
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 7867.1-8096.6 ns)
   7867.1 |####################
   7878.6 |
   7890.1 |
   7901.5 |
   7913.0 |
   7924.5 |
   7936.0 |
   7947.4 |
   7958.9 |
   7970.4 |
   7981.9 |
   7993.4 |
   8004.8 |
   8016.3 |
   8027.8 |
   8039.3 |
   8050.7 |####################
   8062.2 |
   8073.7 |####################
   8085.2 |########################################
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 8797.5-9059.6 ns)
   8797.5 |########################################
   8810.6 |
   8823.7 |
   8836.8 |
   8849.9 |########################################
   8863.0 |
   8876.1 |
   8889.2 |
   8902.3 |
   8915.4 |########################################
   8928.5 |########################################
   8941.7 |
   8954.8 |
   8967.9 |
   8981.0 |
   8994.1 |
   9007.2 |########################################
   9020.3 |
   9033.4 |
   9046.5 |
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 8771.2-9041.0 ns)
   8771.2 |########################################
   8784.7 |
   8798.2 |
   8811.7 |
   8825.2 |
   8838.7 |
   8852.2 |
   8865.6 |
   8879.1 |
   8892.6 |########################################
   8906.1 |########################################
   8919.6 |
   8933.1 |
   8946.6 |########################################
   8960.1 |
   8973.6 |
   8987.1 |
   9000.6 |########################################
   9014.1 |
   9027.6 |
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 9480.4-9566.7 ns)
   9480.4 |########################################
   9484.7 |
   9489.0 |####################
   9493.3 |
   9497.7 |####################
   9502.0 |
   9506.3 |
   9510.6 |
   9514.9 |
   9519.2 |
   9523.5 |
   9527.9 |
   9532.2 |
   9536.5 |
   9540.8 |
   9545.1 |
   9549.4 |
   9553.8 |
   9558.1 |####################
   9562.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=908.5% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=969.9% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=1103.9% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=1006.3% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=1006.7% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=965.3% of algo (FFI overhead may distort results)
