# Effect lattice: thermometer-OR join vs per-lane branch max

2 variants, 6 samples per variant.
Baseline: **hx_effect__thermo**

## Highlights

Baseline for all deltas below: **hx_effect__thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_effect__thermo) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_effect__thermo has the worst median (273 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_effect__branchmax at 263 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_effect__branchmax is fastest but the noisiest (CV 10.1%)

hx_effect__branchmax wins on median (263 ns) yet has the highest variance (CV 10.1%), while hx_effect__thermo is the steadiest (CV 8.4%, 273 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10 ns) is smaller than the fastest variant's own run-to-run std-dev (27 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader hx_effect__branchmax vs stability leader hx_effect__thermo (+4% speed for 1.2x steadier)

hx_effect__branchmax is fastest (263 ns, CV 10.1%); hx_effect__thermo gives up 3.7% median for 1.2x lower variance (CV 8.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### hx_effect__branchmax's edge over baseline is significant but tiny (-0 ns, 0.07%)

hx_effect__branchmax differs from baseline hx_effect__thermo by -0 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: hx_effect__branchmax** at 263.1 ns median (-3.6% vs baseline)
- Spread: 1.04x (fastest 263.1 ns, slowest 272.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_effect__branchmax | 2580ns | 2465ns | 2383ns | 2445ns | 2882ns | -1.59% |
| hx_effect__thermo | 2622ns | 2562ns | 2412ns | 2515ns | 2887ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_effect__branchmax | 273ns | 240ns | 309ns | -2.11% | 0.936 |
| hx_effect__thermo | 279ns | 256ns | 308ns | base | 0.917 |

## Performance model

- Peak throughput: **1.065 Gops/s** (hx_effect__branchmax; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_effect__branchmax | 0.973 | 91.4% |
| hx_effect__thermo | 0.938 | 88.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_effect__branchmax | 2580ns | 2580ns | -1.59% |
| hx_effect__thermo | 2622ns | 2622ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_effect__thermo | 273ns | base | --- | [257, 308] | --- | --- | --- | --- |
| hx_effect__branchmax | 263ns | no significant difference | [-25, +8]ns | [248, 309] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_effect__thermo | hx_effect__branchmax |
|---|---|---|
| 1 | 287ns | -11.1% |
| 2 | 305ns | +2.9% |
| 3 | 311ns | -2.1% |
| 4 | 257ns | +2.4% |
| 5 | 259ns | -7.3% |
| 6 | 256ns | +2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_effect__branchmax | 0.207 | moderate+ |
| hx_effect__thermo | 0.386 | moderate+ |

**Consistency summary:**

- **hx_effect__branchmax**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_effect__branchmax | 3.6ns | 273.4ns | 1.3% |  |
| hx_effect__thermo | 3.7ns | 279.3ns | 1.3% |  |

## Distribution (algo ns)

```
hx_effect__branchmax (n=6, range 240.4-309.4 ns)
    240.4 |####################
    243.8 |
    247.3 |
    250.8 |
    254.2 |####################
    257.6 |
    261.1 |########################################
    264.6 |
    268.0 |
    271.4 |
    274.9 |
    278.3 |
    281.8 |
    285.2 |
    288.7 |
    292.1 |
    295.6 |
    299.0 |
    302.5 |####################
    305.9 |
  (0 below, 1 above range)

hx_effect__thermo (n=6, range 256.2-308.3 ns)
    256.2 |########################################
    258.8 |####################
    261.4 |
    264.0 |
    266.6 |
    269.2 |
    271.8 |
    274.4 |
    277.0 |
    279.6 |
    282.2 |
    284.9 |####################
    287.5 |
    290.1 |
    292.7 |
    295.3 |
    297.9 |
    300.5 |
    303.1 |####################
    305.7 |
  (0 below, 1 above range)

```
