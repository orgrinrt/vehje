# abi_soa_win (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_scatter_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_scatter_null_entry at 3.02 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_scatter_null_entry dominates: 29933% faster than the next best (abi_soa_win_scatter_soa_payload)

abi_soa_win_scatter_null_entry (3.02 us) leads abi_soa_win_scatter_soa_payload (905.74 us) by 29933%, a clear separation rather than a photo finish. CV 34.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_scatter_null_entry beats baseline by 100% (significant)

abi_soa_win_scatter_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_scatter_scalar_payload is an outlier: 713.9x slower than the field

abi_soa_win_scatter_scalar_payload (2.15 ms) is 713.9x the fastest (3.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_scatter_null_entry is fastest but the noisiest (CV 34.2%)

abi_soa_win_scatter_null_entry wins on median (3.02 us) yet has the highest variance (CV 34.2%), while abi_soa_win_scatter_scalar_payload is the steadiest (CV 0.4%, 2.15 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 713.9x the fastest

Fastest abi_soa_win_scatter_null_entry (3.02 us) to slowest abi_soa_win_scatter_scalar_payload (2.15 ms): 713.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_scatter_null_entry** at 3015.8 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 713.95x (fastest 3015.8 ns, slowest 2153157.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 6040ns | 5338ns | 5213ns | 5316ns | 7541ns | -99.72% |
| abi_soa_win_scatter_scalar_payload | 2154944ns | 2156224ns | 2145107ns | 2152546ns | 2163460ns | base |
| abi_soa_win_scatter_soa_payload | 910682ns | 908606ns | 901904ns | 907174ns | 920335ns | -57.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 3460ns | 2945ns | 4416ns | -99.84% | 0.002 |
| abi_soa_win_scatter_scalar_payload | 2151911ns | 2142172ns | 2160285ns | base | 0.000 |
| abi_soa_win_scatter_soa_payload | 907764ns | 899346ns | 917108ns | -57.82% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 35407.4 | 3602.3 | 3459.9 | n/a |
| abi_soa_win_scatter_scalar_payload | 54165.9 | 2151337.4 | 2151910.6 | n/a |
| abi_soa_win_scatter_soa_payload | 47528.3 | 905070.9 | 907763.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_soa_win_scatter_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.003 | 97.7% |
| abi_soa_win_scatter_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_scatter_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_scatter_null_entry | 6040ns | 6040ns | -99.72% |
| abi_soa_win_scatter_scalar_payload | 2154944ns | 2154944ns | base |
| abi_soa_win_scatter_soa_payload | 910682ns | 910682ns | -57.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_scatter_scalar_payload | 2153158ns | base | --- | [2142289, 2160285] | --- | --- | --- | --- |
| abi_soa_win_scatter_null_entry | 3016ns | -2150168.5ns (-99.9%) | [-2155910, -2139273]ns | [2947, 4416] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_scatter_soa_payload | 905738ns | -1243593.8ns (-57.8%) | [-1257863, -1230985]ns | [900445, 917108] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_scatter_scalar_payload | abi_soa_win_scatter_null_entry | abi_soa_win_scatter_soa_payload |
|---|---|---|---|
| 1 | 2142172ns | -99.9% | -57.2% |
| 2 | 2153980ns | -99.9% | -57.8% |
| 3 | 2154013ns | -99.9% | -57.4% |
| 4 | 2142406ns | -99.9% | -58.0% |
| 5 | 2152336ns | -99.9% | -58.1% |
| 6 | 2166557ns | -99.7% | -58.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_scatter_null_entry | -0.056 | ok |
| abi_soa_win_scatter_scalar_payload | -0.082 | ok |
| abi_soa_win_scatter_soa_payload | 0.123 | ok |

**Consistency summary:**

- **abi_soa_win_scatter_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 128226.9ns | 3459.9ns | 3706.1% | HIGH |
| abi_soa_win_scatter_scalar_payload | 6513437.3ns | 2151910.6ns | 302.7% | HIGH |
| abi_soa_win_scatter_soa_payload | 2762336.3ns | 907763.6ns | 304.3% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_scatter_null_entry (n=6, range 2945.4-4416.4 ns)
   2945.4 |########################################
   3019.0 |##########################
   3092.5 |
   3166.1 |
   3239.6 |
   3313.2 |
   3386.7 |
   3460.3 |
   3533.8 |
   3607.4 |
   3680.9 |
   3754.5 |
   3828.0 |
   3901.6 |
   3975.1 |
   4048.7 |
   4122.2 |
   4195.8 |
   4269.3 |
   4342.9 |
  (0 below, 1 above range)

abi_soa_win_scatter_scalar_payload (n=6, range 2142172.5-2160285.0 ns)
  2142172.5 |########################################
  2143078.1 |
  2143983.8 |
  2144889.4 |
  2145795.0 |
  2146700.6 |
  2147606.2 |
  2148511.9 |
  2149417.5 |
  2150323.1 |
  2151228.8 |
  2152134.4 |####################
  2153040.0 |
  2153945.6 |########################################
  2154851.2 |
  2155756.9 |
  2156662.5 |
  2157568.1 |
  2158473.8 |
  2159379.4 |
  (0 below, 1 above range)

abi_soa_win_scatter_soa_payload (n=6, range 899345.8-917108.3 ns)
  899345.8 |####################
  900233.9 |
  901122.1 |########################################
  902010.2 |
  902898.3 |
  903786.4 |
  904674.6 |
  905562.7 |
  906450.8 |
  907338.9 |
  908227.1 |
  909115.2 |####################
  910003.3 |
  910891.4 |
  911779.6 |
  912667.7 |
  913555.8 |
  914443.9 |
  915332.1 |####################
  916220.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_scatter_null_entry**: CV=29.8% (high variance, measurements may be unstable)
- **abi_soa_win_scatter_null_entry**: bridge=3945.1% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_scalar_payload**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_soa_payload**: bridge=303.7% of algo (FFI overhead may distort results)
