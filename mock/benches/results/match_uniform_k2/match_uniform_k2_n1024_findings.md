# Match lowering: if-chain vs jump-table vs decision-tree, K=2 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u2**

## Highlights

Baseline for all deltas below: **ml_jumptable_u2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_ifchain_u2 is fastest but the noisiest (CV 5.1%)

ml_ifchain_u2 wins on median (63.70 us) yet has the highest variance (CV 5.1%), while ml_tree_u2 is the steadiest (CV 1.0%, 85.47 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: ml_ifchain_u2** at 63695.2 ns median (-2.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.34x (fastest 63695.2 ns, slowest 85469.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u2 | 67645ns | 65885ns | 64867ns | 65843ns | 71737ns | -1.09% |
| ml_jumptable_u2 | 68390ns | 67555ns | 65528ns | 67025ns | 71868ns | base |
| ml_tree_u2 | 87570ns | 87743ns | 86018ns | 87542ns | 88387ns | +28.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u2 | 65375ns | 62605ns | 69346ns | -1.03% | 0.016 |
| ml_jumptable_u2 | 66057ns | 63371ns | 69378ns | base | 0.016 |
| ml_tree_u2 | 85294ns | 83845ns | 86096ns | +29.12% | 0.012 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_u2; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u2 | 0.016 | 98.3% |
| ml_jumptable_u2 | 0.016 | 96.0% |
| ml_tree_u2 | 0.012 | 73.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u2 | 67645ns | 67645ns | -1.09% |
| ml_jumptable_u2 | 68390ns | 68390ns | base |
| ml_tree_u2 | 87570ns | 87570ns | +28.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u2 | 65246ns | base | --- | [63548, 69378] | --- | --- | --- | --- |
| ml_ifchain_u2 | 63695ns | no significant difference | [-4256, +2563]ns | [63084, 69346] | no | 1.0000 | 1.0000 | 0 |
| ml_tree_u2 | 85469ns | +19116.8ns (+29.3%) | [+16623, +21970]ns | [84315, 86096] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u2 | ml_ifchain_u2 | ml_tree_u2 |
|---|---|---|---|
| 1 | 63724ns | +0.0% | +33.3% |
| 2 | 64370ns | -1.1% | +30.3% |
| 3 | 63371ns | +0.3% | +35.9% |
| 4 | 66121ns | -5.3% | +28.2% |
| 5 | 67262ns | +7.3% | +27.9% |
| 6 | 71493ns | -7.0% | +20.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u2 | -0.004 | ok |
| ml_jumptable_u2 | 0.321 | moderate+ |
| ml_tree_u2 | -0.202 | moderate- |

**Consistency summary:**

- **ml_ifchain_u2**: won 3/6, lost 2/6
- **ml_tree_u2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u2 | 4.1ns | 65375.2ns | 0.0% |  |
| ml_jumptable_u2 | 3.0ns | 66057.1ns | 0.0% |  |
| ml_tree_u2 | 2.7ns | 85293.7ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u2 (n=6, range 62604.6-69345.9 ns)
  62604.6 |####################
  62941.7 |
  63278.7 |####################
  63615.8 |########################################
  63952.8 |
  64289.9 |
  64627.0 |
  64964.0 |
  65301.1 |
  65638.2 |
  65975.2 |
  66312.3 |####################
  66649.4 |
  66986.4 |
  67323.5 |
  67660.5 |
  67997.6 |
  68334.7 |
  68671.7 |
  69008.8 |
  (0 below, 1 above range)

ml_jumptable_u2 (n=6, range 63371.2-69377.7 ns)
  63371.2 |########################################
  63671.5 |########################################
  63971.8 |
  64272.2 |########################################
  64572.5 |
  64872.8 |
  65173.2 |
  65473.5 |
  65773.8 |
  66074.1 |########################################
  66374.5 |
  66674.8 |
  66975.1 |########################################
  67275.4 |
  67575.8 |
  67876.1 |
  68176.4 |
  68476.7 |
  68777.1 |
  69077.4 |
  (0 below, 1 above range)

ml_tree_u2 (n=6, range 83845.4-86096.2 ns)
  83845.4 |####################
  83957.9 |
  84070.5 |
  84183.0 |
  84295.6 |
  84408.1 |
  84520.7 |
  84633.2 |
  84745.7 |####################
  84858.3 |####################
  84970.8 |
  85083.4 |
  85195.9 |
  85308.5 |
  85421.0 |
  85533.5 |
  85646.1 |
  85758.6 |
  85871.2 |
  85983.7 |########################################
  (0 below, 1 above range)

```
