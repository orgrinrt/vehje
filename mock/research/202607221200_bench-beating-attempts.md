# Bench-beating attempts: break the conclusions, force new shapes

**Date:** 2026-07-22 (living document, accretes as attempts land)
**Status:** intermediate results, not a design record. This is the working companion to the
eventual design topic. It holds the current baselines as bars to beat and logs every attempt to
beat them with a new variant, however novel or untested. A win here is a win or a thing to
consider; a loss is still a recorded data point that hardens the baseline.

The re-measure synthesis (`202607220300_bench-remeasure-synthesis.md`) established honest baselines
after the carrier rebuild. That doc answers "what is true now." This doc answers a different
question: "can we beat it." The two are complementary. Nothing here supersedes the synthesis until
an attempt actually wins and is cross-validated; a winning attempt then feeds the design topic as a
new candidate, not a settled conclusion.

The rule for every attempt: same carrier, same program bytes across FFI-opaque boundaries,
byte-exact cross-validation against the reference, `>=3` runs, a cost-model sanity line, and the
mechanism visible in code. An attempt that wins by measuring something easier does not count. When
a "winner" surprises, scrutinise it before believing it (the synthesis caught three cross-validated
but wrongly-measured results this way).

## Representing the "unrepresentable"

The synthesis parked one comparison as blocked: the guaranteed-tail-call, no-callee-saved
("preserve_none" / Deegen) dispatch shape, which the dispatch generator's own comment calls "not
expressible in Rust." That was wrong, and it is the template for this whole effort: when a shape is
declared unrepresentable, find the representation. The options, cheapest first:

1. A gated-but-real language feature (an experimental calling convention, an intrinsic).
2. Guaranteed tail calls (`become`, `explicit_tail_calls`).
3. Hand-written assembly with a custom pinned-register convention (`naked_asm!` / `global_asm!` in
   Rust, a linked `.s` or naked dispatcher in Zig). This is the ground truth; it is what LuaJIT
   does, and one `.s` serves both the Rust cdylib and the Zig runtime.
4. If neither a feature nor asm can model it, ask what genuinely can, and use that.

On the pinned nightly (`nightly-2026-05-28`, rustc 1.98) the answer landed at rung 1+2, not 3:
`extern "rust-preserve-none"` is a real ABI (feature `rust_preserve_none_cc`, issue #151401), and
`become` (feature `explicit_tail_calls`, #112788) gives the guaranteed tail call. Together they
express a real context-threaded interpreter in pure Rust, verified: an indirect-threaded `become`
through a static function-pointer table compiles and runs correct. The hand-asm rung stays the
portable fallback for the Zig side and for any shape the cc cannot express.

## Baselines to beat (the current winners)

| bench | current winner / number | the bar to beat |
|---|---|---|
| native ceiling | switch interp ~2.0x native, fntable ~2.2x | get interpretation below 2x native |
| dispatch shape | switch beats fn-table; tail (Zig std ABI) 1.5-4x slower | a preserve_none/threaded shape that beats switch |
| record width | 12-32B all tie ~10ns/node (L1/L2); 16B viable | a layout (sub-16B, SoA operands) that beats the tie |
| match lowering | if-chain beats jump-table on M1 | a dispatch (perfect-hash, bit-test tree) that beats if-chain |
| semi-naive reach | 2-5x deep/narrow, ~1.2x wide/many-target | a hybrid or bit-parallel frontier beating both regimes |
| retraction | counted 100-5000x local, recompute wins catastrophic | a scheme that wins both ends of the crossover |
| eqsat bound | bound(512) extracts optimal at 17-17000x speed | a smaller bound or faster hashcons at equal quality |
| incremental cache | warm 2.5-3.7x cold | structural sharing / faster hash pushing the ratio up |
| threaded compile | 2.3x peak (4 threads), degrades at 8 | P-core pinning / better granularity beating 2.3x |
| interner | sharded 1.4-3.6x (dedup-gated) | a single-thread SIMD/swiss interner beating sharded |
| value repr | static/tagged/nanbox small, regime-dependent | a 4th repr (inline-small-int, pointer-tag) that wins a regime |
| cfg-interp | 3.0 ns/step (9.6 cyc) | superinstructions / block threading below 3 ns/step |
| thermometer | 24-69x over branch-max | a wider-vector or bit-trick encoding beating it |

arena-locality (flat past L2) and value-arena (latency-bound) have no assumption left to break on the
throughput side, so they are not beat-targets; they stay as measured.

## Attempt log

Each entry: the assumption attacked, the new variant, the result (WIN / MATCH / LOSS / OPEN), the
cross-validation status, and what it means. Append-only; corrections annotate rather than delete.

### A1. Preserve_none context-threaded dispatch (attacks: "tail-threading is not expressible in Rust")

- Assumption attacked: the dispatch bench's own claim that the guaranteed-tail-call threaded shape
  Deegen relies on cannot be written in Rust, only in a Zig cdylib.
- Toolchain finding (already a win regardless of the perf result): `extern "rust-preserve-none"` +
  `become` express it in pure Rust on the pinned nightly. The "not expressible" comment is retracted.
- Variant: a carrier `interpret_threaded` with one preserve_none handler per opcode, each computing
  its node result with the identical operand decode as the switch, folding the identical rolling
  hash, then `become`-dispatching the next node's handler. Same `Decoded` bytes, cross-validated
  byte-exact against `interpret`.
- Result: WIN on expressibility, MATCH-at-small / LOSS-at-large on perf, plus a bigger baseline break
  (below). Cross-validated byte-exact (`interp_threaded::tests`, 400 and 20k nodes at six seeds; the
  20k case proves the guaranteed tail call runs in constant stack).
- The numbers (median per size, 1.00x = fastest at that size; carrier dispatch bench, M1, rev 70fb75b5):

  | n | vocab | switch | fntable | threaded | fastest |
  |---|---|---|---|---|---|
  | 64 | 17 | 1.02x | 1.09x | 1.00x | threaded |
  | 256 | 17 | 1.04x | 1.17x | 1.00x | threaded |
  | 1024 | 17 | 1.02x | 1.32x | 1.00x | threaded |
  | 4096 | 17 | 1.17x | 1.00x | 1.33x | fntable |
  | 16384 | 17 | 1.19x | 1.00x | 1.31x | fntable |
  | 64 | 4 | 1.00x | 1.16x | 1.04x | switch |
  | 256 | 4 | 1.02x | 1.09x | 1.00x | threaded |
  | 1024 | 4 | 1.00x | 1.25x | 1.07x | switch |
  | 4096 | 4 | 1.20x | 1.00x | 1.09x | fntable |
  | 16384 | 4 | 1.16x | 1.00x | 1.28x | fntable |

- Reading: a clean working-set crossover. In the L1-resident regime (n<=1024, results+program fit
  L1) switch and threaded trade the lead and fntable is worst. Past L1 (n>=4096, ~128KB results +
  ~384KB program spill to L2) the order inverts: **fntable wins, switch and threaded fall behind,
  threaded worst at the extreme.** The interpreter is memory-latency-bound there, and dispatch shape
  stops being the story.
- Two baselines broken. (1) "The threaded shape is not expressible in Rust" is retracted: it is, via
  `rust-preserve-none` + `become`, cross-validated. (2) **"switch is the faster dispatch shape"** (the
  synthesis's reframed headline #3) is now false at scale: fntable beats switch 1.16-1.20x at
  n>=4096 on both vocabs. The old dispatch bench reported one aggregate size and missed the crossover.
- Why threaded loses at scale (mechanism, cost-model, not counters): context threading puts an
  indirect branch at each of the 17 handler tails, so the branch predictor sees 17 distinct
  indirect-branch sites (worse BTB behaviour than fntable's single returning call site or switch's
  single jump-table site), and preserve-none re-materialises all interpreter state across each
  `become`. Both costs are hidden while everything is L1-hot and dominate once memory latency does
  not cover them. This is the M1 read; the Ertl-Gregg per-site-correlation advantage of threaded code
  does not pay off here.
- Next on this bench: try to reclaim the large-n regime for threaded with (a) pointer-threading (thread
  a raw record pointer advanced by stride, drop the per-node index arithmetic) and (b) superinstructions
  (fuse hot op pairs, cutting both dispatch count and records fetched). Superinstructions also attack the
  native-ceiling bench, so they get their own attempt.
