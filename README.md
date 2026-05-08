# `vehje`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/vehje.svg)](https://github.com/orgrinrt/vehje/stargazers)
[![Crates.io](https://img.shields.io/crates/v/vehje)](https://crates.io/crates/vehje)
[![docs.rs](https://img.shields.io/docsrs/vehje)](https://docs.rs/vehje)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/vehje.svg)](https://github.com/orgrinrt/vehje/issues)
![License](https://img.shields.io/github/license/orgrinrt/vehje?color=%23009689)

> Typed authoring language with a stable IR for either direct runtime execution or pluggable transpilation to source in another target language.

</div>

`vehje` is a scripting language for game modding and embeddable runtimes. The compiler runs at development time and ships as a single static binary per host platform. The source surface is typed and modular, with tagged enums and monomorphised dispatch in place of dyn-trait and runtime type checks. Compiler and pluggable backends are both written in Rust; the static guarantees carry into both, so misbehaviour surfaces at compile time rather than after the IR has shipped.

What ships is the IR. Either the runtime executes it directly, or a pluggable backend lowers the IR one more step and emits the result in whatever form the target's host already accepts: Lua source for an embedded interpreter, a Wasm module for a sandboxed runtime, a configuration tree for an engine that reads everything as data. The transpilation path is where the typed surface earns its keep: a target that is neither typed nor strict gains an authoring layer above it that is both.

The C ABI between compiler and runtime is the boundary contract. The runtime is written in Zig, ships as a small, optimised binary suitable for embedding alongside other code, and never sees source: it consumes IR the compiler has already proven valid. The compiler itself runs `no_std`, with no allocator, no dyn dispatch, and no surprise platform calls; the entire compile lifetime is under explicit control. Targets and backends slot in through that same explicit boundary; the surface is pluggable by design, and a new emission target arrives without forking `vehje`.

## The source surface

vehje has a typed, modular surface. Types are declared up front and enforced by the compiler; tagged enums plus trait dispatch take the place of dynamic dispatch and runtime type checks. Diagnostics carry phase, span, and the values the compiler saw; warnings are denied at the crate root. Developers coming from Rust will find much of the syntax and semantics familiar. The language has its own item kinds where it needs them: `event` for typed event hooks, `expect` / `actual` for cross-crate paired declarations, `sealed` for closing a trait against external implementation.

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

The compiler runs as a hilavitkutin pipeline. The front-end emits IR through a fixed sequence of WorkUnits: tokens (`vehje-lex`), AST (`vehje-syntax`), resolved scopes and bindings (`vehje-resolve`), typed and coherence-checked items (`vehje-typecheck`). Macro expansions, validators, and content lints are also WorkUnits, scheduled into the same DAG. Pipeline state (symbol tables, type contexts, diagnostics, the IR itself) lives in scheduler-owned Resources and Columns. The back-end emits IR through `vehje-codegen`, which sits at the end of the pipeline as one more WorkUnit.

Front-end crates are `#![no_std]` with no allocator; they produce IR without ever materialising runtime state. Cross-compilation is first-class: nothing in the pipeline assumes the host platform matches the runtime's target. The compiler ships as a single static binary per host platform.

```rust
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

A backend ships as a hilavitkutin extension: a cdylib loaded through hilavitkutin-linking's pull-based symbol resolution, with no linker magic, no runtime registry, no init-ordering dance. Loading, invoking, and dropping a backend happens at any point during a compile, independent of any siblings.

vehje wraps hilavitkutin's extension interface in its own thin layer that generates the C ABI descriptor underneath an ergonomic Rust-shaped trait. The wrapper covers ABI versioning, item-kind acceptance, stage declarations, and the IR-handover-and-text-emit contract. A vehje build refuses to load a backend whose ABI version doesn't match, surfaced as a load-time diagnostic.

```rust
// a backend cdylib that emits text in another target language.

use vehje::extension::{backend, Backend, ItemKind, Unit, Output};
use vehje::diagnostic::Diagnostic;
use arvo::Bool;
use notko::Outcome;

#[backend(name = "my_target", abi_version = 1)]
pub struct MyTarget;

impl Backend for MyTarget {
    fn accepts(&self, kind: ItemKind) -> Bool {
        matches!(kind, ItemKind::Function | ItemKind::Const | ItemKind::Event).into()
    }

    fn compile(&self, unit: &Unit) -> Outcome<Output, Diagnostic> {
        // lowering + emission; output shape depends on what the target host accepts.
        Outcome::Ok(Output::empty())
    }
}
```

## The runtime

The runtime is a small Zig program that loads a vehje IR artefact and executes it. It speaks the C ABI declared in `vehje-runtime-abi`: an opaque `VehjeRuntime` handle, a `VehjeResult` scalar return code, a `VehjeDiagnostic` for error surfaces, and three core entry points (`vehje_runtime_new` to open an IR artefact, `vehje_runtime_execute` to run it, `vehje_runtime_free` to release the handle). The runtime pulls no Rust runtime dependency; the host program decides whether the runtime ships as a static library, a shared object, or a wasm module loaded into the host's existing wasm sandbox.

The execution path is interpreter-shaped initially, with bytecode dispatch over hilavitkutin WorkUnits inside the runtime itself. AOT-compiled bundles for selected targets ship later as additional output modes; the source language and the runtime ABI don't change when that happens. What survives the boundary in either case is the IR; the runtime's job is to evaluate it.

```rust
// host program embedding the vehje runtime via the C ABI.

use vehje_runtime_abi::{VehjeRuntime, VehjeResult};

fn run(artefact: &CStr) -> VehjeResult {
    let rt = unsafe { VehjeRuntime::open(artefact.as_ptr()) };
    let result = unsafe { rt.execute() };
    if result != VehjeResult::Ok {
        // surface diagnostic via vehje_runtime_diagnostic; details elided.
    }
    unsafe { rt.free() };
    result
}
```

## Status

vehje is in the design phase. The crate split, language item kinds, IR shape, and C ABI between compiler and runtime are scaffolded across the workspace; substantive bodies land per concept area as the work progresses.

The lex pass has a real body for ASCII identifiers, keywords, integer literals, operators, comments, and trivia attachment; string, char, float, raw, and byte literals are deferred to follow-up rounds. The parser, resolver, typecheck framework, scheduler, and codegen target trait have type surfaces in place and skeleton bodies; per-production parser rounds, validator bodies, and codegen lowering land milestone by milestone. The runtime ABI is committed at the descriptor level (opaque handle, scalar result, diagnostic shape); the Zig runtime currently exposes the three entry points as stubs and gains its interpreter body alongside the parser and resolver work.

vehje tracks unstable rustc features (`adt_const_params`, `generic_const_exprs`, `const_trait_impl`, `try_trait_v2`) where they unlock work in `arvo` and `hilavitkutin` that vehje consumes; features known to have soundness issues are intentionally skipped.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/vehje/blob/dev/LICENSE)
