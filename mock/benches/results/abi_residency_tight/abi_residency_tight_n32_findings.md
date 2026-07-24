# abi_residency (tight)

3 variants, 6 samples per variant.
Baseline: **abi_residency_tight_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_tight_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_tight_null_entry dominates: 85894% faster than the next best (abi_residency_tight_reused_buffer)

abi_residency_tight_null_entry (2.33 us) leads abi_residency_tight_reused_buffer (2.00 ms) by 85894%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_tight_null_entry beats baseline by 100% (significant)

abi_residency_tight_null_entry is -2.00 ms (100%) faster than baseline abi_residency_tight_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_tight_fresh_alloc is an outlier: 860.6x slower than the field

abi_residency_tight_fresh_alloc (2.00 ms) is 860.6x the fastest (2.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 860.6x the fastest

Fastest abi_residency_tight_null_entry (2.33 us) to slowest abi_residency_tight_fresh_alloc (2.00 ms): 860.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_tight_null_entry** at 2326.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 860.55x (fastest 2326.9 ns, slowest 2002373.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2008104ns | 2005069ns | 2001770ns | 2004680ns | 2016406ns | -0.02% |
| abi_residency_tight_null_entry | 4604ns | 4635ns | 4305ns | 4563ns | 4814ns | -99.77% |
| abi_residency_tight_reused_buffer | 2008582ns | 2003428ns | 1993825ns | 2002225ns | 2025498ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2005508ns | 1999229ns | 2013854ns | -0.03% | 0.000 |
| abi_residency_tight_null_entry | 2309ns | 2172ns | 2399ns | -99.88% | 0.014 |
| abi_residency_tight_reused_buffer | 2006015ns | 1991388ns | 2022678ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 39536.2 | 2006405.3 | 2005507.7 | n/a |
| abi_residency_tight_null_entry | 26994.1 | 2398.3 | 2308.9 | n/a |
| abi_residency_tight_reused_buffer | 40780.8 | 2013533.4 | 2006014.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.015 Gops/s** (abi_residency_tight_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.000 | 0.1% |
| abi_residency_tight_null_entry | 0.014 | 93.3% |
| abi_residency_tight_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2008104ns | 2008104ns | -0.02% |
| abi_residency_tight_null_entry | 4604ns | 4604ns | -99.77% |
| abi_residency_tight_reused_buffer | 2008582ns | 2008582ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_tight_reused_buffer | 2000962ns | base | --- | [1994405, 2022678] | --- | --- | --- | --- |
| abi_residency_tight_fresh_alloc | 2002374ns | no significant difference | [-8824, +7447]ns | [2000296, 2013854] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_tight_null_entry | 2327ns | -1998697.6ns (-99.9%) | [-2020395, -1992025]ns | [2201, 2399] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_tight_reused_buffer | abi_residency_tight_fresh_alloc | abi_residency_tight_null_entry |
|---|---|---|---|
| 1 | 2000603ns | -0.1% | -99.9% |
| 2 | 1991388ns | +0.5% | -99.9% |
| 3 | 2001321ns | +0.1% | -99.9% |
| 4 | 2037218ns | -0.6% | -99.9% |
| 5 | 1997421ns | +0.2% | -99.9% |
| 6 | 2008137ns | -0.2% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_tight_fresh_alloc | -0.229 | moderate- |
| abi_residency_tight_null_entry | 0.067 | ok |
| abi_residency_tight_reused_buffer | -0.216 | moderate- |

**Consistency summary:**

- **abi_residency_tight_fresh_alloc**: won 2/6, lost 2/6
- **abi_residency_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 6055829.8ns | 2005507.7ns | 302.0% | HIGH |
| abi_residency_tight_null_entry | 116382.7ns | 2308.9ns | 5040.6% | HIGH |
| abi_residency_tight_reused_buffer | 6083653.2ns | 2006014.8ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_residency_tight_fresh_alloc (n=6, range 1999229.2-2013853.5 ns)
  1999229.2 |####################
  1999960.4 |
  2000691.6 |####################
  2001422.9 |
  2002154.1 |########################################
  2002885.3 |
  2003616.5 |####################
  2004347.7 |
  2005078.9 |
  2005810.2 |
  2006541.4 |
  2007272.6 |
  2008003.8 |
  2008735.0 |
  2009466.2 |
  2010197.5 |
  2010928.7 |
  2011659.9 |
  2012391.1 |
  2013122.3 |
  (0 below, 1 above range)

abi_residency_tight_null_entry (n=6, range 2171.7-2399.1 ns)
   2171.7 |########################################
   2183.1 |
   2194.4 |
   2205.8 |
   2217.2 |
   2228.6 |########################################
   2239.9 |
   2251.3 |
   2262.7 |
   2274.1 |
   2285.4 |
   2296.8 |
   2308.2 |########################################
   2319.5 |
   2330.9 |########################################
   2342.3 |
   2353.7 |########################################
   2365.0 |
   2376.4 |
   2387.8 |
  (0 below, 1 above range)

abi_residency_tight_reused_buffer (n=6, range 1991388.3-2022677.5 ns)
  1991388.3 |########################################
  1992952.8 |
  1994517.2 |
  1996081.7 |########################################
  1997646.1 |
  1999210.6 |########################################
  2000775.1 |########################################
  2002339.5 |
  2003904.0 |
  2005468.4 |
  2007032.9 |########################################
  2008597.4 |
  2010161.8 |
  2011726.3 |
  2013290.7 |
  2014855.2 |
  2016419.7 |
  2017984.1 |
  2019548.6 |
  2021113.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_tight_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_tight_null_entry**: bridge=4974.5% of algo (FFI overhead may distort results)
- **abi_residency_tight_reused_buffer**: bridge=302.1% of algo (FFI overhead may distort results)
