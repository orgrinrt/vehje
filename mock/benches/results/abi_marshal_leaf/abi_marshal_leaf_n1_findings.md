# abi_marshal (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_leaf_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_leaf_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_leaf_marshal_null dominates: 14101% faster than the next best (abi_marshal_leaf_soa_native)

abi_marshal_leaf_marshal_null (10.17 us) leads abi_marshal_leaf_soa_native (1.44 ms) by 14101%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_leaf_marshal_null beats baseline by 99% (significant)

abi_marshal_leaf_marshal_null is -1.44 ms (99%) faster than baseline abi_marshal_leaf_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_leaf_soa_transposed is an outlier: 142.9x slower than the field

abi_marshal_leaf_soa_transposed (1.45 ms) is 142.9x the fastest (10.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_leaf_soa_native shows alternating (throttle bounce) (autocorr -0.71)

abi_marshal_leaf_soa_native's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_leaf_marshal_null} vs {abi_marshal_leaf_soa_native, abi_marshal_leaf_aos, abi_marshal_leaf_soa_transposed} (14101% apart)

The field splits into a fast tier {abi_marshal_leaf_marshal_null} and a slow tier {abi_marshal_leaf_soa_native, abi_marshal_leaf_aos, abi_marshal_leaf_soa_transposed} with a 14101% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 142.9x the fastest

Fastest abi_marshal_leaf_marshal_null (10.17 us) to slowest abi_marshal_leaf_soa_transposed (1.45 ms): 142.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_leaf_marshal_null** at 10174.8 ns median (-99.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 142.94x (fastest 10174.8 ns, slowest 1454355.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1452837ns | 1453984ns | 1448006ns | 1452774ns | 1455346ns | base |
| abi_marshal_leaf_marshal_null | 12570ns | 12463ns | 12103ns | 12420ns | 13029ns | -99.13% |
| abi_marshal_leaf_soa_native | 1447121ns | 1447465ns | 1438767ns | 1446981ns | 1451508ns | -0.39% |
| abi_marshal_leaf_soa_transposed | 1456306ns | 1456884ns | 1451671ns | 1455730ns | 1459486ns | +0.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1450302ns | 1445530ns | 1452848ns | base | 0.000 |
| abi_marshal_leaf_marshal_null | 10267ns | 9940ns | 10632ns | -99.29% | 0.000 |
| abi_marshal_leaf_soa_native | 1444656ns | 1436458ns | 1448994ns | -0.39% | 0.000 |
| abi_marshal_leaf_soa_transposed | 1453823ns | 1449279ns | 1457003ns | +0.24% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 36867.8 | 1451283.4 | 1450301.5 | n/a |
| abi_marshal_leaf_marshal_null | 28456.1 | 10443.8 | 10267.2 | n/a |
| abi_marshal_leaf_soa_native | 36665.6 | 1444886.3 | 1444655.8 | n/a |
| abi_marshal_leaf_soa_transposed | 37513.1 | 1455074.7 | 1453822.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_leaf_marshal_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_leaf_aos | 0.000 | 0.7% |
| abi_marshal_leaf_marshal_null | 0.000 | 97.7% |
| abi_marshal_leaf_soa_native | 0.000 | 0.7% |
| abi_marshal_leaf_soa_transposed | 0.000 | 0.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_leaf_aos | 1452837ns | 1452837ns | base |
| abi_marshal_leaf_marshal_null | 12570ns | 12570ns | -99.13% |
| abi_marshal_leaf_soa_native | 1447121ns | 1447121ns | -0.39% |
| abi_marshal_leaf_soa_transposed | 1456306ns | 1456306ns | +0.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_leaf_aos | 1451387ns | base | --- | [1446670, 1452848] | --- | --- | --- | --- |
| abi_marshal_leaf_marshal_null | 10175ns | -1441392.5ns (-99.3%) | [-1442390, -1436321]ns | [9995, 10632] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_leaf_soa_native | 1444951ns | no significant difference | [-11569, +2325]ns | [1440022, 1448994] | no | 0.6875 | 0.6875 | 0 |
| abi_marshal_leaf_soa_transposed | 1454356ns | no significant difference | [-1278, +10333]ns | [1450109, 1457003] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_leaf_aos | abi_marshal_leaf_marshal_null | abi_marshal_leaf_soa_native | abi_marshal_leaf_soa_transposed |
|---|---|---|---|---|
| 1 | 1445530ns | -99.3% | +0.3% | +0.8% |
| 2 | 1450995ns | -99.3% | -1.0% | -0.0% |
| 3 | 1447810ns | -99.3% | +0.1% | +0.6% |
| 4 | 1451780ns | -99.3% | -0.6% | -0.2% |
| 5 | 1453519ns | -99.3% | -0.6% | +0.0% |
| 6 | 1452176ns | -99.3% | -0.5% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_leaf_aos | 0.045 | ok |
| abi_marshal_leaf_marshal_null | -0.555 | HIGH- (thermal bounce) |
| abi_marshal_leaf_soa_native | -0.710 | HIGH- (thermal bounce) |
| abi_marshal_leaf_soa_transposed | -0.561 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_marshal_leaf_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_leaf_soa_native**: won 4/6, lost 1/6
- **abi_marshal_leaf_soa_transposed**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_leaf_aos | 4393343.0ns | 1450301.5ns | 302.9% | HIGH |
| abi_marshal_leaf_marshal_null | 141614.9ns | 10267.2ns | 1379.3% | HIGH |
| abi_marshal_leaf_soa_native | 4373752.7ns | 1444655.8ns | 302.8% | HIGH |
| abi_marshal_leaf_soa_transposed | 4403339.7ns | 1453822.6ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_marshal_leaf_aos (n=6, range 1445529.6-1452847.5 ns)
  1445529.6 |########################################
  1445895.5 |
  1446261.4 |
  1446627.3 |
  1446993.2 |
  1447359.1 |
  1447725.0 |########################################
  1448090.9 |
  1448456.8 |
  1448822.7 |
  1449188.6 |
  1449554.4 |
  1449920.3 |
  1450286.2 |
  1450652.1 |########################################
  1451018.0 |
  1451383.9 |
  1451749.8 |########################################
  1452115.7 |########################################
  1452481.6 |
  (0 below, 1 above range)

abi_marshal_leaf_marshal_null (n=6, range 9940.4-10632.1 ns)
   9940.4 |####################
   9975.0 |
  10009.6 |
  10044.2 |####################
  10078.7 |
  10113.3 |
  10147.9 |########################################
  10182.5 |
  10217.1 |
  10251.7 |
  10286.2 |
  10320.8 |
  10355.4 |
  10390.0 |
  10424.6 |
  10459.2 |
  10493.8 |####################
  10528.3 |
  10562.9 |
  10597.5 |
  (0 below, 1 above range)

abi_marshal_leaf_soa_native (n=6, range 1436457.9-1448994.4 ns)
  1436457.9 |####################
  1437084.7 |
  1437711.5 |
  1438338.4 |
  1438965.2 |
  1439592.0 |
  1440218.8 |
  1440845.7 |
  1441472.5 |
  1442099.3 |
  1442726.1 |
  1443353.0 |####################
  1443979.8 |
  1444606.6 |########################################
  1445233.4 |
  1445860.3 |
  1446487.1 |
  1447113.9 |
  1447740.8 |
  1448367.6 |####################
  (0 below, 1 above range)

abi_marshal_leaf_soa_transposed (n=6, range 1449279.2-1457003.1 ns)
  1449279.2 |########################################
  1449665.4 |
  1450051.6 |
  1450437.8 |
  1450824.0 |########################################
  1451210.2 |
  1451596.4 |
  1451982.6 |
  1452368.8 |
  1452755.0 |
  1453141.1 |
  1453527.3 |########################################
  1453913.5 |
  1454299.7 |
  1454685.9 |########################################
  1455072.1 |
  1455458.3 |
  1455844.5 |########################################
  1456230.7 |
  1456616.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_leaf_aos**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_marshal_null**: bridge=1384.0% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_native**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_marshal_leaf_soa_transposed**: bridge=302.8% of algo (FFI overhead may distort results)
