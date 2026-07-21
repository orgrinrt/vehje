# Dispatch shape (carrier): switch vs function-pointer table

Covers `carrier_dispatch_v4` and `carrier_dispatch_v17` (op vocabularies 4 and
17). The audit-relevant motivation (Wingo): the old bench's "switch beats
tail-threading, tail rejected" was a toolchain verdict measured over a tiny
random three-op stream with no `preserve_none` (which Zig 0.16 does not have),
and the switch's advantage was expected to narrow as the op vocabulary grows.
This carrier bench measures the honest, cross-validated comparison the harness
can express in Rust (a flat single-jump-table switch versus an indirect-threaded
function-pointer table) and sweeps vocabulary. Both variants produce identical
output (validated across 100 seeds per size).

## Result: dispatch is regime-dependent, not "switch always wins"

Medians, function-pointer table relative to the switch baseline:

| n | v4 switch | v4 fntable | v17 switch | v17 fntable |
|---|---|---|---|---|
| 64 | 2.29us (base) | 2.81us (1.23x) | 2.23us (base) | 2.74us (1.23x) |
| 256 | 9.92us | 11.34us (1.14x) | 10.54us | 12.14us (1.15x) |
| 1024 | 34.6us | 45.5us (1.32x) | 36.4us | 47.8us (1.31x) |
| 4096 | 481us (1.17x) | 412us (base) | 560us (1.22x) | 460us (base) |
| 16384 | 2.18ms (1.14x) | 1.91ms (base) | 2.56ms (1.19x) | 2.15ms (base) |

The switch wins on small, L1-resident programs (up to about 1024 nodes) by 14 to
32 percent; the function-pointer table wins on large programs (4096 nodes and
up) by 14 to 22 percent. The crossover sits around a few thousand nodes. The op
vocabulary (4 versus 17) barely moves it, which contradicts the naive
expectation that a bigger vocabulary flips the ranking; both vocabularies show
the same size-driven crossover.

## What it says for the design, and what it does not

The original "switch is the dispatch, tail-threading is a measured-and-rejected
alternative" over-hardens a more nuanced truth. For the small, hot programs the
templating and config consumers run (a few hundred nodes), the switch is the
right default. For large programs, the indirect-threaded shape is faster here.
Neither is universally best, so the design should carry a size-conditioned
dispatch choice, not a single global winner, and it must not cite "switch
refutes Deegen": the guaranteed-tail-call shape Deegen actually relies on is not
expressible in Rust and was never in this comparison.

## Caveats before these rankings harden

Two things. First, the rankings are codegen-sensitive at a level that warrants
disassembly verification: rewriting the switch from a nested match (binary ops
behind a second dispatch) to a flat single-level match already flipped the small
sizes from fntable-wins to switch-wins, and the flat switch is nominally slower
than the nested one at the largest size, which means the backend's jump-table
and i-cache behaviour is doing something worth reading the assembly for before
banking the exact multipliers. The regime-dependence is robust; the precise
numbers are provisional pending disasm. Second, the load-bearing dispatch
question, the guaranteed-tail-call ("threaded") shape with `preserve_none`, is a
Zig cdylib follow-up that consumes identical program bytes (one of the three
places the shipped language is load-bearing). Until that lands, the Deegen claim
stays open, not rejected.

## Cost-model sanity line

At n=16384, v17 switch is 2.56ms for 16384 nodes times 16 passes, 9.8 ns/node,
about 29 cycles for a byte-decode plus a jump-table dispatch plus one or two
arithmetic ops plus a store plus a two-op hash; plausible for a memory-touching
tree-walk at that working-set size (a 24-byte node array of 16384 nodes is
393KB, past L1 into L2). Times scale close to linearly with node count, so the
timed work is the interpretation.
