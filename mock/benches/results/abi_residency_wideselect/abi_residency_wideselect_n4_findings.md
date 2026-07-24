# abi_residency (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_residency_wideselect_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_wideselect_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_wideselect_null_entry dominates: 51515% faster than the next best (abi_residency_wideselect_reused_buffer)

abi_residency_wideselect_null_entry (3.98 us) leads abi_residency_wideselect_reused_buffer (2.06 ms) by 51515%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_wideselect_null_entry beats baseline by 100% (significant)

abi_residency_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_residency_wideselect_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_wideselect_fresh_alloc is an outlier: 516.2x slower than the field

abi_residency_wideselect_fresh_alloc (2.06 ms) is 516.2x the fastest (3.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 516.2x the fastest

Fastest abi_residency_wideselect_null_entry (3.98 us) to slowest abi_residency_wideselect_fresh_alloc (2.06 ms): 516.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_wideselect_null_entry** at 3983.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 516.19x (fastest 3983.9 ns, slowest 2056465.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2060253ns | 2059000ns | 2054560ns | 2057793ns | 2066790ns | +0.12% |
| abi_residency_wideselect_null_entry | 6228ns | 6229ns | 5925ns | 6188ns | 6440ns | -99.70% |
| abi_residency_wideselect_reused_buffer | 2057791ns | 2059065ns | 2046900ns | 2058653ns | 2061942ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2057639ns | 2051778ns | 2064113ns | +0.12% | 0.000 |
| abi_residency_wideselect_null_entry | 3985ns | 3803ns | 4122ns | -99.81% | 0.001 |
| abi_residency_wideselect_reused_buffer | 2055218ns | 2044448ns | 2059455ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 41086.0 | 2059098.7 | 2057638.7 | n/a |
| abi_residency_wideselect_null_entry | 27979.3 | 4108.5 | 3984.8 | n/a |
| abi_residency_wideselect_reused_buffer | 40301.2 | 2054439.6 | 2055218.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_wideselect_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.000 | 0.2% |
| abi_residency_wideselect_null_entry | 0.001 | 95.5% |
| abi_residency_wideselect_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2060253ns | 2060253ns | +0.12% |
| abi_residency_wideselect_null_entry | 6228ns | 6228ns | -99.70% |
| abi_residency_wideselect_reused_buffer | 2057791ns | 2057791ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_wideselect_reused_buffer | 2056320ns | base | --- | [2049879, 2059455] | --- | --- | --- | --- |
| abi_residency_wideselect_fresh_alloc | 2056466ns | no significant difference | [-4679, +14233]ns | [2052338, 2064113] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_wideselect_null_entry | 3984ns | -2052304.0ns (-99.8%) | [-2055365, -2046031]ns | [3848, 4122] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_wideselect_reused_buffer | abi_residency_wideselect_fresh_alloc | abi_residency_wideselect_null_entry |
|---|---|---|---|
| 1 | 2056087ns | -0.1% | -99.8% |
| 2 | 2056552ns | -0.2% | -99.8% |
| 3 | 2061428ns | -0.2% | -99.8% |
| 4 | 2055311ns | +0.6% | -99.8% |
| 5 | 2044448ns | +0.8% | -99.8% |
| 6 | 2057482ns | -0.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.161 | ok |
| abi_residency_wideselect_null_entry | -0.151 | ok |
| abi_residency_wideselect_reused_buffer | -0.095 | ok |

**Consistency summary:**

- **abi_residency_wideselect_fresh_alloc**: won 3/6, lost 2/6
- **abi_residency_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 6217394.0ns | 2057638.7ns | 302.2% | HIGH |
| abi_residency_wideselect_null_entry | 122455.8ns | 3984.8ns | 3073.1% | HIGH |
| abi_residency_wideselect_reused_buffer | 6209796.9ns | 2055218.1ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_residency_wideselect_fresh_alloc (n=6, range 2051777.5-2064112.7 ns)
  2051777.5 |########################################
  2052394.3 |########################################
  2053011.0 |
  2053627.8 |
  2054244.5 |
  2054861.3 |########################################
  2055478.1 |
  2056094.8 |
  2056711.6 |
  2057328.3 |
  2057945.1 |########################################
  2058561.9 |
  2059178.6 |
  2059795.4 |
  2060412.1 |########################################
  2061028.9 |
  2061645.7 |
  2062262.4 |
  2062879.2 |
  2063495.9 |
  (0 below, 1 above range)

abi_residency_wideselect_null_entry (n=6, range 3802.9-4122.1 ns)
   3802.9 |####################
   3818.9 |
   3834.8 |
   3850.8 |
   3866.7 |
   3882.7 |####################
   3898.7 |
   3914.6 |
   3930.6 |
   3946.5 |####################
   3962.5 |
   3978.5 |
   3994.4 |
   4010.4 |########################################
   4026.3 |
   4042.3 |
   4058.3 |
   4074.2 |
   4090.2 |
   4106.1 |
  (0 below, 1 above range)

abi_residency_wideselect_reused_buffer (n=6, range 2044447.9-2059455.2 ns)
  2044447.9 |########################################
  2045198.3 |
  2045948.6 |
  2046699.0 |
  2047449.4 |
  2048199.7 |
  2048950.1 |
  2049700.5 |
  2050450.8 |
  2051201.2 |
  2051951.6 |
  2052701.9 |
  2053452.3 |
  2054202.6 |
  2054953.0 |########################################
  2055703.4 |########################################
  2056453.7 |########################################
  2057204.1 |########################################
  2057954.5 |
  2058704.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_wideselect_fresh_alloc**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_null_entry**: bridge=3081.5% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_reused_buffer**: bridge=302.1% of algo (FFI overhead may distort results)
