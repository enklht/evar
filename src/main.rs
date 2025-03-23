use clap::Parser as ClapParser;
use directories::ProjectDirs;
use evar::{ErrorReporter, args::Args, create_context, lex_and_parse, readline::SevaEditor};
use rustyline::error::ReadlineError;

fn main() {
    let Args {
        fix,
        debug,
        no_color,
        angle_unit,
    } = Args::parse();

    let mut context = create_context(&angle_unit);
    let mut editor = SevaEditor::new(no_color);
    let mut reporter = ErrorReporter::new(no_color);

    let seva_dirs =
        ProjectDirs::from("", "enklht", "seva").expect("no valid home directory path retrieved");
    let mut history_path = std::path::PathBuf::from(seva_dirs.data_local_dir());

    match std::fs::create_dir_all(&history_path) {
        Ok(_) => {}
        Err(e) => eprintln!("failed to create data directory: {}", e),
    };

    history_path.push("history.txt");

    match editor.load_history(history_path.as_path()) {
        Ok(_) => {}
        Err(e) => eprintln!("failed to load historoy: {}", e),
    }

    loop {
        match editor.readline() {
            Ok(input) => {
                if input == "exit" || input == "quit" {
                    break;
                };

                if input == "help" {
                    context.print_help();
                    continue;
                }

                match lex_and_parse(&input) {
                    Ok(stmt) => {
                        if debug {
                            println!("{}", stmt)
                        };
                        match stmt.eval(&mut context) {
                            Ok(out) => out.print(fix),
                            Err(err) => eprintln!("{}", err),
                        }
                    }
                    Err(errs) => reporter.report_error(errs, &input),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                println!("Readline Error: {}", e);
                break;
            }
        }
    }

    match editor.save_history(history_path.as_path()) {
        Ok(_) => {}
        Err(e) => eprintln!("failed to save history: {}", e),
    }
}
