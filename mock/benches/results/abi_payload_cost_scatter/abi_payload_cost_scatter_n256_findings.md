# abi_payload_cost (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_scatter_scalar_payload has the worst median (2.13 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_scatter_null_entry at 2.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_scatter_null_entry dominates: 35698% faster than the next best (abi_payload_cost_scatter_soa_payload)

abi_payload_cost_scatter_null_entry (2.46 us) leads abi_payload_cost_scatter_soa_payload (881.97 us) by 35698%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_scatter_null_entry beats baseline by 100% (significant)

abi_payload_cost_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_payload_cost_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_scatter_scalar_payload is an outlier: 865.3x slower than the field

abi_payload_cost_scatter_scalar_payload (2.13 ms) is 865.3x the fastest (2.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 865.3x the fastest

Fastest abi_payload_cost_scatter_null_entry (2.46 us) to slowest abi_payload_cost_scatter_scalar_payload (2.13 ms): 865.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_scatter_null_entry** at 2463.8 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 865.29x (fastest 2463.8 ns, slowest 2131850.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4686ns | 4681ns | 4592ns | 4664ns | 4766ns | -99.78% |
| abi_payload_cost_scatter_scalar_payload | 2139407ns | 2134394ns | 2126330ns | 2131722ns | 2157474ns | base |
| abi_payload_cost_scatter_soa_payload | 894056ns | 884415ns | 874039ns | 883314ns | 920177ns | -58.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 2460ns | 2420ns | 2493ns | -99.88% | 0.104 |
| abi_payload_cost_scatter_scalar_payload | 2136790ns | 2123823ns | 2154673ns | base | 0.000 |
| abi_payload_cost_scatter_soa_payload | 891516ns | 871716ns | 917365ns | -58.28% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 26282.6 | 2672.2 | 2459.5 | n/a |
| abi_payload_cost_scatter_scalar_payload | 39863.5 | 2142084.2 | 2136790.5 | n/a |
| abi_payload_cost_scatter_soa_payload | 36829.0 | 895583.5 | 891515.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.106 Gops/s** (abi_payload_cost_scatter_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_scatter_null_entry | 0.104 | 98.2% |
| abi_payload_cost_scatter_scalar_payload | 0.000 | 0.1% |
| abi_payload_cost_scatter_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 4686ns | 4686ns | -99.78% |
| abi_payload_cost_scatter_scalar_payload | 2139407ns | 2139407ns | base |
| abi_payload_cost_scatter_soa_payload | 894056ns | 894056ns | -58.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_scatter_scalar_payload | 2131850ns | base | --- | [2123849, 2154673] | --- | --- | --- | --- |
| abi_payload_cost_scatter_null_entry | 2464ns | -2129365.0ns (-99.9%) | [-2152223, -2121405]ns | [2422, 2493] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_scatter_soa_payload | 881968ns | -1246448.1ns (-58.5%) | [-1259463, -1229913]ns | [875214, 917365] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_scatter_scalar_payload | abi_payload_cost_scatter_null_entry | abi_payload_cost_scatter_soa_payload |
|---|---|---|---|
| 1 | 2139444ns | -99.9% | -58.7% |
| 2 | 2169902ns | -99.9% | -56.2% |
| 3 | 2123823ns | -99.9% | -58.5% |
| 4 | 2123874ns | -99.9% | -58.4% |
| 5 | 2135295ns | -99.9% | -59.2% |
| 6 | 2128406ns | -99.9% | -58.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_scatter_null_entry | -0.471 | moderate- |
| abi_payload_cost_scatter_scalar_payload | -0.094 | ok |
| abi_payload_cost_scatter_soa_payload | -0.135 | ok |

**Consistency summary:**

- **abi_payload_cost_scatter_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_scatter_null_entry | 111593.5ns | 2459.5ns | 4537.2% | HIGH |
| abi_payload_cost_scatter_scalar_payload | 6550620.5ns | 2136790.5ns | 306.6% | HIGH |
| abi_payload_cost_scatter_soa_payload | 2730583.5ns | 891515.8ns | 306.3% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_scatter_null_entry (n=6, range 2420.4-2492.9 ns)
   2420.4 |########################################
   2424.0 |
   2427.7 |
   2431.3 |
   2434.9 |
   2438.5 |
   2442.2 |
   2445.8 |
   2449.4 |
   2453.0 |
   2456.7 |
   2460.3 |####################
   2463.9 |####################
   2467.5 |
   2471.2 |
   2474.8 |####################
   2478.4 |
   2482.0 |
   2485.7 |
   2489.3 |
  (0 below, 1 above range)

abi_payload_cost_scatter_scalar_payload (n=6, range 2123823.3-2154672.8 ns)
  2123823.3 |########################################
  2125365.8 |
  2126908.2 |####################
  2128450.7 |
  2129993.2 |
  2131535.7 |
  2133078.1 |
  2134620.6 |####################
  2136163.1 |
  2137705.6 |
  2139248.0 |####################
  2140790.5 |
  2142333.0 |
  2143875.4 |
  2145417.9 |
  2146960.4 |
  2148502.9 |
  2150045.3 |
  2151587.8 |
  2153130.3 |
  (0 below, 1 above range)

abi_payload_cost_scatter_soa_payload (n=6, range 871716.2-917365.2 ns)
  871716.2 |####################
  873998.6 |
  876281.1 |
  878563.5 |########################################
  880846.0 |
  883128.4 |########################################
  885410.9 |
  887693.3 |
  889975.8 |
  892258.2 |
  894540.7 |
  896823.1 |
  899105.6 |
  901388.0 |
  903670.5 |
  905952.9 |
  908235.4 |
  910517.8 |
  912800.3 |
  915082.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_scatter_null_entry**: bridge=4533.9% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_scalar_payload**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_payload_cost_scatter_soa_payload**: bridge=303.4% of algo (FFI overhead may distort results)
