# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_leaf_rec24 shows alternating (throttle bounce) (autocorr -0.62)

carrier_lay_leaf_rec24's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 2.3% of the fastest

All 5 variants sit between 283.60 us and 290.11 us - a 2.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_leaf_rec20** at 283602.0 ns median (-0.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 283602.0 ns, slowest 290114.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 286774ns | 286890ns | 281537ns | 286677ns | 289538ns | -0.06% |
| carrier_lay_leaf_rec16 | 287272ns | 287042ns | 285938ns | 286727ns | 288755ns | +0.12% |
| carrier_lay_leaf_rec20 | 286628ns | 286628ns | 284872ns | 286297ns | 288004ns | -0.11% |
| carrier_lay_leaf_rec24 | 286941ns | 287297ns | 283775ns | 286884ns | 288611ns | base |
| carrier_lay_leaf_rec32 | 292724ns | 293091ns | 290568ns | 292472ns | 294179ns | +2.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 283839ns | 278800ns | 286737ns | -0.06% | 0.014 |
| carrier_lay_leaf_rec16 | 284226ns | 282814ns | 285742ns | +0.08% | 0.014 |
| carrier_lay_leaf_rec20 | 283665ns | 281861ns | 285235ns | -0.12% | 0.014 |
| carrier_lay_leaf_rec24 | 283996ns | 281058ns | 285463ns | base | 0.014 |
| carrier_lay_leaf_rec32 | 289651ns | 287384ns | 291028ns | +1.99% | 0.014 |

## Performance model

- Peak throughput: **0.015 Gops/s** (carrier_lay_leaf_rec12; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.014 | 98.2% |
| carrier_lay_leaf_rec16 | 0.014 | 98.2% |
| carrier_lay_leaf_rec20 | 0.014 | 98.3% |
| carrier_lay_leaf_rec24 | 0.014 | 98.0% |
| carrier_lay_leaf_rec32 | 0.014 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 286774ns | 286774ns | -0.06% |
| carrier_lay_leaf_rec16 | 287272ns | 287272ns | +0.12% |
| carrier_lay_leaf_rec20 | 286628ns | 286628ns | -0.11% |
| carrier_lay_leaf_rec24 | 286941ns | 286941ns | base |
| carrier_lay_leaf_rec32 | 292724ns | 292724ns | +2.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 284468ns | base | --- | [282056, 285463] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 283780ns | no significant difference | [-3296, +2286]ns | [281000, 286737] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_leaf_rec16 | 283982ns | no significant difference | [-2094, +3106]ns | [282954, 285742] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec20 | 283602ns | no significant difference | [-2185, +1949]ns | [282159, 285235] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec32 | 290114ns | +5298.4ns (+1.9%) | [+4527, +7142]ns | [287812, 291028] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 284051ns | -1.8% | -0.3% | -0.8% | +1.5% |
| 2 | 284885ns | -0.5% | +0.1% | +0.0% | +1.7% |
| 3 | 281058ns | +0.8% | +1.1% | +0.5% | +2.3% |
| 4 | 285300ns | +0.9% | -0.5% | -0.5% | +1.8% |
| 5 | 283054ns | +0.3% | +1.1% | +0.9% | +2.8% |
| 6 | 285626ns | +0.0% | -1.0% | -0.8% | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.005 | ok |
| carrier_lay_leaf_rec16 | -0.554 | HIGH- (thermal bounce) |
| carrier_lay_leaf_rec20 | -0.417 | moderate- |
| carrier_lay_leaf_rec24 | -0.624 | HIGH- (thermal bounce) |
| carrier_lay_leaf_rec32 | 0.067 | ok |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 2/6, lost 3/6
- **carrier_lay_leaf_rec16**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec20**: won 3/6, lost 2/6
- **carrier_lay_leaf_rec32**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 284294.7ns | 283839.1ns | 100.2% | HIGH |
| carrier_lay_leaf_rec16 | 284916.7ns | 284226.0ns | 100.2% | HIGH |
| carrier_lay_leaf_rec20 | 284559.2ns | 283665.3ns | 100.3% | HIGH |
| carrier_lay_leaf_rec24 | 285009.4ns | 283995.6ns | 100.4% | HIGH |
| carrier_lay_leaf_rec32 | 289449.7ns | 289651.4ns | 99.9% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 278800.4-286737.1 ns)
  278800.4 |####################
  279197.2 |
  279594.1 |
  279990.9 |
  280387.7 |
  280784.6 |
  281181.4 |
  281578.2 |
  281975.1 |
  282371.9 |
  282768.7 |
  283165.6 |########################################
  283562.4 |
  283959.2 |####################
  284356.1 |
  284752.9 |
  285149.7 |
  285546.6 |####################
  285943.4 |
  286340.2 |
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 282813.8-285742.1 ns)
  282813.8 |########################################
  282960.2 |########################################
  283106.6 |
  283253.0 |
  283399.5 |
  283545.9 |
  283692.3 |
  283838.7 |########################################
  283985.1 |########################################
  284131.5 |
  284277.9 |
  284424.4 |
  284570.8 |
  284717.2 |
  284863.6 |
  285010.0 |
  285156.4 |########################################
  285302.9 |
  285449.3 |
  285595.7 |
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 281860.8-285234.8 ns)
  281860.8 |########################################
  282029.5 |
  282198.2 |
  282366.9 |########################################
  282535.6 |
  282704.3 |
  282873.0 |
  283041.7 |
  283210.4 |
  283379.1 |########################################
  283547.8 |
  283716.5 |########################################
  283885.2 |
  284053.9 |
  284222.6 |
  284391.3 |
  284560.0 |
  284728.7 |
  284897.4 |########################################
  285066.1 |
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 281057.9-285463.1 ns)
  281057.9 |########################################
  281278.2 |
  281498.4 |
  281718.7 |
  281938.9 |
  282159.2 |
  282379.5 |
  282599.7 |
  282820.0 |
  283040.2 |########################################
  283260.5 |
  283480.8 |
  283701.0 |
  283921.3 |########################################
  284141.5 |
  284361.8 |
  284582.1 |
  284802.3 |########################################
  285022.6 |
  285242.8 |########################################
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 287383.8-291027.5 ns)
  287383.8 |########################################
  287566.0 |
  287748.2 |
  287930.4 |
  288112.5 |########################################
  288294.7 |
  288476.9 |
  288659.1 |
  288841.3 |
  289023.5 |
  289205.7 |
  289387.8 |
  289570.0 |########################################
  289752.2 |
  289934.4 |
  290116.6 |
  290298.8 |########################################
  290480.9 |
  290663.1 |
  290845.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=100.0% of algo (FFI overhead may distort results)
