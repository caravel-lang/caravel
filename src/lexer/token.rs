use crate::position::Position;
use std::fmt;

// TokenValueKind is an enum with same names of TokenValue
// just without the values
#[derive(Clone, EnumKind)]
#[enum_kind(TokenValueKind)]
pub enum TokenValue {
    // Literals
    StringLiteral(String),
    CharLiteral(char),
    IntLiteral(i64),
    FloatLiteral(f64),

    // Arithmetic operations
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Modulo,

    // Logical operations

    // Assignment operations

    // Comparison operators

    // Relational operators

    // Whitespace
    EOL,

    // Other
    OpenParen,
    CloseParen,
}

impl TokenValue {
    pub fn get_kind(&self) -> TokenValueKind {
        TokenValueKind::from(self)
    }
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenValue::StringLiteral(string) => write!(f, "StringLiteral({})", string),
            TokenValue::CharLiteral(char) => write!(f, "CharLiteral({})", char),
            TokenValue::IntLiteral(val) => write!(f, "IntLiteral({})", val),
            TokenValue::FloatLiteral(val) => write!(f, "FloatLiteral({})", val),

            TokenValue::Addition => write!(f, "Addition"),
            TokenValue::Subtraction => write!(f, "Subtraction"),
            TokenValue::Multiplication => write!(f, "Multiplication"),
            TokenValue::Division => write!(f, "Division"),
            TokenValue::Exponentiation => write!(f, "Exponentiation"),
            TokenValue::Modulo => write!(f, "Modulo"),

            TokenValue::EOL => write!(f, "<EOL>"),

            TokenValue::OpenParen => write!(f, "("),
            TokenValue::CloseParen => write!(f, ")"),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub position: Position,
    pub value: TokenValue,
    pub representation: String,
    pub length: i32,
}
