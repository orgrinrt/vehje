# Dedup vs reuse composition, 20% shared: the templating norm

4 variants, 6 samples per variant.
Baseline: **dru_u_plain**

## Highlights

Baseline for all deltas below: **dru_u_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_u_reuse dominates: 192% faster than the next best (dru_u_plain)

dru_u_reuse (213.17 us) leads dru_u_plain (622.28 us) by 192%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_u_reuse beats baseline by 66% (significant)

dru_u_reuse is -408.94 us (66%) faster than baseline dru_u_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_u_dedup is an outlier: 18.4x slower than the field

dru_u_dedup (3.92 ms) is 18.4x the fastest (213.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dru_u_dedup shows alternating (throttle bounce) (autocorr -0.70)

dru_u_dedup's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {dru_u_reuse, dru_u_plain, dru_u_both} vs {dru_u_dedup} (361% apart)

The field splits into a fast tier {dru_u_reuse, dru_u_plain, dru_u_both} and a slow tier {dru_u_dedup} with a 361% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 18.4x the fastest

Fastest dru_u_reuse (213.17 us) to slowest dru_u_dedup (3.92 ms): 18.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_u_reuse** at 213170.4 ns median (-65.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 18.41x (fastest 213170.4 ns, slowest 3923716.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_u_both | 853268ns | 852980ns | 844601ns | 851278ns | 860585ns | +33.33% |
| dru_u_dedup | 3941868ns | 3927783ns | 3890982ns | 3925469ns | 3991910ns | +515.93% |
| dru_u_plain | 639982ns | 626934ns | 601811ns | 620880ns | 687721ns | base |
| dru_u_reuse | 216292ns | 215874ns | 211486ns | 215183ns | 220360ns | -66.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_u_both | 850523ns | 841767ns | 857623ns | +33.87% | 0.019 |
| dru_u_dedup | 3937745ns | 3886811ns | 3987546ns | +519.77% | 0.004 |
| dru_u_plain | 635352ns | 597108ns | 683164ns | base | 0.026 |
| dru_u_reuse | 213508ns | 208768ns | 217313ns | -66.40% | 0.077 |

## Performance model

- Peak throughput: **0.078 Gops/s** (dru_u_reuse; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_u_both | 0.019 | 24.5% |
| dru_u_dedup | 0.004 | 5.3% |
| dru_u_plain | 0.026 | 33.5% |
| dru_u_reuse | 0.077 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_u_both | 853268ns | 853268ns | +33.33% |
| dru_u_dedup | 3941868ns | 3941868ns | +515.93% |
| dru_u_plain | 639982ns | 639982ns | base |
| dru_u_reuse | 216292ns | 216292ns | -66.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_u_plain | 622285ns | base | --- | [600606, 683164] | --- | --- | --- | --- |
| dru_u_both | 850415ns | +227286.4ns (+36.5%) | [+172351, +245877]ns | [843532, 857623] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_dedup | 3923717ns | +3297075.5ns (+529.8%) | [+3263275, +3346830]ns | [3901972, 3987546] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_reuse | 213170ns | -408937.5ns (-65.7%) | [-470670, -385925]ns | [210039, 217313] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_u_plain | dru_u_both | dru_u_dedup | dru_u_reuse |
|---|---|---|---|---|
| 1 | 688998ns | +23.3% | +469.8% | -69.3% |
| 2 | 623685ns | +36.9% | +528.8% | -65.1% |
| 3 | 677330ns | +27.2% | +492.0% | -68.5% |
| 4 | 597108ns | +42.6% | +550.9% | -63.7% |
| 5 | 604104ns | +39.3% | +556.3% | -64.8% |
| 6 | 620885ns | +36.1% | +530.9% | -66.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_u_both | 0.343 | moderate+ |
| dru_u_dedup | -0.703 | HIGH- (thermal bounce) |
| dru_u_plain | -0.145 | ok |
| dru_u_reuse | -0.123 | ok |

**Consistency summary:**

- **dru_u_both**: won 0/6, lost 6/6
- **dru_u_dedup**: won 0/6, lost 6/6
- **dru_u_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_u_both | 14371.2ns | 850523.2ns | 1.7% |  |
| dru_u_dedup | 15532.9ns | 3937745.0ns | 0.4% |  |
| dru_u_plain | 17081.2ns | 635351.7ns | 2.7% |  |
| dru_u_reuse | 17101.0ns | 213507.6ns | 8.0% | HIGH |

## Distribution (algo ns)

```
dru_u_both (n=6, range 841766.7-857622.9 ns)
  841766.7 |########################################
  842559.5 |
  843352.3 |
  844145.1 |
  844937.9 |########################################
  845730.8 |
  846523.6 |
  847316.4 |
  848109.2 |
  848902.0 |########################################
  849694.8 |
  850487.6 |########################################
  851280.4 |
  852073.2 |
  852866.0 |
  853658.8 |########################################
  854451.7 |
  855244.5 |
  856037.3 |
  856830.1 |
  (0 below, 1 above range)

dru_u_dedup (n=6, range 3886811.2-3987546.5 ns)
  3886811.2 |####################
  3891848.0 |
  3896884.7 |
  3901921.5 |
  3906958.2 |
  3911995.0 |
  3917031.8 |########################################
  3922068.5 |####################
  3927105.3 |
  3932142.1 |
  3937178.8 |
  3942215.6 |
  3947252.4 |
  3952289.1 |
  3957325.9 |
  3962362.6 |####################
  3967399.4 |
  3972436.2 |
  3977472.9 |
  3982509.7 |
  (0 below, 1 above range)

dru_u_plain (n=6, range 597108.3-683163.9 ns)
  597108.3 |########################################
  601411.1 |########################################
  605713.9 |
  610016.6 |
  614319.4 |
  618622.2 |########################################
  622925.0 |########################################
  627227.8 |
  631530.6 |
  635833.3 |
  640136.1 |
  644438.9 |
  648741.7 |
  653044.5 |
  657347.3 |
  661650.0 |
  665952.8 |
  670255.6 |
  674558.4 |########################################
  678861.2 |
  (0 below, 1 above range)

dru_u_reuse (n=6, range 208767.9-217313.0 ns)
  208767.9 |########################################
  209195.2 |
  209622.4 |
  210049.7 |
  210476.9 |
  210904.2 |########################################
  211331.4 |
  211758.7 |
  212185.9 |
  212613.2 |########################################
  213040.4 |
  213467.7 |########################################
  213894.9 |
  214322.2 |
  214749.4 |
  215176.7 |
  215603.9 |
  216031.2 |
  216458.4 |########################################
  216885.7 |
  (0 below, 1 above range)

```
