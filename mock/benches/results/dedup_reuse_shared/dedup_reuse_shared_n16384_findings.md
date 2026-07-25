# Dedup vs reuse composition, 60% shared: reuse's adversarial regime

4 variants, 6 samples per variant.
Baseline: **dru_s_plain**

## Highlights

Baseline for all deltas below: **dru_s_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_s_reuse dominates: 56% faster than the next best (dru_s_plain)

dru_s_reuse (438.57 us) leads dru_s_plain (685.55 us) by 56%, a clear separation rather than a photo finish. CV 9.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_s_reuse beats baseline by 36% (significant)

dru_s_reuse is -247.57 us (36%) faster than baseline dru_s_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_s_dedup is an outlier: 9.0x slower than the field

dru_s_dedup (3.97 ms) is 9.0x the fastest (438.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {dru_s_reuse, dru_s_plain} vs {dru_s_both, dru_s_dedup} (262% apart)

The field splits into a fast tier {dru_s_reuse, dru_s_plain} and a slow tier {dru_s_both, dru_s_dedup} with a 262% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.0x the fastest

Fastest dru_s_reuse (438.57 us) to slowest dru_s_dedup (3.97 ms): 9.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_s_reuse** at 438571.2 ns median (-36.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 9.04x (fastest 438571.2 ns, slowest 3966674.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_s_both | 2487606ns | 2483064ns | 2397990ns | 2460942ns | 2572410ns | +227.48% |
| dru_s_dedup | 4359415ns | 3970492ns | 3908705ns | 3964377ns | 5177326ns | +473.90% |
| dru_s_plain | 759615ns | 690183ns | 668000ns | 688911ns | 911479ns | base |
| dru_s_reuse | 456032ns | 442124ns | 418903ns | 439632ns | 499195ns | -39.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_s_both | 2483422ns | 2393998ns | 2567978ns | +229.20% | 0.007 |
| dru_s_dedup | 4355077ns | 3904290ns | 5171962ns | +477.30% | 0.004 |
| dru_s_plain | 754391ns | 663096ns | 905284ns | base | 0.022 |
| dru_s_reuse | 452366ns | 415724ns | 495083ns | -40.04% | 0.036 |

## Performance model

- Peak throughput: **0.039 Gops/s** (dru_s_reuse; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_s_both | 0.007 | 16.8% |
| dru_s_dedup | 0.004 | 10.5% |
| dru_s_plain | 0.024 | 60.6% |
| dru_s_reuse | 0.037 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_s_both | 2487606ns | 2487606ns | +227.48% |
| dru_s_dedup | 4359415ns | 4359415ns | +473.90% |
| dru_s_plain | 759615ns | 759615ns | base |
| dru_s_reuse | 456032ns | 456032ns | -39.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_s_plain | 685545ns | base | --- | [672342, 905284] | --- | --- | --- | --- |
| dru_s_both | 2478869ns | +1765877.7ns (+257.6%) | [+1606095, +1815122]ns | [2403420, 2567978] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_dedup | 3966674ns | +3287464.2ns (+479.5%) | [+3240729, +4273867]ns | [3926595, 5171962] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_reuse | 438571ns | -247574.8ns (-36.1%) | [-466726, -191772]ns | [423445, 495083] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_s_plain | dru_s_both | dru_s_dedup | dru_s_reuse |
|---|---|---|---|---|
| 1 | 684864ns | +268.7% | +503.8% | -35.9% |
| 2 | 1111327ns | +135.0% | +458.7% | -59.9% |
| 3 | 686227ns | +259.7% | +469.0% | -36.1% |
| 4 | 699242ns | +256.0% | +466.7% | -38.3% |
| 5 | 681588ns | +251.2% | +482.6% | -20.2% |
| 6 | 663096ns | +263.9% | +495.5% | -37.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_s_both | 0.289 | moderate+ |
| dru_s_dedup | -0.183 | ok |
| dru_s_plain | -0.226 | moderate- |
| dru_s_reuse | -0.455 | moderate- |

**Consistency summary:**

- **dru_s_both**: won 0/6, lost 6/6
- **dru_s_dedup**: won 0/6, lost 6/6
- **dru_s_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_s_both | 15425.0ns | 2483422.2ns | 0.6% |  |
| dru_s_dedup | 17385.1ns | 4355077.4ns | 0.4% |  |
| dru_s_plain | 18732.4ns | 754390.6ns | 2.5% |  |
| dru_s_reuse | 17751.4ns | 452366.2ns | 3.9% |  |

## Distribution (algo ns)

```
dru_s_both (n=6, range 2393997.5-2567977.7 ns)
  2393997.5 |########################################
  2402696.5 |
  2411395.5 |########################################
  2420094.5 |
  2428793.5 |
  2437492.5 |
  2446191.6 |
  2454890.6 |
  2463589.6 |########################################
  2472288.6 |
  2480987.6 |########################################
  2489686.6 |
  2498385.6 |
  2507084.6 |
  2515783.6 |
  2524482.7 |########################################
  2533181.7 |
  2541880.7 |
  2550579.7 |
  2559278.7 |
  (0 below, 1 above range)

dru_s_dedup (n=6, range 3904290.4-5171962.3 ns)
  3904290.4 |########################################
  3967674.0 |#############
  4031057.6 |
  4094441.2 |#############
  4157824.8 |
  4221208.4 |
  4284592.0 |
  4347975.6 |
  4411359.2 |
  4474742.8 |
  4538126.3 |
  4601509.9 |
  4664893.5 |
  4728277.1 |
  4791660.7 |
  4855044.3 |
  4918427.9 |
  4981811.5 |
  5045195.1 |
  5108578.7 |
  (0 below, 1 above range)

dru_s_plain (n=6, range 663095.8-905284.4 ns)
  663095.8 |#############
  675205.2 |########################################
  687314.7 |#############
  699424.1 |
  711533.5 |
  723643.0 |
  735752.4 |
  747861.8 |
  759971.2 |
  772080.7 |
  784190.1 |
  796299.5 |
  808409.0 |
  820518.4 |
  832627.8 |
  844737.2 |
  856846.7 |
  868956.1 |
  881065.5 |
  893175.0 |
  (0 below, 1 above range)

dru_s_reuse (n=6, range 415723.8-495082.7 ns)
  415723.8 |####################
  419691.7 |
  423659.7 |
  427627.6 |####################
  431595.6 |
  435563.5 |########################################
  439531.5 |
  443499.4 |####################
  447467.4 |
  451435.3 |
  455403.2 |
  459371.2 |
  463339.1 |
  467307.1 |
  471275.0 |
  475243.0 |
  479210.9 |
  483178.9 |
  487146.8 |
  491114.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **dru_s_plain**: CV=21.2% (high variance, measurements may be unstable)
