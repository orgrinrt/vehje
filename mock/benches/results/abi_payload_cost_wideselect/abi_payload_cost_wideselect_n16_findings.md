# abi_payload_cost (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_wideselect_scalar_payload has the worst median (110.72 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_wideselect_null_entry at 2.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_wideselect_null_entry dominates: 1297% faster than the next best (abi_payload_cost_wideselect_soa_payload)

abi_payload_cost_wideselect_null_entry (2.57 us) leads abi_payload_cost_wideselect_soa_payload (35.91 us) by 1297%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_wideselect_null_entry beats baseline by 98% (significant)

abi_payload_cost_wideselect_null_entry is -108.11 us (98%) faster than baseline abi_payload_cost_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_wideselect_scalar_payload is an outlier: 43.1x slower than the field

abi_payload_cost_wideselect_scalar_payload (110.72 us) is 43.1x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 43.1x the fastest

Fastest abi_payload_cost_wideselect_null_entry (2.57 us) to slowest abi_payload_cost_wideselect_scalar_payload (110.72 us): 43.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_wideselect_null_entry** at 2570.4 ns median (-97.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 43.08x (fastest 2570.4 ns, slowest 110721.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4818ns | 4859ns | 4532ns | 4828ns | 4946ns | -95.75% |
| abi_payload_cost_wideselect_scalar_payload | 113344ns | 112920ns | 112520ns | 112831ns | 114527ns | base |
| abi_payload_cost_wideselect_soa_payload | 38033ns | 38148ns | 37451ns | 37948ns | 38450ns | -66.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 2553ns | 2405ns | 2628ns | -97.70% | 0.006 |
| abi_payload_cost_wideselect_scalar_payload | 111125ns | 110308ns | 112271ns | base | 0.000 |
| abi_payload_cost_wideselect_soa_payload | 35759ns | 35220ns | 36145ns | -67.82% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 20734.4 | 2748.4 | 2552.8 | n/a |
| abi_payload_cost_wideselect_scalar_payload | 20789.4 | 111300.7 | 111125.0 | n/a |
| abi_payload_cost_wideselect_soa_payload | 20821.1 | 35801.5 | 35759.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_payload_cost_wideselect_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | 0.006 | 93.6% |
| abi_payload_cost_wideselect_scalar_payload | 0.000 | 2.2% |
| abi_payload_cost_wideselect_soa_payload | 0.000 | 6.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 4818ns | 4818ns | -95.75% |
| abi_payload_cost_wideselect_scalar_payload | 113344ns | 113344ns | base |
| abi_payload_cost_wideselect_soa_payload | 38033ns | 38033ns | -66.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_wideselect_scalar_payload | 110722ns | base | --- | [110382, 112271] | --- | --- | --- | --- |
| abi_payload_cost_wideselect_null_entry | 2570ns | -108109.6ns (-97.6%) | [-109731, -107876]ns | [2460, 2628] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_wideselect_soa_payload | 35908ns | -74678.5ns (-67.4%) | [-77047, -74372]ns | [35224, 36145] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_wideselect_scalar_payload | abi_payload_cost_wideselect_null_entry | abi_payload_cost_wideselect_soa_payload |
|---|---|---|---|
| 1 | 110308ns | -97.8% | -67.5% |
| 2 | 112774ns | -97.8% | -68.8% |
| 3 | 110933ns | -97.6% | -67.4% |
| 4 | 110510ns | -97.6% | -67.4% |
| 5 | 110455ns | -97.7% | -67.2% |
| 6 | 111769ns | -97.7% | -68.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_wideselect_null_entry | 0.306 | moderate+ |
| abi_payload_cost_wideselect_scalar_payload | -0.335 | moderate- |
| abi_payload_cost_wideselect_soa_payload | -0.313 | moderate- |

**Consistency summary:**

- **abi_payload_cost_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_wideselect_null_entry | 106901.5ns | 2552.8ns | 4187.7% | HIGH |
| abi_payload_cost_wideselect_scalar_payload | 355493.2ns | 111125.0ns | 319.9% | HIGH |
| abi_payload_cost_wideselect_soa_payload | 200371.0ns | 35759.0ns | 560.3% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_wideselect_null_entry (n=6, range 2405.4-2627.5 ns)
   2405.4 |########################################
   2416.5 |
   2427.6 |
   2438.7 |
   2449.8 |
   2460.9 |
   2472.0 |
   2483.1 |
   2494.2 |
   2505.3 |########################################
   2516.4 |
   2527.6 |
   2538.7 |
   2549.8 |
   2560.9 |########################################
   2572.0 |########################################
   2583.1 |
   2594.2 |
   2605.3 |
   2616.4 |########################################
  (0 below, 1 above range)

abi_payload_cost_wideselect_scalar_payload (n=6, range 110308.3-112271.4 ns)
  110308.3 |########################################
  110406.5 |########################################
  110504.6 |########################################
  110602.8 |
  110700.9 |
  110799.1 |
  110897.2 |########################################
  110995.4 |
  111093.6 |
  111191.7 |
  111289.9 |
  111388.0 |
  111486.2 |
  111584.3 |
  111682.5 |########################################
  111780.7 |
  111878.8 |
  111977.0 |
  112075.1 |
  112173.3 |
  (0 below, 1 above range)

abi_payload_cost_wideselect_soa_payload (n=6, range 35219.6-36145.2 ns)
  35219.6 |########################################
  35265.9 |
  35312.2 |
  35358.4 |
  35404.7 |
  35451.0 |
  35497.3 |
  35543.6 |
  35589.9 |
  35636.1 |
  35682.4 |
  35728.7 |
  35775.0 |
  35821.3 |####################
  35867.6 |
  35913.8 |
  35960.1 |####################
  36006.4 |
  36052.7 |
  36099.0 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_wideselect_null_entry**: bridge=4171.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_scalar_payload**: bridge=320.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_wideselect_soa_payload**: bridge=556.8% of algo (FFI overhead may distort results)
