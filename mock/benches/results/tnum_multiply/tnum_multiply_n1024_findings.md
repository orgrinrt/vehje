# tnum multiply: always-loop vs known-operand fast path (abstract arith)

2 variants, 6 samples per variant.
Baseline: **tm_fastpath**

## Highlights

Baseline for all deltas below: **tm_fastpath**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tm_fastpath dominates: 166% faster than the next best (tm_loop)

tm_fastpath (121.28 us) leads tm_loop (322.17 us) by 166%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### tm_loop shows alternating (throttle bounce) (autocorr -0.64)

tm_loop's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (tm_fastpath)

The baseline tm_fastpath is the fastest (121.28 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (tm_fastpath) is the fastest** at 121275.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.66x (fastest 121275.8 ns, slowest 322173.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tm_fastpath | 123487ns | 123653ns | 117850ns | 122586ns | 127656ns | base |
| tm_loop | 324371ns | 324647ns | 317765ns | 323030ns | 329685ns | +162.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tm_fastpath | 121087ns | 115665ns | 125038ns | base | 0.008 |
| tm_loop | 321797ns | 315072ns | 327103ns | +165.76% | 0.003 |

## Performance model

- Peak throughput: **0.009 Gops/s** (tm_fastpath; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tm_fastpath | 0.008 | 95.4% |
| tm_loop | 0.003 | 35.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tm_fastpath | 123487ns | 123487ns | base |
| tm_loop | 324371ns | 324371ns | +162.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tm_fastpath | 121276ns | base | --- | [116948, 125038] | --- | --- | --- | --- |
| tm_loop | 322174ns | +200815.0ns (+165.6%) | [+192883, +208431]ns | [316113, 327103] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tm_fastpath | tm_loop |
|---|---|---|
| 1 | 115665ns | +181.8% |
| 2 | 118231ns | +170.9% |
| 3 | 121678ns | +169.8% |
| 4 | 125588ns | +150.9% |
| 5 | 124487ns | +160.3% |
| 6 | 120874ns | +162.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tm_fastpath | 0.445 | moderate+ |
| tm_loop | -0.638 | HIGH- (thermal bounce) |

**Consistency summary:**

- **tm_loop**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tm_fastpath | 9.7ns | 121087.1ns | 0.0% |  |
| tm_loop | 16.9ns | 321796.8ns | 0.0% |  |

## Distribution (algo ns)

```
tm_fastpath (n=6, range 115665.4-125037.5 ns)
  115665.4 |########################################
  116134.0 |
  116602.6 |
  117071.2 |
  117539.8 |
  118008.4 |########################################
  118477.0 |
  118945.6 |
  119414.2 |
  119882.8 |
  120351.4 |
  120820.1 |########################################
  121288.7 |########################################
  121757.3 |
  122225.9 |
  122694.5 |
  123163.1 |
  123631.7 |
  124100.3 |########################################
  124568.9 |
  (0 below, 1 above range)

tm_loop (n=6, range 315072.5-327103.1 ns)
  315072.5 |########################################
  315674.0 |
  316275.6 |
  316877.1 |########################################
  317478.6 |
  318080.2 |
  318681.7 |
  319283.2 |
  319884.7 |########################################
  320486.3 |
  321087.8 |
  321689.3 |
  322290.9 |
  322892.4 |
  323493.9 |########################################
  324095.4 |
  324697.0 |
  325298.5 |
  325900.0 |########################################
  326501.6 |
  (0 below, 1 above range)

```
