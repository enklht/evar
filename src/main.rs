use clap::Parser as ClapParser;
use directories::ProjectDirs;
use seva::{ErrorReporter, args::Args, create_context, parser, readline::SevaEditor};
use winnow::Parser;

fn main() {
    let Args {
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
        let input = editor.readline();

        if input == "exit" || input == "quit" {
            break;
        };

        match parser.parse(&input) {
            Ok(stmt) => {
                if debug {
                    println!("{}", stmt)
                };
                match stmt.eval(&mut context) {
                    Ok(out) => println!("{:?}", out),
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    match editor.save_history(history_path.as_path()) {
        Ok(_) => {}
        Err(e) => eprintln!("failed to save history: {}", e),
    }
}
