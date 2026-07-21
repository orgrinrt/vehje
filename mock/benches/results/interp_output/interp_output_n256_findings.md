# Interp output-building: format-to-temp+copy vs in-place vs span-list

3 variants, 6 samples per variant.
Baseline: **interp_out_inplace**

## Highlights

Baseline for all deltas below: **interp_out_inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### interp_out_inplace dominates: 10% faster than the next best (interp_out_temp)

interp_out_inplace (59.11 us) leads interp_out_temp (65.31 us) by 10%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### interp_out_spanlist is an outlier: 2.5x slower than the field

interp_out_spanlist (147.91 us) is 2.5x the fastest (59.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### interp_out_inplace is fastest but the noisiest (CV 6.2%)

interp_out_inplace wins on median (59.11 us) yet has the highest variance (CV 6.2%), while interp_out_spanlist is the steadiest (CV 1.1%, 147.91 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### interp_out_temp shows alternating (throttle bounce) (autocorr -0.56)

interp_out_temp's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (interp_out_inplace)

The baseline interp_out_inplace is the fastest (59.11 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (interp_out_inplace) is the fastest** at 59111.2 ns median
- 2 variants significantly slower than baseline
- Spread: 2.50x (fastest 59111.2 ns, slowest 147912.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| interp_out_inplace | 62891ns | 61564ns | 59897ns | 61431ns | 66580ns | base |
| interp_out_spanlist | 150009ns | 150266ns | 147077ns | 149923ns | 151603ns | +138.52% |
| interp_out_temp | 68637ns | 67608ns | 65110ns | 67534ns | 72056ns | +9.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| interp_out_inplace | 60426ns | 57607ns | 64068ns | base | 0.004 |
| interp_out_spanlist | 147573ns | 144719ns | 149207ns | +144.22% | 0.002 |
| interp_out_temp | 66266ns | 62905ns | 69517ns | +9.66% | 0.004 |

## Performance model

- Peak throughput: **0.004 Gops/s** (interp_out_inplace; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| interp_out_inplace | 0.004 | 97.5% |
| interp_out_spanlist | 0.002 | 38.9% |
| interp_out_temp | 0.004 | 88.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| interp_out_inplace | 62891ns | 62891ns | base |
| interp_out_spanlist | 150009ns | 150009ns | +138.52% |
| interp_out_temp | 68637ns | 68637ns | +9.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| interp_out_inplace | 59111ns | base | --- | [58100, 64068] | --- | --- | --- | --- |
| interp_out_spanlist | 147913ns | +88942.3ns (+150.5%) | [+81533, +90967]ns | [145600, 149207] | YES | 0.0313 | 0.0313 | 0 |
| interp_out_temp | 65311ns | +6371.2ns (+10.8%) | [+2086, +9061]ns | [63969, 69517] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | interp_out_inplace | interp_out_spanlist | interp_out_temp |
|---|---|---|---|
| 1 | 59621ns | +142.7% | +5.5% |
| 2 | 59348ns | +149.3% | +17.3% |
| 3 | 58592ns | +152.3% | +11.0% |
| 4 | 58874ns | +152.9% | +10.7% |
| 5 | 68514ns | +113.8% | +1.3% |
| 6 | 57607ns | +159.6% | +13.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| interp_out_inplace | -0.366 | moderate- |
| interp_out_spanlist | -0.280 | moderate- |
| interp_out_temp | -0.561 | HIGH- (thermal bounce) |

**Consistency summary:**

- **interp_out_spanlist**: won 0/6, lost 6/6
- **interp_out_temp**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| interp_out_inplace | 6.9ns | 60426.2ns | 0.0% |  |
| interp_out_spanlist | 6.2ns | 147573.5ns | 0.0% |  |
| interp_out_temp | 3.1ns | 66265.6ns | 0.0% |  |

## Distribution (algo ns)

```
interp_out_inplace (n=6, range 57607.1-64067.5 ns)
  57607.1 |####################
  57930.1 |
  58253.1 |
  58576.2 |########################################
  58899.2 |
  59222.2 |####################
  59545.2 |####################
  59868.2 |
  60191.3 |
  60514.3 |
  60837.3 |
  61160.3 |
  61483.3 |
  61806.4 |
  62129.4 |
  62452.4 |
  62775.4 |
  63098.4 |
  63421.5 |
  63744.5 |
  (0 below, 1 above range)

interp_out_spanlist (n=6, range 144719.2-149207.3 ns)
  144719.2 |########################################
  144943.6 |
  145168.0 |
  145392.4 |
  145616.8 |
  145841.2 |
  146065.6 |
  146290.0 |########################################
  146514.4 |
  146738.8 |
  146963.2 |
  147187.7 |
  147412.1 |
  147636.5 |########################################
  147860.9 |########################################
  148085.3 |
  148309.7 |
  148534.1 |
  148758.5 |########################################
  148982.9 |
  (0 below, 1 above range)

interp_out_temp (n=6, range 62905.0-69517.0 ns)
  62905.0 |####################
  63235.6 |
  63566.2 |
  63896.8 |
  64227.4 |
  64558.0 |
  64888.6 |########################################
  65219.2 |####################
  65549.8 |
  65880.4 |
  66211.0 |
  66541.6 |
  66872.2 |
  67202.8 |
  67533.4 |
  67864.0 |
  68194.6 |
  68525.2 |
  68855.8 |
  69186.4 |####################
  (0 below, 1 above range)

```
