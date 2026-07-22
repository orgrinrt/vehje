# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, wideselect profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_wideselect_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_wideselect_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_wideselect_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_wideselect_interp has the worst median (2.23 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_wideselect_copypatch at 659 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_wideselect_copypatch beats baseline by 69% (significant)

carrier_nat_wideselect_copypatch is -1.53 us (69%) faster than baseline carrier_nat_wideselect_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_wideselect_interp is an outlier: 3.4x slower than the field

carrier_nat_wideselect_interp (2.23 us) is 3.4x the fastest (659 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.4x the fastest

Fastest carrier_nat_wideselect_copypatch (659 ns) to slowest carrier_nat_wideselect_interp (2.23 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_nat_wideselect_interp is inconsistent: worst-20% is 2.0x its best-20%

carrier_nat_wideselect_interp's best 20% of batches run at 2.09 us but its worst 20% at 4.27 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_nat_wideselect_copypatch** at 659.4 ns median (-70.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.38x (fastest 659.4 ns, slowest 2226.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 3123ns | 3048ns | 2761ns | 2976ns | 3524ns | -43.46% |
| carrier_nat_wideselect_interp | 5523ns | 4645ns | 4287ns | 4564ns | 7581ns | base |
| carrier_nat_wideselect_stencil | 3475ns | 3115ns | 2804ns | 3015ns | 4499ns | -37.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 680ns | 585ns | 794ns | -76.29% | 0.094 |
| carrier_nat_wideselect_interp | 2869ns | 2094ns | 4266ns | base | 0.022 |
| carrier_nat_wideselect_stencil | 854ns | 614ns | 1220ns | -70.24% | 0.075 |

## Performance model

- Peak throughput: **0.109 Gops/s** (carrier_nat_wideselect_copypatch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.097 | 88.8% |
| carrier_nat_wideselect_interp | 0.029 | 26.3% |
| carrier_nat_wideselect_stencil | 0.089 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_wideselect_copypatch | 3123ns | 3123ns | -43.46% |
| carrier_nat_wideselect_interp | 5523ns | 5523ns | base |
| carrier_nat_wideselect_stencil | 3475ns | 3475ns | -37.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_wideselect_interp | 2226ns | base | --- | [2115, 4266] | --- | --- | --- | --- |
| carrier_nat_wideselect_copypatch | 659ns | -1533.3ns (-68.9%) | [-3607, -1426]ns | [587, 794] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_wideselect_stencil | 721ns | -1493.4ns (-67.1%) | [-3569, -984]ns | [620, 1220] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_wideselect_interp | carrier_nat_wideselect_copypatch | carrier_nat_wideselect_stencil |
|---|---|---|---|
| 1 | 2094ns | -72.0% | -70.1% |
| 2 | 2180ns | -71.5% | -69.7% |
| 3 | 5964ns | -90.1% | -89.7% |
| 4 | 2569ns | -71.6% | -69.6% |
| 5 | 2272ns | -62.2% | -27.4% |
| 6 | 2136ns | -67.4% | -63.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.330 | moderate+ |
| carrier_nat_wideselect_interp | -0.164 | ok |
| carrier_nat_wideselect_stencil | -0.001 | ok |

**Consistency summary:**

- **carrier_nat_wideselect_copypatch**: won 6/6, lost 0/6
- **carrier_nat_wideselect_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 68561.3ns | 680.2ns | 10079.6% | HIGH |
| carrier_nat_wideselect_interp | 87926.2ns | 2869.2ns | 3064.5% | HIGH |
| carrier_nat_wideselect_stencil | 73597.0ns | 853.8ns | 8619.9% | HIGH |

## Distribution (algo ns)

```
carrier_nat_wideselect_copypatch (n=6, range 585.4-794.3 ns)
    585.4 |########################################
    595.8 |
    606.3 |
    616.7 |####################
    627.2 |
    637.6 |
    648.1 |
    658.5 |
    669.0 |
    679.4 |
    689.9 |####################
    700.3 |
    710.8 |
    721.2 |####################
    731.7 |
    742.1 |
    752.6 |
    763.0 |
    773.5 |
    783.9 |
  (0 below, 1 above range)

carrier_nat_wideselect_interp (n=6, range 2094.2-4266.5 ns)
   2094.2 |########################################
   2202.8 |#############
   2311.4 |
   2420.0 |
   2528.7 |#############
   2637.3 |
   2745.9 |
   2854.5 |
   2963.1 |
   3071.7 |
   3180.3 |
   3289.0 |
   3397.6 |
   3506.2 |
   3614.8 |
   3723.4 |
   3832.0 |
   3940.7 |
   4049.3 |
   4157.9 |
  (0 below, 1 above range)

carrier_nat_wideselect_stencil (n=6, range 613.8-1220.4 ns)
    613.8 |########################################
    644.1 |####################
    674.5 |
    704.8 |
    735.1 |
    765.5 |########################################
    795.8 |
    826.1 |
    856.4 |
    886.8 |
    917.1 |
    947.4 |
    977.8 |
   1008.1 |
   1038.4 |
   1068.8 |
   1099.1 |
   1129.4 |
   1159.7 |
   1190.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_wideselect_copypatch**: bridge=10418.3% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_interp**: CV=48.5% (high variance, measurements may be unstable)
- **carrier_nat_wideselect_interp**: bridge=3892.5% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_stencil**: CV=42.5% (high variance, measurements may be unstable)
- **carrier_nat_wideselect_stencil**: bridge=9983.5% of algo (FFI overhead may distort results)
