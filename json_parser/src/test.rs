// src/tests.rs

#[cfg(test)]
mod tests {
    use crate::json_value::JsonValue;
    use crate::lexer;
    use crate::parser;
    use crate::parser::ParseError;

    fn parse_json(input: &str) -> Result<JsonValue, ParseError> {
        let mut lexer = lexer::Lexer::new(input);
        let tokens = lexer.tokenize();

        if tokens.is_empty() {
            return Err(ParseError::InvalidJson);
        }

        // Check if the first token is a valid starting token for JSON (object or array)
        if tokens[0] != lexer::Token::OpenBrace && tokens[0] != lexer::Token::OpenBracket {
            return Err(ParseError::InvalidJson);
        }

        let mut parser = parser::Parser::new(tokens);
        let result = parser.parse()?;

        // Double-check the result just to be safe
        match result {
            JsonValue::Object(_) | JsonValue::Array(_) => Ok(result),
            _ => Err(ParseError::InvalidJson),
        }
    }
    
    // Add your actual test functions here
    #[test]
    fn test_empty_object() {
        let input = "{}";
        assert!(parse_json(input).is_ok());
    }
    
    #[test]
    fn test_empty_array() {
        let input = "[]";
        assert!(parse_json(input).is_ok());
    }
    
    #[test]
    fn test_invalid_json() {
        let input = "{";
        assert!(parse_json(input).is_err());
    }
    
    // Add more tests as needed
}