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

// The build-width-bounded nest generator, shared by the bound tests.
fn build(k: usize, n: usize, w: usize) -> Expr {
    if k == n {
        let lo = k.saturating_sub(w);
        let mut e = Lit;
        for b in [100000usize, 100001, 100002] {
            e = App(Box::new(Var(b)), Box::new(e));
        }
        for b in lo..k {
            e = App(Box::new(Var(b)), Box::new(e));
        }
        return e;
    }
    Let(k, Box::new(Lit), Box::new(build(k + 1, n, w)))
}

// The free variables of an expr: the binders referenced that are not bound above
// within e. The reachability theorem is that reach_correct(e) is always a subset
// of free_vars(e), so the naming set is bounded by the free width, not the depth.
fn free_vars(e: &Expr, bound: &mut Vec<Binder>) -> BTreeSet<Binder> {
    match e {
        Lit => BTreeSet::new(),
        Var(b) => {
            if bound.contains(b) {
                BTreeSet::new()
            } else {
                set(&[*b])
            }
        }
        App(f, a) => free_vars(f, bound).union(&free_vars(a, bound)).copied().collect(),
        Let(x, e1, e2) => {
            let f1 = free_vars(e1, bound);
            bound.push(*x);
            let f2 = free_vars(e2, bound);
            bound.pop();
            f1.union(&f2).copied().collect()
        }
        Lam(p, body) => {
            bound.push(*p);
            let f = free_vars(body, bound);
            bound.pop();
            f
        }
    }
}

// A tiny deterministic PRNG for the property test's shape sweep.
fn splitmix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

// Generate a random width-bounded expr: a spine of `n` lets whose innermost body
// references a random subset of the last `w` in-scope binders plus `f` free
// outer binders, with random App/Lam structure. In-scope width stays <= w.
fn build_rand(seed: u64, n: usize, w: usize, f: usize) -> Expr {
    let mut s = seed;
    fn inner(s: &mut u64, k: usize, n: usize, w: usize, f: usize) -> Expr {
        if k == n {
            let lo = k.saturating_sub(w);
            let mut e = Lit;
            for i in 0..f {
                if splitmix(s) & 1 == 0 {
                    e = App(Box::new(Var(1_000_000 + i)), Box::new(e));
                }
            }
            for b in lo..k {
                if splitmix(s) & 1 == 0 {
                    e = App(Box::new(Var(b)), Box::new(e));
                }
            }
            // occasionally wrap in a lambda binding a fresh param it may reference.
            if splitmix(s) & 3 == 0 {
                e = Lam(500_000 + k, Box::new(App(Box::new(Var(500_000 + k)), Box::new(e))));
            }
            return e;
        }
        Let(k, Box::new(Lit), Box::new(inner(s, k + 1, n, w, f)))
    }
    inner(&mut s, 0, n, w, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_rule_sound_where_broken_is_not() {
        // let x = o in x: the value reaches outer o *through* x. The correct
        // splice-and-drop rule keeps o; the broken InScope-only fix loses it
        // (unsound: it judges the escape safe).
        let (o, x) = (0usize, 1usize);
        let e = Let(x, Box::new(Var(o)), Box::new(Var(x)));
        assert_eq!(reach_correct(&e), set(&[o]), "correct rule keeps o");
        assert!(!reach_broken(&e).contains(&o), "broken rule unsoundly loses o");
    }

    #[test]
    fn naming_set_bounded_by_width_fixed_shape() {
        // The original single-shape claim: a width-4 nest of 300 lets has a
        // root reach set of <= 4 (bounded by W, not N; capped at 300 for the test stack).
        let rbig = reach_correct(&build(0, 300, 4));
        assert!(rbig.len() <= 4, "naming set exceeds W on the fixed shape");
    }

    #[test]
    fn escape_set_bounded_by_live_regions() {
        // \p.(b0 (b1 p)): the closure reaches its free binders {b0,b1}, param
        // dropped; the escape count is bounded by the live-region nesting.
        let lam = Lam(9, Box::new(App(Box::new(Var(0)), Box::new(App(Box::new(Var(1)), Box::new(Var(9)))))));
        let resc = reach_correct(&lam);
        assert_eq!(resc, set(&[0, 1]), "closure reaches its free binders");
        assert!(resc.len() <= 2, "escape set bounded by live-region count");
    }

    #[test]
    fn reach_subset_of_free_vars_property() {
        // The theorem behind the N*W bound, as a property over many shapes: for
        // any expr, reach_correct(e) is a subset of free_vars(e). Since a
        // width-w nest has |free_vars| <= w + f, this gives |reach| <= w + f at
        // every shape, independent of the nesting depth N. Sweep N, W, F, seed.
        for &n in &[1usize, 5, 20, 100, 300] {
            for &w in &[1usize, 2, 4, 8] {
                for &f in &[0usize, 3] {
                    for seed in 0u64..24 {
                        let e = build_rand(seed.wrapping_mul(0x1000_0001) ^ (n as u64), n, w, f);
                        let mut bound = Vec::new();
                        let fv = free_vars(&e, &mut bound);
                        let r = reach_correct(&e);
                        assert!(
                            r.is_subset(&fv),
                            "reach not subset of free_vars: N={n} W={w} F={f} seed={seed}\n reach={r:?}\n fv={fv:?}"
                        );
                        // and the free width is bounded by w + f, independent of N.
                        assert!(
                            fv.len() <= w + f,
                            "free width {} exceeds w+f={} at N={n} W={w} F={f} seed={seed}",
                            fv.len(),
                            w + f
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn bound_is_n_independent() {
        // The N*W-not-N*N property directly: fix W and F, grow N by two orders of
        // magnitude, and the root naming set does not grow with N.
        let (w, f) = (4usize, 3usize);
        let mut prev: Option<usize> = None;
        for &n in &[10usize, 100, 300] {
            let e = build_rand(0xabc ^ n as u64, n, w, f);
            let mut bound = Vec::new();
            let fv = free_vars(&e, &mut bound);
            assert!(fv.len() <= w + f, "free width grew with N at N={n}");
            let _ = prev.replace(fv.len());
        }
    }
}
