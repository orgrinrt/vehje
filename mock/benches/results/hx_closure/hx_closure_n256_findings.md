# Closure representation: flat-capture vs linked-env

2 variants, 6 samples per variant.
Baseline: **hx_closure__linked**

## Highlights

Baseline for all deltas below: **hx_closure__linked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_closure__linked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_closure__linked has the worst median (105 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_closure__flat at 104 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hx_closure__flat is fastest but the noisiest (CV 10.5%)

hx_closure__flat wins on median (104 ns) yet has the highest variance (CV 10.5%), while hx_closure__linked is the steadiest (CV 7.8%, 105 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hx_closure__linked shows warm-up / thermal drift (autocorr +0.55)

hx_closure__linked's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader hx_closure__flat vs stability leader hx_closure__linked (+1% speed for 1.4x steadier)

hx_closure__flat is fastest (104 ns, CV 10.5%); hx_closure__linked gives up 0.6% median for 1.4x lower variance (CV 7.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### hx_closure__flat's edge over baseline is significant but tiny (-0 ns, 0.00%)

hx_closure__flat differs from baseline hx_closure__linked by -0 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: hx_closure__flat** at 103.9 ns median (-0.6% vs baseline)
- Spread: 1.01x (fastest 103.9 ns, slowest 104.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_closure__flat | 2505ns | 2421ns | 2218ns | 2386ns | 2827ns | +1.10% |
| hx_closure__linked | 2478ns | 2475ns | 2237ns | 2399ns | 2716ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_closure__flat | 107ns | 92ns | 119ns | +1.16% | 2.403 |
| hx_closure__linked | 105ns | 96ns | 115ns | base | 2.431 |

## Performance model

- Peak throughput: **2.780 Gops/s** (hx_closure__flat; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_closure__flat | 2.463 | 88.6% |
| hx_closure__linked | 2.449 | 88.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_closure__flat | 2505ns | 2505ns | +1.10% |
| hx_closure__linked | 2478ns | 2478ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_closure__linked | 105ns | base | --- | [97, 115] | --- | --- | --- | --- |
| hx_closure__flat | 104ns | no significant difference | [-6, +10]ns | [96, 119] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_closure__linked | hx_closure__flat |
|---|---|---|
| 1 | 98ns | +3.4% |
| 2 | 96ns | +4.4% |
| 3 | 98ns | -6.3% |
| 4 | 114ns | -5.8% |
| 5 | 115ns | -2.9% |
| 6 | 111ns | +14.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_closure__flat | 0.322 | moderate+ |
| hx_closure__linked | 0.547 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **hx_closure__flat**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_closure__flat | 3.2ns | 106.5ns | 3.1% |  |
| hx_closure__linked | 2.7ns | 105.3ns | 2.6% |  |

## Distribution (algo ns)

```
hx_closure__flat (n=6, range 92.1-119.4 ns)
     92.1 |####################
     93.5 |
     94.8 |
     96.2 |
     97.6 |
     98.9 |
    100.3 |########################################
    101.7 |
    103.0 |
    104.4 |
    105.8 |####################
    107.1 |
    108.5 |
    109.8 |
    111.2 |####################
    112.6 |
    113.9 |
    115.3 |
    116.7 |
    118.0 |
  (0 below, 1 above range)

hx_closure__linked (n=6, range 96.2-114.6 ns)
     96.2 |########################################
     97.1 |########################################
     98.0 |########################################
     99.0 |
     99.9 |
    100.8 |
    101.7 |
    102.6 |
    103.5 |
    104.5 |
    105.4 |
    106.3 |
    107.2 |
    108.1 |
    109.0 |
    110.0 |########################################
    110.9 |
    111.8 |
    112.7 |
    113.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **hx_closure__linked**: autocorrelation=0.55 (measurement drift or warm-up artifact)
