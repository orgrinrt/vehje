# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_rec24) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_rec24 has the worst median (384.17 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_rec16 at 379.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5.01 us) is smaller than the fastest variant's own run-to-run std-dev (6.58 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.3% of the fastest

All 5 variants sit between 379.16 us and 384.17 us - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_rec16** at 379161.9 ns median (-1.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 379161.9 ns, slowest 384174.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 384637ns | 384554ns | 376153ns | 383304ns | 390878ns | -0.39% |
| carrier_rec16 | 379200ns | 382010ns | 364590ns | 381234ns | 383452ns | -1.79% |
| carrier_rec20 | 384000ns | 382856ns | 375030ns | 381590ns | 392100ns | -0.55% |
| carrier_rec24 | 386127ns | 386722ns | 381965ns | 385416ns | 389276ns | base |
| carrier_rec32 | 384191ns | 386272ns | 369295ns | 385064ns | 390329ns | -0.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 381848ns | 373552ns | 387987ns | -0.42% | 0.011 |
| carrier_rec16 | 376482ns | 362105ns | 380799ns | -1.81% | 0.011 |
| carrier_rec20 | 381268ns | 372349ns | 389318ns | -0.57% | 0.011 |
| carrier_rec24 | 383440ns | 379370ns | 386450ns | base | 0.011 |
| carrier_rec32 | 381385ns | 366492ns | 387554ns | -0.54% | 0.011 |

## Performance model

- Peak throughput: **0.011 Gops/s** (carrier_rec16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.011 | 94.8% |
| carrier_rec16 | 0.011 | 95.5% |
| carrier_rec20 | 0.011 | 95.2% |
| carrier_rec24 | 0.011 | 94.3% |
| carrier_rec32 | 0.011 | 94.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 384637ns | 384637ns | -0.39% |
| carrier_rec16 | 379200ns | 379200ns | -1.79% |
| carrier_rec20 | 384000ns | 384000ns | -0.55% |
| carrier_rec24 | 386127ns | 386127ns | base |
| carrier_rec32 | 384191ns | 384191ns | -0.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 384174ns | base | --- | [379697, 386450] | --- | --- | --- | --- |
| carrier_rec12 | 381824ns | no significant difference | [-8852, +6966]ns | [375734, 387987] | no | 0.9167 | 0.6875 | 0 |
| carrier_rec16 | 379162ns | -3566.3ns (-0.9%) | [-16089, -1220]ns | [369485, 380799] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_rec20 | 380211ns | no significant difference | [-12175, +5144]ns | [374275, 389318] | no | 0.9167 | 0.6875 | 0 |
| carrier_rec32 | 383431ns | no significant difference | [-11417, +4540]ns | [373170, 387554] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 380023ns | +2.9% | -0.1% | +0.1% | +1.4% |
| 2 | 379370ns | +0.8% | -0.7% | +0.2% | +0.8% |
| 3 | 386071ns | -0.2% | -1.9% | -3.6% | -0.4% |
| 4 | 386005ns | -1.2% | -1.2% | +0.6% | +1.0% |
| 5 | 382344ns | -2.3% | -0.6% | +2.1% | -4.1% |
| 6 | 386829ns | -2.3% | -6.4% | -2.7% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | 0.246 | moderate+ |
| carrier_rec16 | -0.088 | ok |
| carrier_rec20 | -0.137 | ok |
| carrier_rec24 | 0.063 | ok |
| carrier_rec32 | -0.214 | moderate- |

**Consistency summary:**

- **carrier_rec12**: won 4/6, lost 2/6
- **carrier_rec16**: won 5/6, lost 0/6
- **carrier_rec20**: won 2/6, lost 3/6
- **carrier_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 451.1ns | 381848.2ns | 0.1% |  |
| carrier_rec16 | 436.9ns | 376481.8ns | 0.1% |  |
| carrier_rec20 | 414.4ns | 381268.0ns | 0.1% |  |
| carrier_rec24 | 422.7ns | 383440.4ns | 0.1% |  |
| carrier_rec32 | 422.3ns | 381384.7ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 373551.7-387986.8 ns)
  373551.7 |########################################
  374273.5 |
  374995.2 |
  375717.0 |
  376438.7 |
  377160.5 |
  377882.2 |########################################
  378604.0 |
  379325.8 |
  380047.5 |
  380769.3 |########################################
  381491.0 |
  382212.8 |########################################
  382934.5 |
  383656.3 |
  384378.1 |
  385099.8 |########################################
  385821.6 |
  386543.3 |
  387265.1 |
  (0 below, 1 above range)

carrier_rec16 (n=6, range 362104.6-380798.9 ns)
  362104.6 |########################################
  363039.3 |
  363974.0 |
  364908.8 |
  365843.5 |
  366778.2 |
  367712.9 |
  368647.6 |
  369582.3 |
  370517.1 |
  371451.8 |
  372386.5 |
  373321.2 |
  374255.9 |
  375190.6 |
  376125.4 |########################################
  377060.1 |
  377994.8 |########################################
  378929.5 |########################################
  379864.2 |########################################
  (0 below, 1 above range)

carrier_rec20 (n=6, range 372349.2-389317.9 ns)
  372349.2 |####################
  373197.6 |
  374046.1 |
  374894.5 |
  375742.9 |####################
  376591.4 |
  377439.8 |
  378288.2 |
  379136.7 |
  379985.1 |########################################
  380833.6 |
  381682.0 |
  382530.4 |
  383378.9 |
  384227.3 |
  385075.7 |
  385924.2 |
  386772.6 |
  387621.0 |####################
  388469.5 |
  (0 below, 1 above range)

carrier_rec24 (n=6, range 379370.4-386450.0 ns)
  379370.4 |####################
  379724.4 |####################
  380078.4 |
  380432.3 |
  380786.3 |
  381140.3 |
  381494.3 |
  381848.3 |
  382202.2 |####################
  382556.2 |
  382910.2 |
  383264.2 |
  383618.2 |
  383972.1 |
  384326.1 |
  384680.1 |
  385034.1 |
  385388.1 |
  385742.0 |########################################
  386096.0 |
  (0 below, 1 above range)

carrier_rec32 (n=6, range 366492.1-387553.8 ns)
  366492.1 |####################
  367545.2 |
  368598.3 |
  369651.3 |
  370704.4 |
  371757.5 |
  372810.6 |
  373863.7 |
  374916.8 |
  375969.8 |
  377022.9 |
  378076.0 |
  379129.1 |####################
  380182.2 |
  381235.3 |
  382288.3 |####################
  383341.4 |
  384394.5 |########################################
  385447.6 |
  386500.7 |
  (0 below, 1 above range)

```
