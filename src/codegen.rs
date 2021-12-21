use crate::{
    tokens::TokenType::*,
    tokens::{Token},
    ast::{Stmt, Expr, Literal, Expr::Variable},
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    Bool(bool),
    StringLiteral(String),
    Reg(u16),
    Pool(u16),
    CPool(usize),
    VAddr(isize),
    Arg(usize),
}

#[derive(Debug, Copy, Clone)]
pub enum Instr {
    // Load Immediate into register
    LoadI,

    // Load value from one register into another register
    LoadR,

    // Load a value from the local variable stack
    LoadP,

    // Load a value from the arguments (used after function calls)
    LoadA,

    // Push a value to the local variable stack
    PushP,

    // Push a value to the arguments stack
    PushA,

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

    // Compare if r1 < r2 and set flag accordingly
    CmpLT,

    // Compare if r1 <= r2 and set flag accordingly
    CmpLE,

    // Compare if r1 > r2 and set flag accordingly
    CmpGT,

    // Compare if r1 >= r2 and set flag accordingly
    CmpGE,

    // Compare if r1 == r2 and set flag accordingly
    CmpEq,

    // Jump if flag is set using offset relative to IP
    JmpIf,

    // Unconditional jump using offset relative to IP
    Jmp,

    // Call function at provided address
    Call,

    // Return from function call
    Ret,

    // Builtin - print r1 to console
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

#[derive(Debug, Clone)]
pub struct Program {
    pub bytecode: Vec<BcArr>,

    pub entry_point: usize,

    pub function_list: HashMap<String, usize>,

    pub const_pool: Vec<Value>,
}

pub struct Codegen {
    /// Holds bytecode that is later passed on to interpreter
    pub bytecode: Vec<BcArr>,

    /// Pool of constant global variables
    const_pool: Vec<Value>,

    /// Incremented for each value added to constant pool
    const_counter: usize,

    /// List of all functions in the program
    function_list: HashMap<String, usize>,

    /// Increments for each new virtual register
    reg_counter: u16,

    /// Holds current depth counter used for scoping
    cur_depth: u8,

    /// Pool of local variables
    pool: Vec<Vars>,

    /// Entrypoint within bytecode array (necessary because no main function is 
    /// used)
    entry_point: Option<usize>,
}

impl Codegen {

    /// Convert ast into bytecodearray
    pub fn bytecode_gen(ast: Vec<Stmt>) -> Program {
        let mut codegen = Codegen {
            bytecode: Vec::new(),
            const_pool: Vec::new(),
            const_counter: 0,
            function_list: HashMap::new(),
            reg_counter: 1,
            cur_depth: 0,
            pool: Vec::new(),
            entry_point: None,
        };

        for node in ast {
            codegen.interpret_node(&node);
        }

        match codegen.entry_point {
            Some(v) => { 
                Program {
                    bytecode: codegen.bytecode, 
                    entry_point: v,
                    function_list: codegen.function_list,
                    const_pool: codegen.const_pool,
                }
            },
            None    => { panic!(
                            "Runtime Error: Could not determine entry point"); }
        }
    }

    /// Emit instructions
    fn emit_instr(&mut self, instr: BcArr, r1: BcArr, r2: BcArr, res: BcArr) {
        // Set entrypoint on first instruction outside of a function/block
        if self.cur_depth == 0 && self.entry_point == None { 
            self.entry_point = Some(self.bytecode.len());
        }
        match instr {
            BcArr::I(Instr::LoadI) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::LoadR) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::LoadP) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::LoadA) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::PushP) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::PushA) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::LoadC) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::Print) => {
                self.bytecode.push(instr);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::Add) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
            },
            BcArr::I(Instr::Sub) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
            },
            BcArr::I(Instr::Mul) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
            },
            BcArr::I(Instr::Div) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
            },
            BcArr::I(Instr::CmpLT) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
            },
            BcArr::I(Instr::CmpLE) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
            },
            BcArr::I(Instr::CmpGT) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
            },
            BcArr::I(Instr::CmpGE) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
            },
            BcArr::I(Instr::CmpEq) => {
                self.bytecode.push(instr);
                self.bytecode.push(res);
                self.bytecode.push(r1);
                self.bytecode.push(r2);
            },
            BcArr::I(Instr::Jmp) => {
                self.bytecode.push(instr);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::JmpIf) => {
                self.bytecode.push(instr);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::Call) => {
                self.bytecode.push(instr);
                self.bytecode.push(r1);
            },
            BcArr::I(Instr::Ret) => {
                self.bytecode.push(instr);
            },
            _ => { panic!("Runtime Error: Unimplemented Instruction: {:?}",
                          instr); },
        }
    }

    /// Match different kinds of statements
    fn interpret_node(&mut self, node: &Stmt) {
        match node.clone() {
            Stmt::Function(n, a, e) => { self.function_decl(n, a, e); },
            Stmt::Expression(e)     => { self.expression(e);          },
            Stmt::Variable(n, e)    => { self.assignment(n, e);       },
            Stmt::Block(s)          => { self.block(s);               },
            Stmt::If(e, t, f)       => { self.if_stmt(e, t, f);       },
            Stmt::Return(e)         => { self.ret(e);                 },
            Stmt::While(e, b)       => { self.while_stmt(e, b);       },
            Stmt::Print(e)          => { self.print(e);               },
        }
    }

    /// Return next free virtual register
    fn get_next_reg(&mut self) -> u16 {
        let ret = self.reg_counter;
        self.reg_counter += 1;
        ret
    }

    /// Return next free slot in constant pool
    fn get_next_const(&mut self) -> usize {
        let ret = self.const_counter;
        self.const_counter += 1;
        ret
    }

    /// Return index of value from pool given name
    fn get_pool(&mut self, name: &str) -> u16 {
        let arr: Vec<Vars> = self.pool.clone().into_iter()
            .filter(|v| v.name.clone() == name).collect();
        if arr.is_empty() { panic!("Runtime Error: Variable does not exist"); }
        let max = arr.iter().map(|v| v.depth).max().unwrap();
        let index = self.pool.iter().position(|v| v.depth == max 
                                              && v.name == name).unwrap();
        index as u16
    }

    /// Removes every variable at current depth level
    fn clear_depth(&mut self, depth: u8) {
        self.pool.retain(|v| v.depth != depth);
    }

    /// Interpret if statements
    fn if_stmt(&mut self, expr: Expr, t: Box<Stmt>, f: Option<Box<Stmt>>) {

        // Sets flag to true/false depending on expression result
        self.expression(expr);
        let tmp = self.reg_counter;
        let offset1 = self.bytecode.len() + 1;

        self.emit_instr(BcArr::I(Instr::JmpIf), 
                        BcArr::V(Value::VAddr(0)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Nil));

        if let Some(x) = f { self.interpret_node(&*x); }

        let offset2 = self.bytecode.len() + 1;
        self.emit_instr(BcArr::I(Instr::Jmp), 
                        BcArr::V(Value::VAddr(0)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Nil));

        let jmp_1: isize = (self.bytecode.len() - offset1 - 1) as isize;
        self.reg_counter = tmp;
        self.interpret_node(&*t); // Interpret true block
        let jmp_2: isize = (self.bytecode.len() - offset2 - 1) as isize;

        // Patch in correct offsets after calculating them
        self.bytecode[offset1] = BcArr::V(Value::VAddr(jmp_1));
        self.bytecode[offset2] = BcArr::V(Value::VAddr(jmp_2));
    }

    /// Interpret while statements
    fn while_stmt(&mut self, expr: Expr, b: Box<Stmt>) {
        let tmp_reg = self.reg_counter;
        let offset  = self.bytecode.len() + 1;

        self.emit_instr(BcArr::I(Instr::Jmp), 
                        BcArr::V(Value::VAddr(0)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Nil));

        self.interpret_node(&*b);
        self.reg_counter = tmp_reg;
        self.expression(expr);
        let jmp1: isize = (self.bytecode.len() - offset + 1) as isize;

        self.emit_instr(BcArr::I(Instr::JmpIf), 
                        BcArr::V(Value::VAddr(-jmp1)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Nil));
        let jmp2: isize = (self.bytecode.len() - offset - 13) as isize;

        // Patch in correct offset after calculating it
        self.bytecode[offset] = BcArr::V(Value::VAddr(jmp2));
    }

    /// Interpret a block of code while maintaining proper scopes
    fn block(&mut self, stmts: Vec<Stmt>) {
        self.cur_depth += 1;
        for stmt in stmts.iter() {
            self.interpret_node(stmt);
        }
        self.clear_depth(self.cur_depth);
        self.cur_depth -= 1;
    }

    /// If the function attempts to return a value, load it into r0
    fn ret(&mut self, expr: Option<Expr>) {
        match expr {
            Some(e) => { 
                let v = self.expression(e);
                self.emit_instr(BcArr::I(Instr::LoadR), 
                        BcArr::V(Value::Reg(v)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Reg(0)));
            },
            None    => { 
                self.emit_instr(BcArr::I(Instr::LoadI), 
                        BcArr::V(Value::Number(0.0)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Reg(0)));
            },
        }
    }

    /// Helper to add a new function to the list of functions
    fn register_function(&mut self, name: String, pos: usize) {
        if self.function_list.contains_key(&name) {
            panic!("Runtime Error: Cannot redeclare function with name: {}", 
                   name); 
        }
        self.function_list.insert(name, pos);
    }

    /// Generate code for function declarations
    fn function_decl(&mut self, name: Token, args: Vec<Token>, 
            code: Vec<Stmt>) {
        let name    = name.value;
        let tmp_reg = self.reg_counter;
        let pos     = self.bytecode.len();

        self.register_function(name, pos);

        // depth increased to mirror depth of function block
        self.cur_depth += 1;
        for (i, arg) in args.into_iter().enumerate() {
            self.pool.push(Vars { name: arg.value, depth: self.cur_depth });
            self.emit_instr(BcArr::I(Instr::LoadA), 
                        BcArr::V(Value::Arg(i)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Pool((self.pool.len() - 1) as u16 )));
        };
        self.cur_depth -= 1;
        self.block(code);
        self.cur_depth += 1;
        
        self.emit_instr(BcArr::I(Instr::Ret), 
                    BcArr::V(Value::Nil), 
                    BcArr::V(Value::Nil), 
                    BcArr::V(Value::Nil));

        self.cur_depth -= 1;
        self.reg_counter = tmp_reg;
    }

    /// Builtins, currently only supports console.log()
    fn print(&mut self, expr: Expr) {
        let e = self.expression(expr);
        self.emit_instr(BcArr::I(Instr::Print), 
                        BcArr::V(Value::Reg(e)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Nil));
    }

    /// Emit instructions for variable assignment
    fn assignment(&mut self, name: Token, expr: Option<Expr>) -> u16 {
        let e = self.expression(expr.unwrap());
        let depth = self.cur_depth;
        let var = Vars { name: name.value.clone(), depth };

        if self.pool.contains(&var) { 
            panic!("Runtime Error: Cannot redeclare already existing variable"); 
        }
        self.pool.push( var );
        let index: u16 = self.get_pool(&name.value);
        self.emit_instr(BcArr::I(Instr::PushP), 
                        BcArr::V(Value::Reg(e)), 
                        BcArr::V(Value::Nil), 
                        BcArr::V(Value::Pool(index)));
        index
    }

    /// Emit instructions for expressions and return result register
    fn expression(&mut self, expr: Expr) -> u16 {
        let mut res = 0;
        match expr.clone() {
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
                    Minus       => { 
                        self.emit_instr(BcArr::I(Instr::Sub), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    Divide      => { 
                        self.emit_instr(BcArr::I(Instr::Div), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    Multiply    => { 
                        self.emit_instr(BcArr::I(Instr::Mul), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    Less        => { 
                        self.emit_instr(BcArr::I(Instr::CmpLT), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    LessEq        => { 
                        self.emit_instr(BcArr::I(Instr::CmpLE), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    Greater        => { 
                        self.emit_instr(BcArr::I(Instr::CmpGT), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    GreaterEq        => { 
                        self.emit_instr(BcArr::I(Instr::CmpGE), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    Equals        => { 
                        self.emit_instr(BcArr::I(Instr::CmpEq), 
                            BcArr::V(Value::Reg(r1)), 
                            BcArr::V(Value::Reg(r2)), 
                            BcArr::V(Value::Reg(res))); 
                    },
                    _ => { panic!("Runtime Error: Operator not supported: {:#?}"
                                  , expr); },
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
                    _ => { panic!("Runtime ErrorLiteral type not implemented"); 
                    },
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
                res = self.expression(*expr);
            },
            Expr::Assignment { name, expr } => {
                let s = name.value;
                let register_index = self.expression(*expr);
                let pool_index = self.get_pool(&s);

                self.emit_instr(BcArr::I(Instr::PushP), 
                                BcArr::V(Value::Reg(register_index)), 
                                BcArr::V(Value::Nil), 
                                BcArr::V(Value::Pool(pool_index)));
            },
            Expr::Call { callee, arguments } => {
                let pos;
                // Figure out position of called function
                match *callee {
                    Variable { name } => {
                        pos = match self.function_list.get(&name.value) {
                            Some(v) => { *v as isize },
                            None    => { 
                                panic!("Runtime Error: function: '{}' that you \
                                    attempt to call on line {} does not exist",
                                       name.value, name.line_num);
                            },
                        };
                    },
                _ => { panic!("Runtime Error: Error during call"); },
                }

                // Emit push argument instructions for every argument
                for (i, arg) in arguments.into_iter().enumerate() {
                    let register_index = self.expression(arg);

                    self.emit_instr(BcArr::I(Instr::PushA), 
                                BcArr::V(Value::Reg(register_index)), 
                                BcArr::V(Value::Nil), 
                                BcArr::V(Value::Arg(i)));
                }

                self.emit_instr(BcArr::I(Instr::Call), 
                                BcArr::V(Value::VAddr(pos)), 
                                BcArr::V(Value::Nil), 
                                BcArr::V(Value::Nil));
                // res = 0 because return values are stored in r0
                res = 0; 
            },
            _ => { panic!("Expression not yet implemented in codegen: {:#?}"
                          , expr); },
        }
        res
    }
}
