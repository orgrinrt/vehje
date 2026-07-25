//! The whole pipeline: Clause source to a value.
//!
//! Parse, lower to Core, serialize the residual, hand it to the real Zig
//! runtime, service the arithmetic family from a Rust host, and read the value
//! back. This is the test that says the language runs rather than that its
//! parts type-check.

use arvo::USize;
use clause_frontend::ast::{Arena as AstArena, AstRef, Node as AstNode};
use clause_frontend::lower::{op, Lower};
use clause_frontend::{lex, Token, TokenKind};
use core::ffi::c_void;
use hilavitkutin_str::{ArenaInterner, StringInterner};
use notko::{Maybe, Outcome};
use vehje_ir::{Arena as IrArena, Builder, Clause, Literal, Node, NodeRef, Span as IrSpan};
use vehje_runtime_abi::{serialize, Tier, ValueImage, ValueTag};
use vehje_runtime_driver::runtime::{Host, Operand, Runtime};

const N: usize = 4096;
const ARTIFACT: &[u8] = b"../../../runtime-zig/zig-out/lib/libvehje_runtime.dylib\0";

struct TestInterner {
    buf: core::cell::RefCell<([&'static str; 256], usize)>,
}

impl TestInterner {
    fn new() -> Self {
        Self { buf: core::cell::RefCell::new(([""; 256], 0)) }
    }
}

impl ArenaInterner for TestInterner {
    fn arena_intern(&self, s: &str) -> u32 {
        let mut b = self.buf.borrow_mut();
        let n = b.1;
        for i in 0..n {
            if b.0[i] == s {
                return i as u32;
            }
        }
        let leaked: &'static str = Box::leak(s.to_string().into_boxed_str());
        b.0[n] = leaked;
        b.1 = n + 1;
        n as u32
    }
    fn arena_resolve(&self, id: u32) -> &str {
        self.buf.borrow().0[id as usize]
    }
}

fn zero_sym() -> hilavitkutin_sym::Sym {
    hilavitkutin_sym::Sym::new(hilavitkutin_sym::SymKind::from_raw(0b111), arvo::Bits::from_raw(0))
}

/// The language's arithmetic family, serviced by the host.
///
/// The framework defines no family, so the language ships this. The opcode
/// rides as the first operand, which is why one family id serves every
/// operator.
extern "C" fn arith(
    _ud: *mut c_void,
    _family: u32,
    args: *const Operand,
    argc: usize,
    out: *mut Operand,
) -> i32 {
    // SAFETY: the runtime passes `argc` initialised operands and one writable
    // out-slot, both valid for the duration of the call.
    let a = unsafe { core::slice::from_raw_parts(args, argc) };
    if a.is_empty() {
        return -1;
    }
    let code = a[0].payload;
    let x = if a.len() > 1 { a[1].payload } else { 0 };
    let y = if a.len() > 2 { a[2].payload } else { 0 };
    let (tag, v) = match code {
        op::ADD => (2, x + y),
        op::SUB => (2, x - y),
        op::MUL => (2, x * y),
        op::DIV => (2, if y == 0 { return -1 } else { x / y }),
        op::REM => (2, if y == 0 { return -1 } else { x % y }),
        op::EQ => (1, i64::from(x == y)),
        op::NE => (1, i64::from(x != y)),
        op::LT => (1, i64::from(x < y)),
        op::GT => (1, i64::from(x > y)),
        op::LE => (1, i64::from(x <= y)),
        op::GE => (1, i64::from(x >= y)),
        op::AND => (1, i64::from(x != 0 && y != 0)),
        op::OR => (1, i64::from(x != 0 || y != 0)),
        op::NEG => (2, -x),
        op::NOT => (1, i64::from(x == 0)),
        _ => return -1,
    };
    // SAFETY: as above.
    unsafe { *out = Operand { tag, payload: v, bytes: core::ptr::null() } };
    0
}

struct Fx {
    an: [AstNode; N],
    asp: [clause_frontend::Span; N],
    ap: [AstRef; N],
    toks: [Token; N],
    inodes: [Node; N],
    ispans: [IrSpan; N],
    ipool: [NodeRef; N],
    iclauses: [Clause; 8],
    wire: [u8; 8192],
    out: [u8; 4096],
}

impl Fx {
    fn new() -> Box<Self> {
        Box::new(Self {
            an: [AstNode::Continue; N],
            asp: [Default::default(); N],
            ap: [AstRef(USize(0)); N],
            toks: [Token::new(TokenKind::Eof, Default::default()); N],
            inodes: [Node::Lit(Literal::Unit); N],
            ispans: [IrSpan::default(); N],
            ipool: [NodeRef::new(USize(0)); N],
            iclauses: [Clause {
                op: zero_sym(),
                resume: zero_sym(),
                arity: arvo::Uint::<8, arvo::strategy::Hot>::from_raw(0),
                body: NodeRef::new(USize(0)),
                span: IrSpan::default(),
            }; 8],
            wire: [0; 8192],
            out: [0; 4096],
        })
    }
}

/// Run a Clause program and return its value as an integer.
fn run_int(src: &str, entry: &str) -> i64 {
    let mut f = Fx::new();

    let n = match lex(src, &mut f.toks) {
        Outcome::Ok(n) => n.0,
        Outcome::Err(e) => panic!("lex: {e:?}"),
    };
    let ast_arena = AstArena::new(&mut f.an, &mut f.asp, &mut f.ap);
    let mut p = clause_frontend::Parser::new(&f.toks[..n], src, ast_arena);
    let root = match p.parse_file() {
        Outcome::Ok(r) => r,
        Outcome::Err(e) => panic!("parse: {e:?}"),
    };
    let ast = p.into_arena();

    let mut interner = StringInterner::new(TestInterner::new());
    let ir = IrArena::new(&mut f.inodes, &mut f.ispans, &mut f.ipool, &mut f.iclauses);
    let mut b = Builder::new(ir);
    let core_root = {
        let mut lo = Lower { ast: &ast, src, b: &mut b, interner: &mut interner };
        match lo.file(root, entry) {
            Outcome::Ok(r) => r,
            Outcome::Err(e) => panic!("lower: {e:?}"),
        }
    };
    let ir = b.into_arena();

    let len = match serialize(&ir, &interner, core_root, Tier::Arena, &mut f.wire) {
        Maybe::Is(n) => n.0,
        Maybe::Isnt => panic!("the wire buffer is too small"),
    };

    let lib = match vehje_runtime_driver::runtime::dynamic::load(ARTIFACT) {
        Outcome::Ok(l) => l,
        Outcome::Err(_) => panic!("build the runtime first: cd mock/runtime-zig && zig build"),
    };
    let entries = match vehje_runtime_driver::runtime::dynamic::entries(&lib) {
        Outcome::Ok(e) => e,
        Outcome::Err(_) => panic!("the artifact should export the three entries"),
    };

    let host = Host { call: arith, userdata: core::ptr::null_mut() };
    let wire = f.wire[..len].to_vec();
    let written = match Runtime::new(entries).execute_with_host(&wire, &mut f.out, &host) {
        Outcome::Ok(n) => n.0,
        Outcome::Err(e) => panic!("execute: {e:?}"),
    };

    let img = match ValueImage::parse(&f.out[..written]) {
        Maybe::Is(i) => i,
        Maybe::Isnt => panic!("the committed bytes should parse"),
    };
    let node = match img.try_node(img.root()) {
        Maybe::Is(n) => n,
        Maybe::Isnt => panic!("the root should read"),
    };
    assert!(matches!(node.tag, ValueTag::Int | ValueTag::Bool), "expected a scalar, got {:?}", node.tag);
    match img.as_int(node) {
        Maybe::Is(v) => v.to_raw(),
        Maybe::Isnt => match img.as_bool(node) {
            Maybe::Is(b) => i64::from(b.0),
            Maybe::Isnt => panic!("the value should read as a scalar"),
        },
    }
}

#[test]
#[ignore = "needs the built artifact: cd mock/runtime-zig && zig build"]
fn a_clause_program_computes() {
    assert_eq!(run_int("fn main() { 1 + 2 * 3 }", "main"), 7);
}

#[test]
#[ignore = "needs the built artifact: cd mock/runtime-zig && zig build"]
fn bindings_and_a_conditional_run() {
    let src = "
        fn main() {
            let x = 10;
            let y = 4;
            if x > y { x - y } else { y - x }
        }
    ";
    assert_eq!(run_int(src, "main"), 6);
}

#[test]
#[ignore = "needs the built artifact: cd mock/runtime-zig && zig build"]
fn a_function_calls_another_function() {
    let src = "
        fn double(n) { n * 2 }
        fn main() { double(21) }
    ";
    assert_eq!(run_int(src, "main"), 42);
}

#[test]
#[ignore = "needs the built artifact: cd mock/runtime-zig && zig build"]
fn recursion_terminates_because_a_fn_binds_recursively() {
    // the payoff of lowering `fn` to a RECURSIVE binding: the function's own
    // name is in scope for its body
    let src = "
        fn fact(n) { if n <= 1 { 1 } else { n * fact(n - 1) } }
        fn main() { fact(5) }
    ";
    assert_eq!(run_int(src, "main"), 120);
}

#[test]
#[ignore = "catalogue: `Let { rec }` binds ONE name in scope for its own value, not a GROUP, so with bindings nested inside out the second function is not in scope for the first. The census names the fix: mutual recursion needs a recursive binding group, which is the same mechanism applied to a set rather than to one binder. Tracked #51"]
fn mutual_recursion_needs_a_recursive_binding_group() {
    let src = "
        fn is_even(n) { if n == 0 { true } else { is_odd(n - 1) } }
        fn is_odd(n) { if n == 0 { false } else { is_even(n - 1) } }
        fn main() { is_even(10) }
    ";
    assert_eq!(run_int(src, "main"), 1);
}

#[test]
#[ignore = "needs the built artifact: cd mock/runtime-zig && zig build"]
fn a_closure_captures_where_it_was_written() {
    let src = "
        fn adder(n) { |x| x + n }
        fn main() { adder(3)(4) }
    ";
    assert_eq!(run_int(src, "main"), 7);
}

#[test]
#[ignore = "needs the built artifact: cd mock/runtime-zig && zig build"]
fn declaration_only_items_do_not_disturb_the_program() {
    let src = "
        struct Point { x: u32, y: u32 }
        trait Shape { type Unit; fn area(self) -> u32; }
        use std::thing;
        fn main() { 2 + 2 }
    ";
    assert_eq!(run_int(src, "main"), 4);
}
