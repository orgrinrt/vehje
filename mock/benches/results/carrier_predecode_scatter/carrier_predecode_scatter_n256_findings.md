# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null dominates: 24% faster than the next best (carrier_pre_scatter_direct)

carrier_pre_scatter_null (5.90 us) leads carrier_pre_scatter_direct (7.31 us) by 24%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_scatter_null beats baseline by 32% (significant)

carrier_pre_scatter_null is -2.77 us (32%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 2.1x slower than the field

carrier_pre_scatter_fntable (12.57 us) is 2.1x the fastest (5.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_switch, carrier_pre_scatter_threaded, carrier_pre_scatter_regcache} vs {carrier_pre_scatter_fntable} (29% apart)

The field splits into a fast tier {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_switch, carrier_pre_scatter_threaded, carrier_pre_scatter_regcache} and a slow tier {carrier_pre_scatter_fntable} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_pre_scatter_threaded's edge over baseline is significant but tiny (0 ns, 0.00%)

carrier_pre_scatter_threaded differs from baseline carrier_pre_scatter_switch by 0 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 5896.2 ns median (-32.1% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.13x (fastest 5896.2 ns, slowest 12567.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 9552ns | 9565ns | 9122ns | 9482ns | 9870ns | -14.49% |
| carrier_pre_scatter_fntable | 15020ns | 15127ns | 14394ns | 14980ns | 15391ns | +34.47% |
| carrier_pre_scatter_null | 8231ns | 8224ns | 8075ns | 8193ns | 8366ns | -26.31% |
| carrier_pre_scatter_regcache | 12321ns | 12255ns | 12049ns | 12199ns | 12641ns | +10.31% |
| carrier_pre_scatter_switch | 11170ns | 11112ns | 10948ns | 11059ns | 11446ns | base |
| carrier_pre_scatter_threaded | 11137ns | 11170ns | 10924ns | 11102ns | 11296ns | -0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 7278ns | 6957ns | 7492ns | -16.96% | 0.035 |
| carrier_pre_scatter_fntable | 12500ns | 11957ns | 12792ns | +42.63% | 0.020 |
| carrier_pre_scatter_null | 5897ns | 5718ns | 6015ns | -32.71% | 0.043 |
| carrier_pre_scatter_regcache | 9884ns | 9550ns | 10253ns | +12.78% | 0.026 |
| carrier_pre_scatter_switch | 8764ns | 8642ns | 8957ns | base | 0.029 |
| carrier_pre_scatter_threaded | 8679ns | 8512ns | 8767ns | -0.97% | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 294959 | 853252 | 0.346 | 0.98× |
| carrier_pre_scatter_fntable | 318490 | 848887 | 0.375 | 1.06× |
| carrier_pre_scatter_null | 296945 | 1387140 | 0.214 | 0.99× |
| carrier_pre_scatter_regcache | 312806 | 1211595 | 0.258 | 1.04× |
| carrier_pre_scatter_switch | 299451 | 942979 | 0.318 | 1.00× |
| carrier_pre_scatter_threaded | 306898 | 1038355 | 0.296 | 1.02× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.045 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.035 | 78.2% |
| carrier_pre_scatter_fntable | 0.020 | 45.5% |
| carrier_pre_scatter_null | 0.043 | 97.0% |
| carrier_pre_scatter_regcache | 0.026 | 58.5% |
| carrier_pre_scatter_switch | 0.029 | 65.9% |
| carrier_pre_scatter_threaded | 0.029 | 65.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 9552ns | 9552ns | -14.49% |
| carrier_pre_scatter_fntable | 15020ns | 15020ns | +34.47% |
| carrier_pre_scatter_null | 8231ns | 8231ns | -26.31% |
| carrier_pre_scatter_regcache | 12321ns | 12321ns | +10.31% |
| carrier_pre_scatter_switch | 11170ns | 11170ns | base |
| carrier_pre_scatter_threaded | 11137ns | 11137ns | -0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 8680ns | base | --- | [8655, 8957] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 7311ns | -1467.9ns (-16.9%) | [-1775, -1215]ns | [7031, 7492] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 12567ns | +3793.9ns (+43.7%) | [+3337, +4077]ns | [12142, 12792] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_scatter_null | 5896ns | -2769.6ns (-31.9%) | [-3177, -2655]ns | [5781, 6015] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 9778ns | +1110.4ns (+12.8%) | [+676, +1573]ns | [9621, 10253] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_scatter_threaded | 8709ns | no significant difference | [-342, +87]ns | [8561, 8767] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 8942ns | -20.5% | +33.7% | -36.1% | +8.7% | -2.5% |
| 2 | 8689ns | -14.9% | +44.1% | -32.2% | +16.6% | +1.3% |
| 3 | 8642ns | -13.1% | +48.8% | -31.7% | +13.8% | +0.7% |
| 4 | 8672ns | -19.8% | +45.4% | -30.6% | +19.6% | +0.7% |
| 5 | 8668ns | -16.6% | +42.2% | -30.7% | +11.8% | -0.7% |
| 6 | 8972ns | -16.7% | +41.8% | -34.9% | +6.4% | -5.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | -0.253 | moderate- |
| carrier_pre_scatter_fntable | -0.045 | ok |
| carrier_pre_scatter_null | 0.140 | ok |
| carrier_pre_scatter_regcache | -0.220 | moderate- |
| carrier_pre_scatter_switch | -0.036 | ok |
| carrier_pre_scatter_threaded | 0.303 | moderate+ |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 0/6, lost 6/6
- **carrier_pre_scatter_threaded**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 88710.0ns | 7278.1ns | 1218.9% | HIGH |
| carrier_pre_scatter_fntable | 90479.8ns | 12500.5ns | 723.8% | HIGH |
| carrier_pre_scatter_null | 89471.9ns | 5897.3ns | 1517.2% | HIGH |
| carrier_pre_scatter_regcache | 91587.5ns | 9884.2ns | 926.6% | HIGH |
| carrier_pre_scatter_switch | 88864.7ns | 8764.3ns | 1013.9% | HIGH |
| carrier_pre_scatter_threaded | 92210.8ns | 8679.2ns | 1062.4% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 6956.7-7491.9 ns)
   6956.7 |########################################
   6983.5 |
   7010.2 |
   7037.0 |
   7063.7 |
   7090.5 |########################################
   7117.2 |
   7144.0 |
   7170.8 |
   7197.5 |
   7224.3 |########################################
   7251.0 |
   7277.8 |
   7304.5 |
   7331.3 |
   7358.1 |
   7384.8 |########################################
   7411.6 |
   7438.3 |
   7465.1 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 11957.1-12792.3 ns)
  11957.1 |########################################
  11998.9 |
  12040.6 |
  12082.4 |
  12124.1 |
  12165.9 |
  12207.7 |
  12249.4 |
  12291.2 |########################################
  12332.9 |
  12374.7 |
  12416.5 |
  12458.2 |
  12500.0 |########################################
  12541.7 |
  12583.5 |########################################
  12625.3 |
  12667.0 |
  12708.8 |########################################
  12750.5 |
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 5718.3-6015.2 ns)
   5718.3 |########################################
   5733.1 |
   5748.0 |
   5762.8 |
   5777.7 |
   5792.5 |
   5807.4 |
   5822.2 |
   5837.1 |########################################
   5851.9 |
   5866.8 |
   5881.6 |########################################
   5896.4 |########################################
   5911.3 |
   5926.1 |
   5941.0 |
   5955.8 |
   5970.7 |
   5985.5 |
   6000.4 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 9549.6-10253.3 ns)
   9549.6 |####################
   9584.8 |
   9620.0 |
   9655.2 |
   9690.3 |########################################
   9725.5 |
   9760.7 |
   9795.9 |
   9831.1 |####################
   9866.3 |
   9901.5 |
   9936.6 |
   9971.8 |
  10007.0 |
  10042.2 |
  10077.4 |
  10112.6 |####################
  10147.7 |
  10182.9 |
  10218.1 |
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 8642.5-8957.3 ns)
   8642.5 |####################
   8658.2 |########################################
   8674.0 |####################
   8689.7 |
   8705.5 |
   8721.2 |
   8736.9 |
   8752.7 |
   8768.4 |
   8784.2 |
   8799.9 |
   8815.6 |
   8831.4 |
   8847.1 |
   8862.9 |
   8878.6 |
   8894.3 |
   8910.1 |
   8925.8 |
   8941.6 |####################
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 8511.7-8767.0 ns)
   8511.7 |########################################
   8524.5 |
   8537.2 |
   8550.0 |
   8562.8 |
   8575.5 |
   8588.3 |
   8601.1 |########################################
   8613.8 |
   8626.6 |
   8639.4 |
   8652.1 |
   8664.9 |
   8677.7 |
   8690.4 |########################################
   8703.2 |
   8716.0 |########################################
   8728.7 |########################################
   8741.5 |
   8754.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=1209.7% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=716.8% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=1516.3% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=932.9% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=1022.6% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=1063.8% of algo (FFI overhead may distort results)
