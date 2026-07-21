# Effect lattice: thermometer-OR join vs per-lane branch max

2 variants, 6 samples per variant.
Baseline: **hx_effect__thermo**

## Highlights

Baseline for all deltas below: **hx_effect__thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_effect__thermo) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_effect__thermo has the worst median (4.55 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_effect__branchmax at 3.97 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_effect__branchmax dominates: 14% faster than the next best (hx_effect__thermo)

hx_effect__branchmax (3.97 us) leads hx_effect__thermo (4.55 us) by 14%, a clear separation rather than a photo finish. CV 8.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_effect__branchmax is fastest but the noisiest (CV 8.2%)

hx_effect__branchmax wins on median (3.97 us) yet has the highest variance (CV 8.2%), while hx_effect__thermo is the steadiest (CV 4.3%, 4.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: hx_effect__branchmax** at 3974.0 ns median (-12.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.14x (fastest 3974.0 ns, slowest 4546.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_effect__branchmax | 6429ns | 6562ns | 5402ns | 6418ns | 6958ns | -10.13% |
| hx_effect__thermo | 7153ns | 7123ns | 6676ns | 7122ns | 7438ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_effect__branchmax | 3895ns | 3270ns | 4216ns | -14.67% | 1.052 |
| hx_effect__thermo | 4565ns | 4265ns | 4744ns | base | 0.897 |

## Performance model

- Peak throughput: **1.253 Gops/s** (hx_effect__branchmax; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_effect__branchmax | 1.031 | 82.3% |
| hx_effect__thermo | 0.901 | 71.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_effect__branchmax | 6429ns | 6429ns | -10.13% |
| hx_effect__thermo | 7153ns | 7153ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_effect__thermo | 4546ns | base | --- | [4404, 4744] | --- | --- | --- | --- |
| hx_effect__branchmax | 3974ns | -703.9ns (-15.5%) | [-977, -329]ns | [3495, 4216] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_effect__thermo | hx_effect__branchmax |
|---|---|---|
| 1 | 4265ns | -23.3% |
| 2 | 4544ns | -6.8% |
| 3 | 4933ns | -19.4% |
| 4 | 4546ns | -12.6% |
| 5 | 4546ns | -7.7% |
| 6 | 4555ns | -18.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_effect__branchmax | -0.325 | moderate- |
| hx_effect__thermo | -0.034 | ok |

**Consistency summary:**

- **hx_effect__branchmax**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_effect__branchmax | 4.0ns | 3895.0ns | 0.1% |  |
| hx_effect__thermo | 3.3ns | 4564.9ns | 0.1% |  |

## Distribution (algo ns)

```
hx_effect__branchmax (n=6, range 3269.6-4216.4 ns)
   3269.6 |####################
   3316.9 |
   3364.3 |
   3411.6 |
   3459.0 |
   3506.3 |
   3553.7 |
   3601.0 |
   3648.3 |
   3695.7 |####################
   3743.0 |
   3790.4 |
   3837.7 |
   3885.1 |
   3932.4 |########################################
   3979.7 |
   4027.1 |
   4074.4 |
   4121.8 |
   4169.1 |####################
  (0 below, 1 above range)

hx_effect__thermo (n=6, range 4264.6-4744.1 ns)
   4264.6 |#############
   4288.6 |
   4312.6 |
   4336.5 |
   4360.5 |
   4384.5 |
   4408.5 |
   4432.4 |
   4456.4 |
   4480.4 |
   4504.4 |
   4528.4 |########################################
   4552.3 |#############
   4576.3 |
   4600.3 |
   4624.3 |
   4648.2 |
   4672.2 |
   4696.2 |
   4720.2 |
  (0 below, 1 above range)

```
