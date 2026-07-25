//! Lowering items.
//!
//! A file is a chain of bindings ending in the entry expression, which is what
//! makes the whole program one Core term. A `fn` is a RECURSIVE binding of a
//! curried lambda, because a function's own name must be in scope for its body
//! or it cannot call itself.
//!
//! Most items erase entirely rather than lowering: a `struct` declares a shape
//! the checker uses and the Core never sees, a `trait` and its impls
//! monomorphise away before emission, and `use` and `mod` are resolved names.
//! Erasing is not skipping, and the difference is that an item this pass does
//! not recognise is a named refusal rather than a silent omission.

use hilavitkutin_str::ArenaInterner;
use notko::{Maybe, Outcome};
use vehje_ir::NodeRef;

use super::{ok, Lower, LowerError};
use crate::ast::{AstRef, Node};

impl<'ast, 'b, 'ir, 'i, A: ArenaInterner> Lower<'ast, 'b, 'ir, 'i, A> {
    /// Lower a whole file to one Core term.
    ///
    /// `entry` names the function to call; the program's value is that call.
    pub fn file(&mut self, root: AstRef, entry: &str) -> Outcome<NodeRef, LowerError> {
        let Node::File { items, .. } = self.node(root) else {
            return Outcome::Err(LowerError::Unsupported { what: "not a file" });
        };
        let list = self.ast.list(items);

        // the program's value: call the entry function with unit, since Core
        // application always passes something
        let entry_sym = self.fresh(entry)?;
        let f = ok(self.b.var(entry_sym, vehje_ir::Span::default()))?;
        let u = self.unit()?;
        let mut acc = ok(self.b.apply(f, &[u], vehje_ir::Span::default()))?;

        // built inside out, so the first item is outermost and every later item
        // can see it. Mutual recursion between top-level functions follows from
        // each binding being recursive.
        for it in list.iter().rev() {
            acc = self.item(*it, acc)?;
        }
        Outcome::Ok(acc)
    }

    /// Lower one item, wrapping `rest` in whatever binding it introduces.
    fn item(&mut self, at: AstRef, rest: NodeRef) -> Outcome<NodeRef, LowerError> {
        match self.node(at) {
            Node::ItemFn { name, params, body, .. } => {
                let Maybe::Is(body) = body else {
                    // a signature-only fn declares and does not define; it binds
                    // nothing, which is correct rather than an omission
                    return Outcome::Ok(rest);
                };
                let inner = self.expr(body)?;
                let lam = self.curry(params, inner)?;
                let s = self.sym(name)?;
                self.bind_rec(s, lam, rest)
            }

            Node::ItemConst { name, value: Maybe::Is(value), .. } => {
                let v = self.expr(value)?;
                let s = self.sym(name)?;
                self.bind(s, v, rest)
            }

            Node::ItemStatic { name, value, .. } => {
                let v = self.expr(value)?;
                let s = self.sym(name)?;
                self.bind(s, v, rest)
            }

            // items that erase: they inform the checker and the Core never sees
            // them. Listed explicitly rather than caught by a wildcard, so that
            // adding an item kind forces a decision here rather than silently
            // erasing it.
            Node::ItemStruct { .. }
            | Node::ItemEnum { .. }
            | Node::ItemTrait { .. }
            | Node::ItemTypeAlias { .. }
            | Node::ItemUse { .. }
            | Node::ItemMod { .. }
            | Node::ItemConst { .. }
            | Node::Attr { .. } => Outcome::Ok(rest),

            // an impl contributes its methods as ordinary bindings; without a
            // resolve pass the receiver type is not part of the name yet
            Node::ItemImpl { items, .. } => {
                let list = self.ast.list(items);
                let mut acc = rest;
                for m in list.iter().rev() {
                    acc = self.item(*m, acc)?;
                }
                Outcome::Ok(acc)
            }

            Node::ItemExpect { .. } => Outcome::Ok(rest),
            Node::ItemActual { inner } => self.item(inner, rest),
            Node::ItemExtern { .. } => Outcome::Ok(rest),

            Node::ItemMacro { .. } => {
                Outcome::Err(LowerError::Unsupported { what: "macro declaration" })
            }
            Node::ItemEvent { .. } => Outcome::Err(LowerError::Unsupported { what: "event" }),
            Node::ItemMacroCall { .. } => {
                Outcome::Err(LowerError::Unsupported { what: "item macro invocation" })
            }
            _ => Outcome::Err(LowerError::Unsupported { what: "item" }),
        }
    }
}
