# Value representation: static/raw vs runtime-tagged vs NaN-boxed (dynamic-typing cost)

3 variants, 6 samples per variant.
Baseline: **valrepr_static**

## Highlights

Baseline for all deltas below: **valrepr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (valrepr_nanbox, valrepr_static) are a dead heat (<1%)

valrepr_nanbox (35.93 us) and valrepr_static (36.06 us) differ by 0.37%, inside the noise, even though the wider field spreads 5.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader valrepr_nanbox vs stability leader valrepr_static (+0% speed for 1.4x steadier)

valrepr_nanbox is fastest (35.93 us, CV 4.7%); valrepr_static gives up 0.4% median for 1.4x lower variance (CV 3.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: valrepr_nanbox** at 35931.2 ns median (-0.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 35931.2 ns, slowest 38016.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| valrepr_nanbox | 37680ns | 38381ns | 33749ns | 38235ns | 38813ns | -0.93% |
| valrepr_static | 38033ns | 38505ns | 35174ns | 38435ns | 38860ns | base |
| valrepr_tagged | 39815ns | 40537ns | 36085ns | 40465ns | 40704ns | +4.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| valrepr_nanbox | 35252ns | 31518ns | 36297ns | -1.03% | 0.029 |
| valrepr_static | 35619ns | 32916ns | 36415ns | base | 0.029 |
| valrepr_tagged | 37391ns | 33900ns | 38252ns | +4.98% | 0.027 |

## Performance model

- Peak throughput: **0.032 Gops/s** (valrepr_nanbox; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| valrepr_nanbox | 0.028 | 87.7% |
| valrepr_static | 0.028 | 87.4% |
| valrepr_tagged | 0.027 | 82.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| valrepr_nanbox | 37680ns | 37680ns | -0.93% |
| valrepr_static | 38033ns | 38033ns | base |
| valrepr_tagged | 39815ns | 39815ns | +4.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| valrepr_static | 36065ns | base | --- | [34376, 36415] | --- | --- | --- | --- |
| valrepr_nanbox | 35931ns | no significant difference | [-1035, +218]ns | [33527, 36297] | no | 0.6875 | 0.6875 | 0 |
| valrepr_tagged | 38016ns | +1896.9ns (+5.3%) | [+1345, +2075]ns | [35905, 38252] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | valrepr_static | valrepr_nanbox | valrepr_tagged |
|---|---|---|---|
| 1 | 32916ns | -4.2% | +3.0% |
| 2 | 36462ns | -1.8% | +4.7% |
| 3 | 35950ns | -1.2% | +5.8% |
| 4 | 35837ns | +0.7% | +5.8% |
| 5 | 36368ns | -0.4% | +5.4% |
| 6 | 36179ns | +0.6% | +5.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| valrepr_nanbox | 0.015 | ok |
| valrepr_static | -0.149 | ok |
| valrepr_tagged | -0.056 | ok |

**Consistency summary:**

- **valrepr_nanbox**: won 4/6, lost 2/6
- **valrepr_tagged**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| valrepr_nanbox | 160.2ns | 35251.5ns | 0.5% |  |
| valrepr_static | 164.4ns | 35618.7ns | 0.5% |  |
| valrepr_tagged | 199.0ns | 37391.2ns | 0.5% |  |

## Distribution (algo ns)

```
valrepr_nanbox (n=6, range 31518.3-36296.7 ns)
  31518.3 |####################
  31757.2 |
  31996.1 |
  32235.1 |
  32474.0 |
  32712.9 |
  32951.8 |
  33190.7 |
  33429.7 |
  33668.6 |
  33907.5 |
  34146.4 |
  34385.3 |
  34624.3 |
  34863.2 |
  35102.1 |
  35341.0 |####################
  35579.9 |####################
  35818.9 |
  36057.8 |########################################
  (0 below, 1 above range)

valrepr_static (n=6, range 32915.8-36415.2 ns)
  32915.8 |########################################
  33090.8 |
  33265.7 |
  33440.7 |
  33615.7 |
  33790.7 |
  33965.6 |
  34140.6 |
  34315.6 |
  34490.5 |
  34665.5 |
  34840.5 |
  35015.4 |
  35190.4 |
  35365.4 |
  35540.3 |
  35715.3 |########################################
  35890.3 |########################################
  36065.3 |########################################
  36240.2 |########################################
  (0 below, 1 above range)

valrepr_tagged (n=6, range 33900.0-38252.5 ns)
  33900.0 |#############
  34117.6 |
  34335.2 |
  34552.9 |
  34770.5 |
  34988.1 |
  35205.8 |
  35423.4 |
  35641.0 |
  35858.6 |
  36076.2 |
  36293.9 |
  36511.5 |
  36729.1 |
  36946.8 |
  37164.4 |
  37382.0 |
  37599.6 |
  37817.2 |########################################
  38034.9 |#############
  (0 below, 1 above range)

```
