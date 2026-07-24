# abi_payload_cost (tight)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_tight_scalar_payload has the worst median (503.89 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_tight_null_entry at 2.63 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_tight_null_entry dominates: 8355% faster than the next best (abi_payload_cost_tight_soa_payload)

abi_payload_cost_tight_null_entry (2.63 us) leads abi_payload_cost_tight_soa_payload (222.59 us) by 8355%, a clear separation rather than a photo finish. CV 12.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_tight_null_entry beats baseline by 99% (significant)

abi_payload_cost_tight_null_entry is -501.20 us (99%) faster than baseline abi_payload_cost_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_tight_scalar_payload is an outlier: 191.4x slower than the field

abi_payload_cost_tight_scalar_payload (503.89 us) is 191.4x the fastest (2.63 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_tight_null_entry is fastest but the noisiest (CV 12.1%)

abi_payload_cost_tight_null_entry wins on median (2.63 us) yet has the highest variance (CV 12.1%), while abi_payload_cost_tight_scalar_payload is the steadiest (CV 0.4%, 503.89 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 191.4x the fastest

Fastest abi_payload_cost_tight_null_entry (2.63 us) to slowest abi_payload_cost_tight_scalar_payload (503.89 us): 191.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_tight_null_entry** at 2632.5 ns median (-99.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 191.41x (fastest 2632.5 ns, slowest 503888.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 5103ns | 4983ns | 4670ns | 4888ns | 5643ns | -98.99% |
| abi_payload_cost_tight_scalar_payload | 506358ns | 506202ns | 503890ns | 505674ns | 508619ns | base |
| abi_payload_cost_tight_soa_payload | 224941ns | 224853ns | 223707ns | 224495ns | 226225ns | -55.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 2714ns | 2458ns | 3040ns | -99.46% | 0.024 |
| abi_payload_cost_tight_scalar_payload | 504076ns | 501698ns | 506318ns | base | 0.000 |
| abi_payload_cost_tight_soa_payload | 222706ns | 221521ns | 224008ns | -55.82% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 24636.4 | 2994.0 | 2714.2 | 0 |
| abi_payload_cost_tight_scalar_payload | 24477.7 | 504402.0 | 504076.1 | n/a |
| abi_payload_cost_tight_soa_payload | 22628.5 | 222367.8 | 222705.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_payload_cost_tight_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_tight_null_entry | 0.024 | 93.4% |
| abi_payload_cost_tight_scalar_payload | 0.000 | 0.5% |
| abi_payload_cost_tight_soa_payload | 0.000 | 1.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_tight_null_entry | 5103ns | 5103ns | -98.99% |
| abi_payload_cost_tight_scalar_payload | 506358ns | 506358ns | base |
| abi_payload_cost_tight_soa_payload | 224941ns | 224941ns | -55.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_tight_scalar_payload | 503889ns | base | --- | [502021, 506318] | --- | --- | --- | --- |
| abi_payload_cost_tight_null_entry | 2632ns | -501198.8ns (-99.5%) | [-503420, -499467]ns | [2470, 3040] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_tight_soa_payload | 222586ns | -281121.7ns (-55.8%) | [-283697, -279292]ns | [221523, 224008] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_tight_scalar_payload | abi_payload_cost_tight_null_entry | abi_payload_cost_tight_soa_payload |
|---|---|---|---|
| 1 | 501698ns | -99.5% | -55.7% |
| 2 | 505550ns | -99.3% | -56.2% |
| 3 | 507087ns | -99.5% | -55.7% |
| 4 | 502887ns | -99.5% | -55.6% |
| 5 | 502344ns | -99.5% | -55.5% |
| 6 | 504891ns | -99.5% | -56.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_tight_null_entry | -0.250 | moderate- |
| abi_payload_cost_tight_scalar_payload | -0.091 | ok |
| abi_payload_cost_tight_soa_payload | -0.147 | ok |

**Consistency summary:**

- **abi_payload_cost_tight_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 111965.5ns | 2714.2ns | 4125.1% | HIGH |
| abi_payload_cost_tight_scalar_payload | 1541374.7ns | 504076.1ns | 305.8% | HIGH |
| abi_payload_cost_tight_soa_payload | 690793.4ns | 222705.7ns | 310.2% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_tight_null_entry (n=6, range 2458.3-3039.8 ns)
   2458.3 |########################################
   2487.4 |
   2516.5 |
   2545.5 |
   2574.6 |
   2603.7 |####################
   2632.8 |####################
   2661.8 |####################
   2690.9 |
   2720.0 |
   2749.1 |
   2778.1 |
   2807.2 |
   2836.3 |
   2865.4 |
   2894.4 |
   2923.5 |
   2952.6 |
   2981.7 |
   3010.7 |
  (0 below, 1 above range)

abi_payload_cost_tight_scalar_payload (n=6, range 501697.9-506318.2 ns)
  501697.9 |########################################
  501928.9 |
  502159.9 |########################################
  502390.9 |
  502622.0 |
  502853.0 |########################################
  503084.0 |
  503315.0 |
  503546.0 |
  503777.0 |
  504008.0 |
  504239.0 |
  504470.1 |
  504701.1 |########################################
  504932.1 |
  505163.1 |
  505394.1 |########################################
  505625.1 |
  505856.1 |
  506087.1 |
  (0 below, 1 above range)

abi_payload_cost_tight_soa_payload (n=6, range 221520.8-224008.1 ns)
  221520.8 |########################################
  221645.2 |
  221769.5 |
  221893.9 |
  222018.3 |####################
  222142.6 |
  222267.0 |
  222391.4 |
  222515.7 |
  222640.1 |
  222764.4 |
  222888.8 |
  223013.2 |
  223137.5 |####################
  223261.9 |
  223386.3 |####################
  223510.6 |
  223635.0 |
  223759.4 |
  223883.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_tight_null_entry**: bridge=4175.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_scalar_payload**: bridge=305.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_soa_payload**: bridge=310.0% of algo (FFI overhead may distort results)
