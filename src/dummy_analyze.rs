use std::collections::HashSet;

pub fn dummy_analyze(input: &str) -> Result<Success, ParserError> {
    if input == "Rust1488" {
        let mut identifiers = HashSet::new();
        identifiers.insert(("foo".to_owned(), "bar".to_owned()));

        let mut constants = HashSet::new();
        constants.insert((228, "baz".to_owned()));

        Ok(Success {
            identifiers,
            constants,
        })
    } else {
        Err(ParserError::SemanticError(
            "Rust1488 expected".to_owned(),
            1488,
        ))
    }
}

pub struct Success {
    pub identifiers: HashSet<(String, String)>,
    pub constants: HashSet<(i32, String)>,
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    Operation(char),
    Assign,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    EndOfInput,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Token, usize),
    ExpectedToken(String, usize),
    SemanticError(String, usize),
}
