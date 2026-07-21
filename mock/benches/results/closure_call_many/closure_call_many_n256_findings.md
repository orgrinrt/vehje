# Closure representation: create-once-call-many, flat vs linked (access cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_call_many_flat**

## Highlights

Baseline for all deltas below: **closure_call_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### closure_call_many_flat dominates: 201% faster than the next best (closure_call_many_linked)

closure_call_many_flat (10.67 us) leads closure_call_many_linked (32.13 us) by 201%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (closure_call_many_flat)

The baseline closure_call_many_flat is the fastest (10.67 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.0x the fastest

Fastest closure_call_many_flat (10.67 us) to slowest closure_call_many_linked (32.13 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (closure_call_many_flat) is the fastest** at 10666.0 ns median
- 1 variant significantly slower than baseline
- Spread: 3.01x (fastest 10666.0 ns, slowest 32125.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_call_many_flat | 12848ns | 12858ns | 12622ns | 12837ns | 12979ns | base |
| closure_call_many_linked | 34439ns | 34320ns | 34058ns | 34246ns | 34919ns | +168.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_call_many_flat | 10668ns | 10478ns | 10783ns | base | 0.024 |
| closure_call_many_linked | 32237ns | 31875ns | 32687ns | +202.19% | 0.008 |

## Performance model

- Peak throughput: **0.024 Gops/s** (closure_call_many_flat; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_call_many_flat | 0.024 | 98.2% |
| closure_call_many_linked | 0.008 | 32.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_call_many_flat | 12848ns | 12848ns | base |
| closure_call_many_linked | 34439ns | 34439ns | +168.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_call_many_flat | 10666ns | base | --- | [10553, 10783] | --- | --- | --- | --- |
| closure_call_many_linked | 32125ns | +21428.0ns (+200.9%) | [+21298, +21981]ns | [31898, 32687] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_call_many_flat | closure_call_many_linked |
|---|---|---|
| 1 | 10703ns | +199.1% |
| 2 | 10783ns | +199.0% |
| 3 | 10629ns | +200.3% |
| 4 | 10783ns | +203.1% |
| 5 | 10628ns | +207.6% |
| 6 | 10478ns | +204.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_call_many_flat | -0.029 | ok |
| closure_call_many_linked | -0.154 | ok |

**Consistency summary:**

- **closure_call_many_linked**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_call_many_flat | 3.8ns | 10667.6ns | 0.0% |  |
| closure_call_many_linked | 49.9ns | 32236.5ns | 0.2% |  |

## Distribution (algo ns)

```
closure_call_many_flat (n=6, range 10478.3-10783.3 ns)
  10478.3 |####################
  10493.5 |
  10508.8 |
  10524.0 |
  10539.3 |
  10554.5 |
  10569.8 |
  10585.0 |
  10600.3 |
  10615.5 |########################################
  10630.8 |
  10646.0 |
  10661.3 |
  10676.5 |
  10691.8 |####################
  10707.0 |
  10722.3 |
  10737.5 |
  10752.8 |
  10768.0 |
  (0 below, 2 above range)

closure_call_many_linked (n=6, range 31875.0-32686.7 ns)
  31875.0 |########################################
  31915.6 |########################################
  31956.2 |
  31996.7 |########################################
  32037.3 |
  32077.9 |
  32118.5 |
  32159.1 |
  32199.7 |
  32240.2 |########################################
  32280.8 |
  32321.4 |
  32362.0 |
  32402.6 |
  32443.2 |
  32483.7 |
  32524.3 |
  32564.9 |
  32605.5 |
  32646.1 |########################################
  (0 below, 1 above range)

```
