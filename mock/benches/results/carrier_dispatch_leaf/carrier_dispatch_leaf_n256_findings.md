# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 30% faster than the next best (carrier_disp_leaf_ifchain)

carrier_disp_leaf_nullfloor (7.43 us) leads carrier_disp_leaf_ifchain (9.69 us) by 30%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 25% (significant)

carrier_disp_leaf_nullfloor is -2.48 us (25%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_ifchain, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_bittree, carrier_disp_leaf_threaded, carrier_disp_leaf_fntable, carrier_disp_leaf_ifchainlin} (30% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_ifchain, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_bittree, carrier_disp_leaf_threaded, carrier_disp_leaf_fntable, carrier_disp_leaf_ifchainlin} with a 30% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 7432.3 ns median (-25.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.73x (fastest 7432.3 ns, slowest 12846.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 13072ns | 13081ns | 12867ns | 13068ns | 13180ns | +2.87% |
| carrier_disp_leaf_fntable | 15023ns | 15140ns | 13769ns | 15078ns | 15567ns | +18.22% |
| carrier_disp_leaf_ifchain | 12307ns | 12283ns | 11832ns | 12201ns | 12702ns | -3.15% |
| carrier_disp_leaf_ifchainasc | 12551ns | 12755ns | 10881ns | 12533ns | 13412ns | -1.23% |
| carrier_disp_leaf_ifchainlin | 15400ns | 15404ns | 15195ns | 15353ns | 15571ns | +21.19% |
| carrier_disp_leaf_nullfloor | 10047ns | 10020ns | 9885ns | 9976ns | 10236ns | -20.93% |
| carrier_disp_leaf_switch | 12707ns | 12581ns | 12140ns | 12446ns | 13383ns | base |
| carrier_disp_leaf_threaded | 14373ns | 14342ns | 14085ns | 14307ns | 14617ns | +13.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 10478ns | 10410ns | 10535ns | +4.28% | 0.024 |
| carrier_disp_leaf_fntable | 12443ns | 11387ns | 12893ns | +23.84% | 0.021 |
| carrier_disp_leaf_ifchain | 9761ns | 9421ns | 10102ns | -2.85% | 0.026 |
| carrier_disp_leaf_ifchainasc | 10019ns | 8741ns | 10806ns | -0.28% | 0.026 |
| carrier_disp_leaf_ifchainlin | 12851ns | 12734ns | 12968ns | +27.91% | 0.020 |
| carrier_disp_leaf_nullfloor | 7458ns | 7267ns | 7613ns | -25.78% | 0.034 |
| carrier_disp_leaf_switch | 10047ns | 9570ns | 10598ns | base | 0.025 |
| carrier_disp_leaf_threaded | 11783ns | 11630ns | 11942ns | +17.27% | 0.022 |

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.024 | 69.3% |
| carrier_disp_leaf_fntable | 0.020 | 57.8% |
| carrier_disp_leaf_ifchain | 0.026 | 75.0% |
| carrier_disp_leaf_ifchainasc | 0.025 | 72.0% |
| carrier_disp_leaf_ifchainlin | 0.020 | 56.6% |
| carrier_disp_leaf_nullfloor | 0.034 | 97.8% |
| carrier_disp_leaf_switch | 0.026 | 73.0% |
| carrier_disp_leaf_threaded | 0.022 | 61.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 13072ns | 13072ns | +2.87% |
| carrier_disp_leaf_fntable | 15023ns | 15023ns | +18.22% |
| carrier_disp_leaf_ifchain | 12307ns | 12307ns | -3.15% |
| carrier_disp_leaf_ifchainasc | 12551ns | 12551ns | -1.23% |
| carrier_disp_leaf_ifchainlin | 15400ns | 15400ns | +21.19% |
| carrier_disp_leaf_nullfloor | 10047ns | 10047ns | -20.93% |
| carrier_disp_leaf_switch | 12707ns | 12707ns | base |
| carrier_disp_leaf_threaded | 14373ns | 14373ns | +13.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 9961ns | base | --- | [9584, 10598] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 10482ns | no significant difference | [-108, +942]ns | [10417, 10535] | no | 0.3063 | 0.2188 | 0 |
| carrier_disp_leaf_fntable | 12578ns | +2536.5ns (+25.5%) | [+1638, +3013]ns | [11858, 12893] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 9691ns | no significant difference | [-981, +483]ns | [9488, 10102] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_leaf_ifchainasc | 10098ns | no significant difference | [-1057, +673]ns | [9152, 10806] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_leaf_ifchainlin | 12847ns | +2878.8ns (+28.9%) | [+2227, +3306]ns | [12739, 12968] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_leaf_nullfloor | 7432ns | -2478.1ns (-24.9%) | [-3224, -2068]ns | [7327, 7613] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 11767ns | +1863.4ns (+18.7%) | [+1135, +2208]ns | [11640, 11942] | YES (adj: no) | 0.0547 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 10822ns | -3.1% | +20.9% | -9.3% | -19.2% | +17.7% | -32.8% | +8.5% |
| 2 | 10375ns | +1.2% | +9.8% | -9.2% | +3.7% | +24.5% | -26.3% | +16.1% |
| 3 | 9570ns | +10.5% | +30.5% | +0.0% | +8.1% | +33.2% | -20.8% | +23.2% |
| 4 | 9641ns | +8.1% | +27.9% | +3.7% | +2.2% | +33.1% | -23.1% | +22.8% |
| 5 | 10280ns | +1.3% | +23.2% | -7.0% | +5.5% | +25.1% | -28.1% | +13.1% |
| 6 | 9598ns | +9.2% | +32.3% | +6.3% | -0.4% | +35.7% | -22.3% | +21.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | 0.013 | ok |
| carrier_disp_leaf_fntable | -0.423 | moderate- |
| carrier_disp_leaf_ifchain | -0.307 | moderate- |
| carrier_disp_leaf_ifchainasc | -0.398 | moderate- |
| carrier_disp_leaf_ifchainlin | -0.179 | ok |
| carrier_disp_leaf_nullfloor | -0.168 | ok |
| carrier_disp_leaf_switch | 0.068 | ok |
| carrier_disp_leaf_threaded | 0.036 | ok |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 1/6, lost 5/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 3/6, lost 2/6
- **carrier_disp_leaf_ifchainasc**: won 2/6, lost 4/6
- **carrier_disp_leaf_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 91734.0ns | 10477.8ns | 875.5% | HIGH |
| carrier_disp_leaf_fntable | 89654.0ns | 12443.1ns | 720.5% | HIGH |
| carrier_disp_leaf_ifchain | 92901.2ns | 9760.6ns | 951.8% | HIGH |
| carrier_disp_leaf_ifchainasc | 91584.1ns | 10018.9ns | 914.1% | HIGH |
| carrier_disp_leaf_ifchainlin | 90176.8ns | 12851.4ns | 701.7% | HIGH |
| carrier_disp_leaf_nullfloor | 88782.9ns | 7457.6ns | 1190.5% | HIGH |
| carrier_disp_leaf_switch | 93021.8ns | 10047.4ns | 925.8% | HIGH |
| carrier_disp_leaf_threaded | 91427.8ns | 11783.1ns | 775.9% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 10410.4-10534.6 ns)
  10410.4 |########################################
  10416.6 |
  10422.8 |########################################
  10429.0 |
  10435.2 |
  10441.5 |
  10447.7 |
  10453.9 |
  10460.1 |
  10466.3 |
  10472.5 |########################################
  10478.7 |
  10484.9 |########################################
  10491.1 |########################################
  10497.3 |
  10503.5 |
  10509.8 |
  10516.0 |
  10522.2 |
  10528.4 |
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 11386.7-12893.1 ns)
  11386.7 |########################################
  11462.0 |
  11537.3 |
  11612.7 |
  11688.0 |
  11763.3 |
  11838.6 |
  11913.9 |
  11989.3 |
  12064.6 |
  12139.9 |
  12215.2 |
  12290.5 |########################################
  12365.9 |
  12441.2 |########################################
  12516.5 |
  12591.8 |########################################
  12667.1 |########################################
  12742.5 |
  12817.8 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 9420.8-10101.9 ns)
   9420.8 |########################################
   9454.9 |
   9488.9 |
   9523.0 |########################################
   9557.0 |########################################
   9591.1 |
   9625.1 |
   9659.2 |
   9693.2 |
   9727.3 |
   9761.4 |
   9795.4 |########################################
   9829.5 |
   9863.5 |
   9897.6 |
   9931.6 |
   9965.7 |########################################
   9999.7 |
  10033.8 |
  10067.8 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 8741.2-10806.2 ns)
   8741.2 |########################################
   8844.5 |
   8947.7 |
   9051.0 |
   9154.2 |
   9257.5 |
   9360.7 |
   9464.0 |########################################
   9567.2 |
   9670.5 |
   9773.7 |########################################
   9877.0 |
   9980.2 |
  10083.5 |
  10186.7 |
  10290.0 |########################################
  10393.2 |
  10496.5 |
  10599.7 |
  10703.0 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 12733.8-12968.5 ns)
  12733.8 |########################################
  12745.5 |
  12757.3 |
  12769.0 |
  12780.7 |
  12792.5 |
  12804.2 |
  12815.9 |
  12827.7 |####################
  12839.4 |
  12851.1 |
  12862.9 |####################
  12874.6 |
  12886.4 |
  12898.1 |
  12909.8 |####################
  12921.6 |
  12933.3 |
  12945.0 |
  12956.8 |
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 7267.1-7613.1 ns)
   7267.1 |########################################
   7284.4 |
   7301.7 |
   7319.0 |
   7336.3 |
   7353.6 |
   7370.9 |########################################
   7388.2 |
   7405.5 |########################################
   7422.8 |
   7440.1 |########################################
   7457.4 |
   7474.7 |
   7492.0 |
   7509.3 |
   7526.6 |
   7543.9 |
   7561.2 |########################################
   7578.5 |
   7595.8 |
  (0 below, 1 above range)

carrier_disp_leaf_switch (n=6, range 9569.6-10598.2 ns)
   9569.6 |########################################
   9621.0 |####################
   9672.5 |
   9723.9 |
   9775.3 |
   9826.7 |
   9878.2 |
   9929.6 |
   9981.0 |
  10032.4 |
  10083.9 |
  10135.3 |
  10186.7 |
  10238.2 |####################
  10289.6 |
  10341.0 |####################
  10392.4 |
  10443.9 |
  10495.3 |
  10546.7 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 11630.0-11942.5 ns)
  11630.0 |########################################
  11645.6 |########################################
  11661.2 |
  11676.9 |
  11692.5 |
  11708.1 |
  11723.8 |
  11739.4 |########################################
  11755.0 |
  11770.6 |
  11786.2 |########################################
  11801.9 |
  11817.5 |
  11833.1 |########################################
  11848.8 |
  11864.4 |
  11880.0 |
  11895.6 |
  11911.2 |
  11926.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=877.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=711.5% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=967.9% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=910.0% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=700.9% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=1196.7% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=941.9% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=779.1% of algo (FFI overhead may distort results)
