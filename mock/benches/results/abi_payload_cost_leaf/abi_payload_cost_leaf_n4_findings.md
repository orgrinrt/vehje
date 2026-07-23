# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (27.18 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.59 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 150% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.59 us) leads abi_payload_cost_leaf_soa_payload (6.48 us) by 150%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 91% (significant)

abi_payload_cost_leaf_null_entry is -24.60 us (91%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 10.5x slower than the field

abi_payload_cost_leaf_scalar_payload (27.18 us) is 10.5x the fastest (2.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_leaf_null_entry shows alternating (throttle bounce) (autocorr -0.56)

abi_payload_cost_leaf_null_entry's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.5x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.59 us) to slowest abi_payload_cost_leaf_scalar_payload (27.18 us): 10.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2593.1 ns median (-90.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.48x (fastest 2593.1 ns, slowest 27177.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4894ns | 4936ns | 4708ns | 4895ns | 4986ns | -83.43% |
| abi_payload_cost_leaf_scalar_payload | 29534ns | 29456ns | 29001ns | 29391ns | 30014ns | base |
| abi_payload_cost_leaf_soa_payload | 8729ns | 8754ns | 8390ns | 8687ns | 8962ns | -70.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2590ns | 2528ns | 2626ns | -90.48% | 0.002 |
| abi_payload_cost_leaf_scalar_payload | 27213ns | 26695ns | 27652ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 6445ns | 6201ns | 6607ns | -76.32% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 20386.7 | 2712.4 | 2590.2 | n/a |
| abi_payload_cost_leaf_scalar_payload | 20626.3 | 27388.8 | 27213.4 | n/a |
| abi_payload_cost_leaf_soa_payload | 20617.6 | 6549.0 | 6444.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.002 | 97.5% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 9.3% |
| abi_payload_cost_leaf_soa_payload | 0.001 | 39.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4894ns | 4894ns | -83.43% |
| abi_payload_cost_leaf_scalar_payload | 29534ns | 29534ns | base |
| abi_payload_cost_leaf_soa_payload | 8729ns | 8729ns | -70.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 27178ns | base | --- | [26811, 27652] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2593ns | -24598.3ns (-90.5%) | [-25030, -24242]ns | [2551, 2626] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 6479ns | -20696.4ns (-76.2%) | [-21146, -20464]ns | [6248, 6607] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 26695ns | -90.5% | -76.8% |
| 2 | 27983ns | -90.6% | -76.0% |
| 3 | 27228ns | -90.5% | -76.3% |
| 4 | 27320ns | -90.5% | -77.0% |
| 5 | 26926ns | -90.3% | -75.9% |
| 6 | 27128ns | -90.5% | -76.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | -0.564 | HIGH- (thermal bounce) |
| abi_payload_cost_leaf_scalar_payload | -0.408 | moderate- |
| abi_payload_cost_leaf_soa_payload | -0.424 | moderate- |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 108138.7ns | 2590.2ns | 4174.9% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 181234.2ns | 27213.4ns | 666.0% | HIGH |
| abi_payload_cost_leaf_soa_payload | 121606.8ns | 6444.7ns | 1886.9% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2528.3-2626.2 ns)
   2528.3 |########################################
   2533.2 |
   2538.1 |
   2543.0 |
   2547.9 |
   2552.8 |
   2557.7 |
   2562.6 |
   2567.5 |
   2572.4 |########################################
   2577.3 |
   2582.2 |########################################
   2587.1 |
   2592.0 |
   2596.9 |########################################
   2601.8 |
   2606.7 |########################################
   2611.6 |
   2616.5 |
   2621.4 |
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 26695.4-27651.7 ns)
  26695.4 |########################################
  26743.2 |
  26791.0 |
  26838.8 |
  26886.7 |########################################
  26934.5 |
  26982.3 |
  27030.1 |
  27077.9 |
  27125.7 |########################################
  27173.5 |
  27221.3 |########################################
  27269.2 |
  27317.0 |########################################
  27364.8 |
  27412.6 |
  27460.4 |
  27508.2 |
  27556.0 |
  27603.8 |
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 6200.8-6606.6 ns)
   6200.8 |####################
   6221.1 |
   6241.4 |
   6261.7 |
   6282.0 |####################
   6302.3 |
   6322.6 |
   6342.8 |
   6363.1 |
   6383.4 |
   6403.7 |
   6424.0 |
   6444.3 |
   6464.6 |####################
   6484.9 |########################################
   6505.2 |
   6525.5 |
   6545.8 |
   6566.1 |
   6586.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4167.6% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=664.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=1872.5% of algo (FFI overhead may distort results)
