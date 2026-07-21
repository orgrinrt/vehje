# Match lowering: if-chain vs jump-table vs decision-tree, K=8 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u8**

## Highlights

Baseline for all deltas below: **ml_jumptable_u8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u8 is an outlier: 3.2x slower than the field

ml_tree_u8 (205.94 us) is 3.2x the fastest (63.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ml_ifchain_u8 is fastest but the noisiest (CV 5.1%)

ml_ifchain_u8 wins on median (63.98 us) yet has the highest variance (CV 5.1%), while ml_tree_u8 is the steadiest (CV 2.0%, 205.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ml_jumptable_u8 shows alternating (throttle bounce) (autocorr -0.62)

ml_jumptable_u8's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.2x the fastest

Fastest ml_ifchain_u8 (63.98 us) to slowest ml_tree_u8 (205.94 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ml_ifchain_u8** at 63977.1 ns median (-1.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.22x (fastest 63977.1 ns, slowest 205945.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u8 | 67859ns | 66226ns | 65444ns | 66111ns | 71690ns | -1.30% |
| ml_jumptable_u8 | 68755ns | 67115ns | 65930ns | 66758ns | 73163ns | base |
| ml_tree_u8 | 209501ns | 208211ns | 206347ns | 207797ns | 213636ns | +204.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u8 | 65559ns | 63247ns | 69276ns | -1.24% | 0.016 |
| ml_jumptable_u8 | 66381ns | 63612ns | 70700ns | base | 0.015 |
| ml_tree_u8 | 207155ns | 203920ns | 211210ns | +212.07% | 0.005 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_u8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u8 | 0.016 | 98.9% |
| ml_jumptable_u8 | 0.016 | 97.6% |
| ml_tree_u8 | 0.005 | 30.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u8 | 67859ns | 67859ns | -1.30% |
| ml_jumptable_u8 | 68755ns | 68755ns | base |
| ml_tree_u8 | 209501ns | 209501ns | +204.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u8 | 64784ns | base | --- | [63659, 70700] | --- | --- | --- | --- |
| ml_ifchain_u8 | 63977ns | no significant difference | [-3085, +854]ns | [63423, 69276] | no | 0.6875 | 0.6875 | 0 |
| ml_tree_u8 | 205945ns | +141694.8ns (+218.7%) | [+136199, +144429]ns | [204311, 211210] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u8 | ml_ifchain_u8 | ml_tree_u8 |
|---|---|---|---|
| 1 | 63705ns | -0.7% | +221.3% |
| 2 | 69948ns | -5.8% | +209.1% |
| 3 | 65811ns | -3.2% | +212.6% |
| 4 | 63757ns | +0.8% | +223.3% |
| 5 | 71452ns | +1.7% | +185.4% |
| 6 | 63612ns | -0.0% | +224.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u8 | -0.341 | moderate- |
| ml_jumptable_u8 | -0.619 | HIGH- (thermal bounce) |
| ml_tree_u8 | -0.266 | moderate- |

**Consistency summary:**

- **ml_ifchain_u8**: won 3/6, lost 2/6
- **ml_tree_u8**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u8 | 2.1ns | 65558.8ns | 0.0% |  |
| ml_jumptable_u8 | 2.7ns | 66380.8ns | 0.0% |  |
| ml_tree_u8 | 4.0ns | 207155.1ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u8 (n=6, range 63246.7-69276.1 ns)
  63246.7 |####################
  63548.2 |########################################
  63849.6 |
  64151.1 |####################
  64452.6 |
  64754.0 |
  65055.5 |
  65357.0 |
  65658.4 |####################
  65959.9 |
  66261.4 |
  66562.8 |
  66864.3 |
  67165.8 |
  67467.2 |
  67768.7 |
  68070.2 |
  68371.6 |
  68673.1 |
  68974.6 |
  (0 below, 1 above range)

ml_jumptable_u8 (n=6, range 63612.5-70699.8 ns)
  63612.5 |########################################
  63966.9 |
  64321.2 |
  64675.6 |
  65030.0 |
  65384.3 |
  65738.7 |#############
  66093.1 |
  66447.4 |
  66801.8 |
  67156.1 |
  67510.5 |
  67864.9 |
  68219.2 |
  68573.6 |
  68928.0 |
  69282.3 |
  69636.7 |#############
  69991.1 |
  70345.4 |
  (0 below, 1 above range)

ml_tree_u8 (n=6, range 203920.4-211209.6 ns)
  203920.4 |####################
  204284.9 |
  204649.3 |####################
  205013.8 |
  205378.2 |####################
  205742.7 |
  206107.2 |########################################
  206471.6 |
  206836.1 |
  207200.5 |
  207565.0 |
  207929.5 |
  208293.9 |
  208658.4 |
  209022.8 |
  209387.3 |
  209751.8 |
  210116.2 |
  210480.7 |
  210845.1 |
  (0 below, 1 above range)

```
