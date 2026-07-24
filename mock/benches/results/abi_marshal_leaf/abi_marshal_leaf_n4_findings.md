# abi_marshal (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_leaf_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_leaf_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_leaf_marshal_null dominates: 6951% faster than the next best (abi_marshal_leaf_soa_native)

abi_marshal_leaf_marshal_null (20.66 us) leads abi_marshal_leaf_soa_native (1.46 ms) by 6951%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_leaf_marshal_null beats baseline by 99% (significant)

abi_marshal_leaf_marshal_null is -1.44 ms (99%) faster than baseline abi_marshal_leaf_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_leaf_soa_transposed is an outlier: 71.1x slower than the field

abi_marshal_leaf_soa_transposed (1.47 ms) is 71.1x the fastest (20.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_leaf_soa_native shows alternating (throttle bounce) (autocorr -0.65)

abi_marshal_leaf_soa_native's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_leaf_marshal_null} vs {abi_marshal_leaf_soa_native, abi_marshal_leaf_aos, abi_marshal_leaf_soa_transposed} (6951% apart)

The field splits into a fast tier {abi_marshal_leaf_marshal_null} and a slow tier {abi_marshal_leaf_soa_native, abi_marshal_leaf_aos, abi_marshal_leaf_soa_transposed} with a 6951% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 71.1x the fastest

Fastest abi_marshal_leaf_marshal_null (20.66 us) to slowest abi_marshal_leaf_soa_transposed (1.47 ms): 71.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_leaf_marshal_null** at 20659.3 ns median (-98.6% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 71.13x (fastest 20659.3 ns, slowest 1469443.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1463817ns | 1464734ns | 1459619ns | 1463677ns | 1466126ns | base |
| abi_marshal_leaf_marshal_null | 22895ns | 22991ns | 22090ns | 22702ns | 23587ns | -98.44% |
| abi_marshal_leaf_soa_native | 1458815ns | 1459216ns | 1455677ns | 1458587ns | 1460727ns | -0.34% |
| abi_marshal_leaf_soa_transposed | 1472509ns | 1471966ns | 1462996ns | 1471696ns | 1478485ns | +0.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1461247ns | 1457061ns | 1463635ns | base | 0.000 |
| abi_marshal_leaf_marshal_null | 20550ns | 19777ns | 21178ns | -98.59% | 0.000 |
| abi_marshal_leaf_soa_native | 1456282ns | 1453190ns | 1458159ns | -0.34% | 0.000 |
| abi_marshal_leaf_soa_transposed | 1470009ns | 1460596ns | 1475927ns | +0.60% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 37057.7 | 1460112.5 | 1461247.0 | n/a |
| abi_marshal_leaf_marshal_null | 28416.5 | 20587.0 | 20550.0 | n/a |
| abi_marshal_leaf_soa_native | 38169.2 | 1455342.4 | 1456282.5 | 0 |
| abi_marshal_leaf_soa_transposed | 37251.8 | 1468460.2 | 1470009.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_leaf_marshal_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_leaf_aos | 0.000 | 1.4% |
| abi_marshal_leaf_marshal_null | 0.000 | 95.7% |
| abi_marshal_leaf_soa_native | 0.000 | 1.4% |
| abi_marshal_leaf_soa_transposed | 0.000 | 1.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_leaf_aos | 1463817ns | 1463817ns | base |
| abi_marshal_leaf_marshal_null | 22895ns | 22895ns | -98.44% |
| abi_marshal_leaf_soa_native | 1458815ns | 1458815ns | -0.34% |
| abi_marshal_leaf_soa_transposed | 1472509ns | 1472509ns | +0.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1462264ns | base | --- | [1457842, 1463635] | --- | --- | --- | --- |
| abi_marshal_leaf_marshal_null | 20659ns | -1441510.0ns (-98.6%) | [-1443823, -1436758]ns | [19813, 21178] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_leaf_soa_native | 1456659ns | -4105.0ns (-0.3%) | [-6977, -3812]ns | [1454030, 1458159] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_leaf_soa_transposed | 1469443ns | +8826.6ns (+0.6%) | [+1023, +16438]ns | [1464658, 1475927] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_leaf_aos | abi_marshal_leaf_marshal_null | abi_marshal_leaf_soa_native | abi_marshal_leaf_soa_transposed |
|---|---|---|---|---|
| 1 | 1462652ns | -98.5% | -0.3% | +0.5% |
| 2 | 1457061ns | -98.6% | -0.3% | +0.9% |
| 3 | 1461875ns | -98.6% | -0.3% | +1.4% |
| 4 | 1463122ns | -98.6% | -0.5% | -0.2% |
| 5 | 1464148ns | -98.6% | -0.5% | +0.3% |
| 6 | 1458623ns | -98.6% | -0.3% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_leaf_aos | -0.245 | moderate- |
| abi_marshal_leaf_marshal_null | 0.236 | moderate+ |
| abi_marshal_leaf_soa_native | -0.648 | HIGH- (thermal bounce) |
| abi_marshal_leaf_soa_transposed | -0.423 | moderate- |

**Consistency summary:**

- **abi_marshal_leaf_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_leaf_soa_native**: won 6/6, lost 0/6
- **abi_marshal_leaf_soa_transposed**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 4421056.2ns | 1461247.0ns | 302.6% | HIGH |
| abi_marshal_leaf_marshal_null | 169547.4ns | 20550.0ns | 825.0% | HIGH |
| abi_marshal_leaf_soa_native | 4408928.3ns | 1456282.5ns | 302.8% | HIGH |
| abi_marshal_leaf_soa_transposed | 4445800.2ns | 1470009.4ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_marshal_leaf_aos (n=6, range 1457060.8-1463635.4 ns)
  1457060.8 |########################################
  1457389.5 |
  1457718.3 |
  1458047.0 |
  1458375.7 |########################################
  1458704.4 |
  1459033.2 |
  1459361.9 |
  1459690.6 |
  1460019.4 |
  1460348.1 |
  1460676.8 |
  1461005.6 |
  1461334.3 |
  1461663.0 |########################################
  1461991.8 |
  1462320.5 |
  1462649.2 |########################################
  1462977.9 |########################################
  1463306.7 |
  (0 below, 1 above range)

abi_marshal_leaf_marshal_null (n=6, range 19776.7-21177.9 ns)
  19776.7 |########################################
  19846.8 |########################################
  19916.8 |
  19986.9 |
  20056.9 |
  20127.0 |
  20197.1 |
  20267.1 |########################################
  20337.2 |
  20407.2 |
  20477.3 |
  20547.4 |
  20617.4 |
  20687.5 |
  20757.5 |
  20827.6 |
  20897.7 |
  20967.7 |########################################
  21037.8 |
  21107.8 |########################################
  (0 below, 1 above range)

abi_marshal_leaf_soa_native (n=6, range 1453190.4-1458158.5 ns)
  1453190.4 |########################################
  1453438.8 |
  1453687.2 |
  1453935.6 |
  1454184.0 |
  1454432.4 |
  1454680.8 |########################################
  1454929.3 |
  1455177.7 |
  1455426.1 |
  1455674.5 |
  1455922.9 |
  1456171.3 |########################################
  1456419.7 |
  1456668.1 |
  1456916.5 |########################################
  1457164.9 |
  1457413.3 |########################################
  1457661.7 |
  1457910.1 |
  (0 below, 1 above range)

abi_marshal_leaf_soa_transposed (n=6, range 1460595.8-1475926.6 ns)
  1460595.8 |####################
  1461362.3 |
  1462128.9 |
  1462895.4 |
  1463662.0 |
  1464428.5 |
  1465195.1 |
  1465961.6 |
  1466728.1 |
  1467494.7 |
  1468261.2 |########################################
  1469027.8 |
  1469794.3 |########################################
  1470560.9 |
  1471327.4 |
  1472093.9 |
  1472860.5 |
  1473627.0 |
  1474393.6 |
  1475160.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_leaf_aos**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_marshal_null**: bridge=821.5% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_native**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_transposed**: bridge=302.3% of algo (FFI overhead may distort results)
