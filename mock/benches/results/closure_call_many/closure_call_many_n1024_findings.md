# Closure representation: create-once-call-many, flat vs linked (access cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_call_many_flat**

## Highlights

Baseline for all deltas below: **closure_call_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### closure_call_many_flat dominates: 190% faster than the next best (closure_call_many_linked)

closure_call_many_flat (44.12 us) leads closure_call_many_linked (128.07 us) by 190%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (closure_call_many_flat)

The baseline closure_call_many_flat is the fastest (44.12 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (closure_call_many_flat) is the fastest** at 44124.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.90x (fastest 44124.8 ns, slowest 128068.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_call_many_flat | 46278ns | 46452ns | 43176ns | 46349ns | 47724ns | base |
| closure_call_many_linked | 130381ns | 130398ns | 126432ns | 129543ns | 133613ns | +181.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_call_many_flat | 43976ns | 41078ns | 45317ns | base | 0.023 |
| closure_call_many_linked | 128088ns | 124284ns | 131220ns | +191.27% | 0.008 |

## Performance model

- Peak throughput: **0.025 Gops/s** (closure_call_many_flat; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_call_many_flat | 0.023 | 93.1% |
| closure_call_many_linked | 0.008 | 32.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_call_many_flat | 46278ns | 46278ns | base |
| closure_call_many_linked | 130381ns | 130381ns | +181.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_call_many_flat | 44125ns | base | --- | [42488, 45317] | --- | --- | --- | --- |
| closure_call_many_linked | 128069ns | +84774.8ns (+192.1%) | [+80491, +87070]ns | [124976, 131220] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_call_many_flat | closure_call_many_linked |
|---|---|---|
| 1 | 41078ns | +205.9% |
| 2 | 45576ns | +176.8% |
| 3 | 43898ns | +183.1% |
| 4 | 45058ns | +188.6% |
| 5 | 44310ns | +198.9% |
| 6 | 43940ns | +195.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_call_many_flat | -0.367 | moderate- |
| closure_call_many_linked | 0.422 | moderate+ |

**Consistency summary:**

- **closure_call_many_linked**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_call_many_flat | 4.0ns | 43976.5ns | 0.0% |  |
| closure_call_many_linked | 49.4ns | 128088.2ns | 0.0% |  |

## Distribution (algo ns)

```
closure_call_many_flat (n=6, range 41077.9-45316.9 ns)
  41077.9 |####################
  41289.8 |
  41501.8 |
  41713.7 |
  41925.7 |
  42137.6 |
  42349.6 |
  42561.5 |
  42773.5 |
  42985.4 |
  43197.4 |
  43409.3 |
  43621.3 |
  43833.2 |########################################
  44045.2 |
  44257.1 |####################
  44469.1 |
  44681.0 |
  44893.0 |####################
  45104.9 |
  (0 below, 1 above range)

closure_call_many_linked (n=6, range 124284.2-131219.6 ns)
  124284.2 |####################
  124631.0 |
  124977.7 |
  125324.5 |####################
  125671.3 |
  126018.1 |####################
  126364.8 |
  126711.6 |
  127058.4 |
  127405.1 |
  127751.9 |
  128098.7 |
  128445.4 |
  128792.2 |
  129139.0 |
  129485.8 |
  129832.5 |########################################
  130179.3 |
  130526.1 |
  130872.8 |
  (0 below, 1 above range)

```
