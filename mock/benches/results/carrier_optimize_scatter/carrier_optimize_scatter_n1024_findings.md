# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_scatter_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_scatter_none has the worst median (36.18 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_scatter_canon at 33.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (carrier_opt_scatter_canon, carrier_opt_scatter_cse) are a dead heat (<1%)

carrier_opt_scatter_canon (33.16 us) and carrier_opt_scatter_cse (33.38 us) differ by 0.68%, inside the noise, even though the wider field spreads 9.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_opt_scatter_canon** at 33158.3 ns median (-8.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.09x (fastest 33158.3 ns, slowest 36180.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 37040ns | 35819ns | 35095ns | 35660ns | 40083ns | -3.91% |
| carrier_opt_scatter_canon | 35861ns | 35565ns | 35103ns | 35521ns | 36749ns | -6.97% |
| carrier_opt_scatter_cse | 36053ns | 35603ns | 34894ns | 35411ns | 37596ns | -6.47% |
| carrier_opt_scatter_dce | 37717ns | 37399ns | 37031ns | 37332ns | 38638ns | -2.16% |
| carrier_opt_scatter_fold | 38227ns | 38469ns | 36822ns | 37968ns | 39318ns | -0.83% |
| carrier_opt_scatter_none | 38548ns | 38557ns | 36870ns | 38135ns | 40005ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 34636ns | 32921ns | 37391ns | -4.17% | 0.030 |
| carrier_opt_scatter_canon | 33514ns | 32909ns | 34368ns | -7.27% | 0.031 |
| carrier_opt_scatter_cse | 33746ns | 32520ns | 35174ns | -6.63% | 0.030 |
| carrier_opt_scatter_dce | 35338ns | 34641ns | 36276ns | -2.23% | 0.029 |
| carrier_opt_scatter_fold | 35917ns | 34628ns | 36903ns | -0.63% | 0.029 |
| carrier_opt_scatter_none | 36143ns | 34522ns | 37484ns | base | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_scatter_all | 422537 | 2100455 | 0.201 | 0.95× |
| carrier_opt_scatter_canon | 417752 | 2118556 | 0.197 | 0.94× |
| carrier_opt_scatter_cse | 418677 | 2116838 | 0.198 | 0.94× |
| carrier_opt_scatter_dce | 440608 | 2223503 | 0.198 | 0.99× |
| carrier_opt_scatter_fold | 440352 | 2226494 | 0.198 | 0.99× |
| carrier_opt_scatter_none | 444986 | 2221916 | 0.200 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_opt_scatter_cse; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.031 | 97.1% |
| carrier_opt_scatter_canon | 0.031 | 98.1% |
| carrier_opt_scatter_cse | 0.031 | 97.4% |
| carrier_opt_scatter_dce | 0.029 | 92.9% |
| carrier_opt_scatter_fold | 0.028 | 90.0% |
| carrier_opt_scatter_none | 0.028 | 89.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 37040ns | 37040ns | -3.91% |
| carrier_opt_scatter_canon | 35861ns | 35861ns | -6.97% |
| carrier_opt_scatter_cse | 36053ns | 36053ns | -6.47% |
| carrier_opt_scatter_dce | 37717ns | 37717ns | -2.16% |
| carrier_opt_scatter_fold | 38227ns | 38227ns | -0.83% |
| carrier_opt_scatter_none | 38548ns | 38548ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 36180ns | base | --- | [34766, 37484] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 33489ns | no significant difference | [-3454, +1152]ns | [33028, 37391] | no | 0.3646 | 0.2188 | 0 |
| carrier_opt_scatter_canon | 33158ns | -2387.1ns (-6.6%) | [-3762, -1738]ns | [33016, 34368] | YES (adj: no) | 0.1563 | 0.0313 | 0 |
| carrier_opt_scatter_cse | 33384ns | -2279.8ns (-6.3%) | [-4386, -525]ns | [32681, 35174] | YES (adj: no) | 0.3646 | 0.2188 | 0 |
| carrier_opt_scatter_dce | 35008ns | no significant difference | [-2591, +1420]ns | [34729, 36276] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_scatter_fold | 36151ns | no significant difference | [-982, +582]ns | [34697, 36903] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_canon | carrier_opt_scatter_cse | carrier_opt_scatter_dce | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|
| 1 | 37011ns | -10.3% | -6.6% | -12.1% | -4.4% | +0.2% |
| 2 | 35009ns | -5.4% | -6.0% | -5.5% | +0.5% | -1.1% |
| 3 | 36874ns | -8.4% | -10.2% | -4.0% | -5.6% | -0.5% |
| 4 | 37956ns | +7.3% | -9.9% | -11.3% | -8.2% | -3.3% |
| 5 | 35486ns | -7.2% | -6.5% | -7.5% | -2.4% | -2.0% |
| 6 | 34522ns | -1.4% | -4.0% | +1.2% | +7.7% | +3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | -0.249 | moderate- |
| carrier_opt_scatter_canon | -0.329 | moderate- |
| carrier_opt_scatter_cse | -0.204 | moderate- |
| carrier_opt_scatter_dce | -0.134 | ok |
| carrier_opt_scatter_fold | -0.430 | moderate- |
| carrier_opt_scatter_none | -0.069 | ok |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 5/6, lost 1/6
- **carrier_opt_scatter_canon**: won 6/6, lost 0/6
- **carrier_opt_scatter_cse**: won 5/6, lost 1/6
- **carrier_opt_scatter_dce**: won 4/6, lost 2/6
- **carrier_opt_scatter_fold**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 102442.1ns | 34636.0ns | 295.8% | HIGH |
| carrier_opt_scatter_canon | 100699.3ns | 33514.0ns | 300.5% | HIGH |
| carrier_opt_scatter_cse | 101349.2ns | 33746.1ns | 300.3% | HIGH |
| carrier_opt_scatter_dce | 105928.8ns | 35338.0ns | 299.8% | HIGH |
| carrier_opt_scatter_fold | 107756.1ns | 35917.1ns | 300.0% | HIGH |
| carrier_opt_scatter_none | 108095.4ns | 36143.0ns | 299.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 32921.2-37391.2 ns)
  32921.2 |########################################
  33144.7 |####################
  33368.2 |
  33591.7 |####################
  33815.2 |
  34038.7 |####################
  34262.2 |
  34485.7 |
  34709.2 |
  34932.7 |
  35156.2 |
  35379.7 |
  35603.2 |
  35826.7 |
  36050.2 |
  36273.7 |
  36497.2 |
  36720.7 |
  36944.2 |
  37167.7 |
  (0 below, 1 above range)

carrier_opt_scatter_canon (n=6, range 32909.2-34367.5 ns)
  32909.2 |####################
  32982.1 |
  33055.0 |####################
  33127.9 |########################################
  33200.9 |
  33273.8 |
  33346.7 |
  33419.6 |
  33492.5 |
  33565.4 |
  33638.3 |
  33711.3 |
  33784.2 |
  33857.1 |
  33930.0 |
  34002.9 |
  34075.8 |
  34148.8 |####################
  34221.7 |
  34294.6 |
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 32520.4-35173.6 ns)
  32520.4 |########################################
  32653.1 |
  32785.7 |########################################
  32918.4 |
  33051.0 |########################################
  33183.7 |
  33316.3 |
  33449.0 |
  33581.7 |########################################
  33714.3 |
  33847.0 |
  33979.6 |
  34112.3 |
  34244.9 |
  34377.6 |
  34510.3 |
  34642.9 |
  34775.6 |
  34908.2 |########################################
  35040.9 |
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 34640.8-36276.4 ns)
  34640.8 |####################
  34722.6 |
  34804.4 |########################################
  34886.1 |
  34967.9 |
  35049.7 |
  35131.5 |####################
  35213.3 |
  35295.1 |####################
  35376.8 |
  35458.6 |
  35540.4 |
  35622.2 |
  35704.0 |
  35785.8 |
  35867.5 |
  35949.3 |
  36031.1 |
  36112.9 |
  36194.7 |
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 34627.5-36903.1 ns)
  34627.5 |####################
  34741.3 |####################
  34855.1 |
  34968.8 |
  35082.6 |
  35196.4 |
  35310.2 |
  35424.0 |
  35537.7 |####################
  35651.5 |
  35765.3 |
  35879.1 |
  35992.9 |
  36106.6 |
  36220.4 |
  36334.2 |
  36448.0 |
  36561.8 |
  36675.5 |########################################
  36789.3 |
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 34522.5-37483.5 ns)
  34522.5 |########################################
  34670.6 |
  34818.6 |
  34966.7 |########################################
  35114.7 |
  35262.8 |
  35410.8 |########################################
  35558.8 |
  35706.9 |
  35854.9 |
  36003.0 |
  36151.1 |
  36299.1 |
  36447.2 |
  36595.2 |
  36743.2 |########################################
  36891.3 |########################################
  37039.3 |
  37187.4 |
  37335.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_canon**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=300.4% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=299.2% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=300.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=298.9% of algo (FFI overhead may distort results)
