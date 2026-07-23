# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_tight_null dominates: 15% faster than the next best (carrier_pre_tight_direct)

carrier_pre_tight_null (521.40 us) leads carrier_pre_tight_direct (597.77 us) by 15%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_tight_null beats baseline by 20% (significant)

carrier_pre_tight_null is -130.78 us (20%) faster than baseline carrier_pre_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_tight_regcache shows alternating (throttle bounce) (autocorr -0.59)

carrier_pre_tight_regcache's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_tight_null** at 521399.8 ns median (-20.2% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.68x (fastest 521399.8 ns, slowest 873562.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 601697ns | 600184ns | 596708ns | 599676ns | 607222ns | -8.10% |
| carrier_pre_tight_fntable | 806752ns | 804316ns | 803420ns | 804093ns | 812406ns | +23.22% |
| carrier_pre_tight_null | 524366ns | 523630ns | 521210ns | 523508ns | 527231ns | -19.91% |
| carrier_pre_tight_regcache | 870633ns | 875829ns | 827448ns | 873210ns | 888361ns | +32.98% |
| carrier_pre_tight_switch | 654703ns | 655600ns | 651305ns | 654539ns | 656648ns | base |
| carrier_pre_tight_threaded | 603724ns | 603806ns | 601640ns | 603322ns | 605369ns | -7.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 599284ns | 594460ns | 604665ns | -8.13% | 0.027 |
| carrier_pre_tight_fntable | 804436ns | 801148ns | 809939ns | +23.32% | 0.020 |
| carrier_pre_tight_null | 522117ns | 519032ns | 524898ns | -19.96% | 0.031 |
| carrier_pre_tight_regcache | 868248ns | 825211ns | 885862ns | +33.10% | 0.019 |
| carrier_pre_tight_switch | 652338ns | 649038ns | 654231ns | base | 0.025 |
| carrier_pre_tight_threaded | 601417ns | 599433ns | 603018ns | -7.81% | 0.027 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_tight_direct | 3901351 | 8947419 | 0.436 | 0.92× |
| carrier_pre_tight_fntable | 5168568 | 13411178 | 0.385 | 1.22× |
| carrier_pre_tight_null | 3400071 | 12308042 | 0.276 | 0.80× |
| carrier_pre_tight_regcache | 5543829 | 15890356 | 0.349 | 1.31× |
| carrier_pre_tight_switch | 4240101 | 11550696 | 0.367 | 1.00× |
| carrier_pre_tight_threaded | 3896561 | 11828516 | 0.329 | 0.92× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.027 | 86.8% |
| carrier_pre_tight_fntable | 0.020 | 64.7% |
| carrier_pre_tight_null | 0.031 | 99.5% |
| carrier_pre_tight_regcache | 0.019 | 59.4% |
| carrier_pre_tight_switch | 0.025 | 79.5% |
| carrier_pre_tight_threaded | 0.027 | 86.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 601697ns | 601697ns | -8.10% |
| carrier_pre_tight_fntable | 806752ns | 806752ns | +23.22% |
| carrier_pre_tight_null | 524366ns | 524366ns | -19.91% |
| carrier_pre_tight_regcache | 870633ns | 870633ns | +32.98% |
| carrier_pre_tight_switch | 654703ns | 654703ns | base |
| carrier_pre_tight_threaded | 603724ns | 603724ns | -7.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 653163ns | base | --- | [649621, 654231] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 597766ns | -53459.8ns (-8.2%) | [-58573, -47130]ns | [595421, 604665] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_fntable | 802064ns | +152443.1ns (+23.3%) | [+148142, +155709]ns | [801305, 809939] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_null | 521400ns | -130783.5ns (-20.0%) | [-133235, -126646]ns | [520053, 524898] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 873562ns | +220274.1ns (+33.7%) | [+192806, +234650]ns | [845321, 885862] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_threaded | 601487ns | -50634.8ns (-7.8%) | [-53776, -48353]ns | [599745, 603018] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 652940ns | -8.3% | +22.7% | -20.5% | +34.6% | -7.7% |
| 2 | 653636ns | -9.1% | +23.8% | -20.3% | +32.8% | -7.7% |
| 3 | 653387ns | -6.7% | +22.7% | -20.2% | +34.7% | -8.1% |
| 4 | 650203ns | -7.8% | +23.4% | -19.8% | +26.9% | -7.8% |
| 5 | 649038ns | -8.1% | +23.5% | -19.2% | +37.4% | -7.2% |
| 6 | 654826ns | -8.9% | +23.8% | -19.8% | +32.2% | -8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | -0.264 | moderate- |
| carrier_pre_tight_fntable | -0.368 | moderate- |
| carrier_pre_tight_null | 0.376 | moderate+ |
| carrier_pre_tight_regcache | -0.594 | HIGH- (thermal bounce) |
| carrier_pre_tight_switch | -0.051 | ok |
| carrier_pre_tight_threaded | -0.034 | ok |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 6/6, lost 0/6
- **carrier_pre_tight_fntable**: won 0/6, lost 6/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 0/6, lost 6/6
- **carrier_pre_tight_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 651615.7ns | 599284.2ns | 108.7% | HIGH |
| carrier_pre_tight_fntable | 843879.5ns | 804436.1ns | 104.9% | HIGH |
| carrier_pre_tight_null | 560745.1ns | 522116.8ns | 107.4% | HIGH |
| carrier_pre_tight_regcache | 905582.6ns | 868248.3ns | 104.3% | HIGH |
| carrier_pre_tight_switch | 700273.8ns | 652338.2ns | 107.3% | HIGH |
| carrier_pre_tight_threaded | 640424.5ns | 601416.9ns | 106.5% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 594460.4-604664.8 ns)
  594460.4 |########################################
  594970.6 |
  595480.8 |
  595991.1 |########################################
  596501.3 |########################################
  597011.5 |
  597521.7 |
  598031.9 |
  598542.2 |########################################
  599052.4 |########################################
  599562.6 |
  600072.8 |
  600583.0 |
  601093.3 |
  601603.5 |
  602113.7 |
  602623.9 |
  603134.1 |
  603644.4 |
  604154.6 |
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 801147.5-809939.4 ns)
  801147.5 |########################################
  801587.1 |####################
  802026.7 |####################
  802466.3 |
  802905.9 |
  803345.5 |
  803785.1 |
  804224.6 |
  804664.2 |
  805103.8 |
  805543.4 |
  805983.0 |
  806422.6 |
  806862.2 |
  807301.8 |
  807741.4 |
  808181.0 |
  808620.6 |
  809060.2 |####################
  809499.8 |
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 519032.1-524897.7 ns)
  519032.1 |########################################
  519325.4 |
  519618.7 |
  519911.9 |
  520205.2 |
  520498.5 |
  520791.8 |########################################
  521085.1 |########################################
  521378.3 |########################################
  521671.6 |
  521964.9 |
  522258.2 |
  522551.5 |
  522844.7 |
  523138.0 |
  523431.3 |
  523724.6 |
  524017.9 |
  524311.1 |########################################
  524604.4 |
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 825211.2-885862.1 ns)
  825211.2 |########################################
  828243.7 |
  831276.3 |
  834308.8 |
  837341.4 |
  840373.9 |
  843406.5 |
  846439.0 |
  849471.6 |
  852504.1 |
  855536.6 |
  858569.2 |
  861601.7 |
  864634.3 |########################################
  867666.8 |########################################
  870699.4 |
  873731.9 |
  876764.5 |########################################
  879797.0 |########################################
  882829.6 |
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 649038.3-654230.8 ns)
  649038.3 |########################################
  649297.9 |
  649557.6 |
  649817.2 |
  650076.8 |########################################
  650336.4 |
  650596.1 |
  650855.7 |
  651115.3 |
  651374.9 |
  651634.6 |
  651894.2 |
  652153.8 |
  652413.4 |
  652673.1 |
  652932.7 |########################################
  653192.3 |########################################
  653451.9 |########################################
  653711.6 |
  653971.2 |
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 599432.9-603018.3 ns)
  599432.9 |########################################
  599612.2 |
  599791.4 |
  599970.7 |########################################
  600150.0 |
  600329.2 |
  600508.5 |########################################
  600687.8 |
  600867.1 |
  601046.3 |
  601225.6 |
  601404.9 |
  601584.1 |
  601763.4 |
  601942.7 |
  602122.0 |
  602301.2 |########################################
  602480.5 |
  602659.8 |
  602839.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=108.7% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=105.0% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=107.5% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=104.3% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=107.3% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=106.4% of algo (FFI overhead may distort results)
