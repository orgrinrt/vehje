# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 dominates: 11% faster than the next best (iterfuse_push2)

iterfuse_pull2 (788.54 us) leads iterfuse_push2 (875.24 us) by 11%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_pull2 is fastest but the noisiest (CV 7.1%)

iterfuse_pull2 wins on median (788.54 us) yet has the highest variance (CV 7.1%), while iterfuse_mat2 is the steadiest (CV 0.9%, 939.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iterfuse_mat2 shows warm-up / thermal drift (autocorr +0.51)

iterfuse_mat2's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (788.54 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 788537.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.19x (fastest 788537.5 ns, slowest 939155.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 943177ns | 941389ns | 934243ns | 939273ns | 953499ns | +18.35% |
| iterfuse_pull2 | 796940ns | 791522ns | 724871ns | 782818ns | 854156ns | base |
| iterfuse_push2 | 877702ns | 878469ns | 862335ns | 873404ns | 891831ns | +10.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 940910ns | 931992ns | 951220ns | +18.51% | 0.017 |
| iterfuse_pull2 | 793980ns | 722501ns | 850878ns | base | 0.021 |
| iterfuse_push2 | 874873ns | 859324ns | 889461ns | +10.19% | 0.019 |

## Performance model

- Peak throughput: **0.023 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.017 | 76.9% |
| iterfuse_pull2 | 0.021 | 91.6% |
| iterfuse_push2 | 0.019 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 943177ns | 943177ns | +18.35% |
| iterfuse_pull2 | 796940ns | 796940ns | base |
| iterfuse_push2 | 877702ns | 877702ns | +10.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 788538ns | base | --- | [742525, 850878] | --- | --- | --- | --- |
| iterfuse_mat2 | 939156ns | +159658.8ns (+20.2%) | [+82619, +198511]ns | [932354, 951220] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| iterfuse_push2 | 875237ns | +81802.5ns (+10.4%) | [+13940, +146936]ns | [859921, 889461] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 907178ns | +2.7% | -5.3% |
| 2 | 792291ns | +17.7% | +9.9% |
| 3 | 762549ns | +22.4% | +16.0% |
| 4 | 722501ns | +31.3% | +23.8% |
| 5 | 784784ns | +21.5% | +9.7% |
| 6 | 794579ns | +19.0% | +10.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | 0.511 | HIGH+ (drift/warm-up) |
| iterfuse_pull2 | 0.145 | ok |
| iterfuse_push2 | -0.140 | ok |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 4060.5ns | 940909.8ns | 0.4% |  |
| iterfuse_pull2 | 8.4ns | 793980.1ns | 0.0% |  |
| iterfuse_push2 | 5.9ns | 874873.0ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 931992.5-951220.2 ns)
  931992.5 |########################################
  932953.9 |####################
  933915.3 |
  934876.7 |
  935838.1 |
  936799.4 |
  937760.8 |
  938722.2 |
  939683.6 |
  940645.0 |
  941606.4 |
  942567.8 |
  943529.2 |
  944490.5 |####################
  945451.9 |
  946413.3 |
  947374.7 |
  948336.1 |####################
  949297.5 |
  950258.9 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 722500.8-850878.1 ns)
  722500.8 |########################################
  728919.7 |
  735338.5 |
  741757.4 |
  748176.3 |
  754595.1 |
  761014.0 |########################################
  767432.9 |
  773851.7 |
  780270.6 |########################################
  786689.4 |########################################
  793108.3 |########################################
  799527.2 |
  805946.0 |
  812364.9 |
  818783.8 |
  825202.6 |
  831621.5 |
  838040.4 |
  844459.2 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 859323.8-889460.6 ns)
  859323.8 |########################################
  860830.6 |
  862337.5 |
  863844.3 |
  865351.2 |
  866858.0 |
  868364.8 |
  869871.7 |####################
  871378.5 |
  872885.4 |
  874392.2 |
  875899.0 |
  877405.9 |
  878912.7 |####################
  880419.6 |
  881926.4 |
  883433.2 |####################
  884940.1 |
  886446.9 |
  887953.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **iterfuse_mat2**: autocorrelation=0.51 (measurement drift or warm-up artifact)
