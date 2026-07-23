# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (6.29 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.45 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 135712% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.45 us) leads abi_payload_cost_leaf_soa_payload (3.33 ms) by 135712%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 100% (significant)

abi_payload_cost_leaf_null_entry is -6.29 ms (100%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 2567.7x slower than the field

abi_payload_cost_leaf_scalar_payload (6.29 ms) is 2567.7x the fastest (2.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 2567.7x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.45 us) to slowest abi_payload_cost_leaf_scalar_payload (6.29 ms): 2567.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2448.8 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2567.74x (fastest 2448.8 ns, slowest 6287746.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4764ns | 4741ns | 4556ns | 4717ns | 4938ns | -99.92% |
| abi_payload_cost_leaf_scalar_payload | 6284647ns | 6291225ns | 6248373ns | 6287287ns | 6298825ns | base |
| abi_payload_cost_leaf_soa_payload | 3346055ns | 3328750ns | 3319690ns | 3326766ns | 3388172ns | -46.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2472ns | 2377ns | 2571ns | -99.96% | 0.414 |
| abi_payload_cost_leaf_scalar_payload | 6281188ns | 6244922ns | 6295438ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 3342973ns | 3316852ns | 3384878ns | -46.78% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 47297.8 | 2719.4 | 2471.5 | n/a |
| abi_payload_cost_leaf_scalar_payload | 99828.5 | 6276020.3 | 6281187.9 | n/a |
| abi_payload_cost_leaf_soa_payload | 81251.0 | 3341400.8 | 3342972.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.431 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.418 | 97.1% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 0.0% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4764ns | 4764ns | -99.92% |
| abi_payload_cost_leaf_scalar_payload | 6284647ns | 6284647ns | base |
| abi_payload_cost_leaf_soa_payload | 3346055ns | 3346055ns | -46.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 6287746ns | base | --- | [6260380, 6295438] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2449ns | -6285281.9ns (-100.0%) | [-6293027, -6257840]ns | [2395, 2571] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 3325702ns | -2942220.2ns (-46.8%) | [-2972262, -2900164]ns | [3318338, 3384878] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 6289287ns | -100.0% | -47.2% |
| 2 | 6293805ns | -100.0% | -47.3% |
| 3 | 6297071ns | -100.0% | -45.7% |
| 4 | 6286205ns | -100.0% | -46.7% |
| 5 | 6275837ns | -100.0% | -46.9% |
| 6 | 6244922ns | -100.0% | -46.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | -0.092 | ok |
| abi_payload_cost_leaf_scalar_payload | 0.298 | moderate+ |
| abi_payload_cost_leaf_soa_payload | -0.108 | ok |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 131610.9ns | 2471.5ns | 5325.1% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 18958436.9ns | 6281187.9ns | 301.8% | HIGH |
| abi_payload_cost_leaf_soa_payload | 10113194.6ns | 3342972.6ns | 302.5% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2377.1-2571.0 ns)
   2377.1 |########################################
   2386.8 |
   2396.5 |
   2406.2 |########################################
   2415.9 |
   2425.6 |
   2435.3 |########################################
   2445.0 |########################################
   2454.7 |
   2464.4 |
   2474.1 |
   2483.7 |
   2493.4 |
   2503.1 |
   2512.8 |########################################
   2522.5 |
   2532.2 |
   2541.9 |
   2551.6 |
   2561.3 |
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 6244922.1-6295437.9 ns)
  6244922.1 |########################################
  6247447.9 |
  6249973.7 |
  6252499.5 |
  6255025.3 |
  6257551.0 |
  6260076.8 |
  6262602.6 |
  6265128.4 |
  6267654.2 |
  6270180.0 |
  6272705.8 |
  6275231.6 |########################################
  6277757.4 |
  6280283.2 |
  6282809.0 |
  6285334.7 |########################################
  6287860.5 |########################################
  6290386.3 |
  6292912.1 |########################################
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 3316852.1-3384877.7 ns)
  3316852.1 |########################################
  3320253.4 |####################
  3323654.7 |
  3327055.9 |####################
  3330457.2 |
  3333858.5 |
  3337259.8 |
  3340661.1 |
  3344062.3 |
  3347463.6 |####################
  3350864.9 |
  3354266.2 |
  3357667.5 |
  3361068.7 |
  3364470.0 |
  3367871.3 |
  3371272.6 |
  3374673.9 |
  3378075.1 |
  3381476.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=5372.1% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=302.2% of algo (FFI overhead may distort results)
