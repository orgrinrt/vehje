//! `clause` CLI entry point.

fn main() {
    let args: Vec<String> = std::env::args().collect(); // lint:allow(bare_collection) lint:allow(bare_string) reason: std::env::args returns OsString-shaped data; Vec+String is the canonical binary-entry shape and not part of any public API; tracked: #73
    let code = match args.get(1).map(String::as_str) {
        Some("lex") => clause::lex::run(&args[2..]),
        Some("parse") => clause::parse::run(&args[2..]),
        Some("check") => clause::check::run(&args[2..]),
        Some("build") => clause::build::run(&args[2..]),
        Some("run") => clause::run::run(&args[2..]),
        Some(other) => {
            eprintln!("clause: unknown subcommand: {other}");
            print_usage();
            2
        }
        None => {
            print_usage();
            2
        }
    };
    std::process::exit(code);
}

fn print_usage() {
    eprintln!("usage: clause <lex|parse|check|build|run> <file>");
}
