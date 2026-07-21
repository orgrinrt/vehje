# tnum multiply: always-loop vs known-operand fast path (abstract arith)

2 variants, 6 samples per variant.
Baseline: **tm_fastpath**

## Highlights

Baseline for all deltas below: **tm_fastpath**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tm_fastpath dominates: 176% faster than the next best (tm_loop)

tm_fastpath (482.72 us) leads tm_loop (1.33 ms) by 176%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### tm_loop shows alternating (throttle bounce) (autocorr -0.56)

tm_loop's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (tm_fastpath)

The baseline tm_fastpath is the fastest (482.72 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (tm_fastpath) is the fastest** at 482723.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.76x (fastest 482723.5 ns, slowest 1331412.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tm_fastpath | 486323ns | 485484ns | 467761ns | 480252ns | 504709ns | base |
| tm_loop | 1371894ns | 1334170ns | 1265651ns | 1330340ns | 1487346ns | +182.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tm_fastpath | 483878ns | 465555ns | 502351ns | base | 0.008 |
| tm_loop | 1369006ns | 1262924ns | 1484129ns | +182.92% | 0.003 |

## Performance model

- Peak throughput: **0.009 Gops/s** (tm_fastpath; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tm_fastpath | 0.008 | 96.4% |
| tm_loop | 0.003 | 35.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tm_fastpath | 486323ns | 486323ns | base |
| tm_loop | 1371894ns | 1371894ns | +182.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tm_fastpath | 482724ns | base | --- | [466561, 502351] | --- | --- | --- | --- |
| tm_loop | 1331413ns | +849637.5ns (+176.0%) | [+819016, +986728]ns | [1291475, 1484129] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tm_fastpath | tm_loop |
|---|---|---|
| 1 | 467566ns | +170.1% |
| 2 | 507924ns | +188.3% |
| 3 | 496778ns | +170.1% |
| 4 | 465555ns | +183.5% |
| 5 | 486877ns | +208.9% |
| 6 | 478570ns | +176.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tm_fastpath | -0.281 | moderate- |
| tm_loop | -0.555 | HIGH- (thermal bounce) |

**Consistency summary:**

- **tm_loop**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tm_fastpath | 6.0ns | 483878.3ns | 0.0% |  |
| tm_loop | 54.4ns | 1369005.5ns | 0.0% |  |

## Distribution (algo ns)

```
tm_fastpath (n=6, range 465555.4-502350.8 ns)
  465555.4 |########################################
  467395.2 |########################################
  469234.9 |
  471074.7 |
  472914.5 |
  474754.3 |
  476594.0 |
  478433.8 |########################################
  480273.6 |
  482113.4 |
  483953.1 |
  485792.9 |########################################
  487632.7 |
  489472.4 |
  491312.2 |
  493152.0 |
  494991.8 |########################################
  496831.5 |
  498671.3 |
  500511.1 |
  (0 below, 1 above range)

tm_loop (n=6, range 1262923.7-1484129.1 ns)
  1262923.7 |####################
  1273984.0 |
  1285044.2 |
  1296104.5 |
  1307164.8 |
  1318225.1 |########################################
  1329285.3 |
  1340345.6 |####################
  1351405.9 |
  1362466.2 |
  1373526.4 |
  1384586.7 |
  1395647.0 |
  1406707.2 |
  1417767.5 |
  1428827.8 |
  1439888.1 |
  1450948.3 |
  1462008.6 |####################
  1473068.9 |
  (0 below, 1 above range)

```
