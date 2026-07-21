# Per-type strategy: all ifchain branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_ifchain_table**

## Highlights

Baseline for all deltas below: **ab_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_ifchain_pred is an outlier: 2.3x slower than the field

ab_ifchain_pred (52.11 us) is 2.3x the fastest (22.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ab_ifchain_prof is fastest but the noisiest (CV 6.5%)

ab_ifchain_prof wins on median (22.27 us) yet has the highest variance (CV 6.5%), while ab_ifchain_tree is the steadiest (CV 3.7%, 22.49 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ab_ifchain_table shows alternating (throttle bounce) (autocorr -0.52)

ab_ifchain_table's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {ab_ifchain_prof, ab_ifchain_tree, ab_ifchain_table, ab_ifchain_seq} vs {ab_ifchain_pred} (122% apart)

The field splits into a fast tier {ab_ifchain_prof, ab_ifchain_tree, ab_ifchain_table, ab_ifchain_seq} and a slow tier {ab_ifchain_pred} with a 122% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader ab_ifchain_prof vs stability leader ab_ifchain_tree (+1% speed for 1.8x steadier)

ab_ifchain_prof is fastest (22.27 us, CV 6.5%); ab_ifchain_tree gives up 1.0% median for 1.8x lower variance (CV 3.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: ab_ifchain_prof** at 22265.8 ns median (-1.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.34x (fastest 22265.8 ns, slowest 52110.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_ifchain_pred | 52991ns | 54603ns | 47519ns | 52753ns | 56084ns | +111.58% |
| ab_ifchain_prof | 24591ns | 24563ns | 22568ns | 24096ns | 26344ns | -1.81% |
| ab_ifchain_seq | 25726ns | 25945ns | 22881ns | 25784ns | 27062ns | +2.72% |
| ab_ifchain_table | 25045ns | 25155ns | 22389ns | 25088ns | 26308ns | base |
| ab_ifchain_tree | 24942ns | 24909ns | 23350ns | 24863ns | 25857ns | -0.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_ifchain_pred | 50562ns | 45368ns | 53489ns | +123.54% | 0.001 |
| ab_ifchain_prof | 22280ns | 20406ns | 23864ns | -1.50% | 0.003 |
| ab_ifchain_seq | 23282ns | 20675ns | 24490ns | +2.93% | 0.003 |
| ab_ifchain_table | 22619ns | 20237ns | 23779ns | base | 0.003 |
| ab_ifchain_tree | 22531ns | 21129ns | 23355ns | -0.39% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_ifchain_table; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_ifchain_pred | 0.001 | 38.8% |
| ab_ifchain_prof | 0.003 | 90.9% |
| ab_ifchain_seq | 0.003 | 86.2% |
| ab_ifchain_table | 0.003 | 89.2% |
| ab_ifchain_tree | 0.003 | 90.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_ifchain_pred | 52991ns | 52991ns | +111.58% |
| ab_ifchain_prof | 24591ns | 24591ns | -1.81% |
| ab_ifchain_seq | 25726ns | 25726ns | +2.72% |
| ab_ifchain_table | 25045ns | 25045ns | base |
| ab_ifchain_tree | 24942ns | 24942ns | -0.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_ifchain_table | 22695ns | base | --- | [21384, 23779] | --- | --- | --- | --- |
| ab_ifchain_pred | 52111ns | +29003.3ns (+127.8%) | [+24577, +30249]ns | [46088, 53489] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_ifchain_prof | 22266ns | no significant difference | [-1789, +1314]ns | [20709, 23864] | no | 1.0000 | 1.0000 | 0 |
| ab_ifchain_seq | 23490ns | no significant difference | [-371, +1921]ns | [21866, 24490] | no | 0.9167 | 0.6875 | 0 |
| ab_ifchain_tree | 22490ns | no significant difference | [-1288, +1156]ns | [21747, 23355] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_ifchain_table | ab_ifchain_pred | ab_ifchain_prof | ab_ifchain_seq | ab_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 22530ns | +137.7% | -9.4% | +9.1% | +6.3% |
| 2 | 23115ns | +127.5% | +0.4% | -0.2% | -2.8% |
| 3 | 22607ns | +128.4% | +8.2% | +7.9% | -1.1% |
| 4 | 22784ns | +105.4% | -6.4% | +1.9% | -0.1% |
| 5 | 20237ns | +124.2% | +3.8% | +2.2% | +4.4% |
| 6 | 24442ns | +118.5% | -4.8% | -2.8% | -7.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_ifchain_pred | 0.140 | ok |
| ab_ifchain_prof | -0.146 | ok |
| ab_ifchain_seq | -0.169 | ok |
| ab_ifchain_table | -0.516 | HIGH- (thermal bounce) |
| ab_ifchain_tree | -0.100 | ok |

**Consistency summary:**

- **ab_ifchain_pred**: won 0/6, lost 6/6
- **ab_ifchain_prof**: won 3/6, lost 3/6
- **ab_ifchain_seq**: won 2/6, lost 4/6
- **ab_ifchain_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_ifchain_pred | 3.2ns | 50562.3ns | 0.0% |  |
| ab_ifchain_prof | 3.1ns | 22279.9ns | 0.0% |  |
| ab_ifchain_seq | 3.8ns | 23281.9ns | 0.0% |  |
| ab_ifchain_table | 3.9ns | 22619.2ns | 0.0% |  |
| ab_ifchain_tree | 3.8ns | 22530.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_ifchain_pred (n=6, range 45367.5-53488.8 ns)
  45367.5 |########################################
  45773.6 |
  46179.6 |
  46585.7 |########################################
  46991.8 |
  47397.8 |
  47803.9 |
  48209.9 |
  48616.0 |
  49022.1 |
  49428.1 |
  49834.2 |
  50240.2 |
  50646.3 |
  51052.4 |
  51458.4 |########################################
  51864.5 |
  52270.6 |########################################
  52676.6 |
  53082.7 |########################################
  (0 below, 1 above range)

ab_ifchain_prof (n=6, range 20406.2-23864.4 ns)
  20406.2 |####################
  20579.1 |
  20752.0 |
  20924.9 |####################
  21097.8 |
  21270.8 |####################
  21443.7 |
  21616.6 |
  21789.5 |
  21962.4 |
  22135.3 |
  22308.2 |
  22481.1 |
  22654.0 |
  22826.9 |
  22999.9 |
  23172.8 |########################################
  23345.7 |
  23518.6 |
  23691.5 |
  (0 below, 1 above range)

ab_ifchain_seq (n=6, range 20674.6-24489.6 ns)
  20674.6 |########################################
  20865.3 |
  21056.1 |
  21246.8 |
  21437.6 |
  21628.3 |
  21819.1 |
  22009.8 |
  22200.6 |
  22391.3 |
  22582.1 |
  22772.8 |
  22963.6 |########################################
  23154.3 |########################################
  23345.1 |
  23535.8 |
  23726.6 |########################################
  23917.3 |
  24108.1 |
  24298.8 |########################################
  (0 below, 1 above range)

ab_ifchain_table (n=6, range 20237.1-23778.5 ns)
  20237.1 |########################################
  20414.2 |
  20591.2 |
  20768.3 |
  20945.4 |
  21122.5 |
  21299.5 |
  21476.6 |
  21653.7 |
  21830.8 |
  22007.8 |
  22184.9 |
  22362.0 |########################################
  22539.0 |########################################
  22716.1 |########################################
  22893.2 |
  23070.3 |########################################
  23247.3 |
  23424.4 |
  23601.5 |
  (0 below, 1 above range)

ab_ifchain_tree (n=6, range 21128.7-23354.8 ns)
  21128.7 |####################
  21240.0 |
  21351.3 |
  21462.6 |
  21573.9 |
  21685.2 |
  21796.5 |
  21907.8 |
  22019.1 |
  22130.4 |
  22241.8 |
  22353.1 |####################
  22464.4 |########################################
  22575.7 |
  22687.0 |####################
  22798.3 |
  22909.6 |
  23020.9 |
  23132.2 |
  23243.5 |
  (0 below, 1 above range)

```
