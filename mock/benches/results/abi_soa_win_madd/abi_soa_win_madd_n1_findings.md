# abi_soa_win (madd)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_madd_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_madd_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_madd_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_madd_scalar_payload has the worst median (2.72 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_madd_null_entry at 4.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_madd_null_entry dominates: 55466% faster than the next best (abi_soa_win_madd_soa_payload)

abi_soa_win_madd_null_entry (4.87 us) leads abi_soa_win_madd_soa_payload (2.71 ms) by 55466%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_madd_null_entry beats baseline by 100% (significant)

abi_soa_win_madd_null_entry is -2.71 ms (100%) faster than baseline abi_soa_win_madd_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_madd_scalar_payload is an outlier: 557.9x slower than the field

abi_soa_win_madd_scalar_payload (2.72 ms) is 557.9x the fastest (4.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 557.9x the fastest

Fastest abi_soa_win_madd_null_entry (4.87 us) to slowest abi_soa_win_madd_scalar_payload (2.72 ms): 557.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_madd_null_entry** at 4873.9 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 557.93x (fastest 4873.9 ns, slowest 2719325.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 7208ns | 7138ns | 7058ns | 7127ns | 7403ns | -99.74% |
| abi_soa_win_madd_scalar_payload | 2720688ns | 2722728ns | 2709500ns | 2719852ns | 2727535ns | base |
| abi_soa_win_madd_soa_payload | 2712741ns | 2711402ns | 2708639ns | 2710979ns | 2717435ns | -0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 4920ns | 4848ns | 5037ns | -99.82% | 0.000 |
| abi_soa_win_madd_scalar_payload | 2717444ns | 2706624ns | 2724224ns | base | 0.000 |
| abi_soa_win_madd_soa_payload | 2709594ns | 2705767ns | 2714031ns | -0.29% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 26880.3 | 4933.5 | 4920.1 | n/a |
| abi_soa_win_madd_scalar_payload | 64455.1 | 2719049.5 | 2717443.9 | n/a |
| abi_soa_win_madd_soa_payload | 61018.5 | 2708415.1 | 2709594.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_soa_win_madd_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_madd_null_entry | 0.000 | 99.5% |
| abi_soa_win_madd_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_madd_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_madd_null_entry | 7208ns | 7208ns | -99.74% |
| abi_soa_win_madd_scalar_payload | 2720688ns | 2720688ns | base |
| abi_soa_win_madd_soa_payload | 2712741ns | 2712741ns | -0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_madd_scalar_payload | 2719326ns | base | --- | [2708782, 2724224] | --- | --- | --- | --- |
| abi_soa_win_madd_null_entry | 4874ns | -2714447.1ns (-99.8%) | [-2719207, -2703918]ns | [4849, 5037] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_madd_soa_payload | 2708274ns | -8038.1ns (-0.3%) | [-14917, -594]ns | [2706477, 2714031] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_madd_scalar_payload | abi_soa_win_madd_null_entry | abi_soa_win_madd_soa_payload |
|---|---|---|---|
| 1 | 2719725ns | -99.8% | -0.3% |
| 2 | 2718927ns | -99.8% | -0.4% |
| 3 | 2706624ns | -99.8% | +0.0% |
| 4 | 2710941ns | -99.8% | -0.1% |
| 5 | 2724034ns | -99.8% | -0.7% |
| 6 | 2724413ns | -99.8% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_madd_null_entry | -0.082 | ok |
| abi_soa_win_madd_scalar_payload | 0.235 | moderate+ |
| abi_soa_win_madd_soa_payload | -0.308 | moderate- |

**Consistency summary:**

- **abi_soa_win_madd_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_madd_soa_payload**: won 4/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_madd_null_entry | 124971.1ns | 4920.1ns | 2540.0% | HIGH |
| abi_soa_win_madd_scalar_payload | 8213329.3ns | 2717443.9ns | 302.2% | HIGH |
| abi_soa_win_madd_soa_payload | 8194840.8ns | 2709594.1ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_madd_null_entry (n=6, range 4848.3-5037.0 ns)
   4848.3 |########################################
   4857.7 |####################
   4867.2 |
   4876.6 |####################
   4886.1 |####################
   4895.5 |
   4904.9 |
   4914.4 |
   4923.8 |
   4933.2 |
   4942.7 |
   4952.1 |
   4961.5 |
   4971.0 |
   4980.4 |
   4989.9 |
   4999.3 |
   5008.7 |
   5018.2 |
   5027.6 |
  (0 below, 1 above range)

abi_soa_win_madd_scalar_payload (n=6, range 2706623.8-2724223.5 ns)
  2706623.8 |########################################
  2707503.8 |
  2708383.8 |
  2709263.8 |
  2710143.7 |########################################
  2711023.7 |
  2711903.7 |
  2712783.7 |
  2713663.7 |
  2714543.7 |
  2715423.6 |
  2716303.6 |
  2717183.6 |
  2718063.6 |########################################
  2718943.6 |########################################
  2719823.6 |
  2720703.6 |
  2721583.5 |
  2722463.5 |
  2723343.5 |########################################
  (0 below, 1 above range)

abi_soa_win_madd_soa_payload (n=6, range 2705767.1-2714030.9 ns)
  2705767.1 |####################
  2706180.3 |
  2706593.5 |
  2707006.7 |########################################
  2707419.9 |
  2707833.0 |
  2708246.2 |
  2708659.4 |
  2709072.6 |####################
  2709485.8 |
  2709899.0 |
  2710312.2 |
  2710725.4 |
  2711138.5 |
  2711551.7 |
  2711964.9 |
  2712378.1 |####################
  2712791.3 |
  2713204.5 |
  2713617.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_madd_null_entry**: bridge=2560.2% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_scalar_payload**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_soa_win_madd_soa_payload**: bridge=302.3% of algo (FFI overhead may distort results)
