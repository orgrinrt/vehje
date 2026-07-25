# Dedup vs reuse composition, 20% shared: the templating norm

4 variants, 6 samples per variant.
Baseline: **dru_u_plain**

## Highlights

Baseline for all deltas below: **dru_u_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_u_reuse dominates: 253% faster than the next best (dru_u_plain)

dru_u_reuse (2.50 us) leads dru_u_plain (8.83 us) by 253%, a clear separation rather than a photo finish. CV 58.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_u_reuse beats baseline by 72% (significant)

dru_u_reuse is -6.36 us (72%) faster than baseline dru_u_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_u_dedup is an outlier: 22.8x slower than the field

dru_u_dedup (57.17 us) is 22.8x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {dru_u_reuse, dru_u_plain, dru_u_both} vs {dru_u_dedup} (464% apart)

The field splits into a fast tier {dru_u_reuse, dru_u_plain, dru_u_both} and a slow tier {dru_u_dedup} with a 464% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 22.8x the fastest

Fastest dru_u_reuse (2.50 us) to slowest dru_u_dedup (57.17 us): 22.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dru_u_plain is inconsistent: worst-20% is 2.3x its best-20%

dru_u_plain's best 20% of batches run at 8.55 us but its worst 20% at 19.60 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dru_u_reuse** at 2502.9 ns median (-71.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 22.84x (fastest 2502.9 ns, slowest 57168.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_u_both | 14931ns | 12486ns | 11342ns | 12234ns | 20772ns | -1.78% |
| dru_u_dedup | 68108ns | 59676ns | 54469ns | 58048ns | 90019ns | +348.04% |
| dru_u_plain | 15201ns | 11194ns | 10844ns | 11098ns | 23535ns | base |
| dru_u_reuse | 6351ns | 4874ns | 4541ns | 4782ns | 9611ns | -58.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_u_both | 12411ns | 9095ns | 17811ns | +0.59% | 0.021 |
| dru_u_dedup | 65187ns | 52215ns | 86085ns | +428.31% | 0.004 |
| dru_u_plain | 12339ns | 8553ns | 19597ns | base | 0.021 |
| dru_u_reuse | 3431ns | 2382ns | 5405ns | -72.19% | 0.075 |

## Performance model

- Peak throughput: **0.107 Gops/s** (dru_u_reuse; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_u_both | 0.025 | 23.5% |
| dru_u_dedup | 0.004 | 4.2% |
| dru_u_plain | 0.029 | 27.0% |
| dru_u_reuse | 0.102 | 95.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_u_both | 14931ns | 14931ns | -1.78% |
| dru_u_dedup | 68108ns | 68108ns | +348.04% |
| dru_u_plain | 15201ns | 15201ns | base |
| dru_u_reuse | 6351ns | 6351ns | -58.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_u_plain | 8827ns | base | --- | [8592, 19597] | --- | --- | --- | --- |
| dru_u_both | 10137ns | no significant difference | [-4413, +3259]ns | [9286, 17811] | no | 0.2188 | 0.2188 | 0 |
| dru_u_dedup | 57169ns | +43850.6ns (+496.8%) | [+43162, +71533]ns | [52308, 86085] | YES | 0.0469 | 0.0313 | 0 |
| dru_u_reuse | 2503ns | -6356.6ns (-72.0%) | [-14192, -6173]ns | [2386, 5405] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_u_plain | dru_u_both | dru_u_dedup | dru_u_reuse |
|---|---|---|---|---|
| 1 | 8553ns | +22.2% | +510.5% | -71.3% |
| 2 | 8668ns | +4.9% | +504.5% | -72.4% |
| 3 | 8632ns | +9.8% | +509.4% | -72.4% |
| 4 | 8985ns | +33.9% | +662.5% | -71.6% |
| 5 | 19076ns | -48.5% | +223.6% | -75.5% |
| 6 | 20118ns | +17.3% | +415.3% | -69.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_u_both | -0.068 | ok |
| dru_u_dedup | 0.071 | ok |
| dru_u_plain | 0.440 | moderate+ |
| dru_u_reuse | 0.415 | moderate+ |

**Consistency summary:**

- **dru_u_both**: won 1/6, lost 5/6
- **dru_u_dedup**: won 0/6, lost 6/6
- **dru_u_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_u_both | 18082.1ns | 12411.4ns | 145.7% | HIGH |
| dru_u_dedup | 18783.8ns | 65187.2ns | 28.8% | HIGH |
| dru_u_plain | 21925.0ns | 12338.8ns | 177.7% | HIGH |
| dru_u_reuse | 20832.0ns | 3431.4ns | 607.1% | HIGH |

## Distribution (algo ns)

```
dru_u_both (n=6, range 9095.0-17810.7 ns)
   9095.0 |########################################
   9530.8 |####################
   9966.6 |
  10402.3 |####################
  10838.1 |
  11273.9 |
  11709.7 |####################
  12145.5 |
  12581.3 |
  13017.0 |
  13452.8 |
  13888.6 |
  14324.4 |
  14760.2 |
  15196.0 |
  15631.7 |
  16067.5 |
  16503.3 |
  16939.1 |
  17374.9 |
  (0 below, 1 above range)

dru_u_dedup (n=6, range 52215.0-86084.5 ns)
  52215.0 |########################################
  53908.5 |
  55602.0 |
  57295.4 |
  58988.9 |
  60682.4 |#############
  62375.9 |
  64069.3 |
  65762.8 |
  67456.3 |#############
  69149.8 |
  70843.3 |
  72536.7 |
  74230.2 |
  75923.7 |
  77617.2 |
  79310.6 |
  81004.1 |
  82697.6 |
  84391.1 |
  (0 below, 1 above range)

dru_u_plain (n=6, range 8553.3-19597.1 ns)
   8553.3 |########################################
   9105.5 |
   9657.7 |
  10209.9 |
  10762.0 |
  11314.2 |
  11866.4 |
  12418.6 |
  12970.8 |
  13523.0 |
  14075.2 |
  14627.4 |
  15179.6 |
  15731.7 |
  16283.9 |
  16836.1 |
  17388.3 |
  17940.5 |
  18492.7 |
  19044.9 |##########
  (0 below, 1 above range)

dru_u_reuse (n=6, range 2381.7-5404.8 ns)
   2381.7 |########################################
   2532.9 |#############
   2684.0 |
   2835.2 |
   2986.3 |
   3137.5 |
   3288.6 |
   3439.8 |
   3590.9 |
   3742.1 |
   3893.2 |
   4044.4 |
   4195.6 |
   4346.7 |
   4497.9 |
   4649.0 |#############
   4800.2 |
   4951.3 |
   5102.5 |
   5253.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **dru_u_both**: CV=41.0% (high variance, measurements may be unstable)
- **dru_u_both**: bridge=173.1% of algo (FFI overhead may distort results)
- **dru_u_dedup**: CV=28.0% (high variance, measurements may be unstable)
- **dru_u_dedup**: bridge=28.5% of algo (FFI overhead may distort results)
- **dru_u_plain**: CV=41.7% (high variance, measurements may be unstable)
- **dru_u_plain**: bridge=196.8% of algo (FFI overhead may distort results)
- **dru_u_reuse**: CV=42.5% (high variance, measurements may be unstable)
- **dru_u_reuse**: bridge=685.4% of algo (FFI overhead may distort results)
