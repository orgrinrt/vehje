# abi_payload_cost (real)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_real_scalar_payload has the worst median (2.13 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_real_null_entry at 2.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_real_null_entry dominates: 34432% faster than the next best (abi_payload_cost_real_soa_payload)

abi_payload_cost_real_null_entry (2.54 us) leads abi_payload_cost_real_soa_payload (878.06 us) by 34432%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_real_null_entry beats baseline by 100% (significant)

abi_payload_cost_real_null_entry is -2.13 ms (100%) faster than baseline abi_payload_cost_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_real_scalar_payload is an outlier: 838.0x slower than the field

abi_payload_cost_real_scalar_payload (2.13 ms) is 838.0x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 838.0x the fastest

Fastest abi_payload_cost_real_null_entry (2.54 us) to slowest abi_payload_cost_real_scalar_payload (2.13 ms): 838.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_payload_cost_real_null_entry** at 2542.7 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 838.04x (fastest 2542.7 ns, slowest 2130891.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 4812ns | 4839ns | 4638ns | 4772ns | 4960ns | -99.78% |
| abi_payload_cost_real_scalar_payload | 2142166ns | 2133422ns | 2120993ns | 2130340ns | 2170490ns | base |
| abi_payload_cost_real_soa_payload | 886793ns | 880405ns | 877431ns | 879494ns | 902423ns | -58.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 2528ns | 2405ns | 2614ns | -99.88% | 0.101 |
| abi_payload_cost_real_scalar_payload | 2139579ns | 2118474ns | 2167715ns | base | 0.000 |
| abi_payload_cost_real_soa_payload | 884390ns | 874974ns | 899954ns | -58.67% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 27555.5 | 2758.3 | 2527.6 | n/a |
| abi_payload_cost_real_scalar_payload | 42101.4 | 2141168.5 | 2139579.3 | n/a |
| abi_payload_cost_real_soa_payload | 33637.2 | 884074.1 | 884390.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.106 Gops/s** (abi_payload_cost_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_real_null_entry | 0.101 | 94.6% |
| abi_payload_cost_real_scalar_payload | 0.000 | 0.1% |
| abi_payload_cost_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_real_null_entry | 4812ns | 4812ns | -99.78% |
| abi_payload_cost_real_scalar_payload | 2142166ns | 2142166ns | base |
| abi_payload_cost_real_soa_payload | 886793ns | 886793ns | -58.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_real_scalar_payload | 2130892ns | base | --- | [2120130, 2167715] | --- | --- | --- | --- |
| abi_payload_cost_real_null_entry | 2543ns | -2128299.2ns (-99.9%) | [-2165220, -2117636]ns | [2426, 2614] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_real_soa_payload | 878057ns | -1250147.7ns (-58.7%) | [-1272004, -1243414]ns | [875161, 899954] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_real_scalar_payload | abi_payload_cost_real_null_entry | abi_payload_cost_real_soa_payload |
|---|---|---|---|
| 1 | 2198152ns | -99.9% | -58.5% |
| 2 | 2118474ns | -99.9% | -58.7% |
| 3 | 2121787ns | -99.9% | -58.8% |
| 4 | 2132192ns | -99.9% | -58.3% |
| 5 | 2137279ns | -99.9% | -58.8% |
| 6 | 2129592ns | -99.9% | -58.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_real_null_entry | -0.481 | moderate- |
| abi_payload_cost_real_scalar_payload | -0.158 | ok |
| abi_payload_cost_real_soa_payload | -0.179 | ok |

**Consistency summary:**

- **abi_payload_cost_real_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_real_null_entry | 113786.9ns | 2527.6ns | 4501.8% | HIGH |
| abi_payload_cost_real_scalar_payload | 6575352.8ns | 2139579.3ns | 307.3% | HIGH |
| abi_payload_cost_real_soa_payload | 2689358.8ns | 884390.5ns | 304.1% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_real_null_entry (n=6, range 2404.6-2614.2 ns)
   2404.6 |####################
   2415.1 |
   2425.6 |
   2436.0 |
   2446.5 |####################
   2457.0 |
   2467.5 |
   2477.9 |
   2488.4 |
   2498.9 |
   2509.4 |
   2519.9 |
   2530.3 |
   2540.8 |########################################
   2551.3 |
   2561.8 |
   2572.2 |
   2582.7 |####################
   2593.2 |
   2603.7 |
  (0 below, 1 above range)

abi_payload_cost_real_scalar_payload (n=6, range 2118473.8-2167715.5 ns)
  2118473.8 |########################################
  2120935.9 |########################################
  2123398.0 |
  2125860.0 |
  2128322.1 |########################################
  2130784.2 |########################################
  2133246.3 |
  2135708.4 |########################################
  2138170.5 |
  2140632.5 |
  2143094.6 |
  2145556.7 |
  2148018.8 |
  2150480.9 |
  2152943.0 |
  2155405.0 |
  2157867.1 |
  2160329.2 |
  2162791.3 |
  2165253.4 |
  (0 below, 1 above range)

abi_payload_cost_real_soa_payload (n=6, range 874974.2-899953.8 ns)
  874974.2 |########################################
  876223.2 |
  877472.2 |
  878721.1 |
  879970.1 |#############
  881219.1 |
  882468.1 |
  883717.0 |
  884966.0 |
  886215.0 |
  887464.0 |#############
  888713.0 |
  889961.9 |
  891210.9 |
  892459.9 |
  893708.9 |
  894957.8 |
  896206.8 |
  897455.8 |
  898704.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_real_null_entry**: bridge=4465.2% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_scalar_payload**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_payload_cost_real_soa_payload**: bridge=303.7% of algo (FFI overhead may distort results)
