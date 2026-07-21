// SP4 (+SK8): the four-axis graded (co)modal judgment. Grades: binding-time (a
// knowledge-source join-semilattice mask, SK9), effect (a graded monad, op mask),
// lease (a coeffect, reachability binder mask), assurance (derived from binding-time,
// SK8). The typing rules compose the grades bottom-up; the axis interactions fall out
// of the composition (join for bt, union for eff, combine for lease). This checks the
// algebra composes coherently + the Permits inclusion holds; it is a coherence/feasibility
// check, not the lambda_veh metatheorem.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Grade { bt: u8, eff: u16, lease: u64 }
// knowledge sources (SK9 lattice, joined not chained): bit0 lang_author (earliest) .. bit3 runtime (latest)
const LANG: u8 = 1; const BUNDLER: u8 = 2; const HOST: u8 = 4; const RUNTIME: u8 = 8;
// assurance derived from binding-time (SK8): the LATEST source present decides; earlier => higher assurance.
#[derive(Debug, PartialEq, PartialOrd)]
enum Assurance { Static = 3, Bundled = 2, LoadChecked = 1, Dynamic = 0 }
fn assurance_of(bt: u8) -> Assurance {
    if bt & RUNTIME != 0 { Assurance::Dynamic }
    else if bt & HOST != 0 { Assurance::LoadChecked }
    else if bt & BUNDLER != 0 { Assurance::Bundled }
    else { Assurance::Static }
}
// effect operations
const E_HOST_CALL: u16 = 1; const E_ALLOC: u16 = 2; const E_MACRO: u16 = 4;

// expression IR
enum E { Lit(u8 /*bt src*/), Var(u8 /*binder*/, u8 /*bt src*/), Op(u16 /*effect*/, u8 /*bt*/, Box<E>, Box<E>) }
fn grade(e: &E) -> Grade {
    match e {
        E::Lit(bt) => Grade { bt: *bt, eff: 0, lease: 0 },
        E::Var(b, bt) => Grade { bt: *bt, eff: 0, lease: 1u64 << b }, // lease = {b} (coeffect)
        E::Op(eff, bt, f, a) => {
            let gf = grade(f); let ga = grade(a);
            Grade {
                bt: gf.bt | ga.bt | bt,     // binding-time: JOIN (modality)
                eff: gf.eff | ga.eff | eff, // effect: UNION (graded monad)
                lease: gf.lease | ga.lease, // lease: COMBINE (coeffect)
            }
        }
    }
}
fn included(g: &Grade, permits: u16) -> bool { g.eff & !permits == 0 } // inclusion-not-coverage

fn main() {
    // program: host_call(macro_expand(lit@lang, var b0@bundler), var b1@host)
    let prog = E::Op(E_HOST_CALL, HOST,
        Box::new(E::Op(E_MACRO, BUNDLER, Box::new(E::Lit(LANG)), Box::new(E::Var(0, BUNDLER)))),
        Box::new(E::Var(1, HOST)));
    let g = grade(&prog);
    println!("composed grade: {:?}", g);
    println!("  binding-time sources = 0b{:04b} (lang|bundler|host)", g.bt);
    println!("  effect = 0b{:03b} (host_call|alloc|macro)", g.eff);
    println!("  lease  = 0b{:064b} (binders b0,b1)", g.lease);
    println!("  assurance (derived from bt, SK8) = {:?}", assurance_of(g.bt));
    // Permits check: a target permitting host_call + macro but NOT alloc
    let permits = E_HOST_CALL | E_MACRO;
    assert!(included(&g, permits), "well-graded program included");
    println!("included in target permitting {{host_call,macro}}: {}", included(&g, permits));
    // an over-effecting program (does alloc, target forbids it) -> rejected
    let bad = E::Op(E_ALLOC, RUNTIME, Box::new(E::Lit(LANG)), Box::new(E::Lit(LANG)));
    println!("alloc program included in {{host_call,macro}} target: {} (expect false -> rejected)", included(&grade(&bad), permits));
    assert!(!included(&grade(&bad), permits));
    // assurance ordering falls out: an all-lang-author program is Static (highest)
    assert!(assurance_of(LANG) > assurance_of(RUNTIME));
    println!("SP4: four-axis grade composes (bt join, eff union, lease combine); assurance derived from bt; inclusion-not-coverage holds. WORKS.");
}
