# Name resolution: flat shadow-stack vs hashed-per-scope vs linear scope-chain walk

3 variants, 6 samples per variant.
Baseline: **resolve_flat**

## Highlights

Baseline for all deltas below: **resolve_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### resolve_flat dominates: 458% faster than the next best (resolve_linear)

resolve_flat (1.06 us) leads resolve_linear (5.91 us) by 458%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### resolve_hashed is an outlier: 6.5x slower than the field

resolve_hashed (6.89 us) is 6.5x the fastest (1.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### resolve_flat shows alternating (throttle bounce) (autocorr -0.68)

resolve_flat's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (resolve_flat)

The baseline resolve_flat is the fastest (1.06 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.5x the fastest

Fastest resolve_flat (1.06 us) to slowest resolve_hashed (6.89 us): 6.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (resolve_flat) is the fastest** at 1059.3 ns median
- 2 variants significantly slower than baseline
- Spread: 6.51x (fastest 1059.3 ns, slowest 6892.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| resolve_flat | 3514ns | 3489ns | 3219ns | 3479ns | 3714ns | base |
| resolve_hashed | 9235ns | 9337ns | 8019ns | 9230ns | 9851ns | +162.82% |
| resolve_linear | 8290ns | 8339ns | 7638ns | 8233ns | 8700ns | +135.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| resolve_flat | 1068ns | 982ns | 1128ns | base | 0.959 |
| resolve_hashed | 6786ns | 5886ns | 7232ns | +535.67% | 0.151 |
| resolve_linear | 5838ns | 5390ns | 6111ns | +446.84% | 0.175 |

## Performance model

- Peak throughput: **1.043 Gops/s** (resolve_flat; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| resolve_flat | 0.967 | 92.7% |
| resolve_hashed | 0.149 | 14.2% |
| resolve_linear | 0.173 | 16.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| resolve_flat | 3514ns | 3514ns | base |
| resolve_hashed | 9235ns | 9235ns | +162.82% |
| resolve_linear | 8290ns | 8290ns | +135.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| resolve_flat | 1059ns | base | --- | [1016, 1128] | --- | --- | --- | --- |
| resolve_hashed | 6892ns | +5838.8ns (+551.2%) | [+5181, +6136]ns | [6234, 7232] | YES | 0.0313 | 0.0313 | 0 |
| resolve_linear | 5909ns | +4855.6ns (+458.4%) | [+4441, +5014]ns | [5494, 6111] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | resolve_flat | resolve_hashed | resolve_linear |
|---|---|---|---|
| 1 | 1055ns | +457.7% | +410.7% |
| 2 | 1050ns | +527.0% | +433.2% |
| 3 | 1063ns | +581.7% | +468.7% |
| 4 | 1125ns | +509.3% | +425.7% |
| 5 | 982ns | +605.7% | +501.3% |
| 6 | 1130ns | +538.6% | +446.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| resolve_flat | -0.680 | HIGH- (thermal bounce) |
| resolve_hashed | 0.152 | ok |
| resolve_linear | 0.236 | moderate+ |

**Consistency summary:**

- **resolve_hashed**: won 0/6, lost 6/6
- **resolve_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| resolve_flat | 3.7ns | 1067.6ns | 0.3% |  |
| resolve_hashed | 3.4ns | 6786.2ns | 0.0% |  |
| resolve_linear | 3.8ns | 5837.9ns | 0.1% |  |

## Distribution (algo ns)

```
resolve_flat (n=6, range 981.7-1127.7 ns)
    981.7 |########################################
    989.0 |
    996.3 |
   1003.6 |
   1010.9 |
   1018.2 |
   1025.5 |
   1032.8 |
   1040.1 |
   1047.4 |########################################
   1054.7 |########################################
   1062.0 |########################################
   1069.3 |
   1076.6 |
   1083.9 |
   1091.2 |
   1098.5 |
   1105.8 |
   1113.1 |
   1120.4 |########################################
  (0 below, 1 above range)

resolve_hashed (n=6, range 5886.2-7232.5 ns)
   5886.2 |########################################
   5953.5 |
   6020.8 |
   6088.1 |
   6155.5 |
   6222.8 |
   6290.1 |
   6357.4 |
   6424.7 |
   6492.0 |
   6559.4 |########################################
   6626.7 |
   6694.0 |
   6761.3 |
   6828.6 |########################################
   6895.9 |########################################
   6963.2 |
   7030.6 |
   7097.9 |
   7165.2 |########################################
  (0 below, 1 above range)

resolve_linear (n=6, range 5390.4-6111.0 ns)
   5390.4 |####################
   5426.4 |
   5462.5 |
   5498.5 |
   5534.5 |
   5570.6 |####################
   5606.6 |
   5642.6 |
   5678.7 |
   5714.7 |
   5750.7 |
   5786.8 |
   5822.8 |
   5858.8 |
   5894.9 |########################################
   5930.9 |
   5966.9 |
   6003.0 |
   6039.0 |####################
   6075.0 |
  (0 below, 1 above range)

```
