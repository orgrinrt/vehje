# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_pre_tight_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_pre_tight_switch has the worst median (41.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_pre_tight_null at 34.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_pre_tight_threaded shows alternating (throttle bounce) (autocorr -0.51)

carrier_pre_tight_threaded's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_tight_null** at 34163.8 ns median (-18.5% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 1.23x (fastest 34163.8 ns, slowest 41900.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 41266ns | 41474ns | 40152ns | 41426ns | 41583ns | -6.99% |
| carrier_pre_tight_fntable | 43248ns | 43610ns | 40640ns | 43388ns | 44342ns | -2.52% |
| carrier_pre_tight_null | 36379ns | 36454ns | 35510ns | 36413ns | 36764ns | -18.00% |
| carrier_pre_tight_regcache | 37663ns | 37949ns | 35625ns | 37772ns | 38519ns | -15.11% |
| carrier_pre_tight_switch | 44367ns | 44198ns | 43950ns | 44156ns | 44891ns | base |
| carrier_pre_tight_threaded | 41709ns | 41765ns | 41284ns | 41640ns | 42026ns | -5.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 38982ns | 37950ns | 39283ns | -7.30% | 0.026 |
| carrier_pre_tight_fntable | 40972ns | 38463ns | 42007ns | -2.57% | 0.025 |
| carrier_pre_tight_null | 34064ns | 33284ns | 34429ns | -19.00% | 0.030 |
| carrier_pre_tight_regcache | 35378ns | 33440ns | 36161ns | -15.87% | 0.029 |
| carrier_pre_tight_switch | 42053ns | 41659ns | 42568ns | base | 0.024 |
| carrier_pre_tight_threaded | 39407ns | 39030ns | 39716ns | -6.29% | 0.026 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.026 | 85.0% |
| carrier_pre_tight_fntable | 0.025 | 80.5% |
| carrier_pre_tight_null | 0.030 | 97.4% |
| carrier_pre_tight_regcache | 0.029 | 93.3% |
| carrier_pre_tight_switch | 0.024 | 79.4% |
| carrier_pre_tight_threaded | 0.026 | 84.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 41266ns | 41266ns | -6.99% |
| carrier_pre_tight_fntable | 43248ns | 43248ns | -2.52% |
| carrier_pre_tight_null | 36379ns | 36379ns | -18.00% |
| carrier_pre_tight_regcache | 37663ns | 37663ns | -15.11% |
| carrier_pre_tight_switch | 44367ns | 44367ns | base |
| carrier_pre_tight_threaded | 41709ns | 41709ns | -5.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 41901ns | base | --- | [41690, 42568] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 39179ns | -2752.7ns (-6.6%) | [-4008, -2452]ns | [38485, 39283] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_fntable | 41338ns | -785.8ns (-1.9%) | [-2387, -71]ns | [39571, 42007] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_null | 34164ns | -7682.9ns (-18.3%) | [-8881, -7403]ns | [33600, 34429] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 35674ns | -6353.8ns (-15.2%) | [-7969, -5702]ns | [34300, 36161] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_threaded | 39457ns | -2633.1ns (-6.3%) | [-2939, -2365]ns | [39048, 39716] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 42258ns | -7.3% | -9.0% | -21.2% | -14.0% | -6.3% |
| 2 | 41860ns | -6.2% | -0.1% | -18.2% | -20.1% | -6.8% |
| 3 | 41942ns | -7.0% | -2.1% | -17.5% | -16.2% | -6.2% |
| 4 | 41722ns | -6.1% | -0.2% | -17.9% | -13.7% | -5.1% |
| 5 | 41659ns | -5.7% | -2.4% | -18.6% | -13.6% | -6.2% |
| 6 | 42879ns | -11.5% | -1.6% | -20.5% | -17.5% | -7.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | -0.136 | ok |
| carrier_pre_tight_fntable | -0.281 | moderate- |
| carrier_pre_tight_null | 0.030 | ok |
| carrier_pre_tight_regcache | -0.219 | moderate- |
| carrier_pre_tight_switch | -0.170 | ok |
| carrier_pre_tight_threaded | -0.512 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 6/6, lost 0/6
- **carrier_pre_tight_fntable**: won 6/6, lost 0/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 6/6, lost 0/6
- **carrier_pre_tight_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 120314.0ns | 38982.4ns | 308.6% | HIGH |
| carrier_pre_tight_fntable | 116326.5ns | 40971.7ns | 283.9% | HIGH |
| carrier_pre_tight_null | 104957.3ns | 34064.2ns | 308.1% | HIGH |
| carrier_pre_tight_regcache | 108811.0ns | 35378.3ns | 307.6% | HIGH |
| carrier_pre_tight_switch | 115482.6ns | 42053.1ns | 274.6% | HIGH |
| carrier_pre_tight_threaded | 120956.0ns | 39407.1ns | 306.9% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 37950.0-39282.7 ns)
  37950.0 |####################
  38016.6 |
  38083.3 |
  38149.9 |
  38216.5 |
  38283.2 |
  38349.8 |
  38416.4 |
  38483.1 |
  38549.7 |
  38616.3 |
  38683.0 |
  38749.6 |
  38816.3 |
  38882.9 |
  38949.5 |
  39016.2 |####################
  39082.8 |
  39149.4 |########################################
  39216.1 |####################
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 38463.3-42006.7 ns)
  38463.3 |########################################
  38640.5 |
  38817.6 |
  38994.8 |
  39172.0 |
  39349.1 |
  39526.3 |
  39703.5 |
  39880.6 |
  40057.8 |
  40235.0 |
  40412.1 |
  40589.3 |########################################
  40766.5 |
  40943.6 |########################################
  41120.8 |
  41298.0 |
  41475.1 |########################################
  41652.3 |########################################
  41829.5 |
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 33284.2-34429.0 ns)
  33284.2 |########################################
  33341.4 |
  33398.7 |
  33455.9 |
  33513.2 |
  33570.4 |
  33627.6 |
  33684.9 |
  33742.1 |
  33799.4 |
  33856.6 |
  33913.8 |########################################
  33971.1 |
  34028.3 |
  34085.6 |########################################
  34142.8 |
  34200.0 |########################################
  34257.3 |########################################
  34314.5 |
  34371.8 |
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 33440.4-36161.1 ns)
  33440.4 |####################
  33576.4 |
  33712.5 |
  33848.5 |
  33984.5 |
  34120.6 |
  34256.6 |
  34392.6 |
  34528.7 |
  34664.7 |
  34800.7 |
  34936.8 |
  35072.8 |####################
  35208.8 |
  35344.9 |####################
  35480.9 |
  35616.9 |
  35753.0 |
  35889.0 |########################################
  36025.0 |
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 41658.7-42568.3 ns)
  41658.7 |########################################
  41704.2 |########################################
  41749.7 |
  41795.1 |
  41840.6 |########################################
  41886.1 |
  41931.6 |########################################
  41977.1 |
  42022.6 |
  42068.0 |
  42113.5 |
  42159.0 |
  42204.5 |
  42250.0 |########################################
  42295.5 |
  42340.9 |
  42386.4 |
  42431.9 |
  42477.4 |
  42522.9 |
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 39030.0-39716.4 ns)
  39030.0 |####################
  39064.3 |####################
  39098.6 |
  39133.0 |
  39167.3 |
  39201.6 |
  39235.9 |
  39270.3 |
  39304.6 |####################
  39338.9 |
  39373.2 |
  39407.5 |
  39441.9 |
  39476.2 |
  39510.5 |
  39544.8 |
  39579.2 |########################################
  39613.5 |
  39647.8 |
  39682.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=308.1% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=285.2% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=308.1% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=307.5% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=275.8% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=307.1% of algo (FFI overhead may distort results)
