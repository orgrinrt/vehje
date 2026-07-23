# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_leaf_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_leaf_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_leaf_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_leaf_interp has the worst median (61.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_leaf_copypatch at 13.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_leaf_copypatch beats baseline by 78% (significant)

carrier_nat_leaf_copypatch is -47.53 us (78%) faster than baseline carrier_nat_leaf_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_leaf_interp is an outlier: 4.6x slower than the field

carrier_nat_leaf_interp (61.30 us) is 4.6x the fastest (13.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.6x the fastest

Fastest carrier_nat_leaf_copypatch (13.42 us) to slowest carrier_nat_leaf_interp (61.30 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_leaf_copypatch** at 13420.8 ns median (-78.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.57x (fastest 13420.8 ns, slowest 61296.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 16018ns | 15632ns | 15474ns | 15612ns | 16900ns | -74.79% |
| carrier_nat_leaf_direct | 16203ns | 16192ns | 15388ns | 15942ns | 17002ns | -74.50% |
| carrier_nat_leaf_interp | 63544ns | 63744ns | 58531ns | 62977ns | 66900ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 13763ns | 13298ns | 14531ns | -77.46% | 0.074 |
| carrier_nat_leaf_direct | 13869ns | 13150ns | 14585ns | -77.29% | 0.074 |
| carrier_nat_leaf_interp | 61071ns | 56327ns | 64341ns | base | 0.017 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 350849 | 998502 | 0.351 | 0.64× |
| carrier_nat_leaf_direct | 331158 | 927206 | 0.357 | 0.60× |
| carrier_nat_leaf_interp | 548400 | 1685789 | 0.325 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.078 Gops/s** (carrier_nat_leaf_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.076 | 98.0% |
| carrier_nat_leaf_direct | 0.074 | 95.0% |
| carrier_nat_leaf_interp | 0.017 | 21.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_leaf_copypatch | 16018ns | 16018ns | -74.79% |
| carrier_nat_leaf_direct | 16203ns | 16203ns | -74.50% |
| carrier_nat_leaf_interp | 63544ns | 63544ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_leaf_interp | 61297ns | base | --- | [57576, 64341] | --- | --- | --- | --- |
| carrier_nat_leaf_copypatch | 13421ns | -47534.2ns (-77.5%) | [-50152, -44239]ns | [13337, 14531] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_leaf_direct | 13839ns | -47515.2ns (-77.5%) | [-50354, -43737]ns | [13183, 14585] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_leaf_interp | carrier_nat_leaf_copypatch | carrier_nat_leaf_direct |
|---|---|---|---|
| 1 | 63552ns | -77.0% | -79.3% |
| 2 | 59610ns | -77.4% | -75.9% |
| 3 | 56327ns | -76.3% | -76.3% |
| 4 | 62983ns | -78.8% | -79.0% |
| 5 | 58825ns | -77.4% | -75.7% |
| 6 | 65130ns | -77.8% | -77.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_leaf_copypatch | -0.069 | ok |
| carrier_nat_leaf_direct | -0.051 | ok |
| carrier_nat_leaf_interp | -0.343 | moderate- |

**Consistency summary:**

- **carrier_nat_leaf_copypatch**: won 6/6, lost 0/6
- **carrier_nat_leaf_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 99450.3ns | 13762.8ns | 722.6% | HIGH |
| carrier_nat_leaf_direct | 93688.6ns | 13868.9ns | 675.5% | HIGH |
| carrier_nat_leaf_interp | 122040.3ns | 61071.1ns | 199.8% | HIGH |

## Distribution (algo ns)

```
carrier_nat_leaf_copypatch (n=6, range 13298.3-14530.8 ns)
  13298.3 |####################
  13359.9 |########################################
  13421.6 |####################
  13483.2 |
  13544.8 |
  13606.4 |
  13668.1 |
  13729.7 |
  13791.3 |
  13852.9 |
  13914.6 |
  13976.2 |
  14037.8 |
  14099.5 |
  14161.1 |
  14222.7 |
  14284.3 |
  14346.0 |
  14407.6 |####################
  14469.2 |
  (0 below, 1 above range)

carrier_nat_leaf_direct (n=6, range 13149.6-14585.2 ns)
  13149.6 |########################################
  13221.4 |
  13293.2 |
  13364.9 |####################
  13436.7 |
  13508.5 |
  13580.3 |
  13652.1 |
  13723.8 |
  13795.6 |
  13867.4 |
  13939.2 |
  14011.0 |
  14082.7 |
  14154.5 |
  14226.3 |
  14298.1 |########################################
  14369.9 |
  14441.6 |
  14513.4 |
  (0 below, 1 above range)

carrier_nat_leaf_interp (n=6, range 56327.1-64340.8 ns)
  56327.1 |########################################
  56727.8 |
  57128.5 |
  57529.2 |
  57929.8 |
  58330.5 |
  58731.2 |########################################
  59131.9 |
  59532.6 |########################################
  59933.3 |
  60334.0 |
  60734.7 |
  61135.3 |
  61536.0 |
  61936.7 |
  62337.4 |
  62738.1 |########################################
  63138.8 |
  63539.5 |########################################
  63940.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_leaf_copypatch**: bridge=725.1% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_direct**: bridge=677.0% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_interp**: bridge=200.4% of algo (FFI overhead may distort results)
