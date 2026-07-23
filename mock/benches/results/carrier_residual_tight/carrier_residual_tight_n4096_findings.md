# Residual encoding: predecoded register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 241% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (110.99 us) leads carrier_res_tight_stack (378.71 us) by 241%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_tight_stack shows alternating (throttle bounce) (autocorr -0.68)

carrier_res_tight_stack's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (110.99 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest carrier_res_tight_register (110.99 us) to slowest carrier_res_tight_stack (378.71 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 110986.9 ns median
- 1 variant significantly slower than baseline
- Spread: 3.41x (fastest 110986.9 ns, slowest 378705.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 113555ns | 113311ns | 112930ns | 113250ns | 114326ns | base |
| carrier_res_tight_stack | 381559ns | 381797ns | 380665ns | 381539ns | 382035ns | +236.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 111208ns | 110655ns | 111952ns | base | 0.037 |
| carrier_res_tight_stack | 378487ns | 377390ns | 379037ns | +240.34% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_tight_register | 713663 | 2224657 | 0.321 | 1.00× |
| carrier_res_tight_stack | 2402458 | 9465150 | 0.254 | 3.37× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.037 | 99.7% |
| carrier_res_tight_stack | 0.011 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 113555ns | 113555ns | base |
| carrier_res_tight_stack | 381559ns | 381559ns | +236.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 110987ns | base | --- | [110685, 111952] | --- | --- | --- | --- |
| carrier_res_tight_stack | 378705ns | +267627.9ns (+241.1%) | [+266098, +268110]ns | [377717, 379037] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 111415ns | +240.1% |
| 2 | 110750ns | +240.8% |
| 3 | 111224ns | +240.7% |
| 4 | 112488ns | +236.1% |
| 5 | 110714ns | +242.4% |
| 6 | 110655ns | +242.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | -0.181 | ok |
| carrier_res_tight_stack | -0.681 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 113709.8ns | 111207.8ns | 102.2% | HIGH |
| carrier_res_tight_stack | 383313.0ns | 378486.5ns | 101.3% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 110655.4-111951.6 ns)
  110655.4 |########################################
  110720.2 |####################
  110785.0 |
  110849.8 |
  110914.6 |
  110979.5 |
  111044.3 |
  111109.1 |
  111173.9 |####################
  111238.7 |
  111303.5 |
  111368.3 |####################
  111433.1 |
  111498.0 |
  111562.8 |
  111627.6 |
  111692.4 |
  111757.2 |
  111822.0 |
  111886.8 |
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 377390.4-379037.3 ns)
  377390.4 |########################################
  377472.7 |
  377555.1 |
  377637.4 |
  377719.8 |
  377802.1 |
  377884.5 |
  377966.8 |########################################
  378049.2 |
  378131.5 |
  378213.8 |
  378296.2 |
  378378.5 |
  378460.9 |########################################
  378543.2 |
  378625.6 |
  378707.9 |
  378790.3 |
  378872.6 |########################################
  378955.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=102.3% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=101.3% of algo (FFI overhead may distort results)
