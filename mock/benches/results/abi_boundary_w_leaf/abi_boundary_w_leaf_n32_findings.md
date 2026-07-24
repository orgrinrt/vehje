# abi_boundary_w (leaf)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_leaf_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_leaf_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_leaf_null_entry dominates: 34919% faster than the next best (abi_boundary_w_leaf_soa_per_w)

abi_boundary_w_leaf_null_entry (2.33 us) leads abi_boundary_w_leaf_soa_per_w (817.50 us) by 34919%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_leaf_null_entry beats baseline by 100% (significant)

abi_boundary_w_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_boundary_w_leaf_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_leaf_scalar_per_w is an outlier: 622.1x slower than the field

abi_boundary_w_leaf_scalar_per_w (1.45 ms) is 622.1x the fastest (2.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_leaf_null_entry} vs {abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_scalar_per_w} (34919% apart)

The field splits into a fast tier {abi_boundary_w_leaf_null_entry} and a slow tier {abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_scalar_per_w} with a 34919% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 622.1x the fastest

Fastest abi_boundary_w_leaf_null_entry (2.33 us) to slowest abi_boundary_w_leaf_scalar_per_w (1.45 ms): 622.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_boundary_w_leaf_scalar_dispatch's edge over baseline is significant but tiny (39 ns, 0.00%)

abi_boundary_w_leaf_scalar_dispatch differs from baseline abi_boundary_w_leaf_scalar_runtime_w by 39 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_boundary_w_leaf_null_entry** at 2334.4 ns median (-99.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 622.13x (fastest 2334.4 ns, slowest 1452295.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 4635ns | 4624ns | 4411ns | 4588ns | 4818ns | -99.68% |
| abi_boundary_w_leaf_scalar_anchor | 1454715ns | 1453392ns | 1448284ns | 1452047ns | 1461931ns | -0.07% |
| abi_boundary_w_leaf_scalar_dispatch | 1454353ns | 1453651ns | 1450809ns | 1453341ns | 1457645ns | -0.10% |
| abi_boundary_w_leaf_scalar_per_w | 1465620ns | 1454854ns | 1450203ns | 1453582ns | 1491385ns | +0.68% |
| abi_boundary_w_leaf_scalar_runtime_w | 1455758ns | 1454811ns | 1449449ns | 1453082ns | 1462928ns | base |
| abi_boundary_w_leaf_soa_dispatch | 822040ns | 822291ns | 819271ns | 821637ns | 824029ns | -43.53% |
| abi_boundary_w_leaf_soa_per_w | 823451ns | 819889ns | 818272ns | 819453ns | 832036ns | -43.43% |
| abi_boundary_w_leaf_soa_runtime_w | 821035ns | 820267ns | 818467ns | 819821ns | 824140ns | -43.60% |
| abi_boundary_w_leaf_zig_runtime_w | 1436347ns | 1435899ns | 1431898ns | 1435375ns | 1440029ns | -1.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 2336ns | 2212ns | 2436ns | -99.84% | 0.014 |
| abi_boundary_w_leaf_scalar_anchor | 1452087ns | 1445795ns | 1459155ns | -0.08% | 0.000 |
| abi_boundary_w_leaf_scalar_dispatch | 1451867ns | 1448370ns | 1455158ns | -0.09% | 0.000 |
| abi_boundary_w_leaf_scalar_per_w | 1462965ns | 1447677ns | 1488419ns | +0.67% | 0.000 |
| abi_boundary_w_leaf_scalar_runtime_w | 1453200ns | 1447154ns | 1460118ns | base | 0.000 |
| abi_boundary_w_leaf_soa_dispatch | 819612ns | 816978ns | 821477ns | -43.60% | 0.000 |
| abi_boundary_w_leaf_soa_per_w | 821050ns | 816030ns | 829513ns | -43.50% | 0.000 |
| abi_boundary_w_leaf_soa_runtime_w | 818643ns | 816004ns | 821675ns | -43.67% | 0.000 |
| abi_boundary_w_leaf_zig_runtime_w | 1433699ns | 1429363ns | 1437064ns | -1.34% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 27610.6 | 2453.1 | 2335.7 | n/a |
| abi_boundary_w_leaf_scalar_anchor | 39040.5 | 1450888.5 | 1452087.2 | 1 |
| abi_boundary_w_leaf_scalar_dispatch | 35259.1 | 1451991.5 | 1451867.4 | n/a |
| abi_boundary_w_leaf_scalar_per_w | 39867.2 | 1464647.8 | 1462965.3 | n/a |
| abi_boundary_w_leaf_scalar_runtime_w | 37722.5 | 1452457.4 | 1453199.7 | n/a |
| abi_boundary_w_leaf_soa_dispatch | 33420.0 | 819813.6 | 819611.7 | n/a |
| abi_boundary_w_leaf_soa_per_w | 33323.5 | 820691.8 | 821049.8 | n/a |
| abi_boundary_w_leaf_soa_runtime_w | 31780.0 | 818711.2 | 818643.2 | n/a |
| abi_boundary_w_leaf_zig_runtime_w | 178212.4 | 1433139.3 | 1433698.7 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_boundary_w_leaf_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.014 | 94.8% |
| abi_boundary_w_leaf_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_leaf_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_leaf_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_leaf_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 4635ns | 4635ns | -99.68% |
| abi_boundary_w_leaf_scalar_anchor | 1454715ns | 1454715ns | -0.07% |
| abi_boundary_w_leaf_scalar_dispatch | 1454353ns | 1454353ns | -0.10% |
| abi_boundary_w_leaf_scalar_per_w | 1465620ns | 1465620ns | +0.68% |
| abi_boundary_w_leaf_scalar_runtime_w | 1455758ns | 1455758ns | base |
| abi_boundary_w_leaf_soa_dispatch | 822040ns | 822040ns | -43.53% |
| abi_boundary_w_leaf_soa_per_w | 823451ns | 823451ns | -43.43% |
| abi_boundary_w_leaf_soa_runtime_w | 821035ns | 821035ns | -43.60% |
| abi_boundary_w_leaf_zig_runtime_w | 1436347ns | 1436347ns | -1.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_scalar_runtime_w | 1452227ns | base | --- | [1447254, 1460118] | --- | --- | --- | --- |
| abi_boundary_w_leaf_null_entry | 2334ns | -1449963.3ns (-99.8%) | [-1457729, -1444900]ns | [2237, 2436] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_anchor | 1450691ns | no significant difference | [-3484, +1047]ns | [1446415, 1459155] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 1451155ns | no significant difference | [-8122, +4085]ns | [1449290, 1455158] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 1452296ns | no significant difference | [-8703, +38188]ns | [1448181, 1488419] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_leaf_soa_dispatch | 819845ns | -631798.8ns (-43.5%) | [-641247, -627719]ns | [817512, 821477] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_per_w | 817495ns | -633210.2ns (-43.6%) | [-642551, -620688]ns | [816141, 829513] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_runtime_w | 817946ns | -633246.8ns (-43.6%) | [-640053, -630370]ns | [816309, 821675] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_zig_runtime_w | 1433427ns | -19319.6ns (-1.3%) | [-23252, -15931]ns | [1430605, 1437064] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_leaf_scalar_runtime_w | abi_boundary_w_leaf_null_entry | abi_boundary_w_leaf_scalar_anchor | abi_boundary_w_leaf_scalar_dispatch | abi_boundary_w_leaf_scalar_per_w | abi_boundary_w_leaf_soa_dispatch | abi_boundary_w_leaf_soa_per_w | abi_boundary_w_leaf_soa_runtime_w | abi_boundary_w_leaf_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1464993ns | -99.8% | -0.0% | -1.0% | -0.7% | -44.0% | -44.1% | -43.9% | -1.7% |
| 2 | 1453106ns | -99.8% | -0.3% | +0.3% | +4.7% | -43.4% | -42.2% | -43.4% | -1.3% |
| 3 | 1451347ns | -99.8% | +0.2% | -0.1% | -0.1% | -43.6% | -43.8% | -43.8% | -1.3% |
| 4 | 1455243ns | -99.8% | -0.2% | -0.1% | -0.5% | -43.9% | -43.9% | -43.7% | -1.5% |
| 5 | 1447154ns | -99.8% | -0.1% | +0.1% | +0.0% | -43.5% | -43.4% | -43.5% | -1.2% |
| 6 | 1447355ns | -99.8% | -0.0% | +0.3% | +0.6% | -43.3% | -43.6% | -43.6% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_leaf_null_entry | -0.169 | ok |
| abi_boundary_w_leaf_scalar_anchor | -0.036 | ok |
| abi_boundary_w_leaf_scalar_dispatch | -0.456 | moderate- |
| abi_boundary_w_leaf_scalar_per_w | -0.178 | ok |
| abi_boundary_w_leaf_scalar_runtime_w | 0.084 | ok |
| abi_boundary_w_leaf_soa_dispatch | 0.259 | moderate+ |
| abi_boundary_w_leaf_soa_per_w | -0.203 | moderate- |
| abi_boundary_w_leaf_soa_runtime_w | 0.094 | ok |
| abi_boundary_w_leaf_zig_runtime_w | 0.060 | ok |

**Consistency summary:**

- **abi_boundary_w_leaf_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_anchor**: won 2/6, lost 1/6
- **abi_boundary_w_leaf_scalar_dispatch**: won 2/6, lost 2/6
- **abi_boundary_w_leaf_scalar_per_w**: won 2/6, lost 2/6
- **abi_boundary_w_leaf_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 117569.0ns | 2335.7ns | 5033.5% | HIGH |
| abi_boundary_w_leaf_scalar_anchor | 4397179.6ns | 1452087.2ns | 302.8% | HIGH |
| abi_boundary_w_leaf_scalar_dispatch | 4394267.5ns | 1451867.4ns | 302.7% | HIGH |
| abi_boundary_w_leaf_scalar_per_w | 4429354.7ns | 1462965.3ns | 302.8% | HIGH |
| abi_boundary_w_leaf_scalar_runtime_w | 4398234.2ns | 1453199.7ns | 302.7% | HIGH |
| abi_boundary_w_leaf_soa_dispatch | 2495326.9ns | 819611.7ns | 304.5% | HIGH |
| abi_boundary_w_leaf_soa_per_w | 2497904.6ns | 821049.8ns | 304.2% | HIGH |
| abi_boundary_w_leaf_soa_runtime_w | 2488572.0ns | 818643.2ns | 304.0% | HIGH |
| abi_boundary_w_leaf_zig_runtime_w | 4546370.3ns | 1433698.7ns | 317.1% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_leaf_null_entry (n=6, range 2212.5-2435.7 ns)
   2212.5 |########################################
   2223.7 |
   2234.8 |
   2246.0 |
   2257.1 |########################################
   2268.3 |
   2279.4 |
   2290.6 |
   2301.8 |
   2312.9 |########################################
   2324.1 |
   2335.2 |
   2346.4 |########################################
   2357.5 |
   2368.7 |
   2379.9 |
   2391.0 |
   2402.2 |
   2413.3 |########################################
   2424.5 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_anchor (n=6, range 1445794.6-1459155.2 ns)
  1445794.6 |########################################
  1446462.6 |########################################
  1447130.7 |
  1447798.7 |
  1448466.7 |
  1449134.8 |########################################
  1449802.8 |
  1450470.8 |
  1451138.8 |
  1451806.9 |########################################
  1452474.9 |
  1453142.9 |########################################
  1453811.0 |
  1454479.0 |
  1455147.0 |
  1455815.1 |
  1456483.1 |
  1457151.1 |
  1457819.1 |
  1458487.2 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_dispatch (n=6, range 1448369.6-1455157.5 ns)
  1448369.6 |########################################
  1448709.0 |
  1449048.4 |
  1449387.8 |
  1449727.2 |
  1450066.6 |########################################
  1450406.0 |########################################
  1450745.4 |
  1451084.8 |
  1451424.2 |########################################
  1451763.6 |
  1452102.9 |
  1452442.3 |
  1452781.7 |
  1453121.1 |########################################
  1453460.5 |
  1453799.9 |
  1454139.3 |
  1454478.7 |
  1454818.1 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_per_w (n=6, range 1447677.1-1488418.9 ns)
  1447677.1 |########################################
  1449714.2 |####################
  1451751.3 |
  1453788.4 |########################################
  1455825.5 |
  1457862.6 |
  1459899.7 |
  1461936.7 |
  1463973.8 |
  1466010.9 |
  1468048.0 |
  1470085.1 |
  1472122.2 |
  1474159.3 |
  1476196.4 |
  1478233.5 |
  1480270.6 |
  1482307.7 |
  1484344.8 |
  1486381.9 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_runtime_w (n=6, range 1447153.8-1460118.1 ns)
  1447153.8 |########################################
  1447802.0 |
  1448450.2 |
  1449098.4 |
  1449746.7 |
  1450394.9 |
  1451043.1 |####################
  1451691.3 |
  1452339.5 |
  1452987.7 |####################
  1453636.0 |
  1454284.2 |
  1454932.4 |####################
  1455580.6 |
  1456228.8 |
  1456877.0 |
  1457525.2 |
  1458173.5 |
  1458821.7 |
  1459469.9 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_dispatch (n=6, range 816978.3-821477.1 ns)
  816978.3 |########################################
  817203.2 |
  817428.2 |
  817653.1 |
  817878.1 |########################################
  818103.0 |
  818327.9 |
  818552.9 |
  818777.8 |########################################
  819002.8 |
  819227.7 |
  819452.6 |
  819677.6 |
  819902.5 |
  820127.5 |
  820352.4 |
  820577.3 |########################################
  820802.3 |########################################
  821027.2 |
  821252.2 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_per_w (n=6, range 816029.6-829513.3 ns)
  816029.6 |########################################
  816703.8 |
  817378.0 |
  818052.2 |##########################
  818726.3 |
  819400.5 |
  820074.7 |
  820748.9 |
  821423.1 |
  822097.3 |
  822771.4 |
  823445.6 |
  824119.8 |
  824794.0 |
  825468.2 |
  826142.4 |
  826816.6 |
  827490.7 |
  828164.9 |
  828839.1 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_runtime_w (n=6, range 816003.8-821674.8 ns)
  816003.8 |########################################
  816287.4 |
  816570.9 |########################################
  816854.5 |
  817138.0 |########################################
  817421.6 |
  817705.1 |
  817988.7 |
  818272.2 |
  818555.8 |########################################
  818839.3 |
  819122.9 |
  819406.4 |
  819690.0 |
  819973.5 |
  820257.1 |
  820540.6 |
  820824.2 |
  821107.7 |
  821391.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_zig_runtime_w (n=6, range 1429363.3-1437063.9 ns)
  1429363.3 |####################
  1429748.3 |
  1430133.4 |
  1430518.4 |
  1430903.4 |
  1431288.5 |
  1431673.5 |####################
  1432058.5 |
  1432443.6 |
  1432828.6 |
  1433213.6 |########################################
  1433598.7 |####################
  1433983.7 |
  1434368.7 |
  1434753.8 |
  1435138.8 |
  1435523.8 |
  1435908.9 |
  1436293.9 |
  1436678.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_leaf_null_entry**: bridge=5010.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_anchor**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_dispatch**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_runtime_w**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_dispatch**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_per_w**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_runtime_w**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_zig_runtime_w**: bridge=316.2% of algo (FFI overhead may distort results)
