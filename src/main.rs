use either::*;
use std::io;
use std::io::Read;
use std::io::Write;

enum SupportedReplInputMode {
    SingleLine,
    MultiLine,
}

enum EvaluationResult {
    Output(Either<String, String>),
    ReplInputMode(SupportedReplInputMode),
    ReplRunning(bool),
}

fn main() {
    let mut repl_input_mode = SupportedReplInputMode::SingleLine;

    loop {
        let input = read(&repl_input_mode);
        let output = eval(input);

        match output {
            EvaluationResult::Output(output) => {
                match output {
                    Left(o) => {
                        print!("{}", o);
                    }
                    Right(err) => {
                        print!("Something went wrong! {}", err);
                    }
                }

                io::stdout().flush().unwrap();
                // reset input mode to single line after flushing output
                repl_input_mode = SupportedReplInputMode::SingleLine;
            }
            EvaluationResult::ReplInputMode(r) => repl_input_mode = r,
            EvaluationResult::ReplRunning(false) => break,
            _ => continue,
        }
    }
    println!("Bye 👋🏼!")
}

fn read(repl_input_mode: &SupportedReplInputMode) -> String {
    let mut input: String = String::new();

    match repl_input_mode {
        SupportedReplInputMode::MultiLine => {
            print!("");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_to_string(&mut input)
                .expect("Failed to read input");
            input
        }
        SupportedReplInputMode::SingleLine => {
            print!("> ");
            io::stdout().flush().unwrap();
            // read
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input
        }
    }
}

fn eval(input: String) -> EvaluationResult {
    let input = input.trim();

    match input {
        ".m" => EvaluationResult::ReplInputMode(SupportedReplInputMode::MultiLine),
        ".exit" => EvaluationResult::ReplRunning(false),
        _ => EvaluationResult::Output(Left(format!("{}\n", input))),
    }
}
