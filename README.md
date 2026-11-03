# `vehje`

> A typed authoring language with a stable IR, for running directly on a small runtime or transpiling to source in another target language.

`vehje` is a scripting language for game modding and embeddable runtimes. The compiler runs at development time and ships as one static binary per host platform, and what it produces is the IR. Either the runtime executes that directly, or a pluggable backend lowers it one step further into whatever the target host already accepts, such as Lua source, a Wasm module or a configuration tree for an engine. The runtime is a small program written in Zig that never sees source, only IR the compiler has already proven valid, and it talks to the compiler across a C ABI.

The source surface is typed and modular, with tagged enums and trait dispatch where other languages would use dynamic dispatch and runtime type checks. Numbers are exact-width and fixed-point native, so you write `i23`, `u14` or `fixed<2, 9>` with whatever range and precision the value calls for and the compiler picks the storage under [`arvo`](https://github.com/orgrinrt/arvo)'s strategies, while floats are discouraged. References and borrows are part of the IR's contract, and a backend decides how to express them in a target language that may not tell borrow from move.

## The compiler

The compiler is meant to run as a [`hilavitkutin`](https://github.com/orgrinrt/hilavitkutin) pipeline, with the front end as a fixed sequence of WorkUnits, `vehje-lex` for tokens, `vehje-syntax` for the tree, `vehje-resolve` for scopes and bindings and `vehje-typecheck` for typed, coherence-checked items. Validators, lints and macro expansions are meant to be WorkUnits in the same graph. The front-end crates are `#![no_std]` with no allocator, so nothing assumes the host matches the runtime's target.

A backend is a Rust crate implementing the `CodegenTarget` trait in `vehje-codegen` and registered in a const `TargetRegistry`, which makes it a statically linked extension point and not a dynamically loaded plugin. `NativeTarget` and `JominiTarget` are the two built-ins, both stubs for now, and `vehje-jomini`, for Clausewitz engine bytecode, is planned as a sibling repo.

## The runtime

The C ABI is declared in `vehje-runtime-abi`, an opaque `VehjeRuntime` handle, a `VehjeResult` return code, a `VehjeDiagnostic` for errors, and three entry points, `vehje_runtime_new` to open an IR artefact, `vehje_runtime_execute` to run it and `vehje_runtime_free` to release the handle. The bodies are skeletons today, `vehje_runtime_new` returns null and `vehje_runtime_execute` always returns an error. Execution is meant to start out interpreter-shaped, with ahead-of-time bundles for selected targets later and the source language and ABI unchanged.

## Status

In the design phase. The crate split, the language item kinds, the IR shape and the C ABI are scaffolded across the workspace and bodies land per concept area. The lexer has a real body for ASCII identifiers, keywords, integer literals, operators, comments and trivia, with string, char, float, raw and byte literals still to come, and the parser, resolver, typechecker, scheduler and codegen trait have type surfaces with skeleton bodies. It tracks unstable rustc features (`adt_const_params`, `generic_const_exprs`, `const_trait_impl`, `try_trait_v2`) that `arvo` and `hilavitkutin` use, and skips the ones with known soundness problems.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/vehje/blob/main/LICENSE)
