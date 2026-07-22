# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 4.9% of the fastest

All 5 variants sit between 57.54 us and 60.38 us - a 4.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_leaf_rec16** at 57535.4 ns median (-2.6% vs baseline)
- Spread: 1.05x (fastest 57535.4 ns, slowest 60383.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 62816ns | 62975ns | 58905ns | 62540ns | 65187ns | +3.15% |
| carrier_lay_leaf_rec16 | 60264ns | 59862ns | 57802ns | 59756ns | 62258ns | -1.05% |
| carrier_lay_leaf_rec20 | 60731ns | 60740ns | 58147ns | 60074ns | 63008ns | -0.28% |
| carrier_lay_leaf_rec24 | 60900ns | 61580ns | 56890ns | 60498ns | 63510ns | base |
| carrier_lay_leaf_rec32 | 62769ns | 63164ns | 58922ns | 62246ns | 65476ns | +3.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 60282ns | 56554ns | 62623ns | +3.17% | 0.017 |
| carrier_lay_leaf_rec16 | 57807ns | 55578ns | 59686ns | -1.07% | 0.018 |
| carrier_lay_leaf_rec20 | 58271ns | 55880ns | 60567ns | -0.27% | 0.018 |
| carrier_lay_leaf_rec24 | 58431ns | 54413ns | 60998ns | base | 0.018 |
| carrier_lay_leaf_rec32 | 60194ns | 56602ns | 62951ns | +3.02% | 0.017 |

## Performance model

- Peak throughput: **0.019 Gops/s** (carrier_lay_leaf_rec24; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.017 | 90.2% |
| carrier_lay_leaf_rec16 | 0.018 | 94.6% |
| carrier_lay_leaf_rec20 | 0.018 | 93.5% |
| carrier_lay_leaf_rec24 | 0.017 | 92.1% |
| carrier_lay_leaf_rec32 | 0.017 | 90.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 62816ns | 62816ns | +3.15% |
| carrier_lay_leaf_rec16 | 60264ns | 60264ns | -1.05% |
| carrier_lay_leaf_rec20 | 60731ns | 60731ns | -0.28% |
| carrier_lay_leaf_rec24 | 60900ns | 60900ns | base |
| carrier_lay_leaf_rec32 | 62769ns | 62769ns | +3.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 59055ns | base | --- | [55240, 60998] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 60343ns | no significant difference | [-3116, +5738]ns | [57881, 62623] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_leaf_rec16 | 57535ns | no significant difference | [-4386, +2730]ns | [56200, 59686] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec20 | 58193ns | no significant difference | [-1898, +1578]ns | [56053, 60567] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_leaf_rec32 | 60383ns | no significant difference | [-826, +4105]ns | [57248, 62951] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 60440ns | -6.4% | -8.0% | -0.2% | +0.9% |
| 2 | 54413ns | +9.9% | +4.4% | +5.9% | +9.8% |
| 3 | 56068ns | +10.9% | +5.4% | -0.3% | +3.3% |
| 4 | 58818ns | +3.5% | -2.4% | -0.1% | -3.8% |
| 5 | 59292ns | +6.4% | +1.6% | -5.2% | +4.8% |
| 6 | 61556ns | -3.8% | -6.4% | -1.2% | +3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.027 | ok |
| carrier_lay_leaf_rec16 | -0.067 | ok |
| carrier_lay_leaf_rec20 | -0.338 | moderate- |
| carrier_lay_leaf_rec24 | 0.097 | ok |
| carrier_lay_leaf_rec32 | 0.249 | moderate+ |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 2/6, lost 4/6
- **carrier_lay_leaf_rec16**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec20**: won 4/6, lost 1/6
- **carrier_lay_leaf_rec32**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 120599.7ns | 60282.5ns | 200.1% | HIGH |
| carrier_lay_leaf_rec16 | 115633.9ns | 57806.9ns | 200.0% | HIGH |
| carrier_lay_leaf_rec20 | 116804.3ns | 58271.1ns | 200.4% | HIGH |
| carrier_lay_leaf_rec24 | 116997.6ns | 58431.2ns | 200.2% | HIGH |
| carrier_lay_leaf_rec32 | 120282.2ns | 60194.0ns | 199.8% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 56554.2-62622.9 ns)
  56554.2 |########################################
  56857.6 |
  57161.1 |
  57464.5 |
  57767.9 |
  58071.4 |
  58374.8 |
  58678.2 |
  58981.7 |########################################
  59285.1 |
  59588.6 |########################################
  59892.0 |
  60195.4 |
  60498.9 |
  60802.3 |########################################
  61105.7 |
  61409.2 |
  61712.6 |
  62016.0 |########################################
  62319.5 |
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 55578.3-59685.6 ns)
  55578.3 |########################################
  55783.7 |
  55989.0 |
  56194.4 |
  56399.8 |
  56605.1 |
  56810.5 |########################################
  57015.9 |
  57221.2 |########################################
  57426.6 |
  57632.0 |########################################
  57837.3 |
  58042.7 |
  58248.1 |
  58453.4 |
  58658.8 |
  58864.2 |
  59069.5 |########################################
  59274.9 |
  59480.3 |
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 55880.4-60566.9 ns)
  55880.4 |########################################
  56114.7 |########################################
  56349.1 |
  56583.4 |
  56817.7 |
  57052.0 |
  57286.3 |
  57520.7 |########################################
  57755.0 |
  57989.3 |
  58223.6 |
  58458.0 |
  58692.3 |########################################
  58926.6 |
  59160.9 |
  59395.3 |
  59629.6 |
  59863.9 |
  60098.2 |########################################
  60332.6 |
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 54412.9-60997.9 ns)
  54412.9 |########################################
  54742.2 |
  55071.4 |
  55400.7 |
  55729.9 |
  56059.2 |########################################
  56388.4 |
  56717.7 |
  57046.9 |
  57376.2 |
  57705.4 |
  58034.7 |
  58363.9 |
  58693.2 |########################################
  59022.4 |########################################
  59351.7 |
  59680.9 |
  60010.2 |
  60339.4 |########################################
  60668.7 |
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 56601.7-62950.6 ns)
  56601.7 |########################################
  56919.1 |
  57236.6 |
  57554.0 |
  57871.5 |########################################
  58188.9 |
  58506.4 |
  58823.8 |
  59141.3 |
  59458.7 |########################################
  59776.1 |
  60093.6 |
  60411.0 |
  60728.5 |########################################
  61045.9 |
  61363.4 |
  61680.8 |
  61998.3 |########################################
  62315.7 |
  62633.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=200.6% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=199.6% of algo (FFI overhead may distort results)
