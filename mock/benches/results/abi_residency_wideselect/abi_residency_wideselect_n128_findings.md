# abi_residency (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_residency_wideselect_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_wideselect_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_wideselect_null_entry dominates: 75999% faster than the next best (abi_residency_wideselect_reused_buffer)

abi_residency_wideselect_null_entry (2.69 us) leads abi_residency_wideselect_reused_buffer (2.05 ms) by 75999%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_wideselect_null_entry beats baseline by 100% (significant)

abi_residency_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_residency_wideselect_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_wideselect_fresh_alloc is an outlier: 763.1x slower than the field

abi_residency_wideselect_fresh_alloc (2.05 ms) is 763.1x the fastest (2.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 763.1x the fastest

Fastest abi_residency_wideselect_null_entry (2.69 us) to slowest abi_residency_wideselect_fresh_alloc (2.05 ms): 763.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_wideselect_null_entry** at 2692.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 763.08x (fastest 2692.7 ns, slowest 2054743.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2065619ns | 2057375ns | 2053277ns | 2056942ns | 2084807ns | -0.13% |
| abi_residency_wideselect_null_entry | 4953ns | 4947ns | 4792ns | 4919ns | 5084ns | -99.76% |
| abi_residency_wideselect_reused_buffer | 2068376ns | 2051714ns | 2045065ns | 2050985ns | 2106120ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2063004ns | 2050630ns | 2082258ns | -0.13% | 0.000 |
| abi_residency_wideselect_null_entry | 2693ns | 2618ns | 2757ns | -99.87% | 0.048 |
| abi_residency_wideselect_reused_buffer | 2065662ns | 2042506ns | 2103097ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 42654.5 | 2065145.9 | 2063003.5 | n/a |
| abi_residency_wideselect_null_entry | 28450.3 | 2737.6 | 2693.2 | n/a |
| abi_residency_wideselect_reused_buffer | 46211.2 | 2114766.4 | 2065661.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_residency_wideselect_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.000 | 0.1% |
| abi_residency_wideselect_null_entry | 0.048 | 97.2% |
| abi_residency_wideselect_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2065619ns | 2065619ns | -0.13% |
| abi_residency_wideselect_null_entry | 4953ns | 4953ns | -99.76% |
| abi_residency_wideselect_reused_buffer | 2068376ns | 2068376ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_wideselect_reused_buffer | 2049106ns | base | --- | [2044782, 2103097] | --- | --- | --- | --- |
| abi_residency_wideselect_fresh_alloc | 2054744ns | no significant difference | [-24554, +10342]ns | [2052009, 2082258] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_wideselect_null_entry | 2693ns | -2046457.1ns (-99.9%) | [-2100367, -2042082]ns | [2630, 2757] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_wideselect_reused_buffer | abi_residency_wideselect_fresh_alloc | abi_residency_wideselect_null_entry |
|---|---|---|---|
| 1 | 2156238ns | -2.3% | -99.9% |
| 2 | 2047059ns | +0.3% | -99.9% |
| 3 | 2049572ns | +0.1% | -99.9% |
| 4 | 2049956ns | +0.3% | -99.9% |
| 5 | 2042506ns | +0.5% | -99.9% |
| 6 | 2048640ns | +0.5% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | -0.040 | ok |
| abi_residency_wideselect_null_entry | -0.124 | ok |
| abi_residency_wideselect_reused_buffer | -0.038 | ok |

**Consistency summary:**

- **abi_residency_wideselect_fresh_alloc**: won 1/6, lost 4/6
- **abi_residency_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 6297988.2ns | 2063003.5ns | 305.3% | HIGH |
| abi_residency_wideselect_null_entry | 120337.3ns | 2693.2ns | 4468.2% | HIGH |
| abi_residency_wideselect_reused_buffer | 6321807.3ns | 2065661.9ns | 306.0% | HIGH |

## Distribution (algo ns)

```
abi_residency_wideselect_fresh_alloc (n=6, range 2050630.0-2082257.9 ns)
  2050630.0 |####################
  2052211.4 |########################################
  2053792.8 |
  2055374.2 |####################
  2056955.6 |####################
  2058537.0 |
  2060118.4 |
  2061699.8 |
  2063281.2 |
  2064862.6 |
  2066443.9 |
  2068025.3 |
  2069606.7 |
  2071188.1 |
  2072769.5 |
  2074350.9 |
  2075932.3 |
  2077513.7 |
  2079095.1 |
  2080676.5 |
  (0 below, 1 above range)

abi_residency_wideselect_null_entry (n=6, range 2617.9-2756.7 ns)
   2617.9 |####################
   2624.8 |
   2631.8 |
   2638.7 |####################
   2645.7 |
   2652.6 |####################
   2659.5 |
   2666.5 |
   2673.4 |
   2680.3 |
   2687.3 |
   2694.2 |
   2701.2 |
   2708.1 |
   2715.0 |
   2722.0 |
   2728.9 |########################################
   2735.8 |
   2742.8 |
   2749.7 |
  (0 below, 1 above range)

abi_residency_wideselect_reused_buffer (n=6, range 2042506.2-2103097.2 ns)
  2042506.2 |#############
  2045535.8 |#############
  2048565.3 |########################################
  2051594.9 |
  2054624.4 |
  2057654.0 |
  2060683.5 |
  2063713.1 |
  2066742.6 |
  2069772.2 |
  2072801.7 |
  2075831.3 |
  2078860.8 |
  2081890.4 |
  2084919.9 |
  2087949.5 |
  2090979.0 |
  2094008.6 |
  2097038.1 |
  2100067.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_wideselect_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_null_entry**: bridge=4455.2% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_reused_buffer**: bridge=302.3% of algo (FFI overhead may distort results)
