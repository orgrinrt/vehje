# Per-type strategy: all ifchain branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_ifchain_table**

## Highlights

Baseline for all deltas below: **ab_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_ifchain_pred is an outlier: 2.4x slower than the field

ab_ifchain_pred (192.54 us) is 2.4x the fastest (81.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_ifchain_tree, ab_ifchain_table) are a dead heat (<1%)

ab_ifchain_tree (81.27 us) and ab_ifchain_table (81.97 us) differ by 0.86%, inside the noise, even though the wider field spreads 136.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {ab_ifchain_tree, ab_ifchain_table, ab_ifchain_seq, ab_ifchain_prof} vs {ab_ifchain_pred} (126% apart)

The field splits into a fast tier {ab_ifchain_tree, ab_ifchain_table, ab_ifchain_seq, ab_ifchain_prof} and a slow tier {ab_ifchain_pred} with a 126% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: ab_ifchain_tree** at 81271.7 ns median (-0.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.37x (fastest 81271.7 ns, slowest 192536.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_ifchain_pred | 192059ns | 195010ns | 182386ns | 191450ns | 197809ns | +124.02% |
| ab_ifchain_prof | 88596ns | 87530ns | 84440ns | 87127ns | 92878ns | +3.34% |
| ab_ifchain_seq | 87624ns | 85724ns | 83381ns | 85679ns | 92663ns | +2.20% |
| ab_ifchain_table | 85735ns | 84242ns | 81637ns | 83901ns | 90534ns | base |
| ab_ifchain_tree | 84772ns | 83545ns | 81549ns | 82916ns | 89166ns | -1.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_ifchain_pred | 189611ns | 179994ns | 195265ns | +127.37% | 0.001 |
| ab_ifchain_prof | 86315ns | 82125ns | 90482ns | +3.50% | 0.003 |
| ab_ifchain_seq | 85320ns | 81228ns | 90157ns | +2.31% | 0.003 |
| ab_ifchain_table | 83395ns | 79378ns | 88041ns | base | 0.003 |
| ab_ifchain_tree | 82467ns | 79385ns | 86672ns | -1.11% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_ifchain_table; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_ifchain_pred | 0.001 | 41.2% |
| ab_ifchain_prof | 0.003 | 93.0% |
| ab_ifchain_seq | 0.003 | 95.1% |
| ab_ifchain_table | 0.003 | 96.8% |
| ab_ifchain_tree | 0.003 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_ifchain_pred | 192059ns | 192059ns | +124.02% |
| ab_ifchain_prof | 88596ns | 88596ns | +3.34% |
| ab_ifchain_seq | 87624ns | 87624ns | +2.20% |
| ab_ifchain_table | 85735ns | 85735ns | base |
| ab_ifchain_tree | 84772ns | 84772ns | -1.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_ifchain_table | 81971ns | base | --- | [80173, 88041] | --- | --- | --- | --- |
| ab_ifchain_pred | 192536ns | +105278.3ns (+128.4%) | [+100859, +112511]ns | [181032, 195265] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_ifchain_prof | 85344ns | no significant difference | [-1242, +7137]ns | [83118, 90482] | no | 0.2917 | 0.2188 | 0 |
| ab_ifchain_seq | 83505ns | no significant difference | [-31, +3622]ns | [82299, 90157] | no | 0.2917 | 0.2188 | 0 |
| ab_ifchain_tree | 81272ns | no significant difference | [-1958, +265]ns | [79457, 86672] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_ifchain_table | ab_ifchain_pred | ab_ifchain_prof | ab_ifchain_seq | ab_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 79378ns | +126.8% | +3.5% | +2.3% | +0.0% |
| 2 | 84140ns | +128.5% | +2.1% | -0.9% | -2.6% |
| 3 | 80968ns | +124.9% | +4.8% | +3.1% | +0.6% |
| 4 | 91942ns | +111.4% | -4.6% | +4.6% | -0.6% |
| 5 | 82812ns | +136.8% | +12.6% | +0.9% | -2.1% |
| 6 | 81129ns | +137.7% | +3.7% | +3.7% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_ifchain_pred | -0.125 | ok |
| ab_ifchain_prof | -0.068 | ok |
| ab_ifchain_seq | -0.175 | ok |
| ab_ifchain_table | -0.289 | moderate- |
| ab_ifchain_tree | -0.151 | ok |

**Consistency summary:**

- **ab_ifchain_pred**: won 0/6, lost 6/6
- **ab_ifchain_prof**: won 1/6, lost 5/6
- **ab_ifchain_seq**: won 1/6, lost 5/6
- **ab_ifchain_tree**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_ifchain_pred | 5.1ns | 189611.0ns | 0.0% |  |
| ab_ifchain_prof | 5.2ns | 86314.6ns | 0.0% |  |
| ab_ifchain_seq | 4.6ns | 85320.4ns | 0.0% |  |
| ab_ifchain_table | 4.9ns | 83394.9ns | 0.0% |  |
| ab_ifchain_tree | 4.6ns | 82466.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_ifchain_pred (n=6, range 179993.8-195264.5 ns)
  179993.8 |####################
  180757.3 |
  181520.9 |####################
  182284.4 |
  183047.9 |
  183811.5 |
  184575.0 |
  185338.6 |
  186102.1 |
  186865.6 |
  187629.2 |
  188392.7 |
  189156.2 |
  189919.8 |
  190683.3 |
  191446.9 |
  192210.4 |########################################
  192973.9 |
  193737.5 |####################
  194501.0 |
  (0 below, 1 above range)

ab_ifchain_prof (n=6, range 82125.4-90481.6 ns)
  82125.4 |########################################
  82543.2 |
  82961.0 |
  83378.8 |
  83796.6 |########################################
  84214.5 |
  84632.3 |########################################
  85050.1 |
  85467.9 |########################################
  85885.7 |
  86303.5 |
  86721.3 |
  87139.1 |
  87557.0 |########################################
  87974.8 |
  88392.6 |
  88810.4 |
  89228.2 |
  89646.0 |
  90063.8 |
  (0 below, 1 above range)

ab_ifchain_seq (n=6, range 81227.9-90157.1 ns)
  81227.9 |####################
  81674.4 |
  82120.8 |
  82567.3 |
  83013.7 |####################
  83460.2 |########################################
  83906.6 |####################
  84353.1 |
  84799.6 |
  85246.0 |
  85692.5 |
  86138.9 |
  86585.4 |
  87031.8 |
  87478.3 |
  87924.8 |
  88371.2 |
  88817.7 |
  89264.1 |
  89710.6 |
  (0 below, 1 above range)

ab_ifchain_table (n=6, range 79377.9-88040.9 ns)
  79377.9 |########################################
  79811.0 |
  80244.2 |
  80677.3 |########################################
  81110.5 |########################################
  81543.6 |
  81976.8 |
  82409.9 |########################################
  82843.1 |
  83276.2 |
  83709.4 |########################################
  84142.5 |
  84575.7 |
  85008.8 |
  85442.0 |
  85875.1 |
  86308.3 |
  86741.4 |
  87174.6 |
  87607.7 |
  (0 below, 1 above range)

ab_ifchain_tree (n=6, range 79385.0-86671.9 ns)
  79385.0 |########################################
  79749.3 |
  80113.7 |
  80478.0 |
  80842.4 |####################
  81206.7 |####################
  81571.1 |
  81935.4 |####################
  82299.8 |
  82664.1 |
  83028.4 |
  83392.8 |
  83757.1 |
  84121.5 |
  84485.8 |
  84850.2 |
  85214.5 |
  85578.9 |
  85943.2 |
  86307.6 |
  (0 below, 1 above range)

```
