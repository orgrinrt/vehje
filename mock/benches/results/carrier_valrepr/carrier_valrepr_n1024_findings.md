# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_vr_nanbox is fastest but the noisiest (CV 17.6%)

carrier_vr_nanbox wins on median (31.89 us) yet has the highest variance (CV 17.6%), while carrier_vr_tagged is the steadiest (CV 10.5%, 34.47 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.58 us) is smaller than the fastest variant's own run-to-run std-dev (5.61 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_vr_nanbox vs stability leader carrier_vr_tagged (+8% speed for 1.7x steadier)

carrier_vr_nanbox is fastest (31.89 us, CV 17.6%); carrier_vr_tagged gives up 8.1% median for 1.7x lower variance (CV 10.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_vr_nanbox** at 31892.2 ns median (-5.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.08x (fastest 31892.2 ns, slowest 34469.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 36756ns | 34191ns | 33593ns | 34032ns | 42424ns | -3.46% |
| carrier_vr_static | 38075ns | 35897ns | 35715ns | 35851ns | 42590ns | base |
| carrier_vr_tagged | 38377ns | 36709ns | 36028ns | 36504ns | 42362ns | +0.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 34349ns | 31353ns | 39765ns | -3.73% | 0.030 |
| carrier_vr_static | 35680ns | 33486ns | 39918ns | base | 0.029 |
| carrier_vr_tagged | 36103ns | 33883ns | 39933ns | +1.19% | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vr_nanbox | 404796 | 2419046 | 0.167 | 0.96× |
| carrier_vr_static | 420533 | 1752375 | 0.240 | 1.00× |
| carrier_vr_tagged | 432611 | 2572296 | 0.168 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_vr_nanbox; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.032 | 98.3% |
| carrier_vr_static | 0.030 | 93.3% |
| carrier_vr_tagged | 0.030 | 91.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 36756ns | 36756ns | -3.46% |
| carrier_vr_static | 38075ns | 38075ns | base |
| carrier_vr_tagged | 38377ns | 38377ns | +0.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 33622ns | base | --- | [33501, 39918] | --- | --- | --- | --- |
| carrier_vr_nanbox | 31892ns | -1708.8ns (-5.1%) | [-2133, -153]ns | [31389, 39765] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| carrier_vr_tagged | 34469ns | no significant difference | [-812, +1498]ns | [33908, 39933] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 45928ns | +2.0% | -4.0% |
| 2 | 33907ns | -3.6% | +5.5% |
| 3 | 33686ns | -5.3% | +0.6% |
| 4 | 33558ns | -6.6% | +1.1% |
| 5 | 33486ns | -6.2% | +3.4% |
| 6 | 33516ns | -4.9% | +2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | 0.034 | ok |
| carrier_vr_static | -0.008 | ok |
| carrier_vr_tagged | 0.109 | ok |

**Consistency summary:**

- **carrier_vr_nanbox**: won 5/6, lost 1/6
- **carrier_vr_tagged**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 98452.3ns | 34348.7ns | 286.6% | HIGH |
| carrier_vr_static | 102928.2ns | 35680.2ns | 288.5% | HIGH |
| carrier_vr_tagged | 106566.3ns | 36103.3ns | 295.2% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 31352.9-39765.0 ns)
  31352.9 |########################################
  31773.5 |########################################
  32194.1 |
  32614.7 |####################
  33035.3 |
  33455.9 |
  33876.5 |
  34297.1 |
  34717.7 |
  35138.3 |
  35558.9 |
  35979.6 |
  36400.2 |
  36820.8 |
  37241.4 |
  37662.0 |
  38082.6 |
  38503.2 |
  38923.8 |
  39344.4 |
  (0 below, 1 above range)

carrier_vr_static (n=6, range 33485.8-39917.7 ns)
  33485.8 |########################################
  33807.4 |##########
  34129.0 |
  34450.6 |
  34772.2 |
  35093.8 |
  35415.4 |
  35737.0 |
  36058.6 |
  36380.2 |
  36701.8 |
  37023.3 |
  37344.9 |
  37666.5 |
  37988.1 |
  38309.7 |
  38631.3 |
  38952.9 |
  39274.5 |
  39596.1 |
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 33883.3-39932.8 ns)
  33883.3 |########################################
  34185.8 |####################
  34488.2 |####################
  34790.7 |
  35093.2 |
  35395.7 |
  35698.1 |####################
  36000.6 |
  36303.1 |
  36605.6 |
  36908.0 |
  37210.5 |
  37513.0 |
  37815.4 |
  38117.9 |
  38420.4 |
  38722.9 |
  39025.3 |
  39327.8 |
  39630.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: bridge=299.9% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=299.4% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: bridge=300.1% of algo (FFI overhead may distort results)
