# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_rec24, carrier_rec20) are a dead heat (<1%)

carrier_rec24 (45.22 us) and carrier_rec20 (45.24 us) differ by 0.05%, inside the noise, even though the wider field spreads 13.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_rec24 shows alternating (throttle bounce) (autocorr -0.57)

carrier_rec24's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_rec24)

The baseline carrier_rec24 is the fastest (45.22 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader carrier_rec24 vs stability leader carrier_rec20 (+0% speed for 4.9x steadier)

carrier_rec24 is fastest (45.22 us, CV 10.6%); carrier_rec20 gives up 0.0% median for 4.9x lower variance (CV 2.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (carrier_rec24) is the fastest** at 45217.1 ns median
- Spread: 1.13x (fastest 45217.1 ns, slowest 51168.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 50342ns | 47871ns | 47365ns | 47812ns | 55625ns | -0.55% |
| carrier_rec16 | 53839ns | 53325ns | 47050ns | 51515ns | 60719ns | +6.35% |
| carrier_rec20 | 47727ns | 47399ns | 46977ns | 47376ns | 48627ns | -5.72% |
| carrier_rec24 | 50622ns | 47377ns | 47058ns | 47331ns | 57340ns | base |
| carrier_rec32 | 50378ns | 47729ns | 47335ns | 47660ns | 55978ns | -0.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 48173ns | 45257ns | 53401ns | -0.58% | 0.021 |
| carrier_rec16 | 51669ns | 44889ns | 58545ns | +6.64% | 0.020 |
| carrier_rec20 | 45566ns | 44799ns | 46495ns | -5.96% | 0.022 |
| carrier_rec24 | 48453ns | 44912ns | 55129ns | base | 0.021 |
| carrier_rec32 | 48205ns | 45204ns | 53802ns | -0.51% | 0.021 |

## Performance model

- Peak throughput: **0.023 Gops/s** (carrier_rec20; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.022 | 98.0% |
| carrier_rec16 | 0.020 | 87.6% |
| carrier_rec20 | 0.023 | 99.0% |
| carrier_rec24 | 0.023 | 99.1% |
| carrier_rec32 | 0.022 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 50342ns | 50342ns | -0.55% |
| carrier_rec16 | 53839ns | 53839ns | +6.35% |
| carrier_rec20 | 47727ns | 47727ns | -5.72% |
| carrier_rec24 | 50622ns | 50622ns | base |
| carrier_rec32 | 50378ns | 50378ns | -0.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 45217ns | base | --- | [45015, 55129] | --- | --- | --- | --- |
| carrier_rec12 | 45736ns | no significant difference | [-9576, +8340]ns | [45384, 53401] | no | 0.9167 | 0.6875 | 0 |
| carrier_rec16 | 51169ns | no significant difference | [-4318, +12557]ns | [45292, 58545] | no | 0.4375 | 0.2188 | 0 |
| carrier_rec20 | 45238ns | no significant difference | [-8717, +250]ns | [44965, 46495] | no | 1.0000 | 1.0000 | 0 |
| carrier_rec32 | 45534ns | no significant difference | [-4100, +3039]ns | [45279, 53802] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 45225ns | +0.9% | +4.5% | -0.9% | +0.4% |
| 2 | 56318ns | -19.6% | +0.7% | -15.3% | +9.9% |
| 3 | 45117ns | +0.9% | +22.1% | +0.3% | +0.2% |
| 4 | 45209ns | +3.0% | +33.5% | +0.1% | +1.1% |
| 5 | 53939ns | -15.0% | -16.8% | -16.3% | -15.4% |
| 6 | 44912ns | +34.1% | +1.7% | +0.9% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | -0.028 | ok |
| carrier_rec16 | 0.029 | ok |
| carrier_rec20 | -0.355 | moderate- |
| carrier_rec24 | -0.571 | HIGH- (thermal bounce) |
| carrier_rec32 | -0.258 | moderate- |

**Consistency summary:**

- **carrier_rec12**: won 2/6, lost 4/6
- **carrier_rec16**: won 1/6, lost 5/6
- **carrier_rec20**: won 3/6, lost 2/6
- **carrier_rec32**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 140.7ns | 48173.4ns | 0.3% |  |
| carrier_rec16 | 140.3ns | 51668.8ns | 0.3% |  |
| carrier_rec20 | 139.7ns | 45566.2ns | 0.3% |  |
| carrier_rec24 | 141.2ns | 48453.4ns | 0.3% |  |
| carrier_rec32 | 169.0ns | 48205.1ns | 0.4% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 45257.1-53400.7 ns)
  45257.1 |########################################
  45664.3 |#############
  46071.5 |
  46478.6 |#############
  46885.8 |
  47293.0 |
  47700.2 |
  48107.3 |
  48514.5 |
  48921.7 |
  49328.9 |
  49736.1 |
  50143.2 |
  50550.4 |
  50957.6 |
  51364.8 |
  51771.9 |
  52179.1 |
  52586.3 |
  52993.5 |
  (0 below, 1 above range)

carrier_rec16 (n=6, range 44889.2-58545.2 ns)
  44889.2 |########################################
  45572.0 |########################################
  46254.8 |
  46937.6 |########################################
  47620.4 |
  48303.2 |
  48986.0 |
  49668.8 |
  50351.6 |
  51034.4 |
  51717.2 |
  52400.0 |
  53082.8 |
  53765.6 |
  54448.4 |########################################
  55131.2 |
  55814.0 |
  56496.8 |########################################
  57179.6 |
  57862.4 |
  (0 below, 1 above range)

carrier_rec20 (n=6, range 44799.2-46495.4 ns)
  44799.2 |#############
  44884.0 |
  44968.8 |
  45053.6 |#############
  45138.4 |
  45223.2 |########################################
  45308.1 |
  45392.9 |
  45477.7 |
  45562.5 |
  45647.3 |
  45732.1 |
  45816.9 |
  45901.7 |
  45986.5 |
  46071.3 |
  46156.2 |
  46241.0 |
  46325.8 |
  46410.6 |
  (0 below, 1 above range)

carrier_rec24 (n=6, range 44912.1-55128.6 ns)
  44912.1 |########################################
  45422.9 |
  45933.7 |
  46444.6 |
  46955.4 |
  47466.2 |
  47977.0 |
  48487.9 |
  48998.7 |
  49509.5 |
  50020.3 |
  50531.1 |
  51042.0 |
  51552.8 |
  52063.6 |
  52574.4 |
  53085.3 |
  53596.1 |##########
  54106.9 |
  54617.7 |
  (0 below, 1 above range)

carrier_rec32 (n=6, range 45204.2-53802.5 ns)
  45204.2 |########################################
  45634.1 |##########################
  46064.0 |
  46493.9 |
  46923.9 |
  47353.8 |
  47783.7 |
  48213.6 |
  48643.5 |
  49073.4 |
  49503.3 |
  49933.3 |
  50363.2 |
  50793.1 |
  51223.0 |
  51652.9 |
  52082.8 |
  52512.8 |
  52942.7 |
  53372.6 |
  (0 below, 1 above range)

```
