# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_threaded_v4 is fastest but the noisiest (CV 9.1%)

carrier_disp_threaded_v4 wins on median (9.12 us) yet has the highest variance (CV 9.1%), while carrier_disp_fntable_v4 is the steadiest (CV 7.2%, 10.35 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_disp_switch_v4 shows warm-up / thermal drift (autocorr +0.58)

carrier_disp_switch_v4's per-pass series has lag-1 autocorrelation +0.58, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_disp_threaded_v4** at 9115.4 ns median (-1.6% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.14x (fastest 9115.4 ns, slowest 10347.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 12802ns | 12692ns | 11744ns | 12380ns | 13964ns | +10.51% |
| carrier_disp_switch_v4 | 11584ns | 11728ns | 10465ns | 11309ns | 12557ns | base |
| carrier_disp_threaded_v4 | 11579ns | 11436ns | 10308ns | 11186ns | 12804ns | -0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 10410ns | 9524ns | 11339ns | +13.60% | 0.025 |
| carrier_disp_switch_v4 | 9164ns | 8293ns | 9928ns | base | 0.028 |
| carrier_disp_threaded_v4 | 9215ns | 8193ns | 10178ns | +0.56% | 0.028 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_disp_threaded_v4; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.025 | 79.2% |
| carrier_disp_switch_v4 | 0.028 | 88.4% |
| carrier_disp_threaded_v4 | 0.028 | 89.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 12802ns | 12802ns | +10.51% |
| carrier_disp_switch_v4 | 11584ns | 11584ns | base |
| carrier_disp_threaded_v4 | 11579ns | 11579ns | -0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 9268ns | base | --- | [8296, 9928] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 10348ns | +1349.4ns (+14.6%) | [+939, +1450]ns | [9544, 11339] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_disp_threaded_v4 | 9115ns | no significant difference | [-339, +304]ns | [8351, 10178] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 | carrier_disp_threaded_v4 |
|---|---|---|---|
| 1 | 8298ns | +14.8% | +2.5% |
| 2 | 8293ns | +15.3% | +2.9% |
| 3 | 8661ns | +16.5% | -5.4% |
| 4 | 9875ns | +14.8% | +3.7% |
| 5 | 9954ns | +6.5% | +1.6% |
| 6 | 9903ns | +14.5% | -2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | 0.322 | moderate+ |
| carrier_disp_switch_v4 | 0.576 | HIGH+ (drift/warm-up) |
| carrier_disp_threaded_v4 | 0.361 | moderate+ |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 0/6, lost 6/6
- **carrier_disp_threaded_v4**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 82.6ns | 10410.3ns | 0.8% |  |
| carrier_disp_switch_v4 | 75.1ns | 9164.1ns | 0.8% |  |
| carrier_disp_threaded_v4 | 82.6ns | 9215.0ns | 0.9% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 9524.2-11339.4 ns)
   9524.2 |########################################
   9615.0 |
   9705.7 |
   9796.5 |
   9887.2 |
   9978.0 |
  10068.8 |####################
  10159.5 |
  10250.3 |
  10341.0 |
  10431.8 |
  10522.6 |####################
  10613.3 |
  10704.1 |
  10794.8 |
  10885.6 |
  10976.4 |
  11067.1 |
  11157.9 |
  11248.6 |####################
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 8293.3-9928.3 ns)
   8293.3 |########################################
   8375.0 |
   8456.8 |
   8538.5 |
   8620.3 |####################
   8702.0 |
   8783.8 |
   8865.5 |
   8947.3 |
   9029.0 |
   9110.8 |
   9192.5 |
   9274.3 |
   9356.0 |
   9437.8 |
   9519.5 |
   9601.3 |
   9683.0 |
   9764.8 |
   9846.5 |########################################
  (0 below, 1 above range)

carrier_disp_threaded_v4 (n=6, range 8193.3-10178.4 ns)
   8193.3 |####################
   8292.6 |
   8391.8 |
   8491.1 |########################################
   8590.3 |
   8689.6 |
   8788.8 |
   8888.1 |
   8987.3 |
   9086.6 |
   9185.8 |
   9285.1 |
   9384.3 |
   9483.6 |
   9582.8 |
   9682.1 |####################
   9781.3 |
   9880.6 |
   9979.8 |
  10079.1 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_switch_v4**: autocorrelation=0.58 (measurement drift or warm-up artifact)
