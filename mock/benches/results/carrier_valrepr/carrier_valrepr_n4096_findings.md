# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_vr_static is fastest but the noisiest (CV 9.9%)

carrier_vr_static wins on median (143.13 us) yet has the highest variance (CV 9.9%), while carrier_vr_tagged is the steadiest (CV 2.6%, 144.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (carrier_vr_static, carrier_vr_tagged) are a dead heat (<1%)

carrier_vr_static (143.13 us) and carrier_vr_tagged (144.27 us) differ by 0.80%, inside the noise, even though the wider field spreads 4.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5.67 us) is smaller than the fastest variant's own run-to-run std-dev (14.22 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (carrier_vr_static)

The baseline carrier_vr_static is the fastest (143.13 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader carrier_vr_static vs stability leader carrier_vr_tagged (+1% speed for 3.8x steadier)

carrier_vr_static is fastest (143.13 us, CV 9.9%); carrier_vr_tagged gives up 0.8% median for 3.8x lower variance (CV 2.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 4.0% of the fastest

All 3 variants sit between 143.13 us and 148.80 us - a 4.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_vr_static) is the fastest** at 143127.3 ns median
- Spread: 1.04x (fastest 143127.3 ns, slowest 148798.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 151450ns | 151334ns | 136781ns | 147263ns | 165064ns | -1.26% |
| carrier_vr_static | 153380ns | 145626ns | 141210ns | 144156ns | 173302ns | base |
| carrier_vr_tagged | 147308ns | 146953ns | 142327ns | 146100ns | 151611ns | -3.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 148874ns | 134390ns | 162334ns | -1.33% | 0.028 |
| carrier_vr_static | 150878ns | 138887ns | 170612ns | base | 0.027 |
| carrier_vr_tagged | 144729ns | 139988ns | 149161ns | -4.08% | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vr_nanbox | 923023 | 4905110 | 0.188 | 0.99× |
| carrier_vr_static | 934516 | 3536397 | 0.264 | 1.00× |
| carrier_vr_tagged | 899403 | 5175390 | 0.174 | 0.96× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_vr_nanbox; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.028 | 90.3% |
| carrier_vr_static | 0.029 | 93.9% |
| carrier_vr_tagged | 0.028 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 151450ns | 151450ns | -1.26% |
| carrier_vr_static | 153380ns | 153380ns | base |
| carrier_vr_tagged | 147308ns | 147308ns | -3.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 143127ns | base | --- | [138893, 170612] | --- | --- | --- | --- |
| carrier_vr_nanbox | 148798ns | no significant difference | [-19158, +17997]ns | [135490, 162334] | no | 1.0000 | 0.6875 | 0 |
| carrier_vr_tagged | 144266ns | no significant difference | [-24247, +5996]ns | [140760, 149161] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 167149ns | -14.8% | -12.0% |
| 2 | 141794ns | -5.2% | +6.6% |
| 3 | 138900ns | +18.2% | +1.9% |
| 4 | 174075ns | -7.8% | -16.4% |
| 5 | 144460ns | +7.4% | -1.0% |
| 6 | 138887ns | -1.7% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | 0.057 | ok |
| carrier_vr_static | -0.320 | moderate- |
| carrier_vr_tagged | -0.008 | ok |

**Consistency summary:**

- **carrier_vr_nanbox**: won 4/6, lost 2/6
- **carrier_vr_tagged**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 151642.4ns | 148874.0ns | 101.9% | HIGH |
| carrier_vr_static | 150919.2ns | 150877.5ns | 100.0% | HIGH |
| carrier_vr_tagged | 146188.2ns | 144729.0ns | 101.0% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 134389.6-162333.5 ns)
  134389.6 |########################################
  135786.8 |########################################
  137184.0 |
  138581.2 |
  139978.4 |
  141375.6 |########################################
  142772.8 |
  144170.0 |
  145567.2 |
  146964.4 |
  148361.6 |
  149758.8 |
  151156.0 |
  152553.2 |
  153950.4 |########################################
  155347.6 |
  156744.8 |
  158142.0 |
  159539.2 |########################################
  160936.4 |
  (0 below, 1 above range)

carrier_vr_static (n=6, range 138886.7-170611.9 ns)
  138886.7 |########################################
  140473.0 |####################
  142059.2 |
  143645.5 |####################
  145231.7 |
  146818.0 |
  148404.3 |
  149990.5 |
  151576.8 |
  153163.0 |
  154749.3 |
  156335.6 |
  157921.8 |
  159508.1 |
  161094.3 |
  162680.6 |
  164266.9 |
  165853.1 |####################
  167439.4 |
  169025.6 |
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 139987.5-149160.9 ns)
  139987.5 |########################################
  140446.2 |
  140904.8 |
  141363.5 |########################################
  141822.2 |
  142280.8 |
  142739.5 |########################################
  143198.2 |
  143656.8 |
  144115.5 |
  144574.2 |
  145032.8 |
  145491.5 |########################################
  145950.2 |
  146408.8 |
  146867.5 |########################################
  147326.2 |
  147784.8 |
  148243.5 |
  148702.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: bridge=105.3% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: bridge=101.8% of algo (FFI overhead may distort results)
