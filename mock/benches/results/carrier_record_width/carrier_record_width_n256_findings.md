# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (377 ns) is smaller than the fastest variant's own run-to-run std-dev (1.14 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_rec20 vs stability leader carrier_rec16 (+1% speed for 1.0x steadier)

carrier_rec20 is fastest (13.48 us, CV 8.5%); carrier_rec16 gives up 1.3% median for 1.0x lower variance (CV 8.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 2.8% of the fastest

All 5 variants sit between 13.48 us and 13.86 us - a 2.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_rec20** at 13483.8 ns median (-0.3% vs baseline)
- Spread: 1.03x (fastest 13483.8 ns, slowest 13861.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 16539ns | 16360ns | 14520ns | 16101ns | 18206ns | +5.21% |
| carrier_rec16 | 16031ns | 16179ns | 14422ns | 15614ns | 17460ns | +1.98% |
| carrier_rec20 | 15965ns | 15997ns | 13608ns | 15829ns | 17349ns | +1.56% |
| carrier_rec24 | 15720ns | 16047ns | 13741ns | 15282ns | 17366ns | base |
| carrier_rec32 | 16262ns | 16403ns | 14023ns | 15768ns | 18120ns | +3.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 13962ns | 12257ns | 15374ns | +5.26% | 0.018 |
| carrier_rec16 | 13525ns | 12169ns | 14738ns | +1.96% | 0.019 |
| carrier_rec20 | 13454ns | 11440ns | 14631ns | +1.43% | 0.019 |
| carrier_rec24 | 13265ns | 11597ns | 14647ns | base | 0.019 |
| carrier_rec32 | 13727ns | 11834ns | 15291ns | +3.48% | 0.019 |

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_rec20; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.019 | 82.8% |
| carrier_rec16 | 0.019 | 83.7% |
| carrier_rec20 | 0.019 | 84.8% |
| carrier_rec24 | 0.019 | 84.6% |
| carrier_rec32 | 0.018 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 16539ns | 16539ns | +5.21% |
| carrier_rec16 | 16031ns | 16031ns | +1.98% |
| carrier_rec20 | 15965ns | 15965ns | +1.56% |
| carrier_rec24 | 15720ns | 15720ns | base |
| carrier_rec32 | 16262ns | 16262ns | +3.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 13527ns | base | --- | [11622, 14647] | --- | --- | --- | --- |
| carrier_rec12 | 13815ns | no significant difference | [-324, +2004]ns | [12698, 15374] | no | 0.2917 | 0.2188 | 0 |
| carrier_rec16 | 13660ns | no significant difference | [-851, +1301]ns | [12177, 14738] | no | 0.2917 | 0.2188 | 0 |
| carrier_rec20 | 13484ns | no significant difference | [-855, +1524]ns | [12249, 14631] | no | 0.6875 | 0.6875 | 0 |
| carrier_rec32 | 13861ns | no significant difference | [-400, +1396]ns | [12028, 15291] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 11597ns | +5.7% | +14.2% | -1.4% | +5.4% |
| 2 | 13122ns | +16.8% | +7.3% | +7.1% | +16.5% |
| 3 | 15259ns | +1.1% | +0.3% | -0.3% | +0.2% |
| 4 | 11647ns | +15.5% | +4.5% | +18.2% | +1.6% |
| 5 | 13932ns | -5.7% | -12.5% | -6.3% | -6.0% |
| 6 | 14034ns | +1.0% | +1.0% | -6.0% | +4.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | -0.106 | ok |
| carrier_rec16 | -0.083 | ok |
| carrier_rec20 | 0.049 | ok |
| carrier_rec24 | -0.368 | moderate- |
| carrier_rec32 | -0.188 | ok |

**Consistency summary:**

- **carrier_rec12**: won 1/6, lost 5/6
- **carrier_rec16**: won 1/6, lost 5/6
- **carrier_rec20**: won 4/6, lost 2/6
- **carrier_rec32**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 85.6ns | 13962.5ns | 0.6% |  |
| carrier_rec16 | 81.6ns | 13525.2ns | 0.6% |  |
| carrier_rec20 | 84.6ns | 13454.4ns | 0.6% |  |
| carrier_rec24 | 81.6ns | 13265.1ns | 0.6% |  |
| carrier_rec32 | 84.8ns | 13726.9ns | 0.6% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 12257.1-15374.3 ns)
  12257.1 |########################################
  12413.0 |
  12568.8 |
  12724.7 |
  12880.5 |
  13036.4 |########################################
  13192.3 |
  13348.1 |########################################
  13504.0 |
  13659.9 |
  13815.7 |
  13971.6 |
  14127.4 |########################################
  14283.3 |
  14439.2 |
  14595.0 |
  14750.9 |
  14906.8 |
  15062.6 |
  15218.5 |########################################
  (0 below, 1 above range)

carrier_rec16 (n=6, range 12169.2-14738.4 ns)
  12169.2 |########################################
  12297.7 |
  12426.1 |
  12554.6 |
  12683.0 |
  12811.5 |
  12939.9 |
  13068.4 |
  13196.9 |####################
  13325.3 |
  13453.8 |
  13582.2 |
  13710.7 |
  13839.1 |
  13967.6 |####################
  14096.1 |####################
  14224.5 |
  14353.0 |
  14481.4 |
  14609.9 |
  (0 below, 1 above range)

carrier_rec20 (n=6, range 11440.0-14630.9 ns)
  11440.0 |########################################
  11599.5 |
  11759.1 |
  11918.6 |
  12078.2 |
  12237.7 |
  12397.3 |
  12556.8 |
  12716.3 |
  12875.9 |
  13035.4 |########################################
  13195.0 |########################################
  13354.5 |
  13514.1 |
  13673.6 |########################################
  13833.1 |
  13992.7 |########################################
  14152.2 |
  14311.8 |
  14471.3 |
  (0 below, 1 above range)

carrier_rec24 (n=6, range 11596.7-14646.7 ns)
  11596.7 |########################################
  11749.2 |
  11901.7 |
  12054.2 |
  12206.7 |
  12359.2 |
  12511.7 |
  12664.2 |
  12816.7 |
  12969.2 |
  13121.7 |####################
  13274.2 |
  13426.7 |
  13579.2 |
  13731.7 |
  13884.2 |########################################
  14036.7 |
  14189.2 |
  14341.7 |
  14494.2 |
  (0 below, 1 above range)

carrier_rec32 (n=6, range 11833.7-15291.5 ns)
  11833.7 |########################################
  12006.6 |
  12179.5 |########################################
  12352.4 |
  12525.2 |
  12698.1 |
  12871.0 |
  13043.9 |########################################
  13216.8 |
  13389.7 |
  13562.6 |
  13735.5 |
  13908.4 |
  14081.2 |
  14254.1 |
  14427.0 |
  14599.9 |########################################
  14772.8 |
  14945.7 |
  15118.6 |########################################
  (0 below, 1 above range)

```
