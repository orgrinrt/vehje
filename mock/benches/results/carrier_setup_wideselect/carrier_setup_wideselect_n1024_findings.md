# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_wideselect_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_wideselect_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_wideselect_parse)

The baseline carrier_setup_wideselect_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitcopypatch} vs {carrier_setup_wideselect_optall} (688% apart)

The field splits into a fast tier {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitcopypatch} and a slow tier {carrier_setup_wideselect_optall} with a 688% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 2.48 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_wideselect_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 317358ns | 317033ns | 314821ns | 316437ns | 320008ns | +14647.10% |
| carrier_setup_wideselect_emitdirect | 71229ns | 71312ns | 65831ns | 71034ns | 74222ns | +3209.92% |
| carrier_setup_wideselect_optall | 2507573ns | 2482802ns | 2449160ns | 2475314ns | 2585167ns | +116422.90% |
| carrier_setup_wideselect_parse | 2152ns | 2145ns | 2124ns | 2140ns | 2183ns | base |
| carrier_setup_wideselect_predecode | 39384ns | 39356ns | 38965ns | 39284ns | 39744ns | +1730.13% |
| carrier_setup_wideselect_stackcompile | 83651ns | 83563ns | 81722ns | 83291ns | 85156ns | +3787.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 315131ns | 312604ns | 317746ns | +0.00% | 0.003 |
| carrier_setup_wideselect_emitdirect | 69028ns | 63683ns | 72002ns | +0.00% | 0.015 |
| carrier_setup_wideselect_optall | 2504904ns | 2446270ns | 2582632ns | +0.00% | 0.000 |
| carrier_setup_wideselect_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_wideselect_predecode | 37228ns | 36824ns | 37585ns | +0.00% | 0.028 |
| carrier_setup_wideselect_stackcompile | 81467ns | 79540ns | 82980ns | +0.00% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 1965994 | 6994714 | 0.281 | 136.95× |
| carrier_setup_wideselect_emitdirect | 648812 | 3799246 | 0.171 | 45.20× |
| carrier_setup_wideselect_optall | 15534294 | 68426128 | 0.227 | 1082.14× |
| carrier_setup_wideselect_parse | 14355 | 52917 | 0.271 | 1.00× |
| carrier_setup_wideselect_predecode | 471726 | 3391302 | 0.139 | 32.86× |
| carrier_setup_wideselect_stackcompile | 747245 | 4538128 | 0.165 | 52.05× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.028 Gops/s** (carrier_setup_wideselect_predecode; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 0.003 | 11.7% |
| carrier_setup_wideselect_emitdirect | 0.015 | 53.3% |
| carrier_setup_wideselect_optall | 0.000 | 1.5% |
| carrier_setup_wideselect_predecode | 0.028 | 99.0% |
| carrier_setup_wideselect_stackcompile | 0.013 | 45.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 317358ns | 317358ns | +14647.10% |
| carrier_setup_wideselect_emitdirect | 71229ns | 71229ns | +3209.92% |
| carrier_setup_wideselect_optall | 2507573ns | 2507573ns | +116422.90% |
| carrier_setup_wideselect_parse | 2152ns | 2152ns | base |
| carrier_setup_wideselect_predecode | 39384ns | 39384ns | +1730.13% |
| carrier_setup_wideselect_stackcompile | 83651ns | 83651ns | +3787.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_wideselect_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_wideselect_emitcopypatch | 314855ns | +314855.4ns (+0.0%) | [+312792, +317746]ns | [312792, 317746] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_emitdirect | 69084ns | +69083.9ns (+0.0%) | [+65998, +72002]ns | [65998, 72002] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_optall | 2480103ns | +2480103.0ns (+0.0%) | [+2451978, +2582632]ns | [2451978, 2582632] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_predecode | 37183ns | +37182.9ns (+0.0%) | [+36917, +37585]ns | [36917, 37585] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_stackcompile | 81357ns | +81356.9ns (+0.0%) | [+80065, +82980]ns | [80065, 82980] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_wideselect_parse | carrier_setup_wideselect_emitcopypatch | carrier_setup_wideselect_emitdirect | carrier_setup_wideselect_optall | carrier_setup_wideselect_predecode | carrier_setup_wideselect_stackcompile |
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
| carrier_setup_wideselect_emitcopypatch | -0.218 | moderate- |
| carrier_setup_wideselect_emitdirect | -0.005 | ok |
| carrier_setup_wideselect_optall | -0.015 | ok |
| carrier_setup_wideselect_parse | 0.000 | ok |
| carrier_setup_wideselect_predecode | -0.095 | ok |
| carrier_setup_wideselect_stackcompile | -0.189 | ok |

**Consistency summary:**

- **carrier_setup_wideselect_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_wideselect_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_wideselect_optall**: won 0/6, lost 0/6
- **carrier_setup_wideselect_predecode**: won 0/6, lost 0/6
- **carrier_setup_wideselect_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 315394.3ns | 315131.1ns | 100.1% | HIGH |
| carrier_setup_wideselect_emitdirect | 137902.7ns | 69027.7ns | 199.8% | HIGH |
| carrier_setup_wideselect_optall | 2505527.9ns | 2504904.2ns | 100.0% | HIGH |
| carrier_setup_wideselect_parse | 1474.9ns | 0.0ns | 0.0% |  |
| carrier_setup_wideselect_predecode | 111518.0ns | 37228.3ns | 299.6% | HIGH |
| carrier_setup_wideselect_stackcompile | 156267.8ns | 81467.2ns | 191.8% | HIGH |

## Distribution (algo ns)

```
carrier_setup_wideselect_emitcopypatch (n=6, range 312604.2-317745.8 ns)
  312604.2 |####################
  312861.3 |####################
  313118.4 |
  313375.4 |
  313632.5 |
  313889.6 |####################
  314146.7 |
  314403.8 |
  314660.8 |
  314917.9 |
  315175.0 |
  315432.1 |
  315689.2 |########################################
  315946.2 |
  316203.3 |
  316460.4 |
  316717.5 |
  316974.6 |
  317231.6 |
  317488.7 |
  (0 below, 1 above range)

carrier_setup_wideselect_emitdirect (n=6, range 63682.9-72001.5 ns)
  63682.9 |########################################
  64098.8 |
  64514.8 |
  64930.7 |
  65346.6 |
  65762.6 |
  66178.5 |
  66594.4 |
  67010.3 |
  67426.3 |
  67842.2 |
  68258.1 |########################################
  68674.1 |########################################
  69090.0 |########################################
  69505.9 |
  69921.9 |
  70337.8 |
  70753.7 |
  71169.6 |########################################
  71585.6 |
  (0 below, 1 above range)

carrier_setup_wideselect_optall (n=6, range 2446270.0-2582631.9 ns)
  2446270.0 |########################################
  2453088.1 |########################################
  2459906.2 |
  2466724.3 |
  2473542.4 |########################################
  2480360.5 |########################################
  2487178.6 |
  2493996.6 |
  2500814.7 |
  2507632.8 |
  2514450.9 |
  2521269.0 |
  2528087.1 |
  2534905.2 |
  2541723.3 |
  2548541.4 |
  2555359.5 |
  2562177.6 |
  2568995.7 |########################################
  2575813.8 |
  (0 below, 1 above range)

carrier_setup_wideselect_predecode (n=6, range 36823.8-37585.4 ns)
  36823.8 |########################################
  36861.9 |
  36900.0 |
  36938.0 |
  36976.1 |########################################
  37014.2 |########################################
  37052.3 |
  37090.4 |
  37128.4 |
  37166.5 |
  37204.6 |
  37242.7 |
  37280.8 |
  37318.8 |########################################
  37356.9 |
  37395.0 |
  37433.1 |########################################
  37471.2 |
  37509.2 |
  37547.3 |
  (0 below, 1 above range)

carrier_setup_wideselect_stackcompile (n=6, range 79540.0-82979.8 ns)
  79540.0 |########################################
  79712.0 |
  79884.0 |
  80056.0 |
  80228.0 |
  80399.9 |
  80571.9 |########################################
  80743.9 |
  80915.9 |########################################
  81087.9 |
  81259.9 |
  81431.9 |
  81603.9 |########################################
  81775.9 |
  81947.9 |
  82119.8 |
  82291.8 |
  82463.8 |
  82635.8 |########################################
  82807.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_wideselect_emitcopypatch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_emitdirect**: bridge=199.7% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_optall**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_predecode**: bridge=299.8% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_stackcompile**: bridge=191.5% of algo (FFI overhead may distort results)
