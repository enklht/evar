use std::fmt;

use logos::Logos;

#[derive(Logos, Clone, PartialEq, Debug)]
pub enum Token<'a> {
    Error,

    #[regex(r"\s+")]
    Space,

    #[regex(r"(0|[1-9]\d*)(\.\d+)?([eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[regex(r"[+\-\*/%^!]")]
    Operator(&'a str),

    #[token("(", |_| '(')]
    #[token(")", |_| ')')]
    #[token(",", |_| ',')]
    Ctrl(char),

    #[regex(r"[[:alpha:]][[:alnum:]]*")]
    Ident(&'a str),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "unknown symbol"),
            Self::Number(s) => write!(f, "{}", s),
            Self::Operator(s) => write!(f, "{}", s),
            Self::Ctrl(s) => write!(f, "{}", s),
            Self::Ident(s) => write!(f, "{}", s),
            Self::Space => write!(f, " "),
        }
    }
}
