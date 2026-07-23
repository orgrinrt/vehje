# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 25% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (25.67 us) leads carrier_pre_real_direct (32.17 us) by 25%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 32% (significant)

carrier_pre_real_null is -12.21 us (32%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 2.1x slower than the field

carrier_pre_real_fntable (53.64 us) is 2.1x the fastest (25.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_real_regcache shows alternating (throttle bounce) (autocorr -0.65)

carrier_pre_real_regcache's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_real_null, carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache} vs {carrier_pre_real_fntable} (26% apart)

The field splits into a fast tier {carrier_pre_real_null, carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache} and a slow tier {carrier_pre_real_fntable} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_pre_real_switch is inconsistent: worst-20% is 1.6x its best-20%

carrier_pre_real_switch's best 20% of batches run at 36.68 us but its worst 20% at 59.19 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_pre_real_null** at 25673.8 ns median (-32.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.09x (fastest 25673.8 ns, slowest 53636.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 34557ns | 34452ns | 33953ns | 34376ns | 35130ns | -26.46% |
| carrier_pre_real_fntable | 54897ns | 55902ns | 51986ns | 54751ns | 56573ns | +16.83% |
| carrier_pre_real_null | 28002ns | 27989ns | 27553ns | 27933ns | 28330ns | -40.41% |
| carrier_pre_real_regcache | 44513ns | 44889ns | 42984ns | 44516ns | 45273ns | -5.27% |
| carrier_pre_real_switch | 46987ns | 40330ns | 38869ns | 40077ns | 61412ns | base |
| carrier_pre_real_threaded | 39139ns | 39102ns | 38515ns | 38962ns | 39716ns | -16.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 32268ns | 31701ns | 32816ns | -27.88% | 0.032 |
| carrier_pre_real_fntable | 52672ns | 49888ns | 54281ns | +17.72% | 0.019 |
| carrier_pre_real_null | 25739ns | 25350ns | 26067ns | -42.48% | 0.040 |
| carrier_pre_real_regcache | 42252ns | 40800ns | 42971ns | -5.57% | 0.024 |
| carrier_pre_real_switch | 44743ns | 36682ns | 59185ns | base | 0.023 |
| carrier_pre_real_threaded | 36850ns | 36278ns | 37401ns | -17.64% | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_real_direct | 395136 | 1065352 | 0.371 | 0.75× |
| carrier_pre_real_fntable | 481273 | 1058676 | 0.455 | 0.92× |
| carrier_pre_real_null | 399622 | 1744611 | 0.229 | 0.76× |
| carrier_pre_real_regcache | 480480 | 1770726 | 0.271 | 0.92× |
| carrier_pre_real_switch | 525056 | 1383784 | 0.379 | 1.00× |
| carrier_pre_real_threaded | 446536 | 1440910 | 0.310 | 0.85× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.040 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.032 | 78.8% |
| carrier_pre_real_fntable | 0.019 | 47.3% |
| carrier_pre_real_null | 0.040 | 98.7% |
| carrier_pre_real_regcache | 0.024 | 59.5% |
| carrier_pre_real_switch | 0.027 | 66.6% |
| carrier_pre_real_threaded | 0.028 | 68.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 34557ns | 34557ns | -26.46% |
| carrier_pre_real_fntable | 54897ns | 54897ns | +16.83% |
| carrier_pre_real_null | 28002ns | 28002ns | -40.41% |
| carrier_pre_real_regcache | 44513ns | 44513ns | -5.27% |
| carrier_pre_real_switch | 46987ns | 46987ns | base |
| carrier_pre_real_threaded | 39139ns | 39139ns | -16.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 38048ns | base | --- | [36997, 59185] | --- | --- | --- | --- |
| carrier_pre_real_direct | 32172ns | -6153.4ns (-16.2%) | [-26754, -4519]ns | [31816, 32816] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_real_fntable | 53636ns | no significant difference | [-7103, +16965]ns | [50100, 54281] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_real_null | 25674ns | -12208.5ns (-32.1%) | [-33648, -11158]ns | [25475, 26067] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_real_regcache | 42612ns | no significant difference | [-17218, +5691]ns | [41173, 42971] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_real_threaded | 36808ns | -1449.6ns (-3.8%) | [-22176, -53]ns | [36342, 37401] | YES (adj: no) | 0.3646 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 38146ns | -16.9% | +30.8% | -32.9% | +11.8% | -4.9% |
| 2 | 36682ns | -10.0% | +48.2% | -29.0% | +16.1% | +0.0% |
| 3 | 37949ns | -15.4% | +42.9% | -31.3% | +9.5% | -2.7% |
| 4 | 37312ns | -14.4% | +43.2% | -31.3% | +14.7% | -0.3% |
| 5 | 53899ns | -40.2% | -0.1% | -53.0% | -24.3% | -30.2% |
| 6 | 64472ns | -49.4% | -22.0% | -60.1% | -33.1% | -43.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | -0.437 | moderate- |
| carrier_pre_real_fntable | -0.138 | ok |
| carrier_pre_real_null | 0.186 | ok |
| carrier_pre_real_regcache | -0.651 | HIGH- (thermal bounce) |
| carrier_pre_real_switch | 0.397 | moderate+ |
| carrier_pre_real_threaded | 0.018 | ok |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 1/6, lost 4/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 2/6, lost 4/6
- **carrier_pre_real_threaded**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 100262.8ns | 32268.1ns | 310.7% | HIGH |
| carrier_pre_real_fntable | 105310.0ns | 52672.1ns | 199.9% | HIGH |
| carrier_pre_real_null | 105461.4ns | 25738.5ns | 409.7% | HIGH |
| carrier_pre_real_regcache | 117249.4ns | 42252.0ns | 277.5% | HIGH |
| carrier_pre_real_switch | 129587.9ns | 44743.4ns | 289.6% | HIGH |
| carrier_pre_real_threaded | 113165.8ns | 36850.3ns | 307.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 31700.8-32816.1 ns)
  31700.8 |########################################
  31756.6 |
  31812.3 |
  31868.1 |
  31923.8 |########################################
  31979.6 |
  32035.4 |########################################
  32091.1 |
  32146.9 |
  32202.7 |########################################
  32258.4 |
  32314.2 |
  32370.0 |
  32425.7 |
  32481.5 |
  32537.2 |
  32593.0 |########################################
  32648.8 |
  32704.5 |
  32760.3 |
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 49888.3-54280.6 ns)
  49888.3 |########################################
  50107.9 |########################################
  50327.5 |
  50547.2 |
  50766.8 |
  50986.4 |
  51206.0 |
  51425.6 |
  51645.2 |
  51864.9 |
  52084.5 |
  52304.1 |
  52523.7 |
  52743.3 |
  52962.9 |
  53182.6 |
  53402.2 |########################################
  53621.8 |
  53841.4 |########################################
  54061.0 |########################################
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 25350.0-26067.2 ns)
  25350.0 |########################################
  25385.9 |
  25421.7 |
  25457.6 |
  25493.5 |
  25529.3 |
  25565.2 |########################################
  25601.0 |########################################
  25636.9 |
  25672.8 |
  25708.6 |########################################
  25744.5 |
  25780.3 |
  25816.2 |
  25852.1 |
  25887.9 |
  25923.8 |
  25959.7 |
  25995.5 |
  26031.4 |########################################
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 40800.0-42970.8 ns)
  40800.0 |########################################
  40908.5 |
  41017.1 |
  41125.6 |
  41234.2 |
  41342.7 |
  41451.2 |########################################
  41559.8 |
  41668.3 |
  41776.9 |
  41885.4 |
  41993.9 |
  42102.5 |
  42211.0 |
  42319.6 |
  42428.1 |
  42536.6 |########################################
  42645.2 |########################################
  42753.7 |########################################
  42862.3 |
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 36682.1-59185.4 ns)
  36682.1 |########################################
  37807.3 |########################################
  38932.4 |
  40057.6 |
  41182.8 |
  42307.9 |
  43433.1 |
  44558.3 |
  45683.4 |
  46808.6 |
  47933.8 |
  49058.9 |
  50184.1 |
  51309.3 |
  52434.4 |
  53559.6 |####################
  54684.8 |
  55809.9 |
  56935.1 |
  58060.3 |
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 36277.5-37400.6 ns)
  36277.5 |########################################
  36333.7 |
  36389.8 |########################################
  36446.0 |
  36502.1 |
  36558.3 |
  36614.4 |
  36670.6 |########################################
  36726.7 |
  36782.9 |
  36839.1 |
  36895.2 |########################################
  36951.4 |
  37007.5 |
  37063.7 |
  37119.8 |
  37176.0 |########################################
  37232.1 |
  37288.3 |
  37344.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=310.6% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=199.1% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=409.8% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=272.3% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: CV=23.8% (high variance, measurements may be unstable)
- **carrier_pre_real_switch**: bridge=307.5% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=307.1% of algo (FFI overhead may distort results)
