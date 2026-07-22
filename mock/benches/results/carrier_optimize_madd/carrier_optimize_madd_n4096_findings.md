# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_madd_all dominates: 1418% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (6.97 us) leads carrier_opt_madd_fold (105.83 us) by 1418%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 96% (significant)

carrier_opt_madd_all is -156.05 us (96%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_eqsat is an outlier: 127.7x slower than the field

carrier_opt_madd_eqsat (890.55 us) is 127.7x the fastest (6.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_opt_madd_all} vs {carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none, carrier_opt_madd_cseeqsat, carrier_opt_madd_eqsat} (1418% apart)

The field splits into a fast tier {carrier_opt_madd_all} and a slow tier {carrier_opt_madd_fold, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none, carrier_opt_madd_cseeqsat, carrier_opt_madd_eqsat} with a 1418% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 127.7x the fastest

Fastest carrier_opt_madd_all (6.97 us) to slowest carrier_opt_madd_eqsat (890.55 us): 127.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_madd_all** at 6971.2 ns median (-95.7% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 127.75x (fastest 6971.2 ns, slowest 890549.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 9266ns | 9221ns | 9092ns | 9179ns | 9483ns | -94.43% |
| carrier_opt_madd_cse | 156856ns | 155812ns | 154728ns | 155474ns | 159991ns | -5.78% |
| carrier_opt_madd_cseeqsat | 891768ns | 891361ns | 887144ns | 889979ns | 896762ns | +435.66% |
| carrier_opt_madd_dce | 158452ns | 157021ns | 155909ns | 156759ns | 162263ns | -4.82% |
| carrier_opt_madd_eqsat | 894876ns | 894237ns | 889039ns | 893300ns | 900160ns | +437.52% |
| carrier_opt_madd_fold | 107250ns | 108206ns | 101474ns | 108146ns | 108792ns | -35.58% |
| carrier_opt_madd_none | 166481ns | 165240ns | 164438ns | 165008ns | 169712ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 7063ns | 6960ns | 7255ns | -95.70% | 0.580 |
| carrier_opt_madd_cse | 154486ns | 152398ns | 157761ns | -5.91% | 0.027 |
| carrier_opt_madd_cseeqsat | 889087ns | 883512ns | 894024ns | +441.49% | 0.005 |
| carrier_opt_madd_dce | 156127ns | 153560ns | 159973ns | -4.91% | 0.026 |
| carrier_opt_madd_eqsat | 891895ns | 886725ns | 897203ns | +443.20% | 0.005 |
| carrier_opt_madd_fold | 104812ns | 98868ns | 106335ns | -36.17% | 0.039 |
| carrier_opt_madd_none | 164193ns | 161877ns | 167456ns | base | 0.025 |

## Performance model

- Peak throughput: **0.589 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.588 | 99.8% |
| carrier_opt_madd_cse | 0.027 | 4.5% |
| carrier_opt_madd_cseeqsat | 0.005 | 0.8% |
| carrier_opt_madd_dce | 0.026 | 4.5% |
| carrier_opt_madd_eqsat | 0.005 | 0.8% |
| carrier_opt_madd_fold | 0.039 | 6.6% |
| carrier_opt_madd_none | 0.025 | 4.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 9266ns | 9266ns | -94.43% |
| carrier_opt_madd_cse | 156856ns | 156856ns | -5.78% |
| carrier_opt_madd_cseeqsat | 891768ns | 891768ns | +435.66% |
| carrier_opt_madd_dce | 158452ns | 158452ns | -4.82% |
| carrier_opt_madd_eqsat | 894876ns | 894876ns | +437.52% |
| carrier_opt_madd_fold | 107250ns | 107250ns | -35.58% |
| carrier_opt_madd_none | 166481ns | 166481ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 163015ns | base | --- | [162108, 167456] | --- | --- | --- | --- |
| carrier_opt_madd_all | 6971ns | -156052.0ns (-95.7%) | [-160201, -155139]ns | [6961, 7255] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_cse | 153234ns | -10092.7ns (-6.2%) | [-10398, -8632]ns | [152463, 157761] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_cseeqsat | 889061ns | +723949.4ns (+444.1%) | [+721249, +729485]ns | [884178, 894024] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_dce | 154605ns | -8548.9ns (-5.2%) | [-8997, -6654]ns | [153801, 159973] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_eqsat | 890549ns | +727981.5ns (+446.6%) | [+720476, +734648]ns | [887933, 897203] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_fold | 105831ns | -59513.2ns (-36.5%) | [-62600, -56029]ns | [102271, 106335] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_cse | carrier_opt_madd_cseeqsat | carrier_opt_madd_dce | carrier_opt_madd_eqsat | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|---|
| 1 | 162772ns | -95.7% | -6.3% | +446.2% | -5.3% | +452.6% | -34.5% |
| 2 | 167201ns | -95.7% | -5.4% | +436.3% | -3.9% | +430.3% | -36.8% |
| 3 | 167711ns | -95.7% | -6.2% | +430.1% | -5.0% | +430.2% | -36.8% |
| 4 | 162338ns | -95.7% | -6.1% | +444.2% | -5.4% | +451.3% | -39.1% |
| 5 | 163258ns | -95.7% | -6.4% | +442.0% | -5.6% | +445.7% | -35.2% |
| 6 | 161877ns | -95.7% | -5.1% | +450.6% | -4.2% | +449.9% | -34.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | 0.182 | ok |
| carrier_opt_madd_cse | 0.068 | ok |
| carrier_opt_madd_cseeqsat | 0.126 | ok |
| carrier_opt_madd_dce | 0.098 | ok |
| carrier_opt_madd_eqsat | -0.331 | moderate- |
| carrier_opt_madd_fold | -0.207 | moderate- |
| carrier_opt_madd_none | 0.111 | ok |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_cse**: won 6/6, lost 0/6
- **carrier_opt_madd_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_dce**: won 6/6, lost 0/6
- **carrier_opt_madd_eqsat**: won 0/6, lost 6/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 90085.1ns | 7062.6ns | 1275.5% | HIGH |
| carrier_opt_madd_cse | 155195.7ns | 154485.8ns | 100.5% | HIGH |
| carrier_opt_madd_cseeqsat | 891465.6ns | 889087.4ns | 100.3% | HIGH |
| carrier_opt_madd_dce | 156800.0ns | 156126.6ns | 100.4% | HIGH |
| carrier_opt_madd_eqsat | 894533.2ns | 891895.1ns | 100.3% | HIGH |
| carrier_opt_madd_fold | 105478.9ns | 104812.4ns | 100.6% | HIGH |
| carrier_opt_madd_none | 165005.1ns | 164193.1ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 6960.0-7255.4 ns)
   6960.0 |########################################
   6974.8 |#############
   6989.5 |
   7004.3 |
   7019.1 |
   7033.9 |
   7048.6 |
   7063.4 |
   7078.2 |
   7092.9 |
   7107.7 |
   7122.5 |
   7137.2 |
   7152.0 |
   7166.8 |
   7181.5 |
   7196.3 |
   7211.1 |
   7225.9 |#############
   7240.6 |
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 152397.9-157761.0 ns)
  152397.9 |########################################
  152666.1 |####################
  152934.2 |
  153202.4 |
  153470.5 |####################
  153738.7 |
  154006.8 |
  154275.0 |
  154543.1 |
  154811.3 |
  155079.5 |
  155347.6 |
  155615.8 |
  155883.9 |
  156152.1 |
  156420.2 |
  156688.4 |
  156956.5 |
  157224.7 |####################
  157492.8 |
  (0 below, 1 above range)

carrier_opt_madd_cseeqsat (n=6, range 883511.7-894023.9 ns)
  883511.7 |####################
  884037.3 |
  884562.9 |####################
  885088.5 |
  885614.1 |
  886139.8 |
  886665.4 |
  887191.0 |
  887716.6 |
  888242.2 |
  888767.8 |########################################
  889293.4 |
  889819.0 |
  890344.7 |
  890870.3 |####################
  891395.9 |
  891921.5 |
  892447.1 |
  892972.7 |
  893498.3 |
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 153559.6-159973.2 ns)
  153559.6 |####################
  153880.3 |########################################
  154201.0 |
  154521.6 |
  154842.3 |####################
  155163.0 |
  155483.7 |
  155804.3 |
  156125.0 |
  156445.7 |
  156766.4 |
  157087.1 |
  157407.7 |
  157728.4 |
  158049.1 |
  158369.8 |
  158690.4 |
  159011.1 |####################
  159331.8 |
  159652.5 |
  (0 below, 1 above range)

carrier_opt_madd_eqsat (n=6, range 886725.0-897203.3 ns)
  886725.0 |########################################
  887248.9 |
  887772.8 |
  888296.7 |
  888820.7 |########################################
  889344.6 |
  889868.5 |########################################
  890392.4 |########################################
  890916.3 |
  891440.2 |
  891964.2 |
  892488.1 |
  893012.0 |
  893535.9 |
  894059.8 |
  894583.7 |########################################
  895107.6 |
  895631.6 |
  896155.5 |
  896679.4 |
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 98868.3-106335.0 ns)
  98868.3 |#############
  99241.6 |
  99615.0 |
  99988.3 |
  100361.6 |
  100735.0 |
  101108.3 |
  101481.6 |
  101855.0 |
  102228.3 |
  102601.6 |
  102975.0 |
  103348.3 |
  103721.7 |
  104095.0 |
  104468.3 |
  104841.7 |
  105215.0 |
  105588.3 |########################################
  105961.7 |#############
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 161877.1-167456.2 ns)
  161877.1 |########################################
  162156.1 |########################################
  162435.0 |
  162714.0 |########################################
  162992.9 |########################################
  163271.9 |
  163550.8 |
  163829.8 |
  164108.7 |
  164387.7 |
  164666.7 |
  164945.6 |
  165224.6 |
  165503.5 |
  165782.5 |
  166061.4 |
  166340.4 |
  166619.3 |
  166898.3 |
  167177.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=1293.9% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cseeqsat**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_madd_eqsat**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=100.6% of algo (FFI overhead may distort results)
