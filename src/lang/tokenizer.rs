use std::fmt;

#[derive(Debug)]
enum TokenClass {
    Number,
    Alphabet,
    NewLine,
    WhiteSpace,
    OpenParen,
    CloseParen,
    Plus,
    Dash,
    Asterisk,
    ForwardSlash,
    Dot,
    UnknownChar,
}

impl fmt::Display for TokenClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenClass::Number => write!(f, "Number"),
            TokenClass::Alphabet => write!(f, "Alphabet"),
            TokenClass::NewLine => write!(f, "NewLine"),
            TokenClass::WhiteSpace => write!(f, "WhiteSpace"),
            TokenClass::OpenParen => write!(f, "OpenParen"),
            TokenClass::CloseParen => write!(f, "CloseParen"),
            TokenClass::Plus => write!(f, "Plus"),
            TokenClass::Dash => write!(f, "Dash"),
            TokenClass::Asterisk => write!(f, "Asterisk"),
            TokenClass::ForwardSlash => write!(f, "ForwardSlash"),
            TokenClass::Dot => write!(f, "Dot"),
            TokenClass::UnknownChar => write!(f, "UnknownChar"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    class: TokenClass,
    value: char, // char as it can be utf-8 encoded even though we only support ascii
    line: usize,
    col: usize,
}

impl Token {
    fn new(class: TokenClass, val: char, line: usize, col: usize) -> Self {
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
            self.class,
            self.value.escape_default(),
            self.line,
            self.col
        )
    }
}

pub fn tokenize(input: &str) -> super::Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: usize = 1;
    let mut col: usize = 1;
    let mut has_error = false;
    let mut error_token: Token = Token::new(TokenClass::UnknownChar, '🔥', line, col);

    // we iterate over unicode scalar values but only support ascii
    // we do this as we want to print the unicode somewhat correct on error
    for c in input.chars() {
        match c {
            '\n' => {
                // if an error was encountered during tokenizing, we still tokenize till the end of that line
                // so that we can print the full line on screen for user with an ascii drawn cursor pointing
                // to the column the error was on (like how rustc does with -----^)
                if has_error {
                    let error_message: String = format!(
                        "Input {} is not ascii\n{}\n",
                        error_token.value,
                        tokens
                            .into_iter()
                            .filter(|t| t.line == error_token.line) // print only the line with the error
                            .map(|t| t.value) // get the char out of the Token struct
                            .collect::<String>(),
                    );
                    return Err(super::Error::new(
                        super::ErrorKind::Tokenizer,
                        error_token.line,
                        error_token.col,
                        error_message,
                    ));
                }

                let token = Token::new(TokenClass::NewLine, c, line, col);
                line += 1; // increment line since we found \n
                col = 0; // reset column to 0 instead of 1 as it is incremented at the end of this loop
                tokens.push(token)
            }
            '\t' | '\x0C' | '\r' | ' ' => {
                tokens.push(Token::new(TokenClass::WhiteSpace, c, line, col))
            }
            '0'..='9' => tokens.push(Token::new(TokenClass::Number, c, line, col)),
            'A'..='Z' | 'a'..='z' => tokens.push(Token::new(TokenClass::Alphabet, c, line, col)),
            '(' => tokens.push(Token::new(TokenClass::OpenParen, c, line, col)),
            ')' => tokens.push(Token::new(TokenClass::CloseParen, c, line, col)),
            '+' => tokens.push(Token::new(TokenClass::Plus, c, line, col)),
            '-' => tokens.push(Token::new(TokenClass::Dash, c, line, col)),
            '*' => tokens.push(Token::new(TokenClass::Asterisk, c, line, col)),
            '/' => tokens.push(Token::new(TokenClass::ForwardSlash, c, line, col)),
            '.' => tokens.push(Token::new(TokenClass::Dot, c, line, col)),
            _ => {
                if !c.is_ascii() && !has_error {
                    has_error = true;
                    error_token = Token::new(TokenClass::UnknownChar, c, line, col);
                }
                tokens.push(Token::new(TokenClass::UnknownChar, c, line, col))
            }
        }
        col += 1;
    }

    Ok(tokens)
}
