use either::*;
use std::fmt;
use std::io;
use std::io::{Read, Write};

// custom formatted error string from the eval function
// as eval implements a language so it has it's own
// stacktrace printing capabilities.
// This is also why I use Either<String, String> over
// Result<String, MyCustomLanguage::Error>
type EvalError = String;

enum EvaluationResult {
    Output(Either<String, EvalError>),
    ReplInputMode(SupportedReplInputMode),
    ReplRunning(bool),
}

enum SupportedReplInputMode {
    SingleLine,
    MultiLine,
}

#[derive(Debug)]
enum ReplError {
    IO(io::Error),
}

impl From<io::Error> for ReplError {
    fn from(io_error: io::Error) -> ReplError {
        ReplError::IO(io_error)
    }
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplError::IO(io_error) => write!(f, "{}", io_error),
        }
    }
}

fn main() -> Result<(), ReplError> {
    let mut repl_input_mode: SupportedReplInputMode = SupportedReplInputMode::SingleLine;

    println!("Hi! 👋🏼\nType ? for help.");

    loop {
        let input = read(&repl_input_mode);
        let output = eval(input?);

        match output {
            EvaluationResult::Output(output) => {
                match output {
                    Left(err) => {
                        print!("Something went wrong! {}", err);
                    }
                    Right(o) => {
                        print!("{}", o);
                    }
                }

                io::stdout().flush()?;
                // reset input mode to single line after flushing output
                repl_input_mode = SupportedReplInputMode::SingleLine;
            }
            EvaluationResult::ReplInputMode(r) => repl_input_mode = r,
            EvaluationResult::ReplRunning(false) => break,
            EvaluationResult::ReplRunning(true) => continue,
        }
    }
    println!("Bye 👋🏼!");

    Ok(())
}

fn read(repl_input_mode: &SupportedReplInputMode) -> Result<String, ReplError> {
    let mut input = String::new();
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    match repl_input_mode {
        SupportedReplInputMode::MultiLine => {
            println!("Press enter followed ctrl + d to mark multiline input as done");
            print!("");
            stdout.flush()?;

            stdin.read_to_string(&mut input)?;
            Ok(input)
        }
        SupportedReplInputMode::SingleLine => {
            print!("> ");
            stdout.flush()?;

            stdin.read_line(&mut input)?;
            Ok(input)
        }
    }
}

fn eval(input: String) -> EvaluationResult {
    let input = input.trim();

    match input {
        ".m" => EvaluationResult::ReplInputMode(SupportedReplInputMode::MultiLine),
        ".e" => EvaluationResult::ReplRunning(false),
        "?" => EvaluationResult::Output(Right(
            "Commands:\n\t.m -> enable multiline mode\n\t.e -> exit repl\n".to_string(),
        )),
        _ => EvaluationResult::Output(Right(format!("{}\n", input))),
    }
}
