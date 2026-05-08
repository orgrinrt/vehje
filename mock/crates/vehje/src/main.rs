//! `vehje` CLI entry point.

fn main() {
    let args: Vec<String> = std::env::args().collect(); // lint:allow(bare_collection) lint:allow(bare_string) reason: std::env::args returns OsString-shaped data; Vec+String is the canonical binary-entry shape and not part of any public API; tracked: #73 lint:allow(no-bare-string) tracked: #207
    let code = match args.get(1).map(String::as_str) {  // lint:allow(no-bare-string) tracked: #207
        Some("lex") => vehje::lex::run(&args[2..]),
        Some("parse") => vehje::parse::run(&args[2..]),
        Some("check") => vehje::check::run(&args[2..]),
        Some("build") => vehje::build::run(&args[2..]),
        Some("run") => vehje::run::run(&args[2..]),
        Some(other) => {
            eprintln!("vehje: unknown subcommand: {other}");
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
    eprintln!("usage: vehje <lex|parse|check|build|run> <file>");
}
