# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_leaf_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_leaf_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_leaf_parse)

The baseline carrier_setup_leaf_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_leaf_parse, carrier_setup_leaf_predecode, carrier_setup_leaf_emitdirect, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} vs {carrier_setup_leaf_optall} (321% apart)

The field splits into a fast tier {carrier_setup_leaf_parse, carrier_setup_leaf_predecode, carrier_setup_leaf_emitdirect, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} and a slow tier {carrier_setup_leaf_optall} with a 321% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 10.37 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_leaf_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 2473472ns | 2468706ns | 2461925ns | 2466878ns | 2489137ns | +108007.10% |
| carrier_setup_leaf_emitdirect | 1287963ns | 1288115ns | 1284862ns | 1287697ns | 1289912ns | +56192.50% |
| carrier_setup_leaf_optall | 10400998ns | 10376336ns | 10327681ns | 10372359ns | 10480614ns | +454492.38% |
| carrier_setup_leaf_parse | 2288ns | 2263ns | 2120ns | 2224ns | 2468ns | base |
| carrier_setup_leaf_predecode | 760822ns | 758825ns | 752565ns | 757663ns | 769690ns | +33152.95% |
| carrier_setup_leaf_stackcompile | 1663242ns | 1668576ns | 1628679ns | 1662550ns | 1681562ns | +72594.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 2470494ns | 2459185ns | 2485802ns | +0.00% | 0.007 |
| carrier_setup_leaf_emitdirect | 1285388ns | 1282470ns | 1287153ns | +0.00% | 0.013 |
| carrier_setup_leaf_optall | 10397751ns | 10324736ns | 10477197ns | +0.00% | 0.002 |
| carrier_setup_leaf_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_leaf_predecode | 758414ns | 750314ns | 767099ns | +0.00% | 0.022 |
| carrier_setup_leaf_stackcompile | 1660787ns | 1626058ns | 1679189ns | +0.00% | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 15462686 | 65947782 | 0.234 | 843.23× |
| carrier_setup_leaf_emitdirect | 8048161 | 31105171 | 0.259 | 438.89× |
| carrier_setup_leaf_optall | 65060944 | 250160169 | 0.260 | 3547.97× |
| carrier_setup_leaf_parse | 18338 | 79566 | 0.230 | 1.00× |
| carrier_setup_leaf_predecode | 4733544 | 19929999 | 0.238 | 258.13× |
| carrier_setup_leaf_stackcompile | 10405420 | 30530990 | 0.341 | 567.44× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_setup_leaf_predecode; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_leaf_emitcopypatch | 0.007 | 30.4% |
| carrier_setup_leaf_emitdirect | 0.013 | 58.4% |
| carrier_setup_leaf_optall | 0.002 | 7.2% |
| carrier_setup_leaf_predecode | 0.022 | 99.2% |
| carrier_setup_leaf_stackcompile | 0.010 | 45.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 2473472ns | 2473472ns | +108007.10% |
| carrier_setup_leaf_emitdirect | 1287963ns | 1287963ns | +56192.50% |
| carrier_setup_leaf_optall | 10400998ns | 10400998ns | +454492.38% |
| carrier_setup_leaf_parse | 2288ns | 2288ns | base |
| carrier_setup_leaf_predecode | 760822ns | 760822ns | +33152.95% |
| carrier_setup_leaf_stackcompile | 1663242ns | 1663242ns | +72594.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_leaf_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_leaf_emitcopypatch | 2465848ns | +2465848.5ns (+0.0%) | [+2459830, +2485802]ns | [2459830, 2485802] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_emitdirect | 1285603ns | +1285603.1ns (+0.0%) | [+1283409, +1287153]ns | [1283409, 1287153] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_optall | 10372926ns | +10372926.1ns (+0.0%) | [+10343130, +10477197]ns | [10343130, 10477197] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_predecode | 756435ns | +756435.2ns (+0.0%) | [+751707, +767099]ns | [751707, 767099] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_stackcompile | 1666063ns | +1666062.7ns (+0.0%) | [+1637109, +1679189]ns | [1637109, 1679189] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_leaf_parse | carrier_setup_leaf_emitcopypatch | carrier_setup_leaf_emitdirect | carrier_setup_leaf_optall | carrier_setup_leaf_predecode | carrier_setup_leaf_stackcompile |
|---|---|---|---|---|---|---|
| 1 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 2 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 3 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 4 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 5 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 6 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_setup_leaf_emitcopypatch | -0.474 | moderate- |
| carrier_setup_leaf_emitdirect | -0.213 | moderate- |
| carrier_setup_leaf_optall | -0.330 | moderate- |
| carrier_setup_leaf_parse | 0.000 | ok |
| carrier_setup_leaf_predecode | 0.132 | ok |
| carrier_setup_leaf_stackcompile | -0.386 | moderate- |

**Consistency summary:**

- **carrier_setup_leaf_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_leaf_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_leaf_optall**: won 0/6, lost 0/6
- **carrier_setup_leaf_predecode**: won 0/6, lost 0/6
- **carrier_setup_leaf_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 2471657.8ns | 2470493.6ns | 100.0% | HIGH |
| carrier_setup_leaf_emitdirect | 1284973.8ns | 1285388.2ns | 100.0% | HIGH |
| carrier_setup_leaf_optall | 10403231.2ns | 10397751.0ns | 100.1% | HIGH |
| carrier_setup_leaf_parse | 2931.9ns | 0.0ns | 0.0% |  |
| carrier_setup_leaf_predecode | 758742.8ns | 758413.7ns | 100.0% | HIGH |
| carrier_setup_leaf_stackcompile | 1661669.1ns | 1660786.9ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_setup_leaf_emitcopypatch (n=6, range 2459185.4-2485802.5 ns)
  2459185.4 |########################################
  2460516.3 |
  2461847.1 |
  2463178.0 |####################
  2464508.8 |
  2465839.7 |
  2467170.5 |####################
  2468501.4 |
  2469832.2 |
  2471163.1 |
  2472494.0 |####################
  2473824.8 |
  2475155.7 |
  2476486.5 |
  2477817.4 |
  2479148.2 |
  2480479.1 |
  2481809.9 |
  2483140.8 |
  2484471.6 |
  (0 below, 1 above range)

carrier_setup_leaf_emitdirect (n=6, range 1282469.6-1287152.9 ns)
  1282469.6 |########################################
  1282703.8 |
  1282937.9 |
  1283172.1 |
  1283406.3 |
  1283640.4 |
  1283874.6 |
  1284108.8 |
  1284342.9 |########################################
  1284577.1 |
  1284811.2 |
  1285045.4 |########################################
  1285279.6 |
  1285513.7 |
  1285747.9 |
  1285982.1 |########################################
  1286216.2 |
  1286450.4 |
  1286684.6 |########################################
  1286918.7 |
  (0 below, 1 above range)

carrier_setup_leaf_optall (n=6, range 10324735.8-10477197.3 ns)
  10324735.8 |########################################
  10332358.9 |
  10339982.0 |
  10347605.0 |
  10355228.1 |########################################
  10362851.2 |########################################
  10370474.2 |########################################
  10378097.3 |
  10385720.4 |
  10393343.5 |
  10400966.6 |
  10408589.6 |########################################
  10416212.7 |
  10423835.8 |
  10431458.9 |
  10439081.9 |
  10446705.0 |
  10454328.1 |
  10461951.2 |
  10469574.2 |
  (0 below, 1 above range)

carrier_setup_leaf_predecode (n=6, range 750313.7-767098.9 ns)
  750313.7 |####################
  751153.0 |
  751992.2 |
  752831.5 |########################################
  753670.8 |
  754510.0 |
  755349.3 |
  756188.5 |
  757027.8 |
  757867.1 |
  758706.3 |####################
  759545.6 |
  760384.8 |
  761224.1 |
  762063.4 |####################
  762902.6 |
  763741.9 |
  764581.2 |
  765420.4 |
  766259.7 |
  (0 below, 1 above range)

carrier_setup_leaf_stackcompile (n=6, range 1626058.3-1679189.4 ns)
  1626058.3 |########################################
  1628714.9 |
  1631371.4 |
  1634028.0 |
  1636684.5 |
  1639341.1 |
  1641997.6 |
  1644654.2 |
  1647310.7 |########################################
  1649967.3 |
  1652623.9 |
  1655280.4 |
  1657937.0 |########################################
  1660593.5 |
  1663250.1 |
  1665906.6 |
  1668563.2 |
  1671219.7 |########################################
  1673876.3 |########################################
  1676532.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_leaf_emitcopypatch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_emitdirect**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_optall**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_predecode**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_stackcompile**: bridge=100.0% of algo (FFI overhead may distort results)
