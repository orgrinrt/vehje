# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_leaf_rec32 is fastest but the noisiest (CV 5.1%)

carrier_lay_leaf_rec32 wins on median (55.71 us) yet has the highest variance (CV 5.1%), while carrier_lay_leaf_rec24 is the steadiest (CV 2.2%, 56.09 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (630 ns) is smaller than the fastest variant's own run-to-run std-dev (2.82 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_leaf_rec32 vs stability leader carrier_lay_leaf_rec24 (+1% speed for 2.3x steadier)

carrier_lay_leaf_rec32 is fastest (55.71 us, CV 5.1%); carrier_lay_leaf_rec24 gives up 0.7% median for 2.3x lower variance (CV 2.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.1% of the fastest

All 5 variants sit between 55.71 us and 56.34 us - a 1.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_leaf_rec32** at 55706.4 ns median (-0.7% vs baseline)
- Spread: 1.01x (fastest 55706.4 ns, slowest 56336.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 58739ns | 58190ns | 56035ns | 57761ns | 61558ns | +1.32% |
| carrier_lay_leaf_rec16 | 57832ns | 58545ns | 54853ns | 57727ns | 59478ns | -0.24% |
| carrier_lay_leaf_rec20 | 58537ns | 58140ns | 56690ns | 57721ns | 60684ns | +0.97% |
| carrier_lay_leaf_rec24 | 57973ns | 58284ns | 55962ns | 57791ns | 59253ns | base |
| carrier_lay_leaf_rec32 | 58940ns | 57875ns | 56745ns | 57541ns | 62138ns | +1.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 56582ns | 53842ns | 59400ns | +1.45% | 0.018 |
| carrier_lay_leaf_rec16 | 55642ns | 52686ns | 57284ns | -0.24% | 0.018 |
| carrier_lay_leaf_rec20 | 56347ns | 54548ns | 58447ns | +1.02% | 0.018 |
| carrier_lay_leaf_rec24 | 55775ns | 53753ns | 57044ns | base | 0.018 |
| carrier_lay_leaf_rec32 | 56756ns | 54562ns | 59920ns | +1.76% | 0.018 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 537816 | 1687974 | 0.319 | 1.02× |
| carrier_lay_leaf_rec16 | 523062 | 1687156 | 0.310 | 0.99× |
| carrier_lay_leaf_rec20 | 534270 | 1688058 | 0.316 | 1.01× |
| carrier_lay_leaf_rec24 | 527096 | 1688172 | 0.312 | 1.00× |
| carrier_lay_leaf_rec32 | 532832 | 1687979 | 0.316 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.019 Gops/s** (carrier_lay_leaf_rec16; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.018 | 94.0% |
| carrier_lay_leaf_rec16 | 0.018 | 93.5% |
| carrier_lay_leaf_rec20 | 0.018 | 94.1% |
| carrier_lay_leaf_rec24 | 0.018 | 93.9% |
| carrier_lay_leaf_rec32 | 0.018 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 58739ns | 58739ns | +1.32% |
| carrier_lay_leaf_rec16 | 57832ns | 57832ns | -0.24% |
| carrier_lay_leaf_rec20 | 58537ns | 58537ns | +0.97% |
| carrier_lay_leaf_rec24 | 57973ns | 57973ns | base |
| carrier_lay_leaf_rec32 | 58940ns | 58940ns | +1.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 56088ns | base | --- | [54195, 57044] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 56065ns | no significant difference | [-1244, +2796]ns | [54280, 59400] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec16 | 56336ns | no significant difference | [-2088, +2151]ns | [53306, 57284] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec20 | 55966ns | no significant difference | [-1682, +3558]ns | [54627, 58447] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec32 | 55706ns | no significant difference | [-381, +3436]ns | [54642, 59920] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 56903ns | -1.9% | -2.0% | -4.1% | +10.4% |
| 2 | 54638ns | -1.5% | -3.6% | +0.1% | -0.1% |
| 3 | 56025ns | +5.1% | +1.6% | +3.5% | -0.6% |
| 4 | 53753ns | +4.7% | +6.4% | +9.6% | +1.8% |
| 5 | 56150ns | -2.5% | -4.0% | -0.7% | -0.8% |
| 6 | 57184ns | +4.8% | +0.4% | -1.8% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | -0.380 | moderate- |
| carrier_lay_leaf_rec16 | -0.418 | moderate- |
| carrier_lay_leaf_rec20 | 0.197 | ok |
| carrier_lay_leaf_rec24 | -0.260 | moderate- |
| carrier_lay_leaf_rec32 | -0.146 | ok |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec16**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec20**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 113209.9ns | 56581.9ns | 200.1% | HIGH |
| carrier_lay_leaf_rec16 | 111316.0ns | 55642.2ns | 200.1% | HIGH |
| carrier_lay_leaf_rec20 | 112722.8ns | 56346.6ns | 200.1% | HIGH |
| carrier_lay_leaf_rec24 | 111578.0ns | 55775.5ns | 200.0% | HIGH |
| carrier_lay_leaf_rec32 | 113652.8ns | 56756.2ns | 200.2% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 53841.7-59400.0 ns)
  53841.7 |########################################
  54119.6 |
  54397.5 |
  54675.4 |########################################
  54953.4 |
  55231.3 |
  55509.2 |
  55787.1 |########################################
  56065.0 |########################################
  56342.9 |
  56620.8 |
  56898.8 |
  57176.7 |
  57454.6 |
  57732.5 |
  58010.4 |
  58288.3 |
  58566.3 |
  58844.2 |########################################
  59122.1 |
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 52686.2-57284.2 ns)
  52686.2 |########################################
  52916.1 |
  53146.0 |
  53375.9 |
  53605.8 |
  53835.7 |########################################
  54065.6 |
  54295.5 |
  54525.4 |
  54755.3 |
  54985.2 |
  55215.1 |
  55445.0 |
  55674.9 |########################################
  55904.8 |
  56134.7 |
  56364.6 |
  56594.5 |
  56824.4 |########################################
  57054.3 |########################################
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 54548.3-58446.7 ns)
  54548.3 |########################################
  54743.2 |
  54938.1 |
  55133.1 |
  55328.0 |
  55522.9 |
  55717.8 |####################
  55912.7 |
  56107.6 |####################
  56302.6 |
  56497.5 |
  56692.4 |
  56887.3 |
  57082.2 |
  57277.1 |
  57472.1 |
  57667.0 |
  57861.9 |####################
  58056.8 |
  58251.7 |
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 53753.3-57043.6 ns)
  53753.3 |########################################
  53917.8 |
  54082.3 |
  54246.8 |
  54411.4 |
  54575.9 |########################################
  54740.4 |
  54904.9 |
  55069.4 |
  55233.9 |
  55398.4 |
  55562.9 |
  55727.5 |
  55892.0 |########################################
  56056.5 |########################################
  56221.0 |
  56385.5 |
  56550.0 |
  56714.5 |
  56879.0 |########################################
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 54561.7-59920.2 ns)
  54561.7 |########################################
  54829.6 |
  55097.5 |
  55365.5 |
  55633.4 |########################################
  55901.3 |
  56169.2 |
  56437.2 |
  56705.1 |
  56973.0 |####################
  57240.9 |
  57508.9 |
  57776.8 |
  58044.7 |
  58312.6 |
  58580.6 |
  58848.5 |
  59116.4 |
  59384.3 |
  59652.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=200.1% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=200.0% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=199.7% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=200.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=200.3% of algo (FFI overhead may distort results)
