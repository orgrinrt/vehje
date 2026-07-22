# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 dominates: 43% faster than the next best (iterfuse_push2)

iterfuse_pull2 (284.65 us) leads iterfuse_push2 (406.41 us) by 43%, a clear separation rather than a photo finish. CV 7.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_pull2 is fastest but the noisiest (CV 7.8%)

iterfuse_pull2 wins on median (284.65 us) yet has the highest variance (CV 7.8%), while iterfuse_push2 is the steadiest (CV 1.5%, 406.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iterfuse_pull2 shows alternating (throttle bounce) (autocorr -0.58)

iterfuse_pull2's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (284.65 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 284655.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.50x (fastest 284655.0 ns, slowest 427186.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 425774ns | 429361ns | 404191ns | 428649ns | 432252ns | +52.73% |
| iterfuse_pull2 | 278780ns | 287085ns | 233768ns | 281604ns | 297049ns | base |
| iterfuse_push2 | 409476ns | 409081ns | 401925ns | 407114ns | 416794ns | +46.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 423471ns | 401954ns | 430012ns | +53.24% | 0.019 |
| iterfuse_pull2 | 276349ns | 231488ns | 294512ns | base | 0.030 |
| iterfuse_push2 | 406927ns | 399265ns | 414249ns | +47.25% | 0.020 |

## Performance model

- Peak throughput: **0.035 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.019 | 54.2% |
| iterfuse_pull2 | 0.029 | 81.3% |
| iterfuse_push2 | 0.020 | 57.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 425774ns | 425774ns | +52.73% |
| iterfuse_pull2 | 278780ns | 278780ns | base |
| iterfuse_push2 | 409476ns | 409476ns | +46.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 284655ns | base | --- | [249881, 294512] | --- | --- | --- | --- |
| iterfuse_mat2 | 427186ns | +141180.8ns (+49.6%) | [+134987, +165197]ns | [413214, 430012] | YES | 0.0313 | 0.0313 | 0 |
| iterfuse_push2 | 406410ns | +124868.1ns (+43.9%) | [+108759, +158106]ns | [400123, 414249] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 299903ns | +44.0% | +35.8% |
| 2 | 231488ns | +73.6% | +77.3% |
| 3 | 289121ns | +47.7% | +38.1% |
| 4 | 280864ns | +51.1% | +48.8% |
| 5 | 268274ns | +59.6% | +51.2% |
| 6 | 288446ns | +48.1% | +39.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | -0.400 | moderate- |
| iterfuse_pull2 | -0.576 | HIGH- (thermal bounce) |
| iterfuse_push2 | -0.508 | HIGH- (thermal bounce) |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 3060.2ns | 423470.8ns | 0.7% |  |
| iterfuse_pull2 | 5.4ns | 276349.4ns | 0.0% |  |
| iterfuse_push2 | 4.4ns | 406927.3ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 401953.7-430012.5 ns)
  401953.7 |####################
  403356.6 |
  404759.6 |
  406162.5 |
  407565.5 |
  408968.4 |
  410371.3 |
  411774.3 |
  413177.2 |
  414580.2 |
  415983.1 |
  417386.0 |
  418789.0 |
  420191.9 |
  421594.9 |
  422997.8 |
  424400.7 |####################
  425803.7 |########################################
  427206.6 |####################
  428609.6 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 231488.3-294512.0 ns)
  231488.3 |####################
  234639.5 |
  237790.7 |
  240941.9 |
  244093.0 |
  247244.2 |
  250395.4 |
  253546.6 |
  256697.8 |
  259849.0 |
  263000.2 |
  266151.4 |####################
  269302.5 |
  272453.7 |
  275604.9 |
  278756.1 |####################
  281907.3 |
  285058.5 |
  288209.7 |########################################
  291360.9 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 399265.0-414249.2 ns)
  399265.0 |########################################
  400014.2 |
  400763.4 |########################################
  401512.6 |
  402261.8 |
  403011.0 |
  403760.2 |
  404509.5 |
  405258.7 |########################################
  406007.9 |
  406757.1 |########################################
  407506.3 |
  408255.5 |
  409004.7 |
  409753.9 |########################################
  410503.1 |
  411252.3 |
  412001.5 |
  412750.7 |
  413499.9 |
  (0 below, 1 above range)

```
