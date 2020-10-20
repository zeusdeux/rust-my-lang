pub mod tokenizer;
use std::{fmt, iter, result};

#[derive(Debug)]
enum ErrorKind {
    Tokenizer,
    // Parser,
    // Interpreter
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    line: usize,
    col: usize,
    message: String,
}

impl Error {
    fn new(kind: ErrorKind, line: usize, col: usize, message: String) -> Self {
        Error {
            kind,
            line,
            col,
            message,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Tokenizer => write!(
                f,
                "Tokenizer error on line: {}, column: {}\n{}\n{}{}",
                self.line,
                self.col,
                // remove any extraneous newlines in the error message that prevent alignment of ----^ underneath the errored line
                self.message.trim(),
                iter::repeat('-').take(self.col - 1).collect::<String>(),
                '^'
            ),
        }
    }
}

type Result<T> = result::Result<T, Error>;
