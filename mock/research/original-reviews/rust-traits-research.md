# Rust Trait System Research
## For the Clausewitz Mod Authoring Language Design

**Date:** 2026-04-16  
**Purpose:** Inform the design of a Rust-inspired trait language that compiles to Clausewitz script.

---

## 1. Quick Summary

Rust's trait system is a statically-dispatched, compile-time contract mechanism. A trait declares a set of methods, associated types, and associated constants that implementing types must provide. Coherence rules (the orphan rule + overlap prohibition) ensure that for any `(Type, Trait)` pair, at most one impl exists globally, and that impl lives in a crate that "owns" either the type or the trait. Method resolution uses an ordered candidate search — inherent methods first, then trait methods — with automatic deref/ref coercions along the way. `dyn Trait` enables runtime dispatch through fat pointers + vtables, but only for traits that satisfy object safety. The `#[derive]` mechanism auto-generates impls via procedural macros. The module system (`mod`/`pub`/`use`) controls visibility with fine-grained scopes (`pub(crate)`, `pub(super)`, `pub(in path)`). For our mod authoring language, traits become the mechanism for declaring what "capabilities" a modded entity must expose (e.g., `impl Patchable for vanilla::Country`), while coherence rules port directly to "one patch mod may not override another patch mod's impl of the same trait on the same type."

---

## 2. Trait Declaration

### 2.1 Full Syntax

```
trait IDENTIFIER [GenericParams] [: TypeParamBounds] [WhereClause] {
    [InnerAttribute]*
    [AssociatedItem]*    -- functions, types, constants
}
```

Optional `unsafe` prefix for traits with implementation-side safety obligations (e.g., `unsafe trait Send`).

### 2.2 Associated Functions and Methods

Methods are associated functions whose first parameter is a form of `self`. Shorthand forms:

| Shorthand | Desugars to |
|---|---|
| `self` | `self: Self` |
| `&self` | `self: &Self` |
| `&mut self` | `self: &mut Self` |

**Declaration** (no body — implementor must provide it):
```rust
fn method(&self) -> String;
```

**Default implementation** (body provided — implementor may override):
```rust
fn method(&self) -> String {
    format!("default: {:?}", self.name())
}
```

Default methods can call other methods on the same trait, including required ones:
```rust
pub trait Summary {
    fn summarize_author(&self) -> String;   // required

    fn summarize(&self) -> String {          // default — calls required
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

**Constraint:** Trait functions cannot be `const fn` (as of stable Rust 2024). Parameter patterns in trait declarations (no body) may only be `IDENTIFIER`, `_`, or `SelfParam`; `mut IDENTIFIER` is deprecated.

### 2.3 Associated Types

```rust
trait Container {
    type E;                              // required, no default
    type Header: Display;               // with bounds
    fn insert(&mut self, elem: Self::E);
}

impl<T> Container for Vec<T> {
    type E = T;
    type Header = String;
    fn insert(&mut self, x: T) { self.push(x); }
}
```

Access syntax: `<Vec<i32> as Container>::E` or (if unambiguous) `Vec::<i32>::E`.

**Generic Associated Types (GATs):** Associated types can themselves be generic (stable since Rust 1.65):
```rust
trait Lend {
    type Lender<'a> where Self: 'a;
    fn lend<'a>(&'a mut self) -> Self::Lender<'a>;
}
```

### 2.4 Associated Constants

```rust
trait ConstantId {
    const ID: i32;             // required
    const VERSION: &str = "1.0";  // default
}
```

### 2.5 Supertraits

`trait Circle: Shape` means any type implementing `Circle` must also implement `Shape`. This is syntactic sugar for a `where Self: Shape` bound on the trait itself.

```rust
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();   // Display method available here
        println!("** {output} **");
    }
}
```

Supertrait methods are available inside default method bodies. A type that fails to implement the supertrait gets a compile error when attempting `impl OutlinePrint`.

Equivalence:
```rust
trait Circle: Shape { ... }
// Same as:
trait Circle where Self: Shape { ... }
```

### 2.6 Where Clauses

Where clauses can express constraints that cannot go inline:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
    T::Item: Copy,            // bound on associated type
    String: PartialEq<T>,    // bound on a non-parameter type
{ ... }
```

### 2.7 Generic Parameters on Traits

```rust
trait Seq<T> {
    fn len(&self) -> u32;
    fn elt_at(&self, n: u32) -> T;
}
```

A type can implement `Seq<i32>` and `Seq<String>` simultaneously (unlike associated types — see Section 5).

### 2.8 Implicit `Self` Parameter

Every trait definition implicitly declares a type parameter `Self` that resolves to the implementing type. All associated items (methods, types, constants) can reference `Self`.

---

## 3. Impl Resolution Algorithm

### 3.1 The Lookup Algorithm (Method Call Expression)

When the compiler sees `x.method(args)`, the resolution proceeds in two phases:

**Phase 1 — Build the candidate receiver type list:**

Starting from the type of `x`, build a list by:
1. Add the type itself.
2. Repeatedly dereference (using `Deref` / `DerefMut`), appending each new type.
3. At the end, attempt one unsized coercion (e.g., `[T; N]` → `[T]`), append if successful.
4. For each type `T` in the list, also immediately add `&T` and `&mut T`.

Example — for `Box<[i32; 2]>`, candidates are:
```
Box<[i32;2]>, &Box<[i32;2]>, &mut Box<[i32;2]>
[i32;2],      &[i32;2],      &mut [i32;2]
[i32],        &[i32],        &mut [i32]
```

**Phase 2 — For each candidate type in order, search for a matching visible method:**

1. **Inherent methods** — methods defined directly on the type in an `impl Type { }` block.
2. **Trait methods** — methods from traits in scope that this type implements.
   - If the type is a type parameter `T: Foo`, bounds are searched first.
   - Then all remaining in-scope trait methods.

The search stops at the first candidate type + method that matches. This means inherent methods on `T` shadow trait methods on `T`, but a trait method on `T` (requiring `&self`) can be found before an inherent method on `T` requiring `&mut self` (because `&T` appears in the list before `T`'s `&mut self` receiver).

**Counterintuitive example:**
```rust
impl Foo {
    fn bar(&mut self) { println!("inherent"); }
}

impl SomeTrait for Foo {
    fn bar(&self) { println!("trait"); }
}

let mut f = Foo{};
f.bar();   // Prints "trait" — &Foo is found first, trait method matches
```

**Phase 3 — Validation:**

After a match is found, the compiler validates mutability, lifetimes, and unsafe requirements. These do NOT affect which method is selected — only whether it's legal to call.

**Disambiguation:** When two traits both provide `method`, the author must use fully qualified syntax:
```rust
<Type as Trait>::method(receiver, args);
```

### 3.2 Inherent vs Trait Methods

- **Inherent:** `impl MyType { fn method(...) {} }` — no trait involved.
- **Trait:** `impl SomeTrait for MyType { fn method(...) {} }`.

A type can have multiple inherent `impl` blocks (in any file in the same crate). A type can implement any number of distinct traits.

### 3.3 Blanket Implementations

An impl with no concrete types, only generic parameters, is a blanket impl:
```rust
impl<T: Display> ToString for T { ... }
```
This gives `to_string()` to every type that implements `Display`. Blanket impls are resolved last — a concrete `impl ToString for MyType` wins over the blanket.

Source: method resolution is specified in the Rust Reference under "Method-call expressions" — the receiver candidate construction and lookup order is exact as described above. The rustc implementation lives in `rustc_hir_typeck/src/method/probe.rs`.

---

## 4. Coherence / Orphan Rules

### 4.1 The Rule

For `impl<P1..=Pn> Trait<T1..=Tn> for T0`, the impl is **only allowed** if:
- **Condition A:** `Trait` is a local trait (defined in the same crate), OR
- **Condition B:** At least one of `T0..=Tn` is a local type `Ti`, AND no uncovered type parameter `Pj` appears in `T0..Ti` (the types before `Ti`).

A type parameter `P` is **uncovered** if it doesn't appear inside any nominal type wrapper (e.g., bare `T` is uncovered, but `Vec<T>` covers `T`).

**Illegal (both foreign):**
```rust
// In crate C — Display is from std, Vec<T> is from std
impl Display for Vec<String> { ... }   // ERROR: orphan rule
```

**Legal (local trait):**
```rust
// In crate C — MySummary is local
impl MySummary for Vec<String> { ... }   // OK: trait is local
```

**Legal (local type):**
```rust
// In crate C — MyType is local
impl Display for MyType { ... }   // OK: T0 is local
```

**The "fundamental" exemption:** `Box<T>`, `&T`, and `&mut T` are marked `#[fundamental]`, meaning their type parameter is treated as uncovered for coherence purposes. `impl Trait for Box<LocalType>` is allowed.

### 4.2 Overlap Prohibition

Even when the orphan rule passes, two impls of the same trait for overlapping types are forbidden. In stable Rust, no two impls may apply to the same concrete type:
```rust
impl<T: Clone> Foo for T { ... }
impl Foo for String { ... }  // ERROR in stable Rust — String: Clone, so these overlap
```
(Specialization — Section 7 — is the nightly mechanism for allowing this.)

### 4.3 How rustc Enforces It

Coherence checking is in `rustc_trait_selection/src/traits/coherence.rs`. The key entry points are `overlapping_trait_impls()` and `overlap()`. The orphan check itself is now in `rustc_hir_analysis` (formerly `rustc_typeck`). The check constructs a fresh inference context and attempts to unify the two impl headers; if unification succeeds and there's no negative obligation proving one of them inapplicable, the impls are flagged as conflicting.

### 4.4 Implications for Our Mod Language

Direct mapping:

- **"You own the trait OR the type"** → an impl block in mod `A` may only implement a trait declared in mod `A`, or target a type declared in mod `A`. This prevents `patch_mod_A` and `patch_mod_B` from both writing `impl Patchable for vanilla::Country`.
- **Overlap prohibition** → if mod `A` already provides `impl FleetCapable for vanilla::Country`, mod `B` may not provide another `impl FleetCapable for vanilla::Country` — even if the implementations differ. They must be merged into one (which aligns with the "consolidate, don't choose" principle).
- **Blanket impls** → a mod can write `impl Patchable for T where T: HasOwner` as a blanket, but this may conflict with specific impls — use with care.

---

## 5. Associated Types vs Generic Parameters

### 5.1 The Core Trade-off

| Dimension | Associated Type (`type Item`) | Generic Parameter (`<T>`) |
|---|---|---|
| Impl count | One per type (one canonical) | Multiple (one per `T`) |
| Type annotation at call site | Not needed | Sometimes needed to disambiguate |
| Multiple "flavors" per type | No | Yes |
| Ergonomics | Cleaner | More flexible |
| Bound syntax | `where I: Iterator` | `where I: Iterator<Item=u32>` |

**Associated types** are for "this trait has exactly one answer per type." `Iterator` has one `Item` per struct. You cannot write `impl Iterator for Counter` twice with different items.

**Generic parameters** are for "this trait can be instantiated multiple ways per type." `From<T>` — a `String` can be `From<&str>` and `From<u8>` simultaneously.

### 5.2 Iterator as the Canonical Example

```rust
// With associated type — clean, one impl per type
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// With generic — verbose, multiple impls possible, callers must annotate
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
// Now you need: fn consume<T, I: Iterator<T>>(i: I) — extra T everywhere
// And Counter could impl Iterator<u32> AND Iterator<String> — almost never wanted
```

### 5.3 Default Type Parameters

Generics can have defaults: `trait Add<Rhs = Self>`. This lets `Point + Point` work with no annotation while still allowing `Millimeters + Meters`.

### 5.4 Decision Matrix for Our Mod Language

| Scenario | Recommendation |
|---|---|
| `trait Family` — each entity has one canonical member type | `type Member;` (associated) |
| `trait Patchable` — one canonical patch target per entity | `type Target;` (associated) |
| `trait HasModifier<R>` — an entity can carry modifiers of multiple resource types | `<R>` (generic) |
| `trait Converts<From, To>` — multiple conversion pairs | `<From, To>` (generic) |
| `trait SupportsComponent<C>` — ship can support multiple component types | `<C>` (generic) |

Rule of thumb: if you'd be surprised to see two impls of the trait for the same type, use an associated type. If multiple impls make sense (one per `T`), use a generic parameter.

---

## 6. Object Safety and Dynamic Dispatch

### 6.1 How dyn Trait Works

A `dyn Trait` value is a **fat pointer**: 16 bytes total.
- 8 bytes: pointer to the concrete data.
- 8 bytes: pointer to the **vtable** — a static table of function pointers built at compile time per `(ConcreteType, Trait)` pair.

Calling a method goes through the vtable: load function pointer, call indirectly. This is one indirection + one pointer load overhead vs. zero for monomorphization.

### 6.2 Object Safety Rules (dyn-Compatibility)

A trait is **dyn-compatible** (formerly called "object-safe") if and only if:

1. All supertraits are dyn-compatible.
2. `Sized` is NOT a supertrait bound.
3. No associated constants (associated constants cannot be dispatched via vtable).
4. No associated types with generic parameters (GATs are not dyn-compatible).
5. All methods must be dispatchable:
   - Receiver must be one of: `&Self`, `&mut Self`, `Box<Self>`, `Rc<Self>`, `Arc<Self>`, or `Pin<P>` wrapping one of these.
   - No generic type parameters on the method (because vtable slots are fixed at compile time; generics would require infinitely many slots).
   - No `where Self: Sized` clause.
   - No `async fn` or `-> impl Trait` return type.
6. No `Self` return types (other than in allowed receiver positions).

**Not dyn-compatible:**
```rust
trait NotDynCompat {
    fn generic_method<T>(&self, x: T);  // generic method
    fn clone_self(&self) -> Self;        // Self return
    const N: usize;                      // associated constant
}
```

**dyn-compatible:**
```rust
trait DynCompat {
    fn by_ref(&self) -> String;
    fn by_mut(&mut self);
    fn owned(self: Box<Self>);
}
```

Methods with `where Self: Sized` are excluded from the vtable but don't violate object safety — they're simply inaccessible on trait objects.

### 6.3 For Our Compile-Only Language

We do NOT want `dyn Trait` semantics — there is no runtime in a Clausewitz-compiled output. We should:

1. **Disallow `dyn` entirely** in the surface language.
2. **All dispatch is monomorphic** — every trait call site resolves to exactly one impl at compile time.
3. **Object-safety rules still inform design:** generic methods and `Self` return types are fine in our language because we never need a vtable. But we must be careful that our compiler's trait resolution doesn't silently produce ambiguity the way dynamic dispatch hides it.
4. **Practical implication:** we can use traits with associated constants freely, and methods can return `Self` — none of the object-safety restrictions apply to us.

---

## 7. Specialization

### 7.1 Current Status

Specialization (RFC 1210, tracking issue #31844) is **nightly-only** and has been for a decade. The minimal subset `min_specialization` is also unstable. It is used internally in `libstd` but is not exposed to stable Rust.

### 7.2 The Core Idea

Allow two overlapping impls where one is "more specific" than the other:
```rust
// Blanket: applies to all T
impl<T: Debug> Display for T { ... }

// Specialized: applies only to String — takes precedence
impl Display for String { ... }
```

The more specific impl (the one whose applicable type set is a strict subset of the other's) wins.

### 7.3 The Lattice Rule

The RFC describes two ordering strategies:
- **Chain rule (chosen):** impls must form a linear chain — each more specific than the previous. `I` specializes `J` means the types matching `I` are a strict subset of types matching `J`.
- **Lattice rule (deferred):** allowed partial overlaps if a third impl covers exactly the intersection. Deferred because lifetime-dependent overlaps create dispatch ambiguities at monomorphization time (lifetimes are erased, so no impl can be selected).

### 7.4 The `default` Keyword

To allow an associated item to be overridden by a more-specific impl, it must be marked `default`:
```rust
impl<T> Example for T {
    default type Output = Box<T>;   // specializable
    default fn method(&self) { ... }
}

impl Example for bool {
    type Output = bool;             // overrides the default
    fn method(&self) { ... }
}
```

Without `default`, items are implicitly final — the type-checker relies on them being non-specializable for soundness.

### 7.5 Soundness Hazard

The key unsoundness: the type checker might normalize `<T as Example>::Output` to `Box<T>` in the blanket impl, but monomorphization of the bool specialization produces `bool`. This creates a mismatch between what the type-checker proved and what code actually runs — allowing memory unsafety with safe code. `min_specialization` restricts specialization to avoid this (only allows specializing method bodies, never associated types).

### 7.6 Applicability to Our Language

For "specialized corvette inheriting from corvette":
- This is NOT the same as Rust specialization. We want **extension/inheritance**, not **competing impls for overlapping type sets**.
- The right model is **supertrait inheritance**: `trait AscendantCorvette: Corvette`. The more specific type gets all corvette methods plus additional ones. This is stable, well-understood, and precisely what we want for upward compatibility.
- Rust specialization is specifically about "same trait, overlapping types, pick the more specific impl." Our case is "more specific type, additional trait methods" — which is supertraits.
- **Recommendation:** do not port specialization. Use supertrait chains instead.

---

## 8. Negative Reasoning

### 8.1 `impl !Trait for Type`

Rust supports negative impls for **auto traits** (Send, Sync, Unpin, UnwindSafe, RefUnwindSafe):
```rust
impl !Send for *mut T {}
impl !Sync for *mut T {}
```

The compiler auto-implements auto traits for composite types if all fields implement them. Negative impls override this: `*mut T` is not `Send` even though `T: Send`, because raw pointers are explicitly opted out.

**Current limitation:** Negative impls are only supported for auto traits on stable Rust. There is no stable way to write `impl !MyTrait for MyType`.

### 8.2 The `negative_impls` Nightly Feature

Nightly allows:
```rust
#![feature(negative_impls)]

impl !Foo for MyType {}
```

This tells the trait solver "MyType definitively does NOT implement Foo." It participates in coherence: a blanket `impl<T: Foo> Bar for T` can be proven not to apply to `MyType`.

### 8.3 Auto Traits and Negative Reasoning

Auto trait resolution algorithm:
1. Check for explicit positive impl.
2. Check for explicit negative impl.
3. Recursively check all field types.
4. If all fields satisfy the trait, the composite type satisfies it.

### 8.4 Implications for Our Language

We want to express "this Clausewitz scope does NOT have an `owner` field." This is analogous to a negative trait bound: "this impl is only valid for types that do NOT implement `HasOwner`."

Options to model this:
- **Absence trait:** `trait NoOwner {}` — a marker trait implemented for all scopes that lack owner semantics. Then use `where T: NoOwner` as a positive bound.
- **Negative bounds:** if our language has a type-level notion of "this entity has no `owner` field," we can model it as a negative bound: `where T: !HasOwner`.
- **Recommendation:** Implement an **absence marker** system — an explicit set of marker traits like `trait Ownerless {}` — rather than negative impls, since negative impls are nightly-only and complex. Only introduce `!Trait` bounds if the language has a clear need for them at compile time.

---

## 9. Module System and Visibility

### 9.1 Module Declaration

```rust
mod math {            // inline module
    pub fn sin() {}
}

mod util;             // external — loads from util.rs or util/mod.rs
```

Modules are containers in the type namespace. Items in a module are private by default.

### 9.2 Visibility Levels

| Modifier | Meaning |
|---|---|
| (none) | Private — accessible only within this module and its descendants |
| `pub` | Fully public — accessible anywhere the module path is reachable |
| `pub(crate)` | Visible throughout the entire current crate |
| `pub(super)` | Visible to the parent module only |
| `pub(in path)` | Visible within the specified ancestor module path |
| `pub(self)` | Effectively private (same as no pub) |

**Trait items** are public by default when the trait is public. **Enum variants** in a public enum are public by default. Everything else starts private.

### 9.3 Re-exports

```rust
pub use crate::internal::api;   // re-export — callers can use api:: directly
```

Re-exports allow building a clean public API surface while keeping internal structure private.

### 9.4 Use Declarations

```rust
use std::collections::{HashMap, BTreeSet};     // multiple
use std::collections::HashMap as HM;            // rename
use std::collections::*;                        // glob
use foo::Zoo as _;                              // import for trait methods only
```

`use` does not affect what items exist — only what names are in scope in the current module.

### 9.5 Crate-Relative Paths

| Prefix | Meaning |
|---|---|
| `crate::` | Absolute path from crate root |
| `super::` | Parent module |
| `self::` | Current module |
| (no prefix) | Relative from current module |

### 9.6 Implications for Our Language

The module system maps directly to "which mod defined this type or trait":

- Each **Stellaris mod** = a **crate** in our language. It has a root namespace.
- Each **logical grouping** within a mod (e.g., `vanilla::countries`, `my_mod::ships`) = a **module**.
- `pub` items are exported from the mod for others to use (inherit, implement traits on, etc.).
- `pub(crate)` items are internal to the mod — other mods cannot name them.
- **Critical:** the orphan rule enforcement relies on the crate boundary. In our language, the same rule applies at the mod boundary: you may only `impl SomeTrait for SomeType` if your mod defined `SomeTrait` or `SomeType`.
- **Re-exports:** a mod can `pub use vanilla::Country` to bring a type into its own namespace, but this does NOT make `Country` a local type for orphan purposes. The orphan rule tracks the *definition site*, not whether something is re-exported.

---

## 10. Additional Key Mechanisms

### 10.1 The `#[derive]` Mechanism

`derive` invokes procedural macros at compile time that read the type definition and emit an `impl` block:

```rust
#[derive(Clone, Debug, PartialEq)]
struct Fleet {
    ships: Vec<Ship>,
    size: u32,
}
```

Generated (approximately):
```rust
impl Clone for Fleet where Ship: Clone {
    fn clone(&self) -> Self {
        Fleet { ships: self.ships.clone(), size: self.size.clone() }
    }
}
// + Debug, PartialEq similarly
```

Key properties:
- Derives infer their own where-clause bounds (each field's type must also impl the derived trait).
- Multiple derives on one `#[derive(...)]` line all run independently.
- Custom derives are user-defined procedural macros (a separate crate annotated with `#[proc_macro_derive]`).
- The `#[automatically_derived]` attribute is added to generated impls.

**For our language:** `#[derive(HasFleets)]` could auto-generate the boilerplate `impl HasFleets for EmpireType` including all the standard fleet-management Clausewitz trigger/effect bindings, with the author overriding only the non-default ones. This is a high-value feature to port.

### 10.2 Const Generics

Const generics parameterize items by **constant values** (not types):

```rust
struct Grid<const W: usize, const H: usize> {
    cells: [[bool; W]; H],
}

impl<const N: usize> InnerArray<N> {
    fn first(&self) -> i32 { self.0[0] }
}
```

Allowed const generic types (stable): `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `char`, `bool`.

Const expressions in arguments must be in braces: `f::<{1 + 2}>()`.

**For our language:** `trait Hierarchical<const MAX_DEPTH: usize>` is directly applicable. A "dynastic tree with max depth 5" becomes a type-level constant the compiler can check. However, const generics add significant compiler complexity; consider whether the validation benefit outweighs the cost for v1.

### 10.3 Higher-Ranked Trait Bounds (HRTBs)

HRTBs express bounds that must hold **for all lifetimes**:

```rust
where F: for<'a> Fn(&'a (u8, u16)) -> &'a u8
```

This says: `F` must implement `Fn` for any lifetime `'a`. Used primarily with closures and function pointers.

**For our language:** Lifetimes don't exist in Clausewitz, so HRTBs have no direct analog. Skip.

### 10.4 Blanket Impls and Generic Specialization

```rust
impl<T: Display> ToString for T { ... }    // blanket

impl ToString for String { ... }           // specific — stable: ERROR (conflict)
                                           // nightly with specialization: OK
```

In stable Rust, a concrete type impl and a blanket impl for the same trait on an overlapping type set are a coherence error. The only way to avoid this is to ensure the bounds are disjoint (which is hard to prove in general).

**For our language:** blanket impls are useful for "all entities with this marker trait get these scripted effects for free." But the overlap prohibition means we need to design marker traits carefully to avoid spurious conflicts. A `default impl` mechanism (even if simplified compared to full specialization) would be valuable.

---

## 11. Common Trait Pitfalls and Error Quality

### 11.1 Missing Trait Impl

```
error[E0277]: the trait bound `MyType: Display` is not satisfied
  --> src/main.rs:5:20
   |
5  |     println!("{}", my_value);
   |                    ^^^^^^^^ the trait `Display` is not implemented for `MyType`
```

Rust's error messages are excellent here — they tell you exactly which trait is missing, where it's needed, and often suggest how to fix it.

### 11.2 Ambiguous Method Dispatch

```
error[E0034]: multiple applicable items in scope
  --> src/main.rs:7:7
   |
7  |     a.fly();
   |       ^^^ multiple `fly` found
   |
note: candidate #1 is defined in an impl of the trait `Pilot` for the type `Human`
note: candidate #2 is defined in an impl of the trait `Wizard` for the type `Human`
```

Resolution: use `Pilot::fly(&a)` or `<Human as Pilot>::fly(&a)`.

### 11.3 Orphan Rule Violation

```
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
  --> src/main.rs:3:1
   |
3  | impl Display for Vec<String> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: impl doesn't use only types from inside the current crate
```

### 11.4 Object Safety Violation

```
error[E0038]: the trait `MyTrait` cannot be made into an object
  --> src/main.rs:8:12
   |
8  |     let t: Box<dyn MyTrait>;
   |            ^^^^^^^^^^^^^^^^ `MyTrait` cannot be made into an object
   = note: the trait cannot be made into an object because it contains the method `generic_method`, which has generic type parameters
```

### 11.5 Lessons for Our Language

- **Invest in error messages early.** The biggest productivity gain from Rust's trait system isn't the feature itself — it's the quality of errors when it goes wrong. Our compiler should produce error messages that say "patch mod `better_fleets` cannot implement `trait Patchable` for `vanilla::Country` because that trait/type pair is already owned by mod `base_patches`."
- **IDE support:** because all dispatch is static, a language server can always resolve which impl a method call goes to — no ambiguity. This is a major advantage over weakly-typed Clausewitz.
- **Suggest fixes:** when an author forgets a required method in an impl, suggest a scaffold with the method signature.

---

## 12. Concrete Recommendations for Our Language

### 12.1 What to Copy Directly

| Rust feature | Copy as-is | Notes |
|---|---|---|
| Trait declaration syntax (`trait Foo { fn x(&self); }`) | Yes | Core mechanism |
| Associated types (`type Item;`) | Yes | Ideal for one-per-type relationships |
| Associated constants (`const N: usize;`) | Yes | Config values |
| Default methods (with body in trait) | Yes | High value for boilerplate reduction |
| Supertraits (`trait Foo: Bar`) | Yes | Maps to inheritance/capability extension |
| Where clauses (`where T: Foo + Bar`) | Yes | Conditional methods essential |
| Orphan rule (trait OR type must be local) | Yes | Prevents patch conflicts |
| Overlap prohibition | Yes | Enforces "one impl per (type, trait) pair" |
| `#[derive(...)]` syntax | Yes | Must-have for ergonomics |
| `pub` / `pub(crate)` / `pub(super)` visibility | Yes | Mod boundary control |
| `use` / `mod` paths | Yes | Namespace organization |
| Sealed trait pattern (private supertrait) | Yes | For locked system traits |
| Fully qualified syntax `<T as Trait>::method()` | Yes | Disambiguation |
| Blanket impls (`impl<T: Foo> Bar for T`) | Yes | But with care re: conflicts |

### 12.2 What to Simplify

| Rust feature | Simplified version |
|---|---|
| Lifetime parameters (`'a`) | Omit entirely — no ownership/borrowing in Clausewitz |
| Higher-ranked trait bounds (`for<'a>`) | Omit — lifetime-free language |
| `dyn Trait` / vtables | Omit — all dispatch is compile-time |
| `unsafe trait` / `unsafe impl` | Omit or reserve for internal compiler use only |
| `Pin<P>` self types | Omit — async/futures not relevant |
| `Deref` coercion in method resolution | Simplify — Clausewitz entities don't have reference chains. Method resolution: inherent first, then trait, full stop. |
| `where T: 'a` lifetime bounds | Omit |
| `use Trait as _` (import for side effects) | Probably omit unless we have analogous use cases |

### 12.3 What to Add (Novel to Our Language)

| Our language feature | Description |
|---|---|
| `#[clausewitz]` decorator on impl methods | Maps a method to a specific Clausewitz construct (trigger, effect, modifier) |
| `#[maps_to = "scripted_trigger::has_fleet"]` | The Clausewitz binding declaration |
| Absence markers (`trait Ownerless {}`) | Explicit marker traits for "this scope lacks X" — instead of `!Trait` |
| `default impl` (simplified) | Allow a base impl that a more-specific impl can override, for the specialization use case |
| `mod vanilla` built-in | The vanilla namespace is a pseudo-crate with read-only types; impl-ing on vanilla types requires orphan exceptions |
| Compile-to-emit validation | Verify at compile time that all trait methods have valid `#[maps_to]` decorators that reference existing Clausewitz scripted triggers/effects |

### 12.4 The Orphan Rule as Patch Conflict Prevention

The most critical design decision: **make the orphan rule the primary mechanism for preventing two patch mods from conflicting.**

Rule stated for our language:
> An impl block `impl SomeTrait for SomeType` is only legal in mod M if either `SomeTrait` was declared in mod M, OR `SomeType` was declared in mod M.

This means:
- A patch mod can `impl Patchable for vanilla::Country` only if it declares the `Patchable` trait itself.
- Two patch mods cannot both claim `impl Patchable for vanilla::Country` — one will get a "trait not local" error if it doesn't own `Patchable`, or a "duplicate impl" error if it does.
- The resolution: if two mods want to patch `vanilla::Country`, they must produce a **merged impl** via the megapatch system — exactly one impl, consolidating both.

### 12.5 Associated Types for the Clausewitz Type Hierarchy

```
trait Entity {
    type Scope;     // which Clausewitz scope this lives in (country, planet, ship...)
    type Owner;     // what type "owns" instances of this entity
}

trait Patchable: Entity {
    type PatchTarget;  // the vanilla type being patched
    fn apply(&self);   // emits the Clausewitz modification
}

trait HasFleets: Entity where Self::Scope = CountryScope {
    type Fleet;
    const MAX_FLEETS: usize;
    fn fleet_capacity(&self) -> Self::Fleet;
}
```

### 12.6 `#[derive]` for Common Boilerplate

```
#[derive(HasFleets, HasArmies, HasLeaders)]
impl Empire { ... }
```

Each derive expands to an `impl HasFleets for Empire { ... }` with standard Clausewitz trigger/effect bindings. The author overrides only the non-standard methods. This alone would eliminate thousands of lines of hand-written Clausewitz.

---

## 13. References

### Rust Reference (Official Language Spec)

- Traits: https://doc.rust-lang.org/reference/items/traits.html
- Implementations: https://doc.rust-lang.org/reference/items/implementations.html
- Method Call Expressions (resolution algorithm): https://doc.rust-lang.org/reference/expressions/method-call-expr.html
- Trait and Lifetime Bounds: https://doc.rust-lang.org/reference/trait-bounds.html
- Trait Objects (dyn Trait): https://doc.rust-lang.org/reference/types/trait-object.html
- Associated Items: https://doc.rust-lang.org/reference/items/associated-items.html
- Generics: https://doc.rust-lang.org/reference/items/generics.html
- Visibility and Privacy: https://doc.rust-lang.org/reference/visibility-and-privacy.html
- Modules: https://doc.rust-lang.org/reference/items/modules.html
- Use Declarations: https://doc.rust-lang.org/reference/items/use-declarations.html
- Special Types and Traits (Send/Sync/negative): https://doc.rust-lang.org/reference/special-types-and-traits.html
- Derive Attribute: https://doc.rust-lang.org/reference/attributes/derive.html

### The Rust Programming Language (Book)

- Traits (Chapter 10-02): https://doc.rust-lang.org/book/ch10-02-traits.html
- Advanced Traits (Chapter 20-02): https://doc.rust-lang.org/book/ch20-02-advanced-traits.html

### The Rustonomicon (Unsafe Rust)

- Higher-Ranked Trait Bounds: https://doc.rust-lang.org/nomicon/hrtb.html

### RFCs

- RFC 0019 — Opt-in built-in traits (Send, Sync, negative impls): https://rust-lang.github.io/rfcs/0019-opt-in-builtin-traits.html
- RFC 1210 — Specialization: https://rust-lang.github.io/rfcs/1210-impl-specialization.html
  - Tracking issue: https://github.com/rust-lang/rust/issues/31844

### rustc Source Code

- Method resolution / probe: `rustc_hir_typeck/src/method/probe.rs`
- Coherence / orphan check: `rustc_trait_selection/src/traits/coherence.rs`
- Orphan rule implementation: `rustc_hir_analysis/src/coherence/orphan.rs` (formerly `rustc_typeck/check/coherence/orphan.rs`)

### Chalk (Rust's trait solver)

- What is Chalk: https://rust-lang.github.io/chalk/book/what_is_chalk.html

### Sealed Trait Pattern

- Sealed Traits (API Guidelines): https://rust-lang.github.io/api-guidelines/future-proofing.html
- Definitive Guide: https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/

### Coherence Discussion

- Coherence and crate-level where-clauses (Niko Matsakis): https://smallcultfollowing.com/babysteps/blog/2022/04/17/coherence-and-crate-level-where-clauses/
