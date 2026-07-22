# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_tight_all dominates: 1165% faster than the next best (carrier_opt_tight_fold)

carrier_opt_tight_all (7.96 us) leads carrier_opt_tight_fold (100.71 us) by 1165%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 95% (significant)

carrier_opt_tight_all is -144.94 us (95%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_eqsat is an outlier: 56.6x slower than the field

carrier_opt_tight_eqsat (450.80 us) is 56.6x the fastest (7.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_dce, carrier_opt_tight_cseeqsat, carrier_opt_tight_eqsat} (1165% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_dce, carrier_opt_tight_cseeqsat, carrier_opt_tight_eqsat} with a 1165% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 56.6x the fastest

Fastest carrier_opt_tight_all (7.96 us) to slowest carrier_opt_tight_eqsat (450.80 us): 56.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_tight_all** at 7960.6 ns median (-94.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 56.63x (fastest 7960.6 ns, slowest 450798.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 10303ns | 10171ns | 10022ns | 10137ns | 10693ns | -93.37% |
| carrier_opt_tight_cse | 150271ns | 149922ns | 147711ns | 149521ns | 152676ns | -3.29% |
| carrier_opt_tight_cseeqsat | 452909ns | 451403ns | 449983ns | 451346ns | 456716ns | +191.46% |
| carrier_opt_tight_dce | 157216ns | 157654ns | 152773ns | 156612ns | 160343ns | +1.17% |
| carrier_opt_tight_eqsat | 455802ns | 453300ns | 450698ns | 452919ns | 462677ns | +193.33% |
| carrier_opt_tight_fold | 104142ns | 103125ns | 101505ns | 102673ns | 107663ns | -32.98% |
| carrier_opt_tight_none | 155391ns | 155495ns | 150280ns | 154431ns | 159386ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 8046ns | 7879ns | 8292ns | -94.74% | 0.509 |
| carrier_opt_tight_cse | 147805ns | 145488ns | 150160ns | -3.45% | 0.028 |
| carrier_opt_tight_cseeqsat | 450378ns | 447664ns | 454056ns | +194.19% | 0.009 |
| carrier_opt_tight_dce | 154584ns | 150530ns | 157758ns | +0.98% | 0.026 |
| carrier_opt_tight_eqsat | 453269ns | 448482ns | 459994ns | +196.08% | 0.009 |
| carrier_opt_tight_fold | 101754ns | 99339ns | 105095ns | -33.53% | 0.040 |
| carrier_opt_tight_none | 153091ns | 148116ns | 157055ns | base | 0.027 |

## Performance model

- Peak throughput: **0.520 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.515 | 99.0% |
| carrier_opt_tight_cse | 0.028 | 5.3% |
| carrier_opt_tight_cseeqsat | 0.009 | 1.8% |
| carrier_opt_tight_dce | 0.026 | 5.1% |
| carrier_opt_tight_eqsat | 0.009 | 1.7% |
| carrier_opt_tight_fold | 0.041 | 7.8% |
| carrier_opt_tight_none | 0.027 | 5.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 10303ns | 10303ns | -93.37% |
| carrier_opt_tight_cse | 150271ns | 150271ns | -3.29% |
| carrier_opt_tight_cseeqsat | 452909ns | 452909ns | +191.46% |
| carrier_opt_tight_dce | 157216ns | 157216ns | +1.17% |
| carrier_opt_tight_eqsat | 455802ns | 455802ns | +193.33% |
| carrier_opt_tight_fold | 104142ns | 104142ns | -32.98% |
| carrier_opt_tight_none | 155391ns | 155391ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 153217ns | base | --- | [149000, 157055] | --- | --- | --- | --- |
| carrier_opt_tight_all | 7961ns | -144940.5ns (-94.6%) | [-149079, -141116]ns | [7884, 8292] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_cse | 147285ns | -4632.6ns (-3.0%) | [-9949, -1275]ns | [145970, 150160] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| carrier_opt_tight_cseeqsat | 449061ns | +297529.2ns (+194.2%) | [+291276, +303058]ns | [448018, 454056] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_dce | 154802ns | no significant difference | [-4455, +6435]ns | [151192, 157758] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_tight_eqsat | 450799ns | +299030.0ns (+195.2%) | [+295591, +305912]ns | [449013, 459994] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_tight_fold | 100711ns | -50172.9ns (-32.7%) | [-57050, -46788]ns | [99456, 105095] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_cse | carrier_opt_tight_cseeqsat | carrier_opt_tight_dce | carrier_opt_tight_eqsat | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|---|
| 1 | 149885ns | -94.7% | -2.3% | +199.1% | +1.3% | +199.9% | -30.4% |
| 2 | 152554ns | -94.4% | -3.8% | +194.4% | +2.1% | +195.0% | -33.8% |
| 3 | 148116ns | -94.7% | +0.3% | +205.7% | +6.5% | +204.9% | -32.9% |
| 4 | 154755ns | -94.9% | -1.9% | +189.3% | +2.0% | +189.8% | -35.1% |
| 5 | 159355ns | -95.0% | -8.7% | +181.8% | -5.5% | +187.3% | -37.5% |
| 6 | 153881ns | -94.8% | -3.9% | +195.9% | -0.1% | +200.4% | -31.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | -0.430 | moderate- |
| carrier_opt_tight_cse | -0.234 | moderate- |
| carrier_opt_tight_cseeqsat | -0.220 | moderate- |
| carrier_opt_tight_dce | 0.017 | ok |
| carrier_opt_tight_eqsat | 0.295 | moderate+ |
| carrier_opt_tight_fold | -0.083 | ok |
| carrier_opt_tight_none | 0.147 | ok |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_cse**: won 5/6, lost 1/6
- **carrier_opt_tight_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_dce**: won 1/6, lost 4/6
- **carrier_opt_tight_eqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 88344.5ns | 8045.8ns | 1098.0% | HIGH |
| carrier_opt_tight_cse | 148495.7ns | 147805.1ns | 100.5% | HIGH |
| carrier_opt_tight_cseeqsat | 452638.4ns | 450378.4ns | 100.5% | HIGH |
| carrier_opt_tight_dce | 155196.0ns | 154583.8ns | 100.4% | HIGH |
| carrier_opt_tight_eqsat | 454851.6ns | 453268.6ns | 100.3% | HIGH |
| carrier_opt_tight_fold | 102257.6ns | 101753.9ns | 100.5% | HIGH |
| carrier_opt_tight_none | 153329.2ns | 153090.8ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 7879.2-8292.3 ns)
   7879.2 |########################################
   7899.9 |####################
   7920.5 |
   7941.2 |
   7961.8 |
   7982.5 |
   8003.1 |####################
   8023.8 |####################
   8044.4 |
   8065.1 |
   8085.8 |
   8106.4 |
   8127.1 |
   8147.7 |
   8168.4 |
   8189.0 |
   8209.7 |
   8230.3 |
   8251.0 |
   8271.6 |
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 145487.9-150159.8 ns)
  145487.9 |########################################
  145721.5 |
  145955.1 |
  146188.7 |
  146422.3 |########################################
  146655.9 |########################################
  146889.5 |
  147123.1 |
  147356.7 |
  147590.3 |
  147823.8 |########################################
  148057.4 |
  148291.0 |########################################
  148524.6 |
  148758.2 |
  148991.8 |
  149225.4 |
  149459.0 |
  149692.6 |
  149926.2 |
  (0 below, 1 above range)

carrier_opt_tight_cseeqsat (n=6, range 447664.2-454056.1 ns)
  447664.2 |####################
  447983.8 |
  448303.4 |####################
  448623.0 |
  448942.6 |########################################
  449262.2 |
  449581.8 |
  449901.3 |
  450220.9 |
  450540.5 |
  450860.1 |
  451179.7 |
  451499.3 |
  451818.9 |
  452138.5 |
  452458.1 |####################
  452777.7 |
  453097.3 |
  453416.9 |
  453736.5 |
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 150529.6-157758.1 ns)
  150529.6 |########################################
  150891.0 |
  151252.5 |
  151613.9 |########################################
  151975.3 |
  152336.7 |
  152698.1 |
  153059.6 |
  153421.0 |
  153782.4 |########################################
  154143.8 |
  154505.3 |
  154866.7 |
  155228.1 |
  155589.5 |########################################
  155951.0 |
  156312.4 |
  156673.8 |
  157035.2 |
  157396.7 |########################################
  (0 below, 1 above range)

carrier_opt_tight_eqsat (n=6, range 448482.1-459993.5 ns)
  448482.1 |########################################
  449057.7 |########################################
  449633.2 |########################################
  450208.8 |
  450784.4 |
  451360.0 |########################################
  451935.5 |
  452511.1 |
  453086.7 |
  453662.3 |
  454237.8 |
  454813.4 |
  455389.0 |
  455964.5 |
  456540.1 |
  457115.7 |
  457691.3 |########################################
  458266.8 |
  458842.4 |
  459418.0 |
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 99339.2-105094.8 ns)
  99339.2 |########################################
  99627.0 |
  99914.8 |
  100202.5 |####################
  100490.3 |
  100778.1 |####################
  101065.9 |
  101353.6 |
  101641.4 |
  101929.2 |
  102217.0 |
  102504.8 |
  102792.5 |
  103080.3 |
  103368.1 |
  103655.9 |
  103943.6 |
  104231.4 |####################
  104519.2 |
  104807.0 |
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 148115.8-157054.8 ns)
  148115.8 |########################################
  148562.8 |
  149009.7 |
  149456.6 |########################################
  149903.6 |
  150350.5 |
  150797.5 |
  151244.4 |
  151691.4 |
  152138.3 |########################################
  152585.3 |
  153032.2 |
  153479.2 |########################################
  153926.1 |
  154373.1 |########################################
  154820.0 |
  155267.0 |
  155713.9 |
  156160.9 |
  156607.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=1104.2% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cseeqsat**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_opt_tight_eqsat**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=100.4% of algo (FFI overhead may distort results)
