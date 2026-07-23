# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.17 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 2.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 35677% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (2.50 us) leads abi_soa_win_real_soa_payload (893.16 us) by 35677%, a clear separation rather than a photo finish. CV 7.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.17 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 870.2x slower than the field

abi_soa_win_real_scalar_payload (2.17 ms) is 870.2x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_null_entry is fastest but the noisiest (CV 7.7%)

abi_soa_win_real_null_entry wins on median (2.50 us) yet has the highest variance (CV 7.7%), while abi_soa_win_real_scalar_payload is the steadiest (CV 0.2%, 2.17 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 870.2x the fastest

Fastest abi_soa_win_real_null_entry (2.50 us) to slowest abi_soa_win_real_scalar_payload (2.17 ms): 870.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 2496.4 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 870.18x (fastest 2496.4 ns, slowest 2172364.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 4876ns | 4715ns | 4685ns | 4707ns | 5226ns | -99.78% |
| abi_soa_win_real_scalar_payload | 2177548ns | 2175598ns | 2172192ns | 2175056ns | 2183965ns | base |
| abi_soa_win_real_soa_payload | 900944ns | 896143ns | 891147ns | 894690ns | 915223ns | -58.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 2592ns | 2482ns | 2796ns | -99.88% | 0.025 |
| abi_soa_win_real_scalar_payload | 2174228ns | 2168779ns | 2180601ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 897973ns | 888361ns | 912135ns | -58.70% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 29019.1 | 2760.3 | 2592.2 | n/a |
| abi_soa_win_real_scalar_payload | 68198.9 | 2173390.9 | 2174227.5 | n/a |
| abi_soa_win_real_soa_payload | 49410.8 | 897418.2 | 897973.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.026 | 99.4% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 4876ns | 4876ns | -99.78% |
| abi_soa_win_real_scalar_payload | 2177548ns | 2177548ns | base |
| abi_soa_win_real_soa_payload | 900944ns | 900944ns | -58.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2172364ns | base | --- | [2169718, 2180601] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 2496ns | -2169604.1ns (-99.9%) | [-2178112, -2167189]ns | [2484, 2796] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 893164ns | -1280119.0ns (-58.9%) | [-1288945, -1259698]ns | [888621, 912135] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2168779ns | -99.9% | -59.0% |
| 2 | 2177127ns | -99.9% | -58.8% |
| 3 | 2173010ns | -99.9% | -57.4% |
| 4 | 2170656ns | -99.9% | -58.6% |
| 5 | 2184075ns | -99.9% | -59.3% |
| 6 | 2171718ns | -99.9% | -59.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.133 | ok |
| abi_soa_win_real_scalar_payload | -0.481 | moderate- |
| abi_soa_win_real_soa_payload | 0.076 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 116576.0ns | 2592.2ns | 4497.2% | HIGH |
| abi_soa_win_real_scalar_payload | 6590877.6ns | 2174227.5ns | 303.1% | HIGH |
| abi_soa_win_real_soa_payload | 2744899.7ns | 897973.3ns | 305.7% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 2481.7-2795.8 ns)
   2481.7 |########################################
   2497.4 |#############
   2513.1 |
   2528.8 |
   2544.5 |
   2560.2 |#############
   2575.9 |
   2591.7 |
   2607.4 |
   2623.1 |
   2638.8 |
   2654.5 |
   2670.2 |
   2685.9 |
   2701.6 |
   2717.3 |
   2733.0 |
   2748.7 |
   2764.4 |
   2780.1 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2168779.2-2180600.9 ns)
  2168779.2 |########################################
  2169370.3 |
  2169961.4 |
  2170552.4 |########################################
  2171143.5 |########################################
  2171734.6 |
  2172325.7 |
  2172916.8 |########################################
  2173507.9 |
  2174098.9 |
  2174690.0 |
  2175281.1 |
  2175872.2 |
  2176463.3 |
  2177054.4 |########################################
  2177645.4 |
  2178236.5 |
  2178827.6 |
  2179418.7 |
  2180009.8 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 888360.8-912134.6 ns)
  888360.8 |########################################
  889549.5 |
  890738.2 |
  891926.9 |
  893115.6 |
  894304.2 |
  895492.9 |
  896681.6 |#############
  897870.3 |#############
  899059.0 |
  900247.7 |
  901436.4 |
  902625.1 |
  903813.8 |
  905002.5 |
  906191.2 |
  907379.8 |
  908568.5 |
  909757.2 |
  910945.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=4590.3% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=306.1% of algo (FFI overhead may distort results)
