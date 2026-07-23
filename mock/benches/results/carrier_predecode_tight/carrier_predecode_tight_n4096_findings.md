# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: carrier_pre_tight_null** at 131398.8 ns median (-6.8% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.36x (fastest 131398.8 ns, slowest 178128.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 150807ns | 150725ns | 150302ns | 150714ns | 151201ns | +4.96% |
| carrier_pre_tight_fntable | 180693ns | 180337ns | 179175ns | 180064ns | 182394ns | +25.76% |
| carrier_pre_tight_null | 133323ns | 133608ns | 129489ns | 133296ns | 135280ns | -7.21% |
| carrier_pre_tight_regcache | 143120ns | 143188ns | 141676ns | 142704ns | 144467ns | -0.39% |
| carrier_pre_tight_switch | 143675ns | 143196ns | 143009ns | 143176ns | 144758ns | base |
| carrier_pre_tight_threaded | 146900ns | 147322ns | 143791ns | 146149ns | 149581ns | +2.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 148596ns | 148123ns | 148988ns | +5.05% | 0.028 |
| carrier_pre_tight_fntable | 178453ns | 176875ns | 180119ns | +26.15% | 0.023 |
| carrier_pre_tight_null | 131065ns | 127359ns | 132880ns | -7.35% | 0.031 |
| carrier_pre_tight_regcache | 140904ns | 139464ns | 142227ns | -0.39% | 0.029 |
| carrier_pre_tight_switch | 141460ns | 140807ns | 142498ns | base | 0.029 |
| carrier_pre_tight_threaded | 144661ns | 141618ns | 147242ns | +2.26% | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_tight_direct | 963853 | 2253991 | 0.428 | 1.02× |
| carrier_pre_tight_fntable | 1146096 | 3370381 | 0.340 | 1.22× |
| carrier_pre_tight_null | 842292 | 3092998 | 0.272 | 0.89× |
| carrier_pre_tight_regcache | 903665 | 3987802 | 0.227 | 0.96× |
| carrier_pre_tight_switch | 941204 | 2904793 | 0.324 | 1.00× |
| carrier_pre_tight_threaded | 927858 | 2974524 | 0.312 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.028 | 85.7% |
| carrier_pre_tight_fntable | 0.023 | 71.5% |
| carrier_pre_tight_null | 0.031 | 96.9% |
| carrier_pre_tight_regcache | 0.029 | 90.3% |
| carrier_pre_tight_switch | 0.029 | 90.3% |
| carrier_pre_tight_threaded | 0.028 | 87.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 150807ns | 150807ns | +4.96% |
| carrier_pre_tight_fntable | 180693ns | 180693ns | +25.76% |
| carrier_pre_tight_null | 133323ns | 133323ns | -7.21% |
| carrier_pre_tight_regcache | 143120ns | 143120ns | -0.39% |
| carrier_pre_tight_switch | 143675ns | 143675ns | base |
| carrier_pre_tight_threaded | 146900ns | 146900ns | +2.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 141002ns | base | --- | [140879, 142498] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 148525ns | +7460.2ns (+5.3%) | [+5946, +8004]ns | [148276, 148988] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_tight_fntable | 178129ns | +37063.1ns (+26.3%) | [+34718, +39198]ns | [177111, 180119] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_tight_null | 131399ns | -9698.5ns (-6.9%) | [-13476, -8009]ns | [128916, 132880] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 141011ns | no significant difference | [-2087, +1266]ns | [139474, 142227] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_tight_threaded | 145106ns | +2823.6ns (+2.0%) | [+460, +6321]ns | [141635, 147242] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 140950ns | +5.1% | +25.5% | -9.6% | +1.3% | +0.5% |
| 2 | 143835ns | +3.3% | +23.3% | -9.3% | -1.7% | +0.2% |
| 3 | 141161ns | +5.5% | +26.1% | -7.1% | -1.2% | +3.5% |
| 4 | 141034ns | +5.3% | +28.5% | -6.6% | -1.1% | +5.1% |
| 5 | 140970ns | +5.3% | +26.4% | -6.4% | +0.5% | +0.5% |
| 6 | 140807ns | +5.9% | +27.2% | -5.0% | -0.1% | +3.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | -0.147 | ok |
| carrier_pre_tight_fntable | 0.025 | ok |
| carrier_pre_tight_null | 0.228 | moderate+ |
| carrier_pre_tight_regcache | 0.123 | ok |
| carrier_pre_tight_switch | -0.185 | ok |
| carrier_pre_tight_threaded | -0.262 | moderate- |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 0/6, lost 6/6
- **carrier_pre_tight_fntable**: won 0/6, lost 6/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 4/6, lost 2/6
- **carrier_pre_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 161191.5ns | 148596.2ns | 108.5% | HIGH |
| carrier_pre_tight_fntable | 188028.5ns | 178452.8ns | 105.4% | HIGH |
| carrier_pre_tight_null | 140821.9ns | 131065.0ns | 107.4% | HIGH |
| carrier_pre_tight_regcache | 150499.8ns | 140903.8ns | 106.8% | HIGH |
| carrier_pre_tight_switch | 162332.3ns | 141459.6ns | 114.8% | HIGH |
| carrier_pre_tight_threaded | 154281.2ns | 144661.1ns | 106.7% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 148123.3-148987.5 ns)
  148123.3 |########################################
  148166.5 |
  148209.7 |
  148252.9 |
  148296.1 |
  148339.3 |
  148382.6 |
  148425.8 |########################################
  148469.0 |########################################
  148512.2 |########################################
  148555.4 |
  148598.6 |
  148641.8 |
  148685.0 |
  148728.2 |
  148771.5 |
  148814.7 |
  148857.9 |########################################
  148901.1 |
  148944.3 |
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 176875.4-180118.8 ns)
  176875.4 |########################################
  177037.6 |
  177199.7 |########################################
  177361.9 |
  177524.1 |
  177686.2 |
  177848.4 |
  178010.6 |########################################
  178172.7 |########################################
  178334.9 |
  178497.1 |
  178659.2 |
  178821.4 |
  178983.6 |########################################
  179145.7 |
  179307.9 |
  179470.1 |
  179632.2 |
  179794.4 |
  179956.6 |
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 127358.8-132880.0 ns)
  127358.8 |########################################
  127634.9 |
  127910.9 |
  128187.0 |
  128463.0 |
  128739.1 |
  129015.2 |
  129291.2 |
  129567.3 |
  129843.3 |
  130119.4 |
  130395.5 |########################################
  130671.5 |
  130947.6 |########################################
  131223.6 |
  131499.7 |########################################
  131775.8 |########################################
  132051.8 |
  132327.9 |
  132603.9 |
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 139463.8-142226.9 ns)
  139463.8 |########################################
  139602.0 |
  139740.1 |
  139878.3 |
  140016.4 |
  140154.6 |
  140292.7 |
  140430.9 |
  140569.0 |####################
  140707.2 |
  140845.3 |
  140983.5 |
  141121.7 |
  141259.8 |####################
  141398.0 |
  141536.1 |####################
  141674.3 |
  141812.4 |
  141950.6 |
  142088.7 |
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 140807.1-142497.9 ns)
  140807.1 |####################
  140891.6 |########################################
  140976.2 |####################
  141060.7 |
  141145.3 |####################
  141229.8 |
  141314.3 |
  141398.9 |
  141483.4 |
  141568.0 |
  141652.5 |
  141737.0 |
  141821.6 |
  141906.1 |
  141990.7 |
  142075.2 |
  142159.7 |
  142244.3 |
  142328.8 |
  142413.4 |
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 141617.9-147241.9 ns)
  141617.9 |########################################
  141899.1 |
  142180.3 |
  142461.5 |
  142742.7 |
  143023.9 |
  143305.1 |
  143586.3 |
  143867.5 |####################
  144148.7 |
  144429.9 |
  144711.1 |
  144992.3 |
  145273.5 |
  145554.7 |
  145835.9 |
  146117.1 |########################################
  146398.3 |
  146679.5 |
  146960.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=108.6% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=105.5% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=107.3% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=106.6% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=114.9% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=106.5% of algo (FFI overhead may distort results)
