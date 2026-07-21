# Record update (0% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s00**

## Highlights

Baseline for all deltas below: **rec_reuse_s00**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_copy is an outlier: 6.2x slower than the field

rec_copy (7.93 us) is 6.2x the fastest (1.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rec_mut is fastest but the noisiest (CV 5.6%)

rec_mut wins on median (1.29 us) yet has the highest variance (CV 5.6%), while rec_copy is the steadiest (CV 3.3%, 7.93 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6.2x the fastest

Fastest rec_mut (1.29 us) to slowest rec_copy (7.93 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 1288.6 ns median (-7.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.15x (fastest 1288.6 ns, slowest 7926.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 10428ns | 10430ns | 9840ns | 10378ns | 10795ns | +166.65% |
| rec_mut | 3776ns | 3811ns | 3417ns | 3784ns | 3943ns | -3.44% |
| rec_reuse_s00 | 3911ns | 3912ns | 3800ns | 3875ns | 4019ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 7942ns | 7525ns | 8221ns | +464.07% | 0.032 |
| rec_mut | 1286ns | 1142ns | 1355ns | -8.69% | 0.199 |
| rec_reuse_s00 | 1408ns | 1360ns | 1468ns | base | 0.182 |

## Performance model

- Peak throughput: **0.224 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.032 | 14.4% |
| rec_mut | 0.199 | 88.7% |
| rec_reuse_s00 | 0.183 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 10428ns | 10428ns | +166.65% |
| rec_mut | 3776ns | 3776ns | -3.44% |
| rec_reuse_s00 | 3911ns | 3911ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s00 | 1395ns | base | --- | [1361, 1468] | --- | --- | --- | --- |
| rec_copy | 7926ns | +6559.0ns (+470.0%) | [+6246, +6796]ns | [7677, 8221] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 1289ns | -87.9ns (-6.3%) | [-218, -61]ns | [1213, 1355] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s00 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 1362ns | +452.6% | -16.1% |
| 2 | 1502ns | +421.4% | -14.4% |
| 3 | 1360ns | +475.8% | -5.0% |
| 4 | 1364ns | +488.3% | -5.8% |
| 5 | 1427ns | +485.8% | -3.8% |
| 6 | 1434ns | +463.7% | -6.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | 0.356 | moderate+ |
| rec_mut | 0.143 | ok |
| rec_reuse_s00 | -0.436 | moderate- |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21878.9ns | 7941.6ns | 275.5% | HIGH |
| rec_mut | 20809.4ns | 1285.6ns | 1618.6% | HIGH |
| rec_reuse_s00 | 21088.9ns | 1407.9ns | 1497.9% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 7525.4-8221.2 ns)
   7525.4 |####################
   7560.2 |
   7595.0 |
   7629.8 |
   7664.6 |
   7699.4 |
   7734.2 |
   7768.9 |
   7803.7 |########################################
   7838.5 |
   7873.3 |
   7908.1 |
   7942.9 |
   7977.7 |
   8012.5 |####################
   8047.3 |
   8082.1 |####################
   8116.9 |
   8151.7 |
   8186.5 |
  (0 below, 1 above range)

rec_mut (n=6, range 1142.5-1355.0 ns)
   1142.5 |####################
   1153.1 |
   1163.8 |
   1174.4 |
   1185.0 |
   1195.6 |
   1206.2 |
   1216.9 |
   1227.5 |
   1238.1 |
   1248.8 |
   1259.4 |
   1270.0 |
   1280.6 |########################################
   1291.2 |####################
   1301.9 |
   1312.5 |
   1323.1 |
   1333.8 |####################
   1344.4 |
  (0 below, 1 above range)

rec_reuse_s00 (n=6, range 1359.6-1467.7 ns)
   1359.6 |########################################
   1365.0 |
   1370.4 |
   1375.8 |
   1381.2 |
   1386.6 |
   1392.0 |
   1397.4 |
   1402.8 |
   1408.2 |
   1413.7 |
   1419.1 |
   1424.5 |#############
   1429.9 |#############
   1435.3 |
   1440.7 |
   1446.1 |
   1451.5 |
   1456.9 |
   1462.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=277.1% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=1618.4% of algo (FFI overhead may distort results)
- **rec_reuse_s00**: bridge=1506.4% of algo (FFI overhead may distort results)
