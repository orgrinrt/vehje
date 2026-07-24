# abi_soa_win (tight)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_tight_scalar_payload has the worst median (2.00 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_tight_null_entry at 2.31 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_tight_null_entry dominates: 41335% faster than the next best (abi_soa_win_tight_soa_payload)

abi_soa_win_tight_null_entry (2.31 us) leads abi_soa_win_tight_soa_payload (956.44 us) by 41335%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_tight_null_entry beats baseline by 100% (significant)

abi_soa_win_tight_null_entry is -2.00 ms (100%) faster than baseline abi_soa_win_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_tight_scalar_payload is an outlier: 867.4x slower than the field

abi_soa_win_tight_scalar_payload (2.00 ms) is 867.4x the fastest (2.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 867.4x the fastest

Fastest abi_soa_win_tight_null_entry (2.31 us) to slowest abi_soa_win_tight_scalar_payload (2.00 ms): 867.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_tight_null_entry** at 2308.3 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 867.41x (fastest 2308.3 ns, slowest 2002250.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 4638ns | 4587ns | 4467ns | 4572ns | 4823ns | -99.77% |
| abi_soa_win_tight_scalar_payload | 2005621ns | 2004800ns | 2001757ns | 2004064ns | 2009888ns | base |
| abi_soa_win_tight_soa_payload | 957914ns | 958846ns | 952166ns | 958658ns | 959673ns | -52.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 2325ns | 2230ns | 2404ns | -99.88% | 0.014 |
| abi_soa_win_tight_scalar_payload | 2003113ns | 1999355ns | 2007333ns | base | 0.000 |
| abi_soa_win_tight_soa_payload | 955518ns | 949761ns | 957218ns | -52.30% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 26777.2 | 2402.1 | 2324.5 | n/a |
| abi_soa_win_tight_scalar_payload | 36041.2 | 2004187.2 | 2003113.0 | n/a |
| abi_soa_win_tight_soa_payload | 31314.6 | 955087.2 | 955517.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_soa_win_tight_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.014 | 96.6% |
| abi_soa_win_tight_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_tight_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_tight_null_entry | 4638ns | 4638ns | -99.77% |
| abi_soa_win_tight_scalar_payload | 2005621ns | 2005621ns | base |
| abi_soa_win_tight_soa_payload | 957914ns | 957914ns | -52.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_tight_scalar_payload | 2002251ns | base | --- | [1999755, 2007333] | --- | --- | --- | --- |
| abi_soa_win_tight_null_entry | 2308ns | -1999946.1ns (-99.9%) | [-2005025, -1997395]ns | [2261, 2404] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_tight_soa_payload | 956438ns | -1047281.7ns (-52.3%) | [-1052532, -1042972]ns | [952897, 957218] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_tight_scalar_payload | abi_soa_win_tight_null_entry | abi_soa_win_tight_soa_payload |
|---|---|---|---|
| 1 | 2005672ns | -99.9% | -52.3% |
| 2 | 2000155ns | -99.9% | -52.2% |
| 3 | 2001767ns | -99.9% | -52.2% |
| 4 | 1999355ns | -99.9% | -52.1% |
| 5 | 2008995ns | -99.9% | -52.4% |
| 6 | 2002735ns | -99.9% | -52.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_tight_null_entry | -0.457 | moderate- |
| abi_soa_win_tight_scalar_payload | -0.346 | moderate- |
| abi_soa_win_tight_soa_payload | -0.055 | ok |

**Consistency summary:**

- **abi_soa_win_tight_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 116165.1ns | 2324.5ns | 4997.4% | HIGH |
| abi_soa_win_tight_scalar_payload | 6050346.7ns | 2003113.0ns | 302.0% | HIGH |
| abi_soa_win_tight_soa_payload | 2898444.6ns | 955517.8ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_tight_null_entry (n=6, range 2230.4-2404.2 ns)
   2230.4 |####################
   2239.1 |
   2247.8 |
   2256.5 |
   2265.2 |
   2273.8 |
   2282.5 |
   2291.2 |########################################
   2299.9 |
   2308.6 |
   2317.3 |####################
   2326.0 |
   2334.7 |
   2343.4 |
   2352.1 |
   2360.8 |
   2369.4 |
   2378.1 |####################
   2386.8 |
   2395.5 |
  (0 below, 1 above range)

abi_soa_win_tight_scalar_payload (n=6, range 1999355.4-2007333.1 ns)
  1999355.4 |########################################
  1999754.3 |
  2000153.2 |########################################
  2000552.1 |
  2000950.9 |
  2001349.8 |
  2001748.7 |########################################
  2002147.6 |
  2002546.5 |########################################
  2002945.4 |
  2003344.3 |
  2003743.2 |
  2004142.0 |
  2004540.9 |
  2004939.8 |
  2005338.7 |########################################
  2005737.6 |
  2006136.5 |
  2006535.4 |
  2006934.3 |
  (0 below, 1 above range)

abi_soa_win_tight_soa_payload (n=6, range 949760.8-957218.5 ns)
  949760.8 |########################################
  950133.7 |
  950506.6 |
  950879.5 |
  951252.3 |
  951625.2 |
  951998.1 |
  952371.0 |
  952743.9 |
  953116.8 |
  953489.7 |
  953862.5 |
  954235.4 |
  954608.3 |
  954981.2 |
  955354.1 |
  955727.0 |########################################
  956099.8 |########################################
  956472.7 |########################################
  956845.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_tight_null_entry**: bridge=5031.6% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_scalar_payload**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_soa_payload**: bridge=303.2% of algo (FFI overhead may distort results)
