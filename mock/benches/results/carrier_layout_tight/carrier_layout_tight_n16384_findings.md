# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6.52 us) is smaller than the fastest variant's own run-to-run std-dev (9.13 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.8% of the fastest

All 5 variants sit between 838.55 us and 845.07 us - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_tight_rec20** at 838552.7 ns median (-0.4% vs baseline)
- Spread: 1.01x (fastest 838552.7 ns, slowest 845071.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 844803ns | 842155ns | 838892ns | 841335ns | 852961ns | -0.70% |
| carrier_lay_tight_rec16 | 847715ns | 847924ns | 837676ns | 846217ns | 854981ns | -0.36% |
| carrier_lay_tight_rec20 | 846012ns | 842106ns | 837608ns | 841135ns | 857530ns | -0.56% |
| carrier_lay_tight_rec24 | 850757ns | 845093ns | 840175ns | 843771ns | 866529ns | base |
| carrier_lay_tight_rec32 | 846327ns | 846052ns | 840145ns | 845419ns | 850779ns | -0.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 841629ns | 836246ns | 849727ns | -0.71% | 0.019 |
| carrier_lay_tight_rec16 | 844714ns | 834054ns | 852435ns | -0.35% | 0.019 |
| carrier_lay_tight_rec20 | 842635ns | 833880ns | 854671ns | -0.59% | 0.019 |
| carrier_lay_tight_rec24 | 847655ns | 836601ns | 863730ns | base | 0.019 |
| carrier_lay_tight_rec32 | 843357ns | 837843ns | 847984ns | -0.51% | 0.019 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 5269089 | 19144244 | 0.275 | 1.00× |
| carrier_lay_tight_rec16 | 5272159 | 19145375 | 0.275 | 1.00× |
| carrier_lay_tight_rec20 | 5274183 | 19143840 | 0.276 | 1.00× |
| carrier_lay_tight_rec24 | 5284102 | 19146805 | 0.276 | 1.00× |
| carrier_lay_tight_rec32 | 5279015 | 19144130 | 0.276 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.020 Gops/s** (carrier_lay_tight_rec20; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.020 | 99.4% |
| carrier_lay_tight_rec16 | 0.019 | 98.7% |
| carrier_lay_tight_rec20 | 0.020 | 99.4% |
| carrier_lay_tight_rec24 | 0.019 | 99.1% |
| carrier_lay_tight_rec32 | 0.019 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 844803ns | 844803ns | -0.70% |
| carrier_lay_tight_rec16 | 847715ns | 847715ns | -0.36% |
| carrier_lay_tight_rec20 | 846012ns | 846012ns | -0.56% |
| carrier_lay_tight_rec24 | 850757ns | 850757ns | base |
| carrier_lay_tight_rec32 | 846327ns | 846327ns | -0.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 841563ns | base | --- | [837673, 863730] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 838773ns | no significant difference | [-20664, +4876]ns | [836386, 849727] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_tight_rec16 | 845072ns | no significant difference | [-27095, +14762]ns | [836636, 852435] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_tight_rec20 | 838553ns | no significant difference | [-26950, +16998]ns | [834680, 854671] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_tight_rec32 | 842854ns | no significant difference | [-15746, +4779]ns | [839231, 847984] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 838745ns | -0.3% | +1.6% | +2.5% | -0.1% |
| 2 | 843575ns | -0.3% | +0.1% | -0.7% | -0.4% |
| 3 | 839550ns | -0.3% | +0.8% | -0.5% | +0.2% |
| 4 | 874662ns | -4.4% | -4.1% | -4.0% | -2.7% |
| 5 | 852799ns | +0.6% | -2.2% | -2.2% | -0.9% |
| 6 | 836601ns | +0.5% | +1.9% | +1.6% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | -0.197 | ok |
| carrier_lay_tight_rec16 | -0.140 | ok |
| carrier_lay_tight_rec20 | -0.133 | ok |
| carrier_lay_tight_rec24 | -0.065 | ok |
| carrier_lay_tight_rec32 | 0.187 | ok |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 4/6, lost 2/6
- **carrier_lay_tight_rec16**: won 2/6, lost 3/6
- **carrier_lay_tight_rec20**: won 4/6, lost 2/6
- **carrier_lay_tight_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 843235.9ns | 841628.9ns | 100.2% | HIGH |
| carrier_lay_tight_rec16 | 846748.4ns | 844714.2ns | 100.2% | HIGH |
| carrier_lay_tight_rec20 | 844187.1ns | 842634.6ns | 100.2% | HIGH |
| carrier_lay_tight_rec24 | 849882.5ns | 847655.3ns | 100.3% | HIGH |
| carrier_lay_tight_rec32 | 846000.8ns | 843356.7ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 836246.2-849727.1 ns)
  836246.2 |########################################
  836920.2 |
  837594.3 |
  838268.3 |
  838942.4 |
  839616.4 |
  840290.5 |#############
  840964.5 |#############
  841638.5 |
  842312.6 |
  842986.6 |
  843660.7 |
  844334.7 |
  845008.8 |
  845682.8 |
  846356.8 |
  847030.9 |
  847704.9 |
  848379.0 |
  849053.0 |
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 834054.2-852435.2 ns)
  834054.2 |########################################
  834973.2 |
  835892.3 |
  836811.3 |
  837730.4 |
  838649.4 |########################################
  839568.5 |
  840487.5 |
  841406.6 |
  842325.6 |
  843244.7 |
  844163.8 |########################################
  845082.8 |########################################
  846001.8 |
  846920.9 |
  847839.9 |
  848759.0 |
  849678.0 |
  850597.1 |
  851516.1 |########################################
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 833880.4-854670.6 ns)
  833880.4 |########################################
  834919.9 |########################################
  835959.4 |
  836998.9 |########################################
  838038.4 |
  839078.0 |########################################
  840117.5 |
  841157.0 |
  842196.5 |
  843236.0 |
  844275.5 |
  845315.0 |
  846354.5 |
  847394.0 |
  848433.5 |
  849473.1 |########################################
  850512.6 |
  851552.1 |
  852591.6 |
  853631.1 |
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 836601.2-863730.4 ns)
  836601.2 |########################################
  837957.7 |########################################
  839314.1 |########################################
  840670.6 |
  842027.0 |
  843383.5 |########################################
  844740.0 |
  846096.4 |
  847452.9 |
  848809.4 |
  850165.8 |
  851522.3 |########################################
  852878.8 |
  854235.2 |
  855591.7 |
  856948.1 |
  858304.6 |
  859661.1 |
  861017.5 |
  862374.0 |
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 837843.3-847984.4 ns)
  837843.3 |########################################
  838350.4 |
  838857.4 |
  839364.5 |
  839871.5 |
  840378.6 |########################################
  840885.6 |########################################
  841392.7 |
  841899.7 |
  842406.8 |
  842913.8 |
  843420.9 |
  843928.0 |
  844435.0 |########################################
  844942.1 |########################################
  845449.1 |
  845956.2 |
  846463.2 |
  846970.3 |
  847477.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=100.3% of algo (FFI overhead may distort results)
