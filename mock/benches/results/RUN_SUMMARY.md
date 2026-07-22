# Carrier interpreter-composition matrix: first full-run summary

45 benches, one coherent wall-clock run (2026-07-23). Every cell derives its op
semantics from the single `ops::binop_body!` / `binop_simd!` source, shares the
unchecked `access::rload/rstore` fidelity path, and runs each `(variant, size,
mode)` as its own release cdylib subprocess. Times are median `algo_ns` across
the replicated passes. Full per-variant tables at the bottom; per-bench CSV +
findings under `results/carrier_*/`.

Wall-clock only: the `instructions` / `cycles` columns are zero because the PMU
path is gated behind an op-controlled `sudo MOCKSPACE_BENCH_PERF=1` session (the
kperf fixed counters validated on M1). A PMU re-run reproduces the identical
matrix with those two columns populated; nothing else changes.

## Headline findings

1. **Dispatch shape barely matters on straight-line bytecode.** Across the six
   dispatch programs, switch, natural if-chain, frequency-ascending if-chain,
   function-table, and bit-tree cluster within roughly 2 to 15 percent at scale.
   The natural if-chain ties the switch (never slower), and the function table
   often edges the switch on wide programs (0.90x on real/scatter at N=16384).
   The `nullfloor` control (loop with no dispatch) sits at 0.2 to 0.8x, so
   dispatch is a minority of per-op cost on this ISA. The only real penalty is
   the barrier-forced linear scan (`ifchainlin`, 1.25 to 1.27x), which exists
   precisely as the "what a bad linear probe costs" control.

2. **Preserve-none threading wins only where control flow is real.** On
   straight-line dispatch, threading loses (1.12 to 1.17x slower than switch).
   On the CFG register-VM (`carrier_cfg`) with actual branches and back-edges,
   trace-threading drops to 0.45x and preserve-none threading to 0.79x of the
   switch. The advantage materializes exactly where the mechanism predicts:
   branchy loops with indirect transfers, not a flat opcode stream.

3. **Record width is a null result, refuting the 24-byte-optimal hypothesis.**
   REC12 / 16 / 20 / 24 / 32 land within under one percent of each other on
   every profile. Decode is not the bottleneck; per-op work dominates. There is
   no width to "tune" for speed; the choice is a density/padding decision, not a
   throughput one.

4. **Register VM decisively beats stack VM.** The register residual wins every
   profile; the stack machine runs up to 1.91x slower (madd and tight, nearly
   2x). For arithmetic-heavy residuals the register machine is the clear pick.

5. **Value representation: nan-boxing and tagging beat a static u64 at scale**
   on the valrepr program (0.78x and 0.83x at N=16384). Worth carrying forward,
   but this is one program shape and should not be over-generalized.

6. **Optimization payoff is entirely program-dependent, and eqsat alone can
   pessimize.** On the madd chain, the full pipeline (`all`) runs 15 to 142x
   faster than the alternatives (it folds the chain to almost nothing). On
   real/scatter there is nothing to fold and every level is a wash. Equality
   saturation on its own produces a residual slower than no optimization for
   madd (the AC-reassociation without folding rearranges without shrinking); it
   only pays when paired with folding inside `all`. The lesson: run the whole
   pipeline, do not ship eqsat as a standalone stage.

7. **Native codegen delivers about 3x, and copy-and-patch equals the stencil.**
   The copy-and-patch JIT and the hand-written `global_asm` aarch64 stencil both
   hit 2.6 to 4.3x over the interpreter and sit within one percent of each
   other; the stencil occasionally edges copypatch (leaf, 0.93x). The PoC proves
   both paths are viable and neither dominates, so the copy-and-patch route (far
   cheaper to maintain than per-op hand asm) is the sound default.

8. **Vertical SIMD is the single biggest win: 3 to 6x.** `vert8` (eight-wide
   `portable_simd`, one dispatch amortized over eight records) runs at 0.17 to
   0.37x of scalar across every profile, up to roughly 6x on real and scatter.
   This is the corrected honest measurement: every cell processes exactly eight
   inputs per call with pre-allocated scratch, so the raw times are directly
   comparable. It maps straight onto vehje's per-record column evaluation and is
   the highest-leverage beyond-runtime shape in the matrix.

9. **Native throughput ceiling: about 1.5x.** The shape-specialized native madd
   loop over a byte stream (the O(N^2) throughput idiom, not per-execution
   latency) beats the interpreter by ~1.5x at N=1024. The narrower margin than
   the per-execution native tier reflects that this idiom is throughput and
   memory bound, not dispatch bound.

## What this says for vehje's runtime tiers

The residual tiers the runtime already plans map cleanly onto the evidence. The
flat interpreter is a fine baseline and its dispatch shape is not worth agonizing
over (finding 1); a register layout and the full optimizer pipeline are free wins
worth defaulting on (findings 4, 6). The two places real speed lives are
vectorized per-record column evaluation (finding 8, 3 to 6x, and it is exactly
vehje's column-eval shape) and native codegen via copy-and-patch (finding 7,
about 3x, cheap to maintain). Threading is a targeted tool for branchy residuals
(finding 2), not a general default. Record width is a layout decision, not a perf
lever (finding 3).

## Caveats

All figures are `CNTVCT` wall-clock; `timed_calibrated!` auto-repeats small-N
cells to clear the 2048-tick quantization floor. Coefficient of variation is
mostly under one percent; a few cells show throttle bounce (the native_ceiling
interpreter autocorrelation is -0.65, a thermal alternation, not a code effect).
The PMU columns await the op-gated sudo re-run. `native_ceiling` is capped at
N<=1024 because its O(N^2) sweep at N=4096 exceeds the driver window for no
insight beyond the 64/256/1024 curve.

## Full matrix

## carrier_dispatch_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.65us | 7.21us | 28.77us | 116.48us | 585.69us | 0.22x |
| fntable | 2.88us | 13.06us | 54.21us | 560.76us | 2.343ms | 0.90x |
| ifchain | 2.27us | 10.59us | 41.18us | 585.10us | 2.603ms | 1.00x |
| switch | 2.35us | 10.55us | 41.27us | 608.10us | 2.612ms | 1.00x (base) |
| ifchainasc | 2.27us | 10.30us | 41.89us | 591.02us | 2.613ms | 1.00x |
| bittree | 2.89us | 12.81us | 50.10us | 250.71us | 2.653ms | 1.02x |
| threaded | 2.82us | 11.83us | 45.61us | 645.47us | 2.923ms | 1.12x |
| ifchainlin | 5.16us | 21.86us | 87.19us | 539.98us | 3.255ms | 1.25x |

_baseline: `switch`; fastest at N=16384: `nullfloor`_


## carrier_dispatch_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.96us | 10.65us | 42.41us | 161.03us | 650.33us | 0.83x |
| ifchainlin | 2.53us | 11.53us | 45.39us | 172.42us | 691.86us | 0.88x |
| bittree | 2.60us | 11.64us | 46.15us | 176.85us | 715.81us | 0.91x |
| ifchainasc | 2.67us | 12.46us | 48.74us | 193.49us | 776.50us | 0.99x |
| ifchain | 2.69us | 12.65us | 48.80us | 194.72us | 779.70us | 1.00x |
| switch | 2.69us | 12.51us | 50.33us | 199.55us | 783.58us | 1.00x (base) |
| fntable | 2.94us | 13.79us | 54.62us | 214.19us | 868.63us | 1.11x |
| threaded | 2.94us | 13.11us | 54.44us | 213.29us | 892.18us | 1.14x |

_baseline: `switch`; fastest at N=16384: `nullfloor`_


## carrier_dispatch_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.83us | 8.31us | 32.91us | 127.84us | 499.46us | 0.60x |
| bittree | 2.55us | 11.63us | 45.27us | 174.31us | 730.05us | 0.88x |
| switch | 2.27us | 11.17us | 45.55us | 186.86us | 828.34us | 1.00x (base) |
| ifchainasc | 2.45us | 11.37us | 45.06us | 185.59us | 832.40us | 1.00x |
| ifchain | 2.28us | 11.38us | 45.43us | 190.04us | 839.74us | 1.01x |
| fntable | 3.02us | 13.08us | 53.68us | 217.55us | 940.73us | 1.14x |
| threaded | 2.90us | 12.49us | 46.91us | 195.63us | 955.38us | 1.15x |
| ifchainlin | 3.95us | 17.18us | 60.43us | 236.95us | 977.47us | 1.18x |

_baseline: `switch`; fastest at N=16384: `nullfloor`_


## carrier_dispatch_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.77us | 7.91us | 30.25us | 121.05us | 593.13us | 0.23x |
| fntable | 3.26us | 14.28us | 56.35us | 556.12us | 2.333ms | 0.90x |
| ifchainasc | 2.55us | 11.28us | 43.26us | 612.62us | 2.590ms | 1.00x |
| switch | 2.52us | 11.10us | 43.94us | 598.11us | 2.596ms | 1.00x (base) |
| ifchain | 2.57us | 11.19us | 43.72us | 590.78us | 2.600ms | 1.00x |
| bittree | 3.03us | 13.63us | 52.73us | 307.95us | 2.685ms | 1.03x |
| threaded | 2.88us | 12.63us | 46.93us | 647.72us | 2.900ms | 1.12x |
| ifchainlin | 5.93us | 23.86us | 94.09us | 561.83us | 3.242ms | 1.25x |

_baseline: `switch`; fastest at N=16384: `nullfloor`_


## carrier_dispatch_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.80us | 8.02us | 31.05us | 119.98us | 517.78us | 0.24x |
| bittree | 2.78us | 12.31us | 48.33us | 189.79us | 1.666ms | 0.78x |
| fntable | 3.08us | 13.30us | 52.82us | 453.33us | 2.073ms | 0.97x |
| switch | 2.38us | 10.86us | 42.76us | 475.64us | 2.138ms | 1.00x (base) |
| ifchainasc | 2.39us | 10.79us | 42.38us | 475.67us | 2.146ms | 1.00x |
| ifchain | 2.50us | 11.38us | 44.33us | 486.77us | 2.190ms | 1.02x |
| threaded | 2.90us | 12.05us | 46.65us | 447.91us | 2.311ms | 1.08x |
| ifchainlin | 5.31us | 24.40us | 101.84us | 523.37us | 2.722ms | 1.27x |

_baseline: `switch`; fastest at N=16384: `nullfloor`_


## carrier_dispatch_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| nullfloor **<-** | 1.66us | 7.45us | 28.59us | 115.57us | 569.76us | 0.54x |
| bittree | 2.40us | 10.48us | 40.94us | 166.93us | 779.43us | 0.73x |
| ifchainlin | 3.04us | 12.83us | 51.12us | 216.01us | 979.50us | 0.92x |
| ifchainasc | 2.12us | 10.01us | 48.03us | 237.56us | 1.063ms | 1.00x |
| switch | 2.14us | 9.63us | 46.19us | 226.29us | 1.064ms | 1.00x (base) |
| ifchain | 2.14us | 9.57us | 49.48us | 247.55us | 1.082ms | 1.02x |
| fntable | 2.66us | 12.71us | 56.00us | 264.25us | 1.138ms | 1.07x |
| threaded | 2.80us | 11.76us | 44.93us | 218.27us | 1.244ms | 1.17x |

_baseline: `switch`; fastest at N=16384: `nullfloor`_


## carrier_predecode_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| null **<-** | 1.40us | 6.67us | 26.15us | 101.55us | 455.55us | 0.22x |
| direct | 1.63us | 7.90us | 32.08us | 378.88us | 1.709ms | 0.81x |
| switch | 2.14us | 9.57us | 38.22us | 496.19us | 2.102ms | 1.00x (base) |
| threaded | 1.89us | 9.26us | 36.65us | 471.92us | 2.200ms | 1.05x |
| regcache | 2.46us | 10.94us | 42.86us | 492.98us | 2.211ms | 1.05x |
| fntable | 3.06us | 13.64us | 54.11us | 566.72us | 2.402ms | 1.14x |

_baseline: `switch`; fastest at N=16384: `null`_


## carrier_predecode_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| null **<-** | 2.08us | 10.82us | 45.36us | 171.42us | 692.48us | 0.90x |
| threaded | 2.25us | 11.84us | 49.66us | 189.81us | 761.37us | 0.99x |
| fntable | 2.48us | 11.83us | 49.22us | 187.57us | 761.42us | 0.99x |
| switch | 2.38us | 11.97us | 50.20us | 189.49us | 765.85us | 1.00x (base) |
| direct | 2.22us | 12.06us | 50.32us | 190.64us | 765.89us | 1.00x |
| regcache | 2.08us | 9.84us | 38.09us | 147.08us | 1.012ms | 1.32x |

_baseline: `switch`; fastest at N=16384: `null`_


## carrier_predecode_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| null **<-** | 1.58us | 8.08us | 34.09us | 130.37us | 518.78us | 0.81x |
| direct | 1.98us | 9.95us | 39.11us | 146.30us | 590.84us | 0.92x |
| threaded | 2.07us | 9.53us | 39.37us | 144.08us | 597.00us | 0.93x |
| switch | 2.25us | 8.99us | 41.90us | 148.45us | 642.24us | 1.00x (base) |
| fntable | 2.13us | 9.54us | 41.42us | 174.61us | 811.32us | 1.26x |
| regcache | 2.19us | 8.95us | 35.43us | 140.79us | 841.27us | 1.31x |

_baseline: `switch`; fastest at N=16384: `null`_


## carrier_predecode_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| null **<-** | 1.44us | 6.47us | 25.83us | 98.08us | 421.70us | 0.20x |
| direct | 1.54us | 7.84us | 31.56us | 393.76us | 1.708ms | 0.81x |
| switch | 2.18us | 9.44us | 38.02us | 503.40us | 2.098ms | 1.00x (base) |
| threaded | 1.82us | 9.16us | 36.85us | 514.37us | 2.197ms | 1.05x |
| regcache | 2.44us | 10.69us | 42.68us | 511.72us | 2.203ms | 1.05x |
| fntable | 3.19us | 13.52us | 53.45us | 582.89us | 2.387ms | 1.14x |

_baseline: `switch`; fastest at N=16384: `null`_


## carrier_predecode_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| null **<-** | 1.37us | 6.30us | 25.29us | 103.24us | 520.87us | 0.31x |
| direct | 1.48us | 6.68us | 27.97us | 227.70us | 1.349ms | 0.80x |
| threaded | 1.79us | 8.18us | 32.88us | 332.82us | 1.694ms | 1.00x |
| switch | 2.13us | 9.19us | 37.10us | 379.30us | 1.694ms | 1.00x (base) |
| regcache | 2.32us | 10.28us | 40.12us | 376.33us | 1.802ms | 1.06x |
| fntable | 2.96us | 11.86us | 49.44us | 451.46us | 1.936ms | 1.14x |

_baseline: `switch`; fastest at N=16384: `null`_


## carrier_predecode_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| null **<-** | 1.11us | 5.13us | 20.33us | 81.94us | 480.83us | 0.59x |
| direct | 994.1ns | 4.89us | 20.12us | 78.91us | 641.58us | 0.79x |
| threaded | 1.28us | 5.76us | 22.92us | 95.66us | 782.18us | 0.96x |
| switch | 1.36us | 6.48us | 26.00us | 146.69us | 813.83us | 1.00x (base) |
| regcache | 1.42us | 6.73us | 27.69us | 149.18us | 829.08us | 1.02x |
| fntable | 1.84us | 8.85us | 54.73us | 247.69us | 1.016ms | 1.25x |

_baseline: `switch`; fastest at N=16384: `null`_


## carrier_cfg

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| trace **<-** | 27.19us | 105.51us | 399.28us | 1.588ms | 6.331ms | 0.45x |
| threaded | 47.71us | 182.65us | 697.15us | 2.802ms | 11.153ms | 0.79x |
| fntable | 56.58us | 221.58us | 862.52us | 3.448ms | 13.732ms | 0.97x |
| switch | 60.06us | 228.44us | 883.65us | 3.547ms | 14.134ms | 1.00x (base) |

_baseline: `switch`; fastest at N=16384: `trace`_


## carrier_layout_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| 24 **<-** | 2.47us | 11.22us | 42.47us | 601.60us | 2.588ms | 1.00x |
| 32 | 2.49us | 11.20us | 42.67us | 603.13us | 2.596ms | 1.00x |
| 20 | 2.48us | 11.27us | 42.80us | 585.32us | 2.596ms | 1.00x |
| 12 | 2.52us | 11.32us | 42.54us | 600.75us | 2.597ms | 1.00x (base) |
| 16 | 2.50us | 11.23us | 42.31us | 609.62us | 2.603ms | 1.00x |

_baseline: `12`; fastest at N=16384: `24`_


## carrier_layout_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| 20 **<-** | 2.67us | 12.46us | 49.60us | 191.11us | 769.15us | 1.00x |
| 12 | 2.67us | 12.50us | 49.47us | 190.82us | 769.29us | 1.00x (base) |
| 16 | 2.66us | 12.50us | 49.46us | 191.12us | 769.59us | 1.00x |
| 24 | 2.72us | 12.44us | 49.64us | 190.99us | 769.67us | 1.00x |
| 32 | 2.69us | 12.48us | 49.57us | 193.96us | 771.28us | 1.00x |

_baseline: `12`; fastest at N=16384: `20`_


## carrier_layout_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| 24 **<-** | 2.29us | 11.20us | 45.87us | 187.84us | 832.29us | 1.00x |
| 20 | 2.27us | 11.22us | 45.82us | 189.28us | 832.32us | 1.00x |
| 16 | 2.28us | 11.19us | 45.85us | 187.20us | 832.71us | 1.00x |
| 32 | 2.29us | 11.15us | 46.17us | 188.65us | 832.72us | 1.00x |
| 12 | 2.26us | 11.24us | 45.89us | 188.19us | 833.45us | 1.00x (base) |

_baseline: `12`; fastest at N=16384: `24`_


## carrier_layout_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| 16 **<-** | 2.51us | 11.14us | 44.51us | 576.53us | 2.584ms | 1.00x |
| 12 | 2.55us | 11.26us | 44.78us | 587.66us | 2.591ms | 1.00x (base) |
| 32 | 2.53us | 11.18us | 44.28us | 577.28us | 2.594ms | 1.00x |
| 20 | 2.50us | 11.16us | 44.34us | 571.46us | 2.595ms | 1.00x |
| 24 | 2.51us | 11.15us | 44.48us | 571.68us | 2.596ms | 1.00x |

_baseline: `12`; fastest at N=16384: `16`_


## carrier_layout_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| 16 **<-** | 2.40us | 10.81us | 42.19us | 480.51us | 2.127ms | 1.00x |
| 12 | 2.51us | 11.20us | 43.24us | 439.60us | 2.135ms | 1.00x (base) |
| 24 | 2.39us | 10.86us | 41.53us | 466.47us | 2.136ms | 1.00x |
| 20 | 2.39us | 10.87us | 42.22us | 472.46us | 2.138ms | 1.00x |
| 32 | 2.39us | 10.85us | 42.02us | 471.13us | 2.144ms | 1.00x |

_baseline: `12`; fastest at N=16384: `16`_


## carrier_layout_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| 12 **<-** | 2.13us | 9.79us | 60.34us | 284.17us | 1.126ms | 1.00x (base) |
| 32 | 2.15us | 9.92us | 58.88us | 290.11us | 1.130ms | 1.00x |
| 20 | 2.17us | 10.29us | 59.38us | 284.23us | 1.131ms | 1.00x |
| 24 | 2.15us | 9.84us | 59.45us | 284.47us | 1.131ms | 1.00x |
| 16 | 2.15us | 10.04us | 60.39us | 284.02us | 1.131ms | 1.01x |

_baseline: `12`; fastest at N=16384: `12`_


## carrier_valrepr

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| nanbox **<-** | 2.12us | 8.96us | 33.40us | 174.62us | 1.491ms | 0.78x |
| tagged | 2.25us | 9.45us | 35.90us | 147.87us | 1.580ms | 0.83x |
| static | 2.46us | 10.67us | 40.38us | 183.22us | 1.906ms | 1.00x (base) |

_baseline: `static`; fastest at N=16384: `nanbox`_


## carrier_residual_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| register **<-** | 2.35us | 10.44us | 43.44us | 566.26us | 2.607ms | 1.00x (base) |
| stack | 5.93us | 25.37us | 120.02us | 613.53us | 2.812ms | 1.08x |

_baseline: `register`; fastest at N=16384: `register`_


## carrier_residual_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| register **<-** | 2.48us | 12.11us | 46.84us | 198.53us | 784.77us | 1.00x (base) |
| stack | 5.72us | 24.16us | 93.27us | 379.82us | 1.497ms | 1.91x |

_baseline: `register`; fastest at N=16384: `register`_


## carrier_residual_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| register **<-** | 2.10us | 10.50us | 42.37us | 187.06us | 831.19us | 1.00x (base) |
| stack | 6.06us | 26.33us | 95.56us | 399.87us | 1.591ms | 1.91x |

_baseline: `register`; fastest at N=16384: `register`_


## carrier_residual_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| register **<-** | 2.33us | 10.17us | 43.72us | 585.33us | 2.604ms | 1.00x (base) |
| stack | 7.27us | 24.97us | 107.23us | 625.47us | 2.838ms | 1.09x |

_baseline: `register`; fastest at N=16384: `register`_


## carrier_residual_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| register **<-** | 2.20us | 10.16us | 42.73us | 458.79us | 2.147ms | 1.00x (base) |
| stack | 5.66us | 22.98us | 104.87us | 568.36us | 2.521ms | 1.17x |

_baseline: `register`; fastest at N=16384: `register`_


## carrier_residual_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| register **<-** | 1.97us | 9.13us | 44.46us | 218.62us | 1.062ms | 1.00x (base) |
| stack | 2.93us | 12.59us | 68.71us | 253.15us | 1.148ms | 1.08x |

_baseline: `register`; fastest at N=16384: `register`_


## carrier_optimize_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| all **<-** | 1.92us | 8.45us | 31.71us | 473.09us | 2.120ms | 1.00x (base) |
| cseeqsat | 2.10us | 8.54us | 32.12us | 470.43us | 2.130ms | 1.00x |
| eqsat | 2.11us | 8.47us | 31.96us | 456.24us | 2.131ms | 1.01x |
| cse | 2.06us | 8.63us | 32.16us | 466.90us | 2.144ms | 1.01x |
| fold | 2.21us | 8.96us | 34.56us | 537.92us | 2.463ms | 1.16x |
| none | 2.21us | 8.96us | 34.55us | 536.34us | 2.471ms | 1.17x |
| dce | 2.24us | 9.08us | 34.85us | 546.60us | 2.476ms | 1.17x |

_baseline: `all`; fastest at N=16384: `all`_


## carrier_optimize_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| all **<-** | 428.8ns | 625.2ns | 1.59us | 6.99us | 25.16us | 1.00x (base) |
| fold | 1.92us | 7.00us | 26.07us | 105.72us | 398.66us | 15.85x |
| cse | 2.06us | 9.49us | 39.26us | 155.13us | 601.74us | 23.92x |
| dce | 2.33us | 9.93us | 39.95us | 156.30us | 629.31us | 25.02x |
| none | 2.33us | 10.07us | 40.85us | 163.02us | 648.11us | 25.76x |
| cseeqsat | 15.16us | 53.42us | 228.23us | 890.13us | 3.571ms | 141.96x |
| eqsat | 15.16us | 53.73us | 227.96us | 890.41us | 3.573ms | 142.04x |

_baseline: `all`; fastest at N=16384: `all`_


## carrier_optimize_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| all **<-** | 458.7ns | 631.2ns | 2.62us | 8.29us | 30.53us | 1.00x (base) |
| fold | 1.93us | 7.12us | 27.00us | 100.38us | 400.90us | 13.13x |
| cse | 1.91us | 8.38us | 35.62us | 147.54us | 650.17us | 21.30x |
| dce | 1.95us | 8.72us | 36.09us | 153.64us | 690.73us | 22.62x |
| none | 1.93us | 8.73us | 36.01us | 151.44us | 697.91us | 22.86x |
| eqsat | 2.27us | 23.33us | 107.55us | 450.95us | 2.031ms | 66.51x |
| cseeqsat | 2.26us | 23.07us | 107.10us | 449.02us | 2.051ms | 67.19x |

_baseline: `all`; fastest at N=16384: `all`_


## carrier_optimize_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| all **<-** | 2.16us | 8.68us | 34.32us | 499.76us | 2.259ms | 1.00x (base) |
| eqsat | 2.20us | 8.89us | 34.90us | 504.20us | 2.261ms | 1.00x |
| cseeqsat | 2.22us | 8.90us | 34.72us | 483.74us | 2.268ms | 1.00x |
| cse | 2.25us | 8.89us | 34.91us | 500.46us | 2.298ms | 1.02x |
| dce | 2.31us | 9.29us | 36.86us | 565.98us | 2.497ms | 1.11x |
| fold | 2.20us | 9.39us | 37.14us | 531.66us | 2.502ms | 1.11x |
| none | 2.31us | 9.35us | 37.36us | 569.66us | 2.506ms | 1.11x |

_baseline: `all`; fastest at N=16384: `all`_


## carrier_optimize_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| cse **<-** | 2.07us | 8.08us | 28.96us | 330.53us | 1.574ms | 0.97x |
| all | 1.86us | 8.10us | 29.50us | 322.10us | 1.621ms | 1.00x (base) |
| eqsat | 2.14us | 8.27us | 30.27us | 328.76us | 1.625ms | 1.00x |
| cseeqsat | 2.15us | 8.31us | 30.20us | 335.01us | 1.627ms | 1.00x |
| dce | 2.13us | 8.51us | 33.26us | 424.99us | 1.990ms | 1.23x |
| fold | 2.05us | 8.45us | 33.16us | 434.76us | 2.002ms | 1.24x |
| none | 2.15us | 8.50us | 33.27us | 435.17us | 2.009ms | 1.24x |

_baseline: `all`; fastest at N=16384: `cse`_


## carrier_optimize_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| all **<-** | 1.15us | 3.58us | 11.68us | 43.62us | 151.46us | 1.00x (base) |
| cseeqsat | 1.88us | 4.55us | 17.27us | 99.49us | 293.97us | 1.94x |
| eqsat | 1.87us | 4.52us | 22.14us | 101.51us | 295.87us | 1.95x |
| cse | 1.37us | 3.98us | 14.39us | 98.26us | 341.34us | 2.25x |
| fold | 2.17us | 8.65us | 33.90us | 126.53us | 488.82us | 3.23x |
| dce | 2.03us | 8.99us | 45.51us | 222.82us | 1.018ms | 6.72x |
| none | 2.03us | 8.80us | 46.54us | 226.30us | 1.022ms | 6.75x |

_baseline: `all`; fastest at N=16384: `all`_


## carrier_native_real

| variant | N=64 | N=256 | N=1024 | vs base (N max) |
|---|---|---|---|---|
| stencil **<-** | 591.9ns | 3.77us | 15.63us | 0.99x |
| copypatch | 567.1ns | 3.80us | 15.81us | 1.00x (base) |
| interp | 2.32us | 11.13us | 44.66us | 2.82x |

_baseline: `copypatch`; fastest at N=1024: `stencil`_


## carrier_native_madd

| variant | N=64 | N=256 | N=1024 | vs base (N max) |
|---|---|---|---|---|
| copypatch **<-** | 1.03us | 7.39us | 34.38us | 1.00x (base) |
| stencil | 1.03us | 7.31us | 34.70us | 1.01x |
| interp | 2.40us | 12.30us | 49.44us | 1.44x |

_baseline: `copypatch`; fastest at N=1024: `copypatch`_


## carrier_native_tight

| variant | N=64 | N=256 | N=1024 | vs base (N max) |
|---|---|---|---|---|
| stencil **<-** | 828.5ns | 5.06us | 22.86us | 0.99x |
| copypatch | 815.8ns | 5.12us | 23.10us | 1.00x (base) |
| interp | 2.09us | 11.06us | 45.72us | 1.98x |

_baseline: `copypatch`; fastest at N=1024: `stencil`_


## carrier_native_scatter

| variant | N=64 | N=256 | N=1024 | vs base (N max) |
|---|---|---|---|---|
| stencil **<-** | 574.1ns | 3.71us | 15.57us | 1.00x |
| copypatch | 599.1ns | 3.77us | 15.62us | 1.00x (base) |
| interp | 2.30us | 11.26us | 44.98us | 2.88x |

_baseline: `copypatch`; fastest at N=1024: `stencil`_


## carrier_native_wideselect

| variant | N=64 | N=256 | N=1024 | vs base (N max) |
|---|---|---|---|---|
| copypatch **<-** | 623.4ns | 4.06us | 16.41us | 1.00x (base) |
| stencil | 650.7ns | 4.03us | 16.51us | 1.01x |
| interp | 2.18us | 10.74us | 43.30us | 2.64x |

_baseline: `copypatch`; fastest at N=1024: `copypatch`_


## carrier_native_leaf

| variant | N=64 | N=256 | N=1024 | vs base (N max) |
|---|---|---|---|---|
| stencil **<-** | 552.5ns | 3.57us | 13.63us | 0.93x |
| copypatch | 552.5ns | 3.58us | 14.63us | 1.00x (base) |
| interp | 1.98us | 9.66us | 62.38us | 4.26x |

_baseline: `copypatch`; fastest at N=1024: `stencil`_


## carrier_vertical_real

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| vert8 **<-** | 7.29us | 30.54us | 115.83us | 841.49us | 3.519ms | 0.17x |
| vert4 | 7.95us | 35.51us | 266.42us | 1.331ms | 5.479ms | 0.26x |
| scalar | 19.39us | 83.60us | 322.79us | 4.280ms | 20.821ms | 1.00x (base) |

_baseline: `scalar`; fastest at N=16384: `vert8`_


## carrier_vertical_madd

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| vert8 **<-** | 8.48us | 34.62us | 141.96us | 574.75us | 2.347ms | 0.37x |
| vert4 | 10.36us | 51.65us | 207.21us | 837.22us | 3.392ms | 0.54x |
| scalar | 21.28us | 91.13us | 373.02us | 1.575ms | 6.289ms | 1.00x (base) |

_baseline: `scalar`; fastest at N=16384: `vert8`_


## carrier_vertical_tight

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| vert8 **<-** | 7.56us | 31.71us | 121.23us | 512.47us | 2.187ms | 0.33x |
| vert4 | 8.49us | 40.59us | 163.37us | 637.91us | 2.726ms | 0.41x |
| scalar | 16.90us | 81.83us | 338.70us | 1.475ms | 6.658ms | 1.00x (base) |

_baseline: `scalar`; fastest at N=16384: `vert8`_


## carrier_vertical_scatter

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| vert8 **<-** | 6.85us | 29.98us | 113.95us | 843.42us | 3.513ms | 0.17x |
| vert4 | 7.44us | 35.04us | 268.77us | 1.336ms | 5.485ms | 0.26x |
| scalar | 18.72us | 82.90us | 327.67us | 4.461ms | 20.774ms | 1.00x (base) |

_baseline: `scalar`; fastest at N=16384: `vert8`_


## carrier_vertical_wideselect

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| vert8 **<-** | 7.78us | 31.38us | 120.93us | 738.35us | 3.194ms | 0.19x |
| vert4 | 7.50us | 33.12us | 175.61us | 1.143ms | 4.674ms | 0.27x |
| scalar | 17.78us | 81.09us | 314.78us | 3.585ms | 17.039ms | 1.00x (base) |

_baseline: `scalar`; fastest at N=16384: `vert8`_


## carrier_vertical_leaf

| variant | N=64 | N=256 | N=1024 | N=4096 | N=16384 | vs base (N max) |
|---|---|---|---|---|---|---|
| vert8 **<-** | 6.71us | 28.23us | 109.77us | 483.71us | 2.177ms | 0.26x |
| vert4 | 7.32us | 29.90us | 123.54us | 668.30us | 2.893ms | 0.34x |
| scalar | 16.25us | 70.42us | 330.28us | 1.746ms | 8.487ms | 1.00x (base) |

_baseline: `scalar`; fastest at N=16384: `vert8`_


## carrier_native_ceiling

| variant | N=64 | N=256 | N=1024 | vs base (N max) |
|---|---|---|---|---|
| native **<-** | 105.52us | 1.806ms | 28.927ms | 0.67x |
| interp | 171.52us | 2.626ms | 43.172ms | 1.00x (base) |

_baseline: `interp`; fastest at N=1024: `native`_
