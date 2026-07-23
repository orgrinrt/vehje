# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: carrier_opt_scatter_all** at 491099.2 ns median (-9.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.13x (fastest 491099.2 ns, slowest 554135.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 490280ns | 494016ns | 469831ns | 490527ns | 500134ns | -11.31% |
| carrier_opt_scatter_canon | 502839ns | 506009ns | 490904ns | 502898ns | 508717ns | -9.04% |
| carrier_opt_scatter_cse | 503035ns | 502656ns | 458860ns | 497579ns | 533307ns | -9.00% |
| carrier_opt_scatter_dce | 555450ns | 556683ns | 523738ns | 551583ns | 577108ns | +0.48% |
| carrier_opt_scatter_fold | 544859ns | 545319ns | 530557ns | 542037ns | 556243ns | -1.44% |
| carrier_opt_scatter_none | 552792ns | 544256ns | 537400ns | 542093ns | 576538ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 487526ns | 467259ns | 497380ns | -11.32% | 0.008 |
| carrier_opt_scatter_canon | 500156ns | 488649ns | 505852ns | -9.03% | 0.008 |
| carrier_opt_scatter_cse | 500024ns | 456400ns | 529942ns | -9.05% | 0.008 |
| carrier_opt_scatter_dce | 552775ns | 521045ns | 574578ns | +0.55% | 0.007 |
| carrier_opt_scatter_fold | 542205ns | 528242ns | 553221ns | -1.38% | 0.008 |
| carrier_opt_scatter_none | 549776ns | 534405ns | 573390ns | base | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_scatter_all | 3048908 | 4168390 | 0.731 | 0.89× |
| carrier_opt_scatter_canon | 3117040 | 4188226 | 0.744 | 0.91× |
| carrier_opt_scatter_cse | 3135146 | 4187938 | 0.749 | 0.91× |
| carrier_opt_scatter_dce | 3426823 | 4446982 | 0.771 | 1.00× |
| carrier_opt_scatter_fold | 3378065 | 4449216 | 0.759 | 0.98× |
| carrier_opt_scatter_none | 3439757 | 4447356 | 0.773 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_opt_scatter_cse; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.008 | 92.9% |
| carrier_opt_scatter_canon | 0.008 | 90.7% |
| carrier_opt_scatter_cse | 0.008 | 91.3% |
| carrier_opt_scatter_dce | 0.007 | 82.4% |
| carrier_opt_scatter_fold | 0.008 | 84.1% |
| carrier_opt_scatter_none | 0.008 | 84.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 490280ns | 490280ns | -11.31% |
| carrier_opt_scatter_canon | 502839ns | 502839ns | -9.04% |
| carrier_opt_scatter_cse | 503035ns | 503035ns | -9.00% |
| carrier_opt_scatter_dce | 555450ns | 555450ns | +0.48% |
| carrier_opt_scatter_fold | 544859ns | 544859ns | -1.44% |
| carrier_opt_scatter_none | 552792ns | 552792ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 541426ns | base | --- | [534513, 573390] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 491099ns | -59491.2ns (-11.0%) | [-86655, -40603]ns | [474100, 497380] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_scatter_canon | 503179ns | -48080.0ns (-8.9%) | [-69356, -31424]ns | [491438, 505852] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_scatter_cse | 499650ns | -47742.1ns (-8.8%) | [-80312, -21203]ns | [470480, 529942] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_scatter_dce | 554136ns | no significant difference | [-11601, +16604]ns | [529613, 574578] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_scatter_fold | 542806ns | no significant difference | [-25508, +6719]ns | [530586, 553221] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_canon | carrier_opt_scatter_cse | carrier_opt_scatter_dce | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|
| 1 | 534621ns | -8.4% | -8.6% | -6.2% | -2.5% | -0.3% |
| 2 | 538437ns | -13.2% | -5.7% | -1.7% | +0.8% | +0.2% |
| 3 | 579602ns | -15.0% | -13.1% | -8.4% | -1.7% | -5.7% |
| 4 | 544415ns | -8.8% | -9.2% | -8.6% | +3.9% | +2.3% |
| 5 | 567179ns | -15.2% | -11.1% | -19.5% | +2.1% | -3.1% |
| 6 | 534405ns | -6.8% | -6.0% | -9.3% | +0.7% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | -0.337 | moderate- |
| carrier_opt_scatter_canon | -0.365 | moderate- |
| carrier_opt_scatter_cse | 0.417 | moderate+ |
| carrier_opt_scatter_dce | 0.130 | ok |
| carrier_opt_scatter_fold | 0.142 | ok |
| carrier_opt_scatter_none | -0.378 | moderate- |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 6/6, lost 0/6
- **carrier_opt_scatter_canon**: won 6/6, lost 0/6
- **carrier_opt_scatter_cse**: won 6/6, lost 0/6
- **carrier_opt_scatter_dce**: won 2/6, lost 4/6
- **carrier_opt_scatter_fold**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 488491.1ns | 487526.4ns | 100.2% | HIGH |
| carrier_opt_scatter_canon | 500820.8ns | 500156.2ns | 100.1% | HIGH |
| carrier_opt_scatter_cse | 500331.1ns | 500024.0ns | 100.1% | HIGH |
| carrier_opt_scatter_dce | 552888.7ns | 552775.3ns | 100.0% | HIGH |
| carrier_opt_scatter_fold | 543026.5ns | 542204.7ns | 100.2% | HIGH |
| carrier_opt_scatter_none | 550049.0ns | 549776.3ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 467259.2-497380.2 ns)
  467259.2 |########################################
  468765.2 |
  470271.3 |
  471777.4 |
  473283.4 |
  474789.5 |
  476295.5 |
  477801.5 |
  479307.6 |
  480813.7 |########################################
  482319.7 |
  483825.8 |
  485331.8 |
  486837.9 |
  488343.9 |########################################
  489850.0 |
  491356.0 |########################################
  492862.0 |
  494368.1 |
  495874.2 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_canon (n=6, range 488649.2-505851.7 ns)
  488649.2 |########################################
  489509.3 |
  490369.4 |
  491229.6 |
  492089.7 |
  492949.8 |
  493809.9 |########################################
  494670.1 |
  495530.2 |
  496390.3 |
  497250.4 |
  498110.5 |
  498970.7 |
  499830.8 |
  500690.9 |
  501551.0 |
  502411.2 |########################################
  503271.3 |########################################
  504131.4 |########################################
  504991.5 |
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 456399.6-529942.3 ns)
  456399.6 |########################################
  460076.7 |
  463753.9 |
  467431.0 |
  471108.1 |
  474785.3 |
  478462.4 |
  482139.5 |########################################
  485816.7 |
  489493.8 |
  493171.0 |
  496848.1 |########################################
  500525.2 |########################################
  504202.4 |
  507879.5 |
  511556.6 |
  515233.8 |
  518910.9 |
  522588.0 |
  526265.2 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 521045.0-574577.5 ns)
  521045.0 |########################################
  523721.6 |
  526398.2 |
  529074.9 |
  531751.5 |
  534428.1 |
  537104.8 |########################################
  539781.4 |
  542458.0 |########################################
  545134.6 |
  547811.2 |
  550487.9 |
  553164.5 |
  555841.1 |
  558517.8 |
  561194.4 |
  563871.0 |########################################
  566547.6 |
  569224.2 |########################################
  571900.9 |
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 528241.7-553221.2 ns)
  528241.7 |########################################
  529490.7 |
  530739.7 |
  531988.6 |########################################
  533237.6 |
  534486.6 |
  535735.6 |
  536984.5 |
  538233.5 |########################################
  539482.5 |
  540731.5 |
  541980.5 |
  543229.4 |
  544478.4 |
  545727.4 |########################################
  546976.4 |
  548225.3 |########################################
  549474.3 |
  550723.3 |
  551972.3 |
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 534404.6-573390.4 ns)
  534404.6 |########################################
  536353.9 |
  538303.2 |####################
  540252.5 |
  542201.8 |
  544151.1 |####################
  546100.4 |
  548049.6 |
  549998.9 |
  551948.2 |
  553897.5 |
  555846.8 |
  557796.1 |
  559745.4 |
  561694.7 |
  563644.0 |
  565593.3 |####################
  567542.6 |
  569491.9 |
  571441.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_canon**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=100.2% of algo (FFI overhead may distort results)
