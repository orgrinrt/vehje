# Record update (20% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s20**

## Highlights

Baseline for all deltas below: **rec_reuse_s20**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 144% faster than the next best (rec_reuse_s20)

rec_mut (4.52 us) leads rec_reuse_s20 (11.02 us) by 144%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 59% (significant)

rec_mut is -6.53 us (59%) faster than baseline rec_reuse_s20, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 7.0x slower than the field

rec_copy (31.68 us) is 7.0x the fastest (4.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 7.0x the fastest

Fastest rec_mut (4.52 us) to slowest rec_copy (31.68 us): 7.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 4519.4 ns median (-59.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.01x (fastest 4519.4 ns, slowest 31682.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 33992ns | 34144ns | 32351ns | 33958ns | 34865ns | +156.71% |
| rec_mut | 6824ns | 6891ns | 6330ns | 6810ns | 7091ns | -48.47% |
| rec_reuse_s20 | 13242ns | 13492ns | 12151ns | 13142ns | 13937ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 31523ns | 29894ns | 32355ns | +190.30% | 0.032 |
| rec_mut | 4465ns | 4150ns | 4629ns | -58.88% | 0.229 |
| rec_reuse_s20 | 10858ns | 9982ns | 11437ns | base | 0.094 |

## Performance model

- Peak throughput: **0.247 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.032 | 13.1% |
| rec_mut | 0.227 | 91.8% |
| rec_reuse_s20 | 0.093 | 37.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 33992ns | 33992ns | +156.71% |
| rec_mut | 6824ns | 6824ns | -48.47% |
| rec_reuse_s20 | 13242ns | 13242ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s20 | 11020ns | base | --- | [10119, 11437] | --- | --- | --- | --- |
| rec_copy | 31683ns | +20344.8ns (+184.6%) | [+19696, +21952]ns | [30530, 32355] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 4519ns | -6531.2ns (-59.3%) | [-6853, -5796]ns | [4246, 4629] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s20 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 9982ns | +217.3% | -58.4% |
| 2 | 11462ns | +176.6% | -59.7% |
| 3 | 10255ns | +191.5% | -56.2% |
| 4 | 10795ns | +205.8% | -59.8% |
| 5 | 11244ns | +181.8% | -58.8% |
| 6 | 11413ns | +173.1% | -60.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | -0.493 | moderate- |
| rec_mut | -0.321 | moderate- |
| rec_reuse_s20 | -0.340 | moderate- |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 22932.0ns | 31522.6ns | 72.7% | HIGH |
| rec_mut | 20319.2ns | 4465.0ns | 455.1% | HIGH |
| rec_reuse_s20 | 22635.4ns | 10858.5ns | 208.5% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 29894.2-32355.2 ns)
  29894.2 |#############
  30017.2 |
  30140.3 |
  30263.4 |
  30386.4 |
  30509.5 |
  30632.5 |
  30755.5 |
  30878.6 |
  31001.7 |
  31124.7 |#############
  31247.8 |
  31370.8 |
  31493.9 |
  31616.9 |########################################
  31740.0 |
  31863.0 |
  31986.0 |
  32109.1 |
  32232.2 |
  (0 below, 1 above range)

rec_mut (n=6, range 4150.4-4629.2 ns)
   4150.4 |########################################
   4174.3 |
   4198.3 |
   4222.2 |
   4246.2 |
   4270.1 |
   4294.0 |
   4318.0 |
   4341.9 |########################################
   4365.9 |
   4389.8 |
   4413.7 |
   4437.7 |
   4461.6 |
   4485.6 |########################################
   4509.5 |
   4533.4 |########################################
   4557.4 |
   4581.3 |
   4605.3 |########################################
  (0 below, 1 above range)

rec_reuse_s20 (n=6, range 9981.7-11437.3 ns)
   9981.7 |########################################
  10054.5 |
  10127.3 |
  10200.0 |########################################
  10272.8 |
  10345.6 |
  10418.4 |
  10491.2 |
  10563.9 |
  10636.7 |
  10709.5 |
  10782.3 |########################################
  10855.1 |
  10927.8 |
  11000.6 |
  11073.4 |
  11146.2 |
  11219.0 |########################################
  11291.7 |
  11364.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=72.8% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=453.9% of algo (FFI overhead may distort results)
- **rec_reuse_s20**: bridge=212.1% of algo (FFI overhead may distort results)
