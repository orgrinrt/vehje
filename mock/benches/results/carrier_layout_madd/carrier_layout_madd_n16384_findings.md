# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.34 us) is smaller than the fastest variant's own run-to-run std-dev (2.39 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 5 variants sit between 769.23 us and 771.57 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_madd_rec12** at 769233.6 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 769233.6 ns, slowest 771573.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 772608ns | 772503ns | 768998ns | 772196ns | 775032ns | -0.23% |
| carrier_lay_madd_rec16 | 772918ns | 772921ns | 770617ns | 772546ns | 774629ns | -0.19% |
| carrier_lay_madd_rec20 | 773466ns | 774317ns | 764798ns | 773520ns | 777720ns | -0.12% |
| carrier_lay_madd_rec24 | 774422ns | 773478ns | 768500ns | 772962ns | 779572ns | base |
| carrier_lay_madd_rec32 | 773127ns | 774688ns | 763132ns | 774619ns | 775886ns | -0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 769516ns | 765882ns | 772173ns | -0.23% | 0.021 |
| carrier_lay_madd_rec16 | 769476ns | 767346ns | 771300ns | -0.23% | 0.021 |
| carrier_lay_madd_rec20 | 770470ns | 761377ns | 774865ns | -0.10% | 0.021 |
| carrier_lay_madd_rec24 | 771256ns | 765227ns | 776312ns | base | 0.021 |
| carrier_lay_madd_rec32 | 770054ns | 760020ns | 773042ns | -0.16% | 0.021 |

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_lay_madd_rec32; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.021 | 98.8% |
| carrier_lay_madd_rec16 | 0.021 | 98.8% |
| carrier_lay_madd_rec20 | 0.021 | 98.5% |
| carrier_lay_madd_rec24 | 0.021 | 98.6% |
| carrier_lay_madd_rec32 | 0.021 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 772608ns | 772608ns | -0.23% |
| carrier_lay_madd_rec16 | 772918ns | 772918ns | -0.19% |
| carrier_lay_madd_rec20 | 773466ns | 773466ns | -0.12% |
| carrier_lay_madd_rec24 | 774422ns | 774422ns | base |
| carrier_lay_madd_rec32 | 773127ns | 773127ns | -0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 770539ns | base | --- | [766916, 776312] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 769234ns | no significant difference | [-8956, +4789]ns | [767142, 772173] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_madd_rec16 | 769587ns | no significant difference | [-8077, +2220]ns | [767540, 771300] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_madd_rec20 | 771360ns | no significant difference | [-6880, +4290]ns | [765185, 774865] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_madd_rec32 | 771573ns | no significant difference | [-10765, +6126]ns | [765548, 773042] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 765227ns | +1.1% | +0.3% | +0.9% | +0.9% |
| 2 | 769755ns | +0.1% | +0.0% | -1.1% | +0.2% |
| 3 | 776155ns | -1.3% | -1.1% | -0.7% | -0.7% |
| 4 | 776470ns | -1.0% | -0.9% | +0.0% | -2.1% |
| 5 | 771322ns | -0.4% | +0.1% | +0.2% | +0.0% |
| 6 | 768605ns | +0.1% | +0.3% | +0.1% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | 0.111 | ok |
| carrier_lay_madd_rec16 | 0.023 | ok |
| carrier_lay_madd_rec20 | -0.014 | ok |
| carrier_lay_madd_rec24 | 0.283 | moderate+ |
| carrier_lay_madd_rec32 | -0.111 | ok |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 3/6, lost 3/6
- **carrier_lay_madd_rec16**: won 2/6, lost 2/6
- **carrier_lay_madd_rec20**: won 2/6, lost 2/6
- **carrier_lay_madd_rec32**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 772052.9ns | 769516.0ns | 100.3% | HIGH |
| carrier_lay_madd_rec16 | 771732.7ns | 769475.9ns | 100.3% | HIGH |
| carrier_lay_madd_rec20 | 772759.5ns | 770469.9ns | 100.3% | HIGH |
| carrier_lay_madd_rec24 | 773976.9ns | 771255.8ns | 100.4% | HIGH |
| carrier_lay_madd_rec32 | 772973.1ns | 770054.3ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 765882.1-772172.7 ns)
  765882.1 |########################################
  766196.6 |
  766511.2 |
  766825.7 |
  767140.2 |
  767454.8 |
  767769.3 |
  768083.8 |
  768398.3 |########################################
  768712.9 |########################################
  769027.4 |
  769341.9 |########################################
  769656.5 |
  769971.0 |
  770285.5 |########################################
  770600.0 |
  770914.6 |
  771229.1 |
  771543.6 |
  771858.2 |
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 767345.8-771300.4 ns)
  767345.8 |########################################
  767543.5 |########################################
  767741.3 |
  767939.0 |
  768136.7 |
  768334.5 |
  768532.2 |
  768729.9 |
  768927.7 |########################################
  769125.4 |
  769323.1 |
  769520.9 |
  769718.6 |
  769916.3 |########################################
  770114.1 |
  770311.8 |
  770509.5 |########################################
  770707.3 |
  770905.0 |
  771102.7 |
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 761376.7-774864.8 ns)
  761376.7 |########################################
  762051.1 |
  762725.5 |
  763399.9 |
  764074.3 |
  764748.7 |
  765423.1 |
  766097.5 |
  766771.9 |
  767446.3 |
  768120.7 |
  768795.1 |########################################
  769469.5 |
  770143.9 |########################################
  770818.3 |
  771492.7 |########################################
  772167.1 |
  772841.5 |########################################
  773515.9 |
  774190.3 |
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 765226.7-776312.5 ns)
  765226.7 |########################################
  765781.0 |
  766335.3 |
  766889.6 |
  767443.9 |
  767998.1 |
  768552.4 |########################################
  769106.7 |
  769661.0 |########################################
  770215.3 |
  770769.6 |########################################
  771323.9 |
  771878.2 |
  772432.5 |
  772986.8 |
  773541.1 |
  774095.3 |
  774649.6 |
  775203.9 |
  775758.2 |########################################
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 760019.6-773042.1 ns)
  760019.6 |####################
  760670.7 |
  761321.8 |
  761973.0 |
  762624.1 |
  763275.2 |
  763926.3 |
  764577.5 |
  765228.6 |
  765879.7 |
  766530.8 |
  767181.9 |
  767833.1 |
  768484.2 |
  769135.3 |
  769786.4 |
  770437.6 |####################
  771088.7 |########################################
  771739.8 |####################
  772390.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=100.4% of algo (FFI overhead may distort results)
