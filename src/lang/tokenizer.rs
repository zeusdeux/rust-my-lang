use std::fmt;

#[derive(Debug)]
enum TokenClass {
    Integer,
    Alphabet,
    NewLine,
    WhiteSpace,
    UnknownChar,
}

impl fmt::Display for TokenClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenClass::Integer => write!(f, "Integer"),
            TokenClass::Alphabet => write!(f, "Alphabet"),
            TokenClass::NewLine => write!(f, "NewLine"),
            TokenClass::WhiteSpace => write!(f, "WhiteSpace"),
            TokenClass::UnknownChar => write!(f, "UnknownChar"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    class: TokenClass,
    value: String, // TODO: this could maybe be the parsed value of the given TokenClass? idk yet
    line: u32,
    col: u32,
}

impl Token {
    fn new(class: TokenClass, val: String, line: u32, col: u32) -> Token {
        Token {
            class,
            value: val,
            line,
            col,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, '{}', line: {}, column: {}) ",
            self.class, self.value, self.line, self.col
        )
    }
}

pub fn tokenize(input: &str) -> super::Result<Vec<Token>> {
    let input_as_vec = Vec::from(input);
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: u32 = 1;
    let mut col: u32 = 1;

    // TODO: Actually tokenize full numbers, strings, etc instead of each character
    for t in input_as_vec.into_iter() {
        match t {
            10 => {
                let token = Token::new(TokenClass::NewLine, "\\n".to_string(), line, col);
                line = line + 1; // increment line since we found \n
                col = 0; // reset column to 0 instead of 1 as it is incremented at the end of this loop
                tokens.push(token)
            }
            32 => tokens.push(Token::new(
                TokenClass::WhiteSpace,
                String::from(char::from(t)),
                line,
                col,
            )),
            48..=57 => tokens.push(Token::new(
                TokenClass::Integer,
                String::from(char::from(t)),
                line,
                col,
            )),
            65..=90 | 97..=122 => tokens.push(Token::new(
                TokenClass::Alphabet,
                String::from(char::from(t)),
                line,
                col,
            )),
            _ => {
                if !t.is_ascii() {
                    let error = format!(
                        "Input code: {} is not ascii on line: {}, column: {}",
                        t, line, col
                    );
                    return Err(super::Error::Tokenizer(error));
                }
                tokens.push(Token::new(
                    TokenClass::UnknownChar,
                    format!("{}", t),
                    line,
                    col,
                ))
            }
        }
        col += 1;
    }

    Ok(tokens)
}
