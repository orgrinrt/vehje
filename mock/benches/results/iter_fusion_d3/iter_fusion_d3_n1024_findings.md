# Iterator fusion (depth 3): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull3**

## Highlights

Baseline for all deltas below: **iterfuse_pull3**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_push3 dominates: 19% faster than the next best (iterfuse_pull3)

iterfuse_push3 (12.50 us) leads iterfuse_pull3 (14.90 us) by 19%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: iterfuse_push3** at 12497.9 ns median (-16.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.70x (fastest 12497.9 ns, slowest 21290.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat3 | 24062ns | 23734ns | 23398ns | 23672ns | 24980ns | +39.00% |
| iterfuse_pull3 | 17311ns | 17325ns | 15843ns | 17289ns | 18078ns | base |
| iterfuse_push3 | 14932ns | 14986ns | 14141ns | 14887ns | 15395ns | -13.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat3 | 21584ns | 20958ns | 22421ns | +45.11% | 0.047 |
| iterfuse_pull3 | 14875ns | 13547ns | 15555ns | base | 0.069 |
| iterfuse_push3 | 12427ns | 11681ns | 12806ns | -16.45% | 0.082 |

## Performance model

- Peak throughput: **0.088 Gops/s** (iterfuse_push3; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat3 | 0.048 | 54.9% |
| iterfuse_pull3 | 0.069 | 78.4% |
| iterfuse_push3 | 0.082 | 93.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat3 | 24062ns | 24062ns | +39.00% |
| iterfuse_pull3 | 17311ns | 17311ns | base |
| iterfuse_push3 | 14932ns | 14932ns | -13.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull3 | 14901ns | base | --- | [14169, 15555] | --- | --- | --- | --- |
| iterfuse_mat3 | 21291ns | +6393.8ns (+42.9%) | [+5503, +8232]ns | [21041, 22421] | YES | 0.0313 | 0.0313 | 0 |
| iterfuse_push3 | 12498ns | -2296.8ns (-15.4%) | [-3412, -1634]ns | [11978, 12806] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull3 | iterfuse_mat3 | iterfuse_push3 |
|---|---|---|---|
| 1 | 13547ns | +60.3% | -9.4% |
| 2 | 14971ns | +41.1% | -14.7% |
| 3 | 15875ns | +32.0% | -20.6% |
| 4 | 14791ns | +44.8% | -16.2% |
| 5 | 14830ns | +56.0% | -13.5% |
| 6 | 15235ns | +38.9% | -23.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat3 | -0.179 | ok |
| iterfuse_pull3 | -0.044 | ok |
| iterfuse_push3 | -0.351 | moderate- |

**Consistency summary:**

- **iterfuse_mat3**: won 0/6, lost 6/6
- **iterfuse_push3**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat3 | 442.3ns | 21584.4ns | 2.0% |  |
| iterfuse_pull3 | 3.1ns | 14874.9ns | 0.0% |  |
| iterfuse_push3 | 2.8ns | 12427.3ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat3 (n=6, range 20957.5-22420.8 ns)
  20957.5 |####################
  21030.7 |
  21103.8 |########################################
  21177.0 |
  21250.2 |
  21323.3 |
  21396.5 |####################
  21469.7 |
  21542.8 |
  21616.0 |
  21689.2 |####################
  21762.3 |
  21835.5 |
  21908.7 |
  21981.8 |
  22055.0 |
  22128.2 |
  22201.3 |
  22274.5 |
  22347.7 |
  (0 below, 1 above range)

iterfuse_pull3 (n=6, range 13547.1-15555.0 ns)
  13547.1 |####################
  13647.5 |
  13747.9 |
  13848.3 |
  13948.7 |
  14049.1 |
  14149.5 |
  14249.9 |
  14350.3 |
  14450.7 |
  14551.0 |
  14651.4 |
  14751.8 |########################################
  14852.2 |
  14952.6 |####################
  15053.0 |
  15153.4 |####################
  15253.8 |
  15354.2 |
  15454.6 |
  (0 below, 1 above range)

iterfuse_push3 (n=6, range 11680.8-12806.2 ns)
  11680.8 |########################################
  11737.1 |
  11793.3 |
  11849.6 |
  11905.9 |
  11962.2 |
  12018.4 |
  12074.7 |
  12131.0 |
  12187.3 |
  12243.5 |########################################
  12299.8 |
  12356.1 |########################################
  12412.3 |
  12468.6 |
  12524.9 |
  12581.2 |########################################
  12637.4 |
  12693.7 |
  12750.0 |########################################
  (0 below, 1 above range)

```
