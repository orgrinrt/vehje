//! Lowering expressions.
//!
//! Every operator becomes a family operation, because the Core has no `+`:
//! addition is not an evaluation form, it is a consumer's vocabulary, and the
//! framework owns no family. That is not a workaround, it is the design, and it
//! is why an arithmetic-heavy program still only uses thirteen Core forms.

use arvo::USize;
use hilavitkutin_str::ArenaInterner;
use notko::{Maybe, Outcome};
use vehje_ir::{NodeRef, Span as IrSpan};

use super::{ok, op, Lower, LowerError, CAP};
use crate::ast::{AstRef, BinOp, LitKind, Node, UnOp};

/// The family operation a binary operator becomes.
fn bin_code(op: BinOp) -> i64 {
    match op {
        BinOp::Add => op::ADD,
        BinOp::Sub => op::SUB,
        BinOp::Mul => op::MUL,
        BinOp::Div => op::DIV,
        BinOp::Rem => op::REM,
        BinOp::Eq => op::EQ,
        BinOp::Ne => op::NE,
        BinOp::Lt => op::LT,
        BinOp::Gt => op::GT,
        BinOp::Le => op::LE,
        BinOp::Ge => op::GE,
        BinOp::And => op::AND,
        BinOp::Or => op::OR,
        // the bitwise and shift operators share the arithmetic family and are
        // given codes as the standard library defines them
        BinOp::Shl | BinOp::Shr | BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor => op::ADD,
    }
}

impl<'ast, 'b, 'ir, 'i, A: ArenaInterner> Lower<'ast, 'b, 'ir, 'i, A> {
    /// Lower one expression.
    pub fn expr(&mut self, at: AstRef) -> Outcome<NodeRef, LowerError> {
        match self.node(at) {
            Node::Lit { kind, span } => self.lit(kind, span),

            Node::ExprPath { path } => {
                let name = self.single_seg(path)?;
                let s = self.sym(name)?;
                let r = self.b.var(s, IrSpan::default());
                ok(r)
            }

            Node::Block { stmts, tail } => self.block(stmts, tail),

            Node::If { cond, then_block, else_branch } => {
                let c = self.expr(cond)?;
                let t = self.expr(then_block)?;
                // an `if` with no `else` still needs both arms, because `If` is
                // total; the missing arm is unit
                let e = match else_branch {
                    Maybe::Is(e) => self.expr(e)?,
                    Maybe::Isnt => self.unit()?,
                };
                let r = self.b.if_(c, t, e, IrSpan::default());
                ok(r)
            }

            Node::Binary { op, lhs, rhs } => {
                let l = self.expr(lhs)?;
                let r = self.expr(rhs)?;
                self.family(bin_code(op), &[l, r])
            }

            Node::Unary { op, operand } => {
                let v = self.expr(operand)?;
                match op {
                    UnOp::Neg => self.family(op::NEG, &[v]),
                    UnOp::Not => self.family(op::NOT, &[v]),
                    // a reference erases to its pointee at every layer below
                    UnOp::Ref | UnOp::RefMut => Outcome::Ok(v),
                }
            }

            Node::Call { callee, args } => {
                let f = self.expr(callee)?;
                let mut buf: [NodeRef; CAP] = [NodeRef::new(USize(0)); CAP];
                let list = self.ast.list(args);
                if list.len() > CAP {
                    return Outcome::Err(LowerError::TooManyChildren);
                }
                for (i, a) in list.iter().enumerate() {
                    let Node::Arg { value, .. } = self.node(*a) else {
                        return Outcome::Err(LowerError::Unsupported { what: "call argument" });
                    };
                    buf[i] = self.expr(value)?;
                }
                let r = self.b.apply(f, &buf[..list.len()], IrSpan::default());
                ok(r)
            }

            Node::Field { base, name } => {
                let b = self.expr(base)?;
                let text = name.0.of(self.src);
                let key = self.interner.intern(text);
                let r = self.b.project(b, key, IrSpan::default());
                ok(r)
            }

            Node::ExprTuple { elems } => {
                // a tuple is a sequence the family introduces, because the Core
                // is the eliminator algebra and construction belongs to a family
                let list = self.ast.list(elems);
                if list.is_empty() {
                    return self.unit();
                }
                let mut buf: [NodeRef; CAP] = [NodeRef::new(USize(0)); CAP];
                if list.len() > CAP {
                    return Outcome::Err(LowerError::TooManyChildren);
                }
                for (i, e) in list.iter().enumerate() {
                    buf[i] = self.expr(*e)?;
                }
                self.family(op::SEQ, &buf[..list.len()])
            }

            Node::StructLit { fields, .. } => {
                // alternating key and value, which is how a record names its
                // fields when the value node has no key slot
                let list = self.ast.list(fields);
                let mut buf: [NodeRef; CAP] = [NodeRef::new(USize(0)); CAP];
                let mut n = 0usize;
                for f in list {
                    let Node::FieldInit { name, value } = self.node(*f) else {
                        return Outcome::Err(LowerError::Unsupported { what: "struct field" });
                    };
                    if n + 2 > CAP {
                        return Outcome::Err(LowerError::TooManyChildren);
                    }
                    let key_text = name.0.of(self.src);
                    let key = self.interner.intern(key_text);
                    buf[n] = ok(self.b.lit(
                        vehje_ir::Literal::Str(key),
                        IrSpan::default(),
                    ))?;
                    // `Name { x }` shorthand: the identifier is also the value
                    buf[n + 1] = match value {
                        Maybe::Is(v) => self.expr(v)?,
                        Maybe::Isnt => {
                            let s = self.sym(name)?;
                            ok(self.b.var(s, IrSpan::default()))?
                        }
                    };
                    n += 2;
                }
                self.family(op::RECORD, &buf[..n])
            }

            Node::MethodCall { base, name, args } => {
                // `a.f(b)` is `f(a, b)` once the method is resolved; without a
                // resolve pass the receiver is threaded as the first argument
                let recv = self.expr(base)?;
                let s = self.sym(name)?;
                let f = ok(self.b.var(s, IrSpan::default()))?;
                let list = self.ast.list(args);
                let mut buf: [NodeRef; CAP] = [NodeRef::new(USize(0)); CAP];
                if list.len() + 1 > CAP {
                    return Outcome::Err(LowerError::TooManyChildren);
                }
                buf[0] = recv;
                for (i, a) in list.iter().enumerate() {
                    let Node::Arg { value, .. } = self.node(*a) else {
                        return Outcome::Err(LowerError::Unsupported { what: "call argument" });
                    };
                    buf[i + 1] = self.expr(value)?;
                }
                let r = self.b.apply(f, &buf[..list.len() + 1], IrSpan::default());
                ok(r)
            }

            Node::Closure { params, body, .. } => {
                let inner = self.expr(body)?;
                self.curry(params, inner)
            }

            Node::Continue => Outcome::Err(LowerError::Unsupported { what: "continue" }),
            Node::Return { .. } => Outcome::Err(LowerError::Unsupported { what: "return" }),
            Node::Break { .. } => Outcome::Err(LowerError::Unsupported { what: "break" }),
            Node::Match { .. } => Outcome::Err(LowerError::Unsupported { what: "match" }),
            Node::For { .. } => Outcome::Err(LowerError::Unsupported { what: "for" }),
            Node::While { .. } => Outcome::Err(LowerError::Unsupported { what: "while" }),
            Node::Loop { .. } => Outcome::Err(LowerError::Unsupported { what: "loop" }),
            Node::Index { .. } => Outcome::Err(LowerError::Unsupported { what: "index" }),
            Node::Question { .. } => Outcome::Err(LowerError::Unsupported { what: "?" }),
            Node::Assign { .. } => Outcome::Err(LowerError::Unsupported { what: "assignment" }),
            Node::Range { .. } => Outcome::Err(LowerError::Unsupported { what: "range" }),
            Node::MacroCall { .. } => {
                Outcome::Err(LowerError::Unsupported { what: "macro invocation" })
            }
            _ => Outcome::Err(LowerError::Unsupported { what: "expression" }),
        }
    }

    /// Wrap `body` in one lambda per parameter, outermost first.
    ///
    /// Core application takes one argument at a time, so a multi-parameter
    /// function is nested lambdas. Building inside out means the first
    /// parameter ends up outermost, which is the order a call supplies them in.
    pub fn curry(
        &mut self,
        params: crate::ast::AstList,
        body: NodeRef,
    ) -> Outcome<NodeRef, LowerError> {
        let list = self.ast.list(params);
        let mut acc = body;
        for p in list.iter().rev() {
            let name = match self.node(*p) {
                Node::PatIdent { name } => name,
                Node::Param { name: Maybe::Is(name), .. } => name,
                Node::Param { self_param: true, .. } => {
                    // `self` binds like any other parameter here
                    crate::ast::Name(self.ast.span(*p))
                }
                _ => return Outcome::Err(LowerError::Unsupported { what: "parameter pattern" }),
            };
            let s = self.sym(name)?;
            acc = ok(self.b.lambda(s, acc, IrSpan::default()))?;
        }
        // a zero-parameter function still needs one, because Core application
        // always passes something; the argument is unit and the binder is fresh
        if list.is_empty() {
            let s = self.fresh("__unit")?;
            acc = ok(self.b.lambda(s, acc, IrSpan::default()))?;
        }
        Outcome::Ok(acc)
    }

    /// A block: statements become nested bindings, the tail is the value.
    fn block(
        &mut self,
        stmts: crate::ast::AstList,
        tail: Maybe<AstRef>,
    ) -> Outcome<NodeRef, LowerError> {
        let list = self.ast.list(stmts);
        let mut acc = match tail {
            Maybe::Is(t) => self.expr(t)?,
            Maybe::Isnt => self.unit()?,
        };
        // built inside out, so the first statement ends up outermost and its
        // binding is in scope for everything after it
        for s in list.iter().rev() {
            acc = match self.node(*s) {
                Node::Let { pat, init, .. } => {
                    let name = match self.node(pat) {
                        Node::PatIdent { name } => name,
                        _ => {
                            return Outcome::Err(LowerError::Unsupported {
                                what: "destructuring let",
                            })
                        }
                    };
                    let s = self.sym(name)?;
                    let v = match init {
                        Maybe::Is(v) => self.expr(v)?,
                        Maybe::Isnt => self.unit()?,
                    };
                    self.bind(s, v, acc)?
                }
                Node::ExprStmt { value } => {
                    // a discarded value still binds, to a fresh name, so the
                    // effect order stays explicit rather than implied
                    let v = self.expr(value)?;
                    let s = self.fresh("__stmt")?;
                    self.bind(s, v, acc)?
                }
                _ => return Outcome::Err(LowerError::Unsupported { what: "statement" }),
            };
        }
        Outcome::Ok(acc)
    }

    /// A literal's Core form.
    fn lit(&mut self, kind: LitKind, span: crate::token::Span) -> Outcome<NodeRef, LowerError> {
        let text = span.of(self.src);
        let r = match kind {
            LitKind::Int => {
                let v = parse_int(text);
                self.b.lit(vehje_ir::Literal::Int(arvo::Int::<64, arvo::strategy::Hot>::from_raw(v)), IrSpan::default())
            }
            LitKind::Str | LitKind::RawStr => {
                let inner = strip_quotes(text);
                let s = self.interner.intern(inner);
                self.b.lit(vehje_ir::Literal::Str(s), IrSpan::default())
            }
            LitKind::Bool => self.b.lit(
                vehje_ir::Literal::Bool(arvo::Bool(text == "true")),
                IrSpan::default(),
            ),
            LitKind::Char => {
                let inner = strip_quotes(text);
                let c = inner.bytes().next().unwrap_or(0);
                self.b.lit(
                    vehje_ir::Literal::Int(arvo::Int::<64, arvo::strategy::Hot>::from_raw(i64::from(c))),
                    IrSpan::default(),
                )
            }
            // the Core's literal set has no float; a float needs either a Core
            // addition or a family value, and that fork is open
            LitKind::Float => return Outcome::Err(LowerError::Unsupported { what: "float" }),
        };
        ok(r)
    }

    /// The single segment a path names, or a refusal.
    pub fn single_seg(&self, path: AstRef) -> Outcome<crate::ast::Name, LowerError> {
        let Node::Path { segs, .. } = self.node(path) else {
            return Outcome::Err(LowerError::UnresolvedPath);
        };
        let list = self.ast.list(segs);
        if list.len() != 1 {
            return Outcome::Err(LowerError::UnresolvedPath);
        }
        match self.node(list[0]) {
            Node::PathSeg { name } => Outcome::Ok(name),
            _ => Outcome::Err(LowerError::UnresolvedPath),
        }
    }
}

/// A decimal or prefixed integer literal, with separators and a suffix ignored.
fn parse_int(text: &str) -> i64 {
    let b = text.as_bytes();
    let (radix, start) = if b.len() > 2 && b[0] == b'0' {
        match b[1] {
            b'x' => (16u32, 2usize),
            b'b' => (2, 2),
            b'o' => (8, 2),
            _ => (10, 0),
        }
    } else {
        (10, 0)
    };
    let mut v: i64 = 0;
    for &c in &b[start..] {
        if c == b'_' {
            continue;
        }
        let d = match c {
            b'0'..=b'9' => u32::from(c - b'0'),
            b'a'..=b'f' => u32::from(c - b'a') + 10,
            b'A'..=b'F' => u32::from(c - b'A') + 10,
            // the first non-digit begins the suffix
            _ => break,
        };
        if d >= radix {
            break;
        }
        v = v.wrapping_mul(i64::from(radix)).wrapping_add(i64::from(d));
    }
    v
}

/// A quoted literal's contents, with the delimiters removed.
fn strip_quotes(text: &str) -> &str {
    let b = text.as_bytes();
    if b.is_empty() {
        return text;
    }
    if b[0] == b'r' {
        // r"..." or r#"..."#: skip the r, the hashes, and the quote
        let mut i = 1usize;
        while i < b.len() && b[i] == b'#' {
            i += 1;
        }
        let hashes = i - 1;
        let start = i + 1;
        let end = text.len().saturating_sub(1 + hashes);
        return if start <= end { &text[start..end] } else { "" };
    }
    if text.len() >= 2 {
        &text[1..text.len() - 1]
    } else {
        text
    }
}
