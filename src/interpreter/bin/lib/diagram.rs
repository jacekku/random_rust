#[cfg(test)]
mod tests {
    use crate::lib::{scanner::Scanner, Token};

    fn scan(input: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(&input.to_string());
        let result = scanner.scan();
        return result;
    }

    #[test]
    fn empty_diagram() {
        let result = scan("");
    }
}
