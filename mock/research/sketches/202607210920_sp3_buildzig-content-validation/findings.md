# SP3 findings: the native build.zig content-validation step (three-loci)

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 | artifacts `build.zig`, `validate_content.zig`
**Settles:** the three-loci split (1845/05): comptime for type generation, a NATIVE build.zig step for content
validation, native at the consumer.

## Result
A `build.zig` wires a native content-validation executable as a build step (`zig build validate`) that also
gates the default build. It validates 199,995 records (dedup via a native hashmap) at native speed during the
build. This is the content locus: because BN0 showed superlinear/large content validation hits the comptime cost
cliff, content validation lives in a native build.zig step (native hashmap, native speed) rather than at
comptime, while comptime keeps the type generation it is good at (SK1).

## Design impact
The three loci are real and buildable on the pinned toolchain: (1) comptime for type generation and small/linear
checks (SK1, BN0), (2) a native `build.zig` step for large/superlinear content validation (this sketch), (3)
native at the consumer (the shipped runtime). The build step gates the build, so invalid content fails the
build, keeping the certified-generation guarantee. The BN0 boundary (superlinear-or-large -> native step)
decides what goes where.
