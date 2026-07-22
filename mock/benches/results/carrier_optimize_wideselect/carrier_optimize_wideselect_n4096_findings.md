# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_wideselect_all beats baseline by 27% (significant)

carrier_opt_wideselect_all is -115.85 us (27%) faster than baseline carrier_opt_wideselect_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_opt_wideselect_all** at 316989.0 ns median (-25.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.36x (fastest 316989.0 ns, slowest 429768.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 314708ns | 320041ns | 275741ns | 311570ns | 338897ns | -26.02% |
| carrier_opt_wideselect_cse | 348857ns | 346694ns | 319803ns | 343203ns | 371864ns | -18.00% |
| carrier_opt_wideselect_cseeqsat | 319745ns | 330017ns | 263402ns | 317017ns | 352008ns | -24.84% |
| carrier_opt_wideselect_dce | 423615ns | 417979ns | 393485ns | 412894ns | 454762ns | -0.42% |
| carrier_opt_wideselect_eqsat | 335306ns | 328478ns | 276114ns | 320342ns | 387349ns | -21.18% |
| carrier_opt_wideselect_fold | 430332ns | 432897ns | 405154ns | 429888ns | 443589ns | +1.15% |
| carrier_opt_wideselect_none | 425422ns | 426386ns | 370742ns | 424548ns | 454074ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 311847ns | 272775ns | 336193ns | -26.19% | 0.013 |
| carrier_opt_wideselect_cse | 346114ns | 317627ns | 369189ns | -18.07% | 0.012 |
| carrier_opt_wideselect_cseeqsat | 316771ns | 260424ns | 348793ns | -25.02% | 0.013 |
| carrier_opt_wideselect_dce | 420818ns | 390184ns | 451718ns | -0.39% | 0.010 |
| carrier_opt_wideselect_eqsat | 332397ns | 273869ns | 384399ns | -21.32% | 0.012 |
| carrier_opt_wideselect_fold | 427413ns | 402112ns | 440706ns | +1.17% | 0.010 |
| carrier_opt_wideselect_none | 422473ns | 368537ns | 451354ns | base | 0.010 |

## Performance model

- Peak throughput: **0.016 Gops/s** (carrier_opt_wideselect_cseeqsat; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.013 | 82.2% |
| carrier_opt_wideselect_cse | 0.012 | 75.8% |
| carrier_opt_wideselect_cseeqsat | 0.013 | 79.6% |
| carrier_opt_wideselect_dce | 0.010 | 62.7% |
| carrier_opt_wideselect_eqsat | 0.013 | 80.0% |
| carrier_opt_wideselect_fold | 0.010 | 60.6% |
| carrier_opt_wideselect_none | 0.010 | 61.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 314708ns | 314708ns | -26.02% |
| carrier_opt_wideselect_cse | 348857ns | 348857ns | -18.00% |
| carrier_opt_wideselect_cseeqsat | 319745ns | 319745ns | -24.84% |
| carrier_opt_wideselect_dce | 423615ns | 423615ns | -0.42% |
| carrier_opt_wideselect_eqsat | 335306ns | 335306ns | -21.18% |
| carrier_opt_wideselect_fold | 430332ns | 430332ns | +1.15% |
| carrier_opt_wideselect_none | 425422ns | 425422ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 423096ns | base | --- | [392969, 451354] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 316989ns | -115845.1ns (-27.4%) | [-130579, -85454]ns | [282359, 336193] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_wideselect_cse | 343624ns | -84161.7ns (-19.9%) | [-121135, -23780]ns | [325530, 369189] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_wideselect_cseeqsat | 327124ns | -95972.7ns (-22.7%) | [-148864, -72268]ns | [274397, 348793] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_wideselect_dce | 415607ns | no significant difference | [-47351, +40481]ns | [395130, 451718] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_wideselect_eqsat | 325346ns | -110250.2ns (-26.1%) | [-140433, -19543]ns | [287447, 384399] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| carrier_opt_wideselect_fold | 429768ns | no significant difference | [-10648, +22730]ns | [411764, 440706] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_cse | carrier_opt_wideselect_cseeqsat | carrier_opt_wideselect_dce | carrier_opt_wideselect_eqsat | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|---|
| 1 | 368537ns | -26.0% | -5.9% | -21.8% | +12.2% | +15.4% | +9.1% |
| 2 | 417401ns | -30.1% | -6.2% | -15.4% | -4.2% | -34.4% | +1.0% |
| 3 | 418884ns | -17.9% | -20.4% | -19.5% | +8.6% | -22.9% | +2.8% |
| 4 | 464348ns | -29.2% | -26.2% | -25.8% | -16.0% | -26.0% | -3.4% |
| 5 | 438360ns | -26.6% | -27.5% | -40.6% | -4.7% | -31.3% | -1.2% |
| 6 | 427309ns | -27.0% | -19.4% | -25.8% | +4.9% | -23.3% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | 0.256 | moderate+ |
| carrier_opt_wideselect_cse | -0.118 | ok |
| carrier_opt_wideselect_cseeqsat | -0.198 | ok |
| carrier_opt_wideselect_dce | -0.473 | moderate- |
| carrier_opt_wideselect_eqsat | -0.391 | moderate- |
| carrier_opt_wideselect_fold | 0.282 | moderate+ |
| carrier_opt_wideselect_none | 0.178 | ok |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cse**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_wideselect_dce**: won 3/6, lost 3/6
- **carrier_opt_wideselect_eqsat**: won 5/6, lost 1/6
- **carrier_opt_wideselect_fold**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 312655.8ns | 311846.9ns | 100.3% | HIGH |
| carrier_opt_wideselect_cse | 347234.3ns | 346114.1ns | 100.3% | HIGH |
| carrier_opt_wideselect_cseeqsat | 317084.9ns | 316771.3ns | 100.1% | HIGH |
| carrier_opt_wideselect_dce | 421099.4ns | 420818.5ns | 100.1% | HIGH |
| carrier_opt_wideselect_eqsat | 421411.5ns | 332397.4ns | 126.8% | HIGH |
| carrier_opt_wideselect_fold | 428526.6ns | 427412.6ns | 100.3% | HIGH |
| carrier_opt_wideselect_none | 423429.8ns | 422472.9ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 272774.6-336192.7 ns)
  272774.6 |########################################
  275945.5 |
  279116.4 |
  282287.3 |
  285458.2 |
  288629.1 |
  291800.0 |########################################
  294970.9 |
  298141.8 |
  301312.7 |
  304483.7 |
  307654.6 |
  310825.5 |########################################
  313996.4 |
  317167.3 |
  320338.2 |########################################
  323509.1 |
  326680.0 |########################################
  329850.9 |
  333021.8 |
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 317626.7-369189.2 ns)
  317626.7 |########################################
  320204.8 |
  322782.9 |
  325361.1 |
  327939.2 |
  330517.3 |
  333095.4 |########################################
  335673.6 |
  338251.7 |
  340829.8 |########################################
  343407.9 |########################################
  345986.0 |########################################
  348564.2 |
  351142.3 |
  353720.4 |
  356298.5 |
  358876.7 |
  361454.8 |
  364032.9 |
  366611.0 |
  (0 below, 1 above range)

carrier_opt_wideselect_cseeqsat (n=6, range 260424.2-348793.3 ns)
  260424.2 |########################################
  264842.7 |
  269261.1 |
  273679.6 |
  278098.0 |
  282516.5 |
  286934.9 |########################################
  291353.4 |
  295771.9 |
  300190.3 |
  304608.8 |
  309027.2 |
  313445.7 |########################################
  317864.1 |
  322282.6 |
  326701.1 |
  331119.5 |
  335538.0 |########################################
  339956.4 |
  344374.9 |########################################
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 390183.8-451717.9 ns)
  390183.8 |########################################
  393260.5 |
  396337.2 |
  399413.9 |########################################
  402490.6 |
  405567.3 |
  408644.0 |
  411720.7 |########################################
  414797.4 |########################################
  417874.1 |
  420950.8 |
  424027.6 |
  427104.3 |
  430181.0 |
  433257.7 |
  436334.4 |
  439411.1 |
  442487.8 |
  445564.5 |########################################
  448641.2 |
  (0 below, 1 above range)

carrier_opt_wideselect_eqsat (n=6, range 273869.2-384399.0 ns)
  273869.2 |########################################
  279395.7 |
  284922.2 |
  290448.7 |
  295975.2 |########################################
  301501.6 |
  307028.1 |
  312554.6 |
  318081.1 |########################################
  323607.6 |########################################
  329134.1 |
  334660.6 |
  340187.1 |########################################
  345713.5 |
  351240.0 |
  356766.5 |
  362293.0 |
  367819.5 |
  373346.0 |
  378872.5 |
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 402111.7-440705.6 ns)
  402111.7 |########################################
  404041.4 |
  405971.1 |
  407900.8 |
  409830.5 |
  411760.2 |
  413689.9 |
  415619.6 |
  417549.3 |
  419479.0 |
  421408.7 |########################################
  423338.3 |
  425268.0 |
  427197.7 |########################################
  429127.4 |########################################
  431057.1 |
  432986.8 |########################################
  434916.5 |
  436846.2 |
  438775.9 |
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 368536.7-451353.5 ns)
  368536.7 |########################################
  372677.5 |
  376818.4 |
  380959.2 |
  385100.1 |
  389240.9 |
  393381.8 |
  397522.6 |
  401663.4 |
  405804.3 |
  409945.1 |
  414086.0 |########################################
  418226.8 |########################################
  422367.7 |
  426508.5 |########################################
  430649.3 |
  434790.2 |########################################
  438931.0 |
  443071.9 |
  447212.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cseeqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_eqsat**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=100.3% of algo (FFI overhead may distort results)
