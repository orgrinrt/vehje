# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 2.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 35327% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (2.50 us) leads abi_soa_win_real_soa_payload (884.20 us) by 35327%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 860.8x slower than the field

abi_soa_win_real_scalar_payload (2.15 ms) is 860.8x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_null_entry shows alternating (throttle bounce) (autocorr -0.71)

abi_soa_win_real_null_entry's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 860.8x the fastest

Fastest abi_soa_win_real_null_entry (2.50 us) to slowest abi_soa_win_real_scalar_payload (2.15 ms): 860.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 2495.8 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 860.84x (fastest 2495.8 ns, slowest 2148534.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 4776ns | 4721ns | 4686ns | 4718ns | 4907ns | -99.79% |
| abi_soa_win_real_scalar_payload | 2246653ns | 2151146ns | 2143977ns | 2149019ns | 2444441ns | base |
| abi_soa_win_real_soa_payload | 886714ns | 886568ns | 885797ns | 886321ns | 887762ns | -60.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 2511ns | 2478ns | 2560ns | -99.89% | 0.006 |
| abi_soa_win_real_scalar_payload | 2243723ns | 2141433ns | 2440843ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 884331ns | 883373ns | 885403ns | -60.59% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 27652.2 | 2609.0 | 2511.1 | n/a |
| abi_soa_win_real_scalar_payload | 51377.0 | 2214325.9 | 2243722.6 | n/a |
| abi_soa_win_real_soa_payload | 32392.8 | 883533.0 | 884331.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.006 | 99.3% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 4776ns | 4776ns | -99.79% |
| abi_soa_win_real_scalar_payload | 2246653ns | 2246653ns | base |
| abi_soa_win_real_soa_payload | 886714ns | 886714ns | -60.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2148534ns | base | --- | [2141791, 2440843] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 2496ns | -2146046.5ns (-99.9%) | [-2438326, -2139262]ns | [2478, 2560] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 884203ns | -1264269.8ns (-58.8%) | [-1556876, -1257029]ns | [883387, 885403] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2141433ns | -99.9% | -58.6% |
| 2 | 2144270ns | -99.9% | -58.8% |
| 3 | 2142148ns | -99.9% | -58.8% |
| 4 | 2152798ns | -99.9% | -58.9% |
| 5 | 2679450ns | -99.9% | -67.0% |
| 6 | 2202236ns | -99.9% | -59.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.705 | HIGH- (thermal bounce) |
| abi_soa_win_real_scalar_payload | -0.122 | ok |
| abi_soa_win_real_soa_payload | -0.152 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 117910.5ns | 2511.1ns | 4695.5% | HIGH |
| abi_soa_win_real_scalar_payload | 6734416.9ns | 2243722.6ns | 300.1% | HIGH |
| abi_soa_win_real_soa_payload | 2684149.6ns | 884331.1ns | 303.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 2477.5-2559.8 ns)
   2477.5 |########################################
   2481.6 |
   2485.7 |
   2489.8 |
   2494.0 |########################################
   2498.1 |
   2502.2 |
   2506.3 |
   2510.4 |
   2514.5 |
   2518.7 |
   2522.8 |
   2526.9 |
   2531.0 |
   2535.1 |
   2539.2 |
   2543.3 |
   2547.5 |
   2551.6 |
   2555.7 |####################
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2141433.3-2440843.1 ns)
  2141433.3 |########################################
  2156403.8 |
  2171374.3 |
  2186344.8 |
  2201315.3 |##########
  2216285.8 |
  2231256.2 |
  2246226.7 |
  2261197.2 |
  2276167.7 |
  2291138.2 |
  2306108.7 |
  2321079.2 |
  2336049.7 |
  2351020.2 |
  2365990.6 |
  2380961.1 |
  2395931.6 |
  2410902.1 |
  2425872.6 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 883372.9-885403.3 ns)
  883372.9 |########################################
  883474.4 |
  883575.9 |
  883677.5 |
  883779.0 |####################
  883880.5 |
  883982.0 |
  884083.6 |
  884185.1 |
  884286.6 |
  884388.1 |
  884489.6 |####################
  884591.2 |####################
  884692.7 |
  884794.2 |
  884895.7 |
  884997.3 |
  885098.8 |
  885200.3 |
  885301.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=4746.6% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=303.7% of algo (FFI overhead may distort results)
