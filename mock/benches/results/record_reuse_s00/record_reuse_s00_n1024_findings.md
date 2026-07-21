# Record update (0% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s00**

## Highlights

Baseline for all deltas below: **rec_reuse_s00**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_copy is an outlier: 6.5x slower than the field

rec_copy (28.86 us) is 6.5x the fastest (4.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rec_mut is fastest but the noisiest (CV 5.7%)

rec_mut wins on median (4.41 us) yet has the highest variance (CV 5.7%), while rec_reuse_s00 is the steadiest (CV 4.7%, 4.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### rec_reuse_s00 shows warm-up / thermal drift (autocorr +0.52)

rec_reuse_s00's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.5x the fastest

Fastest rec_mut (4.41 us) to slowest rec_copy (28.86 us): 6.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader rec_mut vs stability leader rec_reuse_s00 (+4% speed for 1.2x steadier)

rec_mut is fastest (4.41 us, CV 5.7%); rec_reuse_s00 gives up 3.5% median for 1.2x lower variance (CV 4.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: rec_mut** at 4411.7 ns median (-3.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.54x (fastest 4411.7 ns, slowest 28857.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 31814ns | 31120ns | 30141ns | 30921ns | 33991ns | +356.56% |
| rec_mut | 6760ns | 6744ns | 6340ns | 6621ns | 7178ns | -2.99% |
| rec_reuse_s00 | 6968ns | 6825ns | 6648ns | 6782ns | 7408ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 29517ns | 27963ns | 31558ns | +532.15% | 0.035 |
| rec_mut | 4421ns | 4157ns | 4685ns | -5.32% | 0.232 |
| rec_reuse_s00 | 4669ns | 4456ns | 4966ns | base | 0.219 |

## Performance model

- Peak throughput: **0.246 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.035 | 14.4% |
| rec_mut | 0.232 | 94.2% |
| rec_reuse_s00 | 0.224 | 91.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 31814ns | 31814ns | +356.56% |
| rec_mut | 6760ns | 6760ns | -2.99% |
| rec_reuse_s00 | 6968ns | 6968ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s00 | 4568ns | base | --- | [4474, 4966] | --- | --- | --- | --- |
| rec_copy | 28858ns | +24346.0ns (+533.0%) | [+23605, +26592]ns | [28136, 31558] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| rec_mut | 4412ns | -312.2ns (-6.8%) | [-342, -91]ns | [4166, 4685] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s00 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 4530ns | +544.5% | -7.8% |
| 2 | 4493ns | +534.7% | -6.9% |
| 3 | 4456ns | +535.3% | -6.7% |
| 4 | 4605ns | +507.2% | +2.5% |
| 5 | 4969ns | +543.4% | -6.6% |
| 6 | 4963ns | +527.6% | -6.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | 0.263 | moderate+ |
| rec_mut | 0.417 | moderate+ |
| rec_reuse_s00 | 0.524 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21635.0ns | 29517.1ns | 73.3% | HIGH |
| rec_mut | 20114.5ns | 4420.7ns | 455.0% | HIGH |
| rec_reuse_s00 | 19925.0ns | 4669.3ns | 426.7% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 27962.9-31557.9 ns)
  27962.9 |########################################
  28142.7 |########################################
  28322.4 |
  28502.2 |########################################
  28681.9 |
  28861.7 |
  29041.4 |########################################
  29221.2 |
  29400.9 |
  29580.7 |
  29760.4 |
  29940.2 |
  30119.9 |
  30299.7 |
  30479.4 |
  30659.2 |
  30838.9 |
  31018.7 |########################################
  31198.4 |
  31378.2 |
  (0 below, 1 above range)

rec_mut (n=6, range 4156.7-4684.6 ns)
   4156.7 |########################################
   4183.1 |####################
   4209.5 |
   4235.9 |
   4262.3 |
   4288.7 |
   4315.1 |
   4341.5 |
   4367.9 |
   4394.3 |
   4420.6 |
   4447.0 |
   4473.4 |
   4499.8 |
   4526.2 |
   4552.6 |
   4579.0 |
   4605.4 |
   4631.8 |########################################
   4658.2 |
  (0 below, 1 above range)

rec_reuse_s00 (n=6, range 4455.8-4965.9 ns)
   4455.8 |########################################
   4481.3 |########################################
   4506.8 |########################################
   4532.3 |
   4557.8 |
   4583.3 |########################################
   4608.8 |
   4634.3 |
   4659.8 |
   4685.3 |
   4710.8 |
   4736.3 |
   4761.8 |
   4787.3 |
   4812.8 |
   4838.3 |
   4863.8 |
   4889.3 |
   4914.8 |
   4940.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=73.9% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=453.2% of algo (FFI overhead may distort results)
- **rec_reuse_s00**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **rec_reuse_s00**: bridge=428.7% of algo (FFI overhead may distort results)
