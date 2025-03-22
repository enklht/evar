pub mod args;
mod default_context;
mod error_report;
pub mod models;
mod parser;
pub mod readline;

use chumsky::{
    input::{Input, Stream},
    prelude::*,
};
pub use default_context::create_context;
pub use error_report::ErrorReporter;
use parser::parser;

use models::Stmt;
use models::Token;

pub fn lex_and_parse(input: &str) -> Result<Stmt, Vec<Rich<'_, Token<'_>>>> {
    let token_iter = models::token::lex(input).filter(|token| match token {
        (Token::Space, _) => false,
        _ => true,
    });

    let token_stream = Stream::from_iter(token_iter)
        .map((input.len()..input.len()).into(), |(token, span)| {
            (token, span.into())
        });

    parser().parse(token_stream).into_result()
}
