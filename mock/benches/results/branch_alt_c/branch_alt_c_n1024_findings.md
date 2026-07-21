# Branch strategies, cheap-arm, alt: strict alternation (i&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_alt**

## Key findings

- **Baseline (br_branch_c_alt) is the fastest** at 1244.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1.83x (fastest 1244.6 ns, slowest 2272.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_alt | 3655ns | 3757ns | 3248ns | 3587ns | 3959ns | base |
| br_lut_c_alt | 4671ns | 4726ns | 4085ns | 4675ns | 4958ns | +27.80% |
| br_mask_c_alt | 4640ns | 4597ns | 4312ns | 4503ns | 5010ns | +26.96% |
| br_predicate_c_alt | 4362ns | 4223ns | 3988ns | 4171ns | 4835ns | +19.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_alt | 1206ns | 1077ns | 1294ns | base | 0.849 |
| br_lut_c_alt | 2144ns | 1902ns | 2267ns | +77.80% | 0.478 |
| br_mask_c_alt | 2294ns | 2131ns | 2479ns | +90.24% | 0.446 |
| br_predicate_c_alt | 2030ns | 1859ns | 2250ns | +68.33% | 0.504 |

## Performance model

- Peak throughput: **0.951 Gops/s** (br_branch_c_alt; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_alt | 0.823 | 86.5% |
| br_lut_c_alt | 0.476 | 50.0% |
| br_mask_c_alt | 0.451 | 47.4% |
| br_predicate_c_alt | 0.522 | 54.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_alt | 3655ns | 3655ns | base |
| br_lut_c_alt | 4671ns | 4671ns | +27.80% |
| br_mask_c_alt | 4640ns | 4640ns | +26.96% |
| br_predicate_c_alt | 4362ns | 4362ns | +19.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_alt | 1245ns | base | --- | [1079, 1294] | --- | --- | --- | --- |
| br_lut_c_alt | 2151ns | +924.8ns (+74.3%) | [+836, +1054]ns | [2014, 2267] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_alt | 2272ns | +1065.6ns (+85.6%) | [+1008, +1191]ns | [2131, 2479] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_alt | 1962ns | +883.8ns (+71.0%) | [+626, +962]ns | [1877, 2250] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_alt | br_lut_c_alt | br_mask_c_alt | br_predicate_c_alt |
|---|---|---|---|---|
| 1 | 1299ns | +67.1% | +82.8% | +43.1% |
| 2 | 1290ns | +75.9% | +97.1% | +74.1% |
| 3 | 1286ns | +65.8% | +87.8% | +75.3% |
| 4 | 1081ns | +96.6% | +97.2% | +81.7% |
| 5 | 1077ns | +76.7% | +98.0% | +82.2% |
| 6 | 1203ns | +88.4% | +80.3% | +57.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_alt | 0.386 | moderate+ |
| br_lut_c_alt | -0.256 | moderate- |
| br_mask_c_alt | 0.510 | HIGH+ (drift/warm-up) |
| br_predicate_c_alt | 0.067 | ok |

**Consistency summary:**

- **br_lut_c_alt**: won 0/6, lost 6/6
- **br_mask_c_alt**: won 0/6, lost 6/6
- **br_predicate_c_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_alt | 4.2ns | 1205.9ns | 0.3% |  |
| br_lut_c_alt | 3.6ns | 2144.0ns | 0.2% |  |
| br_mask_c_alt | 3.5ns | 2294.2ns | 0.2% |  |
| br_predicate_c_alt | 3.5ns | 2029.9ns | 0.2% |  |

## Distribution (algo ns)

```
br_branch_c_alt (n=6, range 1076.7-1294.4 ns)
   1076.7 |########################################
   1087.6 |
   1098.5 |
   1109.4 |
   1120.2 |
   1131.1 |
   1142.0 |
   1152.9 |
   1163.8 |
   1174.7 |
   1185.6 |
   1196.4 |####################
   1207.3 |
   1218.2 |
   1229.1 |
   1240.0 |
   1250.9 |
   1261.7 |
   1272.6 |
   1283.5 |########################################
  (0 below, 1 above range)

br_lut_c_alt (n=6, range 1902.1-2267.3 ns)
   1902.1 |####################
   1920.4 |
   1938.6 |
   1956.9 |
   1975.1 |
   1993.4 |
   2011.7 |
   2029.9 |
   2048.2 |
   2066.4 |
   2084.7 |
   2103.0 |
   2121.2 |########################################
   2139.5 |
   2157.7 |####################
   2176.0 |
   2194.3 |
   2212.5 |
   2230.8 |
   2249.0 |####################
  (0 below, 1 above range)

br_mask_c_alt (n=6, range 2131.2-2478.9 ns)
   2131.2 |########################################
   2148.6 |
   2166.0 |####################
   2183.4 |
   2200.8 |
   2218.1 |
   2235.5 |
   2252.9 |
   2270.3 |
   2287.7 |
   2305.1 |
   2322.5 |
   2339.8 |
   2357.2 |
   2374.6 |####################
   2392.0 |
   2409.4 |####################
   2426.8 |
   2444.2 |
   2461.6 |
  (0 below, 1 above range)

br_predicate_c_alt (n=6, range 1858.7-2250.0 ns)
   1858.7 |####################
   1878.3 |####################
   1897.8 |
   1917.4 |
   1937.0 |
   1956.5 |########################################
   1976.1 |
   1995.7 |
   2015.2 |
   2034.8 |
   2054.3 |
   2073.9 |
   2093.5 |
   2113.0 |
   2132.6 |
   2152.2 |
   2171.7 |
   2191.3 |
   2210.9 |
   2230.4 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **br_mask_c_alt**: autocorrelation=0.51 (measurement drift or warm-up artifact)
