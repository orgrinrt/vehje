# Carrier interpreter-composition matrix: PMU re-run summary (corrected)

This is the full-run summary for the 58-bench carrier matrix, re-run in one
coherent batch with the PMU pass armed (`MOCKSPACE_BENCH_PERF=1`), so every
result carries the instructions/cycles columns and a derived IPC. The findings
below supersede the first-run summary: a four-expert review panel
(`mock/research/202607230922_carrier-matrix-result-review-panel/`) audited the
first run and its methodology, and several first-run headlines were artifacts of
the reporting layer, not real effects. Each corrected finding names what changed
and why.

The single most important framing correction: the honest column is **N=16384**,
not the small sizes. At small N the branch predictor memorizes the whole opcode
stream, so dispatch-shape differences collapse to ties (the "dispatch barely
matters" reading of the first run). The N=1024 to N=4096 cost step is the
predictor running out of capacity, after which the near-cold behaviour shows.
Read every dispatch and interpreter table at its largest size.

The PMU caveat, stated up front: this machine's kperf path exposes the two fixed
counters (instructions retired and cycles) only, not branch-mispredict or
L1D-miss events. So IPC corroborates the instruction-count and stall stories, but
it cannot by itself adjudicate predictor-versus-cache for the N=4096 step; that
adjudication rests on the panel's structural argument (below), which the IPC data
is consistent with.

## Headline findings

1. **Dispatch shape matters 10 to 35% at the honest size; the small-N ties were
   a predictor-memorization artifact.** At N=16384 on real control flow
   (`carrier_dispatch_real`), `fntable` leads at 0.90x while `threaded` trails at
   1.12x and `ifchainlin` at 1.23x, a ~35% span. On straight-line madd/tight the
   span tightens to ~15%. The first run read the memorized small-N column and
   concluded shape was irrelevant; it is not, once the predictor is past its
   capacity. The `carrier_entgrid` family isolates the mechanism directly (finding
   11).

2. **Threading wins only where control flow is real, and the PMU shows the
   mechanism: it retires far fewer instructions.** On the CFG workload
   (`carrier_cfg`), `trace` runs at 0.44x of `switch`, and the counters confirm
   the win is structural: trace retires ~40M instructions to switch's ~90M (2.25x
   fewer) at N=16384. Threading real branches removes dispatch work that straight
   line bytecode never had, which is why on the straight-line dispatch families
   `threaded` is slower (1.12 to 1.17x), not faster. Preserve-none threading is a
   control-flow lever, not a general dispatch win.

3. **Record width is a null result: a density choice, refuting 24-byte-optimal.**
   `carrier_layout` holds every width (12/16/20/24/32) within 1% at every size and
   profile, with IPC flat across widths. Width buys nothing for throughput; it is
   purely a cache-footprint/density knob. Choose REC16 for density (panel Q5).

4. **Register VM beats stack VM decisively, on the corrected symmetric baseline.**
   With both cells predecoded and folding the same live-out set (the panel's fix
   for the first run's three-axis asymmetry), register wins every profile:
   `carrier_residual` shows tight 3.06x, madd 2.29x, wideselect 1.63x, leaf 1.53x,
   real/scatter 1.41x. The first run's 1.91x headline was a composite that
   burdened the register side (wire decode plus a full-array checksum); the
   corrected comparison still favors register. The PMU corroborates: register runs
   at higher IPC (0.36 to 1.41) than stack (0.24 to 0.53), the stack VM stalling on
   its operand-stack memory traffic. The stack-bytecode middle tier is closed on
   this evidence.

5. **Value representation: nan-boxing beats a static u64 at scale.**
   `carrier_valrepr` at N=16384: `nanbox` 0.80x, `tagged` 0.93x, `static` 1.00x
   baseline, so nan-boxing wins ~20% at the honest size. The static u64's higher
   IPC (0.78 vs nanbox 0.45) is doing more memory work, not going faster. One-shape
   choice, low-to-medium confidence.

6. **Optimizer: ship fold+CSE+DCE always on; eqsat is OUT. (Corrected.)** The
   first-run finding claimed "CSE+eqsat recovers, run the whole pipeline"; the
   run's own data contradicts it. The full pipeline `all` wins or ties everywhere,
   but eqsat's marginal contribution to it is zero-or-negative: on
   `carrier_optimize_scatter`, `cse` alone (1.00x) matches `all`, and on
   `wideselect` `all` leads only marginally. Standalone eqsat inflated madd (the
   first run's 142x was eqsat-alone pessimization presented as a win, not a
   speedup). The real optimizer payoff is program-dependent: 13 to 25x on
   madd/tight (dead code and constant folding over generated redundancy), ~1.17x on
   real (little to remove). Ship fold+CSE+DCE; park eqsat/DAG-aware extraction
   behind a named trigger (panel Q2).

7. **Native codegen: a 2.6 to 4.3x warm floor; direct isel and copy-and-patch tie
   at warm execution, but that is the wrong axis. (Tags un-inverted.)** The cell
   tagged `direct` is direct instruction-selection; the cell tagged `copypatch` is
   the real Xu-Kjolstad copy-and-patch (OOPSLA 2021). At warm execution they are
   within 1% of each other (`carrier_native_real`: both ~0.41ms vs interp 2.65ms =
   6.48x; native_tight 1.90x, native_madd 1.21x). Warm parity says nothing about
   the axis that actually separates them, which is compile cost. See finding 8.

8. **Compile cost S is now measured, and it reorders the native tiers. (New; the
   panel's number-one gap.)** `carrier_setup` times the construction itself. Direct
   isel (`emitdirect`) compiles ~2x cheaper than copy-and-patch (`emitcopypatch`)
   on every profile (setup_real: emitdirect 0.55x vs emitcopypatch 1.00x baseline).
   Predecode is by far the cheapest form to build (0.12 to 0.31x) and cheap to run;
   `optall` is the most expensive construction (3 to 8x). So for short-lived
   residuals, direct isel is the better native tier (equal warm speed, half the
   compile cost), and predecode dominates when the residual runs only a few times.
   With S measured alongside the per-iteration I, the tier breakeven `k*` (the run
   count at which a heavier-to-build tier repays itself) is now computable from the
   matrix rather than unknowable.

9. **Vertical SIMD is the single biggest lever: vert8 up to 4.8x. (Magnitude
   corrected from 6x.)** `carrier_vertical` at N=16384: `vert8` 0.21x on
   real/scatter (scalar 17.16ms to vert8 3.59ms = 4.8x), 0.34 to 0.43x on
   leaf/tight. The first run's 6x included ~20% wire-versus-predecoded decode
   asymmetry in the scalar baseline; against a predecoded scalar the win is ~4.8x.
   This maps directly onto vehje's column evaluation and is the one ABI-shaping
   result (finding below).

10. **Native throughput ceiling is ~1.5x over the best interpreter.**
    `carrier_native_ceiling`: native 0.67x of interp at N=1024 (28.9ms vs 43.1ms =
    1.49x). The interpreter at its best (predecoded, optimized) is within ~1.5x of
    native throughput; the larger 2.6 to 4.3x native wins are all measured against
    the plain warm interpreter, not its best form.

11. **Dispatch cost is regime-dependent on op-stream predictability. (New.)**
    `carrier_entgrid` sweeps op-correlation x locality window. The
    high-correlation variants (`900_*`) run at 0.40x, the zero-correlation
    (random-stream) variants (`0_*`) at 1.00x baseline: a 2.5x span from
    predictability alone, flat across the locality window. This is the mechanism
    under the small-N dispatch ties: a correlated or memorized op stream hides
    dispatch cost, a random stream exposes it. It is why finding 1's honest reading
    needs the near-cold size.

## What this says for vehje's runtime tiers

- **Baseline tier: predecoded register interpreter.** Cheapest form to build
  (finding 8), beats stack decisively (finding 4), and dispatch shape is a
  second-order 10 to 35% choice on top (finding 1). Use `switch` or `fntable`
  dispatch; add threading only for real-control-flow residuals (finding 2).
- **Middle tier: predecoded register + fold/CSE/DCE.** The optimizer pays 13 to
  25x on redundant generated code and never hurts once eqsat is dropped (finding
  6). Stack bytecode is closed out (finding 4). This is panel Q4's redefinition.
- **Native tier: direct isel for short-lived residuals, measured against k\*.**
  Direct isel and copy-and-patch tie at warm speed but direct isel is ~2x cheaper
  to compile (finding 8); native's honest ceiling over the best interpreter is
  ~1.5x (finding 10), larger only over the plain interpreter.
- **The column-eval lever: vert8.** The 4.8x vertical win (finding 9) is the
  biggest single lever and the one non-deferrable ABI decision: reserve a batched
  `execute(residual, inputs[W], outputs[W])` entry in the runtime C ABI now, so
  column evaluation has the batched arity it needs (panel Q4).

## Caveats

- Values are per-16-execution warm-throughput means with integer-truncation bias,
  not cold latency; the calibrated re-warm makes the surrounding workload
  irrelevant to `algo_ns`. Read them as steady-state throughput.
- PMU counters are instructions and cycles only (fixed counters); IPC corroborates
  but does not isolate branch-mispredict or cache-miss events. The N=4096 step's
  predictor-versus-cache attribution rests on the panel's structural argument
  (madd/tight share real's footprint yet show no step; nullfloor is flat; the step
  orders by op-stream entropy), which the IPC data is consistent with.
- `carrier_setup`'s `parse` cell reads 0.0ns because parsing a REC24 buffer is
  below the counter floor even under calibration; treat parse as free relative to
  the other constructions.
- Every `nullfloor` / `null` row is the null-dispatch floor cell (a real
  interpreter loop with the opcode branch removed); it is the subtrahend for
  isolating pure dispatch cost, not a competing variant.

## Full matrix

The per-bench tables follow (median `algo_ns` across replicated passes, with the
IPC column from the PMU pass; the leading `**<-**` marks the fastest variant at
the largest size, and `vs base` is the ratio against each family's reference
cell).

## carrier_cfg

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| trace **<-** | 26.23us | 100.43us | 407.27us | 1.603ms | 6.418ms | 0.44x | 0.25 |
| threaded | 47.13us | 179.86us | 703.61us | 2.854ms | 11.412ms | 0.79x | 0.52 |
| fntable | 57.63us | 218.22us | 865.99us | 3.504ms | 14.067ms | 0.97x | 0.42 |
| switch | 57.17us | 223.64us | 902.90us | 3.606ms | 14.444ms | 1.00x (base) | 0.48 |

_baseline: `switch`; fastest at N=16384: `trace`; IPC = instructions/cycles from the PMU pass_


## carrier_coldcycle_leaf

| variant | N=64 | N=256 | N=1024 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|
| null **<-** | 1.06us | 5.12us | 28.34us | 0.54x | 0.31 |
| threaded | 1.34us | 5.51us | 51.25us | 0.98x | 0.49 |
| switch | 1.51us | 9.42us | 52.04us | 1.00x (base) | 0.52 |
| fntable | 2.97us | 16.21us | 64.46us | 1.24x | 0.50 |

_baseline: `switch`; fastest at N=1024: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_coldcycle_madd

| variant | N=64 | N=256 | N=1024 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|
| null **<-** | 1.82us | 9.57us | 42.70us | 0.90x | 0.37 |
| switch | 2.28us | 11.09us | 47.43us | 1.00x (base) | 0.44 |
| threaded | 2.27us | 11.06us | 47.53us | 1.00x | 0.42 |
| fntable | 2.52us | 11.73us | 48.50us | 1.02x | 0.37 |

_baseline: `switch`; fastest at N=1024: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_coldcycle_real

| variant | N=64 | N=256 | N=1024 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|
| null **<-** | 1.37us | 6.02us | 31.95us | 0.24x | 0.33 |
| switch | 2.14us | 29.45us | 133.62us | 1.00x (base) | 1.23 |
| threaded | 2.00us | 29.62us | 138.90us | 1.04x | 1.19 |
| fntable | 3.00us | 34.18us | 151.54us | 1.13x | 1.10 |

_baseline: `switch`; fastest at N=1024: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_coldcycle_scatter

| variant | N=64 | N=256 | N=1024 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|
| null **<-** | 1.24us | 5.83us | 31.76us | 0.24x | 0.29 |
| switch | 2.10us | 28.94us | 133.09us | 1.00x (base) | 1.22 |
| threaded | 2.06us | 30.05us | 140.10us | 1.05x | 1.19 |
| fntable | 2.97us | 33.78us | 151.17us | 1.14x | 1.08 |

_baseline: `switch`; fastest at N=1024: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_coldcycle_tight

| variant | N=64 | N=256 | N=1024 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|
| null **<-** | 1.54us | 7.16us | 31.84us | 0.76x | 0.27 |
| threaded | 1.67us | 8.27us | 36.50us | 0.87x | 0.33 |
| switch | 1.77us | 8.35us | 42.10us | 1.00x (base) | 0.40 |
| fntable | 2.24us | 10.94us | 51.64us | 1.23x | 0.40 |

_baseline: `switch`; fastest at N=1024: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_coldcycle_wideselect

| variant | N=64 | N=256 | N=1024 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|
| null **<-** | 1.25us | 6.00us | 30.44us | 0.28x | 0.28 |
| switch | 2.16us | 22.15us | 107.73us | 1.00x (base) | 0.99 |
| threaded | 1.98us | 19.23us | 109.61us | 1.02x | 0.90 |
| fntable | 2.80us | 27.45us | 122.48us | 1.14x | 0.84 |

_baseline: `switch`; fastest at N=1024: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_dispatch_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.56us | 6.97us | 26.88us | 109.99us | 539.93us | 0.50x | 0.22 |
| bittree | 2.29us | 10.02us | 38.16us | 165.31us | 782.51us | 0.73x | 0.27 |
| ifchainlin | 2.80us | 12.02us | 47.95us | 210.07us | 984.07us | 0.92x | 0.26 |
| ifchainasc | 1.93us | 9.37us | 46.14us | 231.17us | 1.075ms | 1.00x | 0.36 |
| switch | 1.99us | 9.23us | 43.97us | 221.36us | 1.075ms | 1.00x (base) | 0.37 |
| ifchain | 2.04us | 9.38us | 46.14us | 241.92us | 1.085ms | 1.01x | 0.37 |
| fntable | 2.47us | 11.71us | 52.08us | 264.51us | 1.153ms | 1.07x | 0.28 |
| threaded | 2.69us | 11.16us | 42.70us | 224.88us | 1.246ms | 1.16x | 0.31 |

_baseline: `switch`; fastest at N=16384: `nullfloor`; IPC = instructions/cycles from the PMU pass_


## carrier_dispatch_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.92us | 10.41us | 41.11us | 161.13us | 655.32us | 0.83x | 0.24 |
| ifchainlin | 2.32us | 10.70us | 44.23us | 171.88us | 697.74us | 0.89x | 0.21 |
| bittree | 2.41us | 11.21us | 45.17us | 179.43us | 717.95us | 0.91x | 0.23 |
| ifchain | 2.63us | 12.55us | 48.19us | 197.25us | 783.87us | 1.00x | 0.25 |
| ifchainasc | 2.61us | 11.87us | 47.88us | 195.99us | 784.15us | 1.00x | 0.25 |
| switch | 2.62us | 12.33us | 48.61us | 200.21us | 787.40us | 1.00x (base) | 0.26 |
| fntable | 2.89us | 13.44us | 52.89us | 216.35us | 871.80us | 1.11x | 0.20 |
| threaded | 2.94us | 12.37us | 52.99us | 216.59us | 912.86us | 1.16x | 0.22 |

_baseline: `switch`; fastest at N=16384: `nullfloor`; IPC = instructions/cycles from the PMU pass_


## carrier_dispatch_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.67us | 7.69us | 29.44us | 115.38us | 513.74us | 0.20x | 0.20 |
| fntable | 3.05us | 13.78us | 52.75us | 555.57us | 2.355ms | 0.90x | 0.56 |
| bittree | 2.85us | 12.91us | 50.61us | 230.91us | 2.608ms | 0.99x | 0.84 |
| ifchain | 2.37us | 10.97us | 40.78us | 571.92us | 2.609ms | 0.99x | 0.87 |
| ifchainasc | 2.38us | 10.28us | 41.72us | 564.44us | 2.616ms | 1.00x | 0.87 |
| switch | 2.40us | 11.05us | 41.37us | 537.16us | 2.623ms | 1.00x (base) | 0.89 |
| threaded | 2.83us | 11.82us | 46.39us | 628.67us | 2.929ms | 1.12x | 0.72 |
| ifchainlin | 5.13us | 22.01us | 85.11us | 479.36us | 3.226ms | 1.23x | 0.39 |

_baseline: `switch`; fastest at N=16384: `nullfloor`; IPC = instructions/cycles from the PMU pass_


## carrier_dispatch_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.75us | 7.49us | 28.89us | 116.91us | 563.73us | 0.22x | 0.21 |
| fntable | 3.18us | 13.83us | 53.75us | 556.65us | 2.356ms | 0.90x | 0.55 |
| bittree | 2.98us | 13.07us | 50.24us | 290.72us | 2.579ms | 0.98x | 0.81 |
| ifchain | 2.51us | 11.10us | 42.74us | 583.30us | 2.602ms | 0.99x | 0.85 |
| switch | 2.44us | 11.09us | 41.52us | 565.86us | 2.621ms | 1.00x (base) | 0.87 |
| ifchainasc | 2.52us | 11.08us | 42.28us | 565.03us | 2.625ms | 1.00x | 0.85 |
| threaded | 2.86us | 12.60us | 46.18us | 589.17us | 2.941ms | 1.12x | 0.70 |
| ifchainlin | 5.75us | 22.83us | 89.83us | 555.88us | 3.264ms | 1.25x | 0.39 |

_baseline: `switch`; fastest at N=16384: `nullfloor`; IPC = instructions/cycles from the PMU pass_


## carrier_dispatch_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.77us | 8.12us | 31.04us | 126.36us | 510.57us | 0.61x | 0.19 |
| bittree | 2.51us | 11.60us | 43.13us | 174.31us | 729.31us | 0.87x | 0.23 |
| switch | 2.25us | 11.12us | 42.83us | 186.84us | 835.92us | 1.00x (base) | 0.27 |
| ifchainasc | 2.26us | 10.75us | 43.54us | 187.22us | 838.12us | 1.00x | 0.27 |
| ifchain | 2.27us | 11.19us | 43.45us | 188.57us | 845.62us | 1.01x | 0.27 |
| fntable | 3.07us | 12.96us | 52.63us | 214.41us | 946.51us | 1.13x | 0.22 |
| threaded | 2.91us | 12.41us | 46.05us | 198.32us | 975.59us | 1.17x | 0.23 |
| ifchainlin | 3.75us | 16.69us | 57.95us | 238.71us | 983.87us | 1.18x | 0.21 |

_baseline: `switch`; fastest at N=16384: `nullfloor`; IPC = instructions/cycles from the PMU pass_


## carrier_dispatch_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.79us | 7.84us | 29.75us | 118.04us | 512.56us | 0.24x | 0.19 |
| bittree | 2.77us | 12.10us | 48.23us | 187.78us | 1.691ms | 0.79x | 0.52 |
| fntable | 2.92us | 12.36us | 50.75us | 452.24us | 2.039ms | 0.95x | 0.47 |
| switch | 2.36us | 10.42us | 40.21us | 442.18us | 2.140ms | 1.00x (base) | 0.70 |
| ifchainasc | 2.33us | 10.21us | 39.98us | 429.39us | 2.151ms | 1.00x | 0.68 |
| ifchain | 2.51us | 10.79us | 42.49us | 458.78us | 2.199ms | 1.03x | 0.70 |
| threaded | 2.77us | 11.69us | 44.06us | 446.35us | 2.351ms | 1.10x | 0.56 |
| ifchainlin | 5.23us | 23.92us | 96.94us | 515.41us | 2.737ms | 1.28x | 0.32 |

_baseline: `switch`; fastest at N=16384: `nullfloor`; IPC = instructions/cycles from the PMU pass_


## carrier_entgrid

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| 900_w64 **<-** | 1.59us | 6.59us | 27.20us | 133.92us | 855.32us | 0.40x | 0.48 |
| 900_w4 | 1.89us | 11.58us | 44.58us | 177.11us | 856.15us | 0.40x | 0.47 |
| 900_wmax | 1.58us | 7.14us | 27.58us | 132.82us | 871.35us | 0.41x | 0.48 |
| 500_wmax | 1.94us | 8.51us | 34.10us | 436.49us | 1.913ms | 0.90x | 1.07 |
| 500_w64 | 1.90us | 8.46us | 34.33us | 433.90us | 1.925ms | 0.91x | 1.06 |
| 500_w4 | 1.93us | 8.61us | 35.50us | 430.32us | 1.931ms | 0.91x | 1.08 |
| 0_wmax | 2.11us | 9.44us | 37.60us | 499.21us | 2.109ms | 0.99x | 1.19 |
| 0_w4 | 2.14us | 9.09us | 37.94us | 475.62us | 2.121ms | 1.00x (base) | 1.20 |
| 0_w64 | 2.11us | 8.82us | 37.21us | 476.14us | 2.126ms | 1.00x | 1.19 |

_baseline: `0_w4`; fastest at N=16384: `900_w64`; IPC = instructions/cycles from the PMU pass_


## carrier_layout_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| 16 **<-** | 2.12us | 9.34us | 56.63us | 282.91us | 1.141ms | 0.99x | 0.40 |
| 32 | 2.05us | 9.45us | 55.71us | 287.00us | 1.143ms | 1.00x | 0.40 |
| 20 | 2.12us | 9.26us | 56.18us | 284.48us | 1.146ms | 1.00x | 0.40 |
| 24 | 2.09us | 9.32us | 56.45us | 283.36us | 1.147ms | 1.00x | 0.40 |
| 12 | 2.01us | 9.39us | 56.67us | 283.32us | 1.147ms | 1.00x (base) | 0.40 |

_baseline: `12`; fastest at N=16384: `16`; IPC = instructions/cycles from the PMU pass_


## carrier_layout_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| 20 **<-** | 2.66us | 11.36us | 48.73us | 191.87us | 770.20us | 1.00x | 0.25 |
| 12 | 2.67us | 11.93us | 47.85us | 191.54us | 770.31us | 1.00x (base) | 0.25 |
| 24 | 2.63us | 11.48us | 48.16us | 192.12us | 770.50us | 1.00x | 0.25 |
| 16 | 2.60us | 11.89us | 48.54us | 191.34us | 770.78us | 1.00x | 0.25 |
| 32 | 2.61us | 11.99us | 48.44us | 194.60us | 772.33us | 1.00x | 0.25 |

_baseline: `12`; fastest at N=16384: `20`; IPC = instructions/cycles from the PMU pass_


## carrier_layout_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| 12 **<-** | 2.50us | 11.27us | 42.05us | 576.24us | 2.622ms | 1.00x (base) | 0.89 |
| 20 | 2.47us | 11.17us | 41.60us | 584.27us | 2.624ms | 1.00x | 0.90 |
| 32 | 2.46us | 11.17us | 41.92us | 567.41us | 2.625ms | 1.00x | 0.90 |
| 24 | 2.47us | 11.21us | 41.75us | 559.13us | 2.627ms | 1.00x | 0.90 |
| 16 | 2.45us | 11.21us | 41.76us | 572.21us | 2.631ms | 1.00x | 0.90 |

_baseline: `12`; fastest at N=16384: `12`; IPC = instructions/cycles from the PMU pass_


## carrier_layout_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| 24 **<-** | 2.44us | 10.68us | 41.98us | 576.66us | 2.614ms | 1.00x | 0.87 |
| 12 | 2.48us | 10.71us | 41.24us | 578.86us | 2.615ms | 1.00x (base) | 0.86 |
| 20 | 2.41us | 10.72us | 41.44us | 576.60us | 2.621ms | 1.00x | 0.87 |
| 16 | 2.41us | 10.66us | 41.73us | 574.65us | 2.622ms | 1.00x | 0.87 |
| 32 | 2.43us | 10.68us | 42.08us | 576.62us | 2.622ms | 1.00x | 0.87 |

_baseline: `12`; fastest at N=16384: `24`; IPC = instructions/cycles from the PMU pass_


## carrier_layout_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| 16 **<-** | 2.18us | 10.37us | 42.42us | 188.82us | 840.72us | 1.00x | 0.28 |
| 12 | 2.19us | 10.46us | 43.44us | 188.17us | 841.06us | 1.00x (base) | 0.28 |
| 32 | 2.21us | 10.54us | 43.18us | 189.82us | 841.54us | 1.00x | 0.28 |
| 20 | 2.19us | 10.51us | 42.58us | 187.81us | 845.81us | 1.01x | 0.28 |
| 24 | 2.21us | 10.52us | 43.35us | 189.65us | 847.13us | 1.01x | 0.28 |

_baseline: `12`; fastest at N=16384: `16`; IPC = instructions/cycles from the PMU pass_


## carrier_layout_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| 12 **<-** | 2.43us | 10.05us | 41.57us | 420.15us | 2.158ms | 1.00x (base) | 0.64 |
| 20 | 2.30us | 10.06us | 40.17us | 439.57us | 2.164ms | 1.00x | 0.70 |
| 24 | 2.31us | 9.81us | 40.45us | 441.59us | 2.165ms | 1.00x | 0.70 |
| 16 | 2.24us | 10.01us | 40.20us | 452.74us | 2.166ms | 1.00x | 0.70 |
| 32 | 2.32us | 10.02us | 40.66us | 444.10us | 2.167ms | 1.00x | 0.70 |

_baseline: `12`; fastest at N=16384: `12`; IPC = instructions/cycles from the PMU pass_


## carrier_native_ceiling

| variant | N=64 | N=256 | N=1024 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|
| native **<-** | 109.20us | 1.821ms | 28.883ms | 0.67x | 0.21 |
| interp | 176.31us | 2.597ms | 43.092ms | 1.00x (base) | 0.23 |

_baseline: `interp`; fastest at N=1024: `native`; IPC = instructions/cycles from the PMU pass_


## carrier_native_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| copypatch **<-** | 520.4ns | 3.40us | 13.42us | 55.58us | 296.02us | 1.00x (base) | 0.39 |
| direct | 529.0ns | 3.52us | 14.32us | 54.80us | 297.43us | 1.00x | 0.42 |
| interp | 2.12us | 9.62us | 61.39us | 285.94us | 1.150ms | 3.88x | 0.40 |

_baseline: `copypatch`; fastest at N=16384: `copypatch`; IPC = instructions/cycles from the PMU pass_


## carrier_native_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| direct **<-** | 990.6ns | 6.87us | 33.18us | 134.74us | 643.92us | 1.00x | 0.61 |
| copypatch | 995.0ns | 7.27us | 33.16us | 133.17us | 644.21us | 1.00x (base) | 0.54 |
| interp | 2.41us | 11.44us | 47.85us | 193.47us | 780.78us | 1.21x | 0.25 |

_baseline: `copypatch`; fastest at N=16384: `direct`; IPC = instructions/cycles from the PMU pass_


## carrier_native_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| copypatch **<-** | 626.2ns | 3.70us | 14.84us | 58.90us | 408.72us | 1.00x (base) | 0.43 |
| direct | 587.2ns | 3.82us | 14.80us | 58.46us | 409.44us | 1.00x | 0.47 |
| interp | 2.19us | 10.74us | 43.27us | 559.83us | 2.647ms | 6.48x | 0.90 |

_baseline: `copypatch`; fastest at N=16384: `copypatch`; IPC = instructions/cycles from the PMU pass_


## carrier_native_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| copypatch **<-** | 562.3ns | 3.49us | 14.69us | 58.06us | 389.86us | 1.00x (base) | 0.45 |
| direct | 558.5ns | 3.79us | 15.02us | 57.75us | 392.71us | 1.01x | 0.49 |
| interp | 2.23us | 10.61us | 43.44us | 569.56us | 2.627ms | 6.74x | 0.87 |

_baseline: `copypatch`; fastest at N=16384: `copypatch`; IPC = instructions/cycles from the PMU pass_


## carrier_native_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| direct **<-** | 845.2ns | 4.90us | 21.95us | 86.66us | 440.99us | 0.99x | 0.45 |
| copypatch | 803.4ns | 4.99us | 21.89us | 86.78us | 443.87us | 1.00x (base) | 0.42 |
| interp | 2.07us | 10.25us | 45.11us | 187.30us | 844.44us | 1.90x | 0.27 |

_baseline: `copypatch`; fastest at N=16384: `direct`; IPC = instructions/cycles from the PMU pass_


## carrier_native_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| direct **<-** | 647.1ns | 3.94us | 15.83us | 61.20us | 456.36us | 0.99x | 0.46 |
| copypatch | 665.2ns | 3.80us | 16.05us | 61.47us | 459.72us | 1.00x (base) | 0.42 |
| interp | 2.20us | 10.36us | 42.64us | 459.29us | 2.178ms | 4.74x | 0.70 |

_baseline: `copypatch`; fastest at N=16384: `direct`; IPC = instructions/cycles from the PMU pass_


## carrier_optimize_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| all **<-** | 1.12us | 3.51us | 11.00us | 40.52us | 150.74us | 1.00x (base) | 0.24 |
| canon | 1.28us | 3.91us | 13.60us | 91.05us | 302.46us | 2.01x | 0.41 |
| cse | 1.29us | 3.87us | 13.65us | 93.75us | 344.98us | 2.29x | 0.43 |
| fold | 2.11us | 8.62us | 31.98us | 124.60us | 493.53us | 3.27x | 0.18 |
| none | 1.97us | 8.62us | 44.72us | 220.49us | 1.032ms | 6.84x | 0.36 |
| dce | 1.86us | 8.61us | 43.88us | 220.74us | 1.036ms | 6.87x | 0.36 |

_baseline: `all`; fastest at N=16384: `all`; IPC = instructions/cycles from the PMU pass_


## carrier_optimize_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| all **<-** | 413.5ns | 609.1ns | 1.56us | 7.36us | 26.16us | 1.00x (base) | 0.23 |
| fold | 1.79us | 6.82us | 25.18us | 101.48us | 402.98us | 15.40x | 0.16 |
| canon | 1.94us | 8.91us | 37.30us | 153.08us | 604.88us | 23.12x | 0.25 |
| cse | 1.91us | 9.20us | 38.16us | 155.75us | 608.12us | 23.24x | 0.24 |
| dce | 2.24us | 9.27us | 38.26us | 157.34us | 633.09us | 24.20x | 0.23 |
| none | 2.17us | 9.41us | 39.22us | 166.12us | 656.11us | 25.08x | 0.24 |

_baseline: `all`; fastest at N=16384: `all`; IPC = instructions/cycles from the PMU pass_


## carrier_optimize_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| all **<-** | 1.74us | 8.04us | 30.88us | 443.48us | 2.141ms | 1.00x (base) | 0.89 |
| canon | 1.98us | 8.29us | 30.41us | 447.99us | 2.156ms | 1.01x | 0.89 |
| cse | 1.89us | 8.26us | 30.40us | 446.64us | 2.157ms | 1.01x | 0.89 |
| none | 2.14us | 8.52us | 34.34us | 507.84us | 2.497ms | 1.17x | 0.94 |
| fold | 2.01us | 8.40us | 33.24us | 512.36us | 2.501ms | 1.17x | 0.93 |
| dce | 2.11us | 8.24us | 33.69us | 497.67us | 2.502ms | 1.17x | 0.94 |

_baseline: `all`; fastest at N=16384: `all`; IPC = instructions/cycles from the PMU pass_


## carrier_optimize_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| cse **<-** | 2.11us | 8.59us | 33.67us | 499.49us | 2.327ms | 1.00x | 0.87 |
| all | 2.14us | 8.65us | 33.91us | 493.09us | 2.330ms | 1.00x (base) | 0.87 |
| canon | 2.22us | 8.75us | 33.24us | 500.83us | 2.338ms | 1.00x | 0.87 |
| dce | 2.27us | 9.07us | 35.27us | 546.47us | 2.517ms | 1.08x | 0.89 |
| fold | 2.15us | 8.73us | 35.68us | 533.57us | 2.519ms | 1.08x | 0.89 |
| none | 2.12us | 8.88us | 36.03us | 541.43us | 2.524ms | 1.08x | 0.89 |

_baseline: `all`; fastest at N=16384: `cse`; IPC = instructions/cycles from the PMU pass_


## carrier_optimize_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| all **<-** | 437.2ns | 612.5ns | 2.09us | 8.21us | 30.99us | 1.00x (base) | 0.23 |
| fold | 1.88us | 7.03us | 25.88us | 101.11us | 407.10us | 13.13x | 0.16 |
| canon | 1.70us | 7.95us | 32.94us | 144.66us | 646.61us | 20.86x | 0.25 |
| cse | 1.85us | 8.28us | 34.34us | 149.01us | 665.04us | 21.46x | 0.25 |
| dce | 1.88us | 8.50us | 35.19us | 152.62us | 699.11us | 22.56x | 0.25 |
| none | 1.76us | 8.45us | 34.85us | 152.06us | 703.84us | 22.71x | 0.26 |

_baseline: `all`; fastest at N=16384: `all`; IPC = instructions/cycles from the PMU pass_


## carrier_optimize_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| all **<-** | 1.83us | 7.54us | 26.83us | 280.04us | 1.559ms | 1.00x (base) | 0.64 |
| canon | 2.07us | 7.59us | 27.86us | 272.42us | 1.573ms | 1.01x | 0.64 |
| cse | 1.99us | 7.58us | 27.18us | 303.47us | 1.575ms | 1.01x | 0.65 |
| none | 2.09us | 7.98us | 31.61us | 408.25us | 2.004ms | 1.28x | 0.72 |
| dce | 2.11us | 8.14us | 32.13us | 400.29us | 2.015ms | 1.29x | 0.73 |
| fold | 1.96us | 7.90us | 31.44us | 422.37us | 2.017ms | 1.29x | 0.73 |

_baseline: `all`; fastest at N=16384: `all`; IPC = instructions/cycles from the PMU pass_


## carrier_predecode_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| null **<-** | 1.11us | 5.08us | 19.96us | 80.77us | 547.75us | 0.68x | 0.35 |
| direct | 975.6ns | 4.95us | 20.03us | 78.51us | 648.17us | 0.81x | 0.54 |
| switch | 1.35us | 6.41us | 25.66us | 146.46us | 800.27us | 1.00x (base) | 0.50 |
| threaded | 1.37us | 6.43us | 25.73us | 108.54us | 824.36us | 1.03x | 0.49 |
| regcache | 1.40us | 6.73us | 27.26us | 153.16us | 835.94us | 1.04x | 0.43 |
| fntable | 1.84us | 9.22us | 55.54us | 249.19us | 1.010ms | 1.26x | 0.51 |

_baseline: `switch`; fastest at N=16384: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_predecode_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| null **<-** | 2.08us | 10.68us | 45.07us | 171.85us | 694.11us | 0.91x | 0.36 |
| switch | 2.37us | 11.86us | 49.81us | 190.74us | 765.80us | 1.00x (base) | 0.43 |
| threaded | 2.22us | 11.82us | 49.16us | 188.00us | 767.70us | 1.00x | 0.42 |
| direct | 2.18us | 11.95us | 49.73us | 189.88us | 772.26us | 1.01x | 0.56 |
| fntable | 2.46us | 11.63us | 48.48us | 191.50us | 774.48us | 1.01x | 0.37 |
| regcache | 2.07us | 9.73us | 37.75us | 145.59us | 1.058ms | 1.38x | 0.43 |

_baseline: `switch`; fastest at N=16384: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_predecode_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| null **<-** | 1.39us | 6.59us | 25.84us | 102.06us | 508.36us | 0.24x | 0.29 |
| direct | 1.61us | 7.90us | 32.01us | 387.01us | 1.734ms | 0.80x | 1.24 |
| switch | 2.12us | 9.47us | 37.40us | 503.27us | 2.154ms | 1.00x (base) | 1.20 |
| threaded | 1.87us | 9.24us | 36.65us | 489.92us | 2.248ms | 1.04x | 1.20 |
| regcache | 2.46us | 10.94us | 42.61us | 502.78us | 2.255ms | 1.05x | 0.92 |
| fntable | 3.04us | 13.64us | 53.64us | 573.94us | 2.449ms | 1.14x | 1.14 |

_baseline: `switch`; fastest at N=16384: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_predecode_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| null **<-** | 1.35us | 5.98us | 25.25us | 100.15us | 493.33us | 0.23x | 0.26 |
| direct | 1.46us | 7.43us | 30.98us | 357.13us | 1.732ms | 0.82x | 1.21 |
| switch | 2.03us | 8.68us | 37.24us | 484.99us | 2.117ms | 1.00x (base) | 1.18 |
| regcache | 2.32us | 9.90us | 40.30us | 471.06us | 2.227ms | 1.05x | 0.90 |
| threaded | 1.70us | 8.71us | 34.78us | 471.23us | 2.237ms | 1.06x | 1.18 |
| fntable | 2.99us | 12.67us | 52.90us | 544.62us | 2.411ms | 1.14x | 1.12 |

_baseline: `switch`; fastest at N=16384: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_predecode_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| null **<-** | 1.54us | 7.69us | 32.44us | 131.08us | 523.95us | 0.80x | 0.28 |
| direct | 1.92us | 9.50us | 37.43us | 148.57us | 596.86us | 0.91x | 0.44 |
| threaded | 1.99us | 8.89us | 36.76us | 144.47us | 600.33us | 0.92x | 0.33 |
| switch | 2.18us | 8.56us | 39.72us | 140.96us | 652.67us | 1.00x (base) | 0.37 |
| fntable | 2.07us | 9.24us | 38.95us | 178.16us | 805.66us | 1.23x | 0.39 |
| regcache | 2.14us | 8.55us | 33.81us | 140.95us | 873.58us | 1.34x | 0.35 |

_baseline: `switch`; fastest at N=16384: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_predecode_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| null **<-** | 1.31us | 5.94us | 23.68us | 97.90us | 480.14us | 0.28x | 0.28 |
| direct | 1.42us | 6.41us | 26.05us | 235.83us | 1.382ms | 0.80x | 0.91 |
| switch | 2.07us | 8.69us | 34.63us | 389.41us | 1.733ms | 1.00x (base) | 0.98 |
| threaded | 1.74us | 7.76us | 31.39us | 320.98us | 1.744ms | 1.01x | 0.88 |
| regcache | 2.24us | 9.77us | 37.83us | 376.09us | 1.825ms | 1.05x | 0.73 |
| fntable | 2.83us | 11.31us | 45.46us | 460.30us | 1.943ms | 1.12x | 0.87 |

_baseline: `switch`; fastest at N=16384: `null`; IPC = instructions/cycles from the PMU pass_


## carrier_residual_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| register **<-** | 1.21us | 5.36us | 21.94us | 132.34us | 759.05us | 1.00x (base) | 0.48 |
| stack | 3.25us | 13.21us | 66.31us | 263.21us | 1.165ms | 1.53x | 0.35 |

_baseline: `register`; fastest at N=16384: `register`; IPC = instructions/cycles from the PMU pass_


## carrier_residual_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| register **<-** | 1.57us | 9.68us | 41.24us | 151.30us | 624.16us | 1.00x (base) | 0.45 |
| stack | 5.70us | 23.42us | 95.05us | 363.61us | 1.431ms | 2.29x | 0.24 |

_baseline: `register`; fastest at N=16384: `register`; IPC = instructions/cycles from the PMU pass_


## carrier_residual_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| register **<-** | 1.83us | 7.04us | 27.39us | 442.13us | 1.991ms | 1.00x (base) | 1.41 |
| stack | 5.71us | 23.42us | 127.10us | 603.45us | 2.802ms | 1.41x | 0.51 |

_baseline: `register`; fastest at N=16384: `register`; IPC = instructions/cycles from the PMU pass_


## carrier_residual_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| register **<-** | 1.91us | 7.26us | 31.72us | 456.41us | 2.006ms | 1.00x (base) | 1.31 |
| stack | 5.92us | 24.30us | 112.00us | 623.96us | 2.844ms | 1.42x | 0.53 |

_baseline: `register`; fastest at N=16384: `register`; IPC = instructions/cycles from the PMU pass_


## carrier_residual_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| register **<-** | 1.75us | 7.25us | 32.63us | 111.18us | 503.70us | 1.00x (base) | 0.36 |
| stack | 5.94us | 23.29us | 98.76us | 378.95us | 1.543ms | 3.06x | 0.26 |

_baseline: `register`; fastest at N=16384: `register`; IPC = instructions/cycles from the PMU pass_


## carrier_residual_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| register **<-** | 1.82us | 6.54us | 26.43us | 342.84us | 1.569ms | 1.00x (base) | 1.13 |
| stack | 6.04us | 24.11us | 100.77us | 581.46us | 2.557ms | 1.63x | 0.43 |

_baseline: `register`; fastest at N=16384: `register`; IPC = instructions/cycles from the PMU pass_


## carrier_setup_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| predecode **<-** | 3.26us | 10.63us | 42.99us | 187.89us | 755.52us | 0.31x | 0.24 |
| emitdirect | 2.81us | 10.09us | 44.55us | 204.01us | 1.284ms | 0.52x | 0.26 |
| stackcompile | 7.85us | 19.17us | 60.61us | 294.01us | 1.642ms | 0.67x | 0.34 |
| emitcopypatch | 10.01us | 36.68us | 139.22us | 580.57us | 2.465ms | 1.00x (base) | 0.23 |
| optall | 108.04us | 253.85us | 742.47us | 2.914ms | 10.373ms | 4.21x | 0.26 |
| parse | 0.0ns | 0.0ns | 0.0ns | 0.0ns | 0.0ns | - | 0.24 |

_baseline: `emitcopypatch`; fastest at N=16384: `predecode`; IPC = instructions/cycles from the PMU pass_


## carrier_setup_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| predecode **<-** | 3.71us | 10.78us | 36.90us | 141.15us | 547.85us | 0.14x | 0.13 |
| stackcompile | 8.59us | 22.86us | 75.37us | 343.75us | 1.291ms | 0.34x | 0.19 |
| emitdirect | 4.56us | 16.97us | 64.70us | 247.33us | 1.563ms | 0.41x | 0.19 |
| emitcopypatch | 14.61us | 57.44us | 216.86us | 853.63us | 3.787ms | 1.00x (base) | 0.22 |
| optall | 129.14us | 368.26us | 983.82us | 3.823ms | 12.010ms | 3.17x | 0.19 |
| parse | 0.0ns | 0.0ns | 0.0ns | 0.0ns | 0.0ns | - | 0.23 |

_baseline: `emitcopypatch`; fastest at N=16384: `predecode`; IPC = instructions/cycles from the PMU pass_


## carrier_setup_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| predecode **<-** | 3.49us | 10.02us | 35.73us | 148.72us | 836.51us | 0.16x | 0.21 |
| stackcompile | 9.52us | 24.12us | 79.42us | 337.03us | 1.998ms | 0.39x | 0.31 |
| emitdirect | 4.17us | 16.71us | 68.06us | 500.64us | 2.854ms | 0.55x | 0.36 |
| emitcopypatch | 15.02us | 66.13us | 283.15us | 1.186ms | 5.153ms | 1.00x (base) | 0.31 |
| optall | 155.20us | 596.29us | 2.461ms | 9.932ms | 40.501ms | 7.86x | 0.23 |
| parse | 0.0ns | 0.0ns | 0.0ns | 0.0ns | 0.0ns | - | 0.23 |

_baseline: `emitcopypatch`; fastest at N=16384: `predecode`; IPC = instructions/cycles from the PMU pass_


## carrier_setup_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| predecode **<-** | 3.35us | 9.64us | 35.31us | 142.93us | 726.30us | 0.12x | 0.18 |
| stackcompile | 8.69us | 24.03us | 81.94us | 332.90us | 1.934ms | 0.32x | 0.29 |
| emitdirect | 4.02us | 15.93us | 67.62us | 514.39us | 3.527ms | 0.59x | 0.47 |
| emitcopypatch | 13.67us | 66.23us | 292.21us | 1.209ms | 5.967ms | 1.00x (base) | 0.36 |
| optall | 167.05us | 643.77us | 2.557ms | 12.318ms | 50.281ms | 8.43x | 0.24 |
| parse | 0.0ns | 0.0ns | 0.0ns | 0.0ns | 0.0ns | - | 0.23 |

_baseline: `emitcopypatch`; fastest at N=16384: `predecode`; IPC = instructions/cycles from the PMU pass_


## carrier_setup_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| predecode **<-** | 3.50us | 10.16us | 36.12us | 143.30us | 553.65us | 0.14x | 0.14 |
| stackcompile | 7.85us | 23.31us | 76.80us | 331.44us | 1.224ms | 0.32x | 0.18 |
| emitdirect | 4.31us | 15.82us | 64.56us | 269.09us | 1.616ms | 0.42x | 0.19 |
| emitcopypatch | 13.74us | 56.38us | 220.42us | 882.29us | 3.883ms | 1.00x (base) | 0.23 |
| optall | 108.06us | 224.74us | 823.21us | 3.229ms | 12.067ms | 3.11x | 0.20 |
| parse | 0.0ns | 0.0ns | 0.0ns | 0.0ns | 0.0ns | - | 0.24 |

_baseline: `emitcopypatch`; fastest at N=16384: `predecode`; IPC = instructions/cycles from the PMU pass_


## carrier_setup_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| predecode **<-** | 3.52us | 10.21us | 37.32us | 169.21us | 1.265ms | 0.23x | 0.30 |
| stackcompile | 9.03us | 24.05us | 81.51us | 396.63us | 2.622ms | 0.47x | 0.38 |
| emitdirect | 3.88us | 15.69us | 69.08us | 381.93us | 2.631ms | 0.48x | 0.30 |
| emitcopypatch | 18.53us | 80.48us | 315.82us | 1.275ms | 5.524ms | 1.00x (base) | 0.27 |
| optall | 156.95us | 651.00us | 2.480ms | 10.756ms | 44.147ms | 7.99x | 0.25 |
| parse | 0.0ns | 0.0ns | 0.0ns | 0.0ns | 0.0ns | - | 0.23 |

_baseline: `emitcopypatch`; fastest at N=16384: `predecode`; IPC = instructions/cycles from the PMU pass_


## carrier_valrepr

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| nanbox **<-** | 2.08us | 8.06us | 31.69us | 143.05us | 1.432ms | 0.80x | 0.45 |
| tagged | 2.26us | 8.53us | 34.21us | 145.12us | 1.653ms | 0.93x | 0.49 |
| static | 2.14us | 8.46us | 33.50us | 154.35us | 1.779ms | 1.00x (base) | 0.78 |

_baseline: `static`; fastest at N=16384: `nanbox`; IPC = instructions/cycles from the PMU pass_


## carrier_vertical_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| vert8 **<-** | 6.41us | 26.29us | 107.03us | 490.06us | 2.209ms | 0.34x | 0.67 |
| vert4 | 6.87us | 29.13us | 119.77us | 680.82us | 2.950ms | 0.45x | 0.60 |
| scalar | 10.01us | 46.31us | 195.04us | 1.115ms | 6.578ms | 1.00x (base) | 0.52 |

_baseline: `scalar`; fastest at N=16384: `vert8`; IPC = instructions/cycles from the PMU pass_


## carrier_vertical_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| vert8 **<-** | 7.91us | 35.57us | 140.58us | 579.10us | 2.381ms | 0.38x | 0.59 |
| vert4 | 9.54us | 50.52us | 210.41us | 846.18us | 3.450ms | 0.55x | 0.62 |
| scalar | 17.57us | 87.05us | 378.44us | 1.526ms | 6.222ms | 1.00x (base) | 0.46 |

_baseline: `scalar`; fastest at N=16384: `vert8`; IPC = instructions/cycles from the PMU pass_


## carrier_vertical_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| vert8 **<-** | 6.68us | 28.23us | 117.91us | 839.62us | 3.586ms | 0.21x | 1.00 |
| vert4 | 7.23us | 33.00us | 264.62us | 1.353ms | 5.566ms | 0.32x | 1.08 |
| scalar | 15.59us | 67.68us | 294.84us | 3.886ms | 17.161ms | 1.00x (base) | 1.26 |

_baseline: `scalar`; fastest at N=16384: `vert8`; IPC = instructions/cycles from the PMU pass_


## carrier_vertical_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| vert8 **<-** | 6.75us | 28.24us | 115.78us | 825.61us | 3.556ms | 0.21x | 0.99 |
| vert4 | 7.52us | 32.95us | 273.24us | 1.341ms | 5.575ms | 0.33x | 1.08 |
| scalar | 16.50us | 67.23us | 294.15us | 3.851ms | 17.063ms | 1.00x (base) | 1.25 |

_baseline: `scalar`; fastest at N=16384: `vert8`; IPC = instructions/cycles from the PMU pass_


## carrier_vertical_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| vert8 **<-** | 7.31us | 30.68us | 122.73us | 520.81us | 2.229ms | 0.43x | 0.62 |
| vert4 | 8.16us | 38.18us | 162.65us | 648.60us | 2.777ms | 0.54x | 0.54 |
| scalar | 16.31us | 64.06us | 319.77us | 1.201ms | 5.129ms | 1.00x (base) | 0.38 |

_baseline: `scalar`; fastest at N=16384: `vert8`; IPC = instructions/cycles from the PMU pass_


## carrier_vertical_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) | IPC (N max) |
|---|---|---|---|---|---|---|---|
| vert8 **<-** | 7.30us | 29.69us | 118.93us | 751.70us | 3.232ms | 0.23x | 0.83 |
| vert4 | 7.62us | 31.67us | 180.51us | 1.156ms | 4.732ms | 0.34x | 0.86 |
| scalar | 15.79us | 65.28us | 278.84us | 3.022ms | 13.847ms | 1.00x (base) | 1.01 |

_baseline: `scalar`; fastest at N=16384: `vert8`; IPC = instructions/cycles from the PMU pass_
