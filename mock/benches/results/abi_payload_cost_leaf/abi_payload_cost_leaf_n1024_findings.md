# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (6.07 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 131836% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.48 us) leads abi_payload_cost_leaf_soa_payload (3.28 ms) by 131836%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 100% (significant)

abi_payload_cost_leaf_null_entry is -6.07 ms (100%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 2444.0x slower than the field

abi_payload_cost_leaf_scalar_payload (6.07 ms) is 2444.0x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 2444.0x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.48 us) to slowest abi_payload_cost_leaf_scalar_payload (6.07 ms): 2444.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2482.8 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2444.04x (fastest 2482.8 ns, slowest 6067951.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4654ns | 4678ns | 4541ns | 4648ns | 4719ns | -99.92% |
| abi_payload_cost_leaf_scalar_payload | 6183784ns | 6070843ns | 6046686ns | 6069976ns | 6423047ns | base |
| abi_payload_cost_leaf_soa_payload | 3316820ns | 3278352ns | 3260209ns | 3276736ns | 3405254ns | -46.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2446ns | 2355ns | 2490ns | -99.96% | 0.419 |
| abi_payload_cost_leaf_scalar_payload | 6180743ns | 6043950ns | 6419739ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 3314086ns | 3257613ns | 3402288ns | -46.38% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 45395.0 | 2641.8 | 2445.9 | n/a |
| abi_payload_cost_leaf_scalar_payload | 81782.7 | 6158023.0 | 6180742.9 | n/a |
| abi_payload_cost_leaf_soa_payload | 65519.2 | 3330945.4 | 3314086.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.435 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.412 | 94.9% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 0.0% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4654ns | 4654ns | -99.92% |
| abi_payload_cost_leaf_scalar_payload | 6183784ns | 6183784ns | base |
| abi_payload_cost_leaf_soa_payload | 3316820ns | 3316820ns | -46.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 6067951ns | base | --- | [6054539, 6419739] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2483ns | -6065531.5ns (-100.0%) | [-6417249, -6052110]ns | [2365, 2490] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 3275646ns | -2800071.3ns (-46.1%) | [-3024696, -2775203]ns | [3264324, 3402288] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 6065128ns | -100.0% | -46.3% |
| 2 | 6100647ns | -100.0% | -46.3% |
| 3 | 6069899ns | -100.0% | -46.0% |
| 4 | 6043950ns | -100.0% | -45.9% |
| 5 | 6738831ns | -100.0% | -47.8% |
| 6 | 6066003ns | -100.0% | -45.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | -0.369 | moderate- |
| abi_payload_cost_leaf_scalar_payload | -0.285 | moderate- |
| abi_payload_cost_leaf_soa_payload | -0.173 | ok |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 131381.7ns | 2445.9ns | 5371.5% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 18646722.9ns | 6180742.9ns | 301.7% | HIGH |
| abi_payload_cost_leaf_soa_payload | 10030698.8ns | 3314086.1ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2355.4-2489.6 ns)
   2355.4 |####################
   2362.1 |
   2368.8 |####################
   2375.5 |
   2382.2 |
   2388.9 |
   2395.6 |
   2402.4 |
   2409.1 |
   2415.8 |
   2422.5 |
   2429.2 |
   2435.9 |
   2442.6 |
   2449.3 |
   2456.0 |
   2462.7 |
   2469.4 |
   2476.1 |####################
   2482.8 |########################################
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 6043949.6-6419739.0 ns)
  6043949.6 |#############
  6062739.1 |########################################
  6081528.5 |
  6100318.0 |#############
  6119107.5 |
  6137896.9 |
  6156686.4 |
  6175475.9 |
  6194265.3 |
  6213054.8 |
  6231844.3 |
  6250633.7 |
  6269423.2 |
  6288212.7 |
  6307002.1 |
  6325791.6 |
  6344581.1 |
  6363370.5 |
  6382160.0 |
  6400949.5 |
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 3257612.9-3402288.4 ns)
  3257612.9 |####################
  3264846.7 |####################
  3272080.4 |########################################
  3279314.2 |
  3286548.0 |####################
  3293781.8 |
  3301015.5 |
  3308249.3 |
  3315483.1 |
  3322716.9 |
  3329950.6 |
  3337184.4 |
  3344418.2 |
  3351651.9 |
  3358885.7 |
  3366119.5 |
  3373353.3 |
  3380587.0 |
  3387820.8 |
  3395054.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=5273.5% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=301.9% of algo (FFI overhead may distort results)
