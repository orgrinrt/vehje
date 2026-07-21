# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_reuse__reuse) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_reuse__reuse has the worst median (648 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_reuse__copy at 631 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_reuse__copy is fastest but the noisiest (CV 10.8%)

hx_reuse__copy wins on median (631 ns) yet has the highest variance (CV 10.8%), while hx_reuse__reuse is the steadiest (CV 8.2%, 648 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (17 ns) is smaller than the fastest variant's own run-to-run std-dev (68 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### hx_reuse__copy's comparison is tie-heavy (17% tied pairs)

17% of paired samples for hx_reuse__copy are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader hx_reuse__copy vs stability leader hx_reuse__reuse (+3% speed for 1.3x steadier)

hx_reuse__copy is fastest (631 ns, CV 10.8%); hx_reuse__reuse gives up 2.7% median for 1.3x lower variance (CV 8.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### hx_reuse__copy's edge over baseline is significant but tiny (-10 ns, 1.47%)

hx_reuse__copy differs from baseline hx_reuse__reuse by -10 ns (1.47%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: hx_reuse__copy** at 630.7 ns median (-2.6% vs baseline)
- Spread: 1.03x (fastest 630.7 ns, slowest 647.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 3250ns | 3321ns | 2650ns | 3206ns | 3616ns | -0.90% |
| hx_reuse__reuse | 3280ns | 3443ns | 2665ns | 3355ns | 3474ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 618ns | 505ns | 686ns | -0.57% | 1.657 |
| hx_reuse__reuse | 621ns | 511ns | 660ns | base | 1.648 |

## Performance model

- Peak throughput: **2.029 Gops/s** (hx_reuse__copy; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 1.624 | 80.0% |
| hx_reuse__reuse | 1.581 | 77.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 3250ns | 3250ns | -0.90% |
| hx_reuse__reuse | 3280ns | 3280ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 648ns | base | --- | [557, 660] | --- | --- | --- | --- |
| hx_reuse__copy | 631ns | no significant difference | [-39, +38]ns | [538, 686] | no | 0.3750 | 0.3750 | **1** (17%, HIGH) |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 511ns | -1.2% |
| 2 | 603ns | -5.5% |
| 3 | 660ns | -6.9% |
| 4 | 650ns | +0.0% |
| 5 | 660ns | -2.0% |
| 6 | 645ns | +11.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | 0.338 | moderate+ |
| hx_reuse__reuse | 0.260 | moderate+ |

**Consistency summary:**

- **hx_reuse__copy**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 3.9ns | 617.9ns | 0.6% |  |
| hx_reuse__reuse | 2.6ns | 621.4ns | 0.4% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 504.6-685.6 ns)
    504.6 |########################################
    513.6 |
    522.7 |
    531.8 |
    540.8 |
    549.9 |
    558.9 |
    568.0 |########################################
    577.0 |
    586.1 |
    595.1 |
    604.2 |
    613.2 |########################################
    622.2 |
    631.3 |
    640.4 |########################################
    649.4 |########################################
    658.5 |
    667.5 |
    676.6 |
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 510.8-659.8 ns)
    510.8 |####################
    518.2 |
    525.7 |
    533.1 |
    540.6 |
    548.0 |
    555.5 |
    563.0 |
    570.4 |
    577.9 |
    585.3 |
    592.8 |
    600.2 |####################
    607.6 |
    615.1 |
    622.5 |
    630.0 |
    637.4 |
    644.9 |########################################
    652.3 |####################
  (0 below, 1 above range)

```
