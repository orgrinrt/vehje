# Per-type strategy (NATIVE tier): all ifchain

5 variants, 6 samples per variant.
Baseline: **an_ifchain_table**

## Highlights

Baseline for all deltas below: **an_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_ifchain_table shows alternating (throttle bounce) (autocorr -0.53)

an_ifchain_table's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {an_ifchain_prof, an_ifchain_seq, an_ifchain_tree, an_ifchain_table} vs {an_ifchain_pred} (26% apart)

The field splits into a fast tier {an_ifchain_prof, an_ifchain_seq, an_ifchain_tree, an_ifchain_table} and a slow tier {an_ifchain_pred} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: an_ifchain_prof** at 305635.6 ns median (-10.0% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.40x (fastest 305635.6 ns, slowest 426664.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_ifchain_pred | 428890ns | 429360ns | 421063ns | 427878ns | 434321ns | +26.22% |
| an_ifchain_prof | 311396ns | 308179ns | 303590ns | 307052ns | 321816ns | -8.36% |
| an_ifchain_seq | 314821ns | 316125ns | 296350ns | 314712ns | 324221ns | -7.35% |
| an_ifchain_table | 339798ns | 342403ns | 328760ns | 338956ns | 346579ns | base |
| an_ifchain_tree | 330309ns | 330419ns | 324611ns | 328602ns | 335719ns | -2.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_ifchain_pred | 426276ns | 418380ns | 431680ns | +26.44% | 0.038 |
| an_ifchain_prof | 308715ns | 300987ns | 319026ns | -8.43% | 0.053 |
| an_ifchain_seq | 312300ns | 294072ns | 321512ns | -7.37% | 0.052 |
| an_ifchain_table | 337141ns | 326175ns | 343879ns | base | 0.049 |
| an_ifchain_tree | 327757ns | 322019ns | 332992ns | -2.78% | 0.050 |

## Performance model

- Peak throughput: **0.056 Gops/s** (an_ifchain_seq; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_ifchain_pred | 0.038 | 68.9% |
| an_ifchain_prof | 0.054 | 96.2% |
| an_ifchain_seq | 0.052 | 93.8% |
| an_ifchain_table | 0.048 | 86.6% |
| an_ifchain_tree | 0.050 | 89.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_ifchain_pred | 428890ns | 428890ns | +26.22% |
| an_ifchain_prof | 311396ns | 311396ns | -8.36% |
| an_ifchain_seq | 314821ns | 314821ns | -7.35% |
| an_ifchain_table | 339798ns | 339798ns | base |
| an_ifchain_tree | 330309ns | 330309ns | -2.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_ifchain_table | 339579ns | base | --- | [327966, 343879] | --- | --- | --- | --- |
| an_ifchain_pred | 426664ns | +91180.1ns (+26.9%) | [+83706, +92518]ns | [420483, 431680] | YES | 0.0313 | 0.0313 | 0 |
| an_ifchain_prof | 305636ns | -26437.2ns (-7.8%) | [-41015, -17828]ns | [301482, 319026] | YES | 0.0313 | 0.0313 | 0 |
| an_ifchain_seq | 313609ns | -27501.5ns (-8.1%) | [-33815, -13209]ns | [301777, 321512] | YES | 0.0313 | 0.0313 | 0 |
| an_ifchain_tree | 328105ns | -8299.8ns (-2.4%) | [-17225, -2628]ns | [322174, 332992] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_ifchain_table | an_ifchain_pred | an_ifchain_prof | an_ifchain_seq | an_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 344490ns | +23.5% | -11.8% | -8.5% | -3.9% |
| 2 | 326175ns | +28.3% | -5.7% | -1.7% | -1.3% |
| 3 | 341427ns | +25.3% | -7.1% | -9.4% | -3.6% |
| 4 | 343269ns | +26.8% | -12.0% | -6.0% | -6.1% |
| 5 | 329756ns | +28.2% | -8.7% | -10.8% | -0.8% |
| 6 | 337732ns | +26.8% | -5.0% | -7.7% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_ifchain_pred | -0.198 | ok |
| an_ifchain_prof | -0.295 | moderate- |
| an_ifchain_seq | -0.396 | moderate- |
| an_ifchain_table | -0.529 | HIGH- (thermal bounce) |
| an_ifchain_tree | -0.261 | moderate- |

**Consistency summary:**

- **an_ifchain_pred**: won 0/6, lost 6/6
- **an_ifchain_prof**: won 6/6, lost 0/6
- **an_ifchain_seq**: won 6/6, lost 0/6
- **an_ifchain_tree**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_ifchain_pred | 9.6ns | 426276.0ns | 0.0% |  |
| an_ifchain_prof | 7.8ns | 308714.8ns | 0.0% |  |
| an_ifchain_seq | 7.1ns | 312299.6ns | 0.0% |  |
| an_ifchain_table | 10.3ns | 337141.4ns | 0.0% |  |
| an_ifchain_tree | 17.6ns | 327757.1ns | 0.0% |  |

## Distribution (algo ns)

```
an_ifchain_pred (n=6, range 418379.6-431680.4 ns)
  418379.6 |####################
  419044.6 |
  419709.7 |
  420374.7 |
  421039.8 |
  421704.8 |
  422369.8 |####################
  423034.9 |
  423699.9 |
  424365.0 |
  425030.0 |####################
  425695.0 |
  426360.1 |
  427025.1 |
  427690.2 |########################################
  428355.2 |
  429020.2 |
  429685.3 |
  430350.3 |
  431015.4 |
  (0 below, 1 above range)

an_ifchain_prof (n=6, range 300986.7-319026.5 ns)
  300986.7 |########################################
  301888.7 |########################################
  302790.7 |
  303692.7 |########################################
  304594.7 |
  305496.6 |
  306398.6 |
  307300.6 |########################################
  308202.6 |
  309104.6 |
  310006.6 |
  310908.6 |
  311810.5 |
  312712.5 |
  313614.5 |
  314516.5 |
  315418.5 |
  316320.5 |
  317222.5 |########################################
  318124.5 |
  (0 below, 1 above range)

an_ifchain_seq (n=6, range 294071.7-321512.5 ns)
  294071.7 |########################################
  295443.7 |
  296815.8 |
  298187.8 |
  299559.9 |
  300931.9 |
  302303.9 |
  303676.0 |
  305048.0 |
  306420.1 |
  307792.1 |
  309164.1 |########################################
  310536.2 |########################################
  311908.2 |
  313280.3 |
  314652.3 |########################################
  316024.3 |
  317396.4 |
  318768.4 |
  320140.5 |########################################
  (0 below, 1 above range)

an_ifchain_table (n=6, range 326175.0-343879.4 ns)
  326175.0 |########################################
  327060.2 |
  327945.4 |
  328830.7 |
  329715.9 |########################################
  330601.1 |
  331486.3 |
  332371.5 |
  333256.8 |
  334142.0 |
  335027.2 |
  335912.4 |
  336797.6 |
  337682.9 |########################################
  338568.1 |
  339453.3 |
  340338.5 |
  341223.7 |########################################
  342109.0 |
  342994.2 |########################################
  (0 below, 1 above range)

an_ifchain_tree (n=6, range 322018.8-332992.1 ns)
  322018.8 |########################################
  322567.5 |
  323116.1 |
  323664.8 |
  324213.5 |
  324762.1 |
  325310.8 |
  325859.5 |
  326408.1 |
  326956.8 |####################
  327505.4 |
  328054.1 |
  328602.8 |####################
  329151.4 |
  329700.1 |
  330248.8 |
  330797.4 |####################
  331346.1 |
  331894.8 |
  332443.4 |
  (0 below, 1 above range)

```
