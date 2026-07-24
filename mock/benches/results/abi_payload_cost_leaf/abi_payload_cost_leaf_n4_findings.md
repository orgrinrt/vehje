# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (26.60 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.62 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 149% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.62 us) leads abi_payload_cost_leaf_soa_payload (6.52 us) by 149%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 90% (significant)

abi_payload_cost_leaf_null_entry is -24.05 us (90%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 10.2x slower than the field

abi_payload_cost_leaf_scalar_payload (26.60 us) is 10.2x the fastest (2.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_leaf_scalar_payload shows alternating (throttle bounce) (autocorr -0.64)

abi_payload_cost_leaf_scalar_payload's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.2x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.62 us) to slowest abi_payload_cost_leaf_scalar_payload (26.60 us): 10.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2620.2 ns median (-90.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.15x (fastest 2620.2 ns, slowest 26604.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4923ns | 4921ns | 4730ns | 4908ns | 5043ns | -83.01% |
| abi_payload_cost_leaf_scalar_payload | 28974ns | 28843ns | 28572ns | 28788ns | 29453ns | base |
| abi_payload_cost_leaf_soa_payload | 8844ns | 8814ns | 8646ns | 8760ns | 9070ns | -69.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2617ns | 2500ns | 2679ns | -90.21% | 0.002 |
| abi_payload_cost_leaf_scalar_payload | 26728ns | 26341ns | 27186ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 6529ns | 6374ns | 6690ns | -75.57% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 20758.1 | 2820.7 | 2616.7 | 0 |
| abi_payload_cost_leaf_scalar_payload | 20735.3 | 26848.9 | 26728.4 | n/a |
| abi_payload_cost_leaf_soa_payload | 20850.6 | 6637.7 | 6529.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.002 | 95.4% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 9.4% |
| abi_payload_cost_leaf_soa_payload | 0.001 | 38.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4923ns | 4923ns | -83.01% |
| abi_payload_cost_leaf_scalar_payload | 28974ns | 28974ns | base |
| abi_payload_cost_leaf_soa_payload | 8844ns | 8844ns | -69.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 26605ns | base | --- | [26395, 27186] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2620ns | -24048.2ns (-90.4%) | [-24571, -23716]ns | [2551, 2679] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 6522ns | -20092.7ns (-75.5%) | [-20664, -19841]ns | [6375, 6690] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 26595ns | -90.6% | -76.0% |
| 2 | 26614ns | -90.2% | -74.7% |
| 3 | 26448ns | -90.1% | -74.9% |
| 4 | 27192ns | -90.3% | -75.6% |
| 5 | 26341ns | -89.6% | -75.8% |
| 6 | 27180ns | -90.4% | -76.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.006 | ok |
| abi_payload_cost_leaf_scalar_payload | -0.645 | HIGH- (thermal bounce) |
| abi_payload_cost_leaf_soa_payload | 0.067 | ok |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 109072.8ns | 2616.7ns | 4168.4% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 180658.5ns | 26728.4ns | 675.9% | HIGH |
| abi_payload_cost_leaf_soa_payload | 121245.6ns | 6529.2ns | 1857.0% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2500.0-2678.6 ns)
   2500.0 |####################
   2508.9 |
   2517.9 |
   2526.8 |
   2535.7 |
   2544.6 |
   2553.6 |
   2562.5 |
   2571.4 |
   2580.3 |
   2589.3 |
   2598.2 |####################
   2607.1 |####################
   2616.1 |
   2625.0 |########################################
   2633.9 |
   2642.8 |
   2651.8 |
   2660.7 |
   2669.6 |
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 26340.8-27186.0 ns)
  26340.8 |####################
  26383.1 |
  26425.3 |####################
  26467.6 |
  26509.8 |
  26552.1 |
  26594.4 |########################################
  26636.6 |
  26678.9 |
  26721.2 |
  26763.4 |
  26805.7 |
  26848.0 |
  26890.2 |
  26932.5 |
  26974.7 |
  27017.0 |
  27059.3 |
  27101.5 |
  27143.8 |####################
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 6374.2-6689.8 ns)
   6374.2 |########################################
   6390.0 |
   6405.8 |####################
   6421.5 |
   6437.3 |
   6453.1 |
   6468.9 |
   6484.7 |
   6500.4 |
   6516.2 |
   6532.0 |
   6547.8 |
   6563.6 |
   6579.3 |
   6595.1 |
   6610.9 |
   6626.7 |####################
   6642.5 |####################
   6658.2 |
   6674.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4154.0% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=679.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=1858.2% of algo (FFI overhead may distort results)
