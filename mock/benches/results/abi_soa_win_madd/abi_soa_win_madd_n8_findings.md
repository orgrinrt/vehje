# abi_soa_win (madd)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_madd_scalar_payload has the worst median (2.72 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_madd_null_entry at 3.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_madd_null_entry dominates: 35296% faster than the next best (abi_soa_win_madd_soa_payload)

abi_soa_win_madd_null_entry (3.08 us) leads abi_soa_win_madd_soa_payload (1.09 ms) by 35296%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_madd_null_entry beats baseline by 100% (significant)

abi_soa_win_madd_null_entry is -2.72 ms (100%) faster than baseline abi_soa_win_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_madd_scalar_payload is an outlier: 882.8x slower than the field

abi_soa_win_madd_scalar_payload (2.72 ms) is 882.8x the fastest (3.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 882.8x the fastest

Fastest abi_soa_win_madd_null_entry (3.08 us) to slowest abi_soa_win_madd_scalar_payload (2.72 ms): 882.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_madd_null_entry** at 3082.1 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 882.83x (fastest 3082.1 ns, slowest 2720975.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 5373ns | 5358ns | 5143ns | 5288ns | 5614ns | -99.80% |
| abi_soa_win_madd_scalar_payload | 2724073ns | 2723999ns | 2715744ns | 2722405ns | 2730740ns | base |
| abi_soa_win_madd_soa_payload | 1094214ns | 1093534ns | 1091641ns | 1093283ns | 1096897ns | -59.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 3069ns | 2941ns | 3174ns | -99.89% | 0.003 |
| abi_soa_win_madd_scalar_payload | 2720975ns | 2712861ns | 2727417ns | base | 0.000 |
| abi_soa_win_madd_soa_payload | 1091623ns | 1089089ns | 1094229ns | -59.88% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 27882.8 | 3151.5 | 3068.7 | n/a |
| abi_soa_win_madd_scalar_payload | 59615.3 | 2723324.6 | 2720975.1 | n/a |
| abi_soa_win_madd_soa_payload | 41256.4 | 1089714.9 | 1091622.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_soa_win_madd_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.003 | 95.4% |
| abi_soa_win_madd_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_madd_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_madd_null_entry | 5373ns | 5373ns | -99.80% |
| abi_soa_win_madd_scalar_payload | 2724073ns | 2724073ns | base |
| abi_soa_win_madd_soa_payload | 1094214ns | 1094214ns | -59.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_madd_scalar_payload | 2720975ns | base | --- | [2714534, 2727417] | --- | --- | --- | --- |
| abi_soa_win_madd_null_entry | 3082ns | -2717865.0ns (-99.9%) | [-2724330, -2711524]ns | [2950, 3174] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_madd_soa_payload | 1090945ns | -1629145.0ns (-59.9%) | [-1637579, -1621334]ns | [1089694, 1094229] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_madd_scalar_payload | abi_soa_win_madd_null_entry | abi_soa_win_madd_soa_payload |
|---|---|---|---|
| 1 | 2722040ns | -99.9% | -59.9% |
| 2 | 2723463ns | -99.9% | -60.0% |
| 3 | 2716206ns | -99.9% | -59.6% |
| 4 | 2719910ns | -99.9% | -59.9% |
| 5 | 2712861ns | -99.9% | -59.8% |
| 6 | 2731371ns | -99.9% | -60.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.217 | moderate+ |
| abi_soa_win_madd_scalar_payload | -0.389 | moderate- |
| abi_soa_win_madd_soa_payload | -0.429 | moderate- |

**Consistency summary:**

- **abi_soa_win_madd_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_madd_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 119730.8ns | 3068.7ns | 3901.7% | HIGH |
| abi_soa_win_madd_scalar_payload | 8222389.6ns | 2720975.1ns | 302.2% | HIGH |
| abi_soa_win_madd_soa_payload | 3311526.7ns | 1091622.6ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_madd_null_entry (n=6, range 2940.8-3173.6 ns)
   2940.8 |####################
   2952.4 |####################
   2964.1 |
   2975.7 |
   2987.4 |
   2999.0 |
   3010.6 |
   3022.3 |
   3033.9 |
   3045.5 |
   3057.2 |
   3068.8 |####################
   3080.5 |########################################
   3092.1 |
   3103.7 |
   3115.4 |
   3127.0 |
   3138.6 |
   3150.3 |
   3161.9 |
  (0 below, 1 above range)

abi_soa_win_madd_scalar_payload (n=6, range 2712861.2-2727416.8 ns)
  2712861.2 |########################################
  2713589.0 |
  2714316.8 |
  2715044.5 |
  2715772.3 |########################################
  2716500.1 |
  2717227.9 |
  2717955.7 |
  2718683.5 |
  2719411.2 |########################################
  2720139.0 |
  2720866.8 |
  2721594.6 |########################################
  2722322.4 |
  2723050.2 |########################################
  2723777.9 |
  2724505.7 |
  2725233.5 |
  2725961.3 |
  2726689.1 |
  (0 below, 1 above range)

abi_soa_win_madd_soa_payload (n=6, range 1089088.8-1094229.4 ns)
  1089088.8 |########################################
  1089345.8 |
  1089602.9 |
  1089859.9 |
  1090116.9 |########################################
  1090373.9 |########################################
  1090631.0 |
  1090888.0 |
  1091145.0 |########################################
  1091402.0 |
  1091659.1 |
  1091916.1 |
  1092173.1 |########################################
  1092430.2 |
  1092687.2 |
  1092944.2 |
  1093201.2 |
  1093458.3 |
  1093715.3 |
  1093972.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_madd_null_entry**: bridge=3869.4% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_scalar_payload**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_soa_payload**: bridge=303.1% of algo (FFI overhead may distort results)
