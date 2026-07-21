# Match lowering: hot-first if-chain vs the rest, K=64 arms, 90% hit one arm

4 variants, 6 samples per variant.
Baseline: **ml_jumptable_h64**

## Highlights

Baseline for all deltas below: **ml_jumptable_h64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_h64 is an outlier: 7.2x slower than the field

ml_tree_h64 (127.60 us) is 7.2x the fastest (17.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ml_hotfirst_h64 is fastest but the noisiest (CV 6.7%)

ml_hotfirst_h64 wins on median (17.69 us) yet has the highest variance (CV 6.7%), while ml_tree_h64 is the steadiest (CV 3.9%, 127.60 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ml_hotfirst_h64, ml_jumptable_h64) are a dead heat (<1%)

ml_hotfirst_h64 (17.69 us) and ml_jumptable_h64 (17.75 us) differ by 0.38%, inside the noise, even though the wider field spreads 621.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {ml_hotfirst_h64, ml_jumptable_h64, ml_ifchain_h64} vs {ml_tree_h64} (618% apart)

The field splits into a fast tier {ml_hotfirst_h64, ml_jumptable_h64, ml_ifchain_h64} and a slow tier {ml_tree_h64} with a 618% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.2x the fastest

Fastest ml_hotfirst_h64 (17.69 us) to slowest ml_tree_h64 (127.60 us): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ml_hotfirst_h64** at 17685.8 ns median (-0.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 7.21x (fastest 17685.8 ns, slowest 127601.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_hotfirst_h64 | 19882ns | 20148ns | 17781ns | 19676ns | 21242ns | -1.84% |
| ml_ifchain_h64 | 19698ns | 20185ns | 17868ns | 19562ns | 20818ns | -2.75% |
| ml_jumptable_h64 | 20255ns | 20249ns | 18130ns | 20223ns | 21363ns | base |
| ml_tree_h64 | 129790ns | 130098ns | 124000ns | 128096ns | 135227ns | +540.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_hotfirst_h64 | 17483ns | 15651ns | 18695ns | -1.84% | 0.015 |
| ml_ifchain_h64 | 17344ns | 15748ns | 18328ns | -2.62% | 0.015 |
| ml_jumptable_h64 | 17810ns | 15957ns | 18823ns | base | 0.014 |
| ml_tree_h64 | 127359ns | 121740ns | 132707ns | +615.09% | 0.002 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_hotfirst_h64; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_hotfirst_h64 | 0.014 | 88.5% |
| ml_ifchain_h64 | 0.014 | 88.1% |
| ml_jumptable_h64 | 0.014 | 88.2% |
| ml_tree_h64 | 0.002 | 12.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_hotfirst_h64 | 19882ns | 19882ns | -1.84% |
| ml_ifchain_h64 | 19698ns | 19698ns | -2.75% |
| ml_jumptable_h64 | 20255ns | 20255ns | base |
| ml_tree_h64 | 129790ns | 129790ns | +540.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_h64 | 17754ns | base | --- | [16854, 18823] | --- | --- | --- | --- |
| ml_hotfirst_h64 | 17686ns | no significant difference | [-1426, +691]ns | [16069, 18695] | no | 1.0000 | 0.6875 | 0 |
| ml_ifchain_h64 | 17766ns | no significant difference | [-1646, +628]ns | [15938, 18328] | no | 1.0000 | 1.0000 | 0 |
| ml_tree_h64 | 127602ns | +109424.8ns (+616.3%) | [+104914, +114309]ns | [121769, 132707] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_h64 | ml_hotfirst_h64 | ml_ifchain_h64 | ml_tree_h64 |
|---|---|---|---|---|
| 1 | 17753ns | +4.8% | +6.1% | +586.1% |
| 2 | 17754ns | -4.2% | +0.4% | +646.5% |
| 3 | 15957ns | +3.3% | +1.1% | +662.9% |
| 4 | 17750ns | -11.8% | -11.3% | +597.2% |
| 5 | 19043ns | -1.4% | -6.8% | +597.8% |
| 6 | 18603ns | -1.2% | -4.4% | +606.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_hotfirst_h64 | 0.065 | ok |
| ml_ifchain_h64 | 0.233 | moderate+ |
| ml_jumptable_h64 | 0.201 | moderate+ |
| ml_tree_h64 | -0.233 | moderate- |

**Consistency summary:**

- **ml_hotfirst_h64**: won 4/6, lost 2/6
- **ml_ifchain_h64**: won 3/6, lost 3/6
- **ml_tree_h64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_hotfirst_h64 | 3.4ns | 17483.2ns | 0.0% |  |
| ml_ifchain_h64 | 3.9ns | 17344.1ns | 0.0% |  |
| ml_jumptable_h64 | 3.2ns | 17810.2ns | 0.0% |  |
| ml_tree_h64 | 5.5ns | 127359.3ns | 0.0% |  |

## Distribution (algo ns)

```
ml_hotfirst_h64 (n=6, range 15651.2-18695.2 ns)
  15651.2 |########################################
  15803.4 |
  15955.6 |
  16107.8 |
  16260.0 |
  16412.2 |########################################
  16564.4 |
  16716.6 |
  16868.8 |########################################
  17021.0 |
  17173.2 |
  17325.4 |
  17477.6 |
  17629.8 |
  17782.0 |
  17934.2 |
  18086.4 |
  18238.6 |########################################
  18390.8 |
  18543.0 |########################################
  (0 below, 1 above range)

ml_ifchain_h64 (n=6, range 15747.9-18328.3 ns)
  15747.9 |####################
  15876.9 |
  16005.9 |####################
  16135.0 |
  16264.0 |
  16393.0 |
  16522.0 |
  16651.0 |
  16780.1 |
  16909.1 |
  17038.1 |
  17167.1 |
  17296.1 |
  17425.2 |
  17554.2 |
  17683.2 |########################################
  17812.2 |####################
  17941.2 |
  18070.3 |
  18199.3 |
  (0 below, 1 above range)

ml_jumptable_h64 (n=6, range 15957.1-18823.3 ns)
  15957.1 |#############
  16100.4 |
  16243.7 |
  16387.0 |
  16530.3 |
  16673.7 |
  16817.0 |
  16960.3 |
  17103.6 |
  17246.9 |
  17390.2 |
  17533.5 |
  17676.8 |########################################
  17820.1 |
  17963.4 |
  18106.8 |
  18250.1 |
  18393.4 |
  18536.7 |#############
  18680.0 |
  (0 below, 1 above range)

ml_tree_h64 (n=6, range 121739.6-132707.3 ns)
  121739.6 |########################################
  122288.0 |
  122836.4 |
  123384.8 |####################
  123933.1 |
  124481.5 |
  125029.9 |
  125578.3 |
  126126.7 |
  126675.1 |
  127223.4 |
  127771.8 |
  128320.2 |
  128868.6 |
  129417.0 |
  129965.4 |
  130513.8 |
  131062.1 |####################
  131610.5 |
  132158.9 |####################
  (0 below, 1 above range)

```
