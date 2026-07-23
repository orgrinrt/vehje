# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_tight_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_tight_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_tight_emitdirect shows alternating (throttle bounce) (autocorr -0.52)

carrier_setup_tight_emitdirect's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_tight_parse)

The baseline carrier_setup_tight_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_emitdirect, carrier_setup_tight_stackcompile, carrier_setup_tight_emitcopypatch} vs {carrier_setup_tight_optall} (276% apart)

The field splits into a fast tier {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_emitdirect, carrier_setup_tight_stackcompile, carrier_setup_tight_emitcopypatch} and a slow tier {carrier_setup_tight_optall} with a 276% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 829.55 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_tight_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 221904ns | 223116ns | 218453ns | 221593ns | 224095ns | +10104.05% |
| carrier_setup_tight_emitdirect | 66843ns | 66599ns | 65459ns | 66503ns | 68045ns | +2973.70% |
| carrier_setup_tight_optall | 829192ns | 831853ns | 812108ns | 827013ns | 841001ns | +38029.59% |
| carrier_setup_tight_parse | 2175ns | 2176ns | 2110ns | 2161ns | 2228ns | base |
| carrier_setup_tight_predecode | 38711ns | 38358ns | 37580ns | 38327ns | 39854ns | +1680.10% |
| carrier_setup_tight_stackcompile | 78252ns | 78329ns | 76564ns | 77889ns | 79640ns | +3498.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 219705ns | 216319ns | 221879ns | +0.00% | 0.005 |
| carrier_setup_tight_emitdirect | 64679ns | 63342ns | 65815ns | +0.00% | 0.016 |
| carrier_setup_tight_optall | 826897ns | 809790ns | 838696ns | +0.00% | 0.001 |
| carrier_setup_tight_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_tight_predecode | 36515ns | 35435ns | 37562ns | +0.00% | 0.028 |
| carrier_setup_tight_stackcompile | 76102ns | 74419ns | 77472ns | +0.00% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 1365692 | 5747505 | 0.238 | 94.96× |
| carrier_setup_tight_emitdirect | 609986 | 3594753 | 0.170 | 42.42× |
| carrier_setup_tight_optall | 5140134 | 24451031 | 0.210 | 357.42× |
| carrier_setup_tight_parse | 14381 | 52935 | 0.272 | 1.00× |
| carrier_setup_tight_predecode | 457095 | 3312424 | 0.138 | 31.78× |
| carrier_setup_tight_stackcompile | 721036 | 4568675 | 0.158 | 50.14× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_setup_tight_predecode; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_tight_emitcopypatch | 0.005 | 16.0% |
| carrier_setup_tight_emitdirect | 0.016 | 55.0% |
| carrier_setup_tight_optall | 0.001 | 4.3% |
| carrier_setup_tight_predecode | 0.028 | 97.9% |
| carrier_setup_tight_stackcompile | 0.013 | 46.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 221904ns | 221904ns | +10104.05% |
| carrier_setup_tight_emitdirect | 66843ns | 66843ns | +2973.70% |
| carrier_setup_tight_optall | 829192ns | 829192ns | +38029.59% |
| carrier_setup_tight_parse | 2175ns | 2175ns | base |
| carrier_setup_tight_predecode | 38711ns | 38711ns | +1680.10% |
| carrier_setup_tight_stackcompile | 78252ns | 78252ns | +3498.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_tight_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_tight_emitcopypatch | 220889ns | +220889.0ns (+0.0%) | [+216349, +221879]ns | [216349, 221879] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_emitdirect | 64456ns | +64455.8ns (+0.0%) | [+63765, +65815]ns | [63765, 65815] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_optall | 829552ns | +829551.6ns (+0.0%) | [+812445, +838696]ns | [812445, 838696] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_predecode | 36207ns | +36206.9ns (+0.0%) | [+35775, +37562]ns | [35775, 37562] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_stackcompile | 76185ns | +76185.0ns (+0.0%) | [+74650, +77472]ns | [74650, 77472] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_tight_parse | carrier_setup_tight_emitcopypatch | carrier_setup_tight_emitdirect | carrier_setup_tight_optall | carrier_setup_tight_predecode | carrier_setup_tight_stackcompile |
|---|---|---|---|---|---|---|
| 1 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 2 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 3 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 4 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 5 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 6 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_setup_tight_emitcopypatch | 0.489 | moderate+ |
| carrier_setup_tight_emitdirect | -0.519 | HIGH- (thermal bounce) |
| carrier_setup_tight_optall | -0.198 | ok |
| carrier_setup_tight_parse | 0.000 | ok |
| carrier_setup_tight_predecode | -0.428 | moderate- |
| carrier_setup_tight_stackcompile | -0.172 | ok |

**Consistency summary:**

- **carrier_setup_tight_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_tight_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_tight_optall**: won 0/6, lost 0/6
- **carrier_setup_tight_predecode**: won 0/6, lost 0/6
- **carrier_setup_tight_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 219318.9ns | 219705.5ns | 99.8% | HIGH |
| carrier_setup_tight_emitdirect | 129369.7ns | 64678.6ns | 200.0% | HIGH |
| carrier_setup_tight_optall | 826329.2ns | 826897.4ns | 99.9% | HIGH |
| carrier_setup_tight_parse | 1458.2ns | 0.0ns | 0.0% |  |
| carrier_setup_tight_predecode | 109316.8ns | 36514.6ns | 299.4% | HIGH |
| carrier_setup_tight_stackcompile | 151579.9ns | 76102.4ns | 199.2% | HIGH |

## Distribution (algo ns)

```
carrier_setup_tight_emitcopypatch (n=6, range 216319.2-221878.8 ns)
  216319.2 |########################################
  216597.2 |
  216875.2 |
  217153.1 |
  217431.1 |
  217709.1 |
  217987.1 |
  218265.0 |
  218543.0 |
  218821.0 |
  219099.0 |
  219377.0 |
  219654.9 |
  219932.9 |
  220210.9 |
  220488.9 |
  220766.8 |########################################
  221044.8 |
  221322.8 |####################
  221600.8 |
  (0 below, 1 above range)

carrier_setup_tight_emitdirect (n=6, range 63342.5-65814.6 ns)
  63342.5 |########################################
  63466.1 |
  63589.7 |
  63713.3 |
  63836.9 |
  63960.5 |
  64084.1 |########################################
  64207.7 |
  64331.3 |########################################
  64454.9 |########################################
  64578.6 |
  64702.2 |########################################
  64825.8 |
  64949.4 |
  65073.0 |
  65196.6 |
  65320.2 |
  65443.8 |
  65567.4 |
  65691.0 |
  (0 below, 1 above range)

carrier_setup_tight_optall (n=6, range 809789.6-838695.8 ns)
  809789.6 |####################
  811234.9 |
  812680.2 |
  814125.5 |####################
  815570.8 |
  817016.2 |
  818461.5 |
  819906.8 |
  821352.1 |
  822797.4 |
  824242.7 |
  825688.0 |
  827133.3 |
  828578.7 |########################################
  830024.0 |
  831469.3 |
  832914.6 |####################
  834359.9 |
  835805.2 |
  837250.5 |
  (0 below, 1 above range)

carrier_setup_tight_predecode (n=6, range 35435.0-37561.9 ns)
  35435.0 |####################
  35541.3 |
  35647.7 |
  35754.0 |
  35860.4 |
  35966.7 |
  36073.1 |########################################
  36179.4 |
  36285.7 |####################
  36392.1 |
  36498.4 |
  36604.8 |
  36711.1 |
  36817.5 |
  36923.8 |
  37030.1 |
  37136.5 |
  37242.8 |
  37349.2 |####################
  37455.5 |
  (0 below, 1 above range)

carrier_setup_tight_stackcompile (n=6, range 74419.2-77472.3 ns)
  74419.2 |####################
  74571.9 |
  74724.5 |
  74877.2 |####################
  75029.8 |
  75182.5 |
  75335.1 |
  75487.8 |
  75640.4 |
  75793.1 |
  75945.8 |
  76098.4 |########################################
  76251.1 |
  76403.7 |
  76556.4 |
  76709.0 |
  76861.7 |
  77014.3 |
  77167.0 |####################
  77319.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_tight_emitcopypatch**: bridge=99.7% of algo (FFI overhead may distort results)
- **carrier_setup_tight_emitdirect**: bridge=199.8% of algo (FFI overhead may distort results)
- **carrier_setup_tight_optall**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_tight_predecode**: bridge=299.1% of algo (FFI overhead may distort results)
- **carrier_setup_tight_stackcompile**: bridge=198.2% of algo (FFI overhead may distort results)
