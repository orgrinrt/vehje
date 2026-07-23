# Residual encoding: predecoded register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 236% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (1.76 us) leads carrier_res_tight_stack (5.90 us) by 236%, a clear separation rather than a photo finish. CV 4.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_tight_stack shows alternating (throttle bounce) (autocorr -0.75)

carrier_res_tight_stack's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (1.76 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest carrier_res_tight_register (1.76 us) to slowest carrier_res_tight_stack (5.90 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 1758.8 ns median
- 1 variant significantly slower than baseline
- Spread: 3.36x (fastest 1758.8 ns, slowest 5900.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 4243ns | 4198ns | 4049ns | 4161ns | 4464ns | base |
| carrier_res_tight_stack | 8325ns | 8285ns | 8109ns | 8228ns | 8580ns | +96.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 1757ns | 1649ns | 1844ns | base | 0.036 |
| carrier_res_tight_stack | 5898ns | 5627ns | 6084ns | +235.66% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_tight_register | 258559 | 908318 | 0.285 | 1.00× |
| carrier_res_tight_stack | 266901 | 1067451 | 0.250 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.036 | 93.8% |
| carrier_res_tight_stack | 0.011 | 27.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 4243ns | 4243ns | base |
| carrier_res_tight_stack | 8325ns | 8325ns | +96.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 1759ns | base | --- | [1669, 1844] | --- | --- | --- | --- |
| carrier_res_tight_stack | 5901ns | +4101.2ns (+233.2%) | [+4007, +4315]ns | [5709, 6084] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 1917ns | +210.5% |
| 2 | 1688ns | +243.0% |
| 3 | 1771ns | +250.8% |
| 4 | 1649ns | +241.2% |
| 5 | 1768ns | +236.9% |
| 6 | 1749ns | +234.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | -0.348 | moderate- |
| carrier_res_tight_stack | -0.745 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 87063.3ns | 1757.1ns | 4954.8% | HIGH |
| carrier_res_tight_stack | 84832.8ns | 5898.1ns | 1438.3% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 1649.2-1843.9 ns)
   1649.2 |####################
   1658.9 |
   1668.7 |
   1678.4 |
   1688.2 |####################
   1697.9 |
   1707.6 |
   1717.4 |
   1727.1 |
   1736.8 |
   1746.6 |####################
   1756.3 |
   1766.0 |########################################
   1775.8 |
   1785.5 |
   1795.3 |
   1805.0 |
   1814.7 |
   1824.5 |
   1834.2 |
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 5627.1-6084.2 ns)
   5627.1 |####################
   5650.0 |
   5672.8 |
   5695.7 |
   5718.5 |
   5741.4 |
   5764.2 |
   5787.1 |####################
   5809.9 |
   5832.8 |####################
   5855.6 |
   5878.5 |
   5901.4 |
   5924.2 |
   5947.1 |########################################
   5969.9 |
   5992.8 |
   6015.6 |
   6038.5 |
   6061.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=4949.4% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=1434.4% of algo (FFI overhead may distort results)
