#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    Title,
    Colon,
    Arrow,
}
#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    token_type: TokenType,
    line: i32,
    text: &'a str,
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    current: usize,
    start: usize,
}

impl Scanner<'static> {
    pub fn new(source: &'static str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            current: 0,
            start: 0,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current].trim();
        println!("{}", &text);

        let token = Token {
            token_type,
            line: 1,
            text,
        };
        self.tokens.push(token);
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' || c == ' ';
    }
    fn is_alpha_option(c: Option<char>) -> bool {
        match c {
            Some(c) => Self::is_alpha(c),
            None => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
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

    pub fn scan(&mut self) -> Vec<&Token> {
        loop {
            self.start = self.current;
            let c = self.advance();

            match c {
                Some(c) => match c {
                    ' ' => continue,
                    ':' => self.add_token(TokenType::Colon),
                    '-' => {
                        if self.match_next_token('>') {
                            self.add_token(TokenType::Arrow)
                        }
                    }

                    _ => {
                        if Self::is_alpha(c) {
                            self.identifier_or_keyword();
                        } else {
                            continue;
                        }
                    }
                },
                None => break,
            }
        }
        let mut tokens = Vec::new();
        for token in self.tokens.iter() {
            tokens.push(token);
        }
        return tokens;
    }

    fn identifier_or_keyword(&mut self) {
        while Self::is_alpha_option(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        if text == "title" {
            self.add_token(TokenType::Title);
            return;
        }
        self.add_token(TokenType::Identifier);
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_type, TokenType::Colon);
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
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].token_type, TokenType::Identifier);
        assert_eq!(result[0].text, "one");
        assert_eq!(result[1].token_type, TokenType::Arrow);
        assert_eq!(result[2].token_type, TokenType::Identifier);
        assert_eq!(result[2].text, "two");
        assert_eq!(result[3].token_type, TokenType::Colon);
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
        assert_eq!(result[2].token_type, TokenType::Identifier);
        assert_eq!(result[2].text, "some title");
    }
}
