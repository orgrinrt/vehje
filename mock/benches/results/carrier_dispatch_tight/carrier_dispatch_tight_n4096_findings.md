# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 38% faster than the next best (carrier_disp_tight_bittree)

carrier_disp_tight_nullfloor (126.34 us) leads carrier_disp_tight_bittree (174.31 us) by 38%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_nullfloor beats baseline by 33% (significant)

carrier_disp_tight_nullfloor is -61.29 us (33%) faster than baseline carrier_disp_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_tight_ifchainasc shows alternating (throttle bounce) (autocorr -0.71)

carrier_disp_tight_ifchainasc's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_tight_nullfloor} vs {carrier_disp_tight_bittree, carrier_disp_tight_ifchainasc, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} (38% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor} and a slow tier {carrier_disp_tight_bittree, carrier_disp_tight_ifchainasc, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} with a 38% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 126339.5 ns median (-32.4% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.89x (fastest 126339.5 ns, slowest 238666.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 176206ns | 176945ns | 173562ns | 176264ns | 177441ns | -7.19% |
| carrier_disp_tight_fntable | 217151ns | 216744ns | 216020ns | 216632ns | 218495ns | +14.38% |
| carrier_disp_tight_ifchain | 192103ns | 192399ns | 186787ns | 191882ns | 195092ns | +1.19% |
| carrier_disp_tight_ifchainasc | 189486ns | 189316ns | 185955ns | 188898ns | 192132ns | -0.19% |
| carrier_disp_tight_ifchainlin | 241091ns | 241179ns | 238716ns | 240761ns | 242773ns | +26.99% |
| carrier_disp_tight_nullfloor | 128932ns | 129026ns | 126218ns | 128165ns | 131439ns | -32.09% |
| carrier_disp_tight_switch | 189851ns | 189627ns | 188380ns | 189411ns | 191246ns | base |
| carrier_disp_tight_threaded | 201383ns | 201438ns | 199829ns | 201243ns | 202371ns | +6.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 173615ns | 171061ns | 174913ns | -7.23% | 0.024 |
| carrier_disp_tight_fntable | 214415ns | 213068ns | 215736ns | +14.57% | 0.019 |
| carrier_disp_tight_ifchain | 189417ns | 184141ns | 192366ns | +1.21% | 0.022 |
| carrier_disp_tight_ifchainasc | 186894ns | 183412ns | 189465ns | -0.14% | 0.022 |
| carrier_disp_tight_ifchainlin | 238519ns | 236197ns | 240045ns | +27.45% | 0.017 |
| carrier_disp_tight_nullfloor | 126341ns | 123737ns | 128842ns | -32.49% | 0.032 |
| carrier_disp_tight_switch | 187150ns | 185730ns | 188432ns | base | 0.022 |
| carrier_disp_tight_threaded | 198723ns | 197247ns | 199656ns | +6.18% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 1090241 | 5080436 | 0.215 | 0.93× |
| carrier_disp_tight_fntable | 1359894 | 6637749 | 0.205 | 1.16× |
| carrier_disp_tight_ifchain | 1182236 | 4931512 | 0.240 | 1.01× |
| carrier_disp_tight_ifchainasc | 1183140 | 4931397 | 0.240 | 1.01× |
| carrier_disp_tight_ifchainlin | 1497105 | 7272441 | 0.206 | 1.28× |
| carrier_disp_tight_nullfloor | 789583 | 4271255 | 0.185 | 0.68× |
| carrier_disp_tight_switch | 1169111 | 4800579 | 0.244 | 1.00× |
| carrier_disp_tight_threaded | 1244353 | 6635437 | 0.188 | 1.06× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.023 | 71.0% |
| carrier_disp_tight_fntable | 0.019 | 57.8% |
| carrier_disp_tight_ifchain | 0.022 | 65.2% |
| carrier_disp_tight_ifchainasc | 0.022 | 66.3% |
| carrier_disp_tight_ifchainlin | 0.017 | 51.8% |
| carrier_disp_tight_nullfloor | 0.032 | 97.9% |
| carrier_disp_tight_switch | 0.022 | 66.2% |
| carrier_disp_tight_threaded | 0.021 | 62.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 176206ns | 176206ns | -7.19% |
| carrier_disp_tight_fntable | 217151ns | 217151ns | +14.38% |
| carrier_disp_tight_ifchain | 192103ns | 192103ns | +1.19% |
| carrier_disp_tight_ifchainasc | 189486ns | 189486ns | -0.19% |
| carrier_disp_tight_ifchainlin | 241091ns | 241091ns | +26.99% |
| carrier_disp_tight_nullfloor | 128932ns | 128932ns | -32.09% |
| carrier_disp_tight_switch | 189851ns | 189851ns | base |
| carrier_disp_tight_threaded | 201383ns | 201383ns | +6.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 186996ns | base | --- | [186020, 188432] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 174311ns | -13004.6ns (-7.0%) | [-16155, -11443]ns | [171622, 174913] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_fntable | 214117ns | +27626.7ns (+14.8%) | [+25431, +28739]ns | [213393, 215736] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 189674ns | no significant difference | [-1585, +5721]ns | [186210, 192366] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_tight_ifchainasc | 186646ns | no significant difference | [-3223, +2820]ns | [184572, 189465] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_tight_ifchainlin | 238667ns | +51878.6ns (+27.7%) | [+48848, +53383]ns | [236847, 240045] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 126340ns | -61294.7ns (-32.8%) | [-63311, -57820]ns | [123842, 128842] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 198794ns | +11970.0ns (+6.4%) | [+9934, +12816]ns | [197720, 199656] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 186310ns | -6.5% | +15.0% | +2.8% | +2.2% | +28.6% | -31.2% | +6.9% |
| 2 | 187013ns | -6.8% | +15.0% | -1.5% | -1.9% | +28.6% | -30.7% | +6.9% |
| 3 | 186979ns | -8.5% | +15.8% | +3.3% | +0.8% | +27.0% | -32.9% | +6.0% |
| 4 | 188575ns | -8.7% | +13.3% | -0.2% | -1.5% | +26.4% | -34.4% | +4.6% |
| 5 | 188289ns | -7.1% | +13.7% | +0.2% | -0.4% | +25.4% | -32.4% | +5.9% |
| 6 | 185730ns | -5.8% | +14.7% | +2.7% | +0.0% | +28.7% | -33.3% | +6.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | 0.155 | ok |
| carrier_disp_tight_fntable | 0.082 | ok |
| carrier_disp_tight_ifchain | -0.708 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchainasc | -0.708 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchainlin | -0.031 | ok |
| carrier_disp_tight_nullfloor | 0.012 | ok |
| carrier_disp_tight_switch | -0.016 | ok |
| carrier_disp_tight_threaded | -0.152 | ok |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 6/6, lost 0/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 2/6, lost 4/6
- **carrier_disp_tight_ifchainasc**: won 3/6, lost 2/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 174078.6ns | 173615.4ns | 100.3% | HIGH |
| carrier_disp_tight_fntable | 219462.1ns | 214415.1ns | 102.4% | HIGH |
| carrier_disp_tight_ifchain | 189840.8ns | 189416.6ns | 100.2% | HIGH |
| carrier_disp_tight_ifchainasc | 187364.6ns | 186894.2ns | 100.3% | HIGH |
| carrier_disp_tight_ifchainlin | 239547.8ns | 238519.5ns | 100.4% | HIGH |
| carrier_disp_tight_nullfloor | 126996.2ns | 126341.0ns | 100.5% | HIGH |
| carrier_disp_tight_switch | 187899.0ns | 187149.6ns | 100.4% | HIGH |
| carrier_disp_tight_threaded | 199843.8ns | 198723.0ns | 100.6% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 171060.8-174913.2 ns)
  171060.8 |########################################
  171253.4 |
  171446.0 |
  171638.7 |
  171831.3 |
  172023.9 |########################################
  172216.5 |
  172409.1 |
  172601.7 |
  172794.4 |
  172987.0 |
  173179.6 |
  173372.2 |
  173564.8 |
  173757.4 |
  173950.1 |
  174142.7 |########################################
  174335.3 |########################################
  174527.9 |
  174720.5 |########################################
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 213067.5-215735.6 ns)
  213067.5 |########################################
  213200.9 |
  213334.3 |
  213467.7 |
  213601.1 |########################################
  213734.5 |
  213867.9 |
  214001.3 |########################################
  214134.7 |########################################
  214268.1 |
  214401.5 |
  214535.0 |
  214668.4 |
  214801.8 |
  214935.2 |########################################
  215068.6 |
  215202.0 |
  215335.4 |
  215468.8 |
  215602.2 |
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 184141.2-192365.8 ns)
  184141.2 |####################
  184552.4 |
  184963.7 |
  185374.9 |
  185786.1 |
  186197.4 |
  186608.6 |
  187019.8 |
  187431.1 |
  187842.3 |
  188253.5 |########################################
  188664.8 |
  189076.0 |
  189487.2 |
  189898.5 |
  190309.7 |
  190720.9 |####################
  191132.2 |
  191543.4 |####################
  191954.6 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 183411.7-189465.0 ns)
  183411.7 |####################
  183714.4 |
  184017.0 |
  184319.7 |
  184622.4 |
  184925.0 |
  185227.7 |
  185530.4 |########################################
  185833.0 |
  186135.7 |
  186438.4 |
  186741.0 |
  187043.7 |
  187346.3 |####################
  187649.0 |
  187951.7 |
  188254.3 |####################
  188557.0 |
  188859.7 |
  189162.3 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 236196.7-240044.6 ns)
  236196.7 |########################################
  236389.1 |
  236581.5 |
  236773.9 |
  236966.3 |
  237158.7 |
  237351.1 |########################################
  237543.5 |
  237735.9 |
  237928.3 |
  238120.7 |
  238313.0 |########################################
  238505.4 |
  238697.8 |
  238890.2 |########################################
  239082.6 |
  239275.0 |
  239467.4 |########################################
  239659.8 |
  239852.2 |
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 123736.7-128841.7 ns)
  123736.7 |########################################
  123991.9 |
  124247.2 |
  124502.4 |
  124757.7 |
  125012.9 |
  125268.2 |####################
  125523.4 |
  125778.7 |
  126033.9 |
  126289.2 |
  126544.4 |
  126799.7 |
  127054.9 |####################
  127310.2 |
  127565.4 |
  127820.7 |
  128075.9 |####################
  128331.2 |
  128586.4 |
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 185730.4-188432.3 ns)
  185730.4 |####################
  185865.5 |
  186000.6 |
  186135.7 |
  186270.8 |####################
  186405.9 |
  186541.0 |
  186676.1 |
  186811.2 |
  186946.3 |########################################
  187081.3 |
  187216.4 |
  187351.5 |
  187486.6 |
  187621.7 |
  187756.8 |
  187891.9 |
  188027.0 |
  188162.1 |####################
  188297.2 |
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 197247.1-199655.6 ns)
  197247.1 |########################################
  197367.5 |
  197488.0 |
  197608.4 |
  197728.8 |
  197849.2 |
  197969.6 |
  198090.1 |########################################
  198210.5 |
  198330.9 |
  198451.4 |########################################
  198571.8 |
  198692.2 |
  198812.6 |
  198933.1 |
  199053.5 |########################################
  199173.9 |
  199294.3 |
  199414.8 |########################################
  199535.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=102.4% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=100.7% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=100.6% of algo (FFI overhead may distort results)
