pub mod tokenizer;
use std::{fmt, result};

#[derive(Debug)]
pub enum Error {
    Tokenizer(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Tokenizer(err) => write!(f, "Tokenizer error\n{}", err),
        }
    }
}

type Result<T> = result::Result<T, Error>;
