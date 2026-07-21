# Per-branch strategy: archetype 6 (ifchain4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b6_table**

## Highlights

Baseline for all deltas below: **ab_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b6_tree, ab_b6_table) are a dead heat (<1%)

ab_b6_tree (326.88 us) and ab_b6_table (327.35 us) differ by 0.14%, inside the noise, even though the wider field spreads 86.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {ab_b6_tree, ab_b6_table, ab_b6_seq, ab_b6_prof} vs {ab_b6_pred} (81% apart)

The field splits into a fast tier {ab_b6_tree, ab_b6_table, ab_b6_seq, ab_b6_prof} and a slow tier {ab_b6_pred} with a 81% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: ab_b6_tree** at 326876.7 ns median (-0.1% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.86x (fastest 326876.7 ns, slowest 608767.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b6_pred | 611331ns | 611533ns | 606909ns | 611211ns | 613722ns | +84.87% |
| ab_b6_prof | 337540ns | 338198ns | 333738ns | 337087ns | 340121ns | +2.07% |
| ab_b6_seq | 337537ns | 337761ns | 335458ns | 337250ns | 339007ns | +2.07% |
| ab_b6_table | 330685ns | 329670ns | 326968ns | 329096ns | 334929ns | base |
| ab_b6_tree | 329838ns | 329616ns | 322700ns | 328561ns | 335323ns | -0.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b6_pred | 608526ns | 604101ns | 611005ns | +85.48% | 0.002 |
| ab_b6_prof | 335055ns | 331285ns | 337584ns | +2.12% | 0.003 |
| ab_b6_seq | 335058ns | 333137ns | 336264ns | +2.13% | 0.003 |
| ab_b6_table | 328084ns | 324369ns | 332157ns | base | 0.003 |
| ab_b6_tree | 327236ns | 320069ns | 332900ns | -0.26% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b6_tree; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b6_pred | 0.002 | 52.6% |
| ab_b6_prof | 0.003 | 95.4% |
| ab_b6_seq | 0.003 | 95.5% |
| ab_b6_table | 0.003 | 97.8% |
| ab_b6_tree | 0.003 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b6_pred | 611331ns | 611331ns | +84.87% |
| ab_b6_prof | 337540ns | 337540ns | +2.07% |
| ab_b6_seq | 337537ns | 337537ns | +2.07% |
| ab_b6_table | 330685ns | 330685ns | base |
| ab_b6_tree | 329838ns | 329838ns | -0.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b6_table | 327347ns | base | --- | [324746, 332157] | --- | --- | --- | --- |
| ab_b6_pred | 608767ns | +282355.0ns (+86.3%) | [+274972, +284000]ns | [605806, 611005] | YES | 0.0417 | 0.0313 | 0 |
| ab_b6_prof | 335618ns | +6288.5ns (+1.9%) | [+4412, +10213]ns | [331963, 337584] | YES | 0.0417 | 0.0313 | 0 |
| ab_b6_seq | 335318ns | +7265.6ns (+2.2%) | [+2140, +11518]ns | [333592, 336264] | YES | 0.0417 | 0.0313 | 0 |
| ab_b6_tree | 326877ns | no significant difference | [-7630, +6481]ns | [321931, 332900] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b6_table | ab_b6_pred | ab_b6_prof | ab_b6_seq | ab_b6_tree |
|---|---|---|---|---|---|
| 1 | 324369ns | +87.5% | +2.1% | +3.8% | +1.2% |
| 2 | 326980ns | +86.3% | +1.7% | +2.5% | -2.1% |
| 3 | 332143ns | +83.7% | +1.0% | +0.3% | -2.5% |
| 4 | 327714ns | +86.7% | +2.5% | +1.9% | +2.4% |
| 5 | 332172ns | +81.9% | +1.7% | +1.0% | -2.1% |
| 6 | 325124ns | +86.9% | +3.8% | +3.3% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b6_pred | -0.107 | ok |
| ab_b6_prof | 0.497 | moderate+ |
| ab_b6_seq | 0.212 | moderate+ |
| ab_b6_table | -0.271 | moderate- |
| ab_b6_tree | -0.230 | moderate- |

**Consistency summary:**

- **ab_b6_pred**: won 0/6, lost 6/6
- **ab_b6_prof**: won 0/6, lost 6/6
- **ab_b6_seq**: won 0/6, lost 6/6
- **ab_b6_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b6_pred | 11.0ns | 608526.0ns | 0.0% |  |
| ab_b6_prof | 6.2ns | 335054.8ns | 0.0% |  |
| ab_b6_seq | 8.2ns | 335058.0ns | 0.0% |  |
| ab_b6_table | 6.3ns | 328083.6ns | 0.0% |  |
| ab_b6_tree | 7.5ns | 327235.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b6_pred (n=6, range 604101.2-611004.6 ns)
  604101.2 |########################################
  604446.4 |
  604791.5 |
  605136.7 |
  605481.9 |
  605827.0 |
  606172.2 |
  606517.4 |
  606862.5 |
  607207.7 |########################################
  607552.9 |
  607898.0 |########################################
  608243.2 |
  608588.4 |
  608933.5 |
  609278.7 |########################################
  609623.9 |
  609969.0 |########################################
  610314.2 |
  610659.4 |
  (0 below, 1 above range)

ab_b6_prof (n=6, range 331285.4-337584.0 ns)
  331285.4 |########################################
  331600.3 |
  331915.3 |
  332230.2 |
  332545.1 |########################################
  332860.1 |
  333175.0 |
  333489.9 |
  333804.8 |
  334119.8 |
  334434.7 |
  334749.6 |
  335064.6 |
  335379.5 |########################################
  335694.4 |########################################
  336009.3 |
  336324.3 |
  336639.2 |
  336954.1 |
  337269.1 |########################################
  (0 below, 1 above range)

ab_b6_seq (n=6, range 333136.7-336264.2 ns)
  333136.7 |########################################
  333293.1 |
  333449.4 |
  333605.8 |
  333762.2 |
  333918.6 |########################################
  334074.9 |
  334231.3 |
  334387.7 |
  334544.1 |
  334700.4 |
  334856.8 |
  335013.2 |
  335169.5 |########################################
  335325.9 |########################################
  335482.3 |
  335638.7 |########################################
  335795.0 |
  335951.4 |
  336107.8 |
  (0 below, 1 above range)

ab_b6_table (n=6, range 324369.2-332157.3 ns)
  324369.2 |########################################
  324758.6 |########################################
  325148.0 |
  325537.4 |
  325926.8 |
  326316.2 |
  326705.6 |########################################
  327095.0 |
  327484.4 |########################################
  327873.8 |
  328263.2 |
  328652.7 |
  329042.1 |
  329431.5 |
  329820.9 |
  330210.3 |
  330599.7 |
  330989.1 |
  331378.5 |
  331767.9 |########################################
  (0 below, 1 above range)

ab_b6_tree (n=6, range 320069.2-332899.6 ns)
  320069.2 |########################################
  320710.7 |
  321352.2 |
  321993.8 |
  322635.3 |
  323276.8 |########################################
  323918.3 |
  324559.8 |
  325201.4 |########################################
  325842.9 |
  326484.4 |
  327125.9 |
  327767.4 |
  328409.0 |########################################
  329050.5 |
  329692.0 |
  330333.5 |########################################
  330975.0 |
  331616.6 |
  332258.1 |
  (0 below, 1 above range)

```
