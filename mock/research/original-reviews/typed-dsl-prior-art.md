# Typed Languages Compiling to Weakly-Typed Targets: Prior Art Review

**Date**: 2026-04-16  
**Purpose**: Inform the design of a Rust-inspired trait-based language that compiles to Clausewitz script (Paradox's weakly-typed mod scripting format).  
**Scope**: Survey of languages and mod tooling systems that have solved (or failed to solve) the typed-source → untyped-target problem.

---

## 1. TL;DR — Recommended Direction

The clearest lesson from a decade of "compile typed code to an untyped target" languages is that **the interop story determines everything**: whether your language gets adopted, whether it stays maintainable, and whether the community grows or fragments. TypeScript succeeded because it could absorb existing JavaScript incrementally; Elm is beloved but isolated because it cannot. For a mod authoring language targeting Clausewitz, **we should take the ReScript/Elm hybrid approach**: sound types with full inference (no `any`, no silent unsoundness), but with a **declared-extern / ambient-declaration interop mechanism** (not an FFI) so raw Clausewitz authored outside the language can be type-described and consumed. Scopes are the dominant type-level concern in Clausewitz — Stellaris has 20 distinct named scope types with hard constraints on which effects and triggers are valid in each — which makes this a **nominally typed domain by nature**: a Country and a Fleet should never be structurally compatible even if both have an `owner`. For error messages, we should target Elm's philosophy (first-person, pedagogical, no jargon) over Haskell/PureScript's (type-theoretically precise but hostile). Migration should be gradual: raw Clausewitz stays valid forever via declarations, and new code adopts the typed layer at whatever pace the author chooses.

---

## 2. Per-Language Summary Table

| Language | Type Flavor | Escape Hatch | Error UX | Most Relevant Lesson |
|---|---|---|---|---|
| TypeScript → JS | Structural, gradual, intentionally unsound | `any` (pervasive) | Good for common errors, jargon for complex | Declaration files (`.d.ts`) for typing legacy code is the right interop model |
| Elm → JS | Nominal-ish HM, total, sound | None (ports only) | Exceptional — friendly, first-person | No escape hatch = fewer runtime bugs but harder adoption; ports teach the right interop boundary |
| PureScript → JS | Structural type classes, total, sound | `foreign import` with manual type annotation | Haskell-grade hostile for type class errors | FFI is powerful but shifts unsafety burden to the author of bindings |
| Haxe → multi-target | Structural, total (mostly), sound | `untyped` keyword + `extern` declarations | Moderate | `extern` declarations for native APIs = exact model for typing raw Clausewitz |
| ReScript → JS | Nominal, total, sound | None (no `any`) | Friendly with ML lineage | Soundness wins over expressivity; nominal avoids footguns TypeScript cannot |
| Reason → JS/OCaml | Same as ReScript (ancestor) | Same | More familiar syntax reduces friction | Syntax ergonomics matter for adoption even when semantics are identical |
| Fable F# → JS | Nominal, total, sound | `Emit` attribute for inline JS | Moderate | Discriminated unions map naturally to Clausewitz's OR/AND block structure |
| Ceylon → JS/JVM | Union + intersection, nominal, sound | Interop blocks | Moderate | Union types are the right model for multi-scope contexts (planet \| ship) |
| Kotlin Multiplatform | Nominal, total, sound | `dynamic` type for JS | Good | `expect`/`actual` is the right model for "mod assumes mod X is loaded" |
| roblox-ts → Luau | Structural (TS), gradual | `any` (TS) | TS-standard | Proof the model works for game scripting; LuaTuple<T> solves multi-return |
| Skript (Minecraft) | Dynamic, implicit | Everything | Poor | Natural-language DSL becomes maintenance nightmare at scale; avoid |
| Bedrock Behavior Packs | JSON Schema validation, gradual | Raw JSON | IDE-dependent | Schema-as-types works for data, fails for logic; not enough for Clausewitz |
| CWTools (Clausewitz) | Rule-based AST validation | Raw Clausewitz always valid | Moderate (VS Code) | Proof-of-concept: typed-ish layer over Clausewitz already exists and is valued |
| OpenRA Traits (YAML) | Implicit structural | Raw YAML | None (runtime) | Condition strings as type-erased interface tokens — exactly what Clausewitz does |

---

## 3. Detailed Lessons Per Language

### 3.1 TypeScript → JavaScript

**Type system flavor**: Structural and intentionally unsound. TypeScript's lead architect Anders Hejlsberg has stated explicitly that soundness is not a design goal. TypeScript favors practicality over correctness: bivariant function subtyping, covariant array subtyping, and the `any` escape hatch are all deliberate choices to ease migration from existing JavaScript codebases.

**Escape hatch**: `any` — the universal super- and sub-type. A value typed `any` satisfies every constraint and accepts every value. This is the original sin of TypeScript: in large teams, `any` leaks through type boundaries invisibly. The `unknown` type was added later as a safer alternative that requires explicit narrowing before use, but `any` persists for legacy and convenience.

**Interop story**: Declaration files (`.d.ts`) are the defining innovation. For any JavaScript library without TypeScript source, you write a `.d.ts` file declaring the shapes TypeScript should assume. The DefinitelyTyped repository (>8,000 packages) is the largest distributed type-annotation effort in software history. This model is powerful: it separates the concern of "what type does this have" from "what code does this produce."

**Error message quality**: Excellent for simple errors (missing property, wrong type in a function call), degrades rapidly for complex generic errors. Type parameter constraint violations can produce multi-line cascading errors that require expert interpretation.

**Migration story**: The gold standard for gradual adoption. Start with `.js` files, rename to `.ts`, add `any` everywhere, progressively tighten. TypeScript can also consume plain JavaScript in the same project. This incrementalism is the primary reason for TypeScript's dominance.

**Notable failures**: Unsoundness accumulates. Teams that tolerate `any` end up with a codebase that is type-annotated but not actually type-safe. The "gradual typing trap": the type system provides false confidence where `any` silently crosses boundaries. Research (Safe & Efficient Gradual Typing for TypeScript, POPL 2015) showed that adding runtime enforcement to TypeScript's gradual types incurs significant overhead — meaning the unsoundness is load-bearing for performance.

**Lessons for our case**: The `.d.ts` ambient declaration model is exactly what we need for raw Clausewitz interop. Write a "Clausewitz declaration file" (`.cdts`?) that describes what effects, triggers, and scope transitions exist. These declarations let typed code call into raw script without losing type information. However, we should NOT copy `any` — our domain (scope types, effect validity) has no analog to "this could be anything." A Country scope is never "anything."

---

### 3.2 Elm → JavaScript

**Type system flavor**: Hindley-Milner with full inference. Total (everything typed), sound (no type can be wrong at runtime), and the language guarantees "no runtime exceptions in practice." Elm uses nominal-ish typing: while the underlying HM inference is structural in mechanism, the user-facing ADTs (custom types) behave nominally because they must be explicitly constructed and destructed.

**Escape hatch**: None. This is by design and by philosophy. There is no `any`, no `unsafe`, no `foreign import` that takes a raw JS value without a type annotation. The boundary with JavaScript is controlled exclusively through **ports** (typed message channels) and **flags** (typed initialization data). Values crossing the port boundary are validated at runtime via Elm's decoder system.

**Interop story**: The port system treats JavaScript like an external system in Clean Architecture — useful for "infrastructure" (localStorage, clipboard, network), but kept at deliberate arm's length from core logic. The official Elm guide states: *"Ports are not a traditional FFI for calling JS functions directly. They need a different mindset!"* The upshot is that Elm programs compose cleanly with JavaScript *orchestration* but cannot call JS *functions* inline. The project elm-ts-interop allows writing TypeScript type annotations that mirror port shapes, creating a type-safe channel in both directions.

**Error message quality**: Genuinely exceptional and frequently cited as an industry benchmark. Elm's compiler uses first-person language ("I found a type mismatch"), points directly to the relevant lines, explains the specific contradiction, and often suggests the fix. The philosophy is that the compiler is a collaborator, not a judge. Unlike Haskell/GHC where error messages expose internal unification state, Elm hides the underlying algorithm and surfaces only the user-visible consequence.

**Migration story**: Essentially all-or-nothing within a module. Elm files cannot gradually adopt types — an Elm file is either fully Elm or not Elm. The "widget approach" (embed Elm into a single `<div>` on a page that is otherwise JavaScript) works for web but has no analog in mod authoring. This is the main practical friction.

**Notable failures / pain points**: The lack of an FFI is also the biggest complaint. When a JavaScript library exists and works, Elm users must either write a port abstraction or go without. For a game modding language, the equivalent is: if a raw Clausewitz construct exists and works, our language must be able to describe it without requiring a full rewrite. Elm's model cannot do this.

**Lessons for our case**: Elm proves that "no escape hatch + friendly errors" produces the best correctness story. We should adopt the error message philosophy wholesale. We should NOT adopt Elm's interop model for this domain — mod authors will always have legacy raw Clausewitz that must remain callable. Ports (typed message channels) are not the right analog; Haxe externs or TypeScript `.d.ts` files are.

---

### 3.3 PureScript → JavaScript

**Type system flavor**: Haskell-style, with type classes (the direct inspiration for Rust's trait system). Structural in some senses (row polymorphism for records), nominal for user-defined types. Total and sound. Full higher-kinded types.

**Escape hatch**: `foreign import` — a declaration that imports a JavaScript function and assigns it a PureScript type. The type annotation is written by the developer and is trusted unconditionally. There is no runtime checking. This is a "pinky-swear FFI": you promise the type is correct, PureScript believes you.

**Interop story**: The FFI is ergonomically clean but puts all the burden on the binding author. A foreign import looks like:

```purescript
foreign import myJsFunc :: String -> Int
```

There is a corresponding `.js` file that provides the actual implementation. PureScript will compile typed PureScript that calls this function and generate JavaScript that calls the raw JS. If the declared type is wrong, the error manifests at runtime in JavaScript, not at PureScript compile time.

**Error message quality**: The most common criticism of PureScript is that type class errors are opaque and require understanding the theory. When a type class constraint fails to unify, the error message describes the failing unification in full type-theoretic terms. Beginners regularly get stuck on errors that would be clear in Elm.

**Migration story**: As all-or-nothing as Elm within a module, but `foreign import` means you can call existing JavaScript from the start. A new PureScript project can delegate to existing JS libraries immediately.

**Notable failures**: The "type class dictionary" overhead in generated JavaScript can be surprising. Heavy use of type classes produces JavaScript that passes implicit dictionaries everywhere, leading to performance and readability concerns. The FFI boundary is also a known source of runtime bugs in production PureScript codebases, because the trust is unconditional.

**Lessons for our case**: The `foreign import` / trusted-annotation model is one candidate for our raw Clausewitz interop. But the pinky-swear problem is real — if the annotation is wrong, the error is silent and runtime-only. A better model (see TypeScript `.d.ts`) separates the trusted annotation from the call site, putting it in a declaration file that can be reviewed and tested independently.

---

### 3.4 Haxe → Multiple Targets

**Type system flavor**: Structural with strict enforcement by default. The compiler produces JavaScript, C++, PHP, C#, Java, Python, and Lua from a single source. The type system is uniform across all targets — the same Haxe type checks regardless of backend.

**Escape hatch**: Two mechanisms. First, `extern` classes: typed descriptions of native APIs that exist on the target but are not defined in Haxe. Extern methods have no body — they declare shape only. Second, the `untyped` keyword: a prefix that disables type checking for an expression entirely, analogous to TypeScript's `as any` but more syntactically explicit. `untyped` is expected to be rare and is considered a last resort.

**Interop story**: Externs are the primary interop mechanism and are extremely well-suited to our case. For a JavaScript target, you write `extern class SomeJSLib` with method signatures. Haxe trusts these annotations and generates calls to the native API. The Haxe Standard Library ships externs for Flash and JavaScript built-ins. This is exactly the model we need: write `extern scope Country` with the effects and triggers that a Country scope supports, and the compiler can validate that your code only calls valid operations.

The documentation states externs "describe target-specific interaction in a type-safe manner" and "assume that the defined types exist at run-time but assume nothing about how and where those types are defined." This is a precise description of what we need for Clausewitz.

**Error message quality**: Moderate. Better than PureScript for type errors, not as friendly as Elm. The multi-target nature means some errors are target-specific and not caught until a particular backend is selected.

**Migration story**: Haxe targets each compile independently — there is no "gradual Haxe" within a target. You either compile your codebase to JS or you don't. However, because externs exist, you can use Haxe alongside raw JavaScript via interop from day one.

**Notable failures**: The multi-target abstraction occasionally leaks. Lua's multi-return, JavaScript's prototype chain, C++'s manual memory management — all produce target-specific edge cases that require `extern` annotations or `untyped` workarounds. This is actually a useful warning for our case: the Clausewitz "target" has quirks (scope chain, THIS/ROOT/PREV/FROM) that will require target-specific annotation.

**Lessons for our case**: The `extern` model is the most directly applicable to Clausewitz interop. Define extern declarations for the Clausewitz standard library (vanilla effects, triggers, scopes), and let the compiler validate against them. The `untyped` escape hatch is the right model for "I know what I'm doing, bypass the type checker here" — rare, explicit, and syntactically loud.

---

### 3.5 ReScript (formerly BuckleScript) → JavaScript

**Type system flavor**: Hindley-Milner with nominal types (no structural subtyping on user-defined types). Sound. No `any`, no `unknown`, no coercion. The compiler's design motto is "if it compiles, it runs correctly."

**Escape hatch**: No `any`. JavaScript interop uses explicit binding annotations (`@val`, `@module`, `@send`) that declare the JavaScript API shape. These are trusted but syntactically loud — there is no way to "slip in" an untyped value without writing an explicit binding.

**Interop story**: ReScript prioritizes "bindings over escape hatches." You write a binding module describing a JavaScript library's API. The ReScript compiler checks all calls against the binding. Incorrect bindings produce runtime errors, but the binding itself is a single auditable point of failure. The community maintains binding packages for common JS libraries.

**Error message quality**: Good. The Hindley-Milner lineage means type inference is complete — the compiler always knows the type of every expression — so errors are specific and localized. Error messages follow the OCaml tradition of being precise but can feel terse to developers not familiar with functional languages.

**Migration story**: More all-or-nothing than TypeScript, but bindings let you use existing JS immediately. ReScript cannot mix ReScript and JavaScript in the same file.

**Notable failures**: ReScript diverged significantly from its OCaml roots, which fragmented the community (Reason, BuckleScript, ReScript). Frequent breaking changes in the early years damaged trust. The "no any" stance means some JavaScript idioms simply cannot be expressed without writing a binding, which creates friction for rapid prototyping.

**Lessons for our case**: ReScript proves that a sound, no-escape-hatch language for JS can be practical and widely adopted. The binding annotation model (`@val "myFunc" external myFunc: string => int`) is a lean alternative to full declaration files — worth considering for inline scope declarations.

---

### 3.6 Reason → JavaScript / OCaml

**Type system flavor**: Same as ReScript (it's the same underlying language with different syntax). The key innovation was making OCaml's syntax look more like JavaScript/C to ease adoption.

**Interop story**: Same as ReScript via Melange (the modern successor to BuckleScript for Reason).

**Notable value**: Reason teaches a lesson in **syntax ergonomics vs. semantics**. The core insight is that developers adopt languages based on how familiar the surface looks, not the underlying theory. Reason chose to look like JavaScript; our language should consider looking like Rust (given the stated design inspiration) because that is what the target author community is or will be familiar with.

**Lessons for our case**: Aesthetic familiarity reduces the cognitive activation energy to learn the language. If the target mod author is a programmer, Rust-inspired syntax is a strong choice. If the target is a modder who has never programmed, a more Clausewitz-adjacent syntax might win. These are different languages for different users — be explicit about which user you are designing for.

---

### 3.7 Fable F# → JavaScript (and more)

**Type system flavor**: F#'s type system — nominal ADTs (discriminated unions), structural interfaces, total, sound within F# semantics. Generics are erased in generated JavaScript (as in TypeScript). Fable now targets not just JavaScript but also Python, Rust, Dart, and Erlang.

**Escape hatch**: The `Emit` attribute allows embedding raw target-language code inside F#:

```fsharp
[<Emit("console.log($0)")>]
let logToConsole (msg: string): unit = jsNative
```

The `jsNative` placeholder is never called at runtime — it is only there to satisfy F# type checking. This is an honest and explicit "here be dragons" marker.

**Interop story**: F# discriminated unions compile to JavaScript objects with a `Case` tag and `Fields` array. Pattern matching compiles to switch statements on the tag. This is directly relevant because Clausewitz's `OR = { ... }` / `AND = { ... }` blocks are essentially tagged union constructs — our language's algebraic types would compile down naturally.

**Error message quality**: Moderate. F#'s error messages are often clear but occasionally expose type unification internals. Fable adds a layer of potential confusion when errors originate in the JS interop boundary.

**Notable failures**: The `Emit` escape hatch is powerful but unsafe — it bypasses the type system entirely and produces raw JavaScript. In practice, `Emit` is overused in Fable codebases because writing proper bindings requires significant boilerplate.

**Lessons for our case**: The `Emit`-style escape hatch is worse than Haxe's `extern` or TypeScript's `.d.ts` because it is per-call-site, not per-declaration. Each use requires trusting a single developer's annotation at a single call site. Prefer declaration-level interop (extern/`.d.ts`) over call-site-level interop (`Emit`/`unsafe`). F#'s discriminated unions also confirm that algebraic types are the right model for Clausewitz's OR/AND logic blocks.

---

### 3.8 Ceylon → JavaScript / JVM

**Type system flavor**: Nominal with first-class **union types** and **intersection types**. Ceylon was the first practical language to implement free-form union types (e.g., `String|Integer`) and intersection types (e.g., `Identifiable&List<String>`). The Ceylon specification notes: *"Ceylon was the first language to demonstrate practical applications of free-form union and intersection types, and alerted the programming language community to the importance of these constructs."*

**Escape hatch**: Ceylon has interop blocks for calling Java from Ceylon and vice versa. These are typed but require explicit annotations.

**Interop story**: Ceylon was explicitly designed to feel "equally at home on the JVM and on any JavaScript VM." Unlike languages designed for one target and ported to another, Ceylon's numeric types were designed from the start with both VMs in mind — no JVM-specific overflow semantics that would break on JS.

**Notable failures**: Ceylon was discontinued after Red Hat ended investment. The ecosystem never grew large enough to sustain community maintenance.

**Lessons for our case**: Union types are the precise right tool for Clausewitz's multi-scope contexts. In Stellaris, many scripted effects accept multiple scope types — a `create_ship` effect works in both Planet and Country scopes. In Ceylon's model: `effect valid_in: Planet | Country`. This is more precise than structural matching and more honest than pretending the effect works in any scope. Intersection types have an analog too: an entity that is both a Leader and a Scientist is `Leader & Scientist`. We should strongly consider union and intersection types in our type system.

---

### 3.9 Kotlin Multiplatform

**Type system flavor**: Nominal, total, sound. Kotlin's type system is one of the cleanest in the mainstream — null-safety built into the type system, no implicit widening, algebraic-ish sealed classes.

**Escape hatch**: `dynamic` type for JavaScript targets only. For the JVM and Native targets, there is no escape hatch.

**Interop story**: The `expect`/`actual` mechanism is the primary multi-target abstraction:

```kotlin
// commonMain
expect fun platformSpecificOp(): String

// jsMain
actual fun platformSpecificOp(): String = js("navigator.userAgent")

// jvmMain  
actual fun platformSpecificOp(): String = System.getProperty("os.name")
```

This mechanism is compile-time verified: the compiler ensures every `expect` declaration has a corresponding `actual` for each target. If you add a new `expect` without providing an `actual` for some target, the build fails.

**Lessons for our case**: The `expect`/`actual` pattern is the right model for **"this mod assumes mod X is loaded."** In our language: `expect modifier stability_boost` declares that a modifier named `stability_boost` must exist (defined either by our mod or by another loaded mod). The compiler can validate usage against the declaration, and at deploy time, verify that the actual definition exists in the mod load order. This is directly analogous to Cargo features but at the semantic level of game content rather than code capabilities.

---

### 3.10 roblox-ts → Luau

**Type system flavor**: TypeScript's structural, gradual system. roblox-ts is TypeScript compiled to Roblox's Luau dialect.

**Interop story**: Typed declaration files describe the Roblox API (partially hand-written, partially auto-generated from the Roblox API dump). A special type `LuaTuple<T>` handles Lua's multi-return idiom:

```typescript
function getCoords(): LuaTuple<[number, number, number]>
```

This compiles to Lua that handles multiple returns correctly while presenting a single typed value at the TypeScript level. This is directly relevant: Clausewitz effects and triggers can have multiple "return-like" outputs (events firing, scope changes, etc.) that need to be modeled.

**Error message quality**: TypeScript-standard — good for common errors, impenetrable for complex generics.

**Notable failures / pain points**: Because roblox-ts uses TypeScript, it inherits all of TypeScript's unsoundness. Teams still hit runtime errors from `any` leakage. The auto-generated declarations from the API dump are sometimes wrong, causing type-safe-looking code to fail at runtime.

**Lessons for our case**: Auto-generating declarations from the engine's own documentation is the right long-term strategy. Stellaris ships `script_documentation/` logs (accessible via the `trigger_docs` console command) that enumerate every effect, trigger, and scope with their valid contexts. We should parse these automatically into declaration files rather than maintaining them by hand.

---

### 3.11 Skript (Minecraft)

**Type system flavor**: Dynamic, implicit. Skript lets server admins write things like:

```
on player join:
    send "Welcome %player%!" to player
```

No types, no declarations, no structure — the language parses natural English-ish prose and pattern-matches against registered patterns.

**Notable failures**: Skript is acknowledged to suffer from "spaghetti code, inelegant control flow, a wasteful parsing process." The natural-language surface is deceptive: it looks simple but has complex and unpredictable precedence rules that trip up advanced users. The community eventually created `skript-parser` (a standalone rewrite) to address foundational design problems. Maintenance became nearly impossible as more features were added to the original parser.

**Lessons for our case**: Natural-language DSLs do not scale. The ambiguity that makes them feel approachable to beginners becomes a serious maintenance liability at complexity. Clausewitz's own scripting format is already fairly readable without being natural-language. Our typed front-end should be syntactically unambiguous and formally defined, even if it trades some surface readability for precision.

---

### 3.12 Minecraft Bedrock Behavior Packs

**Type system flavor**: JSON Schema validation. Minecraft Bedrock's behavior packs are authored as JSON files validated against published JSON Schemas (maintained in the `Blockception/Minecraft-bedrock-json-schemas` repository). This provides IDE autocompletion and schema-level validation but not semantic type checking.

**Notable strengths**: Schema validation is easy to implement and requires no new language tooling. VS Code handles it natively. Schemas can be updated when the game API changes without touching user code.

**Notable failures**: JSON Schema can validate structure (required fields, field types, allowed values) but cannot validate semantics (this effect is only valid in a country scope, this modifier requires a particular DLC). It also cannot express control flow, conditionals, or procedural content — meaning complex logic requires inline script strings that the schema cannot validate.

**Lessons for our case**: Schema validation is a viable low-effort stopgap but not a full solution. It would be worth shipping a JSON Schema for our language's compiled output (the Clausewitz files) as a secondary validation layer, but the primary type system should live in the source language.

---

### 3.13 CWTools (Clausewitz mod validation)

**Type system flavor**: Rule-based AST validation. CWTools (VS Code extension + .NET library) parses Clausewitz script files and validates them against rules that describe which keywords are valid in which contexts. It enforces variable scoping rules and detects type mismatches where expected data types don't align with game schemas.

**What it does well**: CWTools provides syntax highlighting, autocompletion, and live error checking for raw Clausewitz files. It is the closest existing precedent to typed tooling for Paradox mods. The VS Code extension is widely used in the Paradox modding community.

**What it cannot do**: CWTools validates structure but not full scope semantics. It cannot catch all invalid scope transitions (e.g., calling a planet-only effect from a country scope). It also cannot express intent: the rules it validates against are extracted from game data, not from author-declared types.

**Lessons for our case**: CWTools proves the community values typed tooling for Clausewitz. The fact that it exists and is used means our typed language is building on established demand, not creating it. CWTools' rule-based validation is a weaker version of our compile-time scope type checking — we should aim to catch everything CWTools catches plus scope-level semantic errors that CWTools cannot.

---

### 3.14 OpenRA Modding (YAML Traits)

**Type system flavor**: Implicit structural. OpenRA uses a YAML-based trait system where actors are composed of traits:

```yaml
Infantry:
  Inherits: BaseUnit
  Mobile:
    Speed: 3
  Health:
    HP: 150
  WithSpriteBody:
    ...
```

Traits interact via **condition strings** — one trait grants a condition (`GrantCondition: stunned`) and another trait responds to it (`RequiresCondition: stunned`). The conditions are plain strings; there is no type-level check that a condition is valid or that it will ever be granted.

**Lessons for our case**: OpenRA's condition system is structurally identical to Clausewitz's flag/modifier system — both use string tokens as type-erased interface contracts between components. This is exactly the problem our type system needs to solve: make condition/flag/modifier names first-class typed values that the compiler can verify, rather than plain strings that can be misspelled, renamed, or left dangling. The analogy is: OpenRA's `"stunned"` condition is a Clausewitz modifier like `"stability_boost"` — both should be typed references, not strings.

---

## 4. Decision Matrix for Our Language

### 4.1 Gradual vs. Total Typing

| | Gradual (TypeScript) | Total (Elm/ReScript) |
|---|---|---|
| Adoption curve | Easier — start anywhere | Steeper — all-or-nothing per file |
| Runtime safety | Unsafe at `any` boundaries | Safe by construction |
| Legacy interop | Natural | Requires explicit declarations |
| Refactoring | Misleadingly safe | Genuinely safe |
| Complexity | Lower (no inference needed everywhere) | Higher (inference must cover everything) |

**Recommendation**: **Total typing with extern declarations for interop**. Clausewitz scopes are not ambiguous — a Country is a Country, a Planet is a Planet. There is no legitimate reason for a scope to be "any." The domain does not benefit from gradual typing the way a JavaScript migration does. Total typing makes the scope-validation guarantee meaningful: if it compiles, the scope transitions are valid.

The key insight is that "gradual" is only needed when migrating existing untyped code. For raw Clausewitz authored outside our language, we use **ambient declarations** (the TypeScript `.d.ts` / Haxe `extern` model) — not `any`, but explicit type annotations on the untyped boundary.

### 4.2 Structural vs. Nominal Typing

| | Structural | Nominal |
|---|---|---|
| "Has an owner" → is owner-scoped | Yes — any object with `.owner` qualifies | No — Country and Fleet are distinct even if both have owners |
| Cross-scope footguns | High — Planet and Country might accidentally unify | None — they are distinct by declaration |
| Union types | Compatible with structural | Compatible with nominal |
| Extensibility | Easier to add new types that fit existing interfaces | Requires explicit declaration of compatibility |

**Recommendation**: **Nominal types for scopes, structural types for data records**. Scopes in Clausewitz have hard semantic boundaries — calling a planet-only effect in a country scope is a game error, regardless of what fields both share. Scope types must be nominal. Data records (ship design parameters, modifier values) can be structural — what matters is their shape.

**Specific decision**: Use TypeScript's branded type pattern semantically, implemented with nominal scope declarations. A `Country` scope and a `Fleet` scope are distinct nominal types even if they both expose `.owner`. Union types (from Ceylon) should be available: `effect valid_in: Planet | Country` is exact and expressible.

### 4.3 Interop Story

The three models from the prior art:

1. **FFI (PureScript)**: `foreign import` with trusted type annotation per function. Pro: simple. Con: trust is per-call-site, no centralized auditing.
2. **Ambient declarations (TypeScript `.d.ts`)**: Declare the shape of external code in separate files. Pro: centralized, auditable, auto-generatable from engine data. Con: more infrastructure.
3. **Extern classes (Haxe)**: Typed class declarations that describe native APIs. Pro: type-safe, per-module. Con: slightly more boilerplate than TypeScript style.

**Recommendation**: **Ambient declaration files** (`.clauses` or `.d.clauses`?) following the TypeScript `.d.ts` model, but auto-generated from Stellaris' own `script_documentation/` logs wherever possible. The engine already ships machine-readable documentation of all effects, triggers, and scope constraints via `trigger_docs`. Parse these into declaration files and ship them with the compiler. Raw Clausewitz written by the author or by other mods can be described by writing (or generating) a declaration file — no `any`, no silent unsoundness.

For the mod-dependency case (this mod assumes mod X is loaded): adopt the Kotlin `expect`/`actual` pattern. `expect modifier stability_boost` in a shared module; `actual modifier stability_boost = { ... }` either in our mod's output or in a declaration file describing the external mod.

### 4.4 Error Message Philosophy

Recommendation: **Elm's philosophy, not Haskell's**.

Concrete rules:
- First-person, not accusatory: "I found a type mismatch" not "Type error at line 42"
- Name the problem in terms of the domain, not the type system: "A Country scope does not support `planet_event` — try switching to a Planet scope first" not "Expected: ScopeType<Country>; Got: ScopeType<Planet>"
- Always suggest the fix when the fix is knowable: "Did you mean `every_owned_planet = { planet_event { ... } }`?"
- Distinguish errors (compilation stops) from warnings (compilation continues with annotation) — scope transitions that might work at runtime but aren't proven safe are warnings, not errors
- Never expose internal unification state in error messages

---

## 5. Patterns We Should Adopt

**Ambient declaration files (TypeScript `.d.ts` / Haxe `extern`)**: The cleanest model for typing raw Clausewitz. Write declarations once, validate everywhere. Auto-generate from engine data where possible.

**Nominal scope types with union support (Ceylon / Kotlin)**: Country and Fleet are distinct by name, not shape. But `Planet | Country` union types express multi-scope validity correctly.

**Expect/actual for mod dependencies (Kotlin)**: Declare that a modifier, flag, or effect must exist; provide or import the actual definition separately. This is how we model "requires mod X."

**Elm error message philosophy**: First-person, domain-language, fix-suggesting, non-algebraic. Concretely: test every error message with someone unfamiliar with the language. If they cannot understand it without reading a type theory primer, rewrite it.

**Algebraic types for OR/AND blocks (F# discriminated unions / Elm custom types)**: Clausewitz's `OR = { trigger_a trigger_b }` and `AND = { ... }` are tagged union structures. Our language should model these as ADTs and compile them back down cleanly.

**Condition/modifier/flag names as typed references (not strings)**: Analogous to fixing OpenRA's condition string problem. A modifier named `stability_boost` should be a typed identifier that the compiler can check exists in the namespace, not a plain string that silently fails at runtime.

**Scope chain as explicit traversal (THIS/ROOT/PREV/FROM modeling)**: TypeScript's `this` type and method chaining are analogous but too loose. We need an explicit scope stack model: entering a scope pushes a type onto the stack, exiting pops it. `prev` / `from` are typed references into the stack. This is novel — no prior language models this exactly, but it is the core type-level challenge of Clausewitz.

**`untyped` escape hatch (Haxe)**: Rare, syntactically loud, necessary. For the edge case where the author genuinely knows better than the type system (or where the type system's model of Clausewitz is incomplete), `untyped { raw clausewitz here }` should be available but visible in code review.

---

## 6. Patterns We Should Avoid

**`any` / implicit unsoundness (TypeScript)**: No. The value of our type system is in scope validation. An `any` scope destroys that value. If something needs to be untyped, it goes in a declaration file (declared structure) or in an `untyped` block (explicit escape).

**Natural-language surface syntax (Skript)**: Skript-style prose parsing is a maintenance nightmare. Clausewitz is already readable. Our language should be syntactically unambiguous and formally parseable.

**Call-site escape hatches (`Emit` in Fable)**: Embedding raw Clausewitz at the call site is worse than having typed declarations. It distributes trust across every use site rather than centralizing it in a declaration file.

**Elm's port model for interop**: Ports work for UI interactions with JavaScript. For mod authoring, the equivalent would be requiring all calls to vanilla Clausewitz to go through a typed channel — this is too restrictive. Authors need to call vanilla effects inline, not through an async messaging system.

**JSON Schema as the type system (Bedrock Behavior Packs)**: Schema validation alone cannot express scope semantics, conditional validity, or procedural content. Use it as a secondary layer if desired, not as the primary type system.

**Blanket trust in auto-generated declarations (roblox-ts warning)**: The roblox-ts team found that auto-generated API declarations from game data are sometimes wrong, causing type-safe-looking code to fail at runtime. Treat auto-generated declarations as a starting point that requires human review, not as ground truth.

**Big-bang migration (all-or-nothing like Elm within a project)**: Mod authors have existing Clausewitz they trust. The language must allow incremental adoption: describe existing raw Clausewitz with declarations, port one file at a time to the typed language, never require rewriting everything at once.

---

## 7. Open Questions for the User

**Q1 — Gradual within a single file?**  
Should a single source file be allowed to mix typed declarations and `untyped { }` raw Clausewitz blocks? Or should file-level discipline require everything in a typed file to be typed? TypeScript allows mixing at the expression level (which is the source of many `any` leaks). Elm enforces module-level discipline (which makes migration harder). The recommendation is module-level discipline with explicit `untyped` blocks — but you should decide how granular the escape hatch is.

**Q2 — Who writes the declaration files for vanilla Stellaris and other mods?**  
Options: (a) auto-generate from `trigger_docs` and ship with the compiler; (b) maintain by hand (unsustainable); (c) community-maintained like DefinitelyTyped; (d) a combination. Given that the engine ships machine-readable documentation, option (a) + (c) seems right. But who owns the canonical declarations for e.g. NSC3 or ESC, which are third-party mods in the load order? This is the mod-dependency problem — see Q3.

**Q3 — Mod dependency model: Cargo features or Kotlin expect/actual?**  
Two models: (a) Declare feature flags (`#[requires(nsc3)]`) and gate code blocks on them — like Cargo; (b) Declare `expect` content IDs and let the build system verify they exist in the load order — like Kotlin expect/actual. These are not mutually exclusive. Should we support both? Feature-gating is ergonomic for optional compatibility; expect/actual is better for hard dependencies.

**Q4 — How strict should scope inference be for scope chains?**  
Clausewitz allows silent scope transitions: entering `owner = { ... }` changes scope to Country without a type annotation. Should our language: (a) require explicit scope annotations everywhere, (b) infer scope from the block name and validate implicitly, or (c) allow both? Option (b) is closest to what Elm/HM inference does (infer everything, annotate nothing), but requires a complete model of Clausewitz's scope transition rules. This is feasible (the scope graph is finite and documented) but substantial.

**Q5 — Versioned declarations vs. version-neutral declarations?**  
Stellaris patches frequently change effect/trigger signatures and add new scopes. Should declaration files be versioned per game patch (like TypeScript declarations for Node.js versions)? Or should we track only the "stable" API and warn when deprecated features are used? Given that we already track vanilla via `.cache/vanilla/`, a per-patch declaration model seems natural — but it means declaration files must be regenerated after every patch.

**Q6 — Error message language: code-like or prose?**  
Elm uses friendly English prose. The alternative is structured, code-like error messages closer to Rust's (which are also praised for clarity but use more technical vocabulary). Given that our target audience includes modders who may not be programmers, English prose errors (Elm style) are probably more accessible. But for users with programming backgrounds, Rust-style structured messages with code context and caret markers are more informative. Could offer both via a `--verbose-errors` flag.

**Q7 — Scope stack depth limit?**  
Clausewitz allows chaining PREV up to four times (PREVPREVPREVPREV). Our scope stack type would need to model depth — a `Prev<Prev<Prev<Country>>>` type. This is expressible in a type system with generics, but can produce deeply nested types in error messages. Set a practical depth limit (4, matching Clausewitz)? Or represent it as a typed stack without nesting (a more Ceylon-like approach)?

---

## Sources

- TypeScript type erasure overview: [GeeksforGeeks](https://www.geeksforgeeks.org/typescript/what-is-type-erasure-in-typescript/), [FreeCodeCamp](https://www.freecodecamp.org/news/what-is-type-erasure-in-typescript/)
- TypeScript unsoundness analysis: [Effective TypeScript: Seven Sources of Unsoundness](https://effectivetypescript.com/2021/05/06/unsoundness/), [Safe & Efficient Gradual Typing for TypeScript (POPL 2015)](https://goto.ucsd.edu/~pvekris/docs/safets.pdf)
- TypeScript declaration files: [TypeScript Handbook: Type Declarations](https://www.typescriptlang.org/docs/handbook/2/type-declarations.html)
- TypeScript nominal/branded types: [TypeScript Deep Dive: Nominal Typing](https://basarat.gitbook.io/typescript/main-1/nominaltyping), [Effective TypeScript course](https://www.typescript-training.com/course/fundamentals-v3/05-structural-vs-nominal-types/)
- Elm language overview: [Elm Wikipedia](https://en.wikipedia.org/wiki/Elm_(programming_language)), [Introduction to Elm](https://guide.elm-lang.org/)
- Elm error messages: [Elm errors are paternalistic](https://jamalambda.com/posts/2021-06-13-elm-errors.html), [Writing Good Compiler Error Messages](https://calebmer.com/2019/07/01/writing-good-compiler-error-messages.html)
- Elm JavaScript interop limits: [The Limits of Elm/JS Interop](https://guide.elm-lang.org/interop/limits)
- Elm ports: [Ports — An Introduction to Elm](https://guide.elm-lang.org/interop/ports.html), [Bridging Elm and JavaScript with Ports](https://thoughtbot.com/blog/bridging-elm-and-javascript-with-ports)
- Elm migration: [Starting Small with Elm](https://cekrem.github.io/posts/starting-small-with-elm-a-widget-approach/)
- PureScript FFI: [The Foreign Function Interface — PureScript by Example](https://book.purescript.org/chapter10.html), [PureScript documentation: FFI guide](https://github.com/purescript/documentation/blob/master/guides/FFI.md)
- PureScript overview: [PureScript Wikipedia](https://en.wikipedia.org/wiki/PureScript)
- Haxe compiler targets: [Haxe Compiler Targets documentation](https://haxe.org/documentation/introduction/compiler-targets.html)
- Haxe externs: [Externs — Haxe manual](https://haxe.org/manual/lf-externs.html)
- Haxe untyped: [untyped — Haxe manual](https://haxe.org/manual/type-system-untyped.html)
- ReScript overview: [ReScript GitHub](https://github.com/rescript-lang/rescript), [ReScript 2025 deep dive](https://dev.to/cristiansifuentes/rescript-2025-the-top-javascript-alternative-tech-deep-dive-d0g)
- ReScript type system: [Why you should try ReScript](https://sminn.ee/posts/2023-09-04-why-you-should-try-rescript/), [ReScript forum: OCaml type features to preserve](https://forum.rescript-lang.org/t/what-stable-type-system-features-we-are-going-to-preserve-from-ocaml/1251)
- Reason overview: [Reason — What & Why](https://reasonml.github.io/docs/en/what-and-why), [Reason Wikipedia](https://en.wikipedia.org/wiki/Reason_(programming_language))
- Fable overview: [Fable website](https://fable.io/), [Fable GitHub](https://github.com/fable-compiler/Fable)
- Fable F# interop: [F# Interop with Javascript in Fable: The Complete Guide](https://medium.com/@zaid.naom/f-interop-with-javascript-in-fable-the-complete-guide-ccc5b896a59f)
- Ceylon union/intersection types: [Union, intersection, and enumerated types — Eclipse Ceylon](https://ceylon-lang.org/documentation/1.2/tour/types/), [Ceylon Wikipedia](https://en.wikipedia.org/wiki/Ceylon_(programming_language))
- Kotlin expect/actual: [Expected and actual declarations — Kotlin docs](https://kotlinlang.org/docs/multiplatform-expect-actual.html)
- roblox-ts: [roblox-ts GitHub](https://github.com/roblox-ts/roblox-ts), [roblox-ts documentation](https://roblox-ts.com/docs/)
- Skript: [SkriptLang GitHub](https://github.com/SkriptLang/Skript)
- Minecraft Bedrock JSON schemas: [Blockception/Minecraft-bedrock-json-schemas](https://github.com/Blockception/Minecraft-bedrock-json-schemas)
- CWTools: [cwtools GitHub org](https://github.com/cwtools), [CWTools VS Code extension](https://marketplace.visualstudio.com/items?itemName=tboby.cwtools-vscode)
- OpenRA traits: [OpenRA Traits documentation](https://docs.openra.net/en/release/traits/), [OpenRA content and modding — DeepWiki](https://deepwiki.com/OpenRA/OpenRA/3-content-and-modding)
- Stellaris scopes: [Scopes — Stellaris Wiki](https://stellaris.paradoxwikis.com/Scopes), [Scopes — Stellaris Wiki (Fandom)](https://stellaris.fandom.com/wiki/Scopes)
- Clausewitz scripting language: [PDXTools script documentation](https://ititus.github.io/PDXTools/script)
- Jomini parsers: [nickbabcock/jomini (JS)](https://github.com/nickbabcock/jomini), [rakaly/jomini (Rust)](https://github.com/rakaly/jomini)
