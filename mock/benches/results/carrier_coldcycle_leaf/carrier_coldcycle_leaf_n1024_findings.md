# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), leaf profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_leaf_null dominates: 78% faster than the next best (carrier_cold_leaf_threaded)

carrier_cold_leaf_null (28.71 us) leads carrier_cold_leaf_threaded (51.06 us) by 78%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_leaf_null beats baseline by 46% (significant)

carrier_cold_leaf_null is -23.63 us (46%) faster than baseline carrier_cold_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_leaf_fntable is an outlier: 2.3x slower than the field

carrier_cold_leaf_fntable (65.25 us) is 2.3x the fastest (28.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cold_leaf_null} vs {carrier_cold_leaf_threaded, carrier_cold_leaf_switch, carrier_cold_leaf_fntable} (78% apart)

The field splits into a fast tier {carrier_cold_leaf_null} and a slow tier {carrier_cold_leaf_threaded, carrier_cold_leaf_switch, carrier_cold_leaf_fntable} with a 78% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cold_leaf_null** at 28707.5 ns median (-44.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.27x (fastest 28707.5 ns, slowest 65248.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_leaf_fntable | 67238ns | 67516ns | 64522ns | 66871ns | 69147ns | +24.36% |
| carrier_cold_leaf_null | 30712ns | 31156ns | 28653ns | 30463ns | 32115ns | -43.20% |
| carrier_cold_leaf_switch | 54069ns | 53766ns | 52429ns | 53386ns | 55913ns | base |
| carrier_cold_leaf_threaded | 53536ns | 53324ns | 50985ns | 52896ns | 55772ns | -0.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_leaf_fntable | 64912ns | 62335ns | 66626ns | +25.62% | 0.016 |
| carrier_cold_leaf_null | 28319ns | 26325ns | 29659ns | -45.20% | 0.036 |
| carrier_cold_leaf_switch | 51673ns | 50140ns | 53406ns | base | 0.020 |
| carrier_cold_leaf_threaded | 51209ns | 48622ns | 53365ns | -0.90% | 0.020 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_leaf_fntable | 599214 | 1211966 | 0.494 | 1.22× |
| carrier_cold_leaf_null | 403055 | 1272066 | 0.317 | 0.82× |
| carrier_cold_leaf_switch | 489822 | 942973 | 0.519 | 1.00× |
| carrier_cold_leaf_threaded | 480733 | 992494 | 0.484 | 0.98× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_cold_leaf_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_leaf_fntable | 0.016 | 40.3% |
| carrier_cold_leaf_null | 0.036 | 91.7% |
| carrier_cold_leaf_switch | 0.020 | 51.2% |
| carrier_cold_leaf_threaded | 0.020 | 51.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_leaf_fntable | 67238ns | 67238ns | +24.36% |
| carrier_cold_leaf_null | 30712ns | 30712ns | -43.20% |
| carrier_cold_leaf_switch | 54069ns | 54069ns | base |
| carrier_cold_leaf_threaded | 53536ns | 53536ns | -0.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_leaf_switch | 51380ns | base | --- | [50232, 53406] | --- | --- | --- | --- |
| carrier_cold_leaf_fntable | 65249ns | +13303.3ns (+25.9%) | [+12401, +14014]ns | [62862, 66626] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_leaf_null | 28708ns | -23628.1ns (-46.0%) | [-24817, -21617]ns | [26591, 29659] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_leaf_threaded | 51060ns | no significant difference | [-1404, +586]ns | [49202, 53365] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_leaf_switch | carrier_cold_leaf_fntable | carrier_cold_leaf_null | carrier_cold_leaf_threaded |
|---|---|---|---|---|
| 1 | 50140ns | +24.3% | -44.2% | +1.7% |
| 2 | 52251ns | +27.2% | -48.6% | -2.1% |
| 3 | 50325ns | +27.3% | -47.7% | -3.4% |
| 4 | 52973ns | +26.0% | -43.9% | -0.8% |
| 5 | 53839ns | +23.4% | -45.0% | +0.6% |
| 6 | 50509ns | +25.5% | -41.7% | -1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_leaf_fntable | -0.361 | moderate- |
| carrier_cold_leaf_null | 0.350 | moderate+ |
| carrier_cold_leaf_switch | -0.255 | moderate- |
| carrier_cold_leaf_threaded | -0.183 | ok |

**Consistency summary:**

- **carrier_cold_leaf_fntable**: won 0/6, lost 6/6
- **carrier_cold_leaf_null**: won 6/6, lost 0/6
- **carrier_cold_leaf_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_leaf_fntable | 130114.0ns | 64912.4ns | 200.4% | HIGH |
| carrier_cold_leaf_null | 101559.2ns | 28319.0ns | 358.6% | HIGH |
| carrier_cold_leaf_switch | 105892.3ns | 51672.9ns | 204.9% | HIGH |
| carrier_cold_leaf_threaded | 102694.2ns | 51209.2ns | 200.5% | HIGH |

## Distribution (algo ns)

```
carrier_cold_leaf_fntable (n=6, range 62335.0-66626.2 ns)
  62335.0 |####################
  62549.6 |
  62764.1 |
  62978.7 |
  63193.2 |####################
  63407.8 |
  63622.4 |
  63836.9 |
  64051.5 |####################
  64266.1 |
  64480.6 |
  64695.2 |
  64909.8 |
  65124.3 |
  65338.9 |
  65553.4 |
  65768.0 |
  65982.6 |
  66197.1 |
  66411.7 |########################################
  (0 below, 1 above range)

carrier_cold_leaf_null (n=6, range 26325.0-29658.9 ns)
  26325.0 |########################################
  26491.7 |
  26658.4 |
  26825.1 |########################################
  26991.8 |
  27158.5 |
  27325.2 |
  27491.9 |
  27658.6 |
  27825.3 |########################################
  27992.0 |
  28158.7 |
  28325.4 |
  28492.1 |
  28658.8 |
  28825.5 |
  28992.2 |
  29158.9 |
  29325.6 |########################################
  29492.3 |########################################
  (0 below, 1 above range)

carrier_cold_leaf_switch (n=6, range 50139.6-53406.2 ns)
  50139.6 |########################################
  50302.9 |########################################
  50466.3 |########################################
  50629.6 |
  50792.9 |
  50956.3 |
  51119.6 |
  51282.9 |
  51446.3 |
  51609.6 |
  51772.9 |
  51936.3 |
  52099.6 |########################################
  52262.9 |
  52426.3 |
  52589.6 |
  52752.9 |
  52916.3 |########################################
  53079.6 |
  53242.9 |
  (0 below, 1 above range)

carrier_cold_leaf_threaded (n=6, range 48622.5-53364.6 ns)
  48622.5 |########################################
  48859.6 |
  49096.7 |
  49333.8 |
  49570.9 |########################################
  49808.0 |
  50045.1 |
  50282.2 |
  50519.3 |
  50756.4 |########################################
  50993.6 |########################################
  51230.7 |
  51467.8 |
  51704.9 |
  51942.0 |
  52179.1 |
  52416.2 |########################################
  52653.3 |
  52890.4 |
  53127.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_leaf_fntable**: bridge=200.6% of algo (FFI overhead may distort results)
- **carrier_cold_leaf_null**: bridge=355.3% of algo (FFI overhead may distort results)
- **carrier_cold_leaf_switch**: bridge=205.9% of algo (FFI overhead may distort results)
- **carrier_cold_leaf_threaded**: bridge=200.4% of algo (FFI overhead may distort results)
