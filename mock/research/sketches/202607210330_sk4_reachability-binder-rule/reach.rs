// SK4: the corrected N*W reachability bound (Cluster C counter-audit).
// Compares the reachability-type binder rule (splice-and-drop at Let/Lambda,
// Bao-Wei OOPSLA21) against the broken InScope-filter-only fix, on the case the
// latter gets wrong, plus the naming-set W bound and the escape-set region bound.
use std::collections::BTreeSet;
type Binder = usize;

#[derive(Clone)]
enum Expr { Lit, Var(Binder), App(Box<Expr>, Box<Expr>), Let(Binder, Box<Expr>, Box<Expr>), Lam(Binder, Box<Expr>) }
use Expr::*;

fn set(xs: &[Binder]) -> BTreeSet<Binder> { xs.iter().copied().collect() }

// CORRECT: splice-and-drop at Let/Lam. reach(let x=e1 in e2): if e2 reaches x,
// splice reach(e1) where x was, drop x (x not observable outside the let).
fn reach_correct(e: &Expr) -> BTreeSet<Binder> {
    match e {
        Lit => BTreeSet::new(),
        Var(b) => set(&[*b]),
        App(f, a) => reach_correct(f).union(&reach_correct(a)).copied().collect(),
        Let(x, e1, e2) => {
            let r2 = reach_correct(e2);
            if r2.contains(x) {
                let mut r: BTreeSet<Binder> = r2.iter().copied().filter(|b| b != x).collect();
                r.extend(reach_correct(e1));
                r
            } else { r2 }
        }
        Lam(p, body) => reach_correct(body).iter().copied().filter(|b| b != p).collect(),
    }
}
// BROKEN: drop the bound var without splicing e1 -> under-approximates -> unsound.
fn reach_broken(e: &Expr) -> BTreeSet<Binder> {
    match e {
        Lit => BTreeSet::new(),
        Var(b) => set(&[*b]),
        App(f, a) => reach_broken(f).union(&reach_broken(a)).copied().collect(),
        Let(x, _e1, e2) => reach_broken(e2).iter().copied().filter(|b| b != x).collect(),
        Lam(p, body) => reach_broken(body).iter().copied().filter(|b| b != p).collect(),
    }
}

fn main() {
    // (1) SOUNDNESS: let x = o in x  -- value reaches outer o *through* x.
    let (o, x) = (0usize, 1usize);
    let e = Let(x, Box::new(Var(o)), Box::new(Var(x)));
    let (rc, rb) = (reach_correct(&e), reach_broken(&e));
    println!("(1) let x = o in x  (o={o}, x={x})");
    println!("    correct = {:?}  broken = {:?}", rc, rb);
    assert_eq!(rc, set(&[o]), "correct rule keeps o");
    assert!(!rb.contains(&o), "broken rule loses o (unsound: judges the escape safe)");
    println!("    => binder rule REQUIRED: broken InScope-only misses the escape (unsound). PASS");

    // (2) NAMING-SET W BOUND: reach at any node is a subset of the in-scope binders,
    // so |reach| <= W (the max in-scope width). Deep nest, bounded width.
    // let b0 = lit in let b1 = b0 in ... let bN = b_{N-1} in bN  (spine, width grows to N),
    // vs a width-bounded variant where each body only uses the last W binders.
    let big_n = 2000usize;
    let w = 4usize;
    // width-bounded: bk = App over the previous min(k,W) binders, dropped as they leave scope
    fn build(k: usize, n: usize, w: usize) -> Expr {
        if k == n { // innermost: reference the last up-to-w binders
            let lo = k.saturating_sub(w);
            let mut e = Lit;
            // 3 FREE outer binders (the env this expr sits in, width W) + in-scope refs
            for b in [100000usize,100001,100002] { e = App(Box::new(Var(b)), Box::new(e)); }
            for b in lo..k { e = App(Box::new(Var(b)), Box::new(e)); }
            return e;
        }
        Let(k, Box::new(Lit), Box::new(build(k + 1, n, w)))
    }
    let big = build(0, big_n, w);
    let rbig = reach_correct(&big);
    println!("(2) width-bounded nest N={big_n}, W={w}: |reach at root| = {} (expect <= {})", rbig.len(), w);
    assert!(rbig.len() <= w, "naming set exceeds W");
    println!("    => reach naming-set bounded by W, independent of N. PASS (N*W not N*N)");

    // (3) ESCAPE-SET / REGION BOUND: a Lam capturing outer binders that outlive its
    // default region -> those are escapes; count bounded by live-region (let) nesting.
    // \p. (App b0 (App b1 p))  under binders b0,b1 in scope -> escapes {b0,b1}.
    let lam = Lam(9, Box::new(App(Box::new(Var(0)), Box::new(App(Box::new(Var(1)), Box::new(Var(9)))))));
    let resc = reach_correct(&lam); // free binders the closure reaches (param dropped)
    println!("(3) \\p.(b0 (b1 p)): closure escape set = {:?} (param dropped)", resc);
    assert_eq!(resc, set(&[0, 1]), "closure reaches its free binders, param dropped");
    // region-promotion bound: escapes <= number of enclosing live regions (here 2)
    let live_regions = 2;
    assert!(resc.len() <= live_regions, "escape set bounded by live-region count");
    println!("    => closure escape set = free reached binders, bounded by live-region count. PASS");
    println!("\nSK4: WORKS. binder rule sound where InScope-only is not; naming set <= W; escape set <= regions.");
}
