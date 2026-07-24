# abi_payload_cost (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_leaf_scalar_payload has the worst median (312.85 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_leaf_null_entry at 2.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_leaf_null_entry dominates: 7558% faster than the next best (abi_payload_cost_leaf_soa_payload)

abi_payload_cost_leaf_null_entry (2.55 us) leads abi_payload_cost_leaf_soa_payload (195.16 us) by 7558%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_leaf_null_entry beats baseline by 99% (significant)

abi_payload_cost_leaf_null_entry is -310.19 us (99%) faster than baseline abi_payload_cost_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_leaf_scalar_payload is an outlier: 122.8x slower than the field

abi_payload_cost_leaf_scalar_payload (312.85 us) is 122.8x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_leaf_soa_payload shows alternating (throttle bounce) (autocorr -0.51)

abi_payload_cost_leaf_soa_payload's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 122.8x the fastest

Fastest abi_payload_cost_leaf_null_entry (2.55 us) to slowest abi_payload_cost_leaf_scalar_payload (312.85 us): 122.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_leaf_null_entry** at 2548.6 ns median (-99.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 122.75x (fastest 2548.6 ns, slowest 312847.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4810ns | 4811ns | 4527ns | 4748ns | 5045ns | -98.47% |
| abi_payload_cost_leaf_scalar_payload | 313547ns | 315169ns | 307322ns | 313770ns | 316324ns | base |
| abi_payload_cost_leaf_soa_payload | 197417ns | 197437ns | 196068ns | 197201ns | 198414ns | -37.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 2544ns | 2399ns | 2653ns | -99.18% | 0.025 |
| abi_payload_cost_leaf_scalar_payload | 311229ns | 305093ns | 313948ns | base | 0.000 |
| abi_payload_cost_leaf_soa_payload | 195170ns | 193940ns | 196171ns | -37.29% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 21686.5 | 2702.3 | 2543.7 | n/a |
| abi_payload_cost_leaf_scalar_payload | 23832.0 | 311080.0 | 311228.8 | n/a |
| abi_payload_cost_leaf_soa_payload | 21976.5 | 194876.0 | 195169.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_payload_cost_leaf_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.025 | 94.1% |
| abi_payload_cost_leaf_scalar_payload | 0.000 | 0.8% |
| abi_payload_cost_leaf_soa_payload | 0.000 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 4810ns | 4810ns | -98.47% |
| abi_payload_cost_leaf_scalar_payload | 313547ns | 313547ns | base |
| abi_payload_cost_leaf_soa_payload | 197417ns | 197417ns | -37.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_leaf_scalar_payload | 312847ns | base | --- | [306892, 313948] | --- | --- | --- | --- |
| abi_payload_cost_leaf_null_entry | 2549ns | -310194.3ns (-99.2%) | [-311460, -304401]ns | [2430, 2653] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_leaf_soa_payload | 195157ns | -117508.3ns (-37.6%) | [-119448, -111222]ns | [194181, 196171] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_leaf_scalar_payload | abi_payload_cost_leaf_null_entry | abi_payload_cost_leaf_soa_payload |
|---|---|---|---|
| 1 | 313106ns | -99.2% | -38.1% |
| 2 | 312823ns | -99.1% | -37.3% |
| 3 | 312871ns | -99.2% | -37.9% |
| 4 | 314789ns | -99.2% | -38.0% |
| 5 | 308691ns | -99.2% | -36.5% |
| 6 | 305093ns | -99.2% | -36.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_leaf_null_entry | 0.115 | ok |
| abi_payload_cost_leaf_scalar_payload | 0.275 | moderate+ |
| abi_payload_cost_leaf_soa_payload | -0.508 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_payload_cost_leaf_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_leaf_null_entry | 108085.4ns | 2543.7ns | 4249.2% | HIGH |
| abi_payload_cost_leaf_scalar_payload | 958256.1ns | 311228.8ns | 307.9% | HIGH |
| abi_payload_cost_leaf_soa_payload | 607136.7ns | 195169.6ns | 311.1% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_leaf_null_entry (n=6, range 2398.8-2652.7 ns)
   2398.8 |########################################
   2411.5 |
   2424.2 |
   2436.9 |
   2449.6 |########################################
   2462.3 |
   2475.0 |
   2487.7 |
   2500.4 |
   2513.1 |########################################
   2525.8 |
   2538.4 |
   2551.1 |
   2563.8 |
   2576.5 |########################################
   2589.2 |
   2601.9 |
   2614.6 |
   2627.3 |
   2640.0 |########################################
  (0 below, 1 above range)

abi_payload_cost_leaf_scalar_payload (n=6, range 305092.9-313947.5 ns)
  305092.9 |####################
  305535.6 |
  305978.4 |
  306421.1 |
  306863.8 |
  307306.6 |
  307749.3 |
  308192.0 |
  308634.7 |####################
  309077.5 |
  309520.2 |
  309962.9 |
  310405.7 |
  310848.4 |
  311291.1 |
  311733.8 |
  312176.6 |
  312619.3 |########################################
  313062.0 |####################
  313504.8 |
  (0 below, 1 above range)

abi_payload_cost_leaf_soa_payload (n=6, range 193939.6-196170.6 ns)
  193939.6 |########################################
  194051.1 |
  194162.7 |
  194274.2 |
  194385.8 |########################################
  194497.4 |
  194608.9 |
  194720.5 |
  194832.0 |
  194943.6 |
  195055.1 |########################################
  195166.6 |########################################
  195278.2 |
  195389.8 |
  195501.3 |
  195612.9 |
  195724.4 |
  195836.0 |
  195947.5 |
  196059.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_leaf_null_entry**: bridge=4251.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_scalar_payload**: bridge=307.5% of algo (FFI overhead may distort results)
- **abi_payload_cost_leaf_soa_payload**: bridge=311.3% of algo (FFI overhead may distort results)
