# abi_zig_entry (leaf)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_leaf_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_leaf_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_leaf_zig_null dominates: 62025% faster than the next best (abi_zig_entry_leaf_zig_dispatch)

abi_zig_entry_leaf_zig_null (2.30 us) leads abi_zig_entry_leaf_zig_dispatch (1.43 ms) by 62025%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_leaf_zig_null beats baseline by 100% (significant)

abi_zig_entry_leaf_zig_null is -1.43 ms (100%) faster than baseline abi_zig_entry_leaf_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_leaf_zig_tail_runtime_w is an outlier: 1410.2x slower than the field

abi_zig_entry_leaf_zig_tail_runtime_w (3.25 ms) is 1410.2x the fastest (2.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_leaf_zig_null shows alternating (throttle bounce) (autocorr -0.57)

abi_zig_entry_leaf_zig_null's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_leaf_zig_null} vs {abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} (62025% apart)

The field splits into a fast tier {abi_zig_entry_leaf_zig_null} and a slow tier {abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} with a 62025% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1410.2x the fastest

Fastest abi_zig_entry_leaf_zig_null (2.30 us) to slowest abi_zig_entry_leaf_zig_tail_runtime_w (3.25 ms): 1410.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_leaf_zig_null** at 2304.4 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1410.22x (fastest 2304.4 ns, slowest 3249640.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1437399ns | 1437484ns | 1433356ns | 1436418ns | 1440892ns | +0.14% |
| abi_zig_entry_leaf_zig_dispatch | 1437511ns | 1434122ns | 1433456ns | 1434061ns | 1444714ns | +0.14% |
| abi_zig_entry_leaf_zig_null | 4614ns | 4615ns | 4557ns | 4606ns | 4654ns | -99.68% |
| abi_zig_entry_leaf_zig_per_w_set | 1448061ns | 1434759ns | 1432436ns | 1434745ns | 1475846ns | +0.88% |
| abi_zig_entry_leaf_zig_runtime_w | 1435455ns | 1434601ns | 1433730ns | 1434336ns | 1437998ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3196504ns | 3250855ns | 3079799ns | 3195356ns | 3256579ns | +122.68% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3251714ns | 3252455ns | 3245136ns | 3251856ns | 3254791ns | +126.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1434815ns | 1430806ns | 1438262ns | +0.13% | 0.000 |
| abi_zig_entry_leaf_zig_dispatch | 1434870ns | 1430865ns | 1441897ns | +0.14% | 0.000 |
| abi_zig_entry_leaf_zig_null | 2303ns | 2261ns | 2325ns | -99.84% | 0.014 |
| abi_zig_entry_leaf_zig_per_w_set | 1445394ns | 1429923ns | 1473053ns | +0.87% | 0.000 |
| abi_zig_entry_leaf_zig_runtime_w | 1432904ns | 1431238ns | 1435362ns | base | 0.000 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3193584ns | 3076685ns | 3253687ns | +122.87% | 0.000 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3248820ns | 3242340ns | 3251786ns | +126.73% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 172379.7 | 1434929.4 | 1434815.2 | n/a |
| abi_zig_entry_leaf_zig_dispatch | 175601.9 | 1434806.0 | 1434870.1 | n/a |
| abi_zig_entry_leaf_zig_null | 154934.1 | 2542.5 | 2302.8 | n/a |
| abi_zig_entry_leaf_zig_per_w_set | 181044.2 | 1436715.5 | 1445394.0 | n/a |
| abi_zig_entry_leaf_zig_runtime_w | 170637.2 | 1432374.8 | 1432903.7 | n/a |
| abi_zig_entry_leaf_zig_tail_dispatch | 191099.6 | 3195063.7 | 3193584.1 | n/a |
| abi_zig_entry_leaf_zig_tail_runtime_w | 192796.2 | 3250129.2 | 3248819.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_zig_entry_leaf_zig_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_null | 0.014 | 98.1% |
| abi_zig_entry_leaf_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1437399ns | 1437399ns | +0.14% |
| abi_zig_entry_leaf_zig_dispatch | 1437511ns | 1437511ns | +0.14% |
| abi_zig_entry_leaf_zig_null | 4614ns | 4614ns | -99.68% |
| abi_zig_entry_leaf_zig_per_w_set | 1448061ns | 1448061ns | +0.88% |
| abi_zig_entry_leaf_zig_runtime_w | 1435455ns | 1435455ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3196504ns | 3196504ns | +122.68% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3251714ns | 3251714ns | +126.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_runtime_w | 1432089ns | base | --- | [1431261, 1435362] | --- | --- | --- | --- |
| abi_zig_entry_leaf_zig_anchor | 1434914ns | no significant difference | [-938, +5015]ns | [1431269, 1438262] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_leaf_zig_dispatch | 1431588ns | no significant difference | [-2328, +7927]ns | [1431125, 1441897] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_null | 2304ns | -1429810.2ns (-99.8%) | [-1433036, -1428956]ns | [2279, 2325] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_per_w_set | 1432143ns | no significant difference | [-2703, +41276]ns | [1430986, 1473053] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3247965ns | +1813995.4ns (+126.7%) | [+1645639, +1822407]ns | [3079100, 3253687] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3249640ns | +1816643.8ns (+126.9%) | [+1810865, +1820238]ns | [3245032, 3251786] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_leaf_zig_runtime_w | abi_zig_entry_leaf_zig_anchor | abi_zig_entry_leaf_zig_dispatch | abi_zig_entry_leaf_zig_null | abi_zig_entry_leaf_zig_per_w_set | abi_zig_entry_leaf_zig_tail_dispatch | abi_zig_entry_leaf_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1434068ns | +0.0% | -0.2% | -99.8% | -0.1% | +114.9% | +126.7% |
| 2 | 1432854ns | -0.1% | -0.1% | -99.8% | -0.1% | +114.7% | +126.3% |
| 3 | 1436656ns | +0.2% | +0.8% | -99.8% | -0.2% | +126.2% | +126.1% |
| 4 | 1431238ns | +0.3% | +0.0% | -99.8% | +0.1% | +127.6% | +127.1% |
| 5 | 1431324ns | +0.4% | +0.0% | -99.8% | -0.1% | +127.1% | +127.2% |
| 6 | 1431284ns | +0.0% | +0.3% | -99.8% | +5.7% | +126.8% | +126.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | -0.336 | moderate- |
| abi_zig_entry_leaf_zig_dispatch | -0.288 | moderate- |
| abi_zig_entry_leaf_zig_null | -0.569 | HIGH- (thermal bounce) |
| abi_zig_entry_leaf_zig_per_w_set | -0.062 | ok |
| abi_zig_entry_leaf_zig_runtime_w | -0.056 | ok |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.423 | moderate+ |
| abi_zig_entry_leaf_zig_tail_runtime_w | -0.188 | ok |

**Consistency summary:**

- **abi_zig_entry_leaf_zig_anchor**: won 1/6, lost 3/6
- **abi_zig_entry_leaf_zig_dispatch**: won 2/6, lost 2/6
- **abi_zig_entry_leaf_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_leaf_zig_per_w_set**: won 2/6, lost 1/6
- **abi_zig_entry_leaf_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_leaf_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 4540897.4ns | 1434815.2ns | 316.5% | HIGH |
| abi_zig_entry_leaf_zig_dispatch | 4546644.6ns | 1434870.1ns | 316.9% | HIGH |
| abi_zig_entry_leaf_zig_null | 296759.6ns | 2302.8ns | 12887.0% | HIGH |
| abi_zig_entry_leaf_zig_per_w_set | 4558982.4ns | 1445394.0ns | 315.4% | HIGH |
| abi_zig_entry_leaf_zig_runtime_w | 4533657.8ns | 1432903.7ns | 316.4% | HIGH |
| abi_zig_entry_leaf_zig_tail_dispatch | 9847600.1ns | 3193584.1ns | 308.4% | HIGH |
| abi_zig_entry_leaf_zig_tail_runtime_w | 10014933.6ns | 3248819.6ns | 308.3% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_leaf_zig_anchor (n=6, range 1430805.8-1438262.1 ns)
  1430805.8 |########################################
  1431178.6 |
  1431551.4 |########################################
  1431924.2 |
  1432297.1 |
  1432669.9 |
  1433042.7 |
  1433415.5 |
  1433788.3 |
  1434161.1 |########################################
  1434533.9 |
  1434906.7 |
  1435279.6 |########################################
  1435652.4 |
  1436025.2 |
  1436398.0 |
  1436770.8 |########################################
  1437143.6 |
  1437516.4 |
  1437889.2 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_dispatch (n=6, range 1430865.4-1441896.6 ns)
  1430865.4 |########################################
  1431417.0 |#############
  1431968.5 |
  1432520.1 |
  1433071.6 |
  1433623.2 |
  1434174.8 |
  1434726.3 |
  1435277.9 |#############
  1435829.5 |
  1436381.0 |
  1436932.6 |
  1437484.1 |
  1438035.7 |
  1438587.3 |
  1439138.8 |
  1439690.4 |
  1440242.0 |
  1440793.5 |
  1441345.1 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_null (n=6, range 2260.8-2325.2 ns)
   2260.8 |####################
   2264.0 |
   2267.2 |
   2270.5 |
   2273.7 |
   2276.9 |
   2280.1 |
   2283.4 |
   2286.6 |
   2289.8 |
   2293.0 |
   2296.2 |####################
   2299.5 |
   2302.7 |########################################
   2305.9 |
   2309.1 |####################
   2312.4 |
   2315.6 |
   2318.8 |
   2322.0 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_per_w_set (n=6, range 1429922.9-1473052.9 ns)
  1429922.9 |##########################
  1432079.4 |########################################
  1434235.9 |
  1436392.4 |
  1438548.9 |
  1440705.4 |
  1442861.9 |
  1445018.4 |
  1447174.9 |
  1449331.4 |
  1451487.9 |
  1453644.4 |
  1455800.9 |
  1457957.4 |
  1460113.9 |
  1462270.4 |
  1464426.9 |
  1466583.4 |
  1468739.9 |
  1470896.4 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_runtime_w (n=6, range 1431237.5-1435361.6 ns)
  1431237.5 |########################################
  1431443.7 |
  1431649.9 |
  1431856.1 |
  1432062.3 |
  1432268.5 |
  1432474.7 |
  1432681.0 |#############
  1432887.2 |
  1433093.4 |
  1433299.6 |
  1433505.8 |
  1433712.0 |
  1433918.2 |#############
  1434124.4 |
  1434330.6 |
  1434536.8 |
  1434743.0 |
  1434949.2 |
  1435155.4 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_dispatch (n=6, range 3076684.6-3253687.5 ns)
  3076684.6 |##########################
  3085534.7 |
  3094384.9 |
  3103235.0 |
  3112085.2 |
  3120935.3 |
  3129785.5 |
  3138635.6 |
  3147485.7 |
  3156335.9 |
  3165186.0 |
  3174036.2 |
  3182886.3 |
  3191736.5 |
  3200586.6 |
  3209436.7 |
  3218286.9 |
  3227137.0 |
  3235987.2 |
  3244837.3 |########################################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_runtime_w (n=6, range 3242340.0-3251786.5 ns)
  3242340.0 |########################################
  3242812.3 |
  3243284.6 |
  3243757.0 |
  3244229.3 |
  3244701.6 |
  3245173.9 |
  3245646.3 |
  3246118.6 |
  3246590.9 |
  3247063.2 |
  3247535.5 |########################################
  3248007.9 |
  3248480.2 |########################################
  3248952.5 |
  3249424.8 |
  3249897.2 |
  3250369.5 |########################################
  3250841.8 |########################################
  3251314.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_leaf_zig_anchor**: bridge=316.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_dispatch**: bridge=316.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_null**: bridge=12881.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_per_w_set**: bridge=316.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_runtime_w**: bridge=316.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_dispatch**: bridge=308.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_runtime_w**: bridge=308.2% of algo (FFI overhead may distort results)
