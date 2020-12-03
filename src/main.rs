use rustyline::error::ReadlineError;
use rustyline::Editor;
// https://www.reddit.com/r/rust/comments/b1t6da/what_crates_should_i_use_to_make_a_repl/
fn main() {
    println!("Welcome to RusyPL v.0.0.1!");
    println!("Enter in Rust code on the bottom, or hit Ctrl+D to terminate the process");
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
