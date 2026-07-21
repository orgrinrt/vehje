# Per-branch strategy (NATIVE tier): archetype 1

5 variants, 6 samples per variant.
Baseline: **an_b1_table**

## Highlights

Baseline for all deltas below: **an_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b1_pred shows alternating (throttle bounce) (autocorr -0.70)

an_b1_pred's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: an_b1_tree** at 326088.3 ns median (-1.6% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.10x (fastest 326088.3 ns, slowest 357216.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b1_pred | 357488ns | 359905ns | 346345ns | 356066ns | 365194ns | +6.83% |
| an_b1_prof | 337928ns | 334546ns | 324473ns | 333928ns | 350656ns | +0.99% |
| an_b1_seq | 336057ns | 338840ns | 322720ns | 334748ns | 344690ns | +0.43% |
| an_b1_table | 334626ns | 333938ns | 326246ns | 332355ns | 342223ns | base |
| an_b1_tree | 334546ns | 328465ns | 327565ns | 328185ns | 347579ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b1_pred | 354949ns | 344167ns | 362697ns | +6.87% | 0.046 |
| an_b1_prof | 335266ns | 321801ns | 347995ns | +0.94% | 0.049 |
| an_b1_seq | 333477ns | 320362ns | 342075ns | +0.41% | 0.049 |
| an_b1_table | 332129ns | 323930ns | 339710ns | base | 0.049 |
| an_b1_tree | 332126ns | 324950ns | 345142ns | -0.00% | 0.049 |

## Performance model

- Peak throughput: **0.051 Gops/s** (an_b1_seq; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b1_pred | 0.046 | 89.7% |
| an_b1_prof | 0.049 | 96.6% |
| an_b1_seq | 0.049 | 95.3% |
| an_b1_table | 0.049 | 96.6% |
| an_b1_tree | 0.050 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b1_pred | 357488ns | 357488ns | +6.83% |
| an_b1_prof | 337928ns | 337928ns | +0.99% |
| an_b1_seq | 336057ns | 336057ns | +0.43% |
| an_b1_table | 334626ns | 334626ns | base |
| an_b1_tree | 334546ns | 334546ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b1_table | 331501ns | base | --- | [325178, 339710] | --- | --- | --- | --- |
| an_b1_pred | 357217ns | +22407.7ns (+6.8%) | [+11416, +34634]ns | [344932, 362697] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b1_prof | 331805ns | no significant difference | [-6750, +14031]ns | [325997, 347995] | no | 1.0000 | 0.6875 | 0 |
| an_b1_seq | 336165ns | no significant difference | [-9661, +13191]ns | [322191, 342075] | no | 1.0000 | 1.0000 | 0 |
| an_b1_tree | 326088ns | no significant difference | [-14561, +15282]ns | [325148, 345142] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b1_table | an_b1_pred | an_b1_prof | an_b1_seq | an_b1_tree |
|---|---|---|---|---|---|
| 1 | 323930ns | +10.5% | +2.4% | +5.4% | +0.5% |
| 2 | 326426ns | +10.8% | +1.2% | -1.9% | +6.0% |
| 3 | 333837ns | +3.6% | -0.6% | +2.7% | -2.5% |
| 4 | 333295ns | +9.1% | -3.4% | +2.0% | +3.3% |
| 5 | 329707ns | +4.4% | +6.1% | -1.7% | -1.0% |
| 6 | 345582ns | +3.2% | +0.1% | -3.8% | -6.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b1_pred | -0.700 | HIGH- (thermal bounce) |
| an_b1_prof | 0.075 | ok |
| an_b1_seq | -0.476 | moderate- |
| an_b1_table | 0.012 | ok |
| an_b1_tree | -0.575 | HIGH- (thermal bounce) |

**Consistency summary:**

- **an_b1_pred**: won 0/6, lost 6/6
- **an_b1_prof**: won 2/6, lost 4/6
- **an_b1_seq**: won 3/6, lost 3/6
- **an_b1_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b1_pred | 14.2ns | 354948.8ns | 0.0% |  |
| an_b1_prof | 10.9ns | 335265.6ns | 0.0% |  |
| an_b1_seq | 14.7ns | 333476.9ns | 0.0% |  |
| an_b1_table | 9.2ns | 332129.5ns | 0.0% |  |
| an_b1_tree | 7.7ns | 332126.2ns | 0.0% |  |

## Distribution (algo ns)

```
an_b1_pred (n=6, range 344167.1-362697.3 ns)
  344167.1 |########################################
  345093.6 |########################################
  346020.1 |
  346946.6 |
  347873.1 |
  348799.6 |
  349726.2 |
  350652.7 |
  351579.2 |
  352505.7 |
  353432.2 |
  354358.7 |
  355285.2 |
  356211.7 |########################################
  357138.2 |########################################
  358064.8 |
  358991.3 |
  359917.8 |
  360844.3 |########################################
  361770.8 |
  (0 below, 1 above range)

an_b1_prof (n=6, range 321800.8-347994.8 ns)
  321800.8 |####################
  323110.5 |
  324420.2 |
  325729.9 |
  327039.6 |
  328349.3 |
  329659.0 |####################
  330968.7 |########################################
  332278.4 |
  333588.1 |
  334897.8 |
  336207.5 |
  337517.2 |
  338826.9 |
  340136.6 |
  341446.3 |
  342756.0 |
  344065.7 |
  345375.4 |####################
  346685.1 |
  (0 below, 1 above range)

an_b1_seq (n=6, range 320362.1-342074.6 ns)
  320362.1 |########################################
  321447.7 |
  322533.3 |
  323619.0 |########################################
  324704.6 |
  325790.2 |
  326875.8 |
  327961.5 |
  329047.1 |
  330132.7 |
  331218.3 |
  332303.9 |########################################
  333389.6 |
  334475.2 |
  335560.8 |
  336646.4 |
  337732.1 |
  338817.7 |
  339903.3 |########################################
  340988.9 |########################################
  (0 below, 1 above range)

an_b1_table (n=6, range 323930.0-339709.6 ns)
  323930.0 |########################################
  324719.0 |
  325508.0 |
  326296.9 |########################################
  327085.9 |
  327874.9 |
  328663.9 |
  329452.9 |########################################
  330241.8 |
  331030.8 |
  331819.8 |
  332608.8 |########################################
  333397.8 |########################################
  334186.7 |
  334975.7 |
  335764.7 |
  336553.7 |
  337342.7 |
  338131.6 |
  338920.6 |
  (0 below, 1 above range)

an_b1_tree (n=6, range 324950.4-345142.3 ns)
  324950.4 |########################################
  325960.0 |#############
  326969.6 |
  327979.2 |
  328988.8 |
  329998.4 |
  331008.0 |
  332017.6 |
  333027.2 |
  334036.8 |
  335046.3 |
  336055.9 |
  337065.5 |
  338075.1 |
  339084.7 |
  340094.3 |
  341103.9 |
  342113.5 |
  343123.1 |
  344132.7 |#############
  (0 below, 1 above range)

```
