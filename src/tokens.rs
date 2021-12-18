#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Not used in actual token, but good as default
    Whitespace,

    // Single Char tokens
    OpenCurly, CloseCurly, OpenParen, CloseParen, Comma, Dot, Minus, Plus,
    SemiColon, Divide, Multiply,

    // One or two character tokens
    Not, NEqual, EqualSign, Equals, Greater, GreaterEq,
    Less, LessEq,

    // Literals
    Identifier, StringLiteral, Number,

    // Keywords
    And, Else, False, Function, For, If, Nil, Or, Print, 
    Return, This, True, Var, Let, While, Eof,
}

#[derive(Debug, Clone)]
pub struct Token {

    /// Type of the token
    pub t_type: TokenType,

    /// Value of token
    pub value: String,

    /// Line value from which the token was created
    pub line_num: u32,
}
