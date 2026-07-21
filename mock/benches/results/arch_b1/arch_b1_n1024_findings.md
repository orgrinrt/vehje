# Per-branch strategy: archetype 1 (match4_nested), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b1_table**

## Highlights

Baseline for all deltas below: **ab_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b1_seq, ab_b1_prof) are a dead heat (<1%)

ab_b1_seq (323.24 us) and ab_b1_prof (325.83 us) differ by 0.80%, inside the noise, even though the wider field spreads 21.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b1_prof shows alternating (throttle bounce) (autocorr -0.66)

ab_b1_prof's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: ab_b1_seq** at 323237.7 ns median (-1.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.21x (fastest 323237.7 ns, slowest 392463.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b1_pred | 394758ns | 395149ns | 390405ns | 393846ns | 398301ns | +19.28% |
| ab_b1_prof | 327965ns | 328430ns | 324874ns | 327526ns | 330170ns | -0.90% |
| ab_b1_seq | 325985ns | 325709ns | 322046ns | 324626ns | 329991ns | -1.50% |
| ab_b1_table | 330953ns | 330053ns | 326910ns | 329140ns | 335694ns | base |
| ab_b1_tree | 335975ns | 336573ns | 325832ns | 333117ns | 345334ns | +1.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b1_pred | 392217ns | 387850ns | 395996ns | +19.39% | 0.003 |
| ab_b1_prof | 325513ns | 322202ns | 327929ns | -0.91% | 0.003 |
| ab_b1_seq | 323653ns | 319813ns | 327749ns | -1.48% | 0.003 |
| ab_b1_table | 328514ns | 324160ns | 333472ns | base | 0.003 |
| ab_b1_tree | 333502ns | 323279ns | 342777ns | +1.52% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b1_seq; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b1_pred | 0.003 | 81.5% |
| ab_b1_prof | 0.003 | 98.2% |
| ab_b1_seq | 0.003 | 98.9% |
| ab_b1_table | 0.003 | 97.6% |
| ab_b1_tree | 0.003 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b1_pred | 394758ns | 394758ns | +19.28% |
| ab_b1_prof | 327965ns | 327965ns | -0.90% |
| ab_b1_seq | 325985ns | 325985ns | -1.50% |
| ab_b1_table | 330953ns | 330953ns | base |
| ab_b1_tree | 335975ns | 335975ns | +1.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b1_table | 327562ns | base | --- | [324509, 333472] | --- | --- | --- | --- |
| ab_b1_pred | 392464ns | +63631.5ns (+19.4%) | [+58132, +69346]ns | [388192, 395996] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_b1_prof | 325829ns | no significant difference | [-7643, +920]ns | [322781, 327929] | no | 0.2917 | 0.2188 | 0 |
| ab_b1_seq | 323238ns | -4616.1ns (-1.4%) | [-8094, -1872]ns | [319973, 327749] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_b1_tree | 334222ns | no significant difference | [-1371, +12307]ns | [323508, 342777] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b1_table | ab_b1_pred | ab_b1_prof | ab_b1_seq | ab_b1_tree |
|---|---|---|---|---|---|
| 1 | 324858ns | +20.9% | -0.5% | -0.4% | -0.5% |
| 2 | 330105ns | +17.5% | -0.8% | -0.8% | +4.0% |
| 3 | 333696ns | +17.5% | -2.6% | -3.2% | +2.5% |
| 4 | 333248ns | +18.9% | -2.0% | -1.6% | -0.3% |
| 5 | 324160ns | +19.9% | -0.6% | -1.3% | -0.1% |
| 6 | 325019ns | +21.8% | +1.0% | -1.5% | +3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b1_pred | -0.488 | moderate- |
| ab_b1_prof | -0.658 | HIGH- (thermal bounce) |
| ab_b1_seq | -0.153 | ok |
| ab_b1_table | 0.224 | moderate+ |
| ab_b1_tree | -0.109 | ok |

**Consistency summary:**

- **ab_b1_pred**: won 0/6, lost 6/6
- **ab_b1_prof**: won 5/6, lost 1/6
- **ab_b1_seq**: won 6/6, lost 0/6
- **ab_b1_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b1_pred | 5.8ns | 392217.1ns | 0.0% |  |
| ab_b1_prof | 4.6ns | 325513.0ns | 0.0% |  |
| ab_b1_seq | 7.0ns | 323653.3ns | 0.0% |  |
| ab_b1_table | 6.0ns | 328514.0ns | 0.0% |  |
| ab_b1_tree | 9.4ns | 333502.2ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b1_pred (n=6, range 387850.0-395996.2 ns)
  387850.0 |########################################
  388257.3 |########################################
  388664.6 |
  389071.9 |
  389479.2 |
  389886.6 |
  390293.9 |
  390701.2 |
  391108.5 |
  391515.8 |
  391923.1 |########################################
  392330.4 |########################################
  392737.8 |
  393145.1 |
  393552.4 |
  393959.7 |
  394367.0 |
  394774.3 |
  395181.6 |
  395588.9 |########################################
  (0 below, 1 above range)

ab_b1_prof (n=6, range 322202.1-327929.3 ns)
  322202.1 |########################################
  322488.5 |
  322774.8 |
  323061.2 |
  323347.5 |########################################
  323633.9 |
  323920.3 |
  324206.6 |
  324493.0 |
  324779.4 |########################################
  325065.7 |
  325352.1 |
  325638.4 |
  325924.8 |
  326211.2 |
  326497.5 |########################################
  326783.9 |
  327070.3 |
  327356.6 |########################################
  327643.0 |
  (0 below, 1 above range)

ab_b1_seq (n=6, range 319813.3-327749.2 ns)
  319813.3 |########################################
  320210.1 |
  320606.9 |
  321003.7 |
  321400.5 |
  321797.3 |
  322194.1 |
  322590.8 |####################
  322987.6 |
  323384.4 |####################
  323781.2 |
  324178.0 |
  324574.8 |
  324971.6 |
  325368.4 |
  325765.2 |
  326162.0 |
  326558.8 |
  326955.6 |
  327352.4 |####################
  (0 below, 1 above range)

ab_b1_table (n=6, range 324159.6-333471.8 ns)
  324159.6 |####################
  324625.2 |########################################
  325090.8 |
  325556.4 |
  326022.0 |
  326487.7 |
  326953.3 |
  327418.9 |
  327884.5 |
  328350.1 |
  328815.7 |
  329281.3 |
  329746.9 |####################
  330212.6 |
  330678.2 |
  331143.8 |
  331609.4 |
  332075.0 |
  332540.6 |
  333006.2 |####################
  (0 below, 1 above range)

ab_b1_tree (n=6, range 323279.2-342776.7 ns)
  323279.2 |########################################
  324254.1 |
  325228.9 |
  326203.8 |
  327178.7 |
  328153.6 |
  329128.4 |
  330103.3 |
  331078.2 |
  332053.1 |####################
  333027.9 |
  334002.8 |
  334977.7 |
  335952.5 |####################
  336927.4 |
  337902.3 |
  338877.2 |
  339852.0 |
  340826.9 |
  341801.8 |####################
  (0 below, 1 above range)

```
