# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_tight_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_tight_none has the worst median (703.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_tight_all at 31.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_tight_all dominates: 1211% faster than the next best (carrier_opt_tight_fold)

carrier_opt_tight_all (31.19 us) leads carrier_opt_tight_fold (408.99 us) by 1211%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 96% (significant)

carrier_opt_tight_all is -672.25 us (96%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_none is an outlier: 22.5x slower than the field

carrier_opt_tight_none (703.06 us) is 22.5x the fastest (31.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_tight_all shows alternating (throttle bounce) (autocorr -0.71)

carrier_opt_tight_all's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_fold, carrier_opt_tight_canon, carrier_opt_tight_cse, carrier_opt_tight_dce, carrier_opt_tight_none} (1211% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_fold, carrier_opt_tight_canon, carrier_opt_tight_cse, carrier_opt_tight_dce, carrier_opt_tight_none} with a 1211% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 22.5x the fastest

Fastest carrier_opt_tight_all (31.19 us) to slowest carrier_opt_tight_none (703.06 us): 22.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_tight_all** at 31194.2 ns median (-95.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 22.54x (fastest 31194.2 ns, slowest 703064.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 33620ns | 33531ns | 32971ns | 33353ns | 34346ns | -95.27% |
| carrier_opt_tight_canon | 663436ns | 650862ns | 646924ns | 649578ns | 692480ns | -6.60% |
| carrier_opt_tight_cse | 663613ns | 660592ns | 656472ns | 659556ns | 673268ns | -6.57% |
| carrier_opt_tight_dce | 701837ns | 700341ns | 695427ns | 698967ns | 709346ns | -1.19% |
| carrier_opt_tight_fold | 416669ns | 412254ns | 405045ns | 410313ns | 432013ns | -41.34% |
| carrier_opt_tight_none | 710297ns | 706479ns | 704751ns | 706082ns | 719392ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 31267ns | 30658ns | 31945ns | -95.58% | 0.524 |
| carrier_opt_tight_canon | 660284ns | 643685ns | 689182ns | -6.61% | 0.025 |
| carrier_opt_tight_cse | 660678ns | 653523ns | 670810ns | -6.55% | 0.025 |
| carrier_opt_tight_dce | 698685ns | 691925ns | 706745ns | -1.18% | 0.023 |
| carrier_opt_tight_fold | 413663ns | 402306ns | 429112ns | -41.49% | 0.040 |
| carrier_opt_tight_none | 707000ns | 701540ns | 716260ns | base | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_tight_all | 390104 | 1706959 | 0.229 | 0.09× |
| carrier_opt_tight_canon | 4063790 | 16144681 | 0.252 | 0.92× |
| carrier_opt_tight_cse | 4120414 | 16491776 | 0.250 | 0.93× |
| carrier_opt_tight_dce | 4372477 | 17227479 | 0.254 | 0.99× |
| carrier_opt_tight_fold | 2557227 | 15681321 | 0.163 | 0.58× |
| carrier_opt_tight_none | 4420346 | 17228009 | 0.257 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.534 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.525 | 98.3% |
| carrier_opt_tight_canon | 0.025 | 4.7% |
| carrier_opt_tight_cse | 0.025 | 4.7% |
| carrier_opt_tight_dce | 0.024 | 4.4% |
| carrier_opt_tight_fold | 0.040 | 7.5% |
| carrier_opt_tight_none | 0.023 | 4.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 33620ns | 33620ns | -95.27% |
| carrier_opt_tight_canon | 663436ns | 663436ns | -6.60% |
| carrier_opt_tight_cse | 663613ns | 663613ns | -6.57% |
| carrier_opt_tight_dce | 701837ns | 701837ns | -1.19% |
| carrier_opt_tight_fold | 416669ns | 416669ns | -41.34% |
| carrier_opt_tight_none | 710297ns | 710297ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 703064ns | base | --- | [701674, 716260] | --- | --- | --- | --- |
| carrier_opt_tight_all | 31194ns | -672251.1ns (-95.6%) | [-685014, -669934]ns | [30660, 31945] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_tight_canon | 647972ns | -53765.5ns (-7.6%) | [-69150, -17229]ns | [643698, 689182] | YES (adj: no) | 0.2734 | 0.2188 | 0 |
| carrier_opt_tight_cse | 657391ns | -45792.2ns (-6.5%) | [-62307, -30864]ns | [653834, 670810] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_tight_dce | 697082ns | no significant difference | [-24030, +3680]ns | [692230, 706745] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_tight_fold | 408994ns | -293896.7ns (-41.8%) | [-310029, -276083]ns | [402884, 429112] | YES (adj: no) | 0.0521 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_canon | carrier_opt_tight_cse | carrier_opt_tight_dce | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|
| 1 | 723669ns | -95.8% | -11.0% | -9.6% | -4.4% | -44.4% |
| 2 | 701540ns | -95.4% | -8.2% | -3.2% | -0.9% | -41.2% |
| 3 | 703973ns | -95.6% | -6.4% | -7.1% | +0.0% | -42.2% |
| 4 | 701809ns | -95.5% | -7.1% | -5.6% | -0.4% | -41.4% |
| 5 | 702155ns | -95.6% | -8.3% | -5.9% | +1.0% | -42.5% |
| 6 | 708850ns | -95.5% | +1.5% | -7.8% | -2.3% | -37.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | -0.708 | HIGH- (thermal bounce) |
| carrier_opt_tight_canon | -0.123 | ok |
| carrier_opt_tight_cse | -0.523 | HIGH- (thermal bounce) |
| carrier_opt_tight_dce | -0.232 | moderate- |
| carrier_opt_tight_fold | -0.197 | ok |
| carrier_opt_tight_none | -0.115 | ok |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_canon**: won 5/6, lost 1/6
- **carrier_opt_tight_cse**: won 6/6, lost 0/6
- **carrier_opt_tight_dce**: won 4/6, lost 1/6
- **carrier_opt_tight_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 94196.4ns | 31266.5ns | 301.3% | HIGH |
| carrier_opt_tight_canon | 664250.1ns | 660284.5ns | 100.6% | HIGH |
| carrier_opt_tight_cse | 663342.6ns | 660678.5ns | 100.4% | HIGH |
| carrier_opt_tight_dce | 701280.7ns | 698685.5ns | 100.4% | HIGH |
| carrier_opt_tight_fold | 415288.2ns | 413663.5ns | 100.4% | HIGH |
| carrier_opt_tight_none | 709047.2ns | 706999.5ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 30657.9-31945.0 ns)
  30657.9 |########################################
  30722.3 |
  30786.6 |
  30851.0 |
  30915.3 |####################
  30979.7 |
  31044.0 |
  31108.4 |
  31172.7 |
  31237.1 |
  31301.4 |
  31365.8 |####################
  31430.1 |
  31494.5 |
  31558.8 |
  31623.2 |
  31687.5 |
  31751.9 |
  31816.2 |####################
  31880.6 |
  (0 below, 1 above range)

carrier_opt_tight_canon (n=6, range 643684.6-689182.5 ns)
  643684.6 |########################################
  645959.5 |
  648234.4 |
  650509.3 |#############
  652784.2 |
  655059.1 |
  657334.0 |#############
  659608.9 |
  661883.8 |
  664158.7 |
  666433.6 |
  668708.4 |
  670983.3 |
  673258.2 |
  675533.1 |
  677808.0 |
  680082.9 |
  682357.8 |
  684632.7 |
  686907.6 |
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 653523.3-670810.2 ns)
  653523.3 |########################################
  654387.6 |
  655252.0 |
  656116.3 |
  656980.7 |
  657845.0 |
  658709.4 |
  659573.7 |#############
  660438.1 |
  661302.4 |
  662166.8 |#############
  663031.1 |
  663895.4 |
  664759.8 |
  665624.1 |
  666488.5 |
  667352.8 |
  668217.2 |
  669081.5 |
  669945.9 |
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 691924.6-706744.6 ns)
  691924.6 |########################################
  692665.6 |
  693406.6 |
  694147.6 |
  694888.6 |####################
  695629.6 |
  696370.6 |
  697111.6 |
  697852.6 |
  698593.6 |####################
  699334.6 |
  700075.6 |
  700816.6 |
  701557.6 |
  702298.6 |
  703039.6 |
  703780.6 |####################
  704521.6 |
  705262.6 |
  706003.6 |
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 402305.8-429112.5 ns)
  402305.8 |########################################
  403646.1 |
  404986.5 |
  406326.8 |####################
  407667.1 |
  409007.5 |
  410347.8 |####################
  411688.1 |####################
  413028.5 |
  414368.8 |
  415709.2 |
  417049.5 |
  418389.8 |
  419730.2 |
  421070.5 |
  422410.8 |
  423751.2 |
  425091.5 |
  426431.8 |
  427772.2 |
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 701540.0-716259.8 ns)
  701540.0 |########################################
  702276.0 |
  703012.0 |
  703748.0 |#############
  704484.0 |
  705219.9 |
  705955.9 |
  706691.9 |
  707427.9 |
  708163.9 |#############
  708899.9 |
  709635.9 |
  710371.9 |
  711107.9 |
  711843.9 |
  712579.9 |
  713315.8 |
  714051.8 |
  714787.8 |
  715523.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=302.1% of algo (FFI overhead may distort results)
- **carrier_opt_tight_canon**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=99.7% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=100.4% of algo (FFI overhead may distort results)
