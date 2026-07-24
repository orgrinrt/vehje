# abi_lifecycle (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_wideselect_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_wideselect_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_wideselect_null_entry dominates: 89802% faster than the next best (abi_lifecycle_wideselect_held_handle)

abi_lifecycle_wideselect_null_entry (2.28 us) leads abi_lifecycle_wideselect_held_handle (2.05 ms) by 89802%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_wideselect_null_entry beats baseline by 100% (significant)

abi_lifecycle_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_lifecycle_wideselect_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_wideselect_fresh_per_batch is an outlier: 944.6x slower than the field

abi_lifecycle_wideselect_fresh_per_batch (2.15 ms) is 944.6x the fastest (2.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_wideselect_null_entry} vs {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} (89802% apart)

The field splits into a fast tier {abi_lifecycle_wideselect_null_entry} and a slow tier {abi_lifecycle_wideselect_held_handle, abi_lifecycle_wideselect_fresh_per_column, abi_lifecycle_wideselect_fresh_per_batch} with a 89802% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 944.6x the fastest

Fastest abi_lifecycle_wideselect_null_entry (2.28 us) to slowest abi_lifecycle_wideselect_fresh_per_batch (2.15 ms): 944.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_wideselect_null_entry** at 2279.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 944.58x (fastest 2279.6 ns, slowest 2153262.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2208849ns | 2155819ns | 2146008ns | 2154706ns | 2321483ns | +7.64% |
| abi_lifecycle_wideselect_fresh_per_column | 2068771ns | 2069499ns | 2059645ns | 2067184ns | 2075715ns | +0.81% |
| abi_lifecycle_wideselect_held_handle | 2052064ns | 2051883ns | 2040043ns | 2049394ns | 2062079ns | base |
| abi_lifecycle_wideselect_null_entry | 4580ns | 4519ns | 4406ns | 4483ns | 4814ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2206248ns | 2143556ns | 2318734ns | +7.65% | 0.000 |
| abi_lifecycle_wideselect_fresh_per_column | 2066222ns | 2057202ns | 2073129ns | +0.81% | 0.000 |
| abi_lifecycle_wideselect_held_handle | 2049528ns | 2037513ns | 2059440ns | base | 0.000 |
| abi_lifecycle_wideselect_null_entry | 2304ns | 2208ns | 2422ns | -99.89% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 41839.7 | 2163765.8 | 2206247.6 | n/a |
| abi_lifecycle_wideselect_fresh_per_column | 40704.3 | 2065793.2 | 2066222.4 | n/a |
| abi_lifecycle_wideselect_held_handle | 38948.3 | 2049852.0 | 2049528.4 | n/a |
| abi_lifecycle_wideselect_null_entry | 28527.3 | 2411.6 | 2303.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_lifecycle_wideselect_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_wideselect_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_wideselect_held_handle | 0.000 | 0.1% |
| abi_lifecycle_wideselect_null_entry | 0.014 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 2208849ns | 2208849ns | +7.64% |
| abi_lifecycle_wideselect_fresh_per_column | 2068771ns | 2068771ns | +0.81% |
| abi_lifecycle_wideselect_held_handle | 2052064ns | 2052064ns | base |
| abi_lifecycle_wideselect_null_entry | 4580ns | 4580ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_wideselect_held_handle | 2049408ns | base | --- | [2039737, 2059440] | --- | --- | --- | --- |
| abi_lifecycle_wideselect_fresh_per_batch | 2153262ns | +107758.5ns (+5.3%) | [+92583, +269816]ns | [2146747, 2318734] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_wideselect_fresh_per_column | 2067014ns | +13776.7ns (+0.7%) | [+4992, +31313]ns | [2058524, 2073129] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_wideselect_null_entry | 2280ns | -2047049.0ns (-99.9%) | [-2057106, -2037519]ns | [2209, 2422] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_wideselect_held_handle | abi_lifecycle_wideselect_fresh_per_batch | abi_lifecycle_wideselect_fresh_per_column | abi_lifecycle_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2057182ns | +4.7% | +0.6% | -99.9% |
| 2 | 2061697ns | +4.3% | -0.1% | -99.9% |
| 3 | 2042942ns | +5.4% | +0.7% | -99.9% |
| 4 | 2037513ns | +5.2% | +1.4% | -99.9% |
| 5 | 2055875ns | +20.7% | +0.6% | -99.9% |
| 6 | 2041961ns | +5.6% | +1.7% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | -0.240 | moderate- |
| abi_lifecycle_wideselect_fresh_per_column | 0.188 | ok |
| abi_lifecycle_wideselect_held_handle | -0.065 | ok |
| abi_lifecycle_wideselect_null_entry | -0.499 | moderate- |

**Consistency summary:**

- **abi_lifecycle_wideselect_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_wideselect_fresh_per_column**: won 0/6, lost 5/6
- **abi_lifecycle_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_wideselect_fresh_per_batch | 6564562.4ns | 2206247.6ns | 297.5% | HIGH |
| abi_lifecycle_wideselect_fresh_per_column | 6240341.0ns | 2066222.4ns | 302.0% | HIGH |
| abi_lifecycle_wideselect_held_handle | 6189044.9ns | 2049528.4ns | 302.0% | HIGH |
| abi_lifecycle_wideselect_null_entry | 117105.6ns | 2303.6ns | 5083.6% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_wideselect_fresh_per_batch (n=6, range 2143556.2-2318733.8 ns)
  2143556.2 |##########################
  2152315.1 |########################################
  2161074.0 |
  2169832.8 |
  2178591.7 |
  2187350.6 |
  2196109.5 |
  2204868.4 |
  2213627.2 |
  2222386.1 |
  2231145.0 |
  2239903.9 |
  2248662.8 |
  2257421.6 |
  2266180.5 |
  2274939.4 |
  2283698.3 |
  2292457.2 |
  2301216.0 |
  2309974.9 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_fresh_per_column (n=6, range 2057202.1-2073128.8 ns)
  2057202.1 |########################################
  2057998.4 |
  2058794.8 |
  2059591.1 |########################################
  2060387.4 |
  2061183.8 |
  2061980.1 |
  2062776.4 |
  2063572.8 |
  2064369.1 |
  2065165.4 |
  2065961.8 |########################################
  2066758.1 |
  2067554.4 |########################################
  2068350.8 |
  2069147.1 |
  2069943.4 |########################################
  2070739.8 |
  2071536.1 |
  2072332.4 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_held_handle (n=6, range 2037513.3-2059439.6 ns)
  2037513.3 |####################
  2038609.6 |
  2039705.9 |
  2040802.2 |
  2041898.6 |########################################
  2042994.9 |
  2044091.2 |
  2045187.5 |
  2046283.8 |
  2047380.1 |
  2048476.5 |
  2049572.8 |
  2050669.1 |
  2051765.4 |
  2052861.7 |
  2053958.0 |
  2055054.3 |####################
  2056150.7 |####################
  2057247.0 |
  2058343.3 |
  (0 below, 1 above range)

abi_lifecycle_wideselect_null_entry (n=6, range 2207.5-2422.3 ns)
   2207.5 |########################################
   2218.2 |
   2229.0 |####################
   2239.7 |
   2250.5 |
   2261.2 |
   2271.9 |
   2282.7 |
   2293.4 |
   2304.2 |
   2314.9 |
   2325.6 |####################
   2336.4 |
   2347.1 |
   2357.9 |
   2368.6 |
   2379.3 |####################
   2390.1 |
   2400.8 |
   2411.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_wideselect_fresh_per_batch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_fresh_per_column**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_held_handle**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_wideselect_null_entry**: bridge=5089.0% of algo (FFI overhead may distort results)
