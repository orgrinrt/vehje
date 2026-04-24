//! Diagnostic printer.

use vehje_ir::Diagnostic;

/// Print `diag` to stderr with a `<file>:<line>:<col>: <phase>: <message>` prefix.
pub fn print(file_label: &str, src: &str, diag: &Diagnostic) { // lint:allow(bare_string) reason: host-side CLI uses &str for file path and source body; tracked: #73 lint:allow(no-bare-string) tracked: #207
    let off = diag.span.start.0 as usize;  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    let (line, col) = line_col(src, off);
    eprintln!(
        "{file_label}:{line}:{col}: {:?}: {}",
        diag.phase, diag.message
    );
}

fn line_col(src: &str, off: usize) -> (u32, u32) { // lint:allow(bare_numeric) lint:allow(bare_string) reason: 1-based line/col tuple is the conventional CLI diagnostic output shape; tracked: #73 lint:allow(arvo-types-only) lint:allow(no-bare-numeric) lint:allow(no-bare-string) tracked: #207
    let bytes = src.as_bytes();
    let end = off.min(bytes.len());
    let mut line: u32 = 1;  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    let mut line_start: usize = 0;  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    for (i, &b) in bytes.iter().take(end).enumerate() {
        if b == b'\n' {
            line += 1;
            line_start = i + 1;
        }
    }
    let col = (end - line_start) as u32 + 1;  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    (line, col)
}
