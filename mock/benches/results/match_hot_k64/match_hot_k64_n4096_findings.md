# Match lowering: hot-first if-chain vs the rest, K=64 arms, 90% hit one arm

4 variants, 6 samples per variant.
Baseline: **ml_jumptable_h64**

## Highlights

Baseline for all deltas below: **ml_jumptable_h64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_h64 is an outlier: 7.6x slower than the field

ml_tree_h64 (1.94 ms) is 7.6x the fastest (255.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ml_ifchain_h64, ml_hotfirst_h64) are a dead heat (<1%)

ml_ifchain_h64 (255.14 us) and ml_hotfirst_h64 (255.18 us) differ by 0.01%, inside the noise, even though the wider field spreads 658.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {ml_ifchain_h64, ml_hotfirst_h64, ml_jumptable_h64} vs {ml_tree_h64} (645% apart)

The field splits into a fast tier {ml_ifchain_h64, ml_hotfirst_h64, ml_jumptable_h64} and a slow tier {ml_tree_h64} with a 645% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.6x the fastest

Fastest ml_ifchain_h64 (255.14 us) to slowest ml_tree_h64 (1.94 ms): 7.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ml_ifchain_h64** at 255141.0 ns median (-1.8% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 7.59x (fastest 255141.0 ns, slowest 1936064.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_hotfirst_h64 | 259408ns | 257592ns | 254157ns | 256824ns | 265909ns | -0.95% |
| ml_ifchain_h64 | 258537ns | 257551ns | 254499ns | 257035ns | 262809ns | -1.28% |
| ml_jumptable_h64 | 261892ns | 262422ns | 254281ns | 260262ns | 268143ns | base |
| ml_tree_h64 | 1939340ns | 1939631ns | 1914514ns | 1934853ns | 1958483ns | +640.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_hotfirst_h64 | 256887ns | 251586ns | 263381ns | -0.96% | 0.016 |
| ml_ifchain_h64 | 255960ns | 252043ns | 260065ns | -1.32% | 0.016 |
| ml_jumptable_h64 | 259375ns | 251721ns | 265690ns | base | 0.016 |
| ml_tree_h64 | 1935872ns | 1910730ns | 1955361ns | +646.36% | 0.002 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_hotfirst_h64; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_hotfirst_h64 | 0.016 | 98.6% |
| ml_ifchain_h64 | 0.016 | 98.6% |
| ml_jumptable_h64 | 0.016 | 96.8% |
| ml_tree_h64 | 0.002 | 13.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_hotfirst_h64 | 259408ns | 259408ns | -0.95% |
| ml_ifchain_h64 | 258537ns | 258537ns | -1.28% |
| ml_jumptable_h64 | 261892ns | 261892ns | base |
| ml_tree_h64 | 1939340ns | 1939340ns | +640.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_h64 | 259889ns | base | --- | [252547, 265690] | --- | --- | --- | --- |
| ml_hotfirst_h64 | 255176ns | no significant difference | [-10707, +6157]ns | [252103, 263381] | no | 0.6875 | 0.6875 | 0 |
| ml_ifchain_h64 | 255141ns | no significant difference | [-10126, +2594]ns | [252673, 260065] | no | 0.6875 | 0.6875 | 0 |
| ml_tree_h64 | 1936064ns | +1676527.8ns (+645.1%) | [+1653394, +1699570]ns | [1916192, 1955361] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_h64 | ml_hotfirst_h64 | ml_ifchain_h64 | ml_tree_h64 |
|---|---|---|---|---|
| 1 | 265701ns | -5.3% | -1.4% | +630.2% |
| 2 | 251721ns | +1.2% | +1.1% | +680.7% |
| 3 | 259860ns | -1.7% | -0.6% | +648.6% |
| 4 | 265678ns | -0.6% | -5.1% | +623.3% |
| 5 | 259918ns | -2.8% | -2.5% | +635.1% |
| 6 | 253372ns | +3.6% | +1.0% | +662.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_hotfirst_h64 | -0.373 | moderate- |
| ml_ifchain_h64 | -0.160 | ok |
| ml_jumptable_h64 | -0.280 | moderate- |
| ml_tree_h64 | 0.395 | moderate+ |

**Consistency summary:**

- **ml_hotfirst_h64**: won 4/6, lost 2/6
- **ml_ifchain_h64**: won 4/6, lost 2/6
- **ml_tree_h64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_hotfirst_h64 | 4.0ns | 256886.5ns | 0.0% |  |
| ml_ifchain_h64 | 5.7ns | 255959.7ns | 0.0% |  |
| ml_jumptable_h64 | 7.2ns | 259375.1ns | 0.0% |  |
| ml_tree_h64 | 48.1ns | 1935872.4ns | 0.0% |  |

## Distribution (algo ns)

```
ml_hotfirst_h64 (n=6, range 251585.8-263381.1 ns)
  251585.8 |########################################
  252175.6 |########################################
  252765.3 |
  253355.1 |
  253944.9 |
  254534.6 |########################################
  255124.4 |########################################
  255714.1 |
  256303.9 |
  256893.7 |
  257483.4 |
  258073.2 |
  258663.0 |
  259252.7 |
  259842.5 |
  260432.2 |
  261022.0 |
  261611.8 |
  262201.5 |########################################
  262791.3 |
  (0 below, 1 above range)

ml_ifchain_h64 (n=6, range 252043.3-260065.4 ns)
  252043.3 |########################################
  252444.4 |
  252845.5 |
  253246.6 |########################################
  253647.7 |
  254048.8 |########################################
  254449.9 |
  254851.0 |
  255252.1 |
  255653.2 |########################################
  256054.4 |
  256455.5 |
  256856.6 |
  257257.7 |
  257658.8 |
  258059.9 |########################################
  258461.0 |
  258862.1 |
  259263.2 |
  259664.3 |
  (0 below, 1 above range)

ml_jumptable_h64 (n=6, range 251721.2-265689.5 ns)
  251721.2 |####################
  252419.6 |
  253118.0 |####################
  253816.5 |
  254514.9 |
  255213.3 |
  255911.7 |
  256610.1 |
  257308.5 |
  258007.0 |
  258705.4 |
  259403.8 |########################################
  260102.2 |
  260800.6 |
  261499.0 |
  262197.5 |
  262895.9 |
  263594.3 |
  264292.7 |
  264991.1 |####################
  (0 below, 1 above range)

ml_tree_h64 (n=6, range 1910729.6-1955360.6 ns)
  1910729.6 |########################################
  1912961.2 |
  1915192.7 |
  1917424.2 |
  1919655.8 |########################################
  1921887.4 |
  1924118.9 |
  1926350.5 |
  1928582.0 |
  1930813.6 |########################################
  1933045.1 |
  1935276.7 |
  1937508.2 |
  1939739.8 |########################################
  1941971.3 |
  1944202.9 |########################################
  1946434.4 |
  1948666.0 |
  1950897.5 |
  1953129.1 |
  (0 below, 1 above range)

```
