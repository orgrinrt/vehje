# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 dominates: 18% faster than the next best (iterfuse_mat2)

iterfuse_pull2 (183.47 us) leads iterfuse_mat2 (215.88 us) by 18%, a clear separation rather than a photo finish. CV 13.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_pull2 is fastest but the noisiest (CV 13.9%)

iterfuse_pull2 wins on median (183.47 us) yet has the highest variance (CV 13.9%), while iterfuse_push2 is the steadiest (CV 4.1%, 291.28 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iterfuse_push2 shows alternating (throttle bounce) (autocorr -0.60)

iterfuse_push2's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (183.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### iterfuse_pull2 is inconsistent: worst-20% is 1.6x its best-20%

iterfuse_pull2's best 20% of batches run at 132.36 us but its worst 20% at 206.35 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 183465.7 ns median
- 2 variants significantly slower than baseline
- Spread: 1.59x (fastest 183465.7 ns, slowest 291279.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 216688ns | 218038ns | 170062ns | 211401ns | 247931ns | +18.83% |
| iterfuse_pull2 | 182356ns | 185783ns | 134654ns | 180574ns | 208879ns | base |
| iterfuse_push2 | 299713ns | 293710ns | 289173ns | 292309ns | 316090ns | +64.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 214469ns | 167880ns | 245603ns | +19.17% | 0.029 |
| iterfuse_pull2 | 179969ns | 132361ns | 206353ns | base | 0.034 |
| iterfuse_push2 | 297205ns | 286933ns | 313384ns | +65.14% | 0.021 |

## Performance model

- Peak throughput: **0.046 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 6144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.028 | 61.3% |
| iterfuse_pull2 | 0.033 | 72.1% |
| iterfuse_push2 | 0.021 | 45.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 216688ns | 216688ns | +18.83% |
| iterfuse_pull2 | 182356ns | 182356ns | base |
| iterfuse_push2 | 299713ns | 299713ns | +64.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 183466ns | base | --- | [150088, 206353] | --- | --- | --- | --- |
| iterfuse_mat2 | 215876ns | +34741.4ns (+18.9%) | [+19859, +48901]ns | [181928, 245603] | YES | 0.0313 | 0.0313 | 0 |
| iterfuse_push2 | 291280ns | +117552.5ns (+64.1%) | [+80599, +153558]ns | [286952, 313384] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 201588ns | +5.7% | +42.4% |
| 2 | 167815ns | +16.8% | +89.8% |
| 3 | 132361ns | +26.8% | +118.2% |
| 4 | 182287ns | +33.9% | +61.2% |
| 5 | 184645ns | +18.4% | +67.0% |
| 6 | 211118ns | +17.1% | +35.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | -0.053 | ok |
| iterfuse_pull2 | 0.093 | ok |
| iterfuse_push2 | -0.598 | HIGH- (thermal bounce) |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 2082.8ns | 214469.3ns | 1.0% |  |
| iterfuse_pull2 | 3.6ns | 179968.8ns | 0.0% |  |
| iterfuse_push2 | 4.2ns | 297205.3ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 167880.4-245603.1 ns)
  167880.4 |########################################
  171766.5 |
  175652.7 |
  179538.8 |
  183424.9 |
  187311.1 |
  191197.2 |
  195083.3 |########################################
  198969.5 |
  202855.6 |
  206741.8 |
  210627.9 |########################################
  214514.0 |
  218400.2 |########################################
  222286.3 |
  226172.4 |
  230058.6 |
  233944.7 |
  237830.8 |
  241717.0 |########################################
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 132361.2-206352.9 ns)
  132361.2 |########################################
  136060.8 |
  139760.4 |
  143460.0 |
  147159.5 |
  150859.1 |
  154558.7 |
  158258.3 |
  161957.9 |
  165657.5 |########################################
  169357.0 |
  173056.6 |
  176756.2 |
  180455.8 |########################################
  184155.4 |########################################
  187855.0 |
  191554.6 |
  195254.1 |
  198953.7 |########################################
  202653.3 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 286933.3-313384.4 ns)
  286933.3 |########################################
  288255.9 |####################
  289578.4 |
  290901.0 |
  292223.5 |
  293546.1 |####################
  294868.6 |
  296191.2 |
  297513.7 |
  298836.3 |
  300158.8 |
  301481.4 |
  302804.0 |
  304126.5 |
  305449.1 |
  306771.6 |
  308094.2 |####################
  309416.7 |
  310739.3 |
  312061.8 |
  (0 below, 1 above range)

```
