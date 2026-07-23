# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 3.21 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 27381% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (3.21 us) leads abi_soa_win_real_soa_payload (883.28 us) by 27381%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 668.6x slower than the field

abi_soa_win_real_scalar_payload (2.15 ms) is 668.6x the fastest (3.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_scalar_payload shows alternating (throttle bounce) (autocorr -0.74)

abi_soa_win_real_scalar_payload's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 668.6x the fastest

Fastest abi_soa_win_real_null_entry (3.21 us) to slowest abi_soa_win_real_scalar_payload (2.15 ms): 668.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 3214.1 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 668.58x (fastest 3214.1 ns, slowest 2148925.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 5518ns | 5499ns | 5304ns | 5485ns | 5674ns | -99.74% |
| abi_soa_win_real_scalar_payload | 2152946ns | 2152009ns | 2145944ns | 2150680ns | 2159847ns | base |
| abi_soa_win_real_soa_payload | 885628ns | 885828ns | 884581ns | 885494ns | 886354ns | -58.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 3216ns | 3097ns | 3286ns | -99.85% | 0.080 |
| abi_soa_win_real_scalar_payload | 2149871ns | 2142996ns | 2156627ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 883071ns | 882053ns | 883734ns | -58.92% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 27775.6 | 3247.6 | 3215.6 | n/a |
| abi_soa_win_real_scalar_payload | 57364.0 | 2150637.9 | 2149871.1 | n/a |
| abi_soa_win_real_soa_payload | 40281.0 | 884326.2 | 883070.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.080 | 96.4% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 5518ns | 5518ns | -99.74% |
| abi_soa_win_real_scalar_payload | 2152946ns | 2152946ns | base |
| abi_soa_win_real_soa_payload | 885628ns | 885628ns | -58.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2148925ns | base | --- | [2144061, 2156627] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 3214ns | -2145711.0ns (-99.9%) | [-2153420, -2140835]ns | [3147, 3286] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 883276ns | -1265975.8ns (-58.9%) | [-1273641, -1260785]ns | [882202, 883734] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2157615ns | -99.8% | -59.1% |
| 2 | 2142996ns | -99.9% | -58.8% |
| 3 | 2155640ns | -99.9% | -59.0% |
| 4 | 2145126ns | -99.8% | -58.8% |
| 5 | 2151041ns | -99.9% | -59.0% |
| 6 | 2146810ns | -99.9% | -58.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.158 | ok |
| abi_soa_win_real_scalar_payload | -0.745 | HIGH- (thermal bounce) |
| abi_soa_win_real_soa_payload | -0.142 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 121176.3ns | 3215.6ns | 3768.4% | HIGH |
| abi_soa_win_real_scalar_payload | 6510446.2ns | 2149871.1ns | 302.8% | HIGH |
| abi_soa_win_real_soa_payload | 2693559.1ns | 883070.7ns | 305.0% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 3097.1-3286.1 ns)
   3097.1 |########################################
   3106.5 |
   3116.0 |
   3125.4 |
   3134.9 |
   3144.3 |
   3153.8 |
   3163.2 |
   3172.7 |
   3182.1 |
   3191.6 |########################################
   3201.0 |########################################
   3210.5 |
   3219.9 |########################################
   3229.4 |
   3238.8 |
   3248.3 |########################################
   3257.7 |
   3267.2 |
   3276.6 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2142995.8-2156627.3 ns)
  2142995.8 |########################################
  2143677.4 |
  2144358.9 |
  2145040.5 |########################################
  2145722.1 |
  2146403.7 |########################################
  2147085.2 |
  2147766.8 |
  2148448.4 |
  2149130.0 |
  2149811.5 |
  2150493.1 |########################################
  2151174.7 |
  2151856.3 |
  2152537.8 |
  2153219.4 |
  2153901.0 |
  2154582.6 |
  2155264.1 |########################################
  2155945.7 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 882052.9-883733.8 ns)
  882052.9 |########################################
  882136.9 |
  882221.0 |
  882305.0 |########################################
  882389.1 |
  882473.1 |
  882557.2 |
  882641.2 |
  882725.2 |
  882809.3 |
  882893.3 |
  882977.4 |
  883061.4 |
  883145.5 |########################################
  883229.5 |
  883313.5 |########################################
  883397.6 |
  883481.6 |########################################
  883565.7 |
  883649.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=3766.5% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=304.8% of algo (FFI overhead may distort results)
