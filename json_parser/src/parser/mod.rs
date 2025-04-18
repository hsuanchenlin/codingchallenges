use crate::json_value::JsonValue;
use crate::lexer::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    UnexpectedEndOfInput,
    ExpectedColon,
    ExpectedCommaOrCloseBrace,
    ExpectedCommaOrCloseBracket,
    InvalidJson,
}

// Maximum nesting depth for arrays/objects
const MAX_DEPTH: usize = 19;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    depth: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
            depth: 0,
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, ParseError> {
        if self.position >= self.tokens.len() {
            return Err(ParseError::UnexpectedEndOfInput);
        }

        // At this point we should already have validated that the first token is either
        // OpenBrace or OpenBracket in the parse_json function
        let result = self.parse_value()?;

        // Ensure we've consumed all tokens
        if self.position < self.tokens.len() {
            return Err(ParseError::UnexpectedToken);
        }

        Ok(result)
    }

    fn parse_value(&mut self) -> Result<JsonValue, ParseError> {
        if self.position >= self.tokens.len() {
            return Err(ParseError::UnexpectedEndOfInput);
        }

        match &self.tokens[self.position] {
            Token::OpenBrace => self.parse_object(),
            Token::OpenBracket => self.parse_array(),
            Token::String(s) => {
                self.position += 1;
                Ok(JsonValue::String(s.clone()))
            }
            Token::Number(n) => {
                self.position += 1;
                Ok(JsonValue::Number(*n))
            }
            Token::Boolean(b) => {
                self.position += 1;
                Ok(JsonValue::Boolean(*b))
            }
            Token::Null => {
                self.position += 1;
                Ok(JsonValue::Null)
            }
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParseError> {
        // Depth check
        if self.depth >= MAX_DEPTH {
            return Err(ParseError::InvalidJson);
        }
        self.depth += 1;

        // Consume the opening brace
        self.position += 1;

        let mut properties = Vec::new();

        // Handle empty object
        if self.position < self.tokens.len() && self.tokens[self.position] == Token::CloseBrace {
            self.position += 1;
            self.depth -= 1;
            return Ok(JsonValue::Object(properties));
        }

        loop {
            // Parse the key (must be a string)
            if self.position >= self.tokens.len() {
                return Err(ParseError::UnexpectedEndOfInput);
            }

            let key = match &self.tokens[self.position] {
                Token::String(s) => {
                    self.position += 1;
                    s.clone()
                }
                _ => return Err(ParseError::UnexpectedToken),
            };

            // Parse the colon
            if self.position >= self.tokens.len() || self.tokens[self.position] != Token::Colon {
                return Err(ParseError::ExpectedColon);
            }
            self.position += 1;

            // Parse the value
            let value = self.parse_value()?;

            properties.push((key, value));

            // Check for comma or closing brace
            if self.position >= self.tokens.len() {
                return Err(ParseError::UnexpectedEndOfInput);
            }

            match &self.tokens[self.position] {
                Token::Comma => {
                    self.position += 1;
                    // After comma, expect another key-value pair
                    if self.position >= self.tokens.len() || 
                       matches!(self.tokens[self.position], Token::CloseBrace) {
                        return Err(ParseError::UnexpectedToken); // Trailing comma is not allowed
                    }
                    continue;
                }
                Token::CloseBrace => {
                    self.position += 1;
                    self.depth -= 1;
                    break;
                }
                _ => return Err(ParseError::ExpectedCommaOrCloseBrace),
            }
        }

        Ok(JsonValue::Object(properties))
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParseError> {
        // Depth check
        if self.depth >= MAX_DEPTH {
            return Err(ParseError::InvalidJson);
        }
        self.depth += 1;

        // Consume the opening bracket
        self.position += 1;

        let mut elements = Vec::new();

        // Handle empty array
        if self.position < self.tokens.len() && self.tokens[self.position] == Token::CloseBracket {
            self.position += 1;
            self.depth -= 1;
            return Ok(JsonValue::Array(elements));
        }

        loop {
            // Parse the value
            let value = self.parse_value()?;
            elements.push(value);

            // Check for comma or closing bracket
            if self.position >= self.tokens.len() {
                return Err(ParseError::UnexpectedEndOfInput);
            }

            match &self.tokens[self.position] {
                Token::Comma => {
                    self.position += 1;
                    // After comma, expect another value
                    if self.position >= self.tokens.len() || 
                       matches!(self.tokens[self.position], Token::CloseBracket) {
                        return Err(ParseError::UnexpectedToken); // Trailing comma is not allowed
                    }
                    continue;
                }
                Token::CloseBracket => {
                    self.position += 1;
                    self.depth -= 1;
                    break;
                }
                _ => return Err(ParseError::ExpectedCommaOrCloseBracket),
            }
        }

        Ok(JsonValue::Array(elements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> Result<JsonValue, ParseError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        if tokens.is_empty() {
            return Err(ParseError::InvalidJson);
        }

        // Check if the first token is a valid starting token for JSON (object or array)
        if tokens[0] != Token::OpenBrace && tokens[0] != Token::OpenBracket {
            return Err(ParseError::InvalidJson);
        }

        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_parse_object() {
        let result = parse("{}");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), JsonValue::Object(vec![]));
    }

    #[test]
    fn test_parse_array() {
        let result = parse("[]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), JsonValue::Array(vec![]));
    }

    #[test]
    fn test_invalid_root_values() {
        // These should all be invalid as they are not objects or arrays
        assert!(parse("\"hello\"").is_err());
        assert!(parse("42").is_err());
        assert!(parse("true").is_err());
        assert!(parse("false").is_err());
        assert!(parse("null").is_err());
    }

    #[test]
    fn test_valid_nested_values() {
        // These values are valid when nested inside objects or arrays
        let result = parse(r#"{"key": "hello"}"#);
        assert!(result.is_ok());

        let result = parse(r#"{"key": 42}"#);
        assert!(result.is_ok());

        let result = parse(r#"{"key": true}"#);
        assert!(result.is_ok());

        let result = parse(r#"{"key": false}"#);
        assert!(result.is_ok());

        let result = parse(r#"{"key": null}"#);
        assert!(result.is_ok());

        let result = parse(r#"[1, "hello", true, false, null]"#);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_trailing_commas() {
        // JSON doesn't allow trailing commas
        assert!(parse(r#"[1, 2, 3, ]"#).is_err());
        assert!(parse(r#"{"a": 1, "b": 2, }"#).is_err());
    }
    
    #[test]
    fn test_max_depth() {
        // Test valid depth (19 levels)
        let valid_depth = "[[[[[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]]]]]";
        assert!(parse(valid_depth).is_ok());
        
        // Test exceeding max depth (20 levels)
        let exceeding_depth = "[[[[[[[[[[[[[[[[[[[\"Too deep\"]]]]]]]]]]]]]]]]]]]]";
        assert!(parse(exceeding_depth).is_err());
    }
}
