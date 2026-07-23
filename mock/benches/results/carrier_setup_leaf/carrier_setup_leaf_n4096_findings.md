# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_leaf_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_leaf_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_leaf_emitdirect shows alternating (throttle bounce) (autocorr -0.57)

carrier_setup_leaf_emitdirect's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_leaf_parse)

The baseline carrier_setup_leaf_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_leaf_parse, carrier_setup_leaf_predecode, carrier_setup_leaf_emitdirect, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} vs {carrier_setup_leaf_optall} (395% apart)

The field splits into a fast tier {carrier_setup_leaf_parse, carrier_setup_leaf_predecode, carrier_setup_leaf_emitdirect, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} and a slow tier {carrier_setup_leaf_optall} with a 395% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 2.94 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_leaf_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 591290ns | 595596ns | 569896ns | 587355ns | 607889ns | +26199.81% |
| carrier_setup_leaf_emitdirect | 205138ns | 205634ns | 201769ns | 204807ns | 207318ns | +9024.27% |
| carrier_setup_leaf_optall | 2959512ns | 2940764ns | 2867130ns | 2921529ns | 3062677ns | +131535.26% |
| carrier_setup_leaf_parse | 2248ns | 2256ns | 2097ns | 2219ns | 2367ns | base |
| carrier_setup_leaf_predecode | 199652ns | 190393ns | 189781ns | 190236ns | 218710ns | +8780.25% |
| carrier_setup_leaf_stackcompile | 295163ns | 287150ns | 268787ns | 284193ns | 324805ns | +13028.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 588756ns | 567705ns | 605034ns | +0.00% | 0.007 |
| carrier_setup_leaf_emitdirect | 202879ns | 199488ns | 205007ns | +0.00% | 0.020 |
| carrier_setup_leaf_optall | 2956739ns | 2864514ns | 3059674ns | +0.00% | 0.001 |
| carrier_setup_leaf_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_leaf_predecode | 197345ns | 187601ns | 216224ns | +0.00% | 0.021 |
| carrier_setup_leaf_stackcompile | 292640ns | 266627ns | 321909ns | +0.00% | 0.014 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 3596511 | 15042068 | 0.239 | 250.26× |
| carrier_setup_leaf_emitdirect | 1261194 | 6007778 | 0.210 | 87.76× |
| carrier_setup_leaf_optall | 18219205 | 70006617 | 0.260 | 1267.75× |
| carrier_setup_leaf_parse | 14371 | 52954 | 0.271 | 1.00× |
| carrier_setup_leaf_predecode | 1197429 | 5081692 | 0.236 | 83.32× |
| carrier_setup_leaf_stackcompile | 1739421 | 7939330 | 0.219 | 121.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_setup_leaf_predecode; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_leaf_emitcopypatch | 0.007 | 31.6% |
| carrier_setup_leaf_emitdirect | 0.020 | 92.2% |
| carrier_setup_leaf_optall | 0.001 | 6.4% |
| carrier_setup_leaf_predecode | 0.022 | 99.7% |
| carrier_setup_leaf_stackcompile | 0.014 | 65.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 591290ns | 591290ns | +26199.81% |
| carrier_setup_leaf_emitdirect | 205138ns | 205138ns | +9024.27% |
| carrier_setup_leaf_optall | 2959512ns | 2959512ns | +131535.26% |
| carrier_setup_leaf_parse | 2248ns | 2248ns | base |
| carrier_setup_leaf_predecode | 199652ns | 199652ns | +8780.25% |
| carrier_setup_leaf_stackcompile | 295163ns | 295163ns | +13028.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_leaf_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_leaf_emitcopypatch | 593065ns | +593064.8ns (+0.0%) | [+568171, +605034]ns | [568171, 605034] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_emitdirect | 203421ns | +203420.6ns (+0.0%) | [+200210, +205007]ns | [200210, 205007] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_optall | 2937877ns | +2937877.1ns (+0.0%) | [+2872667, +3059674]ns | [2872667, 3059674] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_predecode | 188209ns | +188209.4ns (+0.0%) | [+187601, +216224]ns | [187601, 216224] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_stackcompile | 284800ns | +284800.4ns (+0.0%) | [+271210, +321909]ns | [271210, 321909] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_leaf_parse | carrier_setup_leaf_emitcopypatch | carrier_setup_leaf_emitdirect | carrier_setup_leaf_optall | carrier_setup_leaf_predecode | carrier_setup_leaf_stackcompile |
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
| carrier_setup_leaf_emitcopypatch | -0.401 | moderate- |
| carrier_setup_leaf_emitdirect | -0.574 | HIGH- (thermal bounce) |
| carrier_setup_leaf_optall | 0.550 | HIGH+ (drift/warm-up) |
| carrier_setup_leaf_parse | 0.000 | ok |
| carrier_setup_leaf_predecode | -0.209 | moderate- |
| carrier_setup_leaf_stackcompile | -0.167 | ok |

**Consistency summary:**

- **carrier_setup_leaf_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_leaf_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_leaf_optall**: won 0/6, lost 0/6
- **carrier_setup_leaf_predecode**: won 0/6, lost 0/6
- **carrier_setup_leaf_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 589094.6ns | 588756.5ns | 100.1% | HIGH |
| carrier_setup_leaf_emitdirect | 203171.4ns | 202879.3ns | 100.1% | HIGH |
| carrier_setup_leaf_optall | 2950569.4ns | 2956739.2ns | 99.8% | HIGH |
| carrier_setup_leaf_parse | 1544.4ns | 0.0ns | 0.0% |  |
| carrier_setup_leaf_predecode | 195317.4ns | 197344.7ns | 99.0% | HIGH |
| carrier_setup_leaf_stackcompile | 305249.6ns | 292640.0ns | 104.3% | HIGH |

## Distribution (algo ns)

```
carrier_setup_leaf_emitcopypatch (n=6, range 567705.4-605033.8 ns)
  567705.4 |########################################
  569571.8 |
  571438.2 |
  573304.7 |
  575171.1 |
  577037.5 |
  578903.9 |
  580770.3 |
  582636.7 |
  584503.2 |####################
  586369.6 |
  588236.0 |
  590102.4 |
  591968.8 |
  593835.2 |
  595701.7 |
  597568.1 |
  599434.5 |####################
  601300.9 |####################
  603167.3 |
  (0 below, 1 above range)

carrier_setup_leaf_emitdirect (n=6, range 199488.3-205006.9 ns)
  199488.3 |########################################
  199764.2 |
  200040.2 |
  200316.1 |
  200592.0 |
  200868.0 |########################################
  201143.9 |
  201419.8 |
  201695.7 |
  201971.7 |
  202247.6 |
  202523.5 |########################################
  202799.5 |
  203075.4 |
  203351.3 |
  203627.2 |
  203903.2 |
  204179.1 |########################################
  204455.0 |
  204731.0 |########################################
  (0 below, 1 above range)

carrier_setup_leaf_optall (n=6, range 2864514.2-3059673.8 ns)
  2864514.2 |########################################
  2874272.2 |########################################
  2884030.2 |########################################
  2893788.1 |
  2903546.1 |
  2913304.1 |
  2923062.1 |
  2932820.0 |
  2942578.0 |
  2952336.0 |
  2962094.0 |
  2971852.0 |
  2981609.9 |########################################
  2991367.9 |
  3001125.9 |
  3010883.9 |
  3020641.8 |########################################
  3030399.8 |
  3040157.8 |
  3049915.8 |
  (0 below, 1 above range)

carrier_setup_leaf_predecode (n=6, range 187600.8-216224.0 ns)
  187600.8 |########################################
  189032.0 |
  190463.1 |##########
  191894.3 |
  193325.4 |
  194756.6 |
  196187.8 |
  197618.9 |
  199050.1 |
  200481.2 |
  201912.4 |
  203343.6 |
  204774.7 |
  206205.9 |
  207637.0 |
  209068.2 |
  210499.4 |
  211930.5 |
  213361.7 |
  214792.8 |
  (0 below, 1 above range)

carrier_setup_leaf_stackcompile (n=6, range 266627.1-321909.3 ns)
  266627.1 |########################################
  269391.2 |
  272155.3 |
  274919.4 |########################################
  277683.5 |########################################
  280447.7 |
  283211.8 |
  285975.9 |
  288740.0 |########################################
  291504.1 |
  294268.2 |########################################
  297032.3 |
  299796.4 |
  302560.6 |
  305324.7 |
  308088.8 |
  310852.9 |
  313617.0 |
  316381.1 |
  319145.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_leaf_emitcopypatch**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_emitdirect**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_optall**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **carrier_setup_leaf_optall**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_predecode**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_stackcompile**: bridge=99.3% of algo (FFI overhead may distort results)
