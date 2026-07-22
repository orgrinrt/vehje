# Value representation: static/raw vs runtime-tagged vs NaN-boxed (dynamic-typing cost)

3 variants, 6 samples per variant.
Baseline: **valrepr_static**

## Highlights

Baseline for all deltas below: **valrepr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (139 ns) is smaller than the fastest variant's own run-to-run std-dev (256 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (valrepr_static)

The baseline valrepr_static is the fastest (2.09 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader valrepr_static vs stability leader valrepr_tagged (+7% speed for 3.9x steadier)

valrepr_static is fastest (2.09 us, CV 12.2%); valrepr_tagged gives up 6.7% median for 3.9x lower variance (CV 3.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (valrepr_static) is the fastest** at 2093.8 ns median
- Spread: 1.07x (fastest 2093.8 ns, slowest 2233.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| valrepr_nanbox | 4685ns | 4411ns | 4273ns | 4398ns | 5321ns | +2.64% |
| valrepr_static | 4564ns | 4336ns | 3988ns | 4329ns | 5205ns | base |
| valrepr_tagged | 4553ns | 4485ns | 4452ns | 4477ns | 4718ns | -0.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| valrepr_nanbox | 2316ns | 2077ns | 2707ns | +5.22% | 0.028 |
| valrepr_static | 2201ns | 1913ns | 2526ns | base | 0.029 |
| valrepr_tagged | 2268ns | 2209ns | 2361ns | +3.03% | 0.028 |

## Performance model

- Peak throughput: **0.033 Gops/s** (valrepr_static; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| valrepr_nanbox | 0.030 | 89.5% |
| valrepr_static | 0.031 | 91.4% |
| valrepr_tagged | 0.029 | 85.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| valrepr_nanbox | 4685ns | 4685ns | +2.64% |
| valrepr_static | 4564ns | 4564ns | base |
| valrepr_tagged | 4553ns | 4553ns | -0.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| valrepr_static | 2094ns | base | --- | [1984, 2526] | --- | --- | --- | --- |
| valrepr_nanbox | 2138ns | no significant difference | [-301, +529]ns | [2103, 2707] | no | 0.6875 | 0.6875 | 0 |
| valrepr_tagged | 2233ns | no significant difference | [-206, +277]ns | [2209, 2361] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | valrepr_static | valrepr_nanbox | valrepr_tagged |
|---|---|---|---|
| 1 | 1913ns | +12.0% | +15.5% |
| 2 | 2362ns | +35.1% | +1.4% |
| 3 | 2070ns | +3.1% | +12.4% |
| 4 | 2690ns | -20.8% | -16.6% |
| 5 | 2054ns | +8.3% | +8.2% |
| 6 | 2118ns | -1.9% | +4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| valrepr_nanbox | -0.256 | moderate- |
| valrepr_static | -0.487 | moderate- |
| valrepr_tagged | 0.085 | ok |

**Consistency summary:**

- **valrepr_nanbox**: won 2/6, lost 4/6
- **valrepr_tagged**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| valrepr_nanbox | 52.7ns | 2316.1ns | 2.3% |  |
| valrepr_static | 57.6ns | 2201.1ns | 2.6% |  |
| valrepr_tagged | 90.1ns | 2267.7ns | 4.0% |  |

## Distribution (algo ns)

```
valrepr_nanbox (n=6, range 2077.1-2707.2 ns)
   2077.1 |####################
   2108.6 |########################################
   2140.1 |####################
   2171.6 |
   2203.1 |####################
   2234.6 |
   2266.1 |
   2297.7 |
   2329.2 |
   2360.7 |
   2392.2 |
   2423.7 |
   2455.2 |
   2486.7 |
   2518.2 |
   2549.7 |
   2581.2 |
   2612.7 |
   2644.2 |
   2675.7 |
  (0 below, 1 above range)

valrepr_static (n=6, range 1913.3-2526.1 ns)
   1913.3 |########################################
   1943.9 |
   1974.6 |
   2005.2 |
   2035.8 |########################################
   2066.5 |########################################
   2097.1 |########################################
   2127.8 |
   2158.4 |
   2189.0 |
   2219.7 |
   2250.3 |
   2280.9 |
   2311.6 |
   2342.2 |########################################
   2372.9 |
   2403.5 |
   2434.1 |
   2464.8 |
   2495.4 |
  (0 below, 1 above range)

valrepr_tagged (n=6, range 2208.8-2360.8 ns)
   2208.8 |########################################
   2216.4 |####################
   2224.0 |
   2231.6 |
   2239.2 |####################
   2246.8 |
   2254.4 |
   2262.0 |
   2269.6 |
   2277.2 |
   2284.8 |
   2292.4 |
   2300.0 |
   2307.6 |
   2315.2 |
   2322.8 |####################
   2330.4 |
   2338.0 |
   2345.6 |
   2353.2 |
  (0 below, 1 above range)

```
