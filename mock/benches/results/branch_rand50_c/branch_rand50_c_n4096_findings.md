# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Highlights

Baseline for all deltas below: **br_branch_c_rand50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (br_branch_c_rand50) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline br_branch_c_rand50 has the worst median (14.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest br_lut_c_rand50 at 8.99 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### br_predicate_c_rand50 beats baseline by 40% (significant)

br_predicate_c_rand50 is -5.87 us (40%) faster than baseline br_branch_c_rand50, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### br_lut_c_rand50 is fastest but the noisiest (CV 7.3%)

br_lut_c_rand50 wins on median (8.99 us) yet has the highest variance (CV 7.3%), while br_branch_c_rand50 is the steadiest (CV 6.1%, 14.50 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (br_lut_c_rand50, br_predicate_c_rand50) are a dead heat (<1%)

br_lut_c_rand50 (8.99 us) and br_predicate_c_rand50 (9.00 us) differ by 0.14%, inside the noise, even though the wider field spreads 61.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {br_lut_c_rand50, br_predicate_c_rand50, br_mask_c_rand50} vs {br_branch_c_rand50} (42% apart)

The field splits into a fast tier {br_lut_c_rand50, br_predicate_c_rand50, br_mask_c_rand50} and a slow tier {br_branch_c_rand50} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: br_lut_c_rand50** at 8991.2 ns median (-38.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.61x (fastest 8991.2 ns, slowest 14502.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 17068ns | 16951ns | 15719ns | 16770ns | 18187ns | base |
| br_lut_c_rand50 | 11322ns | 11565ns | 9818ns | 11317ns | 12083ns | -33.66% |
| br_mask_c_rand50 | 12478ns | 12805ns | 10764ns | 12559ns | 13214ns | -26.89% |
| br_predicate_c_rand50 | 11170ns | 11605ns | 10030ns | 11086ns | 11865ns | -34.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 14685ns | 13562ns | 15694ns | base | 0.279 |
| br_lut_c_rand50 | 8792ns | 7608ns | 9376ns | -40.13% | 0.466 |
| br_mask_c_rand50 | 9908ns | 8600ns | 10402ns | -32.53% | 0.413 |
| br_predicate_c_rand50 | 8664ns | 7793ns | 9190ns | -41.00% | 0.473 |

## Performance model

- Peak throughput: **0.538 Gops/s** (br_lut_c_rand50; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.282 | 52.5% |
| br_lut_c_rand50 | 0.456 | 84.6% |
| br_mask_c_rand50 | 0.400 | 74.3% |
| br_predicate_c_rand50 | 0.455 | 84.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 17068ns | 17068ns | base |
| br_lut_c_rand50 | 11322ns | 11322ns | -33.66% |
| br_mask_c_rand50 | 12478ns | 12478ns | -26.89% |
| br_predicate_c_rand50 | 11170ns | 11170ns | -34.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 14503ns | base | --- | [13857, 15694] | --- | --- | --- | --- |
| br_lut_c_rand50 | 8991ns | -5849.6ns (-40.3%) | [-6702, -5126]ns | [8008, 9376] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 10232ns | -4689.3ns (-32.3%) | [-5371, -4270]ns | [9090, 10402] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 9004ns | -5871.8ns (-40.5%) | [-6696, -5494]ns | [7799, 9190] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 13562ns | -43.9% | -36.6% | -42.5% |
| 2 | 14223ns | -36.8% | -28.0% | -36.8% |
| 3 | 14152ns | -40.6% | -32.3% | -44.9% |
| 4 | 16349ns | -45.0% | -35.3% | -43.0% |
| 5 | 14782ns | -34.0% | -30.8% | -39.0% |
| 6 | 15038ns | -40.2% | -32.0% | -39.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | 0.016 | ok |
| br_lut_c_rand50 | -0.003 | ok |
| br_mask_c_rand50 | -0.168 | ok |
| br_predicate_c_rand50 | -0.333 | moderate- |

**Consistency summary:**

- **br_lut_c_rand50**: won 6/6, lost 0/6
- **br_mask_c_rand50**: won 6/6, lost 0/6
- **br_predicate_c_rand50**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 4.0ns | 14684.5ns | 0.0% |  |
| br_lut_c_rand50 | 3.8ns | 8791.7ns | 0.0% |  |
| br_mask_c_rand50 | 3.8ns | 9907.9ns | 0.0% |  |
| br_predicate_c_rand50 | 3.4ns | 8664.2ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 13562.1-15693.5 ns)
  13562.1 |########################################
  13668.7 |
  13775.2 |
  13881.8 |
  13988.4 |
  14095.0 |########################################
  14201.5 |########################################
  14308.1 |
  14414.7 |
  14521.3 |
  14627.8 |
  14734.4 |########################################
  14841.0 |
  14947.5 |########################################
  15054.1 |
  15160.7 |
  15267.3 |
  15373.8 |
  15480.4 |
  15587.0 |
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 7607.5-9376.2 ns)
   7607.5 |#############
   7695.9 |
   7784.4 |
   7872.8 |
   7961.2 |
   8049.7 |
   8138.1 |
   8226.6 |
   8315.0 |
   8403.4 |#############
   8491.9 |
   8580.3 |
   8668.8 |
   8757.2 |
   8845.6 |
   8934.1 |########################################
   9022.5 |
   9110.9 |
   9199.4 |
   9287.8 |
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 8599.6-10401.9 ns)
   8599.6 |#############
   8689.7 |
   8779.8 |
   8869.9 |
   8960.1 |
   9050.2 |
   9140.3 |
   9230.4 |
   9320.5 |
   9410.6 |
   9500.8 |#############
   9590.9 |
   9681.0 |
   9771.1 |
   9861.2 |
   9951.3 |
  10041.4 |
  10131.6 |
  10221.7 |########################################
  10311.8 |
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 7793.3-9189.6 ns)
   7793.3 |########################################
   7863.1 |
   7932.9 |
   8002.7 |
   8072.6 |
   8142.4 |
   8212.2 |
   8282.0 |
   8351.8 |
   8421.6 |
   8491.5 |
   8561.3 |
   8631.1 |
   8700.9 |
   8770.7 |
   8840.5 |
   8910.3 |
   8980.2 |########################################
   9050.0 |####################
   9119.8 |
  (0 below, 1 above range)

```
