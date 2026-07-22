# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_lay_madd_rec24)

The baseline carrier_lay_madd_rec24 is the fastest (190.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.9% of the fastest

All 5 variants sit between 190.49 us and 194.18 us - a 1.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_lay_madd_rec24) is the fastest** at 190486.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.02x (fastest 190486.5 ns, slowest 194177.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 193362ns | 193603ns | 190957ns | 193506ns | 194350ns | +0.11% |
| carrier_lay_madd_rec16 | 194422ns | 193935ns | 193575ns | 193844ns | 195714ns | +0.66% |
| carrier_lay_madd_rec20 | 193984ns | 194114ns | 192845ns | 193985ns | 194550ns | +0.43% |
| carrier_lay_madd_rec24 | 193152ns | 193189ns | 191693ns | 192760ns | 194469ns | base |
| carrier_lay_madd_rec32 | 197025ns | 197014ns | 195993ns | 196770ns | 197925ns | +2.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 190641ns | 188157ns | 191727ns | +0.14% | 0.021 |
| carrier_lay_madd_rec16 | 191682ns | 190602ns | 193165ns | +0.68% | 0.021 |
| carrier_lay_madd_rec20 | 191117ns | 190070ns | 191668ns | +0.39% | 0.021 |
| carrier_lay_madd_rec24 | 190379ns | 188744ns | 191683ns | base | 0.022 |
| carrier_lay_madd_rec32 | 194286ns | 193502ns | 195146ns | +2.05% | 0.021 |

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_lay_madd_rec12; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.021 | 98.6% |
| carrier_lay_madd_rec16 | 0.021 | 98.4% |
| carrier_lay_madd_rec20 | 0.021 | 98.4% |
| carrier_lay_madd_rec24 | 0.022 | 98.8% |
| carrier_lay_madd_rec32 | 0.021 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 193362ns | 193362ns | +0.11% |
| carrier_lay_madd_rec16 | 194422ns | 194422ns | +0.66% |
| carrier_lay_madd_rec20 | 193984ns | 193984ns | +0.43% |
| carrier_lay_madd_rec24 | 193152ns | 193152ns | base |
| carrier_lay_madd_rec32 | 197025ns | 197025ns | +2.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 190486ns | base | --- | [188968, 191683] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 190781ns | no significant difference | [-2210, +2562]ns | [189416, 191727] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_madd_rec16 | 191125ns | +1491.2ns (+0.8%) | [+58, +2360]ns | [190756, 193165] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| carrier_lay_madd_rec20 | 191228ns | no significant difference | [-769, +2260]ns | [190455, 191668] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_madd_rec32 | 194178ns | +4090.4ns (+2.1%) | [+2167, +5462]ns | [193534, 195146] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 192419ns | -2.2% | +1.0% | -0.4% | +0.6% |
| 2 | 189192ns | +0.8% | +1.5% | +1.1% | +2.3% |
| 3 | 190768ns | +0.2% | +0.1% | -0.4% | +2.2% |
| 4 | 190205ns | +0.3% | +0.6% | +0.3% | +2.1% |
| 5 | 190947ns | -0.1% | -0.0% | +0.4% | +1.7% |
| 6 | 188744ns | +1.9% | +1.0% | +1.3% | +3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | 0.028 | ok |
| carrier_lay_madd_rec16 | 0.210 | moderate+ |
| carrier_lay_madd_rec20 | 0.078 | ok |
| carrier_lay_madd_rec24 | -0.455 | moderate- |
| carrier_lay_madd_rec32 | -0.050 | ok |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 1/6, lost 4/6
- **carrier_lay_madd_rec16**: won 0/6, lost 4/6
- **carrier_lay_madd_rec20**: won 2/6, lost 4/6
- **carrier_lay_madd_rec32**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 191417.6ns | 190641.1ns | 100.4% | HIGH |
| carrier_lay_madd_rec16 | 192264.7ns | 191682.4ns | 100.3% | HIGH |
| carrier_lay_madd_rec20 | 191717.9ns | 191116.9ns | 100.3% | HIGH |
| carrier_lay_madd_rec24 | 191168.0ns | 190379.2ns | 100.4% | HIGH |
| carrier_lay_madd_rec32 | 194982.6ns | 194285.9ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 188156.7-191726.7 ns)
  188156.7 |#############
  188335.2 |
  188513.7 |
  188692.2 |
  188870.7 |
  189049.2 |
  189227.7 |
  189406.2 |
  189584.7 |
  189763.2 |
  189941.7 |
  190120.2 |
  190298.7 |
  190477.2 |
  190655.7 |########################################
  190834.2 |
  191012.7 |#############
  191191.2 |
  191369.7 |
  191548.2 |
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 190602.5-193165.5 ns)
  190602.5 |####################
  190730.6 |
  190858.8 |########################################
  190986.9 |
  191115.1 |
  191243.2 |####################
  191371.4 |
  191499.5 |
  191627.7 |
  191755.8 |
  191884.0 |
  192012.1 |####################
  192140.3 |
  192268.4 |
  192396.6 |
  192524.7 |
  192652.9 |
  192781.0 |
  192909.2 |
  193037.3 |
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 190070.4-191667.7 ns)
  190070.4 |####################
  190150.3 |
  190230.1 |
  190310.0 |
  190389.9 |
  190469.7 |
  190549.6 |
  190629.5 |
  190709.3 |
  190789.2 |####################
  190869.0 |
  190948.9 |
  191028.8 |
  191108.6 |
  191188.5 |########################################
  191268.4 |
  191348.2 |
  191428.1 |
  191508.0 |####################
  191587.8 |
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 188744.2-191683.0 ns)
  188744.2 |########################################
  188891.1 |
  189038.1 |
  189185.0 |########################################
  189332.0 |
  189478.9 |
  189625.8 |
  189772.8 |
  189919.7 |
  190066.6 |########################################
  190213.6 |
  190360.5 |
  190507.5 |
  190654.4 |########################################
  190801.3 |########################################
  190948.3 |
  191095.2 |
  191242.1 |
  191389.1 |
  191536.0 |
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 193501.7-195145.9 ns)
  193501.7 |########################################
  193583.9 |
  193666.1 |
  193748.3 |
  193830.5 |
  193912.7 |
  193994.9 |
  194077.2 |####################
  194159.4 |####################
  194241.6 |
  194323.8 |
  194406.0 |
  194488.2 |
  194570.4 |
  194652.6 |
  194734.8 |
  194817.0 |
  194899.2 |####################
  194981.4 |
  195063.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=100.4% of algo (FFI overhead may distort results)
