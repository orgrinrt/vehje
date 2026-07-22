# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null dominates: 290% faster than the next best (carrier_pre_scatter_direct)

carrier_pre_scatter_null (99.78 us) leads carrier_pre_scatter_direct (389.50 us) by 290%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_scatter_null beats baseline by 80% (significant)

carrier_pre_scatter_null is -406.39 us (80%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 5.8x slower than the field

carrier_pre_scatter_fntable (582.66 us) is 5.8x the fastest (99.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_scatter_null} vs {carrier_pre_scatter_direct, carrier_pre_scatter_switch, carrier_pre_scatter_threaded, carrier_pre_scatter_regcache, carrier_pre_scatter_fntable} (290% apart)

The field splits into a fast tier {carrier_pre_scatter_null} and a slow tier {carrier_pre_scatter_direct, carrier_pre_scatter_switch, carrier_pre_scatter_threaded, carrier_pre_scatter_regcache, carrier_pre_scatter_fntable} with a 290% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.8x the fastest

Fastest carrier_pre_scatter_null (99.78 us) to slowest carrier_pre_scatter_fntable (582.66 us): 5.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 99782.7 ns median (-80.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.84x (fastest 99782.7 ns, slowest 582662.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 377695ns | 391798ns | 323257ns | 377398ns | 405359ns | -25.76% |
| carrier_pre_scatter_fntable | 574527ns | 584846ns | 528594ns | 582418ns | 585658ns | +12.93% |
| carrier_pre_scatter_null | 102287ns | 102017ns | 98881ns | 100982ns | 105947ns | -79.89% |
| carrier_pre_scatter_regcache | 515616ns | 525100ns | 475150ns | 515471ns | 536066ns | +1.35% |
| carrier_pre_scatter_switch | 508739ns | 508963ns | 500253ns | 507552ns | 514762ns | base |
| carrier_pre_scatter_threaded | 512299ns | 516603ns | 487362ns | 510111ns | 528049ns | +0.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 375435ns | 320962ns | 403162ns | -25.88% | 0.011 |
| carrier_pre_scatter_fntable | 572312ns | 526361ns | 583437ns | +12.99% | 0.007 |
| carrier_pre_scatter_null | 100052ns | 96708ns | 103641ns | -80.25% | 0.041 |
| carrier_pre_scatter_regcache | 513402ns | 472967ns | 533844ns | +1.36% | 0.008 |
| carrier_pre_scatter_switch | 506503ns | 497963ns | 512518ns | base | 0.008 |
| carrier_pre_scatter_threaded | 510060ns | 485148ns | 525803ns | +0.70% | 0.008 |

## Performance model

- Peak throughput: **0.042 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.011 | 24.8% |
| carrier_pre_scatter_fntable | 0.007 | 16.6% |
| carrier_pre_scatter_null | 0.041 | 96.9% |
| carrier_pre_scatter_regcache | 0.008 | 18.5% |
| carrier_pre_scatter_switch | 0.008 | 19.1% |
| carrier_pre_scatter_threaded | 0.008 | 18.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 377695ns | 377695ns | -25.76% |
| carrier_pre_scatter_fntable | 574527ns | 574527ns | +12.93% |
| carrier_pre_scatter_null | 102287ns | 102287ns | -79.89% |
| carrier_pre_scatter_regcache | 515616ns | 515616ns | +1.35% |
| carrier_pre_scatter_switch | 508739ns | 508739ns | base |
| carrier_pre_scatter_threaded | 512299ns | 512299ns | +0.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 506760ns | base | --- | [500231, 512518] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 389495ns | -117265.0ns (-23.1%) | [-172335, -103605]ns | [333647, 403162] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 582662ns | +70659.2ns (+13.9%) | [+47244, +79524]ns | [550838, 583437] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_scatter_null | 99783ns | -406392.7ns (-80.2%) | [-414211, -398751]ns | [96731, 103641] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 522920ns | no significant difference | [-24223, +24875]ns | [483443, 533844] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_scatter_threaded | 514371ns | no significant difference | [-17661, +18610]ns | [490004, 525803] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 497963ns | -35.5% | +5.7% | -79.6% | +4.9% | +3.2% |
| 2 | 504295ns | -21.9% | +15.6% | -79.5% | -2.1% | -3.8% |
| 3 | 511036ns | -20.8% | +14.0% | -79.6% | -7.4% | -3.2% |
| 4 | 509225ns | -24.4% | +13.0% | -81.0% | +5.0% | +4.2% |
| 5 | 514001ns | -32.6% | +13.6% | -80.9% | +3.7% | +1.4% |
| 6 | 502499ns | -20.1% | +16.0% | -80.8% | +4.2% | +2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | -0.210 | moderate- |
| carrier_pre_scatter_fntable | -0.073 | ok |
| carrier_pre_scatter_null | 0.323 | moderate+ |
| carrier_pre_scatter_regcache | 0.123 | ok |
| carrier_pre_scatter_switch | 0.065 | ok |
| carrier_pre_scatter_threaded | 0.176 | ok |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 2/6, lost 4/6
- **carrier_pre_scatter_threaded**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 388469.0ns | 375435.0ns | 103.5% | HIGH |
| carrier_pre_scatter_fntable | 582602.7ns | 572312.4ns | 101.8% | HIGH |
| carrier_pre_scatter_null | 110684.2ns | 100051.7ns | 110.6% | HIGH |
| carrier_pre_scatter_regcache | 524316.4ns | 513402.4ns | 102.1% | HIGH |
| carrier_pre_scatter_switch | 530035.0ns | 506503.2ns | 104.6% | HIGH |
| carrier_pre_scatter_threaded | 520571.6ns | 510059.5ns | 102.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 320962.1-403162.5 ns)
  320962.1 |########################################
  325072.1 |
  329182.1 |
  333292.2 |
  337402.2 |
  341512.2 |
  345622.2 |########################################
  349732.2 |
  353842.3 |
  357952.3 |
  362062.3 |
  366172.3 |
  370282.3 |
  374392.4 |
  378502.4 |
  382612.4 |########################################
  386722.4 |
  390832.4 |########################################
  394942.5 |
  399052.5 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 526361.2-583436.6 ns)
  526361.2 |#############
  529215.0 |
  532068.7 |
  534922.5 |
  537776.3 |
  540630.1 |
  543483.8 |
  546337.6 |
  549191.4 |
  552045.2 |
  554898.9 |
  557752.7 |
  560606.5 |
  563460.2 |
  566314.0 |
  569167.8 |
  572021.6 |
  574875.3 |#############
  577729.1 |
  580582.9 |########################################
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 96708.3-103641.4 ns)
  96708.3 |########################################
  97055.0 |
  97401.6 |
  97748.3 |####################
  98094.9 |
  98441.6 |
  98788.2 |
  99134.9 |
  99481.6 |
  99828.2 |
  100174.9 |
  100521.5 |
  100868.2 |
  101214.8 |####################
  101561.5 |
  101908.2 |
  102254.8 |
  102601.5 |
  102948.1 |####################
  103294.8 |
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 472966.7-533844.1 ns)
  472966.7 |####################
  476010.6 |
  479054.4 |
  482098.3 |
  485142.2 |
  488186.1 |
  491229.9 |####################
  494273.8 |
  497317.7 |
  500361.6 |
  503405.4 |
  506449.3 |
  509493.2 |
  512537.0 |
  515580.9 |
  518624.8 |
  521668.7 |########################################
  524712.5 |
  527756.4 |
  530800.3 |####################
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 497962.9-512518.5 ns)
  497962.9 |########################################
  498690.7 |
  499418.5 |
  500146.2 |
  500874.0 |
  501601.8 |
  502329.6 |########################################
  503057.4 |
  503785.1 |########################################
  504512.9 |
  505240.7 |
  505968.5 |
  506696.3 |
  507424.0 |
  508151.8 |
  508879.6 |########################################
  509607.4 |
  510335.2 |########################################
  511062.9 |
  511790.7 |
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 485147.5-525802.7 ns)
  485147.5 |####################
  487180.3 |
  489213.0 |
  491245.8 |
  493278.5 |####################
  495311.3 |
  497344.1 |
  499376.8 |
  501409.6 |
  503442.3 |
  505475.1 |
  507507.9 |
  509540.6 |
  511573.4 |
  513606.1 |########################################
  515638.9 |
  517671.7 |
  519704.4 |####################
  521737.2 |
  523769.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=103.4% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=110.4% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=102.0% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=105.1% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=102.0% of algo (FFI overhead may distort results)
