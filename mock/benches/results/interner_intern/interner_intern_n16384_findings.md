# Interner intern hot path: FNV vs FxHash x load factor 25% vs 75%

4 variants, 6 samples per variant.
Baseline: **intern_fnv_lf25**

## Highlights

Baseline for all deltas below: **intern_fnv_lf25**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (intern_fnv_lf75, intern_fnv_lf25) are a dead heat (<1%)

intern_fnv_lf75 (284.22 us) and intern_fnv_lf25 (284.78 us) differ by 0.20%, inside the noise, even though the wider field spreads 4.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### intern_fnv_lf75 shows alternating (throttle bounce) (autocorr -0.56)

intern_fnv_lf75's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 4.4% of the fastest

All 4 variants sit between 284.22 us and 296.71 us - a 4.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: intern_fnv_lf75** at 284221.5 ns median (-0.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 284221.5 ns, slowest 296712.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 286724ns | 287032ns | 276141ns | 285943ns | 293187ns | base |
| intern_fnv_lf75 | 285382ns | 286964ns | 280369ns | 285289ns | 288029ns | -0.47% |
| intern_fx_lf25 | 299052ns | 298967ns | 297422ns | 298842ns | 300183ns | +4.30% |
| intern_fx_lf75 | 294106ns | 293239ns | 289409ns | 292349ns | 299089ns | +2.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| intern_fnv_lf25 | 284474ns | 273957ns | 290890ns | base | 0.058 |
| intern_fnv_lf75 | 282828ns | 277763ns | 285534ns | -0.58% | 0.058 |
| intern_fx_lf25 | 296808ns | 295210ns | 297930ns | +4.34% | 0.055 |
| intern_fx_lf75 | 291737ns | 287140ns | 296713ns | +2.55% | 0.056 |

## Performance model

- Peak throughput: **0.060 Gops/s** (intern_fnv_lf25; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| intern_fnv_lf25 | 0.058 | 96.2% |
| intern_fnv_lf75 | 0.058 | 96.4% |
| intern_fx_lf25 | 0.055 | 92.3% |
| intern_fx_lf75 | 0.056 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| intern_fnv_lf25 | 286724ns | 286724ns | base |
| intern_fnv_lf75 | 285382ns | 285382ns | -0.47% |
| intern_fx_lf25 | 299052ns | 299052ns | +4.30% |
| intern_fx_lf75 | 294106ns | 294106ns | +2.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 284778ns | base | --- | [277754, 290890] | --- | --- | --- | --- |
| intern_fnv_lf75 | 284221ns | no significant difference | [-9970, +7500]ns | [278729, 285534] | no | 0.6875 | 0.6875 | 0 |
| intern_fx_lf25 | 296712ns | +12156.2ns (+4.3%) | [+5695, +19151]ns | [295781, 297930] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| intern_fx_lf75 | 290881ns | no significant difference | [-493, +15373]ns | [287616, 296713] | no | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | intern_fnv_lf25 | intern_fnv_lf75 | intern_fx_lf25 | intern_fx_lf75 |
|---|---|---|---|---|
| 1 | 288142ns | -1.1% | +3.2% | +2.1% |
| 2 | 287972ns | -1.4% | +2.9% | +1.6% |
| 3 | 281583ns | +1.0% | +6.1% | +2.7% |
| 4 | 293638ns | -5.4% | +1.0% | -1.9% |
| 5 | 273957ns | +4.4% | +7.8% | +4.8% |
| 6 | 281551ns | -0.7% | +5.4% | +6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| intern_fnv_lf25 | -0.377 | moderate- |
| intern_fnv_lf75 | -0.557 | HIGH- (thermal bounce) |
| intern_fx_lf25 | -0.161 | ok |
| intern_fx_lf75 | -0.078 | ok |

**Consistency summary:**

- **intern_fnv_lf75**: won 4/6, lost 2/6
- **intern_fx_lf25**: won 0/6, lost 6/6
- **intern_fx_lf75**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| intern_fnv_lf25 | 2725.2ns | 284473.8ns | 1.0% |  |
| intern_fnv_lf75 | 715.8ns | 282828.1ns | 0.3% |  |
| intern_fx_lf25 | 2716.6ns | 296807.9ns | 0.9% |  |
| intern_fx_lf75 | 716.3ns | 291736.6ns | 0.2% |  |

## Distribution (algo ns)

```
intern_fnv_lf25 (n=6, range 273957.1-290889.8 ns)
  273957.1 |####################
  274803.7 |
  275650.4 |
  276497.0 |
  277343.6 |
  278190.3 |
  279036.9 |
  279883.5 |
  280730.2 |####################
  281576.8 |####################
  282423.5 |
  283270.1 |
  284116.7 |
  284963.4 |
  285810.0 |
  286656.6 |
  287503.3 |########################################
  288349.9 |
  289196.5 |
  290043.2 |
  (0 below, 1 above range)

intern_fnv_lf75 (n=6, range 277762.9-285534.3 ns)
  277762.9 |########################################
  278151.5 |
  278540.0 |
  278928.6 |
  279317.2 |########################################
  279705.8 |
  280094.3 |
  280482.9 |
  280871.5 |
  281260.1 |
  281648.6 |
  282037.2 |
  282425.8 |
  282814.3 |
  283202.9 |
  283591.5 |########################################
  283980.1 |
  284368.6 |########################################
  284757.2 |########################################
  285145.8 |
  (0 below, 1 above range)

intern_fx_lf25 (n=6, range 295210.4-297930.4 ns)
  295210.4 |########################################
  295346.4 |
  295482.4 |
  295618.4 |
  295754.4 |
  295890.4 |
  296026.4 |
  296162.4 |
  296298.4 |########################################
  296434.4 |
  296570.4 |########################################
  296706.4 |########################################
  296842.4 |
  296978.4 |
  297114.4 |########################################
  297250.4 |
  297386.4 |
  297522.4 |
  297658.4 |
  297794.4 |
  (0 below, 1 above range)

intern_fx_lf75 (n=6, range 287139.6-296712.9 ns)
  287139.6 |########################################
  287618.3 |########################################
  288096.9 |
  288575.6 |
  289054.3 |########################################
  289532.9 |
  290011.6 |
  290490.3 |
  290968.9 |
  291447.6 |
  291926.2 |
  292404.9 |########################################
  292883.6 |
  293362.2 |
  293840.9 |########################################
  294319.6 |
  294798.2 |
  295276.9 |
  295755.6 |
  296234.2 |
  (0 below, 1 above range)

```
