# `vehje`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/vehje.svg)](https://github.com/orgrinrt/vehje/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/vehje.svg)](https://github.com/orgrinrt/vehje/issues)
![License](https://img.shields.io/github/license/orgrinrt/vehje?color=%23009689)

> Typed authoring language with a stable IR for either direct runtime execution or pluggable transpilation to source in another target language.

</div>

`vehje` is a scripting language for game modding and embeddable runtimes. The compiler runs at development time and ships as a single static binary per host platform. The source surface is typed and modular, with tagged enums and monomorphised dispatch in place of dyn-trait and runtime type checks. Compiler and pluggable backends are both written in Rust; the static guarantees carry into both, so misbehaviour surfaces at compile time rather than after the IR has shipped.

What ships is the IR. Either the runtime executes it directly, or a pluggable backend lowers the IR one more step and emits the result in whatever form the target's host already accepts: Lua source for an embedded interpreter, a Wasm module for a sandboxed runtime, a configuration tree for an engine that reads everything as data. The transpilation path is where the typed surface earns its keep: a target that is neither typed nor strict gains an authoring layer above it that is both.

The C ABI between compiler and runtime is the boundary contract. The runtime is written in Zig, ships as a small binary suitable for embedding alongside other code, and never sees source: it consumes IR the compiler has already proven valid. The compiler's phase crates run `no_std`, with no allocator and no surprise platform calls, and host-side phases get no dispensation from that discipline. Two places fall short of it today: the name resolver still holds scope state in `std` collections, tracked as a skeleton escape until the scheduler owns that state, and the CLI binary is an ordinary `std` program. Dynamic dispatch is confined to a small set of documented extension-point registries (the validator list, the codegen target list) rather than spread through the pipeline. Targets and backends slot in through that same explicit boundary; the surface is pluggable by design, and a new emission target arrives without forking `vehje`.

## The source surface

vehje has a typed, modular surface. Types are declared up front and enforced by the compiler; tagged enums plus trait dispatch take the place of dynamic dispatch and runtime type checks. Diagnostics carry phase, severity, and span, plus a static message string and related secondary spans; warnings are denied at the crate root. Developers coming from Rust will find much of the syntax and semantics familiar. The language has its own item kinds where it needs them: `event` for typed event hooks, `expect` / `actual` for cross-crate paired declarations, `sealed` for closing a trait against external implementation.

Three axes are intentionally unusual. Numerics are exact-width and fixed-point native: write `i23`, `u14`, `fixed<2, 9>`, whatever range and precision the value calls for; the compiler picks the storage container under arvo's strategy markers. Floats are discouraged in favour of fixed-point arithmetic but available through arvo when a workload genuinely calls for them; pick fast or strict explicitly, with `f24` the default fast variant and `f24!` the strict counterpart for paths where determinism or precision matters more than throughput. Macros are proc-only and run as scheduler passes inside the compiler; the AST never leaves vehje's own arena, and macro-emitted items go through the same type-checking that authored items do.

References and borrows are part of the IR's authoring contract. The runtime executes them directly; a pluggable transpilation backend decides how to express them in a target language that may not disambiguate between borrow and move at all.

```rust
// vehje source: aspirational, surface still being designed.

mod audio {
    // signed fixed-point: 2 integer bits + 9 fractional.
    // range -2 to +2, 1/512 precision; native shape for normalised gain.
    pub type Sample = fixed<2, 9>;

    pub const MAX_GAIN: Sample = 1.95;
    pub const SILENCE: Sample = 0.0;

    pub sealed trait Channel {
        fn process(&self, sample: Sample) -> Sample;
    }

    pub struct Gain { pub factor: Sample }

    impl Channel for Gain {
        fn process(&self, sample: Sample) -> Sample {
            let scaled = sample * self.factor;
            scaled.clamp(-MAX_GAIN, MAX_GAIN)
        }
    }

    pub event SampleClipped {
        pub channel: u11,
        pub raw: Sample,
        pub clamped: Sample,
    }
}

mod platform {
    // downstream platform crate supplies the matching `actual fn`.
    expect fn now_micros() -> u47;
}

// generic mixer: borrowed channel, mutable buffer processed in place.
pub fn run<C: audio::Channel>(channel: &C, buf: &mut [audio::Sample], id: u11) -> u31 {
    let mut clipped: u31 = 0;
    for sample in buf.iter_mut() {
        let raw = *sample;
        *sample = channel.process(raw);
        if *sample == audio::MAX_GAIN || *sample == -audio::MAX_GAIN {
            emit audio::SampleClipped { channel: id, raw, clamped: *sample };
            clipped += 1;
        }
    }
    clipped
}
```

## The compiler

The compiler is designed to run as a hilavitkutin pipeline. The front-end emits IR through a fixed sequence of WorkUnits: tokens (`vehje-lex`), AST (`vehje-syntax`), resolved scopes and bindings (`vehje-resolve`), typed and coherence-checked items (`vehje-typecheck`). Macro expansions, validators, and content lints are also meant to be WorkUnits, scheduled into the same DAG, with pipeline state (symbol tables, type contexts, diagnostics, the IR itself) living in scheduler-owned Resources and Columns, and the back-end emitting IR through `vehje-codegen` as one more WorkUnit at the end of the pipeline. That wiring is `vehje-schedule`'s target shape; today the crate ships only the `CompilerSchedule` marker type, and the real pass DAG is follow-up work.

Front-end crates are `#![no_std]` with no allocator; they produce IR without ever materialising runtime state. Nothing in the pipeline assumes the host platform matches the runtime's target, so cross-compilation needs no special-casing. The compiler ships as a single static binary per host platform.

```rust
// aspirational: the intended WorkUnit shape once vehje-schedule's
// real pass DAG lands. `vehje::pass::CoherenceCtx` does not exist yet.
// custom validator: a hilavitkutin WorkUnit. reads resolved items,
// writes diagnostics for coherence violations.

use hilavitkutin_api::{
    Always, Atomic, Column, Immediate, Normal, WorkUnit, read, write,
};
use vehje::ir::{ResolvedItem, Diagnostic};
use vehje::pass::CoherenceCtx;

pub struct NoUnusedSealed;

impl WorkUnit<Always> for NoUnusedSealed {
    type Read = read![Column<ResolvedItem>];
    type Write = write![Column<Diagnostic>];
    type Hint = (Immediate, Atomic, Normal);
    type Ctx = CoherenceCtx;

    fn execute(&self, ctx: &Self::Ctx) {
        ctx.each(|item: &ResolvedItem| {
            if item.is_sealed_trait() && item.implementors().is_empty() {
                ctx.emit(Diagnostic::warn(
                    item.span(),
                    "sealed trait has no implementors",
                ));
            }
        });
    }
}
```

## Pluggable backends

A backend is a Rust crate that implements `vehje-codegen`'s `CodegenTarget` trait and registers into the compiler's const `TargetRegistry`; this is a statically linked extension point, not a dynamically loaded plugin. `vehje-jomini` (Clausewitz engine bytecode, a planned sibling repo) is the first target meant to ship outside the two built-ins, `NativeTarget` and `JominiTarget`. The finalised trait carries associated `NAME` and `VERSION` consts, `accepts(kind)` for AST-node routing, `stages()` to opt into pipeline stages, and a `compile` entry point for lowering and emission.

```rust
// design: the finalised CodegenTarget shape a target crate implements.
// the shipped skeleton today registers a narrower, object-safe form
// (`name(&self)`, `emit(&self, ctx, bytes, diagnostics)`) behind the
// same TargetRegistry, with NativeTarget and JominiTarget as stubs.

pub trait CodegenTarget: Send + Sync + 'static {
    const NAME: &'static str;
    const VERSION: u32;

    fn accepts(&self, kind: AstNodeKind) -> bool;
    fn stages(&self) -> &'static [Stage];

    fn compile(
        &self,
        unit: &TypedUnit,
        ctx: &mut CodegenCtx<'_>,
    ) -> Outcome<CodegenOutput, Diagnostic>;

    fn render_diagnostic(&self, err: &TargetError) -> Diagnostic;
}
```

## The runtime

The runtime is a small Zig program that loads a vehje IR artefact and executes it. It speaks the C ABI declared in `vehje-runtime-abi`: an opaque `VehjeRuntime` handle, a `VehjeResult` scalar return code, a `VehjeDiagnostic` for error surfaces, and three core entry points (`vehje_runtime_new` to open an IR artefact, `vehje_runtime_execute` to run it, `vehje_runtime_free` to release the handle). The runtime pulls no Rust runtime dependency; the host program decides whether the runtime ships as a static library, a shared object, or a wasm module loaded into the host's existing wasm sandbox.

The execution path is interpreter-shaped initially, with bytecode dispatch over hilavitkutin WorkUnits inside the runtime itself. AOT-compiled bundles for selected targets ship later as additional output modes; the source language and the runtime ABI don't change when that happens. What survives the boundary in either case is the IR; the runtime's job is to evaluate it.

```rust
// host program embedding the vehje runtime via the C ABI.
// skeleton bodies today: vehje_runtime_new always returns null,
// vehje_runtime_execute always returns VehjeResult::Err.

use vehje_runtime_abi::{vehje_runtime_execute, vehje_runtime_free, vehje_runtime_new, VehjeResult};

fn run(input: *const u8, len: arvo::USize) -> VehjeResult {
    let rt = unsafe { vehje_runtime_new() };
    let result = unsafe { vehje_runtime_execute(rt, input, len) };
    if result != VehjeResult::Ok {
        // surface a diagnostic once vehje_runtime_diagnostic lands; details elided.
    }
    unsafe { vehje_runtime_free(rt) };
    result
}
```

## Status

vehje is in the design phase. The crate split, language item kinds, IR shape, and C ABI between compiler and runtime are scaffolded across the workspace; substantive bodies land per concept area as the work progresses.

The lex pass has a real body for ASCII identifiers, keywords, integer literals, operators, comments, and trivia attachment; string, char, float, raw, and byte literals are deferred to follow-up rounds. The parser, resolver, typecheck framework, scheduler, and codegen target trait have type surfaces in place and skeleton bodies; per-production parser rounds, validator bodies, and codegen lowering land milestone by milestone. The runtime ABI is committed at the descriptor level (opaque handle, scalar result, diagnostic shape); the Zig runtime currently exposes the three entry points as stubs and gains its interpreter body alongside the parser and resolver work.

vehje tracks unstable rustc features (`adt_const_params`, `generic_const_exprs`, `const_trait_impl`, `try_trait_v2`) where they enable work in `arvo` and `hilavitkutin` that vehje consumes; features known to have soundness issues are intentionally skipped.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/vehje/blob/main/LICENSE)
