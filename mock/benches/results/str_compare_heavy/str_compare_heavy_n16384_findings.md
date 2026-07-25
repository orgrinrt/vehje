# Runtime string interning, 8 compares per build: interning's best case

3 variants, 6 samples per variant.
Baseline: **str_h_plain**

## Highlights

Baseline for all deltas below: **str_h_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_h_plain dominates: 156% faster than the next best (str_h_eager)

str_h_plain (138.46 us) leads str_h_eager (354.04 us) by 156%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_h_lazy is an outlier: 3.3x slower than the field

str_h_lazy (458.84 us) is 3.3x the fastest (138.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (str_h_plain)

The baseline str_h_plain is the fastest (138.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.3x the fastest

Fastest str_h_plain (138.46 us) to slowest str_h_lazy (458.84 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_h_plain) is the fastest** at 138455.4 ns median
- 2 variants significantly slower than baseline
- Spread: 3.31x (fastest 138455.4 ns, slowest 458838.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_h_eager | 370573ns | 356492ns | 354203ns | 356215ns | 400294ns | +160.91% |
| str_h_lazy | 464049ns | 461575ns | 455245ns | 459947ns | 474605ns | +226.72% |
| str_h_plain | 142031ns | 141076ns | 139946ns | 140928ns | 144727ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_h_eager | 367864ns | 351605ns | 397193ns | +163.66% | 0.045 |
| str_h_lazy | 461391ns | 452848ns | 471840ns | +230.69% | 0.036 |
| str_h_plain | 139524ns | 137341ns | 142266ns | base | 0.117 |

## Performance model

- Peak throughput: **0.119 Gops/s** (str_h_plain; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_h_eager | 0.046 | 38.8% |
| str_h_lazy | 0.036 | 29.9% |
| str_h_plain | 0.118 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_h_eager | 370573ns | 370573ns | +160.91% |
| str_h_lazy | 464049ns | 464049ns | +226.72% |
| str_h_plain | 142031ns | 142031ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_h_plain | 138455ns | base | --- | [137851, 142266] | --- | --- | --- | --- |
| str_h_eager | 354037ns | +215888.5ns (+155.9%) | [+211132, +258000]ns | [352363, 397193] | YES | 0.0313 | 0.0313 | 0 |
| str_h_lazy | 458839ns | +317639.2ns (+229.4%) | [+315040, +332923]ns | [453495, 471840] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_h_plain | str_h_eager | str_h_lazy |
|---|---|---|---|
| 1 | 138365ns | +155.5% | +227.3% |
| 2 | 137341ns | +157.1% | +234.3% |
| 3 | 138545ns | +155.9% | +227.8% |
| 4 | 140026ns | +155.5% | +227.5% |
| 5 | 138360ns | +215.5% | +248.7% |
| 6 | 144505ns | +143.3% | +219.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_h_eager | -0.223 | moderate- |
| str_h_lazy | -0.011 | ok |
| str_h_plain | -0.066 | ok |

**Consistency summary:**

- **str_h_eager**: won 0/6, lost 6/6
- **str_h_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_h_eager | 4365.8ns | 367864.1ns | 1.2% |  |
| str_h_lazy | 4049.3ns | 461391.2ns | 0.9% |  |
| str_h_plain | 4249.0ns | 139523.9ns | 3.0% |  |

## Distribution (algo ns)

```
str_h_eager (n=6, range 351604.6-397192.7 ns)
  351604.6 |########################################
  353884.0 |#############
  356163.4 |#############
  358442.8 |
  360722.2 |
  363001.6 |
  365281.0 |
  367560.4 |
  369839.8 |
  372119.2 |
  374398.7 |
  376678.1 |
  378957.5 |
  381236.9 |
  383516.3 |
  385795.7 |
  388075.1 |
  390354.5 |
  392633.9 |
  394913.3 |
  (0 below, 1 above range)

str_h_lazy (n=6, range 452848.3-471839.6 ns)
  452848.3 |####################
  453797.9 |####################
  454747.4 |
  455697.0 |
  456646.6 |
  457596.1 |
  458545.7 |########################################
  459495.3 |
  460444.8 |####################
  461394.4 |
  462343.9 |
  463293.5 |
  464243.1 |
  465192.6 |
  466142.2 |
  467091.8 |
  468041.3 |
  468990.9 |
  469940.5 |
  470890.0 |
  (0 below, 1 above range)

str_h_plain (n=6, range 137341.2-142265.6 ns)
  137341.2 |#############
  137587.4 |
  137833.6 |
  138079.9 |
  138326.1 |########################################
  138572.3 |
  138818.5 |
  139064.7 |
  139311.0 |
  139557.2 |
  139803.4 |#############
  140049.6 |
  140295.8 |
  140542.1 |
  140788.3 |
  141034.5 |
  141280.7 |
  141526.9 |
  141773.2 |
  142019.4 |
  (0 below, 1 above range)

```
