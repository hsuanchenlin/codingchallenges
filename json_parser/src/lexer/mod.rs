mod token;
pub use token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        let c = self.input[self.position];
        self.position += 1;

        match c {
            '{' => Some(Token::OpenBrace),
            '}' => Some(Token::CloseBrace),
            '[' => Some(Token::OpenBracket),
            ']' => Some(Token::CloseBracket),
            ':' => Some(Token::Colon),
            ',' => Some(Token::Comma),
            '"' => self.parse_string(),
            '-' | '0'..='9' => {
                self.position -= 1; // Go back to first digit
                self.parse_number()
            }
            't' => {
                self.position -= 1; // Go back to start of keyword
                self.parse_true()
            }
            'f' => {
                self.position -= 1; // Go back to start of keyword
                self.parse_false()
            }
            'n' => {
                self.position -= 1; // Go back to start of keyword
                self.parse_null()
            }
            _ => {
                // Found an unexpected character
                None
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let c = self.input[self.position];
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                self.position += 1;
            } else {
                break;
            }
        }
    }
    
    fn consume_digits(&mut self) {
        while self.position < self.input.len() && self.input[self.position] >= '0' && self.input[self.position] <= '9' {
            self.position += 1;
        }
    }

    fn parse_string(&mut self) -> Option<Token> {
        let mut result = String::new();
        let mut escape = false;

        while self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;

            if escape {
                match c {
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    '/' => result.push('/'),
                    'b' => result.push('\u{0008}'),  // backspace
                    'f' => result.push('\u{000C}'),  // form feed
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    'u' => {
                        // Parse 4 hex digits
                        if self.position + 4 > self.input.len() {
                            // Not enough characters for a complete Unicode escape
                            return None;
                        }

                        let hex = &self.input[self.position..self.position + 4]
                            .iter()
                            .collect::<String>();
                        self.position += 4;

                        if let Ok(code) = u32::from_str_radix(hex, 16) {
                            if let Some(c) = std::char::from_u32(code) {
                                result.push(c);
                            } else {
                                // Invalid Unicode code point
                                return None;
                            }
                        } else {
                            // Invalid hex digits in Unicode escape
                            return None;
                        }
                    }
                    _ => {
                        // Invalid escape sequence
                        return None;
                    }
                }
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                return Some(Token::String(result));
            } else if c < '\u{0020}' {
                // Control characters not allowed in JSON strings
                return None;
            } else {
                result.push(c);
            }
        }

        // String wasn't closed
        None
    }

    fn parse_number(&mut self) -> Option<Token> {
        let start = self.position;

        // Optional negative sign
        if self.position < self.input.len() && self.input[self.position] == '-' {
            self.position += 1;
        }

        // Integer part
        if self.position < self.input.len() && self.input[self.position] == '0' {
            self.position += 1;
            // In JSON, leading zeros are not allowed (except for just "0")
            if self.position < self.input.len() && self.input[self.position] >= '0' && self.input[self.position] <= '9' {
                return None; // Invalid JSON number format with leading zero
            }
        } else if self.position < self.input.len() && self.input[self.position] >= '1' && self.input[self.position] <= '9' {
            self.position += 1;
            self.consume_digits();
        } else {
            return None; // Invalid number format
        }

        // Fractional part
        if self.position < self.input.len() && self.input[self.position] == '.' {
            self.position += 1;

            if self.position >= self.input.len() || self.input[self.position] < '0' || self.input[self.position] > '9' {
                return None; // Must have at least one digit after decimal point
            }

            self.consume_digits();
        }

        // Exponent part
        if self.position < self.input.len() && (self.input[self.position] == 'e' || self.input[self.position] == 'E') {
            self.position += 1;

            // Optional sign
            if self.position < self.input.len() && (self.input[self.position] == '+' || self.input[self.position] == '-') {
                self.position += 1;
            }

            if self.position >= self.input.len() || self.input[self.position] < '0' || self.input[self.position] > '9' {
                return None; // Must have at least one digit in exponent
            }

            self.consume_digits();
        }

        let number_str: String = self.input[start..self.position].iter().collect();
        match number_str.parse::<f64>() {
            Ok(num) => Some(Token::Number(num)),
            Err(_) => None,
        }
    }

    fn parse_true(&mut self) -> Option<Token> {
        let keyword = "true";
        if self.position + keyword.len() <= self.input.len() {
            let slice: String = self.input[self.position..self.position + keyword.len()].iter().collect();
            if slice == keyword {
                self.position += keyword.len();
                return Some(Token::Boolean(true));
            }
        }
        None
    }

    fn parse_false(&mut self) -> Option<Token> {
        let keyword = "false";
        if self.position + keyword.len() <= self.input.len() {
            let slice: String = self.input[self.position..self.position + keyword.len()].iter().collect();
            if slice == keyword {
                self.position += keyword.len();
                return Some(Token::Boolean(false));
            }
        }
        None
    }

    fn parse_null(&mut self) -> Option<Token> {
        let keyword = "null";
        if self.position + keyword.len() <= self.input.len() {
            let slice: String = self.input[self.position..self.position + keyword.len()].iter().collect();
            if slice == keyword {
                self.position += keyword.len();
                return Some(Token::Null);
            }
        }
        None
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty_object() {
        let mut lexer = Lexer::new("{}");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![Token::OpenBrace, Token::CloseBrace]);
    }

    #[test]
    fn test_tokenize_string() {
        let mut lexer = Lexer::new("\"hello\"");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![Token::String("hello".to_string())]);
    }

    #[test]
    fn test_tokenize_number() {
        let mut lexer = Lexer::new("123.45");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![Token::Number(123.45)]);
    }

    #[test]
    fn test_tokenize_keywords() {
        let mut lexer = Lexer::new("true false null");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Boolean(true),
            Token::Boolean(false),
            Token::Null
        ]);
    }

    #[test]
    fn test_tokenize_complex() {
        let mut lexer = Lexer::new("{\"key\": [1, true, null]}");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::OpenBrace,
            Token::String("key".to_string()),
            Token::Colon,
            Token::OpenBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::Boolean(true),
            Token::Comma,
            Token::Null,
            Token::CloseBracket,
            Token::CloseBrace
        ]);
    }
    
    #[test]
    fn test_invalid_tokens() {
        // Leading zeros
        let mut lexer = Lexer::new("01");
        let tokens = lexer.tokenize();
        assert!(tokens.is_empty());
        
        // Unterminated string
        let mut lexer = Lexer::new("\"hello");
        let tokens = lexer.tokenize();
        assert!(tokens.is_empty());
        
        // Invalid escape sequence
        let mut lexer = Lexer::new("\"\\x\"");
        let tokens = lexer.tokenize();
        assert!(tokens.is_empty());
    }
}