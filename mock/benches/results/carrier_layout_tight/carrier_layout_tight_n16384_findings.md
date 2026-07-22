# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.46 us) is smaller than the fastest variant's own run-to-run std-dev (4.64 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 5 variants sit between 830.99 us and 833.45 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_tight_rec16** at 830988.3 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 830988.3 ns, slowest 833446.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 835716ns | 836640ns | 829284ns | 836426ns | 837867ns | +0.22% |
| carrier_lay_tight_rec16 | 831914ns | 834080ns | 824844ns | 831329ns | 836325ns | -0.24% |
| carrier_lay_tight_rec20 | 836421ns | 836005ns | 834442ns | 835542ns | 838730ns | +0.30% |
| carrier_lay_tight_rec24 | 833914ns | 835059ns | 828049ns | 834367ns | 836166ns | base |
| carrier_lay_tight_rec32 | 834042ns | 835048ns | 826927ns | 834666ns | 836663ns | +0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 832628ns | 826116ns | 834840ns | +0.24% | 0.020 |
| carrier_lay_tight_rec16 | 828800ns | 821751ns | 832895ns | -0.22% | 0.020 |
| carrier_lay_tight_rec20 | 833123ns | 831230ns | 835533ns | +0.30% | 0.020 |
| carrier_lay_tight_rec24 | 830618ns | 824625ns | 832882ns | base | 0.020 |
| carrier_lay_tight_rec32 | 830675ns | 823157ns | 833388ns | +0.01% | 0.020 |

## Performance model

- Peak throughput: **0.020 Gops/s** (carrier_lay_tight_rec16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.020 | 98.6% |
| carrier_lay_tight_rec16 | 0.020 | 98.9% |
| carrier_lay_tight_rec20 | 0.020 | 98.7% |
| carrier_lay_tight_rec24 | 0.020 | 98.8% |
| carrier_lay_tight_rec32 | 0.020 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 835716ns | 835716ns | +0.22% |
| carrier_lay_tight_rec16 | 831914ns | 831914ns | -0.24% |
| carrier_lay_tight_rec20 | 836421ns | 836421ns | +0.30% |
| carrier_lay_tight_rec24 | 833914ns | 833914ns | base |
| carrier_lay_tight_rec32 | 834042ns | 834042ns | +0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 831858ns | base | --- | [827115, 832882] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 833447ns | no significant difference | [-1795, +6102]ns | [829596, 834840] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_tight_rec16 | 830988ns | no significant difference | [-9930, +5780]ns | [822515, 832895] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec20 | 832320ns | no significant difference | [-1146, +7602]ns | [831515, 835533] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec32 | 831851ns | no significant difference | [-5805, +6273]ns | [826786, 833388] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 824625ns | +1.0% | +1.0% | +1.5% | +1.0% |
| 2 | 832587ns | +0.2% | +0.0% | -0.2% | -0.2% |
| 3 | 833178ns | -0.0% | -1.2% | -0.1% | -1.2% |
| 4 | 829605ns | -0.4% | +0.4% | +0.3% | +0.5% |
| 5 | 832003ns | +0.4% | -0.3% | -0.0% | -0.2% |
| 6 | 831713ns | +0.2% | -1.2% | +0.3% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | -0.320 | moderate- |
| carrier_lay_tight_rec16 | -0.241 | moderate- |
| carrier_lay_tight_rec20 | -0.235 | moderate- |
| carrier_lay_tight_rec24 | -0.183 | ok |
| carrier_lay_tight_rec32 | -0.367 | moderate- |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 1/6, lost 4/6
- **carrier_lay_tight_rec16**: won 3/6, lost 2/6
- **carrier_lay_tight_rec20**: won 2/6, lost 3/6
- **carrier_lay_tight_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 834627.2ns | 832627.5ns | 100.2% | HIGH |
| carrier_lay_tight_rec16 | 831396.6ns | 828799.5ns | 100.3% | HIGH |
| carrier_lay_tight_rec20 | 835916.9ns | 833122.8ns | 100.3% | HIGH |
| carrier_lay_tight_rec24 | 833455.5ns | 830618.5ns | 100.3% | HIGH |
| carrier_lay_tight_rec32 | 833413.8ns | 830674.7ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 826115.8-834839.8 ns)
  826115.8 |########################################
  826552.0 |
  826988.2 |
  827424.4 |
  827860.6 |
  828296.8 |
  828733.0 |
  829169.2 |
  829605.4 |
  830041.6 |
  830477.8 |
  830914.0 |
  831350.2 |
  831786.4 |
  832222.6 |
  832658.8 |########################################
  833095.0 |########################################
  833531.2 |########################################
  833967.4 |########################################
  834403.6 |
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 821750.8-832894.8 ns)
  821750.8 |####################
  822308.0 |
  822865.2 |####################
  823422.4 |
  823979.6 |
  824536.8 |
  825094.0 |
  825651.2 |
  826208.4 |
  826765.6 |
  827322.8 |
  827880.0 |
  828437.2 |
  828994.4 |####################
  829551.6 |
  830108.8 |
  830666.0 |
  831223.2 |
  831780.4 |
  832337.6 |########################################
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 831229.6-835533.1 ns)
  831229.6 |########################################
  831444.8 |
  831660.0 |########################################
  831875.1 |
  832090.3 |########################################
  832305.5 |########################################
  832520.7 |
  832735.8 |
  832951.0 |
  833166.2 |
  833381.4 |
  833596.6 |
  833811.7 |
  834026.9 |########################################
  834242.1 |
  834457.3 |
  834672.4 |
  834887.6 |
  835102.8 |
  835318.0 |
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 824625.0-832882.5 ns)
  824625.0 |####################
  825037.9 |
  825450.8 |
  825863.6 |
  826276.5 |
  826689.4 |
  827102.2 |
  827515.1 |
  827928.0 |
  828340.9 |
  828753.8 |
  829166.6 |
  829579.5 |####################
  829992.4 |
  830405.2 |
  830818.1 |
  831231.0 |
  831643.9 |########################################
  832056.8 |
  832469.6 |####################
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 823157.1-833387.7 ns)
  823157.1 |####################
  823668.6 |
  824180.2 |
  824691.7 |
  825203.2 |
  825714.8 |
  826226.3 |
  826737.8 |
  827249.3 |
  827760.9 |
  828272.4 |
  828783.9 |
  829295.5 |
  829807.0 |
  830318.5 |####################
  830830.0 |####################
  831341.6 |
  831853.1 |
  832364.6 |########################################
  832876.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=100.3% of algo (FFI overhead may distort results)
