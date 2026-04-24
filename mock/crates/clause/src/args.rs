//! Hand-rolled argument parser for the round-one CLI surface.

use notko::Maybe;

/// Extract the first positional argument from a subcommand's
/// remaining argv slice, returning the path as a `&str`.
///
/// Returns `Maybe::Isnt` if the slice is empty.
pub fn single_file_arg(rest: &[String]) -> Maybe<&str> { // lint:allow(bare_string) reason: binary-entry plumbing; argv is OsString-shaped at the std boundary; tracked: #73 lint:allow(no-bare-string) tracked: #207
    match rest.first() {
        Some(path) => Maybe::Is(path.as_str()),
        None => Maybe::Isnt,
    }
}
