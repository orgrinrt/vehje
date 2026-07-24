# abi_soa_win (tight)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_tight_scalar_payload has the worst median (2.01 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_tight_null_entry at 2.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_tight_null_entry dominates: 37102% faster than the next best (abi_soa_win_tight_soa_payload)

abi_soa_win_tight_null_entry (2.57 us) leads abi_soa_win_tight_soa_payload (954.39 us) by 37102%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_tight_null_entry beats baseline by 100% (significant)

abi_soa_win_tight_null_entry is -2.00 ms (100%) faster than baseline abi_soa_win_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_tight_scalar_payload is an outlier: 781.8x slower than the field

abi_soa_win_tight_scalar_payload (2.01 ms) is 781.8x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 781.8x the fastest

Fastest abi_soa_win_tight_null_entry (2.57 us) to slowest abi_soa_win_tight_scalar_payload (2.01 ms): 781.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_tight_null_entry** at 2565.4 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 781.76x (fastest 2565.4 ns, slowest 2005527.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 4881ns | 4898ns | 4598ns | 4868ns | 5041ns | -99.76% |
| abi_soa_win_tight_scalar_payload | 2034541ns | 2008082ns | 2005692ns | 2007538ns | 2089470ns | base |
| abi_soa_win_tight_soa_payload | 986802ns | 956856ns | 950408ns | 956179ns | 1050933ns | -51.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 2557ns | 2410ns | 2655ns | -99.87% | 0.025 |
| abi_soa_win_tight_scalar_payload | 2031805ns | 2003216ns | 2086356ns | base | 0.000 |
| abi_soa_win_tight_soa_payload | 984243ns | 947955ns | 1048186ns | -51.56% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 27296.9 | 2813.3 | 2556.9 | n/a |
| abi_soa_win_tight_scalar_payload | 44386.3 | 2042997.8 | 2031805.1 | n/a |
| abi_soa_win_tight_soa_payload | 36486.2 | 969106.2 | 984242.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_soa_win_tight_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.025 | 94.0% |
| abi_soa_win_tight_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_tight_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_tight_null_entry | 4881ns | 4881ns | -99.76% |
| abi_soa_win_tight_scalar_payload | 2034541ns | 2034541ns | base |
| abi_soa_win_tight_soa_payload | 986802ns | 986802ns | -51.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_tight_scalar_payload | 2005528ns | base | --- | [2003531, 2086356] | --- | --- | --- | --- |
| abi_soa_win_tight_null_entry | 2565ns | -2002872.7ns (-99.9%) | [-2083813, -2001059]ns | [2450, 2655] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_tight_soa_payload | 954387ns | -1052133.1ns (-52.5%) | [-1054858, -1035696]ns | [950155, 1048186] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_tight_scalar_payload | abi_soa_win_tight_null_entry | abi_soa_win_tight_soa_payload |
|---|---|---|---|
| 1 | 2003847ns | -99.9% | -52.4% |
| 2 | 2003216ns | -99.9% | -52.7% |
| 3 | 2005981ns | -99.9% | -52.5% |
| 4 | 2162690ns | -99.9% | -48.5% |
| 5 | 2005074ns | -99.9% | -51.0% |
| 6 | 2010022ns | -99.9% | -52.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_tight_null_entry | -0.216 | moderate- |
| abi_soa_win_tight_scalar_payload | -0.231 | moderate- |
| abi_soa_win_tight_soa_payload | -0.098 | ok |

**Consistency summary:**

- **abi_soa_win_tight_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 112956.9ns | 2556.9ns | 4417.8% | HIGH |
| abi_soa_win_tight_scalar_payload | 6167552.7ns | 2031805.1ns | 303.6% | HIGH |
| abi_soa_win_tight_soa_payload | 3015008.6ns | 984242.8ns | 306.3% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_tight_null_entry (n=6, range 2410.4-2655.0 ns)
   2410.4 |####################
   2422.6 |
   2434.9 |
   2447.1 |
   2459.3 |
   2471.6 |
   2483.8 |####################
   2496.0 |
   2508.2 |
   2520.5 |
   2532.7 |####################
   2544.9 |
   2557.2 |
   2569.4 |
   2581.6 |
   2593.8 |########################################
   2606.1 |
   2618.3 |
   2630.5 |
   2642.8 |
  (0 below, 1 above range)

abi_soa_win_tight_scalar_payload (n=6, range 2003215.8-2086356.2 ns)
  2003215.8 |########################################
  2007372.8 |##########
  2011529.8 |
  2015686.9 |
  2019843.9 |
  2024000.9 |
  2028157.9 |
  2032315.0 |
  2036472.0 |
  2040629.0 |
  2044786.0 |
  2048943.0 |
  2053100.1 |
  2057257.1 |
  2061414.1 |
  2065571.1 |
  2069728.2 |
  2073885.2 |
  2078042.2 |
  2082199.2 |
  (0 below, 1 above range)

abi_soa_win_tight_soa_payload (n=6, range 947955.0-1048186.2 ns)
  947955.0 |########################################
  952966.6 |########################################
  957978.1 |
  962989.7 |
  968001.2 |
  973012.8 |
  978024.4 |####################
  983035.9 |
  988047.5 |
  993059.1 |
  998070.6 |
  1003082.2 |
  1008093.8 |
  1013105.3 |
  1018116.9 |
  1023128.4 |
  1028140.0 |
  1033151.6 |
  1038163.1 |
  1043174.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_tight_null_entry**: bridge=4389.4% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_scalar_payload**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_soa_payload**: bridge=303.9% of algo (FFI overhead may distort results)
