# Closure representation: create-once-call-many, flat vs linked (access cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_call_many_flat**

## Highlights

Baseline for all deltas below: **closure_call_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### closure_call_many_flat dominates: 193% faster than the next best (closure_call_many_linked)

closure_call_many_flat (174.06 us) leads closure_call_many_linked (510.21 us) by 193%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (closure_call_many_flat)

The baseline closure_call_many_flat is the fastest (174.06 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (closure_call_many_flat) is the fastest** at 174056.2 ns median
- 1 variant significantly slower than baseline
- Spread: 2.93x (fastest 174056.2 ns, slowest 510214.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_call_many_flat | 177536ns | 176489ns | 172179ns | 175941ns | 182608ns | base |
| closure_call_many_linked | 513978ns | 512964ns | 510638ns | 512421ns | 517984ns | +189.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_call_many_flat | 175151ns | 169935ns | 180188ns | base | 0.023 |
| closure_call_many_linked | 511384ns | 507748ns | 515537ns | +191.97% | 0.008 |

## Performance model

- Peak throughput: **0.024 Gops/s** (closure_call_many_flat; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_call_many_flat | 0.024 | 97.6% |
| closure_call_many_linked | 0.008 | 33.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_call_many_flat | 177536ns | 177536ns | base |
| closure_call_many_linked | 513978ns | 513978ns | +189.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_call_many_flat | 174056ns | base | --- | [171208, 180188] | --- | --- | --- | --- |
| closure_call_many_linked | 510214ns | +336157.9ns (+193.1%) | [+328213, +344329]ns | [508401, 515537] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_call_many_flat | closure_call_many_linked |
|---|---|---|
| 1 | 172481ns | +199.2% |
| 2 | 179258ns | +183.2% |
| 3 | 181118ns | +181.1% |
| 4 | 173150ns | +194.1% |
| 5 | 169935ns | +203.1% |
| 6 | 174962ns | +192.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_call_many_flat | 0.143 | ok |
| closure_call_many_linked | -0.205 | moderate- |

**Consistency summary:**

- **closure_call_many_linked**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_call_many_flat | 6.2ns | 175150.8ns | 0.0% |  |
| closure_call_many_linked | 82.6ns | 511383.9ns | 0.0% |  |

## Distribution (algo ns)

```
closure_call_many_flat (n=6, range 169935.4-180188.1 ns)
  169935.4 |########################################
  170448.0 |
  170960.7 |
  171473.3 |
  171985.9 |########################################
  172498.6 |
  173011.2 |########################################
  173523.8 |
  174036.5 |
  174549.1 |########################################
  175061.8 |
  175574.4 |
  176087.0 |
  176599.7 |
  177112.3 |
  177624.9 |
  178137.6 |
  178650.2 |
  179162.8 |########################################
  179675.5 |
  (0 below, 1 above range)

closure_call_many_linked (n=6, range 507747.5-515536.7 ns)
  507747.5 |####################
  508137.0 |
  508526.4 |
  508915.9 |########################################
  509305.3 |
  509694.8 |
  510084.2 |
  510473.7 |
  510863.2 |####################
  511252.6 |
  511642.1 |
  512031.5 |
  512421.0 |
  512810.4 |
  513199.9 |
  513589.4 |
  513978.8 |
  514368.3 |
  514757.7 |####################
  515147.2 |
  (0 below, 1 above range)

```
