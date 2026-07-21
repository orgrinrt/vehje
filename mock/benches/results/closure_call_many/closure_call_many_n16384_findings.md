# Closure representation: create-once-call-many, flat vs linked (access cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_call_many_flat**

## Highlights

Baseline for all deltas below: **closure_call_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### closure_call_many_flat dominates: 203% faster than the next best (closure_call_many_linked)

closure_call_many_flat (671.13 us) leads closure_call_many_linked (2.03 ms) by 203%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (closure_call_many_flat)

The baseline closure_call_many_flat is the fastest (671.13 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.0x the fastest

Fastest closure_call_many_flat (671.13 us) to slowest closure_call_many_linked (2.03 ms): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (closure_call_many_flat) is the fastest** at 671133.3 ns median
- 1 variant significantly slower than baseline
- Spread: 3.03x (fastest 671133.3 ns, slowest 2032425.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_call_many_flat | 676045ns | 673976ns | 667374ns | 672332ns | 685950ns | base |
| closure_call_many_linked | 2032225ns | 2035359ns | 2000403ns | 2024417ns | 2059848ns | +200.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_call_many_flat | 673235ns | 664460ns | 683384ns | base | 0.024 |
| closure_call_many_linked | 2029122ns | 1996635ns | 2056787ns | +201.40% | 0.008 |

## Performance model

- Peak throughput: **0.025 Gops/s** (closure_call_many_flat; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_call_many_flat | 0.024 | 99.0% |
| closure_call_many_linked | 0.008 | 32.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_call_many_flat | 676045ns | 676045ns | base |
| closure_call_many_linked | 2032225ns | 2032225ns | +200.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_call_many_flat | 671133ns | base | --- | [665187, 683384] | --- | --- | --- | --- |
| closure_call_many_linked | 2032425ns | +1367237.9ns (+203.7%) | [+1316792, +1383631]ns | [1998153, 2056787] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_call_many_flat | closure_call_many_linked |
|---|---|---|
| 1 | 686752ns | +190.7% |
| 2 | 664460ns | +207.6% |
| 3 | 666295ns | +207.4% |
| 4 | 675972ns | +195.8% |
| 5 | 665915ns | +203.5% |
| 6 | 680016ns | +203.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_call_many_flat | -0.353 | moderate- |
| closure_call_many_linked | -0.210 | moderate- |

**Consistency summary:**

- **closure_call_many_linked**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_call_many_flat | 11.4ns | 673234.8ns | 0.0% |  |
| closure_call_many_linked | 283.3ns | 2029121.7ns | 0.0% |  |

## Distribution (algo ns)

```
closure_call_many_flat (n=6, range 664459.6-683383.9 ns)
  664459.6 |####################
  665405.8 |########################################
  666352.0 |
  667298.3 |
  668244.5 |
  669190.7 |
  670136.9 |
  671083.1 |
  672029.3 |
  672975.6 |
  673921.8 |
  674868.0 |
  675814.2 |####################
  676760.4 |
  677706.6 |
  678652.9 |
  679599.1 |####################
  680545.3 |
  681491.5 |
  682437.7 |
  (0 below, 1 above range)

closure_call_many_linked (n=6, range 1996635.0-2056786.6 ns)
  1996635.0 |########################################
  1999642.6 |########################################
  2002650.2 |
  2005657.7 |
  2008665.3 |
  2011672.9 |
  2014680.5 |
  2017688.1 |
  2020695.7 |########################################
  2023703.2 |
  2026710.8 |
  2029718.4 |
  2032726.0 |
  2035733.6 |
  2038741.2 |
  2041748.7 |########################################
  2044756.3 |
  2047763.9 |########################################
  2050771.5 |
  2053779.1 |
  (0 below, 1 above range)

```
