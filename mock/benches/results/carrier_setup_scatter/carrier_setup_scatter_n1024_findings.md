# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_scatter_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_scatter_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_scatter_optall shows alternating (throttle bounce) (autocorr -0.53)

carrier_setup_scatter_optall's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_scatter_parse)

The baseline carrier_setup_scatter_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_emitdirect, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitcopypatch} vs {carrier_setup_scatter_optall} (775% apart)

The field splits into a fast tier {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_emitdirect, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitcopypatch} and a slow tier {carrier_setup_scatter_optall} with a 775% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 2.56 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_scatter_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 294093ns | 294467ns | 291610ns | 294307ns | 295014ns | +13047.16% |
| carrier_setup_scatter_emitdirect | 72009ns | 72009ns | 66462ns | 71245ns | 75930ns | +3119.11% |
| carrier_setup_scatter_optall | 2556793ns | 2560167ns | 2489202ns | 2539930ns | 2615884ns | +114199.04% |
| carrier_setup_scatter_parse | 2237ns | 2212ns | 2103ns | 2178ns | 2391ns | base |
| carrier_setup_scatter_predecode | 37396ns | 37121ns | 36405ns | 36958ns | 38549ns | +1571.75% |
| carrier_setup_scatter_stackcompile | 83875ns | 83960ns | 81971ns | 83534ns | 85340ns | +3649.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 291878ns | 289458ns | 292809ns | +0.00% | 0.004 |
| carrier_setup_scatter_emitdirect | 69782ns | 64247ns | 73741ns | +0.00% | 0.015 |
| carrier_setup_scatter_optall | 2553996ns | 2486580ns | 2613148ns | +0.00% | 0.000 |
| carrier_setup_scatter_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_scatter_predecode | 35191ns | 34268ns | 36249ns | +0.00% | 0.029 |
| carrier_setup_scatter_stackcompile | 81639ns | 79808ns | 83037ns | +0.00% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 1826553 | 5857677 | 0.312 | 127.47× |
| carrier_setup_scatter_emitdirect | 642621 | 3486082 | 0.184 | 44.85× |
| carrier_setup_scatter_optall | 15927351 | 72484712 | 0.220 | 1111.51× |
| carrier_setup_scatter_parse | 14330 | 52940 | 0.271 | 1.00× |
| carrier_setup_scatter_predecode | 438076 | 3184292 | 0.138 | 30.57× |
| carrier_setup_scatter_stackcompile | 718253 | 4384106 | 0.164 | 50.12× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_setup_scatter_predecode; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_scatter_emitcopypatch | 0.004 | 11.7% |
| carrier_setup_scatter_emitdirect | 0.015 | 49.1% |
| carrier_setup_scatter_optall | 0.000 | 1.3% |
| carrier_setup_scatter_predecode | 0.029 | 98.1% |
| carrier_setup_scatter_stackcompile | 0.013 | 41.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 294093ns | 294093ns | +13047.16% |
| carrier_setup_scatter_emitdirect | 72009ns | 72009ns | +3119.11% |
| carrier_setup_scatter_optall | 2556793ns | 2556793ns | +114199.04% |
| carrier_setup_scatter_parse | 2237ns | 2237ns | base |
| carrier_setup_scatter_predecode | 37396ns | 37396ns | +1571.75% |
| carrier_setup_scatter_stackcompile | 83875ns | 83875ns | +3649.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_scatter_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_scatter_emitcopypatch | 292207ns | +292207.1ns (+0.0%) | [+290620, +292809]ns | [290620, 292809] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_emitdirect | 69798ns | +69798.3ns (+0.0%) | [+65806, +73741]ns | [65806, 73741] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_optall | 2557292ns | +2557292.0ns (+0.0%) | [+2491547, +2613148]ns | [2491547, 2613148] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_predecode | 34944ns | +34943.8ns (+0.0%) | [+34381, +36249]ns | [34381, 36249] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_stackcompile | 81728ns | +81727.5ns (+0.0%) | [+80152, +83037]ns | [80152, 83037] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_scatter_parse | carrier_setup_scatter_emitcopypatch | carrier_setup_scatter_emitdirect | carrier_setup_scatter_optall | carrier_setup_scatter_predecode | carrier_setup_scatter_stackcompile |
|---|---|---|---|---|---|---|
| 1 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 2 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 3 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 4 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 5 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 6 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_setup_scatter_emitcopypatch | -0.131 | ok |
| carrier_setup_scatter_emitdirect | -0.261 | moderate- |
| carrier_setup_scatter_optall | -0.535 | HIGH- (thermal bounce) |
| carrier_setup_scatter_parse | 0.000 | ok |
| carrier_setup_scatter_predecode | -0.403 | moderate- |
| carrier_setup_scatter_stackcompile | -0.170 | ok |

**Consistency summary:**

- **carrier_setup_scatter_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_scatter_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_scatter_optall**: won 0/6, lost 0/6
- **carrier_setup_scatter_predecode**: won 0/6, lost 0/6
- **carrier_setup_scatter_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 292514.6ns | 291878.5ns | 100.2% | HIGH |
| carrier_setup_scatter_emitdirect | 138171.9ns | 69781.6ns | 198.0% | HIGH |
| carrier_setup_scatter_optall | 2549077.8ns | 2553995.8ns | 99.8% | HIGH |
| carrier_setup_scatter_parse | 1525.5ns | 0.0ns | 0.0% |  |
| carrier_setup_scatter_predecode | 105212.6ns | 35191.2ns | 299.0% | HIGH |
| carrier_setup_scatter_stackcompile | 150464.9ns | 81638.9ns | 184.3% | HIGH |

## Distribution (algo ns)

```
carrier_setup_scatter_emitcopypatch (n=6, range 289457.5-292808.8 ns)
  289457.5 |########################################
  289625.1 |
  289792.6 |
  289960.2 |
  290127.8 |
  290295.3 |
  290462.9 |
  290630.4 |
  290798.0 |
  290965.6 |
  291133.1 |
  291300.7 |
  291468.2 |
  291635.8 |########################################
  291803.4 |########################################
  291970.9 |
  292138.5 |
  292306.1 |
  292473.6 |########################################
  292641.2 |########################################
  (0 below, 1 above range)

carrier_setup_scatter_emitdirect (n=6, range 64247.1-73740.9 ns)
  64247.1 |####################
  64721.8 |
  65196.5 |
  65671.2 |
  66145.9 |
  66620.5 |
  67095.2 |####################
  67569.9 |####################
  68044.6 |
  68519.3 |
  68994.0 |
  69468.7 |
  69943.4 |
  70418.0 |
  70892.7 |
  71367.4 |
  71842.1 |########################################
  72316.8 |
  72791.5 |
  73266.2 |
  (0 below, 1 above range)

carrier_setup_scatter_optall (n=6, range 2486580.0-2613148.0 ns)
  2486580.0 |########################################
  2492908.4 |########################################
  2499236.8 |
  2505565.2 |########################################
  2511893.6 |
  2518222.0 |
  2524550.4 |
  2530878.8 |
  2537207.2 |
  2543535.6 |
  2549864.0 |
  2556192.4 |
  2562520.8 |
  2568849.2 |
  2575177.6 |
  2581506.0 |
  2587834.4 |
  2594162.8 |
  2600491.2 |########################################
  2606819.6 |########################################
  (0 below, 1 above range)

carrier_setup_scatter_predecode (n=6, range 34267.5-36248.8 ns)
  34267.5 |####################
  34366.6 |
  34465.6 |########################################
  34564.7 |
  34663.8 |
  34762.8 |
  34861.9 |
  34960.9 |
  35060.0 |
  35159.1 |
  35258.1 |
  35357.2 |####################
  35456.2 |
  35555.3 |
  35654.4 |
  35753.4 |####################
  35852.5 |
  35951.6 |
  36050.6 |
  36149.7 |
  (0 below, 1 above range)

carrier_setup_scatter_stackcompile (n=6, range 79807.5-83037.1 ns)
  79807.5 |########################################
  79969.0 |
  80130.5 |
  80291.9 |
  80453.4 |########################################
  80614.9 |
  80776.4 |
  80937.8 |########################################
  81099.3 |
  81260.8 |
  81422.3 |
  81583.8 |
  81745.2 |
  81906.7 |
  82068.2 |
  82229.7 |
  82391.1 |########################################
  82552.6 |
  82714.1 |########################################
  82875.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_scatter_emitcopypatch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_emitdirect**: bridge=200.8% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_optall**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_predecode**: bridge=298.6% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_stackcompile**: bridge=186.4% of algo (FFI overhead may distort results)
