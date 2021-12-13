use crate::{
    tokens::TokenType::*,
    tokens::{Token},
    ast::{Stmt,Expr, Literal},
};


#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    StringLiteral(String),
    Reg(u16),
    Pool(u16),
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum OpCode {
    Add,
    And,
    Div,
    Equals,
    False,
    Greater,
    GE,
    Inherit,
    Less,
    LE,
    Mul,
    Negate,
    Nil,
    Not,
    Pop,
    Print,
    Return,
    Sub,
    True,
    Or,
    NEqual,
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    // Opcode
    Op(OpCode),

    // Load Immediate into register
    LoadI,

    // Load a value from the constant pool
    LoadP,

    // Push a value to the constant pool
    PushP,

    // Builtin - print
    Print,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vars {
    name: String,
    depth: u8,
}

pub struct Interpreter {
    /// Holds bytecode that is later passed on to interpreter
    pub bytecode: Vec<u8>,

    /// Increments for each new virtual register
    reg_counter: u16,

    /// Holds current depth counter used for scoping
    cur_depth: u8,

    /// Pool of variables
    pool: Vec<Vars>,

    /// Program Counter
    ip: u32,
}

impl Interpreter {

    /// Convert ast into bytecodearray
    pub fn bytecode_gen(ast: Vec<Stmt>) -> Vec<u8> {
        let mut interpreter = Interpreter {
            bytecode: Vec::new(),
            reg_counter: 0,
            cur_depth: 0,
            pool: Vec::new(),
            ip: 0,
        };
        for node in ast {
            interpreter.interpret_node(&node);
        }

        for e in interpreter.pool.iter() {
            println!("\n{:?}", e);
        }

        interpreter.bytecode
    }

    /// Emit instructions
    fn emit_instr(&mut self, 
                  instr: Instr, r1: Value, r2: Value, res: Value) -> () {
        //self.bytecode.instructions.push(op_code);
        match instr {
            Instr::LoadI => {
                //self.bytecode.push(0x01);
                //self.bytecode.push(r1));
                println!("LoadI {:?}, {:?}", res, r1); 
            },
            Instr::LoadP => {
                println!("LoadP {:?}, {:?}", res, r1); 
            },
            Instr::PushP => {
                println!("PushP {:?}, {:?}", res, r1); 
            },
            Instr::Print => {
                println!("Print {:?}", r1); 
            },
            Instr::Op(OpCode::Add) => {
                println!("Add {:?}, {:?}, {:?}", res, r1, r2); 
            },
            Instr::Op(OpCode::Sub) => {
                println!("Sub {:?}, {:?}, {:?}", res, r1, r2); 
            },
            Instr::Op(OpCode::Mul) => {
                println!("Mul {:?}, {:?}, {:?}", res, r1, r2); 
            },
            Instr::Op(OpCode::Div) => {
                println!("Div {:?}, {:?}, {:?}", res, r1, r2); 
            },
            //Instr::Op(OpCode::Greater) => {
            //    println!("Cmpgt {:?}, {:?}, {:?}", res, r1, r2); 
            //},
            //Instr::Op(OpCode::Less) => {
            //    println!("Cmplt {:?}, {:?}, {:?}", res, r1, r2); 
            //},
            //Instr::Op(OpCode::GE) => {
            //    println!("Cmpge {:?}, {:?}, {:?}", res, r1, r2); 
            //},
            //Instr::Op(OpCode::LE) => {
            //    println!("Cmple {:?}, {:?}, {:?}", res, r1, r2); 
            //},
            //Instr::Op(OpCode::Or) => {
            //    println!("Or {:?}, {:?}, {:?}", res, r1, r2); 
            //},
            //Instr::Op(OpCode::And) => {
            //    println!("And {:?}, {:?}, {:?}", res, r1, r2); 
            //},
            //Instr::Op(OpCode::Equals) => {
            //    println!("Cmp {:?}, {:?}, {:?}", res, r1, r2); 
            //},
            //Instr::Op(OpCode::NEqual) => {
            //    println!("NE? {:?}, {:?}, {:?}", res, r1, r2); 
            //},
            _ => { panic!("unimplemented instruction"); }
        }
    }

    /// Match different kinds of statements
    fn interpret_node(&mut self, node: &Stmt) -> () {
        match node.clone() {
            Stmt::Function(_,_,_) => { panic!("FUNC"); },
            Stmt::Expression(e)   => { self.expression(e);    },
            Stmt::Variable(n,e)   => { self.assignment(n, e); },
            Stmt::Block(s)        => { self.block(s);   },
            Stmt::If(_,_,_)       => { panic!("IF"); },
            Stmt::Return(_)       => { panic!("RETURN"); },
            Stmt::While(_,_)      => { panic!("WHILE"); },
            Stmt::Print(e)        => { self.print(e);         },
        }
    }

    /// Return next virtual register by simply incrementing a counter 
    fn get_next_reg(&mut self) -> u16 {
        self.reg_counter += 1;
        self.reg_counter
    }

    /// Return index of value from pool given name
    fn get_pool(&mut self, name: &str) -> u16 {
        let arr: Vec<Vars> = self.pool.clone().into_iter()
            .filter(|v| v.name.clone() == name).collect();
        let max = arr.iter().map(|v| v.depth).max().unwrap_or(0);
        let index = self.pool.iter().position(|v| v.depth == max).unwrap();
        index as u16
    }

    /// Removes every variable at current depth level
    fn clear_depth(&mut self, depth: u8) {
        self.pool.retain(|v| v.depth != depth);
    }

    /// Interpret a block of code
    fn block(&mut self, stmts: Vec<Stmt>) -> () {
        self.cur_depth += 1;
        for stmt in stmts.iter() {
            self.interpret_node(stmt);
        }
        self.clear_depth(self.cur_depth);
        self.cur_depth -= 1;
    }

    /// Builtins, currently only supports print
    fn print(&mut self, expr: Expr) -> () {
        let e = self.expression(expr);
        self.emit_instr(Instr::Print, Value::Reg(e), Value::Nil, Value::Nil);
    }


    /// Emit instructions for variable assignment
    fn assignment(&mut self, name: Token, expr: Option<Expr>) -> u16 {
        let e = self.expression(expr.unwrap());
        let depth = self.cur_depth;
        let var = Vars { name: name.value.clone(), depth: depth };
        if self.pool.contains(&var) {
            panic!("Runtime Error: Cannot redeclare already existing variable");
        }
        self.pool.push( var );
        let index: u16 = self.get_pool(&name.value);
        self.emit_instr(Instr::PushP, Value::Reg(e), Value::Nil, 
                        Value::Pool(index));
        index
    }

    /// Emit instructions for expressions
    fn expression(&mut self, expr: Expr) -> u16 {
        let mut res = 0;
        match expr {
            Expr::Binary {left, op, right } => {
                let r1 = self.expression(*left);
                let r2 = self.expression(*right);
                res = self.get_next_reg();
                match op.t_type {
                    Plus        => { self.emit_instr(Instr::Op(OpCode::Add), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    Minus       => { self.emit_instr(Instr::Op(OpCode::Sub), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    Divide      => { self.emit_instr(Instr::Op(OpCode::Div), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    Multiply    => { self.emit_instr(Instr::Op(OpCode::Mul), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    Greater     => { self.emit_instr(Instr::Op(OpCode::Greater), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    Less        => { self.emit_instr(Instr::Op(OpCode::Less), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    GreaterEqual=> { self.emit_instr(Instr::Op(OpCode::GE), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    LessEqual   => { self.emit_instr(Instr::Op(OpCode::LE), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    Or          => { self.emit_instr(Instr::Op(OpCode::Or), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    And         => { self.emit_instr(Instr::Op(OpCode::And), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    NEqual      => { self.emit_instr(Instr::Op(OpCode::NEqual), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    Equals      => { self.emit_instr(Instr::Op(OpCode::Equals), 
                            Value::Reg(r1), Value::Reg(r2), Value::Reg(res)); },
                    _ => { panic!("Operator not supported"); },
                }
            },
            Expr::Literal { literal } => {
                match literal { 
                    Literal::Number(i) => { 
                        res = self.get_next_reg();
                        self.emit_instr(Instr::LoadI, Value::Number(i), 
                                        Value::Nil, Value::Reg(res)); 
                    },
                    Literal::StringLiteral(s) => {
                        let depth = self.cur_depth;
                        let var = Vars {name: s.clone(), depth: depth};
                        if !self.pool.contains(&var) {  self.pool.push( var ); }
                        let index = self.get_pool(&s);
                        res = self.get_next_reg();

                        self.emit_instr(Instr::PushP, Value::StringLiteral(s), 
                                        Value::Nil, Value::Pool(index)); 

                        self.emit_instr(Instr::LoadP, Value::Pool(index),
                                        Value::Nil, Value::Reg(res));
                    }
                    _ => { panic!("Literal type not implemented"); },
                }
            },
            Expr::Variable { name } => {
                let index = self.get_pool(&name.value);
                res = self.get_next_reg();
                self.emit_instr(Instr::LoadP, Value::Pool(index), 
                                Value::Nil, Value::Reg(res));
            },
            _ => { panic!("Expression not yet implemented in codegen: 
                          {:#?}", expr); },
        }
        res
    }
}
