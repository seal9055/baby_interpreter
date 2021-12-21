use crate::tokens::{Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Assignment {
        name: Token,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>, 
        arguments: Vec<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        literal: Literal,
    },
    Logical {
        l_expr: Box<Expr>,
        operator: LogicalOp,
        r_expr: Box<Expr>,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    Or,
    And,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    StringLiteral(String),
    True,
    False,
    Nil
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(Expr),
    Variable(Token, Option<Expr>),
    Block(Vec<Stmt>),
    Function(Token, Vec<Token>, Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Return(Option<Expr>),
    While(Expr, Box<Stmt>),
    Print(Expr),
}
