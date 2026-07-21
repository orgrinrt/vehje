# Record update (20% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s20**

## Highlights

Baseline for all deltas below: **rec_reuse_s20**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 151% faster than the next best (rec_reuse_s20)

rec_mut (63.78 us) leads rec_reuse_s20 (159.98 us) by 151%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 58% (significant)

rec_mut is -93.20 us (58%) faster than baseline rec_reuse_s20, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 7.1x slower than the field

rec_copy (454.73 us) is 7.1x the fastest (63.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 7.1x the fastest

Fastest rec_mut (63.78 us) to slowest rec_copy (454.73 us): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 63782.1 ns median (-60.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.13x (fastest 63782.1 ns, slowest 454727.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 455654ns | 457033ns | 447600ns | 456825ns | 457924ns | +180.55% |
| rec_mut | 67758ns | 66040ns | 65065ns | 66011ns | 71726ns | -58.28% |
| rec_reuse_s20 | 162413ns | 162202ns | 155871ns | 160601ns | 168404ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 453354ns | 445364ns | 455631ns | +183.09% | 0.036 |
| rec_mut | 65450ns | 62857ns | 69272ns | -59.13% | 0.250 |
| rec_reuse_s20 | 160145ns | 153608ns | 166056ns | base | 0.102 |

## Performance model

- Peak throughput: **0.261 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.036 | 13.8% |
| rec_mut | 0.257 | 98.5% |
| rec_reuse_s20 | 0.102 | 39.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 455654ns | 455654ns | +180.55% |
| rec_mut | 67758ns | 67758ns | -58.28% |
| rec_reuse_s20 | 162413ns | 162413ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s20 | 159978ns | base | --- | [154402, 166056] | --- | --- | --- | --- |
| rec_copy | 454727ns | +294749.3ns (+184.2%) | [+284578, +300300]ns | [449704, 455631] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 63782ns | -93204.8ns (-58.3%) | [-99784, -91098]ns | [63294, 69272] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s20 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 158342ns | +187.4% | -59.8% |
| 2 | 161614ns | +181.1% | -56.8% |
| 3 | 166526ns | +173.8% | -58.7% |
| 4 | 153608ns | +195.6% | -59.1% |
| 5 | 155196ns | +193.4% | -58.9% |
| 6 | 165586ns | +169.0% | -61.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | -0.110 | ok |
| rec_mut | 0.122 | ok |
| rec_reuse_s20 | -0.207 | moderate- |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21426.8ns | 453354.4ns | 4.7% |  |
| rec_mut | 22053.4ns | 65449.5ns | 33.7% | HIGH |
| rec_reuse_s20 | 22771.2ns | 160145.3ns | 14.2% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 445364.2-455631.4 ns)
  445364.2 |####################
  445877.6 |
  446390.9 |
  446904.3 |
  447417.7 |
  447931.0 |
  448444.4 |
  448957.7 |
  449471.1 |
  449984.5 |
  450497.8 |
  451011.2 |
  451524.5 |
  452037.9 |
  452551.3 |
  453064.6 |
  453578.0 |####################
  454091.4 |####################
  454604.7 |
  455118.1 |########################################
  (0 below, 1 above range)

rec_mut (n=6, range 62857.1-69272.1 ns)
  62857.1 |#############
  63177.8 |
  63498.6 |########################################
  63819.3 |
  64140.1 |
  64460.8 |
  64781.6 |
  65102.3 |
  65423.1 |
  65743.9 |
  66064.6 |
  66385.4 |
  66706.1 |
  67026.9 |
  67347.6 |
  67668.4 |
  67989.1 |
  68309.9 |
  68630.6 |#############
  68951.4 |
  (0 below, 1 above range)

rec_reuse_s20 (n=6, range 153607.5-166056.2 ns)
  153607.5 |########################################
  154229.9 |
  154852.4 |########################################
  155474.8 |
  156097.2 |
  156719.7 |
  157342.1 |
  157964.5 |########################################
  158587.0 |
  159209.4 |
  159831.9 |
  160454.3 |
  161076.7 |########################################
  161699.2 |
  162321.6 |
  162944.0 |
  163566.5 |
  164188.9 |
  164811.3 |
  165433.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **rec_mut**: bridge=33.8% of algo (FFI overhead may distort results)
- **rec_reuse_s20**: bridge=14.1% of algo (FFI overhead may distort results)
