# abi_payload_cost (madd)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_madd_scalar_payload has the worst median (539.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_madd_null_entry at 2.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_madd_null_entry dominates: 9252% faster than the next best (abi_payload_cost_madd_soa_payload)

abi_payload_cost_madd_null_entry (2.55 us) leads abi_payload_cost_madd_soa_payload (238.33 us) by 9252%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_madd_null_entry beats baseline by 100% (significant)

abi_payload_cost_madd_null_entry is -536.51 us (100%) faster than baseline abi_payload_cost_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_madd_scalar_payload is an outlier: 211.5x slower than the field

abi_payload_cost_madd_scalar_payload (539.09 us) is 211.5x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 211.5x the fastest

Fastest abi_payload_cost_madd_null_entry (2.55 us) to slowest abi_payload_cost_madd_scalar_payload (539.09 us): 211.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_madd_null_entry** at 2548.6 ns median (-99.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 211.53x (fastest 2548.6 ns, slowest 539091.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4827ns | 4820ns | 4645ns | 4773ns | 4998ns | -99.11% |
| abi_payload_cost_madd_scalar_payload | 541633ns | 541536ns | 535170ns | 540347ns | 546794ns | base |
| abi_payload_cost_madd_soa_payload | 241617ns | 240600ns | 239622ns | 240287ns | 244610ns | -55.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 2547ns | 2435ns | 2629ns | -99.53% | 0.025 |
| abi_payload_cost_madd_scalar_payload | 539264ns | 532925ns | 544431ns | base | 0.000 |
| abi_payload_cost_madd_soa_payload | 239369ns | 237480ns | 242266ns | -55.61% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 22068.8 | 2707.6 | 2547.2 | n/a |
| abi_payload_cost_madd_scalar_payload | 25593.1 | 537511.8 | 539264.1 | n/a |
| abi_payload_cost_madd_soa_payload | 22792.6 | 238870.0 | 239368.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_payload_cost_madd_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_madd_null_entry | 0.025 | 95.5% |
| abi_payload_cost_madd_scalar_payload | 0.000 | 0.5% |
| abi_payload_cost_madd_soa_payload | 0.000 | 1.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4827ns | 4827ns | -99.11% |
| abi_payload_cost_madd_scalar_payload | 541633ns | 541633ns | base |
| abi_payload_cost_madd_soa_payload | 241617ns | 241617ns | -55.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_madd_scalar_payload | 539091ns | base | --- | [534270, 544431] | --- | --- | --- | --- |
| abi_payload_cost_madd_null_entry | 2549ns | -536511.2ns (-99.5%) | [-541967, -531672]ns | [2464, 2629] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_madd_soa_payload | 238331ns | -298388.8ns (-55.4%) | [-306131, -295166]ns | [237510, 242266] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_madd_scalar_payload | abi_payload_cost_madd_null_entry | abi_payload_cost_madd_soa_payload |
|---|---|---|---|
| 1 | 547988ns | -99.6% | -56.4% |
| 2 | 540875ns | -99.5% | -56.1% |
| 3 | 532925ns | -99.5% | -55.4% |
| 4 | 537824ns | -99.5% | -55.8% |
| 5 | 535615ns | -99.5% | -55.4% |
| 6 | 540358ns | -99.5% | -54.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_madd_null_entry | 0.029 | ok |
| abi_payload_cost_madd_scalar_payload | 0.105 | ok |
| abi_payload_cost_madd_soa_payload | 0.091 | ok |

**Consistency summary:**

- **abi_payload_cost_madd_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 108539.2ns | 2547.2ns | 4261.1% | HIGH |
| abi_payload_cost_madd_scalar_payload | 1642732.4ns | 539264.1ns | 304.6% | HIGH |
| abi_payload_cost_madd_soa_payload | 740907.2ns | 239368.9ns | 309.5% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_madd_null_entry (n=6, range 2435.0-2629.4 ns)
   2435.0 |####################
   2444.7 |
   2454.4 |
   2464.2 |
   2473.9 |
   2483.6 |####################
   2493.3 |
   2503.0 |
   2512.8 |
   2522.5 |
   2532.2 |
   2541.9 |########################################
   2551.6 |
   2561.4 |
   2571.1 |
   2580.8 |
   2590.5 |
   2600.2 |
   2610.0 |####################
   2619.7 |
  (0 below, 1 above range)

abi_payload_cost_madd_scalar_payload (n=6, range 532925.0-544431.1 ns)
  532925.0 |########################################
  533500.3 |
  534075.6 |
  534650.9 |
  535226.2 |########################################
  535801.5 |
  536376.8 |
  536952.1 |
  537527.4 |########################################
  538102.7 |
  538678.0 |
  539253.3 |
  539828.6 |########################################
  540403.9 |########################################
  540979.2 |
  541554.5 |
  542129.8 |
  542705.1 |
  543280.4 |
  543855.7 |
  (0 below, 1 above range)

abi_payload_cost_madd_soa_payload (n=6, range 237480.4-242266.0 ns)
  237480.4 |########################################
  237719.7 |####################
  237959.0 |
  238198.2 |
  238437.5 |
  238676.8 |####################
  238916.1 |####################
  239155.4 |
  239394.6 |
  239633.9 |
  239873.2 |
  240112.5 |
  240351.8 |
  240591.0 |
  240830.3 |
  241069.6 |
  241308.9 |
  241548.2 |
  241787.4 |
  242026.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_madd_null_entry**: bridge=4242.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_scalar_payload**: bridge=305.1% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_soa_payload**: bridge=309.5% of algo (FFI overhead may distort results)
