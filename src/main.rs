use std::fmt;
use std::io::{self, Read, Write};

mod lang;
use crate::lang::tokenizer;

enum EvaluationResult {
    Output(Result<String, lang::Error>),
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
    Lang(lang::Error),
}

impl From<io::Error> for ReplError {
    fn from(io_error: io::Error) -> ReplError {
        ReplError::IO(io_error)
    }
}

impl From<lang::Error> for ReplError {
    fn from(lang_error: lang::Error) -> ReplError {
        ReplError::Lang(lang_error)
    }
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplError::IO(io_error) => write!(f, "{}", io_error),
            ReplError::Lang(lang_error) => write!(f, "{}", lang_error),
        }
    }
}

fn main() -> Result<(), ReplError> {
    let mut repl_input_mode: SupportedReplInputMode = SupportedReplInputMode::SingleLine;

    println!("Hi! 👋🏼\nType ? for help.");

    loop {
        let input = read(&repl_input_mode)?;
        let output = eval(input);

        match output {
            EvaluationResult::Output(output) => {
                match output {
                    Err(err) => {
                        println!("Something went wrong! {}", err);
                    }
                    Ok(o) => {
                        println!("{}", o);
                    }
                }

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
    match &input[..] {
        ".m\n" => EvaluationResult::ReplInputMode(SupportedReplInputMode::MultiLine),
        ".e\n" => EvaluationResult::ReplRunning(false),
        ".q\n" => EvaluationResult::ReplRunning(false),
        "?\n" => EvaluationResult::Output(Ok(
            "Commands:\n\t.m -> enable multiline mode\n\t.q/.e -> quit/exit repl\n".to_string(),
        )),
        s => match tokenizer::tokenize(s) {
            Ok(tokens) => {
                let result = tokens.into_iter().map(|t| t.to_string()).collect();
                EvaluationResult::Output(Ok(result))
            }
            Err(error) => EvaluationResult::Output(Err(error)),
        },
    }
}
