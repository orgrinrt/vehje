# abi_residency (tight)

3 variants, 6 samples per variant.
Baseline: **abi_residency_tight_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_tight_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_tight_null_entry dominates: 77193% faster than the next best (abi_residency_tight_reused_buffer)

abi_residency_tight_null_entry (2.59 us) leads abi_residency_tight_reused_buffer (2.00 ms) by 77193%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_tight_null_entry beats baseline by 100% (significant)

abi_residency_tight_null_entry is -2.00 ms (100%) faster than baseline abi_residency_tight_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_tight_fresh_alloc is an outlier: 775.1x slower than the field

abi_residency_tight_fresh_alloc (2.01 ms) is 775.1x the fastest (2.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_tight_null_entry shows alternating (throttle bounce) (autocorr -0.59)

abi_residency_tight_null_entry's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 775.1x the fastest

Fastest abi_residency_tight_null_entry (2.59 us) to slowest abi_residency_tight_fresh_alloc (2.01 ms): 775.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_tight_null_entry** at 2587.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 775.08x (fastest 2587.7 ns, slowest 2005685.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2006053ns | 2008215ns | 1994992ns | 2005385ns | 2012584ns | +0.20% |
| abi_residency_tight_null_entry | 4867ns | 4961ns | 4628ns | 4859ns | 4999ns | -99.76% |
| abi_residency_tight_reused_buffer | 2002108ns | 2002691ns | 1995905ns | 2001857ns | 2005586ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2003528ns | 1992405ns | 2010034ns | +0.20% | 0.000 |
| abi_residency_tight_null_entry | 2560ns | 2465ns | 2620ns | -99.87% | 0.006 |
| abi_residency_tight_reused_buffer | 1999528ns | 1993417ns | 2002944ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 38851.1 | 2003177.8 | 2003528.3 | n/a |
| abi_residency_tight_null_entry | 27143.6 | 2652.1 | 2559.9 | n/a |
| abi_residency_tight_reused_buffer | 38570.4 | 1999661.2 | 1999528.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_residency_tight_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.000 | 0.1% |
| abi_residency_tight_null_entry | 0.006 | 95.2% |
| abi_residency_tight_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2006053ns | 2006053ns | +0.20% |
| abi_residency_tight_null_entry | 4867ns | 4867ns | -99.76% |
| abi_residency_tight_reused_buffer | 2002108ns | 2002108ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_tight_reused_buffer | 2000107ns | base | --- | [1995535, 2002944] | --- | --- | --- | --- |
| abi_residency_tight_fresh_alloc | 2005685ns | no significant difference | [-5241, +14157]ns | [1994866, 2010034] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_tight_null_entry | 2588ns | -1997486.5ns (-99.9%) | [-2000413, -1993006]ns | [2471, 2620] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_tight_reused_buffer | abi_residency_tight_fresh_alloc | abi_residency_tight_null_entry |
|---|---|---|---|
| 1 | 2004200ns | +0.2% | -99.9% |
| 2 | 2000935ns | -0.2% | -99.9% |
| 3 | 1997652ns | +0.5% | -99.9% |
| 4 | 1999279ns | -0.3% | -99.9% |
| 5 | 2001688ns | +0.1% | -99.9% |
| 6 | 1993417ns | +0.9% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_tight_fresh_alloc | -0.425 | moderate- |
| abi_residency_tight_null_entry | -0.586 | HIGH- (thermal bounce) |
| abi_residency_tight_reused_buffer | -0.135 | ok |

**Consistency summary:**

- **abi_residency_tight_fresh_alloc**: won 2/6, lost 3/6
- **abi_residency_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 6053597.1ns | 2003528.3ns | 302.1% | HIGH |
| abi_residency_tight_null_entry | 117601.1ns | 2559.9ns | 4594.0% | HIGH |
| abi_residency_tight_reused_buffer | 6040597.0ns | 1999528.4ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_residency_tight_fresh_alloc (n=6, range 1992404.6-2010033.6 ns)
  1992404.6 |####################
  1993286.0 |
  1994167.5 |
  1995048.9 |
  1995930.4 |
  1996811.8 |####################
  1997693.3 |
  1998574.7 |
  1999456.2 |
  2000337.6 |
  2001219.1 |
  2002100.5 |
  2002982.0 |####################
  2003863.4 |
  2004744.9 |
  2005626.3 |
  2006507.8 |
  2007389.2 |
  2008270.7 |########################################
  2009152.1 |
  (0 below, 1 above range)

abi_residency_tight_null_entry (n=6, range 2464.6-2620.4 ns)
   2464.6 |####################
   2472.4 |####################
   2480.2 |
   2488.0 |
   2495.8 |
   2503.6 |
   2511.4 |
   2519.1 |
   2526.9 |
   2534.7 |
   2542.5 |
   2550.3 |
   2558.1 |
   2565.9 |
   2573.7 |####################
   2581.5 |
   2589.3 |
   2597.1 |########################################
   2604.9 |
   2612.7 |
  (0 below, 1 above range)

abi_residency_tight_reused_buffer (n=6, range 1993416.7-2002943.8 ns)
  1993416.7 |########################################
  1993893.1 |
  1994369.4 |
  1994845.8 |
  1995322.1 |
  1995798.5 |
  1996274.8 |
  1996751.2 |
  1997227.5 |########################################
  1997703.9 |
  1998180.2 |
  1998656.6 |
  1999132.9 |########################################
  1999609.3 |
  2000085.6 |
  2000562.0 |########################################
  2001038.3 |
  2001514.7 |########################################
  2001991.0 |
  2002467.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_tight_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_tight_null_entry**: bridge=4539.3% of algo (FFI overhead may distort results)
- **abi_residency_tight_reused_buffer**: bridge=302.2% of algo (FFI overhead may distort results)
