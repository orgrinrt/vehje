# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, leaf profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_leaf_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_leaf_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_leaf_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_leaf_interp has the worst median (60.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_leaf_copypatch at 14.13 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_leaf_copypatch beats baseline by 78% (significant)

carrier_nat_leaf_copypatch is -47.07 us (78%) faster than baseline carrier_nat_leaf_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_leaf_interp is an outlier: 4.3x slower than the field

carrier_nat_leaf_interp (60.70 us) is 4.3x the fastest (14.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_leaf_copypatch, carrier_nat_leaf_stencil) are a dead heat (<1%)

carrier_nat_leaf_copypatch (14.13 us) and carrier_nat_leaf_stencil (14.18 us) differ by 0.34%, inside the noise, even though the wider field spreads 329.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_nat_leaf_interp shows alternating (throttle bounce) (autocorr -0.54)

carrier_nat_leaf_interp's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest carrier_nat_leaf_copypatch (14.13 us) to slowest carrier_nat_leaf_interp (60.70 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_leaf_copypatch** at 14133.2 ns median (-76.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.29x (fastest 14133.2 ns, slowest 60699.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 16492ns | 16515ns | 15371ns | 16287ns | 17361ns | -74.36% |
| carrier_nat_leaf_interp | 64313ns | 63074ns | 61163ns | 62775ns | 68195ns | base |
| carrier_nat_leaf_stencil | 16462ns | 16572ns | 15344ns | 16231ns | 17368ns | -74.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 14115ns | 13141ns | 14834ns | -77.17% | 0.073 |
| carrier_nat_leaf_interp | 61830ns | 58608ns | 65621ns | base | 0.017 |
| carrier_nat_leaf_stencil | 14102ns | 13205ns | 14844ns | -77.19% | 0.073 |

## Performance model

- Peak throughput: **0.078 Gops/s** (carrier_nat_leaf_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.072 | 93.0% |
| carrier_nat_leaf_interp | 0.017 | 21.6% |
| carrier_nat_leaf_stencil | 0.072 | 92.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_leaf_copypatch | 16492ns | 16492ns | -74.36% |
| carrier_nat_leaf_interp | 64313ns | 64313ns | base |
| carrier_nat_leaf_stencil | 16462ns | 16462ns | -74.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_leaf_interp | 60699ns | base | --- | [59169, 65621] | --- | --- | --- | --- |
| carrier_nat_leaf_copypatch | 14133ns | -47066.2ns (-77.5%) | [-50800, -45279]ns | [13377, 14834] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_leaf_stencil | 14181ns | -46293.3ns (-76.3%) | [-51474, -45416]ns | [13281, 14844] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_leaf_interp | carrier_nat_leaf_copypatch | carrier_nat_leaf_stencil |
|---|---|---|---|
| 1 | 58608ns | -77.6% | -76.6% |
| 2 | 67486ns | -78.3% | -77.9% |
| 3 | 60694ns | -77.6% | -75.7% |
| 4 | 59731ns | -75.5% | -77.9% |
| 5 | 63757ns | -76.4% | -79.0% |
| 6 | 60704ns | -77.5% | -75.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_leaf_copypatch | -0.340 | moderate- |
| carrier_nat_leaf_interp | -0.537 | HIGH- (thermal bounce) |
| carrier_nat_leaf_stencil | -0.034 | ok |

**Consistency summary:**

- **carrier_nat_leaf_copypatch**: won 6/6, lost 0/6
- **carrier_nat_leaf_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 95087.2ns | 14114.9ns | 673.7% | HIGH |
| carrier_nat_leaf_interp | 123895.4ns | 61829.9ns | 200.4% | HIGH |
| carrier_nat_leaf_stencil | 100534.7ns | 14102.1ns | 712.9% | HIGH |

## Distribution (algo ns)

```
carrier_nat_leaf_copypatch (n=6, range 13140.8-14834.3 ns)
  13140.8 |####################
  13225.5 |
  13310.2 |
  13394.8 |
  13479.5 |
  13564.2 |####################
  13648.9 |####################
  13733.5 |
  13818.2 |
  13902.9 |
  13987.6 |
  14072.3 |
  14156.9 |
  14241.6 |
  14326.3 |
  14411.0 |
  14495.6 |
  14580.3 |########################################
  14665.0 |
  14749.7 |
  (0 below, 1 above range)

carrier_nat_leaf_interp (n=6, range 58607.5-65621.2 ns)
  58607.5 |####################
  58958.2 |
  59308.9 |
  59659.6 |####################
  60010.2 |
  60360.9 |########################################
  60711.6 |
  61062.3 |
  61413.0 |
  61763.7 |
  62114.4 |
  62465.1 |
  62815.8 |
  63166.4 |
  63517.1 |####################
  63867.8 |
  64218.5 |
  64569.2 |
  64919.9 |
  65270.6 |
  (0 below, 1 above range)

carrier_nat_leaf_stencil (n=6, range 13204.6-14844.1 ns)
  13204.6 |########################################
  13286.6 |########################################
  13368.6 |
  13450.5 |
  13532.5 |
  13614.5 |
  13696.5 |########################################
  13778.4 |
  13860.4 |
  13942.4 |
  14024.4 |
  14106.4 |
  14188.3 |
  14270.3 |
  14352.3 |
  14434.3 |
  14516.2 |
  14598.2 |########################################
  14680.2 |########################################
  14762.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_leaf_copypatch**: bridge=674.7% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_interp**: bridge=200.2% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_stencil**: bridge=713.5% of algo (FFI overhead may distort results)
