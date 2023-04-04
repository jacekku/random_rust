use super::{CharValidations, Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner<'_> {
    pub fn new(source: &'static str) -> Self {
        Scanner {
            source,
            current: 0,
            start: 0,
            line: 1,
        }
    }

    fn add_token(&mut self, tokens: &mut Vec<Token>, token_type: TokenType) {
        let text = &self.source[self.start..self.current].trim();
        println!("{}", &text);
        let token = Token {
            token_type,
            line: self.line,
            text: text.to_string(),
        };
        tokens.push(token);
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.source.chars().nth(self.current);
        self.current += 1;
        return next;
    }

    fn match_next_token(&mut self, c: char) -> bool {
        if let Some(next) = self.source.chars().nth(self.current) {
            if next == c {
                self.current += 1;
                return true;
            }
            return false;
        };
        return false;
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.start = self.current;
            let c = self.advance();

            match c {
                Some(c) => match c {
                    '\n' => {
                        self.add_token(&mut tokens, TokenType::Endl);
                        self.line += 1;
                    }
                    ' ' => continue,
                    ':' => {
                        self.add_token(&mut tokens, TokenType::Colon);
                        self.start += 1;
                        let token_type = self.identifier_or_keyword();
                        match token_type {
                            TokenType::Identifier => self.add_token(&mut tokens, TokenType::Value),
                            _ => self.add_token(&mut tokens, TokenType::Error),
                        }
                    }
                    '-' => {
                        if self.match_next_token('>') {
                            self.add_token(&mut tokens, TokenType::Arrow)
                        } else {
                            self.add_token(&mut tokens, TokenType::Unexpected);
                            continue;
                        }
                    }
                    '(' => self.number(&mut tokens),
                    _ => {
                        if c.alphabetic() {
                            let token_type = self.identifier_or_keyword();
                            self.add_token(&mut tokens, token_type);
                        } else {
                            self.add_token(&mut tokens, TokenType::Unexpected);
                            continue;
                        }
                    }
                },
                None => break,
            }
        }

        return tokens;
    }

    fn identifier_or_keyword(&mut self) -> TokenType {
        while self.peek().alpha_numeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        if text == "title" {
            return TokenType::Title;
        }
        TokenType::Identifier
    }

    fn number(&mut self, tokens: &mut Vec<Token>) {
        let valid = |c: char| c != ')' && c.digit();
        while valid(self.peek()) && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            self.add_token(tokens, TokenType::Error);
        }

        self.advance();

        self.add_token(tokens, TokenType::Number)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::TokenType;

    use super::*;

    #[test]
    fn empty_token_list() {
        let mut scanner = Scanner::new("");
        let result = scanner.scan();
        assert_eq!(result.len(), 0);
    }
    #[test]
    fn colon_token() {
        let mut scanner = Scanner::new(":");
        let result = scanner.scan();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].token_type, TokenType::Colon);
        assert_eq!(result[1].token_type, TokenType::Value);
    }

    #[test]
    fn arrow_token() {
        let mut scanner = Scanner::new("->");
        let result = scanner.scan();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Arrow);
    }

    #[test]
    fn identifier() {
        let mut scanner = Scanner::new("identifier");
        let result = scanner.scan();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Identifier);
        assert_eq!(result[0].text, "identifier");
    }

    #[test]
    fn multiple_tokens() {
        let mut scanner = Scanner::new("one->two:");
        let result = scanner.scan();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].token_type, TokenType::Identifier);
        assert_eq!(result[0].text, "one");
        assert_eq!(result[1].token_type, TokenType::Arrow);
        assert_eq!(result[2].token_type, TokenType::Identifier);
        assert_eq!(result[2].text, "two");
        assert_eq!(result[3].token_type, TokenType::Colon);
        assert_eq!(result[4].token_type, TokenType::Value);
    }

    #[test]
    fn identifier_with_space() {
        let mut scanner = Scanner::new("identifier with space");
        let result = scanner.scan();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Identifier);
        assert_eq!(result[0].text, "identifier with space");
    }

    #[test]
    fn trim_whitespace_before_identifier() {
        let mut scanner = Scanner::new(" identifier with space");
        let result = scanner.scan();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "identifier with space");
    }

    #[test]
    fn trim_whitespace_after_identifier() {
        let mut scanner = Scanner::new("identifier with space  ");
        let result = scanner.scan();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "identifier with space");
    }

    #[test]
    fn title_keyword_token() {
        let mut scanner = Scanner::new("title");
        let result = scanner.scan();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Title);
        assert_eq!(result[0].text, "title");
    }

    #[test]
    fn title_with_identifier() {
        let mut scanner = Scanner::new("title: some title");
        let result = scanner.scan();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].token_type, TokenType::Title);
        assert_eq!(result[1].token_type, TokenType::Colon);
        assert_eq!(result[2].token_type, TokenType::Value);
        assert_eq!(result[2].text, "some title");
    }

    #[test]
    fn mutliple_lines() {
        let mut scanner = Scanner::new(
            "title: some title
            one->two: hello",
        );
        let result = scanner.scan();

        assert_eq!(result.len(), 9);
        assert_eq!(result[0].token_type, TokenType::Title);
        assert_eq!(result[2].line, 1);
        assert_eq!(result[3].line, 1);
        assert_eq!(result[3].token_type, TokenType::Endl);
        assert_eq!(result[4].line, 2);
        assert_eq!(result[8].token_type, TokenType::Value);
    }

    #[test]
    fn unexpected_token() {
        let mut scanner = Scanner::new("one->(2)two");
        let result = scanner.scan();

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].token_type, TokenType::Identifier);
        assert_eq!(result[1].token_type, TokenType::Arrow);
        assert_eq!(result[2].token_type, TokenType::Number);
        assert_eq!(result[2].text, "(2)");
        assert_eq!(result[3].token_type, TokenType::Identifier);
    }
}
