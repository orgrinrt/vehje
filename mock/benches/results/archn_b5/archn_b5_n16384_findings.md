# Per-branch strategy (NATIVE tier): archetype 5

5 variants, 6 samples per variant.
Baseline: **an_b5_table**

## Highlights

Baseline for all deltas below: **an_b5_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b5_tree, an_b5_table) are a dead heat (<1%)

an_b5_tree (328.10 us) and an_b5_table (329.19 us) differ by 0.33%, inside the noise, even though the wider field spreads 9.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### an_b5_seq shows warm-up / thermal drift (autocorr +0.52)

an_b5_seq's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: an_b5_tree** at 328099.6 ns median (-0.3% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.10x (fastest 328099.6 ns, slowest 359975.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b5_pred | 362575ns | 362645ns | 353512ns | 362201ns | 367669ns | +9.17% |
| an_b5_prof | 354724ns | 354931ns | 347838ns | 353225ns | 360416ns | +6.81% |
| an_b5_seq | 353894ns | 353380ns | 349792ns | 352529ns | 357994ns | +6.56% |
| an_b5_table | 332108ns | 331683ns | 323324ns | 330151ns | 339435ns | base |
| an_b5_tree | 331319ns | 330796ns | 327265ns | 330443ns | 334660ns | -0.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b5_pred | 359886ns | 351033ns | 364850ns | +9.20% | 0.046 |
| an_b5_prof | 352097ns | 345419ns | 357871ns | +6.84% | 0.047 |
| an_b5_seq | 351289ns | 347108ns | 355420ns | +6.59% | 0.047 |
| an_b5_table | 329557ns | 320685ns | 336748ns | base | 0.050 |
| an_b5_tree | 328749ns | 325000ns | 332087ns | -0.25% | 0.050 |

## Performance model

- Peak throughput: **0.051 Gops/s** (an_b5_table; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b5_pred | 0.046 | 89.1% |
| an_b5_prof | 0.047 | 91.1% |
| an_b5_seq | 0.047 | 91.5% |
| an_b5_table | 0.050 | 97.4% |
| an_b5_tree | 0.050 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b5_pred | 362575ns | 362575ns | +9.17% |
| an_b5_prof | 354724ns | 354724ns | +6.81% |
| an_b5_seq | 353894ns | 353894ns | +6.56% |
| an_b5_table | 332108ns | 332108ns | base |
| an_b5_tree | 331319ns | 331319ns | -0.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b5_table | 329191ns | base | --- | [322731, 336748] | --- | --- | --- | --- |
| an_b5_pred | 359975ns | +28746.9ns (+8.7%) | [+24947, +37294]ns | [354833, 364850] | YES | 0.0417 | 0.0313 | 0 |
| an_b5_prof | 352191ns | +25955.2ns (+7.9%) | [+9482, +32185]ns | [346230, 357871] | YES | 0.0417 | 0.0313 | 0 |
| an_b5_seq | 350629ns | +19431.5ns (+5.9%) | [+13864, +31903]ns | [347819, 355420] | YES | 0.0417 | 0.0313 | 0 |
| an_b5_tree | 328100ns | no significant difference | [-9794, +6212]ns | [326061, 332087] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b5_table | an_b5_pred | an_b5_prof | an_b5_seq | an_b5_tree |
|---|---|---|---|---|---|
| 1 | 340816ns | +8.1% | +1.4% | +3.9% | -3.5% |
| 2 | 324776ns | +11.3% | +8.7% | +9.8% | +0.7% |
| 3 | 320685ns | +11.8% | +9.8% | +9.9% | +2.2% |
| 4 | 328606ns | +6.8% | +7.2% | +6.1% | -0.0% |
| 5 | 329776ns | +9.0% | +10.0% | +5.7% | +1.7% |
| 6 | 332680ns | +8.4% | +4.3% | +4.3% | -2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b5_pred | 0.165 | ok |
| an_b5_prof | -0.309 | moderate- |
| an_b5_seq | 0.516 | HIGH+ (drift/warm-up) |
| an_b5_table | -0.010 | ok |
| an_b5_tree | -0.393 | moderate- |

**Consistency summary:**

- **an_b5_pred**: won 0/6, lost 6/6
- **an_b5_prof**: won 0/6, lost 6/6
- **an_b5_seq**: won 0/6, lost 6/6
- **an_b5_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b5_pred | 9.0ns | 359885.8ns | 0.0% |  |
| an_b5_prof | 14.9ns | 352097.1ns | 0.0% |  |
| an_b5_seq | 19.9ns | 351289.3ns | 0.0% |  |
| an_b5_table | 8.0ns | 329556.6ns | 0.0% |  |
| an_b5_tree | 13.7ns | 328749.0ns | 0.0% |  |

## Distribution (algo ns)

```
an_b5_pred (n=6, range 351032.9-364849.5 ns)
  351032.9 |########################################
  351723.7 |
  352414.6 |
  353105.4 |
  353796.2 |
  354487.1 |
  355177.9 |
  355868.7 |
  356559.6 |
  357250.4 |
  357941.2 |
  358632.1 |########################################
  359322.9 |########################################
  360013.7 |########################################
  360704.6 |
  361395.4 |########################################
  362086.2 |
  362777.1 |
  363467.9 |
  364158.7 |
  (0 below, 1 above range)

an_b5_prof (n=6, range 345418.7-357870.6 ns)
  345418.7 |########################################
  346041.3 |
  346663.9 |########################################
  347286.5 |
  347909.1 |
  348531.7 |
  349154.3 |
  349776.9 |
  350399.5 |
  351022.1 |
  351644.7 |########################################
  352267.2 |########################################
  352889.8 |########################################
  353512.4 |
  354135.0 |
  354757.6 |
  355380.2 |
  356002.8 |
  356625.4 |
  357248.0 |
  (0 below, 1 above range)

an_b5_seq (n=6, range 347108.3-355420.0 ns)
  347108.3 |####################
  347523.9 |
  347939.5 |
  348355.1 |########################################
  348770.6 |
  349186.2 |
  349601.8 |
  350017.4 |
  350433.0 |
  350848.6 |
  351264.2 |
  351679.7 |
  352095.3 |
  352510.9 |####################
  352926.5 |
  353342.1 |
  353757.7 |####################
  354173.2 |
  354588.8 |
  355004.4 |
  (0 below, 1 above range)

an_b5_table (n=6, range 320685.0-336747.9 ns)
  320685.0 |########################################
  321488.1 |
  322291.3 |
  323094.4 |
  323897.6 |
  324700.7 |########################################
  325503.9 |
  326307.0 |
  327110.2 |
  327913.3 |########################################
  328716.5 |
  329519.6 |########################################
  330322.7 |
  331125.9 |
  331929.0 |########################################
  332732.2 |
  333535.3 |
  334338.5 |
  335141.6 |
  335944.8 |
  (0 below, 1 above range)

an_b5_tree (n=6, range 324999.6-332086.7 ns)
  324999.6 |########################################
  325354.0 |
  325708.3 |
  326062.7 |
  326417.0 |
  326771.4 |########################################
  327125.7 |
  327480.1 |########################################
  327834.4 |
  328188.8 |
  328543.1 |########################################
  328897.5 |########################################
  329251.9 |
  329606.2 |
  329960.6 |
  330314.9 |
  330669.3 |
  331023.6 |
  331378.0 |
  331732.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **an_b5_seq**: autocorrelation=0.52 (measurement drift or warm-up artifact)
