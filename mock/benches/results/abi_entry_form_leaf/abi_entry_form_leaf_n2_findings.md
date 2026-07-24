# abi_entry_form (leaf)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_leaf_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_leaf_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_leaf_null_entry dominates: 41900% faster than the next best (abi_entry_form_leaf_per_w_set)

abi_entry_form_leaf_null_entry (4.11 us) leads abi_entry_form_leaf_per_w_set (1.72 ms) by 41900%, a clear separation rather than a photo finish. CV 20.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_leaf_null_entry beats baseline by 100% (significant)

abi_entry_form_leaf_null_entry is -1.79 ms (100%) faster than baseline abi_entry_form_leaf_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_leaf_scalar_anchor is an outlier: 451.6x slower than the field

abi_entry_form_leaf_scalar_anchor (1.85 ms) is 451.6x the fastest (4.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_leaf_per_w_set shows warm-up / thermal drift (autocorr +0.52)

abi_entry_form_leaf_per_w_set's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_leaf_null_entry} vs {abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_scalar_anchor} (41900% apart)

The field splits into a fast tier {abi_entry_form_leaf_null_entry} and a slow tier {abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_scalar_anchor} with a 41900% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 451.6x the fastest

Fastest abi_entry_form_leaf_null_entry (4.11 us) to slowest abi_entry_form_leaf_scalar_anchor (1.85 ms): 451.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_entry_form_leaf_runtime_w is inconsistent: worst-20% is 1.8x its best-20%

abi_entry_form_leaf_runtime_w's best 20% of batches run at 1.55 ms but its worst 20% at 2.86 ms (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_entry_form_leaf_null_entry** at 4105.0 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 451.60x (fastest 4105.0 ns, slowest 1853804.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1887516ns | 1833869ns | 1564873ns | 1744805ns | 2262904ns | -9.20% |
| abi_entry_form_leaf_null_entry | 10623ns | 6860ns | 5799ns | 6519ns | 19189ns | -99.49% |
| abi_entry_form_leaf_per_w_set | 1828864ns | 1728886ns | 1567960ns | 1679213ns | 2183794ns | -12.02% |
| abi_entry_form_leaf_runtime_w | 2078753ns | 1799781ns | 1552955ns | 1732226ns | 2861444ns | base |
| abi_entry_form_leaf_scalar_anchor | 2033094ns | 1858730ns | 1537287ns | 1785312ns | 2652670ns | -2.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1882303ns | 1561240ns | 2256345ns | -9.23% | 0.000 |
| abi_entry_form_leaf_null_entry | 4261ns | 3472ns | 5206ns | -99.79% | 0.000 |
| abi_entry_form_leaf_per_w_set | 1824032ns | 1564089ns | 2177875ns | -12.04% | 0.000 |
| abi_entry_form_leaf_runtime_w | 2073592ns | 1549302ns | 2855148ns | base | 0.000 |
| abi_entry_form_leaf_scalar_anchor | 2028063ns | 1533677ns | 2646252ns | -2.20% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 110726.7 | 1897642.8 | 1882302.6 | n/a |
| abi_entry_form_leaf_null_entry | 39925.4 | 7338.1 | 4261.2 | n/a |
| abi_entry_form_leaf_per_w_set | 104864.6 | 1864250.1 | 1824032.2 | n/a |
| abi_entry_form_leaf_runtime_w | 114454.7 | 2075600.1 | 2073592.0 | n/a |
| abi_entry_form_leaf_scalar_anchor | 118533.1 | 2048558.3 | 2028063.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_leaf_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_leaf_null_entry | 0.000 | 84.6% |
| abi_entry_form_leaf_per_w_set | 0.000 | 0.2% |
| abi_entry_form_leaf_runtime_w | 0.000 | 0.2% |
| abi_entry_form_leaf_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1887516ns | 1887516ns | -9.20% |
| abi_entry_form_leaf_null_entry | 10623ns | 10623ns | -99.49% |
| abi_entry_form_leaf_per_w_set | 1828864ns | 1828864ns | -12.02% |
| abi_entry_form_leaf_runtime_w | 2078753ns | 2078753ns | base |
| abi_entry_form_leaf_scalar_anchor | 2033094ns | 2033094ns | -2.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_leaf_runtime_w | 1794330ns | base | --- | [1571298, 2855148] | --- | --- | --- | --- |
| abi_entry_form_leaf_dispatch_table | 1828639ns | no significant difference | [-799102, +293497]ns | [1561923, 2256345] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_leaf_null_entry | 4105ns | -1790337.2ns (-99.8%) | [-2849943, -1567712]ns | [3473, 5206] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_leaf_per_w_set | 1724104ns | no significant difference | [-785809, +101465]ns | [1570118, 2177875] | no | 1.0000 | 0.6875 | 0 |
| abi_entry_form_leaf_scalar_anchor | 1853805ns | no significant difference | [-296587, +147166]ns | [1584133, 2646252] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_leaf_runtime_w | abi_entry_form_leaf_dispatch_table | abi_entry_form_leaf_null_entry | abi_entry_form_leaf_per_w_set | abi_entry_form_leaf_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 1549302ns | +0.8% | -99.8% | +1.0% | -1.0% |
| 2 | 1593295ns | +8.8% | -99.8% | -1.1% | +2.6% |
| 3 | 1711073ns | -8.7% | -99.8% | -6.5% | -3.6% |
| 4 | 3120585ns | -38.3% | -99.8% | -40.8% | +2.1% |
| 5 | 2589711ns | -15.5% | -99.8% | -11.6% | -20.5% |
| 6 | 1877586ns | +23.8% | -99.8% | +10.0% | +12.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.444 | moderate+ |
| abi_entry_form_leaf_null_entry | 0.064 | ok |
| abi_entry_form_leaf_per_w_set | 0.524 | HIGH+ (drift/warm-up) |
| abi_entry_form_leaf_runtime_w | 0.238 | moderate+ |
| abi_entry_form_leaf_scalar_anchor | -0.030 | ok |

**Consistency summary:**

- **abi_entry_form_leaf_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_leaf_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_leaf_per_w_set**: won 4/6, lost 2/6
- **abi_entry_form_leaf_scalar_anchor**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 5843436.8ns | 1882302.6ns | 310.4% | HIGH |
| abi_entry_form_leaf_null_entry | 140248.7ns | 4261.2ns | 3291.3% | HIGH |
| abi_entry_form_leaf_per_w_set | 5712844.0ns | 1824032.2ns | 313.2% | HIGH |
| abi_entry_form_leaf_runtime_w | 6319404.3ns | 2073592.0ns | 304.8% | HIGH |
| abi_entry_form_leaf_scalar_anchor | 6109625.8ns | 2028063.3ns | 301.3% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_leaf_dispatch_table (n=6, range 1561240.4-2256345.4 ns)
  1561240.4 |########################################
  1595995.6 |
  1630750.9 |
  1665506.1 |
  1700261.4 |####################
  1735016.6 |
  1769771.9 |
  1804527.1 |
  1839282.4 |
  1874037.6 |
  1908792.9 |####################
  1943548.1 |
  1978303.4 |
  2013058.6 |
  2047813.9 |
  2082569.1 |
  2117324.4 |
  2152079.6 |
  2186834.9 |####################
  2221590.1 |
  (0 below, 1 above range)

abi_entry_form_leaf_null_entry (n=6, range 3471.7-5205.6 ns)
   3471.7 |########################################
   3558.4 |
   3645.1 |####################
   3731.8 |
   3818.5 |
   3905.2 |
   3991.9 |
   4078.6 |
   4165.3 |
   4252.0 |
   4338.6 |
   4425.3 |####################
   4512.0 |####################
   4598.7 |
   4685.4 |
   4772.1 |
   4858.8 |
   4945.5 |
   5032.2 |
   5118.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_per_w_set (n=6, range 1564088.7-2177875.2 ns)
  1564088.7 |########################################
  1594778.0 |####################
  1625467.4 |
  1656156.7 |
  1686846.0 |
  1717535.3 |
  1748224.6 |
  1778914.0 |
  1809603.3 |
  1840292.6 |####################
  1870982.0 |
  1901671.3 |
  1932360.6 |
  1963049.9 |
  1993739.2 |
  2024428.6 |
  2055117.9 |####################
  2085807.2 |
  2116496.6 |
  2147185.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_runtime_w (n=6, range 1549301.7-2855148.3 ns)
  1549301.7 |########################################
  1614594.0 |
  1679886.4 |####################
  1745178.7 |
  1810471.0 |
  1875763.3 |####################
  1941055.7 |
  2006348.0 |
  2071640.3 |
  2136932.7 |
  2202225.0 |
  2267517.3 |
  2332809.7 |
  2398102.0 |
  2463394.3 |
  2528686.6 |####################
  2593979.0 |
  2659271.3 |
  2724563.6 |
  2789856.0 |
  (0 below, 1 above range)

abi_entry_form_leaf_scalar_anchor (n=6, range 1533677.1-2646251.8 ns)
  1533677.1 |########################################
  1589305.8 |########################################
  1644934.6 |########################################
  1700563.3 |
  1756192.1 |
  1811820.8 |
  1867449.5 |
  1923078.3 |
  1978707.0 |
  2034335.7 |########################################
  2089964.5 |########################################
  2145593.2 |
  2201221.9 |
  2256850.7 |
  2312479.4 |
  2368108.2 |
  2423736.9 |
  2479365.6 |
  2534994.4 |
  2590623.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_leaf_dispatch_table**: bridge=309.3% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_null_entry**: bridge=3218.5% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_per_w_set**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **abi_entry_form_leaf_per_w_set**: bridge=316.1% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_runtime_w**: CV=28.1% (high variance, measurements may be unstable)
- **abi_entry_form_leaf_runtime_w**: bridge=308.9% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_scalar_anchor**: CV=27.7% (high variance, measurements may be unstable)
- **abi_entry_form_leaf_scalar_anchor**: bridge=299.7% of algo (FFI overhead may distort results)
