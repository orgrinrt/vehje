# abi_marshal (tight)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_tight_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_tight_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_tight_marshal_null dominates: 19296% faster than the next best (abi_marshal_tight_soa_native)

abi_marshal_tight_marshal_null (10.35 us) leads abi_marshal_tight_soa_native (2.01 ms) by 19296%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_tight_marshal_null beats baseline by 99% (significant)

abi_marshal_tight_marshal_null is -2.00 ms (99%) faster than baseline abi_marshal_tight_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_tight_soa_transposed is an outlier: 194.6x slower than the field

abi_marshal_tight_soa_transposed (2.01 ms) is 194.6x the fastest (10.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_tight_soa_native shows alternating (throttle bounce) (autocorr -0.60)

abi_marshal_tight_soa_native's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_tight_marshal_null} vs {abi_marshal_tight_soa_native, abi_marshal_tight_aos, abi_marshal_tight_soa_transposed} (19296% apart)

The field splits into a fast tier {abi_marshal_tight_marshal_null} and a slow tier {abi_marshal_tight_soa_native, abi_marshal_tight_aos, abi_marshal_tight_soa_transposed} with a 19296% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 194.6x the fastest

Fastest abi_marshal_tight_marshal_null (10.35 us) to slowest abi_marshal_tight_soa_transposed (2.01 ms): 194.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_tight_marshal_null** at 10347.3 ns median (-99.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 194.64x (fastest 10347.3 ns, slowest 2014044.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2018076ns | 2015491ns | 2010407ns | 2014397ns | 2027430ns | base |
| abi_marshal_tight_marshal_null | 12702ns | 12680ns | 12243ns | 12544ns | 13168ns | -99.37% |
| abi_marshal_tight_soa_native | 2007973ns | 2009503ns | 1996030ns | 2007908ns | 2014042ns | -0.50% |
| abi_marshal_tight_soa_transposed | 2016370ns | 2016523ns | 2007763ns | 2016118ns | 2021050ns | -0.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2015531ns | 2007956ns | 2024857ns | base | 0.000 |
| abi_marshal_tight_marshal_null | 10390ns | 10011ns | 10805ns | -99.48% | 0.000 |
| abi_marshal_tight_soa_native | 2005470ns | 1993545ns | 2011515ns | -0.50% | 0.000 |
| abi_marshal_tight_soa_transposed | 2013806ns | 2005235ns | 2018395ns | -0.09% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_tight_aos | 40655.5 | 2014338.8 | 2015530.9 | n/a |
| abi_marshal_tight_marshal_null | 28408.0 | 10446.9 | 10389.6 | n/a |
| abi_marshal_tight_soa_native | 39474.1 | 2005661.1 | 2005469.8 | n/a |
| abi_marshal_tight_soa_transposed | 39787.8 | 2014676.3 | 2013806.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_tight_marshal_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_tight_aos | 0.000 | 0.5% |
| abi_marshal_tight_marshal_null | 0.000 | 96.7% |
| abi_marshal_tight_soa_native | 0.000 | 0.5% |
| abi_marshal_tight_soa_transposed | 0.000 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_tight_aos | 2018076ns | 2018076ns | base |
| abi_marshal_tight_marshal_null | 12702ns | 12702ns | -99.37% |
| abi_marshal_tight_soa_native | 2007973ns | 2007973ns | -0.50% |
| abi_marshal_tight_soa_transposed | 2016370ns | 2016370ns | -0.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2012948ns | base | --- | [2008787, 2024857] | --- | --- | --- | --- |
| abi_marshal_tight_marshal_null | 10347ns | -2002342.1ns (-99.5%) | [-2014620, -1998461]ns | [10016, 10805] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_tight_soa_native | 2006989ns | no significant difference | [-26160, +2020]ns | [1997905, 2011515] | no | 0.3281 | 0.2188 | 0 |
| abi_marshal_tight_soa_transposed | 2014044ns | no significant difference | [-15352, +7473]ns | [2008979, 2018395] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_tight_aos | abi_marshal_tight_marshal_null | abi_marshal_tight_soa_native | abi_marshal_tight_soa_transposed |
|---|---|---|---|---|
| 1 | 2013888ns | -99.5% | -0.2% | +0.4% |
| 2 | 2012305ns | -99.5% | -0.9% | +0.1% |
| 3 | 2007956ns | -99.5% | +0.3% | +0.3% |
| 4 | 2013592ns | -99.5% | -0.4% | -0.4% |
| 5 | 2009618ns | -99.5% | -0.1% | +0.2% |
| 6 | 2035826ns | -99.5% | -1.6% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_tight_aos | -0.123 | ok |
| abi_marshal_tight_marshal_null | 0.034 | ok |
| abi_marshal_tight_soa_native | -0.597 | HIGH- (thermal bounce) |
| abi_marshal_tight_soa_transposed | 0.062 | ok |

**Consistency summary:**

- **abi_marshal_tight_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_tight_soa_native**: won 4/6, lost 1/6
- **abi_marshal_tight_soa_transposed**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_tight_aos | 6088894.0ns | 2015530.9ns | 302.1% | HIGH |
| abi_marshal_tight_marshal_null | 141227.5ns | 10389.6ns | 1359.3% | HIGH |
| abi_marshal_tight_soa_native | 6059368.2ns | 2005469.8ns | 302.1% | HIGH |
| abi_marshal_tight_soa_transposed | 6085047.5ns | 2013806.0ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_marshal_tight_aos (n=6, range 2007955.8-2024857.2 ns)
  2007955.8 |########################################
  2008800.9 |########################################
  2009645.9 |
  2010491.0 |
  2011336.1 |
  2012181.2 |########################################
  2013026.2 |########################################
  2013871.3 |########################################
  2014716.4 |
  2015561.5 |
  2016406.5 |
  2017251.6 |
  2018096.7 |
  2018941.7 |
  2019786.8 |
  2020631.9 |
  2021477.0 |
  2022322.0 |
  2023167.1 |
  2024012.2 |
  (0 below, 1 above range)

abi_marshal_tight_marshal_null (n=6, range 10010.8-10805.4 ns)
  10010.8 |########################################
  10050.5 |
  10090.3 |
  10130.0 |####################
  10169.7 |
  10209.5 |
  10249.2 |
  10288.9 |
  10328.6 |
  10368.4 |
  10408.1 |
  10447.8 |
  10487.6 |
  10527.3 |####################
  10567.0 |
  10606.8 |####################
  10646.5 |
  10686.2 |
  10725.9 |
  10765.7 |
  (0 below, 1 above range)

abi_marshal_tight_soa_native (n=6, range 1993545.0-2011514.6 ns)
  1993545.0 |########################################
  1994443.5 |
  1995342.0 |
  1996240.4 |
  1997138.9 |
  1998037.4 |
  1998935.9 |
  1999834.4 |
  2000732.8 |
  2001631.3 |########################################
  2002529.8 |
  2003428.3 |
  2004326.8 |
  2005225.2 |
  2006123.7 |########################################
  2007022.2 |########################################
  2007920.7 |
  2008819.2 |########################################
  2009717.6 |
  2010616.1 |
  (0 below, 1 above range)

abi_marshal_tight_soa_transposed (n=6, range 2005235.4-2018395.0 ns)
  2005235.4 |####################
  2005893.4 |
  2006551.4 |
  2007209.3 |
  2007867.3 |
  2008525.3 |
  2009183.3 |
  2009841.3 |
  2010499.2 |
  2011157.2 |
  2011815.2 |
  2012473.2 |####################
  2013131.2 |####################
  2013789.1 |
  2014447.1 |########################################
  2015105.1 |
  2015763.1 |
  2016421.1 |
  2017079.0 |
  2017737.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_tight_aos**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_marshal_tight_marshal_null**: bridge=1348.3% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_native**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_transposed**: bridge=302.1% of algo (FFI overhead may distort results)
