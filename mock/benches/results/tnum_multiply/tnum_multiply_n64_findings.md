# tnum multiply: always-loop vs known-operand fast path (abstract arith)

2 variants, 6 samples per variant.
Baseline: **tm_fastpath**

## Highlights

Baseline for all deltas below: **tm_fastpath**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tm_fastpath dominates: 120% faster than the next best (tm_loop)

tm_fastpath (10.85 us) leads tm_loop (23.88 us) by 120%, a clear separation rather than a photo finish. CV 11.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### tm_fastpath is fastest but the noisiest (CV 11.2%)

tm_fastpath wins on median (10.85 us) yet has the highest variance (CV 11.2%), while tm_loop is the steadiest (CV 10.5%, 23.88 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (tm_fastpath)

The baseline tm_fastpath is the fastest (10.85 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (tm_fastpath) is the fastest** at 10854.4 ns median
- 1 variant significantly slower than baseline
- Spread: 2.20x (fastest 10854.4 ns, slowest 23884.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tm_fastpath | 13192ns | 13447ns | 10533ns | 13010ns | 14792ns | base |
| tm_loop | 26899ns | 26504ns | 22868ns | 26044ns | 30198ns | +103.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tm_fastpath | 10540ns | 8344ns | 11730ns | base | 0.006 |
| tm_loop | 24250ns | 20610ns | 27242ns | +130.07% | 0.003 |

## Performance model

- Peak throughput: **0.008 Gops/s** (tm_fastpath; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tm_fastpath | 0.006 | 76.9% |
| tm_loop | 0.003 | 34.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tm_fastpath | 13192ns | 13192ns | base |
| tm_loop | 26899ns | 26899ns | +103.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tm_fastpath | 10854ns | base | --- | [9036, 11730] | --- | --- | --- | --- |
| tm_loop | 23884ns | +13108.1ns (+120.8%) | [+12510, +15512]ns | [21625, 27242] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tm_fastpath | tm_loop |
|---|---|---|
| 1 | 12113ns | +136.3% |
| 2 | 11348ns | +127.9% |
| 3 | 10974ns | +116.2% |
| 4 | 9729ns | +132.7% |
| 5 | 10735ns | +124.0% |
| 6 | 8344ns | +147.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tm_fastpath | 0.077 | ok |
| tm_loop | 0.215 | moderate+ |

**Consistency summary:**

- **tm_loop**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tm_fastpath | 4.0ns | 10540.4ns | 0.0% |  |
| tm_loop | 3.8ns | 24250.3ns | 0.0% |  |

## Distribution (algo ns)

```
tm_fastpath (n=6, range 8343.8-11730.4 ns)
   8343.8 |########################################
   8513.1 |
   8682.5 |
   8851.8 |
   9021.1 |
   9190.4 |
   9359.8 |
   9529.1 |
   9698.4 |########################################
   9867.8 |
  10037.1 |
  10206.4 |
  10375.8 |
  10545.1 |
  10714.4 |########################################
  10883.8 |########################################
  11053.1 |
  11222.4 |########################################
  11391.7 |
  11561.1 |
  (0 below, 1 above range)

tm_loop (n=6, range 20610.4-27242.1 ns)
  20610.4 |########################################
  20942.0 |
  21273.6 |
  21605.1 |
  21936.7 |
  22268.3 |
  22599.9 |########################################
  22931.5 |
  23263.1 |
  23594.6 |########################################
  23926.2 |########################################
  24257.8 |
  24589.4 |
  24921.0 |
  25252.6 |
  25584.1 |########################################
  25915.7 |
  26247.3 |
  26578.9 |
  26910.5 |
  (0 below, 1 above range)

```
