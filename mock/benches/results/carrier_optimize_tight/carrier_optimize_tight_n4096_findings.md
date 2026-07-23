# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_tight_all dominates: 1128% faster than the next best (carrier_opt_tight_fold)

carrier_opt_tight_all (8.26 us) leads carrier_opt_tight_fold (101.48 us) by 1128%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 94% (significant)

carrier_opt_tight_all is -144.13 us (94%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_dce is an outlier: 18.7x slower than the field

carrier_opt_tight_dce (154.27 us) is 18.7x the fastest (8.26 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_tight_all shows alternating (throttle bounce) (autocorr -0.63)

carrier_opt_tight_all's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_fold, carrier_opt_tight_canon, carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_dce} (1128% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_fold, carrier_opt_tight_canon, carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_dce} with a 1128% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 18.7x the fastest

Fastest carrier_opt_tight_all (8.26 us) to slowest carrier_opt_tight_dce (154.27 us): 18.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_tight_all** at 8264.6 ns median (-94.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 18.67x (fastest 8264.6 ns, slowest 154266.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 10595ns | 10545ns | 10202ns | 10451ns | 11007ns | -93.16% |
| carrier_opt_tight_canon | 146822ns | 146978ns | 143930ns | 146925ns | 148115ns | -5.24% |
| carrier_opt_tight_cse | 151783ns | 151608ns | 148972ns | 150738ns | 154755ns | -2.04% |
| carrier_opt_tight_dce | 156752ns | 156650ns | 154595ns | 156056ns | 158873ns | +1.17% |
| carrier_opt_tight_fold | 104302ns | 103855ns | 101838ns | 103216ns | 107164ns | -32.68% |
| carrier_opt_tight_none | 154943ns | 155024ns | 151859ns | 154142ns | 157685ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 8287ns | 7952ns | 8615ns | -94.57% | 0.494 |
| carrier_opt_tight_canon | 144407ns | 141722ns | 145570ns | -5.33% | 0.028 |
| carrier_opt_tight_cse | 149329ns | 146451ns | 152332ns | -2.10% | 0.027 |
| carrier_opt_tight_dce | 154260ns | 151870ns | 156316ns | +1.13% | 0.027 |
| carrier_opt_tight_fold | 101945ns | 99477ns | 104816ns | -33.17% | 0.040 |
| carrier_opt_tight_none | 152536ns | 149606ns | 155185ns | base | 0.027 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_tight_all | 304190 | 1400115 | 0.217 | 0.32× |
| carrier_opt_tight_canon | 907310 | 4084214 | 0.222 | 0.96× |
| carrier_opt_tight_cse | 929322 | 4171391 | 0.223 | 0.98× |
| carrier_opt_tight_dce | 959683 | 4319684 | 0.222 | 1.02× |
| carrier_opt_tight_fold | 638684 | 3933678 | 0.162 | 0.68× |
| carrier_opt_tight_none | 945398 | 4319626 | 0.219 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.515 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.496 | 96.2% |
| carrier_opt_tight_canon | 0.028 | 5.5% |
| carrier_opt_tight_cse | 0.027 | 5.3% |
| carrier_opt_tight_dce | 0.027 | 5.2% |
| carrier_opt_tight_fold | 0.040 | 7.8% |
| carrier_opt_tight_none | 0.027 | 5.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 10595ns | 10595ns | -93.16% |
| carrier_opt_tight_canon | 146822ns | 146822ns | -5.24% |
| carrier_opt_tight_cse | 151783ns | 151783ns | -2.04% |
| carrier_opt_tight_dce | 156752ns | 156752ns | +1.17% |
| carrier_opt_tight_fold | 104302ns | 104302ns | -32.68% |
| carrier_opt_tight_none | 154943ns | 154943ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 152581ns | base | --- | [149842, 155185] | --- | --- | --- | --- |
| carrier_opt_tight_all | 8265ns | -144133.3ns (-94.5%) | [-146753, -141860]ns | [7982, 8615] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_tight_canon | 144663ns | -7464.8ns (-4.9%) | [-11743, -5180]ns | [142988, 145570] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_tight_cse | 149137ns | no significant difference | [-7056, +1225]ns | [146516, 152332] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_tight_dce | 154266ns | no significant difference | [-445, +3332]ns | [152197, 156316] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_tight_fold | 101483ns | -49914.2ns (-32.7%) | [-53946, -47915]ns | [99535, 104816] | YES (adj: no) | 0.0521 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_canon | carrier_opt_tight_cse | carrier_opt_tight_dce | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|
| 1 | 149606ns | -94.6% | -3.4% | -2.1% | +2.1% | -31.3% |
| 2 | 152135ns | -94.5% | -4.6% | +0.0% | -0.2% | -33.1% |
| 3 | 153935ns | -94.7% | -5.1% | -2.9% | +1.4% | -35.4% |
| 4 | 153026ns | -94.5% | -5.7% | -4.2% | +2.3% | -34.9% |
| 5 | 150079ns | -94.7% | -3.5% | +1.6% | +1.6% | -32.6% |
| 6 | 156435ns | -94.4% | -9.4% | -4.9% | -0.4% | -31.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | -0.633 | HIGH- (thermal bounce) |
| carrier_opt_tight_canon | -0.005 | ok |
| carrier_opt_tight_cse | -0.544 | HIGH- (thermal bounce) |
| carrier_opt_tight_dce | -0.144 | ok |
| carrier_opt_tight_fold | 0.104 | ok |
| carrier_opt_tight_none | -0.295 | moderate- |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_canon**: won 6/6, lost 0/6
- **carrier_opt_tight_cse**: won 4/6, lost 1/6
- **carrier_opt_tight_dce**: won 2/6, lost 4/6
- **carrier_opt_tight_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 89652.5ns | 8287.1ns | 1081.8% | HIGH |
| carrier_opt_tight_canon | 145309.2ns | 144406.9ns | 100.6% | HIGH |
| carrier_opt_tight_cse | 150140.4ns | 149328.5ns | 100.5% | HIGH |
| carrier_opt_tight_dce | 155139.6ns | 154259.7ns | 100.6% | HIGH |
| carrier_opt_tight_fold | 102603.6ns | 101944.6ns | 100.6% | HIGH |
| carrier_opt_tight_none | 153122.3ns | 152536.0ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 7951.7-8614.8 ns)
   7951.7 |########################################
   7984.9 |########################################
   8018.0 |
   8051.2 |
   8084.3 |########################################
   8117.5 |
   8150.6 |
   8183.8 |
   8216.9 |
   8250.1 |
   8283.2 |
   8316.4 |
   8349.6 |
   8382.7 |
   8415.9 |########################################
   8449.0 |########################################
   8482.2 |
   8515.3 |
   8548.5 |
   8581.6 |
  (0 below, 1 above range)

carrier_opt_tight_canon (n=6, range 141722.5-145570.4 ns)
  141722.5 |########################################
  141914.9 |
  142107.3 |
  142299.7 |
  142492.1 |
  142684.5 |
  142876.9 |
  143069.3 |
  143261.7 |
  143454.1 |
  143646.5 |
  143838.8 |
  144031.2 |
  144223.6 |########################################
  144416.0 |########################################
  144608.4 |########################################
  144800.8 |
  144993.2 |########################################
  145185.6 |
  145378.0 |
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 146450.8-152332.1 ns)
  146450.8 |########################################
  146744.9 |
  147038.9 |
  147333.0 |
  147627.1 |
  147921.1 |
  148215.2 |
  148509.3 |####################
  148803.3 |
  149097.4 |
  149391.5 |####################
  149685.5 |
  149979.6 |
  150273.6 |
  150567.7 |
  150861.8 |
  151155.8 |
  151449.9 |
  151744.0 |
  152038.0 |####################
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 151870.0-156316.0 ns)
  151870.0 |########################################
  152092.3 |
  152314.6 |########################################
  152536.9 |########################################
  152759.2 |
  152981.5 |
  153203.8 |
  153426.1 |
  153648.4 |
  153870.7 |
  154093.0 |
  154315.3 |
  154537.6 |
  154759.9 |
  154982.2 |
  155204.5 |
  155426.8 |
  155649.1 |########################################
  155871.4 |########################################
  156093.7 |
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 99476.7-104815.9 ns)
  99476.7 |########################################
  99743.7 |
  100010.6 |
  100277.6 |
  100544.5 |
  100811.5 |
  101078.4 |####################
  101345.4 |
  101612.4 |####################
  101879.3 |
  102146.3 |
  102413.2 |
  102680.2 |####################
  102947.1 |
  103214.1 |
  103481.1 |
  103748.0 |
  104015.0 |
  104281.9 |
  104548.9 |
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 149605.8-155185.2 ns)
  149605.8 |########################################
  149884.8 |########################################
  150163.7 |
  150442.7 |
  150721.7 |
  151000.6 |
  151279.6 |
  151558.6 |
  151837.6 |
  152116.5 |########################################
  152395.5 |
  152674.5 |
  152953.4 |########################################
  153232.4 |
  153511.4 |
  153790.4 |########################################
  154069.3 |
  154348.3 |
  154627.3 |
  154906.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=1082.3% of algo (FFI overhead may distort results)
- **carrier_opt_tight_canon**: bridge=100.7% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=100.7% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=100.7% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=100.3% of algo (FFI overhead may distort results)
