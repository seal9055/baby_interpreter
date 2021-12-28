use crate::tokens::TokenType::*;
use crate::tokens::{Token, TokenType};

/// Returns true if the name is a valid keyword
fn is_keyword(word: &str) -> bool {
    matches!(word, "and" | "else" | "false" | "function" | "for" | 
             "if" | "nil" | "or" | "return" | "this" | 
             "true" | "var" | "let" | "while" | "console.log")
}

/// Returns correct token for provided keyword
fn get_keyword(word: &str) -> TokenType {
    match word {
        "and"         => And,
        "else"        => Else,
        "false"       => False,
        "function"    => Function,
        "for"         => For,
        "if"          => If,
        "nil"         => Nil,
        "or"          => Or,
        "console.log" => Print,
        "return"      => Return,
        "this"        => This,
        "true"        => True,
        "var"         => Var,
        "let"         => Let,
        "while"       => While,
        _             => Whitespace
    }
}

/// Returns true if c == number
fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => {
            true
        },
        _ => { 
            false
        }
    }
}

/// Parses the file and create tokens
pub fn tokenize(file: &str) -> Vec<Token>  {

    #[allow(unused_mut)]
    let mut tokens = vec![];

    // Initialize a base token that will be changed to represent each individual
    // token and pushed to the tokens vector whenever a token is completed
    let mut cur_token: Token = Token {
        t_type: Whitespace, 
        value: "".to_string(), 
        line_num: 1
    };

    let mut lexer = file.chars().collect::<Vec<_>>().into_iter().peekable();

    while let Some(c) = lexer.next() {
        match c {
            // Handle single character tokens
            '(' | ')' | '{' | '}' | ',' |
            '.' | '-' | '+' | ';' | '*' => {
                match c {
                    '(' => cur_token.t_type = OpenParen,
                    ')' => cur_token.t_type = CloseParen,
                    '{' => cur_token.t_type = OpenCurly,
                    '}' => cur_token.t_type = CloseCurly,
                    ',' => cur_token.t_type = Comma,
                    '.' => cur_token.t_type = Dot,
                    '-' => cur_token.t_type = Minus,
                    '+' => cur_token.t_type = Plus,
                    ';' => cur_token.t_type = SemiColon,
                    '*' => cur_token.t_type = Multiply,
                    _ => { panic!("unreachable"); }
                }
                cur_token.value.push(c);
                end_token(&mut cur_token, &mut tokens);
            },
            // Skip comments
            '/' => { 
                if *lexer.peek().unwrap() == '/' {
                    while *lexer.peek().unwrap() != '\n'  && 
                            *lexer.peek().unwrap() != '\r' {
                        lexer.next();
                    }
                    lexer.next();
                    cur_token.line_num += 1;
                } else if *lexer.peek().unwrap() == '*' {
                    loop {
                        if *lexer.peek().unwrap() == '\n' || 
                            *lexer.peek().unwrap() == '\r' {
                            cur_token.line_num += 1;
                            lexer.next();
                        } else if lexer.next().unwrap() == '*' &&
                                   *lexer.peek().unwrap() == '/' {
                            lexer.next();
                            break;
                        }
                    }
                } else {
                    cur_token.t_type = Divide;
                    cur_token.value.push(c);
                    end_token(&mut cur_token, &mut tokens);
                }
            },
            // Create StringLiteral's
            '"' => {
                cur_token.t_type = StringLiteral;
                let mut d = lexer.next().unwrap();
                while d != '"' {
                    cur_token.value.push(d);
                    d = lexer.next().unwrap();
                }
                end_token(&mut cur_token, &mut tokens);
            },
            // Escape Characters
            '\n' | '\r' => {
                end_token(&mut cur_token, &mut tokens);
                cur_token.line_num +=1;
            },
            '\t' | ' ' => {
                end_token(&mut cur_token, &mut tokens);
            },
            '=' => {
                if *lexer.peek().unwrap() == '=' {
                    end_token(&mut cur_token, &mut tokens);
                    cur_token.value.push(c);
                    cur_token.value.push(c);
                    cur_token.t_type = Equals;
                    lexer.next();
                    end_token(&mut cur_token, &mut tokens);
                } else {
                    end_token(&mut cur_token, &mut tokens);
                    cur_token.t_type = EqualSign;
                    cur_token.value.push(c);
                    end_token(&mut cur_token, &mut tokens);
                }
            },
            '>' => {
                    end_token(&mut cur_token, &mut tokens);
                    cur_token.value.push(c);
                if *lexer.peek().unwrap() == '=' {
                    cur_token.value.push('=');
                    cur_token.t_type = GreaterEq;
                    lexer.next();
                } else {
                    cur_token.t_type = Greater;
                }
                end_token(&mut cur_token, &mut tokens);
            },
            '<' => {
                    end_token(&mut cur_token, &mut tokens);
                    cur_token.value.push(c);
                if *lexer.peek().unwrap() == '=' {
                    cur_token.value.push('=');
                    cur_token.t_type = LessEq;
                    lexer.next();
                } else {
                    cur_token.t_type = Less;
                }
                end_token(&mut cur_token, &mut tokens);
            },
            '!' => {
                if *lexer.peek().unwrap() == '=' {
                    end_token(&mut cur_token, &mut tokens);
                    cur_token.value.push(c);
                    cur_token.value.push('=');
                    cur_token.t_type = NEqual;
                    lexer.next();
                    end_token(&mut cur_token, &mut tokens);
                } else {
                    end_token(&mut cur_token, &mut tokens);
                    cur_token.t_type = Not;
                    cur_token.value.push(c);
                    end_token(&mut cur_token, &mut tokens);
                }
            },
            '&' => {
                if *lexer.peek().unwrap() == '&' {
                    end_token(&mut cur_token, &mut tokens);
                    cur_token.value.push(c);
                    cur_token.value.push('&');
                    cur_token.t_type = And;
                    lexer.next();
                    end_token(&mut cur_token, &mut tokens);
                }
            },
            '|' => {
                if *lexer.peek().unwrap() == '|' {
                    end_token(&mut cur_token, &mut tokens);
                    cur_token.value.push(c);
                    cur_token.value.push('|');
                    cur_token.t_type = Or;
                    lexer.next();
                    end_token(&mut cur_token, &mut tokens);
                }
            },
            '0'..='9' => {
                let mut is_float = false;
                cur_token.t_type = Number;
                cur_token.value.push(c);
                while is_digit(*lexer.peek().unwrap()) || 
                               *lexer.peek().unwrap() == '.' {
                    if *lexer.peek().unwrap() == '.' {
                        if !is_float {
                            is_float = true;
                        } else { 
                            panic!("2 dots is invalid syntax");
                        }
                    }
                    let c = lexer.next().unwrap();
                    cur_token.value.push(c);
                }
                end_token(&mut cur_token, &mut tokens);
            },
            'A'..='z' => {
                end_token(&mut cur_token, &mut tokens);
                cur_token.value.push(c);
                cur_token.t_type = Identifier;
                while char::is_alphanumeric(*lexer.peek().unwrap()) || 
                        *lexer.peek().unwrap() == '.' ||
                        *lexer.peek().unwrap() == '_' {
                    let d = lexer.next().unwrap();
                    cur_token.value.push(d);
                }
                if is_keyword(&cur_token.value) {
                    cur_token.t_type = get_keyword(&cur_token.value);
                }
                end_token(&mut cur_token, &mut tokens);
            },
            _ => {},
        }
    }
    end_token(&mut cur_token, &mut tokens);
    cur_token.t_type = Eof;
    tokens.push(cur_token);
    tokens
}

/// Terminate a token and add it to the Token vector before
/// resetting the token to a fresh state
fn end_token(token: &mut Token, tokens: &mut Vec<Token>) {

    if !matches!(token.t_type, Whitespace) {
        tokens.push(token.clone());
    }
    token.t_type = Whitespace; 
    token.value.clear();
}
