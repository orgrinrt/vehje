# Strategy showdown (skew2): all strategies x tiers (native/interp) same footing

12 variants, 6 samples per variant.
Baseline: **sd_bintree_int_skew2**

## Key findings

- **Fastest: sd_chain_rev_nat_skew2** at 43696.4 ns median (-69.9% vs baseline)
- 8 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.08x (fastest 43696.4 ns, slowest 178294.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 147417ns | 147542ns | 143810ns | 146624ns | 150411ns | base |
| sd_bintree_nat_skew2 | 64952ns | 64930ns | 62162ns | 64169ns | 67522ns | -55.94% |
| sd_chain_int_skew2 | 182298ns | 180852ns | 173588ns | 178903ns | 191747ns | +23.66% |
| sd_chain_nat_skew2 | 47409ns | 47569ns | 45147ns | 46950ns | 49228ns | -67.84% |
| sd_chain_rev_int_skew2 | 156440ns | 157298ns | 149101ns | 155009ns | 162256ns | +6.12% |
| sd_chain_rev_nat_skew2 | 45952ns | 46155ns | 44735ns | 45952ns | 46561ns | -68.83% |
| sd_evalall_int_skew2 | 96319ns | 94665ns | 91071ns | 94191ns | 102135ns | -34.66% |
| sd_evalall_nat_skew2 | 93594ns | 93813ns | 88185ns | 93466ns | 96490ns | -36.51% |
| sd_jumptable_int_skew2 | 47037ns | 47765ns | 43867ns | 46644ns | 49213ns | -68.09% |
| sd_jumptable_nat_skew2 | 47183ns | 46866ns | 42012ns | 46298ns | 51098ns | -67.99% |
| sd_profiled_int_skew2 | 150318ns | 149600ns | 146568ns | 148671ns | 154662ns | +1.97% |
| sd_profiled_nat_skew2 | 47437ns | 46521ns | 44332ns | 46097ns | 50999ns | -67.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 144872ns | 141557ns | 147554ns | base | 0.113 |
| sd_bintree_nat_skew2 | 62493ns | 59708ns | 65022ns | -56.86% | 0.262 |
| sd_chain_int_skew2 | 179778ns | 171307ns | 189193ns | +24.09% | 0.091 |
| sd_chain_nat_skew2 | 45043ns | 42789ns | 46817ns | -68.91% | 0.364 |
| sd_chain_rev_int_skew2 | 153877ns | 146693ns | 159470ns | +6.22% | 0.106 |
| sd_chain_rev_nat_skew2 | 43553ns | 42346ns | 44160ns | -69.94% | 0.376 |
| sd_evalall_int_skew2 | 93769ns | 88769ns | 99314ns | -35.27% | 0.175 |
| sd_evalall_nat_skew2 | 91245ns | 85940ns | 94121ns | -37.02% | 0.180 |
| sd_jumptable_int_skew2 | 44697ns | 41624ns | 46793ns | -69.15% | 0.367 |
| sd_jumptable_nat_skew2 | 44753ns | 39777ns | 48500ns | -69.11% | 0.366 |
| sd_profiled_int_skew2 | 147814ns | 144053ns | 152188ns | +2.03% | 0.111 |
| sd_profiled_nat_skew2 | 45077ns | 42069ns | 48500ns | -68.88% | 0.363 |

## Performance model

- Peak throughput: **0.412 Gops/s** (sd_jumptable_nat_skew2; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sd_bintree_int_skew2 | 0.113 | 27.4% |
| sd_bintree_nat_skew2 | 0.262 | 63.7% |
| sd_chain_int_skew2 | 0.092 | 22.3% |
| sd_chain_nat_skew2 | 0.363 | 88.1% |
| sd_chain_rev_int_skew2 | 0.106 | 25.7% |
| sd_chain_rev_nat_skew2 | 0.375 | 91.0% |
| sd_evalall_int_skew2 | 0.178 | 43.2% |
| sd_evalall_nat_skew2 | 0.179 | 43.5% |
| sd_jumptable_int_skew2 | 0.361 | 87.6% |
| sd_jumptable_nat_skew2 | 0.369 | 89.5% |
| sd_profiled_int_skew2 | 0.111 | 27.0% |
| sd_profiled_nat_skew2 | 0.370 | 89.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sd_bintree_int_skew2 | 147417ns | 147417ns | base |
| sd_bintree_nat_skew2 | 64952ns | 64952ns | -55.94% |
| sd_chain_int_skew2 | 182298ns | 182298ns | +23.66% |
| sd_chain_nat_skew2 | 47409ns | 47409ns | -67.84% |
| sd_chain_rev_int_skew2 | 156440ns | 156440ns | +6.12% |
| sd_chain_rev_nat_skew2 | 45952ns | 45952ns | -68.83% |
| sd_evalall_int_skew2 | 96319ns | 96319ns | -34.66% |
| sd_evalall_nat_skew2 | 93594ns | 93594ns | -36.51% |
| sd_jumptable_int_skew2 | 47037ns | 47037ns | -68.09% |
| sd_jumptable_nat_skew2 | 47183ns | 47183ns | -67.99% |
| sd_profiled_int_skew2 | 150318ns | 150318ns | +1.97% |
| sd_profiled_nat_skew2 | 47437ns | 47437ns | -67.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 145079ns | base | --- | [141983, 147554] | --- | --- | --- | --- |
| sd_bintree_nat_skew2 | 62479ns | -82343.7ns (-56.8%) | [-86007, -78786]ns | [59979, 65022] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_int_skew2 | 178294ns | +33978.1ns (+23.4%) | [+27753, +42987]ns | [171847, 189193] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_nat_skew2 | 45168ns | -100361.2ns (-69.2%) | [-103837, -95290]ns | [43144, 46817] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_rev_int_skew2 | 154770ns | +9695.8ns (+6.7%) | [+965, +16354]ns | [147392, 159470] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| sd_chain_rev_nat_skew2 | 43696ns | -100918.3ns (-69.6%) | [-103858, -99180]ns | [42803, 44160] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_int_skew2 | 92180ns | -53097.1ns (-36.6%) | [-55064, -45149]ns | [89812, 99314] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_nat_skew2 | 91422ns | -53491.7ns (-36.9%) | [-57309, -50080]ns | [88192, 94121] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_int_skew2 | 45390ns | -100594.4ns (-69.3%) | [-102687, -97244]ns | [41909, 46793] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_nat_skew2 | 44428ns | -101429.2ns (-69.9%) | [-104092, -94836]ns | [41332, 48500] | YES | 0.0382 | 0.0313 | 0 |
| sd_profiled_int_skew2 | 147095ns | no significant difference | [-2110, +8461]ns | [144158, 152188] | no | 0.2188 | 0.2188 | 0 |
| sd_profiled_nat_skew2 | 44236ns | -100227.2ns (-69.1%) | [-103930, -95227]ns | [42497, 48500] | YES | 0.0382 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | sd_bintree_int_skew2 | sd_bintree_nat_skew2 | sd_chain_int_skew2 | sd_chain_nat_skew2 | sd_chain_rev_int_skew2 | sd_chain_rev_nat_skew2 | sd_evalall_int_skew2 | sd_evalall_nat_skew2 | sd_jumptable_int_skew2 | sd_jumptable_nat_skew2 | sd_profiled_int_skew2 | sd_profiled_nat_skew2 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 142410ns | -57.7% | +21.1% | -67.4% | +10.9% | -69.6% | -29.2% | -39.7% | -70.8% | -65.0% | +1.3% | -69.0% |
| 2 | 141557ns | -55.3% | +26.8% | -66.8% | +12.2% | -70.1% | -37.3% | -35.4% | -67.4% | -71.9% | +7.9% | -65.7% |
| 3 | 146516ns | -59.2% | +28.2% | -70.0% | +3.5% | -70.1% | -33.3% | -34.1% | -67.7% | -70.7% | +0.2% | -70.7% |
| 4 | 144261ns | -57.2% | +18.7% | -70.3% | +1.7% | -69.2% | -37.0% | -36.4% | -70.8% | -67.3% | +2.1% | -70.8% |
| 5 | 145896ns | -54.3% | +30.7% | -68.0% | +9.8% | -69.9% | -36.9% | -37.3% | -68.8% | -69.3% | +4.0% | -66.8% |
| 6 | 148592ns | -57.3% | +19.2% | -70.7% | -0.3% | -70.6% | -37.8% | -39.1% | -69.5% | -70.4% | -3.1% | -70.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sd_bintree_int_skew2 | 0.140 | ok |
| sd_bintree_nat_skew2 | -0.031 | ok |
| sd_chain_int_skew2 | -0.600 | HIGH- (thermal bounce) |
| sd_chain_nat_skew2 | -0.184 | ok |
| sd_chain_rev_int_skew2 | -0.334 | moderate- |
| sd_chain_rev_nat_skew2 | 0.256 | moderate+ |
| sd_evalall_int_skew2 | -0.568 | HIGH- (thermal bounce) |
| sd_evalall_nat_skew2 | 0.046 | ok |
| sd_jumptable_int_skew2 | -0.337 | moderate- |
| sd_jumptable_nat_skew2 | -0.338 | moderate- |
| sd_profiled_int_skew2 | -0.569 | HIGH- (thermal bounce) |
| sd_profiled_nat_skew2 | -0.439 | moderate- |

**Consistency summary:**

- **sd_bintree_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_int_skew2**: won 0/6, lost 6/6
- **sd_chain_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_rev_int_skew2**: won 1/6, lost 5/6
- **sd_chain_rev_nat_skew2**: won 6/6, lost 0/6
- **sd_evalall_int_skew2**: won 6/6, lost 0/6
- **sd_evalall_nat_skew2**: won 6/6, lost 0/6
- **sd_jumptable_int_skew2**: won 6/6, lost 0/6
- **sd_jumptable_nat_skew2**: won 6/6, lost 0/6
- **sd_profiled_int_skew2**: won 1/6, lost 5/6
- **sd_profiled_nat_skew2**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sd_bintree_int_skew2 | 4.6ns | 144872.1ns | 0.0% |  |
| sd_bintree_nat_skew2 | 4.5ns | 62493.2ns | 0.0% |  |
| sd_chain_int_skew2 | 5.1ns | 179778.2ns | 0.0% |  |
| sd_chain_nat_skew2 | 4.1ns | 45042.9ns | 0.0% |  |
| sd_chain_rev_int_skew2 | 5.5ns | 153877.3ns | 0.0% |  |
| sd_chain_rev_nat_skew2 | 3.8ns | 43553.2ns | 0.0% |  |
| sd_evalall_int_skew2 | 5.6ns | 93768.6ns | 0.0% |  |
| sd_evalall_nat_skew2 | 4.4ns | 91245.3ns | 0.0% |  |
| sd_jumptable_int_skew2 | 4.3ns | 44697.2ns | 0.0% |  |
| sd_jumptable_nat_skew2 | 4.5ns | 44753.2ns | 0.0% |  |
| sd_profiled_int_skew2 | 5.7ns | 147813.7ns | 0.0% |  |
| sd_profiled_nat_skew2 | 3.1ns | 45077.4ns | 0.0% |  |

## Distribution (algo ns)

```
sd_bintree_int_skew2 (n=6, range 141556.7-147554.4 ns)
  141556.7 |########################################
  141856.6 |
  142156.5 |########################################
  142456.3 |
  142756.2 |
  143056.1 |
  143356.0 |
  143655.9 |
  143955.8 |
  144255.6 |########################################
  144555.5 |
  144855.4 |
  145155.3 |
  145455.2 |
  145755.1 |########################################
  146054.9 |
  146354.8 |########################################
  146654.7 |
  146954.6 |
  147254.5 |
  (0 below, 1 above range)

sd_bintree_nat_skew2 (n=6, range 59707.9-65022.1 ns)
  59707.9 |####################
  59973.6 |
  60239.3 |####################
  60505.0 |
  60770.7 |
  61036.4 |
  61302.2 |
  61567.9 |####################
  61833.6 |
  62099.3 |
  62365.0 |
  62630.7 |
  62896.4 |
  63162.1 |########################################
  63427.8 |
  63693.6 |
  63959.3 |
  64225.0 |
  64490.7 |
  64756.4 |
  (0 below, 1 above range)

sd_chain_int_skew2 (n=6, range 171307.1-189193.0 ns)
  171307.1 |########################################
  172201.4 |########################################
  173095.7 |
  173990.0 |
  174884.3 |
  175778.6 |
  176672.9 |########################################
  177567.1 |
  178461.4 |
  179355.7 |########################################
  180250.0 |
  181144.3 |
  182038.6 |
  182932.9 |
  183827.2 |
  184721.5 |
  185615.8 |
  186510.1 |
  187404.4 |########################################
  188298.7 |
  (0 below, 1 above range)

sd_chain_nat_skew2 (n=6, range 42788.8-46816.6 ns)
  42788.8 |########################################
  42990.2 |
  43191.6 |
  43393.0 |########################################
  43594.4 |
  43795.8 |########################################
  43997.2 |
  44198.5 |
  44399.9 |
  44601.3 |
  44802.7 |
  45004.1 |
  45205.5 |
  45406.9 |
  45608.3 |
  45809.7 |
  46011.1 |
  46212.5 |########################################
  46413.9 |
  46615.3 |########################################
  (0 below, 1 above range)

sd_chain_rev_int_skew2 (n=6, range 146692.9-159469.5 ns)
  146692.9 |########################################
  147331.7 |
  147970.6 |########################################
  148609.4 |
  149248.2 |
  149887.1 |
  150525.9 |
  151164.7 |########################################
  151803.6 |
  152442.4 |
  153081.2 |
  153720.1 |
  154358.9 |
  154997.7 |
  155636.6 |
  156275.4 |
  156914.2 |
  157553.1 |########################################
  158191.9 |########################################
  158830.7 |
  (0 below, 1 above range)

sd_chain_rev_nat_skew2 (n=6, range 42346.2-44160.4 ns)
  42346.2 |########################################
  42436.9 |
  42527.6 |
  42618.3 |
  42709.0 |
  42799.8 |
  42890.5 |
  42981.2 |
  43071.9 |
  43162.6 |
  43253.3 |########################################
  43344.0 |
  43434.7 |
  43525.4 |
  43616.1 |########################################
  43706.8 |########################################
  43797.6 |
  43888.3 |########################################
  43979.0 |
  44069.7 |
  (0 below, 1 above range)

sd_evalall_int_skew2 (n=6, range 88768.7-99313.9 ns)
  88768.7 |####################
  89296.0 |
  89823.2 |
  90350.5 |####################
  90877.8 |
  91405.0 |
  91932.3 |########################################
  92459.5 |
  92986.8 |
  93514.1 |
  94041.3 |
  94568.6 |
  95095.8 |
  95623.1 |
  96150.4 |
  96677.6 |
  97204.9 |
  97732.2 |####################
  98259.4 |
  98786.7 |
  (0 below, 1 above range)

sd_evalall_nat_skew2 (n=6, range 85939.6-94121.1 ns)
  85939.6 |####################
  86348.7 |
  86757.7 |
  87166.8 |
  87575.9 |
  87985.0 |
  88394.0 |
  88803.1 |
  89212.2 |
  89621.3 |
  90030.3 |
  90439.4 |####################
  90848.5 |
  91257.5 |########################################
  91666.6 |####################
  92075.7 |
  92484.8 |
  92893.8 |
  93302.9 |
  93712.0 |
  (0 below, 1 above range)

sd_jumptable_int_skew2 (n=6, range 41623.8-46792.8 ns)
  41623.8 |####################
  41882.2 |
  42140.7 |####################
  42399.1 |
  42657.6 |
  42916.0 |
  43174.5 |
  43432.9 |
  43691.4 |
  43949.8 |
  44208.3 |
  44466.7 |
  44725.2 |
  44983.6 |
  45242.1 |########################################
  45500.5 |
  45759.0 |
  46017.4 |####################
  46275.9 |
  46534.3 |
  (0 below, 1 above range)

sd_jumptable_nat_skew2 (n=6, range 39776.7-48500.0 ns)
  39776.7 |########################################
  40212.9 |
  40649.0 |
  41085.2 |
  41521.4 |
  41957.5 |
  42393.7 |
  42829.9 |########################################
  43266.0 |
  43702.2 |########################################
  44138.3 |
  44574.5 |########################################
  45010.7 |
  45446.8 |
  45883.0 |
  46319.2 |
  46755.3 |########################################
  47191.5 |
  47627.7 |
  48063.8 |
  (0 below, 1 above range)

sd_profiled_int_skew2 (n=6, range 144052.9-152187.9 ns)
  144052.9 |########################################
  144459.6 |
  144866.4 |
  145273.1 |
  145679.9 |
  146086.6 |
  146493.4 |####################
  146900.1 |
  147306.9 |####################
  147713.6 |
  148120.4 |
  148527.2 |
  148933.9 |
  149340.7 |
  149747.4 |
  150154.2 |
  150560.9 |
  150967.7 |
  151374.4 |####################
  151781.2 |
  (0 below, 1 above range)

sd_profiled_nat_skew2 (n=6, range 42069.2-48499.6 ns)
  42069.2 |########################################
  42390.7 |
  42712.2 |########################################
  43033.8 |
  43355.3 |
  43676.8 |
  43998.3 |########################################
  44319.8 |########################################
  44641.4 |
  44962.9 |
  45284.4 |
  45605.9 |
  45927.4 |
  46249.0 |
  46570.5 |
  46892.0 |
  47213.5 |
  47535.0 |
  47856.6 |
  48178.1 |########################################
  (0 below, 1 above range)

```
