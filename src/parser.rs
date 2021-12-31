use crate::tokens::TokenType::*;
use crate::tokens::{Token, TokenType};
use crate::ast::*;
use crate::err::{Error};

#[derive(Clone, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn next(&mut self) -> &Token {
        if !self.is_at_end() {
            self.index +=1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
       &self.tokens[self.index - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().t_type == Eof
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for cur_type in types {
            if self.check(*cur_type) {
                self.next();
                return true
            }
        }
        false
    }

    fn check(&self, t_type: TokenType) -> bool {
        if self.is_at_end() { return false; }
        return self.peek().t_type == t_type;
    }

    fn lc(&self) -> u32 {
        return self.peek().line_num;
    }

    /// Consume a token if it has the correct type and advance the parser
    fn consume(&mut self, t_type: TokenType, msg: &str, l: u32)
            -> Result<Token, Error> {
        if self.check(t_type) {
            Ok(self.next().clone())
        } else {
            Err(Error::new(msg.to_string(), l))
        }
    }

    /// Parse the program, and return either a vector of statements if the
    /// parsing is successful or a vector of errors containing all found errors
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<Error>> {
        let mut stmts:  Vec<Stmt> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    errors.push(err.clone());
                    //self.synchronize();
                }
            }
        }

        if !errors.is_empty() {
           Err(errors)
        } else {
            Ok(stmts)
        }
    }

    // Statements ====================================================

    fn declaration(&mut self) -> Result<Stmt, Error> {
        match self.peek().t_type {
            Var => {
                self.next();
                self.var_decl()
            },
            Let => {
                self.next();
                self.let_decl()
            },
            Function => {
                self.next();
                self.fun_decl()
            },
            _ => { self.statement() },
        }
    }

    fn fun_decl(&mut self) -> Result<Stmt, Error> {
        let fun_name = self.consume(Identifier, "Expected function name",
                                    self.lc())?;
        self.consume(OpenParen, "Expected '(' after function declaration",
                    self.lc())?;

        let mut args: Vec<Token> = Vec::new();
        if !self.check(CloseParen) {
            args.push(self.consume(Identifier, "Expected parameter name",
                                   self.lc())?);
            while self.check(Comma) {
                self.consume(Comma, "weird error", self.lc())?;
                args.push(self.consume(Identifier, "Expected parameter name",
                                       self.lc())?);
            }
        }
        self.consume(CloseParen, "Expected ')' after function arguments"
                     ,self.lc())?;
        self.consume(OpenCurly, "Expected '{' after function header",
                     self.lc())?;
        let fun_body = self.block_statement()?;
        Ok(Stmt::Function(fun_name, args, vec![fun_body]))
    }

    fn var_decl(&mut self) -> Result<Stmt, Error> {
        let var_name = self.consume(Identifier, "Expected var name",
                                    self.lc())?;
        let mut initializer: Option<Expr> = None;

        // Parse variable initialization using equal sign
        if self.match_tokens(&[EqualSign]) {
            initializer = Some(self.expression()?);
        }

        // Parse SemiColon after var declaration
        self.consume(SemiColon, "Expected ';' after variable declaration",
                     self.lc())?;

        Ok(Stmt::Variable(var_name, initializer))
    }

    fn let_decl(&mut self) -> Result<Stmt, Error> {
        let var_name = self.consume(Identifier, "Expected var name",
                                    self.lc())?;
        let mut initializer: Option<Expr> = None;

        // Parse variable initialization using equal sign
        if self.match_tokens(&[EqualSign]) {
            initializer = Some(self.expression()?);
        }

        // Parse SemiColon after var declaration
        self.consume(SemiColon, "Expected ';' after variable declaration",
                     self.lc())?;

        Ok(Stmt::Variable(var_name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.match_tokens(&[Print]) {
            return self.print_statement();
        }
        if self.match_tokens(&[OpenCurly]) {
            return self.block_statement();
        }
        if self.match_tokens(&[If]) {
            return self.if_statement();
        }
        if self.match_tokens(&[While]) {
            return self.while_statement();
        }
        if self.match_tokens(&[For]) {
            return self.for_statement();
        }
        if self.match_tokens(&[Return]) {
            return self.return_statement();
        }
        self.expr_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(SemiColon, "Expected ';' after value", self.lc())?;
        Ok(Stmt::Print(expr))
    }

    fn block_statement(&mut self) -> Result<Stmt, Error> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.check(CloseCurly) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }
        self.consume(CloseCurly, "Expected '}' after block.", self.lc())?;
        Ok(Stmt::Block(stmts))
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(OpenParen, "Expected '(' after if statement", self.lc())?;
        let cond = self.expression()?;
        self.consume(CloseParen, "Expected ')' after if condition", self.lc())?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_tokens(&[Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If(cond, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(OpenParen, "Expected '(' after while statement",
                     self.lc())?;
        let cond = self.expression()?;
        self.consume(CloseParen, "Expected ')' after while condition",
                     self.lc())?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While(cond, body))
    }

    fn for_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(OpenParen, "Expected '(' after for statement", self.lc())?;
        let initializer = if self.match_tokens(&[SemiColon]) {
            None
        } else if self.match_tokens(&[Var]) {
            Some(self.var_decl()?)
        } else {
            Some(self.expr_statement()?)
        };

        let cond = if !self.check(SemiColon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(SemiColon, "Expected ';' after loop condition",
                     self.lc())?;

        let increment = if !self.check(CloseParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(CloseParen, "Expected ')' after for clause", self.lc())?;

        let mut body = self.statement()?;

        if let Some(i) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(i)]);
        }

        body = Stmt::While(cond.unwrap(), Box::new(body));

        if let Some(init) = initializer {
            body = Stmt::Block(vec![init, body]);
        }
        Ok(body)
    }

    fn return_statement(&mut self) -> Result<Stmt, Error> {
        if self.check(SemiColon) {
            self.next();
            return Ok(Stmt::Return(None));
        }
        let expr = self.expression()?;
        self.consume(SemiColon, "Expected ';' after value", self.lc())?;

        Ok(Stmt::Return(Some(expr)))
    }

    fn expr_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(SemiColon, "Expected a ';' after expression", self.lc())?;
        Ok(Stmt::Expression(expr))
    }

    // Expressions ==================================================

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.or()?;

        if self.match_tokens(&[EqualSign]) {
            let _equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { name, ..} => {
                    return Ok(Expr::Assignment {
                        name,
                        expr: Box::new(value),
                    })
                },
                _ => { return Err(Error::new("Invalid assignment target"
                                             .to_string(), self.lc())); }
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.and()?;

        while self.match_tokens(&[Or]) {
            let right = self.and()?;
            expr = Expr::Logical {
                l_expr: Box::new(expr),
                operator: LogicalOp::Or,
                r_expr: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;
        while self.match_tokens(&[And]) {
            let right = self.equality()?;
            expr = Expr::Logical {
                l_expr: Box::new(expr),
                operator: LogicalOp::And,
                r_expr: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[Equals, NEqual]) {
            let op = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;

        while self.match_tokens(&[Greater, GreaterEq, Less, LessEq]) {
            let op = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[Minus, Plus]) {
            let op = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[Divide, Multiply]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_tokens(&[Not, Minus]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            let expr = Expr::Unary {
                op,
                right: Box::new(right),
            };
            return Ok(expr);
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;
        if self.match_tokens(&[OpenParen]) {
            expr = self.finish_call(expr)?;
        }
        Ok(expr)
    }

    fn finish_call(&mut self, expr: Expr) -> Result<Expr, Error> {
        let mut args: Vec<Expr> = Vec::new();
        if !self.check(CloseParen) {
            args.push(self.expression()?);
            while self.match_tokens(&[Comma]) {
                args.push(self.expression()?);
            }
        }
        self.consume(CloseParen, "Expected closing parantheses", self.lc())?;
        Ok(Expr::Call {
            callee: Box::new(expr),
            arguments: args
            })
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_tokens(&[True]) {
            return Ok(Expr::Literal { literal: Literal::True });
        }

        if self.match_tokens(&[False]) {
            return Ok(Expr::Literal { literal: Literal::False });
        }

        if self.match_tokens(&[Nil]) {
            return Ok(Expr::Literal { literal: Literal::Nil });
        }

        if self.match_tokens(&[Number]) {
            return Ok(Expr::Literal { literal: Literal::Number(
                        self.previous().value.parse::<f32>().unwrap() as f64)
            });
        }

        if self.match_tokens(&[StringLiteral]) {
            return Ok(Expr::Literal { literal: Literal::StringLiteral(
                        self.previous().clone().value)
            });
        }

        if self.match_tokens(&[Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().clone()
            });
        }

        if self.match_tokens(&[OpenParen]) {
            let expr = self.expression()?;
            self.consume(CloseParen, "Expected closing parantheses",
                         self.lc())?;
            return Ok(Expr::Grouping { expr: Box::new(expr), });
        }

        self.next();
        Err(Error::new(format!("Error on line: {} at token: {}",
                    self.peek().line_num, self.previous().value), self.lc()))
    }
}
