# Closure representation: flat-capture vs linked-env

2 variants, 6 samples per variant.
Baseline: **hx_closure__linked**

## Highlights

Baseline for all deltas below: **hx_closure__linked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (hx_closure__linked)

The baseline hx_closure__linked is the fastest (998 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### hx_closure__flat's edge over baseline is significant but tiny (4 ns, 0.36%)

hx_closure__flat differs from baseline hx_closure__linked by 4 ns (0.36%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (hx_closure__linked) is the fastest** at 997.5 ns median
- Spread: 1.02x (fastest 997.5 ns, slowest 1015.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_closure__flat | 3263ns | 3236ns | 3162ns | 3212ns | 3388ns | +3.02% |
| hx_closure__linked | 3167ns | 3158ns | 3095ns | 3154ns | 3223ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_closure__flat | 1028ns | 995ns | 1071ns | +2.79% | 3.986 |
| hx_closure__linked | 1000ns | 978ns | 1016ns | base | 4.097 |

## Performance model

- Peak throughput: **4.187 Gops/s** (hx_closure__linked; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_closure__flat | 4.034 | 96.3% |
| hx_closure__linked | 4.106 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_closure__flat | 3263ns | 3263ns | +3.02% |
| hx_closure__linked | 3167ns | 3167ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_closure__linked | 998ns | base | --- | [985, 1016] | --- | --- | --- | --- |
| hx_closure__flat | 1015ns | no significant difference | [-0, +80]ns | [996, 1071] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_closure__linked | hx_closure__flat |
|---|---|---|
| 1 | 978ns | +13.2% |
| 2 | 1000ns | +3.1% |
| 3 | 997ns | -0.1% |
| 4 | 992ns | +0.5% |
| 5 | 998ns | +0.1% |
| 6 | 1032ns | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_closure__flat | 0.201 | moderate+ |
| hx_closure__linked | -0.018 | ok |

**Consistency summary:**

- **hx_closure__flat**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_closure__flat | 3.2ns | 1027.5ns | 0.3% |  |
| hx_closure__linked | 3.2ns | 999.6ns | 0.3% |  |

## Distribution (algo ns)

```
hx_closure__flat (n=6, range 995.4-1070.8 ns)
    995.4 |########################################
    999.2 |####################
   1002.9 |
   1006.7 |
   1010.5 |
   1014.3 |
   1018.0 |
   1021.8 |
   1025.6 |
   1029.4 |####################
   1033.1 |####################
   1036.9 |
   1040.7 |
   1044.4 |
   1048.2 |
   1052.0 |
   1055.8 |
   1059.5 |
   1063.3 |
   1067.1 |
  (0 below, 1 above range)

hx_closure__linked (n=6, range 978.3-1016.5 ns)
    978.3 |########################################
    980.2 |
    982.1 |
    984.0 |
    985.9 |
    987.8 |
    989.7 |
    991.7 |########################################
    993.6 |
    995.5 |########################################
    997.4 |########################################
    999.3 |########################################
   1001.2 |
   1003.1 |
   1005.0 |
   1006.9 |
   1008.8 |
   1010.7 |
   1012.6 |
   1014.5 |
  (0 below, 1 above range)

```
