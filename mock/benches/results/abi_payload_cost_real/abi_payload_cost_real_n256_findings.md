# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 35163% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.52 us) leads abi_payload_cost_real_soa_payload (887.52 us) by 35163%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 100% (significant)

abi_payload_cost_real_null_entry is -2.14 ms (100%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 853.1x slower than the field

abi_payload_cost_real_scalar_payload (2.15 ms) is 853.1x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 853.1x the fastest

Fastest abi_payload_cost_real_null_entry (2.52 us) to slowest abi_payload_cost_real_scalar_payload (2.15 ms): 853.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2516.8 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 853.14x (fastest 2516.8 ns, slowest 2147221.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4794ns | 4784ns | 4658ns | 4752ns | 4924ns | -99.78% |
| abi_payload_cost_real_scalar_payload | 2165454ns | 2150072ns | 2139682ns | 2147578ns | 2205155ns | base |
| abi_payload_cost_real_soa_payload | 945363ns | 890171ns | 885964ns | 889346ns | 1059088ns | -56.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2507ns | 2430ns | 2575ns | -99.88% | 0.102 |
| abi_payload_cost_real_scalar_payload | 2162484ns | 2137275ns | 2201709ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 942316ns | 883558ns | 1055040ns | -56.42% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 27889.4 | 2722.0 | 2507.3 | n/a |
| abi_payload_cost_real_scalar_payload | 52650.2 | 2172503.0 | 2162484.0 | n/a |
| abi_payload_cost_real_soa_payload | 50438.9 | 929584.3 | 942315.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.105 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.102 | 96.5% |
| abi_payload_cost_real_scalar_payload | 0.000 | 0.1% |
| abi_payload_cost_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4794ns | 4794ns | -99.78% |
| abi_payload_cost_real_scalar_payload | 2165454ns | 2165454ns | base |
| abi_payload_cost_real_soa_payload | 945363ns | 945363ns | -56.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 2147222ns | base | --- | [2138522, 2201709] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2517ns | -2144722.3ns (-99.9%) | [-2199154, -2136054]ns | [2430, 2575] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 887521ns | -1256484.4ns (-58.5%) | [-1268529, -1135492]ns | [884386, 1055040] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 2137275ns | -99.9% | -57.4% |
| 2 | 2139769ns | -99.9% | -58.5% |
| 3 | 2159628ns | -99.9% | -58.9% |
| 4 | 2243789ns | -99.9% | -46.5% |
| 5 | 2148083ns | -99.9% | -58.9% |
| 6 | 2146361ns | -99.9% | -58.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | -0.482 | moderate- |
| abi_payload_cost_real_scalar_payload | -0.065 | ok |
| abi_payload_cost_real_soa_payload | -0.265 | moderate- |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 113633.8ns | 2507.3ns | 4532.0% | HIGH |
| abi_payload_cost_real_scalar_payload | 6607081.7ns | 2162484.0ns | 305.5% | HIGH |
| abi_payload_cost_real_soa_payload | 2870763.3ns | 942315.8ns | 304.6% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2429.6-2574.8 ns)
   2429.6 |########################################
   2436.9 |
   2444.1 |
   2451.4 |
   2458.6 |
   2465.9 |
   2473.2 |
   2480.4 |
   2487.7 |
   2494.9 |
   2502.2 |####################
   2509.5 |
   2516.7 |
   2524.0 |####################
   2531.2 |
   2538.5 |
   2545.8 |
   2553.0 |
   2560.3 |
   2567.5 |####################
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 2137274.6-2201708.5 ns)
  2137274.6 |########################################
  2140496.3 |
  2143718.0 |####################
  2146939.7 |####################
  2150161.4 |
  2153383.1 |
  2156604.8 |####################
  2159826.5 |
  2163048.2 |
  2166269.9 |
  2169491.6 |
  2172713.3 |
  2175935.0 |
  2179156.7 |
  2182378.4 |
  2185600.1 |
  2188821.8 |
  2192043.5 |
  2195265.2 |
  2198486.9 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 883557.9-1055040.4 ns)
  883557.9 |########################################
  892132.0 |
  900706.2 |
  909280.3 |##########
  917854.4 |
  926428.5 |
  935002.7 |
  943576.8 |
  952150.9 |
  960725.0 |
  969299.1 |
  977873.3 |
  986447.4 |
  995021.5 |
  1003595.6 |
  1012169.8 |
  1020743.9 |
  1029318.0 |
  1037892.1 |
  1046466.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4503.8% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=305.1% of algo (FFI overhead may distort results)
