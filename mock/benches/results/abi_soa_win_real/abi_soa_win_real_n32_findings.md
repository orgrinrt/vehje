# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.18 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 2.28 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 39431% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (2.28 us) leads abi_soa_win_real_soa_payload (902.71 us) by 39431%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.18 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 953.8x slower than the field

abi_soa_win_real_scalar_payload (2.18 ms) is 953.8x the fastest (2.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 953.8x the fastest

Fastest abi_soa_win_real_null_entry (2.28 us) to slowest abi_soa_win_real_scalar_payload (2.18 ms): 953.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 2283.6 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 953.80x (fastest 2283.6 ns, slowest 2178057.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 4598ns | 4598ns | 4409ns | 4574ns | 4729ns | -99.79% |
| abi_soa_win_real_scalar_payload | 2207083ns | 2181701ns | 2171319ns | 2179185ns | 2266812ns | base |
| abi_soa_win_real_soa_payload | 903812ns | 905750ns | 895468ns | 902699ns | 909654ns | -59.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 2287ns | 2224ns | 2338ns | -99.90% | 0.014 |
| abi_soa_win_real_scalar_payload | 2203461ns | 2167928ns | 2262915ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 900825ns | 892649ns | 906618ns | -59.12% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 28894.3 | 2436.0 | 2287.0 | n/a |
| abi_soa_win_real_scalar_payload | 73026.5 | 2211616.2 | 2203461.0 | n/a |
| abi_soa_win_real_soa_payload | 54465.6 | 901133.0 | 900825.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.014 | 97.4% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 4598ns | 4598ns | -99.79% |
| abi_soa_win_real_scalar_payload | 2207083ns | 2207083ns | base |
| abi_soa_win_real_soa_payload | 903812ns | 903812ns | -59.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2178058ns | base | --- | [2169410, 2262915] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 2284ns | -2175818.1ns (-99.9%) | [-2260602, -2167102]ns | [2239, 2338] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 902714ns | -1276119.0ns (-58.6%) | [-1366251, -1265537]ns | [893144, 906618] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2170892ns | -99.9% | -58.5% |
| 2 | 2172964ns | -99.9% | -58.3% |
| 3 | 2183151ns | -99.9% | -59.1% |
| 4 | 2335641ns | -99.9% | -61.8% |
| 5 | 2167928ns | -99.9% | -58.3% |
| 6 | 2190190ns | -99.9% | -58.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | 0.456 | moderate+ |
| abi_soa_win_real_scalar_payload | -0.249 | moderate- |
| abi_soa_win_real_soa_payload | 0.110 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 117936.6ns | 2287.0ns | 5156.8% | HIGH |
| abi_soa_win_real_scalar_payload | 6716701.7ns | 2203461.0ns | 304.8% | HIGH |
| abi_soa_win_real_soa_payload | 2762101.9ns | 900825.3ns | 306.6% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 2224.2-2338.1 ns)
   2224.2 |########################################
   2229.9 |
   2235.6 |
   2241.3 |
   2247.0 |
   2252.7 |########################################
   2258.4 |
   2264.1 |########################################
   2269.8 |
   2275.5 |
   2281.2 |
   2286.8 |
   2292.5 |
   2298.2 |########################################
   2303.9 |
   2309.6 |
   2315.3 |
   2321.0 |########################################
   2326.7 |
   2332.4 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2167928.3-2262915.2 ns)
  2167928.3 |########################################
  2172677.6 |####################
  2177427.0 |
  2182176.3 |####################
  2186925.7 |####################
  2191675.0 |
  2196424.4 |
  2201173.7 |
  2205923.1 |
  2210672.4 |
  2215421.8 |
  2220171.1 |
  2224920.4 |
  2229669.8 |
  2234419.1 |
  2239168.5 |
  2243917.8 |
  2248667.2 |
  2253416.5 |
  2258165.9 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 892649.2-906617.5 ns)
  892649.2 |########################################
  893347.6 |########################################
  894046.0 |
  894744.4 |
  895442.9 |
  896141.3 |
  896839.7 |
  897538.1 |
  898236.5 |
  898934.9 |
  899633.3 |
  900331.8 |
  901030.2 |########################################
  901728.6 |
  902427.0 |
  903125.4 |
  903823.8 |########################################
  904522.3 |
  905220.7 |########################################
  905919.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=5153.6% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=306.1% of algo (FFI overhead may distort results)
