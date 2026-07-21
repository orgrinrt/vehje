# Per-branch strategy (NATIVE tier): archetype 0

5 variants, 6 samples per variant.
Baseline: **an_b0_table**

## Highlights

Baseline for all deltas below: **an_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5.53 us) is smaller than the fastest variant's own run-to-run std-dev (7.69 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.7% of the fastest

All 5 variants sit between 327.78 us and 333.31 us - a 1.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: an_b0_seq** at 327780.8 ns median (-0.3% vs baseline)
- Spread: 1.02x (fastest 327780.8 ns, slowest 333315.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b0_pred | 336117ns | 333132ns | 325628ns | 331480ns | 348316ns | +1.07% |
| an_b0_prof | 332156ns | 332256ns | 322767ns | 330282ns | 339660ns | -0.12% |
| an_b0_seq | 329768ns | 330444ns | 320015ns | 327147ns | 338577ns | -0.84% |
| an_b0_table | 332558ns | 331270ns | 328290ns | 330455ns | 337847ns | base |
| an_b0_tree | 335049ns | 335782ns | 327157ns | 333235ns | 341715ns | +0.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b0_pred | 333648ns | 323452ns | 345714ns | +1.07% | 0.049 |
| an_b0_prof | 329681ns | 320518ns | 336968ns | -0.13% | 0.050 |
| an_b0_seq | 327219ns | 317532ns | 335953ns | -0.87% | 0.050 |
| an_b0_table | 330100ns | 325739ns | 335445ns | base | 0.050 |
| an_b0_tree | 332508ns | 324837ns | 338959ns | +0.73% | 0.049 |

## Performance model

- Peak throughput: **0.052 Gops/s** (an_b0_seq; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b0_pred | 0.050 | 96.0% |
| an_b0_prof | 0.050 | 96.3% |
| an_b0_seq | 0.050 | 96.9% |
| an_b0_table | 0.050 | 96.6% |
| an_b0_tree | 0.049 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b0_pred | 336117ns | 336117ns | +1.07% |
| an_b0_prof | 332156ns | 332156ns | -0.12% |
| an_b0_seq | 329768ns | 329768ns | -0.84% |
| an_b0_table | 332558ns | 332558ns | base |
| an_b0_tree | 335049ns | 335049ns | +0.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b0_table | 328756ns | base | --- | [326097, 335445] | --- | --- | --- | --- |
| an_b0_pred | 330646ns | no significant difference | [-10861, +18218]ns | [324583, 345714] | no | 0.6875 | 0.6875 | 0 |
| an_b0_prof | 329862ns | no significant difference | [-7110, +7155]ns | [322213, 336968] | no | 0.6875 | 0.6875 | 0 |
| an_b0_seq | 327781ns | no significant difference | [-15591, +8235]ns | [317923, 335953] | no | 0.6875 | 0.6875 | 0 |
| an_b0_tree | 333315ns | no significant difference | [-8488, +12862]ns | [325249, 338959] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b0_table | an_b0_pred | an_b0_prof | an_b0_seq | an_b0_tree |
|---|---|---|---|---|---|
| 1 | 325739ns | +1.6% | -0.6% | -0.2% | +5.1% |
| 2 | 328979ns | +0.4% | -2.6% | +1.3% | -1.0% |
| 3 | 332395ns | -2.7% | -0.2% | -0.6% | +0.7% |
| 4 | 338495ns | -3.8% | -1.7% | -6.0% | -4.0% |
| 5 | 328534ns | +3.5% | +3.9% | -3.3% | +1.1% |
| 6 | 326456ns | +7.6% | +0.5% | +3.7% | +2.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b0_pred | 0.345 | moderate+ |
| an_b0_prof | 0.218 | moderate+ |
| an_b0_seq | -0.129 | ok |
| an_b0_table | 0.127 | ok |
| an_b0_tree | -0.443 | moderate- |

**Consistency summary:**

- **an_b0_pred**: won 2/6, lost 4/6
- **an_b0_prof**: won 4/6, lost 2/6
- **an_b0_seq**: won 4/6, lost 2/6
- **an_b0_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b0_pred | 10.6ns | 333647.6ns | 0.0% |  |
| an_b0_prof | 16.6ns | 329680.8ns | 0.0% |  |
| an_b0_seq | 7.5ns | 327218.9ns | 0.0% |  |
| an_b0_table | 13.4ns | 330099.5ns | 0.0% |  |
| an_b0_tree | 12.8ns | 332507.8ns | 0.0% |  |

## Distribution (algo ns)

```
an_b0_pred (n=6, range 323451.7-345713.5 ns)
  323451.7 |####################
  324564.8 |
  325677.9 |####################
  326791.0 |
  327904.1 |
  329017.2 |
  330130.2 |########################################
  331243.3 |
  332356.4 |
  333469.5 |
  334582.6 |
  335695.7 |
  336808.8 |
  337921.9 |
  339035.0 |####################
  340148.0 |
  341261.1 |
  342374.2 |
  343487.3 |
  344600.4 |
  (0 below, 1 above range)

an_b0_prof (n=6, range 320518.3-336967.9 ns)
  320518.3 |########################################
  321340.8 |
  322163.3 |
  322985.7 |
  323808.2 |########################################
  324630.7 |
  325453.2 |
  326275.7 |
  327098.1 |
  327920.6 |########################################
  328743.1 |
  329565.6 |
  330388.1 |
  331210.5 |########################################
  332033.0 |########################################
  332855.5 |
  333678.0 |
  334500.5 |
  335322.9 |
  336145.4 |
  (0 below, 1 above range)

an_b0_seq (n=6, range 317531.7-335952.8 ns)
  317531.7 |########################################
  318452.8 |
  319373.8 |
  320294.9 |
  321215.9 |
  322137.0 |
  323058.0 |
  323979.1 |
  324900.1 |####################
  325821.2 |
  326742.2 |
  327663.3 |
  328584.3 |
  329505.4 |
  330426.4 |####################
  331347.5 |
  332268.5 |
  333189.6 |####################
  334110.6 |
  335031.7 |
  (0 below, 1 above range)

an_b0_table (n=6, range 325738.7-335444.6 ns)
  325738.7 |########################################
  326224.0 |########################################
  326709.3 |
  327194.6 |
  327679.9 |
  328165.2 |########################################
  328650.5 |########################################
  329135.8 |
  329621.1 |
  330106.4 |
  330591.7 |
  331076.9 |
  331562.2 |
  332047.5 |########################################
  332532.8 |
  333018.1 |
  333503.4 |
  333988.7 |
  334474.0 |
  334959.3 |
  (0 below, 1 above range)

an_b0_tree (n=6, range 324837.1-338959.0 ns)
  324837.1 |########################################
  325543.2 |########################################
  326249.3 |
  326955.4 |
  327661.5 |
  328367.6 |
  329073.7 |
  329779.7 |
  330485.8 |
  331191.9 |
  331898.0 |########################################
  332604.1 |
  333310.2 |
  334016.3 |########################################
  334722.4 |
  335428.5 |########################################
  336134.6 |
  336840.7 |
  337546.8 |
  338252.9 |
  (0 below, 1 above range)

```
