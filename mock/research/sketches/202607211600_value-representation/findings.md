# Interpreter value representation: NaN-box vs tagged (bench-driven fork resolution)

**Date:** 2026-07-21
**Type:** Zig-native microbench, ReleaseFast, 50M value-ops over a 64-register file (90% int / 10% float, the
typical script mix), 7-run best with per-run register reset. Zig 0.16.0, aarch64. Artifact: `valuerepr.zig`.
**Settles:** the interpreter's runtime value representation. The cfg-interp bench used untyped u64 registers,
but a dynamic script language tags each value at runtime (int / float / bool / string-ref / nil). This benches
the representation fork before the design commits, per the workspace's bench-driven-fork-resolution discipline.

## The fork

A dynamic value must carry its type tag. Three classic shapes, two of which are the real contenders:

- **A, NaN-box (8 bytes):** everything lives in a 64-bit float. Real doubles are stored as themselves; other
  types hide in the quiet-NaN payload, discriminated by high mantissa bits. Floats need no unpack; non-floats
  cost a mask and a branch.
- **C, tagged 8 bytes:** 3 low bits are the tag, the payload is the high 61. Ints cost a shift to extract;
  floats do not fit (only 61 payload bits and the tag collides with float bit patterns), so a float-heavy
  language must box floats out-of-line or truncate them.
- **B, tagged union (16 bytes):** `{tag, payload}`, no bit games. Omitted from the timing: it is strictly 2x the
  memory of either 8-byte scheme, so it loses on cache-line density and register pressure before any per-op cost
  comparison. For a hot interpreter register file, doubling the value width is the dominant cost; the 8-byte
  schemes are the fork that matters.

## Results

| representation | ns/op | throughput |
|---|---|---|
| A, NaN-box 8B | **0.96** | 1040 M-op/s |
| C, tagged 8B | 1.17 | 857 M-op/s |

(The printed checksums are anti-DCE keep-alive sinks, not correctness oracles: the 10% float path does raw bit
reinterpretation rather than real value semantics, so its accumulator is meaningless. The 90%-int path, which
dominates the timing, does a real tagged add in both variants and is the clean, apples-to-apples comparison.)

## The finding: NaN-box wins on speed and on floats; recommend it for vehje's float-carrying consumers

NaN-boxing is ~18% faster per value-op (0.96 vs 1.17 ns) and handles floats better (natively, no box). The speed
edge comes from the int hot path: NaN-box extracts an int with a single mask-compare plus a truncate, while
tagged-8B pays a shift on every int extract and a shift on every int construct. And crucially, NaN-box stores
real doubles with zero unpack cost, whereas tagged-8B cannot hold a full f64 in its 61 payload bits, forcing
float-heavy code to box or lose precision.

For vehje this is decisive in one direction: the biggest consumers (doc DSLs, config languages, game scripts)
all carry floats as first-class values (positions, weights, ratios, percentages). Tagged-8B's float penalty is
therefore a real cost for exactly the workloads that matter, on top of its slower int path. **Recommend NaN-boxing
for the interpreter's runtime value representation**, with these documented constraints:

- Pointers / string-refs must fit in the 51-bit NaN payload. Fine on every current 64-bit platform (48-bit
  virtual addresses); a note for any future platform with wider canonical addresses.
- Ints are limited to the payload width chosen (32-bit here; 51-bit is available in the NaN payload if the
  language wants wider small-ints before promoting to boxed bignums).
- The scheme is well-trodden (LuaJIT, SpiderMonkey, JavaScriptCore all NaN-box), so it is a proven contract, not
  a novel risk.

## Design impact

- Interpreter register file and value-arena scalar slots use the 8-byte NaN-box representation. This composes
  with the 24-byte node record (BN2): the record's inline operands that carry immediate values encode them in
  the same NaN-box form, so there is one value representation across the arena and the register file.
- Value-op cost (~1 ns) sits just under the dispatch cost (~1.7 ns/instr from cfg-interp), so a typical
  interpreted instruction is dispatch + one or two value-ops = ~3 to 4 ns. Value representation is a real but not
  dominant contributor; it does not change the "interpreter floor is fast enough that native is a ceiling"
  conclusion.
- The 16-byte tagged union is rejected: 2x memory on the hottest data structure (the register file and scalar
  arena slots) is the wrong trade when an 8-byte scheme carries the same information.

## Boundary

Microbench of the value-op cost in isolation (a tight add loop), not a full interpreter, so it isolates the
representation cost cleanly but does not capture interactions with real dispatch and memory traffic (measured
separately in cfg-interp). The float path is modelled thinly (10% of ops, and not with real value semantics in
this microbench); a genuinely float-dominated workload would widen NaN-box's lead further, since that is exactly
where tagged-8B's boxing penalty bites.

## Artifacts
- `valuerepr.zig` (NaN-box and tagged-8B value-op loops over a 64-register file with the 90/10 int/float mix).
