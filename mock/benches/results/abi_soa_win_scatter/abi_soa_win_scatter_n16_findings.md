# abi_soa_win (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_scatter_scalar_payload has the worst median (2.21 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_scatter_null_entry at 2.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_scatter_null_entry dominates: 35612% faster than the next best (abi_soa_win_scatter_soa_payload)

abi_soa_win_scatter_null_entry (2.55 us) leads abi_soa_win_scatter_soa_payload (911.69 us) by 35612%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_scatter_null_entry beats baseline by 100% (significant)

abi_soa_win_scatter_null_entry is -2.20 ms (100%) faster than baseline abi_soa_win_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_scatter_scalar_payload is an outlier: 864.2x slower than the field

abi_soa_win_scatter_scalar_payload (2.21 ms) is 864.2x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 864.2x the fastest

Fastest abi_soa_win_scatter_null_entry (2.55 us) to slowest abi_soa_win_scatter_scalar_payload (2.21 ms): 864.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_scatter_null_entry** at 2552.9 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 864.25x (fastest 2552.9 ns, slowest 2206338.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 4876ns | 4870ns | 4640ns | 4838ns | 5052ns | -99.79% |
| abi_soa_win_scatter_scalar_payload | 2297707ns | 2209826ns | 2152782ns | 2202026ns | 2513691ns | base |
| abi_soa_win_scatter_soa_payload | 926223ns | 914624ns | 905181ns | 912011ns | 958061ns | -59.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 2567ns | 2460ns | 2649ns | -99.89% | 0.006 |
| abi_soa_win_scatter_scalar_payload | 2293904ns | 2149705ns | 2509086ns | base | 0.000 |
| abi_soa_win_scatter_soa_payload | 923071ns | 902492ns | 954349ns | -59.76% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 28615.3 | 2700.8 | 2566.6 | n/a |
| abi_soa_win_scatter_scalar_payload | 74214.1 | 2306413.8 | 2293903.6 | n/a |
| abi_soa_win_scatter_soa_payload | 51574.4 | 918436.6 | 923070.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_soa_win_scatter_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.006 | 96.3% |
| abi_soa_win_scatter_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_scatter_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_scatter_null_entry | 4876ns | 4876ns | -99.79% |
| abi_soa_win_scatter_scalar_payload | 2297707ns | 2297707ns | base |
| abi_soa_win_scatter_soa_payload | 926223ns | 926223ns | -59.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_scatter_scalar_payload | 2206338ns | base | --- | [2166287, 2509086] | --- | --- | --- | --- |
| abi_soa_win_scatter_null_entry | 2553ns | -2203824.0ns (-99.9%) | [-2506463, -2163724]ns | [2498, 2649] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_scatter_soa_payload | 911687ns | -1292301.7ns (-58.6%) | [-1562769, -1257428]ns | [903176, 954349] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_scatter_scalar_payload | abi_soa_win_scatter_null_entry | abi_soa_win_scatter_soa_payload |
|---|---|---|---|
| 1 | 2149705ns | -99.9% | -58.0% |
| 2 | 2201802ns | -99.9% | -58.3% |
| 3 | 2210874ns | -99.9% | -58.9% |
| 4 | 2239556ns | -99.9% | -59.7% |
| 5 | 2182869ns | -99.9% | -58.1% |
| 6 | 2778616ns | -99.9% | -64.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_scatter_null_entry | -0.015 | ok |
| abi_soa_win_scatter_scalar_payload | -0.078 | ok |
| abi_soa_win_scatter_soa_payload | -0.000 | ok |

**Consistency summary:**

- **abi_soa_win_scatter_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 118710.0ns | 2566.6ns | 4625.2% | HIGH |
| abi_soa_win_scatter_scalar_payload | 6983411.8ns | 2293903.6ns | 304.4% | HIGH |
| abi_soa_win_scatter_soa_payload | 2802523.0ns | 923070.7ns | 303.6% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_scatter_null_entry (n=6, range 2459.6-2649.3 ns)
   2459.6 |########################################
   2469.1 |
   2478.6 |
   2488.1 |
   2497.5 |
   2507.0 |
   2516.5 |
   2526.0 |########################################
   2535.5 |########################################
   2545.0 |
   2554.5 |
   2564.0 |########################################
   2573.4 |
   2582.9 |########################################
   2592.4 |
   2601.9 |
   2611.4 |
   2620.9 |
   2630.4 |
   2639.9 |
  (0 below, 1 above range)

abi_soa_win_scatter_scalar_payload (n=6, range 2149704.6-2509085.8 ns)
  2149704.6 |########################################
  2167673.7 |########################################
  2185642.7 |########################################
  2203611.8 |########################################
  2221580.8 |
  2239549.9 |########################################
  2257519.0 |
  2275488.0 |
  2293457.1 |
  2311426.1 |
  2329395.2 |
  2347364.3 |
  2365333.3 |
  2383302.4 |
  2401271.4 |
  2419240.5 |
  2437209.6 |
  2455178.6 |
  2473147.7 |
  2491116.7 |
  (0 below, 1 above range)

abi_soa_win_scatter_soa_payload (n=6, range 902492.1-954348.9 ns)
  902492.1 |########################################
  905084.9 |
  907677.8 |####################
  910270.6 |
  912863.5 |####################
  915456.3 |
  918049.2 |####################
  920642.0 |
  923234.8 |
  925827.7 |
  928420.5 |
  931013.4 |
  933606.2 |
  936199.1 |
  938791.9 |
  941384.7 |
  943977.6 |
  946570.4 |
  949163.3 |
  951756.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_scatter_null_entry**: bridge=4621.4% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_scalar_payload**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_soa_payload**: bridge=304.2% of algo (FFI overhead may distort results)
