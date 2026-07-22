# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, wideselect profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_wideselect_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_wideselect_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_wideselect_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_wideselect_interp has the worst median (10.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_wideselect_stencil at 3.90 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_wideselect_copypatch beats baseline by 62% (significant)

carrier_nat_wideselect_copypatch is -6.74 us (62%) faster than baseline carrier_nat_wideselect_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_wideselect_interp is an outlier: 2.8x slower than the field

carrier_nat_wideselect_interp (10.78 us) is 2.8x the fastest (3.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Speed leader carrier_nat_wideselect_stencil vs stability leader carrier_nat_wideselect_copypatch (+4% speed for 1.3x steadier)

carrier_nat_wideselect_stencil is fastest (3.90 us, CV 5.2%); carrier_nat_wideselect_copypatch gives up 4.2% median for 1.3x lower variance (CV 4.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_nat_wideselect_stencil** at 3900.8 ns median (-63.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.76x (fastest 3900.8 ns, slowest 10782.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 6523ns | 6547ns | 5905ns | 6464ns | 6920ns | -50.64% |
| carrier_nat_wideselect_interp | 13216ns | 13258ns | 12240ns | 12920ns | 14146ns | base |
| carrier_nat_wideselect_stencil | 6275ns | 6294ns | 5842ns | 6199ns | 6605ns | -52.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 3993ns | 3661ns | 4121ns | -62.92% | 0.064 |
| carrier_nat_wideselect_interp | 10767ns | 9940ns | 11572ns | base | 0.024 |
| carrier_nat_wideselect_stencil | 3881ns | 3611ns | 4085ns | -63.95% | 0.066 |

## Performance model

- Peak throughput: **0.071 Gops/s** (carrier_nat_wideselect_stencil; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.063 | 88.9% |
| carrier_nat_wideselect_interp | 0.024 | 33.5% |
| carrier_nat_wideselect_stencil | 0.066 | 92.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_wideselect_copypatch | 6523ns | 6523ns | -50.64% |
| carrier_nat_wideselect_interp | 13216ns | 13216ns | base |
| carrier_nat_wideselect_stencil | 6275ns | 6275ns | -52.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_wideselect_interp | 10783ns | base | --- | [9947, 11572] | --- | --- | --- | --- |
| carrier_nat_wideselect_copypatch | 4063ns | -6738.5ns (-62.5%) | [-7757, -5828]ns | [3794, 4121] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_wideselect_stencil | 3901ns | -6697.9ns (-62.1%) | [-7732, -6227]ns | [3659, 4085] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_wideselect_interp | carrier_nat_wideselect_copypatch | carrier_nat_wideselect_stencil |
|---|---|---|---|
| 1 | 12294ns | -70.2% | -70.6% |
| 2 | 9940ns | -59.1% | -62.4% |
| 3 | 9953ns | -58.1% | -62.8% |
| 4 | 10808ns | -63.7% | -62.2% |
| 5 | 10850ns | -62.6% | -62.5% |
| 6 | 10758ns | -62.1% | -62.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_wideselect_copypatch | -0.139 | ok |
| carrier_nat_wideselect_interp | -0.168 | ok |
| carrier_nat_wideselect_stencil | 0.440 | moderate+ |

**Consistency summary:**

- **carrier_nat_wideselect_copypatch**: won 6/6, lost 0/6
- **carrier_nat_wideselect_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 82275.9ns | 3992.6ns | 2060.7% | HIGH |
| carrier_nat_wideselect_interp | 89704.9ns | 10767.2ns | 833.1% | HIGH |
| carrier_nat_wideselect_stencil | 82563.3ns | 3881.5ns | 2127.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_wideselect_copypatch (n=6, range 3660.8-4121.1 ns)
   3660.8 |#############
   3683.8 |
   3706.8 |
   3729.8 |
   3752.9 |
   3775.9 |
   3798.9 |
   3821.9 |
   3844.9 |
   3867.9 |
   3890.9 |
   3913.9 |#############
   3937.0 |
   3960.0 |
   3983.0 |
   4006.0 |
   4029.0 |
   4052.0 |########################################
   4075.0 |
   4098.0 |
  (0 below, 1 above range)

carrier_nat_wideselect_interp (n=6, range 9940.4-11572.1 ns)
   9940.4 |########################################
  10022.0 |
  10103.6 |
  10185.2 |
  10266.7 |
  10348.3 |
  10429.9 |
  10511.5 |
  10593.1 |
  10674.7 |
  10756.2 |########################################
  10837.8 |####################
  10919.4 |
  11001.0 |
  11082.6 |
  11164.2 |
  11245.8 |
  11327.3 |
  11408.9 |
  11490.5 |
  (0 below, 1 above range)

carrier_nat_wideselect_stencil (n=6, range 3611.2-4084.9 ns)
   3611.2 |####################
   3634.9 |
   3658.6 |
   3682.3 |
   3705.9 |####################
   3729.6 |####################
   3753.3 |
   3777.0 |
   3800.7 |
   3824.4 |
   3848.1 |
   3871.8 |
   3895.4 |
   3919.1 |
   3942.8 |
   3966.5 |
   3990.2 |
   4013.9 |
   4037.6 |
   4061.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_wideselect_copypatch**: bridge=2007.3% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_interp**: bridge=813.5% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_stencil**: bridge=2119.9% of algo (FFI overhead may distort results)
