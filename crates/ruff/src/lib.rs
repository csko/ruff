//! This is the library for the [Ruff] Python linter.
//!
//! **The API is currently completely unstable**
//! and subject to change drastically.
//!
//! [Ruff]: https://github.com/charliermarsh/ruff

pub use ruff_python_ast::source_code::round_trip;
pub use ruff_python_ast::types::Range;
pub use rule_selector::RuleSelector;
pub use rules::pycodestyle::rules::IOError;
pub use violation::{AutofixKind, Availability as AutofixAvailability};

mod autofix;
mod checkers;
mod codes;
mod cst;
pub mod directives;
mod doc_lines;
mod docstrings;
pub mod fix;
pub mod flake8_to_ruff;
pub mod fs;
mod lex;
pub mod linter;
pub mod logging;
pub mod message;
mod noqa;
pub mod packaging;
pub mod registry;
pub mod resolver;
mod rule_redirects;
mod rule_selector;
pub mod rules;
pub mod settings;
mod violation;

#[cfg(test)]
mod test;
