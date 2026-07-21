# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_rec24)

The baseline carrier_rec24 is the fastest (9.14 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader carrier_rec24 vs stability leader carrier_rec32 (+6% speed for 1.3x steadier)

carrier_rec24 is fastest (9.14 us, CV 8.0%); carrier_rec32 gives up 6.1% median for 1.3x lower variance (CV 6.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (carrier_rec24) is the fastest** at 9142.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.12x (fastest 9142.5 ns, slowest 10271.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 12014ns | 11771ns | 10618ns | 11684ns | 13206ns | +3.24% |
| carrier_rec16 | 11764ns | 11655ns | 10820ns | 11430ns | 12738ns | +1.10% |
| carrier_rec20 | 12878ns | 12805ns | 11017ns | 12555ns | 14293ns | +10.67% |
| carrier_rec24 | 11636ns | 11399ns | 10626ns | 11145ns | 12879ns | base |
| carrier_rec32 | 12160ns | 12088ns | 11122ns | 11988ns | 12937ns | +4.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 9691ns | 8522ns | 10683ns | +4.14% | 0.026 |
| carrier_rec16 | 9435ns | 8686ns | 10219ns | +1.39% | 0.027 |
| carrier_rec20 | 10332ns | 8899ns | 11447ns | +11.02% | 0.025 |
| carrier_rec24 | 9306ns | 8524ns | 10253ns | base | 0.028 |
| carrier_rec32 | 9778ns | 8915ns | 10411ns | +5.07% | 0.026 |

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.027 | 89.8% |
| carrier_rec16 | 0.027 | 91.2% |
| carrier_rec20 | 0.025 | 83.0% |
| carrier_rec24 | 0.028 | 93.2% |
| carrier_rec32 | 0.026 | 87.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 12014ns | 12014ns | +3.24% |
| carrier_rec16 | 11764ns | 11764ns | +1.10% |
| carrier_rec20 | 12878ns | 12878ns | +10.67% |
| carrier_rec24 | 11636ns | 11636ns | base |
| carrier_rec32 | 12160ns | 12160ns | +4.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 9142ns | base | --- | [8524, 10253] | --- | --- | --- | --- |
| carrier_rec12 | 9490ns | no significant difference | [-101, +908]ns | [8901, 10683] | no | 0.6875 | 0.6875 | 0 |
| carrier_rec16 | 9347ns | no significant difference | [-479, +650]ns | [8740, 10219] | no | 0.6875 | 0.6875 | 0 |
| carrier_rec20 | 10272ns | +800.2ns (+8.8%) | [+444, +1832]ns | [9275, 11447] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_rec32 | 9700ns | +410.2ns (+4.5%) | [+147, +858]ns | [9222, 10411] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 8524ns | -0.0% | +1.9% | +4.4% | +4.6% |
| 2 | 9146ns | +5.7% | +7.9% | +7.7% | +4.4% |
| 3 | 8524ns | +8.9% | +3.2% | +25.5% | +11.8% |
| 4 | 9139ns | +1.9% | -3.4% | +5.6% | +7.8% |
| 5 | 9969ns | +10.6% | +5.8% | +14.9% | -1.0% |
| 6 | 10537ns | -1.9% | -6.2% | +8.5% | +3.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | 0.140 | ok |
| carrier_rec16 | -0.127 | ok |
| carrier_rec20 | 0.139 | ok |
| carrier_rec24 | 0.336 | moderate+ |
| carrier_rec32 | 0.156 | ok |

**Consistency summary:**

- **carrier_rec12**: won 1/6, lost 4/6
- **carrier_rec16**: won 2/6, lost 4/6
- **carrier_rec20**: won 0/6, lost 6/6
- **carrier_rec32**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 76.7ns | 9691.4ns | 0.8% |  |
| carrier_rec16 | 79.0ns | 9435.3ns | 0.8% |  |
| carrier_rec20 | 87.1ns | 10331.5ns | 0.8% |  |
| carrier_rec24 | 76.9ns | 9306.3ns | 0.8% |  |
| carrier_rec32 | 78.6ns | 9777.9ns | 0.8% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 8522.1-10683.2 ns)
   8522.1 |####################
   8630.2 |
   8738.2 |
   8846.3 |
   8954.3 |
   9062.4 |
   9170.4 |
   9278.5 |########################################
   9386.5 |
   9494.6 |
   9602.6 |####################
   9710.7 |
   9818.7 |
   9926.8 |
  10034.8 |
  10142.9 |
  10250.9 |####################
  10359.0 |
  10467.0 |
  10575.1 |
  (0 below, 1 above range)

carrier_rec16 (n=6, range 8685.8-10219.1 ns)
   8685.8 |####################
   8762.5 |########################################
   8839.1 |
   8915.8 |
   8992.5 |
   9069.1 |
   9145.8 |
   9222.5 |
   9299.1 |
   9375.8 |
   9452.5 |
   9529.1 |
   9605.8 |
   9682.5 |
   9759.1 |
   9835.8 |########################################
   9912.5 |
   9989.1 |
  10065.8 |
  10142.5 |
  (0 below, 1 above range)

carrier_rec20 (n=6, range 8899.2-11447.5 ns)
   8899.2 |########################################
   9026.6 |
   9154.0 |
   9281.4 |
   9408.9 |
   9536.3 |########################################
   9663.7 |
   9791.1 |########################################
   9918.5 |
  10045.9 |
  10173.3 |
  10300.7 |
  10428.2 |
  10555.6 |
  10683.0 |########################################
  10810.4 |
  10937.8 |
  11065.2 |
  11192.6 |
  11320.0 |########################################
  (0 below, 1 above range)

carrier_rec24 (n=6, range 8523.8-10252.8 ns)
   8523.8 |########################################
   8610.2 |
   8696.7 |
   8783.1 |
   8869.6 |
   8956.0 |
   9042.5 |
   9128.9 |########################################
   9215.4 |
   9301.8 |
   9388.3 |
   9474.7 |
   9561.2 |
   9647.6 |
   9734.1 |
   9820.5 |
   9907.0 |####################
   9993.4 |
  10079.9 |
  10166.3 |
  (0 below, 1 above range)

carrier_rec32 (n=6, range 8915.4-10411.5 ns)
   8915.4 |####################
   8990.2 |
   9065.0 |
   9139.8 |
   9214.6 |
   9289.4 |
   9364.2 |
   9439.0 |
   9513.8 |########################################
   9588.6 |
   9663.4 |
   9738.2 |
   9813.0 |########################################
   9887.8 |
   9962.6 |
  10037.4 |
  10112.2 |
  10187.0 |
  10261.8 |
  10336.6 |
  (0 below, 1 above range)

```
