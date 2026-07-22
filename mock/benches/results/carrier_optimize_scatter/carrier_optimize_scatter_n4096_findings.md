# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_scatter_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_scatter_none has the worst median (570.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_scatter_cseeqsat at 487.78 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Speed leader carrier_opt_scatter_cseeqsat vs stability leader carrier_opt_scatter_all (+7% speed for 2.2x steadier)

carrier_opt_scatter_cseeqsat is fastest (487.78 us, CV 6.6%); carrier_opt_scatter_all gives up 6.7% median for 2.2x lower variance (CV 3.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_opt_scatter_cseeqsat** at 487782.9 ns median (-14.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.17x (fastest 487782.9 ns, slowest 570249.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 517786ns | 523621ns | 495341ns | 515408ns | 532576ns | -9.34% |
| carrier_opt_scatter_cse | 515845ns | 507445ns | 470773ns | 500634ns | 561197ns | -9.68% |
| carrier_opt_scatter_cseeqsat | 502644ns | 490704ns | 467134ns | 487427ns | 543224ns | -11.99% |
| carrier_opt_scatter_dce | 571545ns | 571351ns | 528921ns | 565246ns | 602306ns | +0.07% |
| carrier_opt_scatter_eqsat | 497568ns | 495657ns | 433540ns | 492002ns | 537931ns | -12.88% |
| carrier_opt_scatter_fold | 533412ns | 528532ns | 489321ns | 526455ns | 565895ns | -6.60% |
| carrier_opt_scatter_none | 571133ns | 572965ns | 539483ns | 566879ns | 593340ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 514826ns | 491942ns | 529890ns | -9.40% | 0.008 |
| carrier_opt_scatter_cse | 513054ns | 468601ns | 557660ns | -9.71% | 0.008 |
| carrier_opt_scatter_cseeqsat | 499542ns | 463924ns | 539875ns | -12.09% | 0.008 |
| carrier_opt_scatter_dce | 568599ns | 526539ns | 599287ns | +0.06% | 0.007 |
| carrier_opt_scatter_eqsat | 494543ns | 430826ns | 534670ns | -12.97% | 0.008 |
| carrier_opt_scatter_fold | 530742ns | 487055ns | 563021ns | -6.60% | 0.008 |
| carrier_opt_scatter_none | 568231ns | 536793ns | 590305ns | base | 0.007 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_opt_scatter_eqsat; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.008 | 82.8% |
| carrier_opt_scatter_cse | 0.008 | 85.3% |
| carrier_opt_scatter_cseeqsat | 0.008 | 88.3% |
| carrier_opt_scatter_dce | 0.007 | 75.8% |
| carrier_opt_scatter_eqsat | 0.008 | 87.4% |
| carrier_opt_scatter_fold | 0.008 | 81.9% |
| carrier_opt_scatter_none | 0.007 | 75.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 517786ns | 517786ns | -9.34% |
| carrier_opt_scatter_cse | 515845ns | 515845ns | -9.68% |
| carrier_opt_scatter_cseeqsat | 502644ns | 502644ns | -11.99% |
| carrier_opt_scatter_dce | 571545ns | 571545ns | +0.07% |
| carrier_opt_scatter_eqsat | 497568ns | 497568ns | -12.88% |
| carrier_opt_scatter_fold | 533412ns | 533412ns | -6.60% |
| carrier_opt_scatter_none | 571133ns | 571133ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 570250ns | base | --- | [544140, 590305] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 520565ns | -43601.9ns (-7.6%) | [-83454, -33159]ns | [494024, 529890] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_scatter_cse | 505076ns | -39063.8ns (-6.9%) | [-112679, -13790]ns | [476425, 557660] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| carrier_opt_scatter_cseeqsat | 487783ns | -82466.7ns (-14.5%) | [-87849, -35753]ns | [470967, 539875] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_scatter_dce | 568151ns | no significant difference | [-42995, +37434]ns | [538361, 599287] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_scatter_eqsat | 492909ns | -72756.0ns (-12.8%) | [-114198, -34110]ns | [456051, 534670] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_scatter_fold | 525883ns | no significant difference | [-71909, +6098]ns | [503320, 563021] | no | 0.2625 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_cse | carrier_opt_scatter_cseeqsat | carrier_opt_scatter_dce | carrier_opt_scatter_eqsat | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|---|
| 1 | 572219ns | -8.1% | -15.4% | -14.2% | +6.6% | -24.7% | +4.2% |
| 2 | 568280ns | -6.0% | +0.0% | -14.7% | -0.0% | -15.3% | -8.6% |
| 3 | 551486ns | -5.8% | -5.5% | -15.9% | +6.7% | -10.3% | -11.7% |
| 4 | 536793ns | -7.6% | -8.9% | -11.0% | +2.5% | -2.1% | -2.2% |
| 5 | 574621ns | -9.2% | -4.8% | -2.2% | -8.4% | -14.5% | -7.8% |
| 6 | 605989ns | -18.8% | -22.7% | -14.5% | -6.3% | -10.2% | -13.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | -0.054 | ok |
| carrier_opt_scatter_cse | -0.479 | moderate- |
| carrier_opt_scatter_cseeqsat | 0.196 | ok |
| carrier_opt_scatter_dce | 0.097 | ok |
| carrier_opt_scatter_eqsat | 0.074 | ok |
| carrier_opt_scatter_fold | 0.002 | ok |
| carrier_opt_scatter_none | 0.206 | moderate+ |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 6/6, lost 0/6
- **carrier_opt_scatter_cse**: won 5/6, lost 0/6
- **carrier_opt_scatter_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_scatter_dce**: won 2/6, lost 3/6
- **carrier_opt_scatter_eqsat**: won 6/6, lost 0/6
- **carrier_opt_scatter_fold**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 515634.3ns | 514826.4ns | 100.2% | HIGH |
| carrier_opt_scatter_cse | 513721.2ns | 513053.6ns | 100.1% | HIGH |
| carrier_opt_scatter_cseeqsat | 499989.8ns | 499541.7ns | 100.1% | HIGH |
| carrier_opt_scatter_dce | 569844.9ns | 568599.5ns | 100.2% | HIGH |
| carrier_opt_scatter_eqsat | 494726.4ns | 494543.3ns | 100.0% | HIGH |
| carrier_opt_scatter_fold | 531299.5ns | 530741.5ns | 100.1% | HIGH |
| carrier_opt_scatter_none | 568610.5ns | 568231.5ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 491941.7-529890.0 ns)
  491941.7 |########################################
  493839.1 |
  495736.5 |########################################
  497633.9 |
  499531.4 |
  501428.8 |
  503326.2 |
  505223.6 |
  507121.0 |
  509018.4 |
  510915.8 |
  512813.3 |
  514710.7 |
  516608.1 |
  518505.5 |########################################
  520402.9 |########################################
  522300.3 |
  524197.8 |########################################
  526095.2 |
  527992.6 |
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 468600.8-557660.4 ns)
  468600.8 |########################################
  473053.8 |
  477506.8 |
  481959.7 |########################################
  486412.7 |########################################
  490865.7 |
  495318.7 |
  499771.7 |
  504224.6 |
  508677.6 |
  513130.6 |
  517583.6 |########################################
  522036.6 |
  526489.5 |
  530942.5 |
  535395.5 |
  539848.5 |
  544301.5 |########################################
  548754.4 |
  553207.4 |
  (0 below, 1 above range)

carrier_opt_scatter_cseeqsat (n=6, range 463923.8-539875.0 ns)
  463923.8 |########################################
  467721.4 |
  471518.9 |
  475316.5 |########################################
  479114.0 |
  482911.6 |########################################
  486709.2 |
  490506.7 |########################################
  494304.3 |
  498101.8 |
  501899.4 |
  505697.0 |
  509494.5 |
  513292.1 |
  517089.6 |########################################
  520887.2 |
  524684.8 |
  528482.3 |
  532279.9 |
  536077.4 |
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 526538.8-599286.7 ns)
  526538.8 |####################
  530176.2 |
  533813.6 |
  537451.0 |
  541088.4 |
  544725.8 |
  548363.2 |####################
  552000.6 |
  555638.0 |
  559275.4 |
  562912.8 |
  566550.1 |########################################
  570187.5 |
  573824.9 |
  577462.3 |
  581099.7 |
  584737.1 |
  588374.5 |####################
  592011.9 |
  595649.3 |
  (0 below, 1 above range)

carrier_opt_scatter_eqsat (n=6, range 430825.8-534669.6 ns)
  430825.8 |########################################
  436018.0 |
  441210.2 |
  446402.4 |
  451594.6 |
  456786.8 |
  461978.9 |
  467171.1 |
  472363.3 |
  477555.5 |########################################
  482747.7 |
  487939.9 |########################################
  493132.1 |########################################
  498324.3 |
  503516.5 |
  508708.6 |
  513900.8 |
  519093.0 |
  524285.2 |########################################
  529477.4 |
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 487054.6-563021.1 ns)
  487054.6 |####################
  490852.9 |
  494651.2 |
  498449.6 |
  502247.9 |
  506046.2 |
  509844.5 |
  513642.9 |
  517441.2 |####################
  521239.5 |
  525037.8 |########################################
  528836.1 |####################
  532634.5 |
  536432.8 |
  540231.1 |
  544029.4 |
  547827.8 |
  551626.1 |
  555424.4 |
  559222.7 |
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 536793.3-590305.2 ns)
  536793.3 |########################################
  539468.9 |
  542144.5 |
  544820.1 |
  547495.7 |
  550171.3 |########################################
  552846.9 |
  555522.5 |
  558198.1 |
  560873.7 |
  563549.2 |
  566224.8 |########################################
  568900.4 |
  571576.0 |########################################
  574251.6 |########################################
  576927.2 |
  579602.8 |
  582278.4 |
  584954.0 |
  587629.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cseeqsat**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_eqsat**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=100.0% of algo (FFI overhead may distort results)
