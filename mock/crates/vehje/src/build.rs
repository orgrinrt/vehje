//! `vehje build`, stub. Real codegen dispatch lands in a follow-up round.

pub fn run(_args: &[String]) -> i32 {
    // lint:allow(bare_string) lint:allow(bare_numeric) reason: CLI subcommand entry; argv and exit code are std-boundary shapes; tracked: #73 lint:allow(arvo-types-only) lint:allow(no-bare-numeric) lint:allow(no-bare-string) tracked: #207
    eprintln!("build: not yet implemented");
    2
}
