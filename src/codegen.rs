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
    CPool(usize),
}

#[derive(Debug, Copy, Clone)]
pub enum Instr {
    // Load Immediate into register
    LoadI,

    // Load a value from the local variable stack
    LoadP,

    // Push a value to the local variable stack
    PushP,

    // Load value from constant pool
    LoadC,

    // res = r1 + r2
    Add,

    // res = r1 - r2
    Sub,

    // res = r1 * r2
    Mul,

    // res = r1 / r2
    Div,

    // Builtin - print r1
    Print,
}

#[derive(Debug, Clone)]
pub enum BcArr {
    I(Instr),
    V(Value),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vars {
    name: String,
    depth: u8,
}

pub struct Codegen {
    /// Holds bytecode that is later passed on to interpreter
    pub bytecode: Vec<BcArr>,

    /// Pool of constant global variables
    const_pool: Vec<Value>,

    /// Incremented for each value added to constant pool
    const_counter: usize,

    /// Increments for each new virtual register
    reg_counter: u16,

    /// Holds current depth counter used for scoping
    cur_depth: u8,

    /// Pool of local variables
    pool: Vec<Vars>,
}

impl Codegen {

    /// Convert ast into bytecodearray
    pub fn bytecode_gen(ast: Vec<Stmt>) -> (Vec<BcArr>, Vec<Value>) {
        let mut codegen = Codegen {
            bytecode: Vec::new(),
            const_pool: Vec::new(),
            const_counter: 0,
            reg_counter: 0,
            cur_depth: 0,
            pool: Vec::new(),
        };
        for node in ast {
            codegen.interpret_node(&node);
        }

        //for e in interpreter.pool.iter() {
        //    println!("\n{:?}", e);
        //}

        (codegen.bytecode, codegen.const_pool)
    }

    /// Emit instructions
    fn emit_instr(&mut self, instr: BcArr, 
                  r1: BcArr, r2: BcArr, res: BcArr) -> () {
        match instr {
            BcArr::I(Instr::LoadI) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                //println!("LoadI {:?}, {:?}", res, r1); 
            },
            BcArr::I(Instr::LoadP) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                //println!("LoadP {:?}, {:?}", res, r1); 
            },
            BcArr::I(Instr::PushP) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                //println!("PushP {:?}, {:?}", res, r1); 
            },
            BcArr::I(Instr::LoadC) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                //println!("LoadC {:?}, {:?}", res, r1); 
            },
            BcArr::I(Instr::Print) => {
                self.bytecode.push(instr);
                self.bytecode.push(r1);
                //println!("Print {:?}", r1); 
            },
            BcArr::I(Instr::Add) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
                //println!("Add {:?}, {:?}, {:?}", res, r1, r2); 
            },
            BcArr::I(Instr::Sub) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
                //println!("Sub {:?}, {:?}, {:?}", res, r1, r2); 
            },
            BcArr::I(Instr::Mul) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
                //println!("Mul {:?}, {:?}, {:?}", res, r1, r2); 
            },
            BcArr::I(Instr::Div) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
                //println!("Div {:?}, {:?}, {:?}", res, r1, r2); 
            },
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
        let ret = self.reg_counter;
        self.reg_counter += 1;
        ret
    }

    /// Return next virtual register by simply incrementing a counter 
    fn get_next_const(&mut self) -> usize {
        let ret = self.const_counter;
        self.const_counter += 1;
        ret
    }

    /// Return index of value from pool given name
    fn get_pool(&mut self, name: &str) -> u16 {
        let arr: Vec<Vars> = self.pool.clone().into_iter()
            .filter(|v| v.name.clone() == name).collect();
        let max = arr.iter().map(|v| v.depth).max().unwrap_or(0);
        let index = self.pool.iter().position(|v| v.depth == max 
                                              && v.name == name).unwrap();
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
        self.emit_instr(BcArr::I(Instr::Print), BcArr::V(Value::Reg(e)), 
                        BcArr::V(Value::Nil), BcArr::V(Value::Nil));
    }


    /// Emit instructions for variable assignment
    fn assignment(&mut self, name: Token, expr: Option<Expr>) -> u16 {
        let e = self.expression(expr.unwrap());
        let depth = self.cur_depth;
        let var = Vars { name: name.value.clone(), depth: depth };
        if self.pool.contains(&var) {
            panic!("Runtime Error: Cannot redeclare already existing variable");
        }
        self.pool.push( var.clone() );
        let index: u16 = self.get_pool(&name.value);
        self.emit_instr(BcArr::I(Instr::PushP), 
                        BcArr::V(Value::Reg(e)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Pool(index)));
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
                    Plus        => { 
                        self.emit_instr(BcArr::I(Instr::Add), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    Minus        => { 
                        self.emit_instr(BcArr::I(Instr::Sub), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    Divide        => { 
                        self.emit_instr(BcArr::I(Instr::Div), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    Multiply        => { 
                        self.emit_instr(BcArr::I(Instr::Mul), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    _ => { panic!("Operator not supported"); },
                }
            },
            Expr::Literal { literal } => {
                match literal { 
                    Literal::Number(i) => { 
                        res = self.get_next_reg();
                        self.emit_instr(BcArr::I(Instr::LoadI), 
                                        BcArr::V(Value::Number(i)), 
                                        BcArr::V(Value::Nil), 
                                        BcArr::V(Value::Reg(res))); 
                    },
                    Literal::StringLiteral(s) => {
                        self.const_pool.push(Value::StringLiteral(s));
                        let const_index = self.get_next_const();

                        //let r1 = self.get_next_reg();
                        res = self.get_next_reg();

                        self.emit_instr(BcArr::I(Instr::LoadC), 
                                        BcArr::V(Value::CPool(const_index)), 
                                        BcArr::V(Value::Nil), 
                                        BcArr::V(Value::Reg(res))); 
                    },
                    _ => { panic!("Literal type not implemented"); },
                }
            },
            Expr::Variable { name } => {
                let index = self.get_pool(&name.value);
                res = self.get_next_reg();
                self.emit_instr(BcArr::I(Instr::LoadP), 
                                BcArr::V(Value::Pool(index)), 
                                BcArr::V(Value::Nil), 
                                BcArr::V(Value::Reg(res)));
            },
            Expr::Grouping { expr } => {
                res = self.expression(*expr)
            },
            Expr::Assignment { name, expr } => {
                let s = name.value;
                let register_index = self.expression(*expr);
                let pool_index = self.get_pool(&s);

                self.emit_instr(BcArr::I(Instr::PushP), 
                                BcArr::V(Value::Reg(register_index)), 
                                BcArr::V(Value::Nil), 
                                BcArr::V(Value::Pool(pool_index)));

                //panic!("Name: {:#?} ; expr: {:#?}", s, v);
            }
            _ => { panic!("Expression not yet implemented in codegen: 
                          {:#?}", expr); },
        }
        res
    }
}