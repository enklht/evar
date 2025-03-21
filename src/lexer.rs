use logos::Logos;

#[derive(Logos, Clone, PartialEq, Debug)]
pub enum Token<'a> {
    Error,

    #[regex(r"\s+")]
    Space,

    #[regex(r"-?(0|[1-9]\d*)", |lex| lex.slice().parse::<i32>().unwrap())]
    Int(i32),

    #[regex(r"-?(0|[1-9]\d*)((\.\d+([eE][+-]?\d+)?)|((\.\d+)?[eE][+-]?\d+))", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[token("let")]
    Let,
    #[token("=")]
    Equal,

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
    #[token("_")]
    Underscore,

    #[regex(r"[[:alpha:]][[:alnum:]]*")]
    Ident(&'a str),
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "unknown symbol"),
            Self::Space => write!(f, " "),
            Self::Int(s) => write!(f, "{}", s),
            Self::Float(s) => write!(f, "{}", s),
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
            Self::Let => write!(f, "let"),
            Self::Equal => write!(f, "="),
            Self::Underscore => write!(f, "_"),
        }
    }
}

pub fn lex(input: &str) -> impl Iterator<Item = (Token<'_>, std::ops::Range<usize>)> {
    Token::lexer(input).spanned().map(|(tok, span)| match tok {
        Ok(tok) => (tok, span),
        Err(()) => (Token::Error, span),
    })
}
