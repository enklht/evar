use std::fmt;

use logos::Logos;

#[derive(Logos, Clone, PartialEq, Debug)]
pub enum Token<'a> {
    Error,

    #[regex(r"\s+")]
    Space,

    #[regex(r"(0|[1-9]\d*)(\.\d+)?([eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("^")]
    Caret,
    #[token("!")]
    Exclamation,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,

    #[regex(r"[[:alpha:]][[:alnum:]]*")]
    Ident(&'a str),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "unknown symbol"),
            Self::Space => write!(f, " "),
            Self::Number(s) => write!(f, "{}", s),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Caret => write!(f, "^"),
            Self::Exclamation => write!(f, "!"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::Comma => write!(f, ","),
            Self::Ident(s) => write!(f, "{}", s),
        }
    }
}
