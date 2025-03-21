use std::borrow::Cow;

use crate::lexer::Token;
use colored::Colorize;
use logos::Logos;
use rustyline::{
    Completer, Config, Editor, Helper, Highlighter, Hinter, Validator,
    completion::FilenameCompleter, highlight::Highlighter, hint::HistoryHinter,
    history::FileHistory, validate::MatchingBracketValidator,
};

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
struct RustyLineHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    #[rustyline(Highlighter)]
    highlighter: SevaHighlighter,
}

struct SevaHighlighter;

impl Highlighter for SevaHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        let tokens = Token::lexer(line).spanned();

        let highlighted_line = tokens.fold(String::new(), |acc, (lex_result, span)| {
            acc + &{
                match lex_result {
                    Err(_) => format!("{}", line[span].truecolor(237, 135, 150)),
                    Ok(token) => match token {
                        Token::Int(_) | Token::Float(_) => {
                            format!("{}", line[span].truecolor(245, 169, 127))
                        }
                        Token::Ident(_) => format!("{}", line[span].truecolor(138, 173, 244)),
                        Token::Plus
                        | Token::Minus
                        | Token::Asterisk
                        | Token::Slash
                        | Token::Percent
                        | Token::Caret
                        | Token::Exclamation => format!("{}", line[span].truecolor(125, 196, 228)),
                        Token::LParen | Token::RParen => {
                            format!("{}", line[span].truecolor(238, 212, 159))
                        }
                        Token::Let => format!("{}", line[span].truecolor(198, 160, 246)),
                        Token::Equal => format!("{}", line[span].truecolor(125, 196, 228)),
                        _ => line[span].to_string(),
                    },
                }
            }
        });

        highlighted_line.into()
    }
    fn highlight_char(
        &self,
        _line: &str,
        _pos: usize,
        kind: rustyline::highlight::CmdKind,
    ) -> bool {
        use rustyline::highlight::CmdKind;
        kind != CmdKind::MoveCursor && kind != CmdKind::ForcedRefresh
    }
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        format!("{}", hint.truecolor(91, 96, 120)).into()
    }
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        format!("{}", prompt.truecolor(166, 218, 149)).into()
    }
}

pub struct SevaEditor(Editor<RustyLineHelper, FileHistory>);

impl SevaEditor {
    pub fn new(no_color: bool) -> SevaEditor {
        if no_color {
            colored::control::set_override(false);
        }

        let editor_config = Config::builder()
            .auto_add_history(true)
            .completion_type(rustyline::CompletionType::List)
            .bell_style(rustyline::config::BellStyle::None)
            .build();

        let helper = RustyLineHelper {
            completer: FilenameCompleter::new(),
            validator: MatchingBracketValidator::new(),
            hinter: HistoryHinter::new(),
            highlighter: SevaHighlighter,
        };

        let mut editor = Editor::with_config(editor_config).expect("failed to create editor");
        editor.set_helper(Some(helper));
        editor.bind_sequence(rustyline::KeyEvent::ctrl('f'), rustyline::Cmd::CompleteHint);
        SevaEditor(editor)
    }

    pub fn readline(&mut self) -> String {
        self.0.readline("> ").expect("failed to read line")
    }
}
