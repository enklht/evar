use std::borrow::Cow;

use crate::{args::Args, errors::SevaError, lexer::Token};
use logos::Logos;
use rustyline::{
    Completer, Config, Editor, Helper, Highlighter, Hinter, Validator,
    completion::FilenameCompleter, highlight::Highlighter, hint::HistoryHinter,
    history::FileHistory, validate::MatchingBracketValidator,
};
use yansi::Paint;

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
                    Err(_) => format!("{}", line[span].rgb(237, 135, 150)),
                    Ok(token) => match token {
                        Token::Number(_) => format!("{}", line[span].rgb(245, 169, 127)),
                        Token::Ident(_) => format!("{}", line[span].rgb(138, 173, 244)),
                        Token::Plus
                        | Token::Minus
                        | Token::Asterisk
                        | Token::Slash
                        | Token::Percent
                        | Token::Caret
                        | Token::Exclamation => format!("{}", line[span].rgb(125, 196, 228)),
                        Token::LParen | Token::RParen => {
                            format!("{}", line[span].rgb(238, 212, 159))
                        }
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
        format!("{}", hint.rgb(91, 96, 120)).into()
    }
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        format!("{}", prompt.rgb(166, 218, 149)).into()
    }
}

pub struct SevaEditor(Editor<RustyLineHelper, FileHistory>);

impl SevaEditor {
    pub fn new(_args: &Args) -> SevaEditor {
        if _args.no_color {
            yansi::disable();
        } else {
            yansi::enable();
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

    pub fn readline(&mut self) -> Result<String, SevaError> {
        self.0.readline("> ").map_err(SevaError::ReadlineError)
    }
}
