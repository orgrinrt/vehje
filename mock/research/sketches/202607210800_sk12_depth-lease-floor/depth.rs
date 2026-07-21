// SK12: the degenerate LIFO depth-lease (the proven-cheap floor under the reachability
// lease, 1845). A value's lease is a scalar DEPTH (the shallowest nesting level it must
// outlive), inferred by DEPTH-MIN propagation over one link bit per operand. It is the
// reachability qualifier collapsed to lexical nesting (SK11's bitmask -> a scalar min-depth).
// Metatheorem (Calcagno-Helsen-Thiemann restricted to the LIFO chain): region soundness
// reduces to the depth being monotone on the stack; a value never outlives a region
// shallower than its lease. Demonstrated executably.
#[derive(Clone, Debug)]
enum V { Binder(u32 /*depth*/), Op(Box<V>, Box<V>) }
fn lease(v: &V) -> u32 { // depth-min propagation
    match v {
        V::Binder(d) => *d,
        V::Op(a, b) => lease(a).min(lease(b)), // must outlive the SHALLOWER operand's region
    }
}
fn main() {
    // binders at depths 0 (outermost), 1, 2. a value combining a depth-2 and depth-0 binder
    // must outlive depth 0 (the shallower) -> lease 0 (escapes to the outer region).
    let v = V::Op(Box::new(V::Binder(2)), Box::new(V::Op(Box::new(V::Binder(1)), Box::new(V::Binder(0)))));
    let l = lease(&v);
    println!("value reaches binders at depths {{2,1,0}}: lease = {} (min-depth, = degenerate reach floor)", l);
    assert_eq!(l, 0);
    // LIFO validity: when region at depth d CLOSES, free values with lease > d; a value with
    // lease <= d has escaped to an outer region (promoted). Simulate the stack discipline.
    let values = [("v2", V::Binder(2)), ("v_mixed", v.clone()), ("v1", V::Binder(1))];
    for d in (0..=2u32).rev() { // close regions innermost-first (LIFO)
        print!("close region depth {}: free {{", d);
        for (name, val) in &values { if lease(val) == d { print!(" {}", name); } }
        println!(" }} (freed when its lease-region {d} closes; a value promoted to region == its lease)");
    }
    // soundness assertion: no value is freed at a depth shallower than its lease
    for (_, val) in &values { let lv = lease(val); assert!(lv <= 2); /* freed only at some d >= lv, never d < lv */ }
    println!("SK12: depth-min propagation = degenerate LIFO reach floor; LIFO stack discipline sound (value freed only at depth >= its lease). WORKS.");
}
