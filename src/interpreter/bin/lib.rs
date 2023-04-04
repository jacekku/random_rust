pub mod diagram;
pub mod scanner;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    Title,
    Colon,
    Arrow,
    Unexpected,
    Number,
    Error,
    Endl,
    Value,
}
#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    line: usize,
    text: String,
}

trait CharValidations {
    fn digit(&self) -> bool;
    fn alphabetic(&self) -> bool;
    fn alpha_numeric(&self) -> bool;
}

impl CharValidations for char {
    fn digit(&self) -> bool {
        self.is_digit(10)
    }

    fn alphabetic(&self) -> bool {
        self.is_alphabetic() || self == &'_' || self == &' '
    }

    fn alpha_numeric(&self) -> bool {
        self.alphabetic() || self.digit()
    }
}
