# Per-branch strategy (NATIVE tier): archetype 0

5 variants, 6 samples per variant.
Baseline: **an_b0_table**

## Highlights

Baseline for all deltas below: **an_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: an_b0_seq** at 52059.2 ns median (-1.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.11x (fastest 52059.2 ns, slowest 57881.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b0_pred | 58516ns | 60194ns | 51045ns | 58785ns | 61846ns | +4.11% |
| an_b0_prof | 60727ns | 60197ns | 56205ns | 59184ns | 65303ns | +8.04% |
| an_b0_seq | 55334ns | 54312ns | 52152ns | 53914ns | 59054ns | -1.55% |
| an_b0_table | 56206ns | 55226ns | 51669ns | 55052ns | 60204ns | base |
| an_b0_tree | 56283ns | 56742ns | 47322ns | 55500ns | 61939ns | +0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b0_pred | 56180ns | 48701ns | 59587ns | +4.18% | 0.073 |
| an_b0_prof | 58369ns | 53777ns | 62839ns | +8.24% | 0.070 |
| an_b0_seq | 53031ns | 49838ns | 56685ns | -1.66% | 0.077 |
| an_b0_table | 53928ns | 49482ns | 57858ns | base | 0.076 |
| an_b0_tree | 53883ns | 44646ns | 59595ns | -0.08% | 0.076 |

## Performance model

- Peak throughput: **0.092 Gops/s** (an_b0_tree; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b0_pred | 0.071 | 77.2% |
| an_b0_prof | 0.071 | 77.1% |
| an_b0_seq | 0.079 | 85.8% |
| an_b0_table | 0.077 | 84.1% |
| an_b0_tree | 0.075 | 82.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b0_pred | 58516ns | 58516ns | +4.11% |
| an_b0_prof | 60727ns | 60727ns | +8.04% |
| an_b0_seq | 55334ns | 55334ns | -1.55% |
| an_b0_table | 56206ns | 56206ns | base |
| an_b0_tree | 56283ns | 56283ns | +0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b0_table | 53062ns | base | --- | [50863, 57858] | --- | --- | --- | --- |
| an_b0_pred | 57863ns | no significant difference | [-102, +5606]ns | [51090, 59587] | no | 0.4375 | 0.2188 | 0 |
| an_b0_prof | 57881ns | +4795.6ns (+9.0%) | [+1696, +6831]ns | [54386, 62839] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b0_seq | 52059ns | no significant difference | [-4330, +2218]ns | [50350, 56685] | no | 1.0000 | 1.0000 | 0 |
| an_b0_tree | 54328ns | no significant difference | [-3614, +3149]ns | [47725, 59595] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b0_table | an_b0_pred | an_b0_prof | an_b0_seq | an_b0_tree |
|---|---|---|---|---|---|
| 1 | 49482ns | -1.6% | +11.1% | +0.7% | -9.8% |
| 2 | 52928ns | +10.2% | +1.6% | -2.6% | +6.7% |
| 3 | 56832ns | +1.0% | +10.4% | +0.4% | +4.9% |
| 4 | 53195ns | +10.9% | +4.8% | -4.4% | -4.5% |
| 5 | 52244ns | +2.4% | +14.9% | +7.8% | -0.1% |
| 6 | 58885ns | +2.2% | +6.9% | -10.7% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b0_pred | -0.304 | moderate- |
| an_b0_prof | -0.154 | ok |
| an_b0_seq | -0.421 | moderate- |
| an_b0_table | -0.135 | ok |
| an_b0_tree | -0.183 | ok |

**Consistency summary:**

- **an_b0_pred**: won 1/6, lost 5/6
- **an_b0_prof**: won 0/6, lost 6/6
- **an_b0_seq**: won 3/6, lost 3/6
- **an_b0_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b0_pred | 5.2ns | 56180.0ns | 0.0% |  |
| an_b0_prof | 5.2ns | 58368.6ns | 0.0% |  |
| an_b0_seq | 10.1ns | 53031.5ns | 0.0% |  |
| an_b0_table | 6.3ns | 53927.6ns | 0.0% |  |
| an_b0_tree | 6.0ns | 53882.6ns | 0.0% |  |

## Distribution (algo ns)

```
an_b0_pred (n=6, range 48700.8-59586.9 ns)
  48700.8 |########################################
  49245.1 |
  49789.4 |
  50333.7 |
  50878.0 |
  51422.3 |
  51966.6 |
  52510.9 |
  53055.2 |########################################
  53599.5 |
  54143.8 |
  54688.1 |
  55232.4 |
  55776.7 |
  56321.0 |
  56865.3 |########################################
  57409.6 |
  57953.9 |########################################
  58498.2 |########################################
  59042.5 |
  (0 below, 1 above range)

an_b0_prof (n=6, range 53776.7-62838.9 ns)
  53776.7 |########################################
  54229.8 |
  54682.9 |########################################
  55136.0 |
  55589.1 |########################################
  56042.3 |
  56495.4 |
  56948.5 |
  57401.6 |
  57854.7 |
  58307.8 |
  58760.9 |
  59214.0 |
  59667.2 |########################################
  60120.3 |
  60573.4 |
  61026.5 |
  61479.6 |
  61932.7 |
  62385.8 |########################################
  (0 below, 1 above range)

an_b0_seq (n=6, range 49838.3-56684.8 ns)
  49838.3 |########################################
  50180.6 |
  50523.0 |########################################
  50865.3 |
  51207.6 |
  51549.9 |########################################
  51892.2 |
  52234.6 |########################################
  52576.9 |
  52919.2 |
  53261.6 |
  53603.9 |
  53946.2 |
  54288.5 |
  54630.9 |
  54973.2 |
  55315.5 |
  55657.8 |
  56000.2 |########################################
  56342.5 |
  (0 below, 1 above range)

an_b0_table (n=6, range 49481.7-57858.3 ns)
  49481.7 |####################
  49900.5 |
  50319.4 |
  50738.2 |
  51157.0 |
  51575.9 |
  51994.7 |####################
  52413.5 |
  52832.4 |########################################
  53251.2 |
  53670.0 |
  54088.9 |
  54507.7 |
  54926.5 |
  55345.4 |
  55764.2 |
  56183.0 |
  56601.9 |####################
  57020.7 |
  57439.5 |
  (0 below, 1 above range)

an_b0_tree (n=6, range 44645.8-59594.8 ns)
  44645.8 |########################################
  45393.2 |
  46140.7 |
  46888.2 |
  47635.6 |
  48383.1 |
  49130.5 |
  49878.0 |
  50625.4 |########################################
  51372.9 |
  52120.3 |########################################
  52867.8 |
  53615.2 |
  54362.7 |
  55110.1 |
  55857.6 |########################################
  56605.0 |
  57352.5 |
  58099.9 |
  58847.4 |########################################
  (0 below, 1 above range)

```
