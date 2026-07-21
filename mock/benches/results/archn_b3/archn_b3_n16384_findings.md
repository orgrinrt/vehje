# Per-branch strategy (NATIVE tier): archetype 3

5 variants, 6 samples per variant.
Baseline: **an_b3_table**

## Highlights

Baseline for all deltas below: **an_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b3_seq, an_b3_table) are a dead heat (<1%)

an_b3_seq (325.75 us) and an_b3_table (325.81 us) differ by 0.02%, inside the noise, even though the wider field spreads 25.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: an_b3_seq** at 325751.5 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.26x (fastest 325751.5 ns, slowest 409127.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b3_pred | 413025ns | 411891ns | 404997ns | 409749ns | 421952ns | +25.35% |
| an_b3_prof | 334639ns | 333229ns | 320760ns | 331291ns | 346601ns | +1.56% |
| an_b3_seq | 327411ns | 328185ns | 319164ns | 328125ns | 330464ns | -0.63% |
| an_b3_table | 329501ns | 328437ns | 318057ns | 326100ns | 340324ns | base |
| an_b3_tree | 334990ns | 332947ns | 316827ns | 332446ns | 347888ns | +1.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b3_pred | 410307ns | 402735ns | 419052ns | +25.51% | 0.040 |
| an_b3_prof | 332121ns | 318418ns | 343763ns | +1.59% | 0.049 |
| an_b3_seq | 324979ns | 316965ns | 327988ns | -0.59% | 0.050 |
| an_b3_table | 326909ns | 315380ns | 337857ns | base | 0.050 |
| an_b3_tree | 332423ns | 314256ns | 345171ns | +1.69% | 0.049 |

## Performance model

- Peak throughput: **0.052 Gops/s** (an_b3_tree; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b3_pred | 0.040 | 76.8% |
| an_b3_prof | 0.050 | 95.0% |
| an_b3_seq | 0.050 | 96.5% |
| an_b3_table | 0.050 | 96.5% |
| an_b3_tree | 0.050 | 95.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b3_pred | 413025ns | 413025ns | +25.35% |
| an_b3_prof | 334639ns | 334639ns | +1.56% |
| an_b3_seq | 327411ns | 327411ns | -0.63% |
| an_b3_table | 329501ns | 329501ns | base |
| an_b3_tree | 334990ns | 334990ns | +1.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b3_table | 325813ns | base | --- | [317056, 337857] | --- | --- | --- | --- |
| an_b3_pred | 409128ns | +84076.2ns (+25.8%) | [+68372, +97748]ns | [402742, 419052] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b3_prof | 330794ns | no significant difference | [-4822, +16982]ns | [321805, 343763] | no | 0.9167 | 0.6875 | 0 |
| an_b3_seq | 325751ns | no significant difference | [-11738, +7535]ns | [321197, 327988] | no | 1.0000 | 1.0000 | 0 |
| an_b3_tree | 330634ns | no significant difference | [-7433, +18657]ns | [321465, 345171] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b3_table | an_b3_pred | an_b3_prof | an_b3_seq | an_b3_tree |
|---|---|---|---|---|---|
| 1 | 318733ns | +26.4% | +4.4% | +3.3% | +3.1% |
| 2 | 321279ns | +29.8% | +1.2% | +1.4% | -2.2% |
| 3 | 330347ns | +22.0% | +6.0% | -1.4% | +0.2% |
| 4 | 338835ns | +18.9% | -0.4% | -3.5% | -2.3% |
| 5 | 336879ns | +25.0% | -2.4% | -3.4% | +6.7% |
| 6 | 315380ns | +31.6% | +1.0% | +0.5% | +4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b3_pred | -0.216 | moderate- |
| an_b3_prof | -0.012 | ok |
| an_b3_seq | 0.033 | ok |
| an_b3_table | 0.148 | ok |
| an_b3_tree | -0.001 | ok |

**Consistency summary:**

- **an_b3_pred**: won 0/6, lost 6/6
- **an_b3_prof**: won 2/6, lost 4/6
- **an_b3_seq**: won 3/6, lost 3/6
- **an_b3_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b3_pred | 8.1ns | 410307.4ns | 0.0% |  |
| an_b3_prof | 9.5ns | 332120.8ns | 0.0% |  |
| an_b3_seq | 9.0ns | 324978.6ns | 0.0% |  |
| an_b3_table | 13.9ns | 326908.8ns | 0.0% |  |
| an_b3_tree | 9.8ns | 332423.3ns | 0.0% |  |

## Distribution (algo ns)

```
an_b3_pred (n=6, range 402734.6-419052.5 ns)
  402734.6 |########################################
  403550.5 |
  404366.4 |
  405182.3 |
  405998.2 |
  406814.1 |
  407630.0 |
  408445.9 |
  409261.8 |
  410077.7 |
  410893.5 |
  411709.4 |
  412525.3 |
  413341.2 |
  414157.1 |
  414973.0 |#############
  415788.9 |
  416604.8 |#############
  417420.7 |
  418236.6 |
  (0 below, 1 above range)

an_b3_prof (n=6, range 318417.5-343763.2 ns)
  318417.5 |########################################
  319684.8 |
  320952.1 |
  322219.3 |
  323486.6 |
  324753.9 |########################################
  326021.2 |
  327288.5 |
  328555.8 |########################################
  329823.0 |
  331090.3 |
  332357.6 |########################################
  333624.9 |
  334892.2 |
  336159.5 |########################################
  337426.7 |
  338694.0 |
  339961.3 |
  341228.6 |
  342495.9 |
  (0 below, 1 above range)

an_b3_seq (n=6, range 316965.4-327987.5 ns)
  316965.4 |####################
  317516.5 |
  318067.6 |
  318618.7 |
  319169.8 |
  319720.9 |
  320272.0 |
  320823.1 |
  321374.2 |
  321925.3 |
  322476.5 |
  323027.6 |
  323578.7 |
  324129.8 |
  324680.9 |
  325232.0 |########################################
  325783.1 |####################
  326334.2 |####################
  326885.3 |
  327436.4 |
  (0 below, 1 above range)

an_b3_table (n=6, range 315380.0-337856.9 ns)
  315380.0 |########################################
  316503.8 |
  317627.7 |########################################
  318751.5 |
  319875.4 |
  320999.2 |########################################
  322123.1 |
  323246.9 |
  324370.8 |
  325494.6 |
  326618.5 |
  327742.3 |
  328866.1 |
  329990.0 |########################################
  331113.8 |
  332237.7 |
  333361.5 |
  334485.4 |
  335609.2 |
  336733.1 |########################################
  (0 below, 1 above range)

an_b3_tree (n=6, range 314255.8-345171.5 ns)
  314255.8 |#############
  315801.6 |
  317347.4 |
  318893.1 |
  320438.9 |
  321984.7 |
  323530.5 |
  325076.3 |
  326622.1 |
  328167.8 |#############
  329713.6 |########################################
  331259.4 |
  332805.2 |
  334351.0 |
  335896.8 |
  337442.5 |
  338988.3 |
  340534.1 |
  342079.9 |
  343625.7 |
  (0 below, 1 above range)

```
