# Value representation: static/raw vs runtime-tagged vs NaN-boxed (dynamic-typing cost)

3 variants, 6 samples per variant.
Baseline: **valrepr_static**

## Highlights

Baseline for all deltas below: **valrepr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### valrepr_static is fastest but the noisiest (CV 9.1%)

valrepr_static wins on median (9.34 us) yet has the highest variance (CV 9.1%), while valrepr_nanbox is the steadiest (CV 5.9%, 9.64 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (573 ns) is smaller than the fastest variant's own run-to-run std-dev (853 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (valrepr_static)

The baseline valrepr_static is the fastest (9.34 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader valrepr_static vs stability leader valrepr_nanbox (+3% speed for 1.6x steadier)

valrepr_static is fastest (9.34 us, CV 9.1%); valrepr_nanbox gives up 3.2% median for 1.6x lower variance (CV 5.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### valrepr_nanbox's edge over baseline is significant but tiny (-37 ns, 0.39%)

valrepr_nanbox differs from baseline valrepr_static by -37 ns (0.39%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (valrepr_static) is the fastest** at 9337.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 9337.9 ns, slowest 9911.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| valrepr_nanbox | 11973ns | 12250ns | 10383ns | 12218ns | 12399ns | +2.05% |
| valrepr_static | 11732ns | 11828ns | 10265ns | 11432ns | 12915ns | base |
| valrepr_tagged | 12385ns | 12426ns | 11223ns | 12031ns | 13497ns | +5.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| valrepr_nanbox | 9443ns | 8191ns | 9805ns | +1.95% | 0.027 |
| valrepr_static | 9262ns | 8109ns | 10196ns | base | 0.028 |
| valrepr_tagged | 9888ns | 8977ns | 10766ns | +6.76% | 0.026 |

## Performance model

- Peak throughput: **0.032 Gops/s** (valrepr_static; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| valrepr_nanbox | 0.027 | 84.1% |
| valrepr_static | 0.027 | 86.8% |
| valrepr_tagged | 0.026 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| valrepr_nanbox | 11973ns | 11973ns | +2.05% |
| valrepr_static | 11732ns | 11732ns | base |
| valrepr_tagged | 12385ns | 12385ns | +5.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| valrepr_static | 9338ns | base | --- | [8252, 10196] | --- | --- | --- | --- |
| valrepr_nanbox | 9638ns | no significant difference | [-487, +1066]ns | [8885, 9805] | no | 0.6875 | 0.6875 | 0 |
| valrepr_tagged | 9911ns | +603.9ns (+6.5%) | [+203, +1071]ns | [8986, 10766] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | valrepr_static | valrepr_nanbox | valrepr_tagged |
|---|---|---|---|
| 1 | 8109ns | +19.5% | +18.1% |
| 2 | 9782ns | -0.1% | +4.7% |
| 3 | 9645ns | -0.6% | +6.3% |
| 4 | 10610ns | -7.3% | +6.3% |
| 5 | 9031ns | +6.1% | -0.6% |
| 6 | 8396ns | -2.4% | +7.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| valrepr_nanbox | 0.035 | ok |
| valrepr_static | 0.001 | ok |
| valrepr_tagged | 0.019 | ok |

**Consistency summary:**

- **valrepr_nanbox**: won 4/6, lost 2/6
- **valrepr_tagged**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| valrepr_nanbox | 86.6ns | 9442.8ns | 0.9% |  |
| valrepr_static | 86.3ns | 9262.1ns | 0.9% |  |
| valrepr_tagged | 117.2ns | 9887.8ns | 1.2% |  |

## Distribution (algo ns)

```
valrepr_nanbox (n=6, range 8191.2-9804.6 ns)
   8191.2 |####################
   8271.9 |
   8352.5 |
   8433.2 |
   8513.9 |
   8594.5 |
   8675.2 |
   8755.9 |
   8836.6 |
   8917.2 |
   8997.9 |
   9078.6 |
   9159.2 |
   9239.9 |
   9320.6 |
   9401.2 |
   9481.9 |
   9562.6 |########################################
   9643.3 |####################
   9723.9 |####################
  (0 below, 1 above range)

valrepr_static (n=6, range 8108.8-10196.0 ns)
   8108.8 |########################################
   8213.2 |
   8317.5 |########################################
   8421.9 |
   8526.2 |
   8630.6 |
   8735.0 |
   8839.3 |
   8943.7 |########################################
   9048.1 |
   9152.4 |
   9256.8 |
   9361.1 |
   9465.5 |
   9569.9 |########################################
   9674.2 |
   9778.6 |########################################
   9883.0 |
   9987.3 |
  10091.7 |
  (0 below, 1 above range)

valrepr_tagged (n=6, range 8977.1-10766.2 ns)
   8977.1 |########################################
   9066.6 |
   9156.0 |
   9245.5 |
   9334.9 |
   9424.4 |
   9513.8 |####################
   9603.3 |
   9692.8 |
   9782.2 |
   9871.7 |
   9961.1 |
  10050.6 |
  10140.0 |
  10229.5 |########################################
  10319.0 |
  10408.4 |
  10497.9 |
  10587.3 |
  10676.8 |
  (0 below, 1 above range)

```
