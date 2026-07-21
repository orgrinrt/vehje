# Match lowering: if-chain vs jump-table vs decision-tree, K=2 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u2**

## Highlights

Baseline for all deltas below: **ml_jumptable_u2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ml_jumptable_u2, ml_ifchain_u2) are a dead heat (<1%)

ml_jumptable_u2 (258.56 us) and ml_ifchain_u2 (259.33 us) differ by 0.30%, inside the noise, even though the wider field spreads 30.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ml_jumptable_u2 shows alternating (throttle bounce) (autocorr -0.51)

ml_jumptable_u2's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ml_jumptable_u2)

The baseline ml_jumptable_u2 is the fastest (258.56 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ml_jumptable_u2) is the fastest** at 258562.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.30x (fastest 258562.1 ns, slowest 337251.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u2 | 265982ns | 262106ns | 260058ns | 261547ns | 275597ns | +1.38% |
| ml_jumptable_u2 | 262364ns | 261139ns | 256744ns | 259947ns | 268799ns | base |
| ml_tree_u2 | 339948ns | 339733ns | 339382ns | 339679ns | 340636ns | +29.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u2 | 263349ns | 257742ns | 272788ns | +1.36% | 0.016 |
| ml_jumptable_u2 | 259816ns | 253968ns | 266258ns | base | 0.016 |
| ml_tree_u2 | 337321ns | 336869ns | 337819ns | +29.83% | 0.012 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_jumptable_u2; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u2 | 0.016 | 97.9% |
| ml_jumptable_u2 | 0.016 | 98.2% |
| ml_tree_u2 | 0.012 | 75.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u2 | 265982ns | 265982ns | +1.38% |
| ml_jumptable_u2 | 262364ns | 262364ns | base |
| ml_tree_u2 | 339948ns | 339948ns | +29.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u2 | 258562ns | base | --- | [254629, 266258] | --- | --- | --- | --- |
| ml_ifchain_u2 | 259333ns | no significant difference | [-5802, +13447]ns | [257927, 272788] | no | 0.6875 | 0.6875 | 0 |
| ml_tree_u2 | 337252ns | +78602.3ns (+30.4%) | [+71288, +82623]ns | [336892, 337819] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u2 | ml_ifchain_u2 | ml_tree_u2 |
|---|---|---|---|
| 1 | 255289ns | +1.1% | +32.1% |
| 2 | 267801ns | -3.8% | +26.3% |
| 3 | 253968ns | +6.0% | +32.8% |
| 4 | 256775ns | +1.2% | +31.4% |
| 5 | 264714ns | +4.4% | +27.3% |
| 6 | 260350ns | -0.6% | +29.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u2 | -0.433 | moderate- |
| ml_jumptable_u2 | -0.509 | HIGH- (thermal bounce) |
| ml_tree_u2 | 0.010 | ok |

**Consistency summary:**

- **ml_ifchain_u2**: won 2/6, lost 4/6
- **ml_tree_u2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u2 | 10.3ns | 263349.1ns | 0.0% |  |
| ml_jumptable_u2 | 5.5ns | 259816.2ns | 0.0% |  |
| ml_tree_u2 | 6.7ns | 337320.6ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u2 (n=6, range 257742.5-272787.9 ns)
  257742.5 |########################################
  258494.8 |####################
  259247.0 |####################
  259999.3 |
  260751.6 |
  261503.9 |
  262256.1 |
  263008.4 |
  263760.7 |
  264512.9 |
  265265.2 |
  266017.5 |
  266769.7 |
  267522.0 |
  268274.3 |
  269026.6 |####################
  269778.8 |
  270531.1 |
  271283.4 |
  272035.6 |
  (0 below, 1 above range)

ml_jumptable_u2 (n=6, range 253968.3-266257.7 ns)
  253968.3 |########################################
  254582.8 |
  255197.2 |########################################
  255811.7 |
  256426.2 |########################################
  257040.6 |
  257655.1 |
  258269.6 |
  258884.1 |
  259498.5 |
  260113.0 |########################################
  260727.5 |
  261341.9 |
  261956.4 |
  262570.9 |
  263185.3 |
  263799.8 |
  264414.3 |########################################
  265028.8 |
  265643.2 |
  (0 below, 1 above range)

ml_tree_u2 (n=6, range 336869.2-337818.6 ns)
  336869.2 |########################################
  336916.7 |
  336964.1 |
  337011.6 |
  337059.1 |
  337106.5 |
  337154.0 |
  337201.5 |####################
  337248.9 |####################
  337296.4 |
  337343.9 |
  337391.3 |####################
  337438.8 |
  337486.3 |
  337533.7 |
  337581.2 |
  337628.7 |
  337676.1 |
  337723.6 |
  337771.1 |
  (0 below, 1 above range)

```
