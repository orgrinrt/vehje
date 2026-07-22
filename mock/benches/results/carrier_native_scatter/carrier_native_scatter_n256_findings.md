# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, scatter profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_scatter_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_scatter_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_scatter_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_scatter_interp has the worst median (11.38 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_scatter_stencil at 3.62 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_scatter_copypatch beats baseline by 67% (significant)

carrier_nat_scatter_copypatch is -7.58 us (67%) faster than baseline carrier_nat_scatter_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_scatter_interp is an outlier: 3.1x slower than the field

carrier_nat_scatter_interp (11.38 us) is 3.1x the fastest (3.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_scatter_stencil is fastest but the noisiest (CV 18.4%)

carrier_nat_scatter_stencil wins on median (3.62 us) yet has the highest variance (CV 18.4%), while carrier_nat_scatter_copypatch is the steadiest (CV 4.0%, 3.70 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_nat_scatter_interp shows alternating (throttle bounce) (autocorr -0.61)

carrier_nat_scatter_interp's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_nat_scatter_stencil (3.62 us) to slowest carrier_nat_scatter_interp (11.38 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader carrier_nat_scatter_stencil vs stability leader carrier_nat_scatter_copypatch (+2% speed for 4.6x steadier)

carrier_nat_scatter_stencil is fastest (3.62 us, CV 18.4%); carrier_nat_scatter_copypatch gives up 2.1% median for 4.6x lower variance (CV 4.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_nat_scatter_stencil** at 3619.8 ns median (-68.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.14x (fastest 3619.8 ns, slowest 11384.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 6073ns | 6094ns | 5761ns | 5995ns | 6346ns | -56.20% |
| carrier_nat_scatter_interp | 13865ns | 14245ns | 12177ns | 13622ns | 15074ns | base |
| carrier_nat_scatter_stencil | 6296ns | 5950ns | 5510ns | 5842ns | 7370ns | -54.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 3688ns | 3512ns | 3848ns | -66.65% | 0.069 |
| carrier_nat_scatter_interp | 11056ns | 9975ns | 11761ns | base | 0.023 |
| carrier_nat_scatter_stencil | 3863ns | 3359ns | 4575ns | -65.06% | 0.066 |

## Performance model

- Peak throughput: **0.076 Gops/s** (carrier_nat_scatter_stencil; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.069 | 90.8% |
| carrier_nat_scatter_interp | 0.022 | 29.5% |
| carrier_nat_scatter_stencil | 0.071 | 92.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_scatter_copypatch | 6073ns | 6073ns | -56.20% |
| carrier_nat_scatter_interp | 13865ns | 13865ns | base |
| carrier_nat_scatter_stencil | 6296ns | 6296ns | -54.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_scatter_interp | 11384ns | base | --- | [10024, 11761] | --- | --- | --- | --- |
| carrier_nat_scatter_copypatch | 3698ns | -7579.8ns (-66.6%) | [-8138, -6388]ns | [3518, 3848] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_scatter_stencil | 3620ns | -7026.4ns (-61.7%) | [-8000, -6554]ns | [3394, 4575] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_scatter_interp | carrier_nat_scatter_copypatch | carrier_nat_scatter_stencil |
|---|---|---|---|
| 1 | 9975ns | -62.4% | -65.6% |
| 2 | 11860ns | -68.5% | -55.3% |
| 3 | 10073ns | -65.0% | -65.5% |
| 4 | 11662ns | -69.9% | -71.2% |
| 5 | 11219ns | -67.4% | -66.5% |
| 6 | 11549ns | -65.8% | -66.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.168 | ok |
| carrier_nat_scatter_interp | -0.607 | HIGH- (thermal bounce) |
| carrier_nat_scatter_stencil | -0.350 | moderate- |

**Consistency summary:**

- **carrier_nat_scatter_copypatch**: won 6/6, lost 0/6
- **carrier_nat_scatter_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 81082.5ns | 3687.8ns | 2198.6% | HIGH |
| carrier_nat_scatter_interp | 92639.6ns | 11056.4ns | 837.9% | HIGH |
| carrier_nat_scatter_stencil | 88765.1ns | 3862.7ns | 2298.0% | HIGH |

## Distribution (algo ns)

```
carrier_nat_scatter_copypatch (n=6, range 3512.5-3848.3 ns)
   3512.5 |########################################
   3529.3 |
   3546.1 |
   3562.9 |
   3579.7 |
   3596.5 |
   3613.3 |
   3630.0 |
   3646.8 |####################
   3663.6 |
   3680.4 |
   3697.2 |
   3714.0 |
   3730.8 |####################
   3747.6 |####################
   3764.4 |
   3781.2 |
   3798.0 |
   3814.8 |
   3831.6 |
  (0 below, 1 above range)

carrier_nat_scatter_interp (n=6, range 9975.4-11760.9 ns)
   9975.4 |########################################
  10064.7 |########################################
  10153.9 |
  10243.2 |
  10332.5 |
  10421.8 |
  10511.0 |
  10600.3 |
  10689.6 |
  10778.9 |
  10868.1 |
  10957.4 |
  11046.7 |
  11135.9 |########################################
  11225.2 |
  11314.5 |
  11403.8 |
  11493.0 |########################################
  11582.3 |########################################
  11671.6 |
  (0 below, 1 above range)

carrier_nat_scatter_stencil (n=6, range 3358.8-4574.5 ns)
   3358.8 |####################
   3419.6 |########################################
   3480.4 |
   3541.2 |
   3601.9 |
   3662.7 |
   3723.5 |####################
   3784.3 |
   3845.1 |####################
   3905.9 |
   3966.7 |
   4027.5 |
   4088.2 |
   4149.0 |
   4209.8 |
   4270.6 |
   4331.4 |
   4392.2 |
   4453.0 |
   4513.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_scatter_copypatch**: bridge=2183.7% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_interp**: bridge=803.0% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_stencil**: bridge=2238.4% of algo (FFI overhead may distort results)
