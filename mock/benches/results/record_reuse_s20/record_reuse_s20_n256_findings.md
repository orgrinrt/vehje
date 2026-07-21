# Record update (20% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s20**

## Highlights

Baseline for all deltas below: **rec_reuse_s20**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 130% faster than the next best (rec_reuse_s20)

rec_mut (1.29 us) leads rec_reuse_s20 (2.97 us) by 130%, a clear separation rather than a photo finish. CV 5.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 57% (significant)

rec_mut is -1.68 us (57%) faster than baseline rec_reuse_s20, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 6.1x slower than the field

rec_copy (7.88 us) is 6.1x the fastest (1.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rec_mut is fastest but the noisiest (CV 5.4%)

rec_mut wins on median (1.29 us) yet has the highest variance (CV 5.4%), while rec_reuse_s20 is the steadiest (CV 3.6%, 2.97 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6.1x the fastest

Fastest rec_mut (1.29 us) to slowest rec_copy (7.88 us): 6.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 1291.1 ns median (-56.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.10x (fastest 1291.1 ns, slowest 7878.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 10366ns | 10370ns | 9489ns | 10355ns | 10820ns | +91.76% |
| rec_mut | 3683ns | 3745ns | 3366ns | 3660ns | 3877ns | -31.86% |
| rec_reuse_s20 | 5406ns | 5412ns | 5022ns | 5379ns | 5636ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 7869ns | 7205ns | 8221ns | +165.01% | 0.033 |
| rec_mut | 1268ns | 1159ns | 1335ns | -57.28% | 0.202 |
| rec_reuse_s20 | 2969ns | 2822ns | 3085ns | base | 0.086 |

## Performance model

- Peak throughput: **0.221 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.032 | 14.7% |
| rec_mut | 0.198 | 89.7% |
| rec_reuse_s20 | 0.086 | 39.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 10366ns | 10366ns | +91.76% |
| rec_mut | 3683ns | 3683ns | -31.86% |
| rec_reuse_s20 | 5406ns | 5406ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s20 | 2967ns | base | --- | [2856, 3085] | --- | --- | --- | --- |
| rec_copy | 7879ns | +4979.4ns (+167.8%) | [+4505, +5214]ns | [7505, 8221] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 1291ns | -1677.7ns (-56.5%) | [-1817, -1608]ns | [1180, 1335] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s20 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 2822ns | +176.6% | -57.5% |
| 2 | 3002ns | +180.7% | -54.2% |
| 3 | 3156ns | +150.1% | -59.0% |
| 4 | 2889ns | +172.2% | -55.1% |
| 5 | 3014ns | +166.0% | -57.3% |
| 6 | 2932ns | +145.7% | -60.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | -0.153 | ok |
| rec_mut | -0.172 | ok |
| rec_reuse_s20 | -0.282 | moderate- |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21798.2ns | 7868.6ns | 277.0% | HIGH |
| rec_mut | 20371.7ns | 1268.4ns | 1606.1% | HIGH |
| rec_reuse_s20 | 21659.2ns | 2969.2ns | 729.5% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 7205.0-8221.5 ns)
   7205.0 |########################################
   7255.8 |
   7306.6 |
   7357.5 |
   7408.3 |
   7459.1 |
   7509.9 |
   7560.8 |
   7611.6 |
   7662.4 |
   7713.2 |
   7764.0 |########################################
   7814.9 |########################################
   7865.7 |########################################
   7916.5 |
   7967.3 |########################################
   8018.2 |
   8069.0 |
   8119.8 |
   8170.6 |
  (0 below, 1 above range)

rec_mut (n=6, range 1158.7-1334.5 ns)
   1158.7 |####################
   1167.5 |
   1176.3 |
   1185.1 |
   1193.9 |####################
   1202.7 |
   1211.5 |
   1220.2 |
   1229.0 |
   1237.8 |
   1246.6 |
   1255.4 |
   1264.2 |
   1273.0 |
   1281.8 |####################
   1290.6 |########################################
   1299.4 |
   1308.2 |
   1317.0 |
   1325.8 |
  (0 below, 1 above range)

rec_reuse_s20 (n=6, range 2822.5-3084.8 ns)
   2822.5 |########################################
   2835.6 |
   2848.7 |
   2861.8 |
   2874.9 |
   2888.1 |########################################
   2901.2 |
   2914.3 |
   2927.4 |########################################
   2940.5 |
   2953.6 |
   2966.7 |
   2979.8 |
   2993.0 |########################################
   3006.1 |########################################
   3019.2 |
   3032.3 |
   3045.4 |
   3058.5 |
   3071.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=277.7% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=1595.9% of algo (FFI overhead may distort results)
- **rec_reuse_s20**: bridge=740.8% of algo (FFI overhead may distort results)
