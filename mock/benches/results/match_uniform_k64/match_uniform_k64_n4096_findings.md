# Match lowering: if-chain vs jump-table vs decision-tree, K=64 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u64**

## Highlights

Baseline for all deltas below: **ml_jumptable_u64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u64 is an outlier: 7.5x slower than the field

ml_tree_u64 (1.93 ms) is 7.5x the fastest (256.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ml_jumptable_u64, ml_ifchain_u64) are a dead heat (<1%)

ml_jumptable_u64 (256.31 us) and ml_ifchain_u64 (256.72 us) differ by 0.16%, inside the noise, even though the wider field spreads 652.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ml_tree_u64 shows alternating (throttle bounce) (autocorr -0.56)

ml_tree_u64's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ml_jumptable_u64)

The baseline ml_jumptable_u64 is the fastest (256.31 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.5x the fastest

Fastest ml_jumptable_u64 (256.31 us) to slowest ml_tree_u64 (1.93 ms): 7.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ml_jumptable_u64) is the fastest** at 256305.6 ns median
- 1 variant significantly slower than baseline
- Spread: 7.53x (fastest 256305.6 ns, slowest 1929039.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u64 | 258885ns | 259165ns | 254245ns | 258175ns | 262271ns | -0.36% |
| ml_jumptable_u64 | 259823ns | 258943ns | 256488ns | 258669ns | 263222ns | base |
| ml_tree_u64 | 1940354ns | 1932594ns | 1930990ns | 1932133ns | 1957369ns | +646.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u64 | 256353ns | 251639ns | 259623ns | -0.35% | 0.016 |
| ml_jumptable_u64 | 257259ns | 254048ns | 260693ns | base | 0.016 |
| ml_tree_u64 | 1937162ns | 1927468ns | 1954819ns | +653.00% | 0.002 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_u64; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u64 | 0.016 | 98.0% |
| ml_jumptable_u64 | 0.016 | 98.2% |
| ml_tree_u64 | 0.002 | 13.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u64 | 258885ns | 258885ns | -0.36% |
| ml_jumptable_u64 | 259823ns | 259823ns | base |
| ml_tree_u64 | 1940354ns | 1940354ns | +646.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u64 | 256306ns | base | --- | [254779, 260693] | --- | --- | --- | --- |
| ml_ifchain_u64 | 256718ns | no significant difference | [-6785, +3318]ns | [252719, 259623] | no | 1.0000 | 1.0000 | 0 |
| ml_tree_u64 | 1929040ns | +1672823.8ns (+652.7%) | [+1669153, +1697731]ns | [1927627, 1954819] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u64 | ml_ifchain_u64 | ml_tree_u64 |
|---|---|---|---|
| 1 | 254048ns | +1.3% | +658.8% |
| 2 | 258667ns | -1.0% | +654.3% |
| 3 | 255558ns | +1.3% | +654.2% |
| 4 | 262720ns | -4.2% | +634.7% |
| 5 | 255509ns | -0.7% | +666.5% |
| 6 | 257053ns | +1.3% | +650.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u64 | -0.204 | moderate- |
| ml_jumptable_u64 | -0.528 | HIGH- (thermal bounce) |
| ml_tree_u64 | -0.565 | HIGH- (thermal bounce) |

**Consistency summary:**

- **ml_ifchain_u64**: won 3/6, lost 3/6
- **ml_tree_u64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u64 | 6.9ns | 256353.4ns | 0.0% |  |
| ml_jumptable_u64 | 4.6ns | 257259.2ns | 0.0% |  |
| ml_tree_u64 | 28.3ns | 1937161.7ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u64 (n=6, range 251639.2-259623.4 ns)
  251639.2 |########################################
  252038.4 |
  252437.6 |
  252836.8 |
  253236.0 |
  253635.2 |########################################
  254034.4 |
  254433.7 |
  254832.9 |
  255232.1 |
  255631.3 |
  256030.5 |########################################
  256429.7 |
  256828.9 |
  257228.1 |########################################
  257627.3 |
  258026.5 |
  258425.7 |
  258824.9 |########################################
  259224.1 |
  (0 below, 1 above range)

ml_jumptable_u64 (n=6, range 254048.3-260693.4 ns)
  254048.3 |####################
  254380.6 |
  254712.8 |
  255045.1 |
  255377.3 |########################################
  255709.6 |
  256041.8 |
  256374.1 |
  256706.3 |
  257038.6 |####################
  257370.8 |
  257703.1 |
  258035.3 |
  258367.6 |####################
  258699.8 |
  259032.1 |
  259364.3 |
  259696.6 |
  260028.8 |
  260361.1 |
  (0 below, 1 above range)

ml_tree_u64 (n=6, range 1927468.3-1954818.5 ns)
  1927468.3 |########################################
  1928835.8 |#############
  1930203.3 |
  1931570.8 |
  1932938.4 |
  1934305.9 |
  1935673.4 |
  1937040.9 |
  1938408.4 |
  1939775.9 |
  1941143.4 |
  1942510.9 |
  1943878.4 |
  1945246.0 |
  1946613.5 |
  1947981.0 |
  1949348.5 |
  1950716.0 |#############
  1952083.5 |
  1953451.0 |
  (0 below, 1 above range)

```
