# abi_payload_cost (madd)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_madd_scalar_payload has the worst median (101.34 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_madd_null_entry at 2.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_madd_null_entry dominates: 1717% faster than the next best (abi_payload_cost_madd_soa_payload)

abi_payload_cost_madd_null_entry (2.56 us) leads abi_payload_cost_madd_soa_payload (46.45 us) by 1717%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_madd_null_entry beats baseline by 97% (significant)

abi_payload_cost_madd_null_entry is -98.73 us (97%) faster than baseline abi_payload_cost_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_madd_scalar_payload is an outlier: 39.6x slower than the field

abi_payload_cost_madd_scalar_payload (101.34 us) is 39.6x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 39.6x the fastest

Fastest abi_payload_cost_madd_null_entry (2.56 us) to slowest abi_payload_cost_madd_scalar_payload (101.34 us): 39.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_madd_null_entry** at 2556.7 ns median (-97.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 39.64x (fastest 2556.7 ns, slowest 101340.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4805ns | 4850ns | 4539ns | 4770ns | 4990ns | -95.37% |
| abi_payload_cost_madd_scalar_payload | 103666ns | 103579ns | 103377ns | 103536ns | 104004ns | base |
| abi_payload_cost_madd_soa_payload | 48489ns | 48673ns | 47495ns | 48459ns | 49031ns | -53.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 2544ns | 2405ns | 2645ns | -97.49% | 0.006 |
| abi_payload_cost_madd_scalar_payload | 101408ns | 101095ns | 101737ns | base | 0.000 |
| abi_payload_cost_madd_soa_payload | 46237ns | 45283ns | 46734ns | -54.40% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 20544.8 | 2727.5 | 2544.0 | n/a |
| abi_payload_cost_madd_scalar_payload | 21318.8 | 101358.3 | 101407.6 | n/a |
| abi_payload_cost_madd_soa_payload | 19879.6 | 46058.5 | 46236.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_payload_cost_madd_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_madd_null_entry | 0.006 | 94.1% |
| abi_payload_cost_madd_scalar_payload | 0.000 | 2.4% |
| abi_payload_cost_madd_soa_payload | 0.000 | 5.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_madd_null_entry | 4805ns | 4805ns | -95.37% |
| abi_payload_cost_madd_scalar_payload | 103666ns | 103666ns | base |
| abi_payload_cost_madd_soa_payload | 48489ns | 48489ns | -53.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_madd_scalar_payload | 101341ns | base | --- | [101145, 101737] | --- | --- | --- | --- |
| abi_payload_cost_madd_null_entry | 2557ns | -98733.3ns (-97.4%) | [-99307, -98551]ns | [2430, 2645] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_madd_soa_payload | 46450ns | -55014.4ns (-54.3%) | [-55719, -54779]ns | [45527, 46734] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_madd_scalar_payload | abi_payload_cost_madd_null_entry | abi_payload_cost_madd_soa_payload |
|---|---|---|---|
| 1 | 101635ns | -97.6% | -54.2% |
| 2 | 101095ns | -97.4% | -54.1% |
| 3 | 101839ns | -97.6% | -53.9% |
| 4 | 101385ns | -97.5% | -54.1% |
| 5 | 101296ns | -97.3% | -54.8% |
| 6 | 101195ns | -97.5% | -55.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_madd_null_entry | -0.128 | ok |
| abi_payload_cost_madd_scalar_payload | -0.481 | moderate- |
| abi_payload_cost_madd_soa_payload | 0.366 | moderate+ |

**Consistency summary:**

- **abi_payload_cost_madd_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_madd_null_entry | 106688.5ns | 2544.0ns | 4193.8% | HIGH |
| abi_payload_cost_madd_scalar_payload | 326320.6ns | 101407.6ns | 321.8% | HIGH |
| abi_payload_cost_madd_soa_payload | 205000.1ns | 46236.9ns | 443.4% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_madd_null_entry (n=6, range 2404.6-2645.0 ns)
   2404.6 |########################################
   2416.6 |
   2428.6 |
   2440.7 |
   2452.7 |########################################
   2464.7 |
   2476.7 |
   2488.7 |
   2500.8 |
   2512.8 |
   2524.8 |
   2536.8 |########################################
   2548.8 |
   2560.9 |########################################
   2572.9 |
   2584.9 |########################################
   2596.9 |
   2608.9 |
   2621.0 |
   2633.0 |
  (0 below, 1 above range)

abi_payload_cost_madd_scalar_payload (n=6, range 101095.0-101737.1 ns)
  101095.0 |########################################
  101127.1 |
  101159.2 |
  101191.3 |########################################
  101223.4 |
  101255.5 |
  101287.6 |########################################
  101319.7 |
  101351.8 |
  101383.9 |########################################
  101416.1 |
  101448.2 |
  101480.3 |
  101512.4 |
  101544.5 |
  101576.6 |
  101608.7 |########################################
  101640.8 |
  101672.9 |
  101705.0 |
  (0 below, 1 above range)

abi_payload_cost_madd_soa_payload (n=6, range 45282.9-46733.6 ns)
  45282.9 |########################################
  45355.4 |
  45428.0 |
  45500.5 |
  45573.0 |
  45645.6 |
  45718.1 |########################################
  45790.6 |
  45863.2 |
  45935.7 |
  46008.2 |
  46080.8 |
  46153.3 |
  46225.8 |
  46298.4 |
  46370.9 |########################################
  46443.4 |########################################
  46516.0 |########################################
  46588.5 |
  46661.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_madd_null_entry**: bridge=4168.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_scalar_payload**: bridge=321.3% of algo (FFI overhead may distort results)
- **abi_payload_cost_madd_soa_payload**: bridge=442.8% of algo (FFI overhead may distort results)
