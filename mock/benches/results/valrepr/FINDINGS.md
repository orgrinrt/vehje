# Value representation: static/raw vs runtime-tagged vs NaN-boxed

**Strength: measurement** (wall-clock via CNTVCT_EL0, 6 passes; all three representations cross-validated to
byte-identical output). The fourth carrier variant axis the audit named (dispatch shape, record layout, block
structure, value representation), previously unbenched.

## What it measures, and what it deliberately does not

The same mixed int/float program is interpreted three ways: values raw `u64` with the type known statically
per node (static), a `(tag, bits)` pair read at run time (tagged), or NaN-boxed into a `u64` (nanbox). The
question maps to vehje: the graded type system can prove a value's type at compile time, so how much does
carrying a runtime representation cost the interpreter?

Scope, stated up front: each op site here is MONOMORPHIC (an IADD always takes int operands; the generator
types them so), so the tag/box branches are predictable. This is the interpreter-tier cost of CARRYING a
representation (tag storage, box/unbox work), not the megamorphic misprediction cost that hurts dynamic typing
at a polymorphic site. That misprediction cost is a compiled/JIT-tier concern (specializing a site to its
observed types); a pure interpreter branches per node either way, so it cannot show the static-specialization
win. This bench is therefore correctly a representation-cost measurement, not a static-vs-dynamic-dispatch one.

## Result, ratio to static (< 1 = faster than static), and ns/op

| n | static ns/op | tagged | nanbox |
|---|---|---|---|
| 64    | 2.04 | 1.07x | 1.02x |
| 256   | 2.28 | 1.06x | 1.03x |
| 1024  | 2.20 | 1.05x | 1.00x |
| 4096  | 2.48 | 0.91x | 0.85x |
| 16384 | 6.79 | 0.91x | 0.83x |

## The finding

The representation cost is small and regime-dependent, and it inverts at scale:

- **Compute-bound (n <= 1024, in cache):** static is marginally fastest, as expected. A runtime tag costs
  5-7% (tagged); NaN-boxing is a statistical tie (0-3%). Carrying a representation is cheap but nonzero when
  the interpreter is not waiting on memory.
- **Memory-bound (n >= 4096, working set past cache):** the overhead vanishes and inverts. NaN-boxed is 15-17%
  FASTER than static, tagged 9% faster. The transition is visible in the static ns/op (2.2 -> 6.79 as the
  value array plus the 24-byte-per-node program array exceed cache and the loop goes DRAM-latency-bound).

The likely mechanism, stated as a hypothesis because pinning it needs the PMU counters unavailable in M1
userspace: when the interpreter is memory-bound, the extra tag/box work executes in the shadow of memory-load
latency (filling stall cycles the out-of-order engine would otherwise waste), so it is not merely free but can
raise memory-level parallelism. When compute-bound there are no stall cycles to hide under, so the same work
is pure overhead. The static baseline is a fair, minimal interpreter (it does the least work: no tag read, no
box check), so this is not a weak-opponent artifact; it is a real interaction between representation work and
the memory regime.

Design implication for vehje's interpreter tier: the value-representation choice costs little. A tagged or
NaN-boxed dynamic representation is within ~7% of raw when compute-bound and actually faster at scale, so
supporting dynamically-typed values in the baseline interpreter is not the performance liability it is often
assumed to be. The static-typing performance win is real but lives in the COMPILED tier (specializing
polymorphic sites), which the graded type system enables and which this interpreter bench does not measure.

## Cost-model sanity line

static at n=16384: 6.79 ns/op is ~21.7 cycles/op at 3.2 GHz, for a per-op cost of a 24-byte program-node read
plus two operand loads plus a write, at a working set (value array 128 KB + program array 393 KB) well past
L2. ~22 cycles/op is DRAM-latency-bound, physically consistent, not an artifact. At n=64 (in cache) it is 2.04
ns/op ~ 6.5 cycles/op, compute-bound.

## Cross-validation and boundary

All three representations fold the identical checksum over the mixed int/float program at every size (the
harness confirmed identical output). Boundary: i32 integers (so NaN-boxing fits the quiet-NaN payload) and
finite f64 floats (which never collide with the box tag). The megamorphic dynamic-dispatch cost is the natural
follow-up, but it belongs on a compiled-tier bench (a specializing JIT), not the interpreter carrier.
