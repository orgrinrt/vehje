# abi_marshal (tight)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_tight_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_tight_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_tight_marshal_null dominates: 16702% faster than the next best (abi_marshal_tight_soa_native)

abi_marshal_tight_marshal_null (11.97 us) leads abi_marshal_tight_soa_native (2.01 ms) by 16702%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_tight_marshal_null beats baseline by 99% (significant)

abi_marshal_tight_marshal_null is -2.01 ms (99%) faster than baseline abi_marshal_tight_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_tight_soa_transposed is an outlier: 169.4x slower than the field

abi_marshal_tight_soa_transposed (2.03 ms) is 169.4x the fastest (11.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_tight_marshal_null shows alternating (throttle bounce) (autocorr -0.58)

abi_marshal_tight_marshal_null's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_tight_marshal_null} vs {abi_marshal_tight_soa_native, abi_marshal_tight_aos, abi_marshal_tight_soa_transposed} (16702% apart)

The field splits into a fast tier {abi_marshal_tight_marshal_null} and a slow tier {abi_marshal_tight_soa_native, abi_marshal_tight_aos, abi_marshal_tight_soa_transposed} with a 16702% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 169.4x the fastest

Fastest abi_marshal_tight_marshal_null (11.97 us) to slowest abi_marshal_tight_soa_transposed (2.03 ms): 169.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_tight_marshal_null** at 11974.4 ns median (-99.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 169.39x (fastest 11974.4 ns, slowest 2028290.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2034339ns | 2024778ns | 2007159ns | 2021419ns | 2067310ns | base |
| abi_marshal_tight_marshal_null | 14388ns | 14222ns | 13865ns | 14171ns | 14974ns | -99.29% |
| abi_marshal_tight_soa_native | 2043793ns | 2014698ns | 2003079ns | 2013158ns | 2110103ns | +0.46% |
| abi_marshal_tight_soa_transposed | 2098614ns | 2030980ns | 2022721ns | 2028283ns | 2242057ns | +3.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2031741ns | 2004726ns | 2064546ns | base | 0.000 |
| abi_marshal_tight_marshal_null | 12126ns | 11707ns | 12615ns | -99.40% | 0.000 |
| abi_marshal_tight_soa_native | 2041085ns | 2000673ns | 2107232ns | +0.46% | 0.000 |
| abi_marshal_tight_soa_transposed | 2095743ns | 2020067ns | 2238842ns | +3.15% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_tight_aos | 43129.6 | 2030111.5 | 2031740.9 | n/a |
| abi_marshal_tight_marshal_null | 27327.6 | 12277.3 | 12126.2 | n/a |
| abi_marshal_tight_soa_native | 43554.7 | 2046922.3 | 2041085.3 | n/a |
| abi_marshal_tight_soa_transposed | 49554.9 | 2101130.1 | 2095743.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_tight_marshal_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_tight_aos | 0.000 | 0.6% |
| abi_marshal_tight_marshal_null | 0.000 | 97.8% |
| abi_marshal_tight_soa_native | 0.000 | 0.6% |
| abi_marshal_tight_soa_transposed | 0.000 | 0.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_tight_aos | 2034339ns | 2034339ns | base |
| abi_marshal_tight_marshal_null | 14388ns | 14388ns | -99.29% |
| abi_marshal_tight_soa_native | 2043793ns | 2043793ns | +0.46% |
| abi_marshal_tight_soa_transposed | 2098614ns | 2098614ns | +3.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2022183ns | base | --- | [2008494, 2064546] | --- | --- | --- | --- |
| abi_marshal_tight_marshal_null | 11974ns | -2010063.7ns (-99.4%) | [-2052076, -1996704]ns | [11789, 12615] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_tight_soa_native | 2011942ns | no significant difference | [-18101, +48982]ns | [2004082, 2107232] | no | 1.0000 | 1.0000 | 0 |
| abi_marshal_tight_soa_transposed | 2028291ns | +14506.5ns (+0.7%) | [+3204, +174297]ns | [2020097, 2238842] | YES (adj: no) | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_tight_aos | abi_marshal_tight_marshal_null | abi_marshal_tight_soa_native | abi_marshal_tight_soa_transposed |
|---|---|---|---|---|
| 1 | 2004726ns | -99.4% | +0.3% | +0.8% |
| 2 | 2106636ns | -99.4% | +4.4% | +14.4% |
| 3 | 2021995ns | -99.4% | -0.7% | +0.4% |
| 4 | 2022455ns | -99.4% | -0.3% | +2.2% |
| 5 | 2022371ns | -99.4% | -1.1% | -0.1% |
| 6 | 2012261ns | -99.4% | +0.1% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_tight_aos | -0.342 | moderate- |
| abi_marshal_tight_marshal_null | -0.576 | HIGH- (thermal bounce) |
| abi_marshal_tight_soa_native | -0.238 | moderate- |
| abi_marshal_tight_soa_transposed | -0.290 | moderate- |

**Consistency summary:**

- **abi_marshal_tight_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_tight_soa_native**: won 3/6, lost 2/6
- **abi_marshal_tight_soa_transposed**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_tight_aos | 6245854.7ns | 2031740.9ns | 307.4% | HIGH |
| abi_marshal_tight_marshal_null | 145371.7ns | 12126.2ns | 1198.8% | HIGH |
| abi_marshal_tight_soa_native | 6201330.8ns | 2041085.3ns | 303.8% | HIGH |
| abi_marshal_tight_soa_transposed | 6347219.1ns | 2095743.3ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_marshal_tight_aos (n=6, range 2004725.8-2064545.8 ns)
  2004725.8 |#############
  2007716.8 |
  2010707.8 |#############
  2013698.8 |
  2016689.8 |
  2019680.8 |########################################
  2022671.8 |
  2025662.8 |
  2028653.8 |
  2031644.8 |
  2034635.8 |
  2037626.8 |
  2040617.8 |
  2043608.8 |
  2046599.8 |
  2049590.8 |
  2052581.8 |
  2055572.8 |
  2058563.8 |
  2061554.8 |
  (0 below, 1 above range)

abi_marshal_tight_marshal_null (n=6, range 11707.1-12615.2 ns)
  11707.1 |########################################
  11752.5 |
  11797.9 |
  11843.3 |########################################
  11888.7 |########################################
  11934.1 |
  11979.5 |
  12024.9 |########################################
  12070.3 |
  12115.7 |
  12161.2 |
  12206.6 |
  12252.0 |
  12297.4 |
  12342.8 |
  12388.2 |########################################
  12433.6 |
  12479.0 |
  12524.4 |
  12569.8 |
  (0 below, 1 above range)

abi_marshal_tight_soa_native (n=6, range 2000672.9-2107232.5 ns)
  2000672.9 |####################
  2006000.9 |########################################
  2011328.9 |########################################
  2016656.8 |
  2021984.8 |
  2027312.8 |
  2032640.8 |
  2037968.8 |
  2043296.7 |
  2048624.7 |
  2053952.7 |
  2059280.7 |
  2064608.7 |
  2069936.6 |
  2075264.6 |
  2080592.6 |
  2085920.6 |
  2091248.6 |
  2096576.5 |
  2101904.5 |
  (0 below, 1 above range)

abi_marshal_tight_soa_transposed (n=6, range 2020067.1-2238842.5 ns)
  2020067.1 |########################################
  2031005.9 |
  2041944.6 |
  2052883.4 |
  2063822.2 |##########
  2074761.0 |
  2085699.7 |
  2096638.5 |
  2107577.3 |
  2118516.0 |
  2129454.8 |
  2140393.6 |
  2151332.3 |
  2162271.1 |
  2173209.9 |
  2184148.6 |
  2195087.4 |
  2206026.2 |
  2216965.0 |
  2227903.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_tight_aos**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_marshal_tight_marshal_null**: bridge=1214.3% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_native**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_transposed**: bridge=301.9% of algo (FFI overhead may distort results)
