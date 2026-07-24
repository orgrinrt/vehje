# abi_entry_form (tight)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_tight_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_tight_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_tight_null_entry dominates: 51290% faster than the next best (abi_entry_form_tight_dispatch_table)

abi_entry_form_tight_null_entry (3.97 us) leads abi_entry_form_tight_dispatch_table (2.04 ms) by 51290%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_tight_null_entry beats baseline by 100% (significant)

abi_entry_form_tight_null_entry is -2.08 ms (100%) faster than baseline abi_entry_form_tight_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_tight_scalar_anchor is an outlier: 524.6x slower than the field

abi_entry_form_tight_scalar_anchor (2.08 ms) is 524.6x the fastest (3.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_tight_null_entry} vs {abi_entry_form_tight_dispatch_table, abi_entry_form_tight_per_w_set, abi_entry_form_tight_runtime_w, abi_entry_form_tight_scalar_anchor} (51290% apart)

The field splits into a fast tier {abi_entry_form_tight_null_entry} and a slow tier {abi_entry_form_tight_dispatch_table, abi_entry_form_tight_per_w_set, abi_entry_form_tight_runtime_w, abi_entry_form_tight_scalar_anchor} with a 51290% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 524.6x the fastest

Fastest abi_entry_form_tight_null_entry (3.97 us) to slowest abi_entry_form_tight_scalar_anchor (2.08 ms): 524.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_tight_null_entry** at 3972.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 524.57x (fastest 3972.9 ns, slowest 2084049.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2095331ns | 2045262ns | 2028860ns | 2042267ns | 2208163ns | +0.08% |
| abi_entry_form_tight_null_entry | 6284ns | 6233ns | 6095ns | 6192ns | 6517ns | -99.70% |
| abi_entry_form_tight_per_w_set | 2076348ns | 2049762ns | 2036400ns | 2046545ns | 2141028ns | -0.82% |
| abi_entry_form_tight_runtime_w | 2093571ns | 2086399ns | 2041622ns | 2071689ns | 2152369ns | base |
| abi_entry_form_tight_scalar_anchor | 2130587ns | 2087941ns | 2055963ns | 2077910ns | 2246913ns | +1.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2091753ns | 2026384ns | 2203971ns | +0.08% | 0.000 |
| abi_entry_form_tight_null_entry | 3987ns | 3880ns | 4106ns | -99.81% | 0.001 |
| abi_entry_form_tight_per_w_set | 2072898ns | 2033597ns | 2137015ns | -0.82% | 0.000 |
| abi_entry_form_tight_runtime_w | 2090053ns | 2038521ns | 2148650ns | base | 0.000 |
| abi_entry_form_tight_scalar_anchor | 2126958ns | 2052501ns | 2243365ns | +1.77% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 67154.7 | 2105992.4 | 2091753.5 | n/a |
| abi_entry_form_tight_null_entry | 28185.8 | 4158.4 | 3987.3 | n/a |
| abi_entry_form_tight_per_w_set | 71041.2 | 2073057.9 | 2072898.3 | 0 |
| abi_entry_form_tight_runtime_w | 66911.9 | 2108222.2 | 2090053.4 | n/a |
| abi_entry_form_tight_scalar_anchor | 74580.2 | 2082691.4 | 2126957.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_tight_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_tight_null_entry | 0.001 | 97.7% |
| abi_entry_form_tight_per_w_set | 0.000 | 0.2% |
| abi_entry_form_tight_runtime_w | 0.000 | 0.2% |
| abi_entry_form_tight_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2095331ns | 2095331ns | +0.08% |
| abi_entry_form_tight_null_entry | 6284ns | 6284ns | -99.70% |
| abi_entry_form_tight_per_w_set | 2076348ns | 2076348ns | -0.82% |
| abi_entry_form_tight_runtime_w | 2093571ns | 2093571ns | base |
| abi_entry_form_tight_scalar_anchor | 2130587ns | 2130587ns | +1.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_tight_runtime_w | 2082729ns | base | --- | [2038781, 2148650] | --- | --- | --- | --- |
| abi_entry_form_tight_dispatch_table | 2041680ns | no significant difference | [-98541, +105490]ns | [2029609, 2203971] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_tight_null_entry | 3973ns | -2078671.9ns (-99.8%) | [-2144767, -2034760]ns | [3883, 4106] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_tight_per_w_set | 2046339ns | no significant difference | [-97557, +42605]ns | [2035341, 2137015] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_tight_scalar_anchor | 2084050ns | no significant difference | [-64557, +150343]ns | [2053459, 2243365] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_tight_runtime_w | abi_entry_form_tight_dispatch_table | abi_entry_form_tight_null_entry | abi_entry_form_tight_per_w_set | abi_entry_form_tight_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2038521ns | +0.1% | -99.8% | +0.6% | +2.3% |
| 2 | 2118273ns | -3.6% | -99.8% | -3.8% | -1.6% |
| 3 | 2147522ns | -5.6% | -99.8% | -5.3% | +11.8% |
| 4 | 2149778ns | +9.6% | -99.8% | +2.7% | -4.4% |
| 5 | 2047184ns | +0.2% | -99.8% | -0.3% | +0.3% |
| 6 | 2039041ns | -0.3% | -99.8% | +1.4% | +2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_tight_dispatch_table | -0.235 | moderate- |
| abi_entry_form_tight_null_entry | -0.125 | ok |
| abi_entry_form_tight_per_w_set | -0.317 | moderate- |
| abi_entry_form_tight_runtime_w | 0.219 | moderate+ |
| abi_entry_form_tight_scalar_anchor | -0.231 | moderate- |

**Consistency summary:**

- **abi_entry_form_tight_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_tight_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_tight_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_tight_scalar_anchor**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 6393409.8ns | 2091753.5ns | 305.6% | HIGH |
| abi_entry_form_tight_null_entry | 122932.5ns | 3987.3ns | 3083.1% | HIGH |
| abi_entry_form_tight_per_w_set | 6308295.6ns | 2072898.3ns | 304.3% | HIGH |
| abi_entry_form_tight_runtime_w | 6374058.5ns | 2090053.4ns | 305.0% | HIGH |
| abi_entry_form_tight_scalar_anchor | 6366520.7ns | 2126957.8ns | 299.3% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_tight_dispatch_table (n=6, range 2026384.2-2203971.5 ns)
  2026384.2 |########################################
  2035263.6 |########################################
  2044142.9 |####################
  2053022.3 |
  2061901.6 |
  2070781.0 |
  2079660.4 |
  2088539.7 |
  2097419.1 |
  2106298.5 |
  2115177.8 |
  2124057.2 |
  2132936.6 |
  2141815.9 |
  2150695.3 |
  2159574.6 |
  2168454.0 |
  2177333.4 |
  2186212.7 |
  2195092.1 |
  (0 below, 1 above range)

abi_entry_form_tight_null_entry (n=6, range 3880.4-4105.6 ns)
   3880.4 |########################################
   3891.7 |
   3902.9 |
   3914.2 |
   3925.4 |
   3936.7 |
   3948.0 |
   3959.2 |####################
   3970.5 |
   3981.7 |####################
   3993.0 |
   4004.3 |
   4015.5 |
   4026.8 |
   4038.0 |
   4049.3 |
   4060.6 |####################
   4071.8 |
   4083.1 |
   4094.3 |
  (0 below, 1 above range)

abi_entry_form_tight_per_w_set (n=6, range 2033597.1-2137014.8 ns)
  2033597.1 |########################################
  2038768.0 |####################
  2043938.9 |
  2049109.8 |####################
  2054280.6 |
  2059451.5 |
  2064622.4 |####################
  2069793.3 |
  2074964.2 |
  2080135.1 |
  2085305.9 |
  2090476.8 |
  2095647.7 |
  2100818.6 |
  2105989.5 |
  2111160.4 |
  2116331.3 |
  2121502.1 |
  2126673.0 |
  2131843.9 |
  (0 below, 1 above range)

abi_entry_form_tight_runtime_w (n=6, range 2038521.2-2148650.4 ns)
  2038521.2 |########################################
  2044027.7 |####################
  2049534.1 |
  2055040.6 |
  2060547.0 |
  2066053.5 |
  2071560.0 |
  2077066.4 |
  2082572.9 |
  2088079.3 |
  2093585.8 |
  2099092.3 |
  2104598.7 |
  2110105.2 |
  2115611.6 |####################
  2121118.1 |
  2126624.6 |
  2132131.0 |
  2137637.5 |
  2143143.9 |####################
  (0 below, 1 above range)

abi_entry_form_tight_scalar_anchor (n=6, range 2052501.2-2243364.8 ns)
  2052501.2 |##########################
  2062044.4 |
  2071587.6 |
  2081130.7 |########################################
  2090673.9 |
  2100217.1 |
  2109760.3 |
  2119303.5 |
  2128846.6 |
  2138389.8 |
  2147933.0 |
  2157476.2 |
  2167019.4 |
  2176562.5 |
  2186105.7 |
  2195648.9 |
  2205192.1 |
  2214735.3 |
  2224278.4 |
  2233821.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_tight_dispatch_table**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_null_entry**: bridge=3087.8% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_per_w_set**: bridge=304.9% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_runtime_w**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_scalar_anchor**: bridge=303.4% of algo (FFI overhead may distort results)
