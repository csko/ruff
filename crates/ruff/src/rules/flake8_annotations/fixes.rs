use anyhow::{bail, Result};
use rustpython_parser::ast::Stmt;
use rustpython_parser::{lexer, Mode, Tok};

use ruff_python_ast::source_code::Locator;
use ruff_python_ast::types::Range;

use crate::fix::Fix;

/// ANN204
pub fn add_return_none_annotation(locator: &Locator, stmt: &Stmt) -> Result<Fix> {
    let range = Range::from(stmt);
    let contents = locator.slice(range);

    // Find the colon (following the `def` keyword).
    let mut seen_lpar = false;
    let mut seen_rpar = false;
    let mut count: usize = 0;
    for (start, tok, ..) in lexer::lex_located(contents, Mode::Module, range.location).flatten() {
        if seen_lpar && seen_rpar {
            if matches!(tok, Tok::Colon) {
                return Ok(Fix::insertion(" -> None".to_string(), start));
            }
        }

        if matches!(tok, Tok::Lpar) {
            if count == 0 {
                seen_lpar = true;
            }
            count += 1;
        }
        if matches!(tok, Tok::Rpar) {
            count -= 1;
            if count == 0 {
                seen_rpar = true;
            }
        }
    }

    bail!("Unable to locate colon in function definition");
}
