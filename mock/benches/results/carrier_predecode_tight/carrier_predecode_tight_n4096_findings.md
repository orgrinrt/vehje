# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_tight_regcache shows alternating (throttle bounce) (autocorr -0.71)

carrier_pre_tight_regcache's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_tight_null** at 131202.0 ns median (-11.6% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.35x (fastest 131202.0 ns, slowest 176962.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 148534ns | 147630ns | 145100ns | 147128ns | 152360ns | -2.07% |
| carrier_pre_tight_fntable | 178289ns | 179149ns | 172927ns | 177958ns | 181467ns | +17.55% |
| carrier_pre_tight_null | 132907ns | 133450ns | 129139ns | 132188ns | 135868ns | -12.37% |
| carrier_pre_tight_regcache | 142961ns | 143046ns | 139294ns | 142258ns | 145850ns | -5.74% |
| carrier_pre_tight_switch | 151674ns | 150656ns | 149360ns | 150354ns | 154810ns | base |
| carrier_pre_tight_threaded | 147963ns | 147401ns | 143677ns | 146869ns | 151746ns | -2.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 146330ns | 142952ns | 150052ns | -2.09% | 0.028 |
| carrier_pre_tight_fntable | 176091ns | 170817ns | 179216ns | +17.82% | 0.023 |
| carrier_pre_tight_null | 130665ns | 127007ns | 133542ns | -12.58% | 0.031 |
| carrier_pre_tight_regcache | 140722ns | 137150ns | 143555ns | -5.85% | 0.029 |
| carrier_pre_tight_switch | 149460ns | 147159ns | 152569ns | base | 0.027 |
| carrier_pre_tight_threaded | 145732ns | 141481ns | 149452ns | -2.49% | 0.028 |

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.028 | 87.3% |
| carrier_pre_tight_fntable | 0.023 | 71.8% |
| carrier_pre_tight_null | 0.031 | 96.8% |
| carrier_pre_tight_regcache | 0.029 | 90.2% |
| carrier_pre_tight_switch | 0.028 | 85.6% |
| carrier_pre_tight_threaded | 0.028 | 87.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 148534ns | 148534ns | -2.07% |
| carrier_pre_tight_fntable | 178289ns | 178289ns | +17.55% |
| carrier_pre_tight_null | 132907ns | 132907ns | -12.37% |
| carrier_pre_tight_regcache | 142961ns | 142961ns | -5.74% |
| carrier_pre_tight_switch | 151674ns | 151674ns | base |
| carrier_pre_tight_threaded | 147963ns | 147963ns | -2.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 148452ns | base | --- | [147358, 152569] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 145470ns | no significant difference | [-7644, +1764]ns | [143470, 150052] | no | 0.2188 | 0.2188 | 0 |
| carrier_pre_tight_fntable | 176963ns | +26950.0ns (+18.2%) | [+21085, +31858]ns | [172094, 179216] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_tight_null | 131202ns | -19318.6ns (-13.0%) | [-21196, -15871]ns | [127250, 133542] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 140793ns | -9525.4ns (-6.4%) | [-10292, -6396]ns | [137817, 143555] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_tight_threaded | 145185ns | -4039.0ns (-2.7%) | [-7120, -26]ns | [142557, 149452] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 147885ns | -2.6% | +17.2% | -9.0% | -6.4% | -2.5% |
| 2 | 154804ns | -5.1% | +13.2% | -14.4% | -6.2% | -2.9% |
| 3 | 147558ns | +3.0% | +21.8% | -13.6% | -7.1% | -1.0% |
| 4 | 147159ns | -2.2% | +21.5% | -13.7% | -3.9% | +0.9% |
| 5 | 149019ns | -0.6% | +14.6% | -12.5% | -4.7% | -5.1% |
| 6 | 150335ns | -4.9% | +18.9% | -12.2% | -6.8% | -4.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | -0.368 | moderate- |
| carrier_pre_tight_fntable | -0.308 | moderate- |
| carrier_pre_tight_null | 0.307 | moderate+ |
| carrier_pre_tight_regcache | -0.707 | HIGH- (thermal bounce) |
| carrier_pre_tight_switch | -0.332 | moderate- |
| carrier_pre_tight_threaded | -0.129 | ok |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 5/6, lost 1/6
- **carrier_pre_tight_fntable**: won 0/6, lost 6/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 6/6, lost 0/6
- **carrier_pre_tight_threaded**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 159001.8ns | 146330.5ns | 108.7% | HIGH |
| carrier_pre_tight_fntable | 186304.4ns | 176090.8ns | 105.8% | HIGH |
| carrier_pre_tight_null | 140072.5ns | 130664.6ns | 107.2% | HIGH |
| carrier_pre_tight_regcache | 150613.8ns | 140722.0ns | 107.0% | HIGH |
| carrier_pre_tight_switch | 159998.0ns | 149459.9ns | 107.1% | HIGH |
| carrier_pre_tight_threaded | 155582.7ns | 145731.6ns | 106.8% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 142951.7-150051.9 ns)
  142951.7 |########################################
  143306.7 |
  143661.7 |########################################
  144016.7 |########################################
  144371.7 |
  144726.7 |
  145081.7 |
  145436.8 |
  145791.8 |
  146146.8 |
  146501.8 |
  146856.8 |########################################
  147211.8 |
  147566.8 |
  147921.8 |########################################
  148276.8 |
  148631.8 |
  148986.8 |
  149341.8 |
  149696.8 |
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 170817.1-179216.0 ns)
  170817.1 |####################
  171237.0 |
  171657.0 |
  172076.9 |
  172496.9 |
  172916.8 |
  173336.8 |####################
  173756.7 |
  174176.7 |
  174596.6 |
  175016.6 |####################
  175436.5 |
  175856.5 |
  176276.4 |
  176696.4 |
  177116.3 |
  177536.3 |
  177956.2 |
  178376.2 |########################################
  178796.1 |
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 127007.1-133541.6 ns)
  127007.1 |########################################
  127333.8 |########################################
  127660.6 |
  127987.3 |
  128314.0 |
  128640.7 |
  128967.5 |
  129294.2 |
  129620.9 |
  129947.6 |
  130274.4 |########################################
  130601.1 |
  130927.8 |
  131254.6 |
  131581.3 |
  131908.0 |########################################
  132234.7 |
  132561.5 |########################################
  132888.2 |
  133214.9 |
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 137150.0-143555.5 ns)
  137150.0 |########################################
  137470.3 |
  137790.5 |
  138110.8 |
  138431.1 |########################################
  138751.4 |
  139071.6 |
  139391.9 |
  139712.2 |
  140032.5 |########################################
  140352.7 |
  140673.0 |
  140993.3 |
  141313.5 |########################################
  141633.8 |
  141954.1 |########################################
  142274.4 |
  142594.6 |
  142914.9 |
  143235.2 |
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 147158.8-152569.4 ns)
  147158.8 |########################################
  147429.3 |########################################
  147699.9 |########################################
  147970.4 |
  148240.9 |
  148511.4 |
  148782.0 |########################################
  149052.5 |
  149323.0 |
  149593.6 |
  149864.1 |
  150134.6 |########################################
  150405.2 |
  150675.7 |
  150946.2 |
  151216.8 |
  151487.3 |
  151757.8 |
  152028.3 |
  152298.9 |
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 141481.2-149452.5 ns)
  141481.2 |########################################
  141879.8 |
  142278.3 |
  142676.9 |
  143075.5 |
  143474.0 |########################################
  143872.6 |########################################
  144271.2 |
  144669.7 |
  145068.3 |
  145466.9 |
  145865.4 |########################################
  146264.0 |
  146662.5 |
  147061.1 |
  147459.7 |
  147858.2 |
  148256.8 |########################################
  148655.4 |
  149053.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=108.8% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=105.5% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=107.3% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=107.0% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=107.0% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=106.9% of algo (FFI overhead may distort results)
