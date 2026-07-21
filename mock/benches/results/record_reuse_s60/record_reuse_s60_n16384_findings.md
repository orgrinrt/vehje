# Record update (60% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s60**

## Highlights

Baseline for all deltas below: **rec_reuse_s60**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 383% faster than the next best (rec_reuse_s60)

rec_mut (65.59 us) leads rec_reuse_s60 (317.12 us) by 383%, a clear separation rather than a photo finish. CV 4.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 79% (significant)

rec_mut is -250.29 us (79%) faster than baseline rec_reuse_s60, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 6.9x slower than the field

rec_copy (450.60 us) is 6.9x the fastest (65.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 6.9x the fastest

Fastest rec_mut (65.59 us) to slowest rec_copy (450.60 us): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 65591.7 ns median (-79.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.87x (fastest 65591.7 ns, slowest 450595.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 452713ns | 452846ns | 446280ns | 451058ns | 458412ns | +42.34% |
| rec_mut | 68374ns | 67917ns | 64750ns | 66961ns | 72306ns | -78.50% |
| rec_reuse_s60 | 318055ns | 319456ns | 312450ns | 318588ns | 320059ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 450409ns | 443958ns | 456047ns | +42.63% | 0.036 |
| rec_mut | 66024ns | 62391ns | 69862ns | -79.09% | 0.248 |
| rec_reuse_s60 | 315782ns | 310225ns | 317789ns | base | 0.052 |

## Performance model

- Peak throughput: **0.263 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.036 | 13.8% |
| rec_mut | 0.250 | 95.1% |
| rec_reuse_s60 | 0.052 | 19.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 452713ns | 452713ns | +42.34% |
| rec_mut | 68374ns | 68374ns | -78.50% |
| rec_reuse_s60 | 318055ns | 318055ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s60 | 317122ns | base | --- | [312435, 317789] | --- | --- | --- | --- |
| rec_copy | 450595ns | +133473.1ns (+42.1%) | [+128670, +141739]ns | [444585, 456047] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 65592ns | -250293.8ns (-78.9%) | [-255137, -243843]ns | [62618, 69862] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s60 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 317118ns | +41.9% | -80.3% |
| 2 | 314644ns | +41.5% | -79.7% |
| 3 | 317186ns | +40.0% | -77.9% |
| 4 | 317127ns | +42.3% | -78.8% |
| 5 | 318392ns | +44.1% | -80.3% |
| 6 | 310225ns | +46.1% | -77.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | 0.407 | moderate+ |
| rec_mut | -0.194 | ok |
| rec_reuse_s60 | -0.274 | moderate- |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21581.9ns | 450409.2ns | 4.8% |  |
| rec_mut | 22243.8ns | 66023.8ns | 33.7% | HIGH |
| rec_reuse_s60 | 22519.6ns | 315781.9ns | 7.1% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 443958.3-456046.9 ns)
  443958.3 |########################################
  444562.7 |
  445167.2 |########################################
  445771.6 |
  446376.0 |
  446980.5 |
  447584.9 |
  448189.3 |
  448793.7 |
  449398.2 |########################################
  450002.6 |
  450607.0 |
  451211.5 |########################################
  451815.9 |
  452420.3 |
  453024.8 |########################################
  453629.2 |
  454233.6 |
  454838.0 |
  455442.5 |
  (0 below, 1 above range)

rec_mut (n=6, range 62390.8-69862.3 ns)
  62390.8 |########################################
  62764.4 |########################################
  63138.0 |
  63511.5 |
  63885.1 |########################################
  64258.7 |
  64632.2 |
  65005.8 |
  65379.4 |
  65753.0 |
  66126.5 |
  66500.1 |
  66873.7 |
  67247.3 |########################################
  67620.8 |
  67994.4 |
  68368.0 |
  68741.6 |
  69115.1 |
  69488.7 |########################################
  (0 below, 1 above range)

rec_reuse_s60 (n=6, range 310225.0-317788.8 ns)
  310225.0 |#############
  310603.2 |
  310981.4 |
  311359.6 |
  311737.8 |
  312115.9 |
  312494.1 |
  312872.3 |
  313250.5 |
  313628.7 |
  314006.9 |
  314385.1 |#############
  314763.2 |
  315141.4 |
  315519.6 |
  315897.8 |
  316276.0 |
  316654.2 |
  317032.4 |########################################
  317410.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_mut**: bridge=33.6% of algo (FFI overhead may distort results)
