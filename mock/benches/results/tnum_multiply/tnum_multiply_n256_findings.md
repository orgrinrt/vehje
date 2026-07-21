# tnum multiply: always-loop vs known-operand fast path (abstract arith)

2 variants, 6 samples per variant.
Baseline: **tm_fastpath**

## Highlights

Baseline for all deltas below: **tm_fastpath**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tm_fastpath dominates: 156% faster than the next best (tm_loop)

tm_fastpath (35.00 us) leads tm_loop (89.43 us) by 156%, a clear separation rather than a photo finish. CV 37.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### tm_fastpath is fastest but the noisiest (CV 37.1%)

tm_fastpath wins on median (35.00 us) yet has the highest variance (CV 37.1%), while tm_loop is the steadiest (CV 10.3%, 89.43 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (tm_fastpath)

The baseline tm_fastpath is the fastest (35.00 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### tm_fastpath is inconsistent: worst-20% is 1.6x its best-20%

tm_fastpath's best 20% of batches run at 32.39 us but its worst 20% at 53.02 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (tm_fastpath) is the fastest** at 34999.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.56x (fastest 34999.8 ns, slowest 89426.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tm_fastpath | 42785ns | 37474ns | 34638ns | 36703ns | 55982ns | base |
| tm_loop | 92408ns | 91921ns | 82685ns | 88948ns | 102459ns | +115.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tm_fastpath | 40224ns | 32393ns | 53024ns | base | 0.006 |
| tm_loop | 89830ns | 80451ns | 99440ns | +123.32% | 0.003 |

## Performance model

- Peak throughput: **0.008 Gops/s** (tm_fastpath; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tm_fastpath | 0.007 | 92.6% |
| tm_loop | 0.003 | 36.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tm_fastpath | 42785ns | 42785ns | base |
| tm_loop | 92408ns | 92408ns | +115.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tm_fastpath | 35000ns | base | --- | [32649, 53024] | --- | --- | --- | --- |
| tm_loop | 89427ns | +50666.0ns (+144.8%) | [+43339, +54812]ns | [80623, 99440] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tm_fastpath | tm_loop |
|---|---|---|
| 1 | 32393ns | +149.4% |
| 2 | 34710ns | +161.3% |
| 3 | 35289ns | +152.0% |
| 4 | 36997ns | +143.1% |
| 5 | 69051ns | +56.7% |
| 6 | 32906ns | +144.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tm_fastpath | -0.215 | moderate- |
| tm_loop | -0.352 | moderate- |

**Consistency summary:**

- **tm_loop**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tm_fastpath | 9.4ns | 40224.3ns | 0.0% |  |
| tm_loop | 12.7ns | 89830.0ns | 0.0% |  |

## Distribution (algo ns)

```
tm_fastpath (n=6, range 32392.9-53023.8 ns)
  32392.9 |########################################
  33424.4 |
  34456.0 |########################################
  35487.5 |
  36519.1 |####################
  37550.6 |
  38582.2 |
  39613.7 |
  40645.2 |
  41676.8 |
  42708.3 |
  43739.9 |
  44771.4 |
  45803.0 |
  46834.5 |
  47866.0 |
  48897.6 |
  49929.1 |
  50960.7 |
  51992.2 |
  (0 below, 1 above range)

tm_loop (n=6, range 80451.2-99440.0 ns)
  80451.2 |########################################
  81400.6 |
  82350.1 |
  83299.5 |
  84249.0 |
  85198.4 |
  86147.8 |
  87097.3 |
  88046.7 |####################
  88996.2 |####################
  89945.6 |####################
  90895.0 |
  91844.5 |
  92793.9 |
  93743.4 |
  94692.8 |
  95642.2 |
  96591.7 |
  97541.1 |
  98490.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **tm_fastpath**: CV=32.3% (high variance, measurements may be unstable)
