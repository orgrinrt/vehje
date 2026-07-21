# Iterator fusion (depth 3): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull3**

## Highlights

Baseline for all deltas below: **iterfuse_pull3**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_push3 is fastest but the noisiest (CV 16.0%)

iterfuse_push3 wins on median (141.07 us) yet has the highest variance (CV 16.0%), while iterfuse_mat3 is the steadiest (CV 9.7%, 165.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iterfuse_mat3 shows alternating (throttle bounce) (autocorr -0.90)

iterfuse_mat3's per-pass series has lag-1 autocorrelation -0.90, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### iterfuse_push3 is inconsistent: worst-20% is 1.5x its best-20%

iterfuse_push3's best 20% of batches run at 111.76 us but its worst 20% at 168.88 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iterfuse_push3** at 141073.1 ns median (-7.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 141073.1 ns, slowest 165198.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat3 | 166921ns | 167487ns | 145540ns | 162078ns | 184878ns | +13.33% |
| iterfuse_pull3 | 147294ns | 154519ns | 122045ns | 145336ns | 162856ns | base |
| iterfuse_push3 | 144913ns | 143363ns | 114052ns | 137717ns | 171137ns | -1.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat3 | 164623ns | 143098ns | 182587ns | +13.55% | 0.025 |
| iterfuse_pull3 | 144982ns | 119744ns | 160431ns | base | 0.028 |
| iterfuse_push3 | 142642ns | 111755ns | 168878ns | -1.61% | 0.029 |

## Performance model

- Peak throughput: **0.037 Gops/s** (iterfuse_push3; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat3 | 0.025 | 67.6% |
| iterfuse_pull3 | 0.027 | 73.4% |
| iterfuse_push3 | 0.029 | 79.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat3 | 166921ns | 166921ns | +13.33% |
| iterfuse_pull3 | 147294ns | 147294ns | base |
| iterfuse_push3 | 144913ns | 144913ns | -1.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull3 | 152264ns | base | --- | [122252, 160431] | --- | --- | --- | --- |
| iterfuse_mat3 | 165198ns | +21912.0ns (+14.4%) | [+3631, +33380]ns | [146085, 182587] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| iterfuse_push3 | 141073ns | no significant difference | [-17254, +13635]ns | [117976, 168878] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull3 | iterfuse_mat3 | iterfuse_push3 |
|---|---|---|---|
| 1 | 119744ns | +30.1% | -6.7% |
| 2 | 149679ns | +20.5% | -4.2% |
| 3 | 154848ns | -7.6% | +18.0% |
| 4 | 165226ns | +11.8% | -16.1% |
| 5 | 124761ns | +19.5% | -0.5% |
| 6 | 155636ns | +12.2% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat3 | -0.898 | HIGH- (thermal bounce) |
| iterfuse_pull3 | -0.294 | moderate- |
| iterfuse_push3 | -0.100 | ok |

**Consistency summary:**

- **iterfuse_mat3**: won 1/6, lost 5/6
- **iterfuse_push3**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat3 | 1062.1ns | 164623.3ns | 0.6% |  |
| iterfuse_pull3 | 4.6ns | 144982.3ns | 0.0% |  |
| iterfuse_push3 | 2.4ns | 142642.1ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat3 (n=6, range 143097.9-182586.7 ns)
  143097.9 |########################################
  145072.3 |
  147046.8 |
  149021.2 |########################################
  150995.7 |
  152970.1 |
  154944.5 |########################################
  156919.0 |
  158893.4 |
  160867.9 |
  162842.3 |
  164816.7 |
  166791.2 |
  168765.6 |
  170740.1 |
  172714.5 |########################################
  174688.9 |
  176663.4 |
  178637.8 |########################################
  180612.3 |
  (0 below, 1 above range)

iterfuse_pull3 (n=6, range 119743.7-160431.0 ns)
  119743.7 |####################
  121778.1 |
  123812.4 |####################
  125846.8 |
  127881.2 |
  129915.5 |
  131949.9 |
  133984.3 |
  136018.6 |
  138053.0 |
  140087.4 |
  142121.7 |
  144156.1 |
  146190.4 |
  148224.8 |####################
  150259.2 |
  152293.5 |
  154327.9 |########################################
  156362.3 |
  158396.6 |
  (0 below, 1 above range)

iterfuse_push3 (n=6, range 111755.4-168877.5 ns)
  111755.4 |########################################
  114611.5 |
  117467.6 |
  120323.7 |
  123179.8 |########################################
  126035.9 |
  128892.0 |
  131748.1 |
  134604.2 |
  137460.3 |########################################
  140316.5 |
  143172.6 |########################################
  146028.7 |
  148884.8 |
  151740.9 |
  154597.0 |########################################
  157453.1 |
  160309.2 |
  163165.3 |
  166021.4 |
  (0 below, 1 above range)

```
