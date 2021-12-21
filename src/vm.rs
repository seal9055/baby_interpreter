use crate::{ codegen::{Value, Instr, BcArr, Program},
};
use std::collections::HashMap;

/// Macro used to extract known enum variants from enums
#[macro_export]
macro_rules! extract_enum_value {
  ($value:expr, $pattern:pat => $extracted_value:expr) => {
    match $value {
      $pattern => $extracted_value,
      _ => panic!("Pattern doesn't match!"),
    }
  };
}

#[derive(Clone, Debug)]
pub struct Interpreter {
    /// Holds bytecode that is used to retrieve instructions and operands
    bytecode: Vec<BcArr>,

    /// Holds program counter
    ip: usize,

    /// Holds registers
    regs: Vec<Value>,

    /// Holds variables currently in scope
    local_pool: Vec<Value>, 

    /// Holds all declared functions
    function_list: HashMap<String, usize>,

    /// Holds constants
    const_pool: Vec<Value>, 

    /// Used to pass function arguments
    args: Vec<Value>,

    /// Call stack that holds the return values
    call_stack: Vec<usize>,

    /// Flag used to determine conditional jumps
    flag: bool,
}

impl Interpreter {

    /// Returns new interpreter object
    //pub fn new(bytecode: Vec<BcArr>, const_pool: Vec<Value>, entry_point: usize) 
    pub fn new(program: Program) 
            -> Self {
        Self {
            bytecode: program.bytecode,
            const_pool: program.const_pool,
            ip: program.entry_point,
            function_list: program.function_list,
            regs: Vec::new(),
            args: Vec::new(),
            call_stack: Vec::new(),
            local_pool: Vec::new(),
            flag: false,
        }
    }

    /// Convert ast into bytecodearray
    pub fn interpret(&mut self) -> () {
        let len = self.bytecode.len();

        while self.ip < len {
            self.execute_instr();
        }
    }

    /// Retrieves the next value from the bytecode vector
    fn fetch_val(&mut self) -> BcArr {
        self.ip += 1;
        self.bytecode[self.ip-1].clone()
    }

    /// Inserts value into specified register
    fn register_insert(&mut self, regid: usize, val: Value) {
        if self.regs.len() > regid {
            self.regs[regid] = val.clone();
        } else {
            self.regs.push(val.clone());
        }
    }

    /// Inserts value into specified pool_slot
    fn pool_insert(&mut self, index: usize, val: Value) {
        if self.local_pool.len() > index {
            self.local_pool[index] = val.clone();
        } else {
            self.local_pool.push(val.clone());
        }
    }

    /// Inserts value into specified pool_slot
    fn args_insert(&mut self, index: usize, val: Value) {
        if self.args.len() > index {
            self.args[index] = val.clone();
        } else {
            self.args.push(val.clone());
        }
    }

    /// Unpacks a register from the BcArr enum
    fn unpack_register(reg: BcArr) -> usize {
        extract_enum_value!(reg, BcArr::V(Value::Reg(c)) => c) as usize
    }

    /// Unpacks a value from the BcArr enum
    fn unpack_value(val: BcArr) -> Value {
        extract_enum_value!(val, BcArr::V(c) => c)
    }

    /// Unpacks a pool_index from the BcArr enum
    fn unpack_pool(reg: BcArr) -> usize {
        extract_enum_value!(reg, BcArr::V(Value::Pool(c)) => c) as usize
    }

    /// Unpacks a VAddr from the BcArr enum
    fn unpack_vaddr(reg: BcArr) -> usize {
        extract_enum_value!(reg, BcArr::V(Value::VAddr(c)) => c) as usize
    }

    /// Unpacks an argument index from the BcArr enum
    fn unpack_arg(arg: BcArr) -> usize {
        extract_enum_value!(arg, BcArr::V(Value::Arg(c)) => c) as usize
    }

    /// Unpacks a pool_index from the BcArr enum
    fn unpack_cpool(reg: BcArr) -> usize {
        extract_enum_value!(reg, BcArr::V(Value::CPool(c)) => c) as usize
    }

    /// Unpacks a number from the Value enum
    fn unpack_number(num: &Value) -> f64 {
        *extract_enum_value!(num, Value::Number(c) => c)
    }

    /// Unpacks a number from the Value enum
    fn unpack_string(val: &Value) -> &str {
        extract_enum_value!(val, Value::StringLiteral(c) => c)
    }

    /// Checks if provided value is of type number
    fn check_num(v: &Value) -> bool {
        matches!(v, Value::Number(_)) 
    }

    /// Checks if provided value is of type StringLiteral
    fn check_str(v: &Value) -> bool {
        matches!(v, Value::StringLiteral(_)) 
    }

    /// Switch to determine the instruction and execute the appropriate function
    fn execute_instr(&mut self) {
        let instr = self.fetch_val();
        //println!("{}  {:?}", self.ip, instr);
        match instr { 
            BcArr::I(Instr::LoadI) => {
                self.loadi();
            }, BcArr::I(Instr::PushP) => {
                self.pushp();
            }, BcArr::I(Instr::PushA) => {
                self.pusha();
            },
            BcArr::I(Instr::LoadP) => {
                self.loadp();
            },
            BcArr::I(Instr::LoadA) => {
                self.loada();
            },
            BcArr::I(Instr::LoadC) => {
                self.loadc();
            },
            BcArr::I(Instr::Jmp) => {
                self.jmp_unconditional();
            },
            BcArr::I(Instr::Call) => {
                self.function_call();
            },
            BcArr::I(Instr::JmpIf) => {
                self.jmp_if();
            },
            BcArr::I(Instr::Print) => {
                self.print();
            },
            BcArr::I(Instr::Add) => {
                self.add();
            },
            BcArr::I(Instr::Sub) => {
                self.sub();
            },
            BcArr::I(Instr::Mul) => {
                self.mul();
            },
            BcArr::I(Instr::Div) => {
                self.div();
            },
            BcArr::I(Instr::CmpLT) => {
                self.cmp_less_than();
            },
            BcArr::I(Instr::CmpLE) => {
                self.cmp_less_equal();
            },
            BcArr::I(Instr::CmpGT) => {
                self.cmp_greater_than();
            },
            BcArr::I(Instr::CmpGE) => {
                self.cmp_greater_equal();
            },
            BcArr::I(Instr::CmpEq) => {
                self.cmp_equals();
            },
            BcArr::I(Instr::Ret) => {
                self.ret();
            },
            _ => { panic!("Instruction not implemented in vm: {:?}", instr); },
        }
    }

    /// Loadi instruction
    fn loadi(&mut self) {
        let reg = self.fetch_val();
        let v   = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let val = Interpreter::unpack_value(v);

        self.register_insert(register_index, val);
    }

    /// PushP instruction
    fn pushp(&mut self) {
        let pool = self.fetch_val();
        let reg  = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let pool_index = Interpreter::unpack_pool(pool);
        let val = self.regs[register_index].clone();

        self.pool_insert(pool_index, val);
    }

    /// PushA instruction
    fn pusha(&mut self) {
        let arg = self.fetch_val();
        let reg = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let args_index = Interpreter::unpack_arg(arg);
        let val = self.regs[register_index].clone();

        self.args_insert(args_index, val);
    }

    /// LoadP instruction
    fn loadp(&mut self) {
        let reg  = self.fetch_val();
        let pool = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let pool_index = Interpreter::unpack_pool(pool);
        let val = self.local_pool[pool_index].clone();

        self.register_insert(register_index, val);
    }

    /// LoadA instruction
    fn loada(&mut self) {
        let pool  = self.fetch_val();
        let arg   = self.fetch_val();

        let pool_index = Interpreter::unpack_pool(pool);
        let arg_index  = Interpreter::unpack_arg(arg);
        let val = self.args[arg_index].clone();

        self.pool_insert(pool_index, val);
    }

    /// LoadP instruction
    fn loadc(&mut self) {
        let reg  = self.fetch_val();
        let cpool = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let cpool_index = Interpreter::unpack_cpool(cpool);
        let val = self.const_pool[cpool_index].clone();

        self.register_insert(register_index, val);
    }

    /// Jmp if flag is set
    fn jmp_if(&mut self) {
        let offset: isize = (Interpreter::unpack_vaddr(self.fetch_val())) as isize;
        let mut fake_ip: isize = self.ip as isize;

        if self.flag {
            fake_ip += offset;
            self.ip = fake_ip as usize;
        }
    }

    /// Unconditional jmp
    fn jmp_unconditional(&mut self) {
        let offset: isize = (Interpreter::unpack_vaddr(self.fetch_val())) as isize;
        let mut fake_ip: isize = self.ip as isize;
        fake_ip += offset;
        self.ip = fake_ip as usize;
    }

    /// Function Call
    fn function_call(&mut self) {
        self.call_stack.push(self.ip + 1);
        let ip: usize = Interpreter::unpack_vaddr(self.fetch_val());
        self.ip = ip;
    }

    /// Return from function
    fn ret(&mut self) {
        self.ip = self.call_stack.pop().unwrap();
    }

    /// Print instruction
    fn print(&mut self) {
        let reg = self.fetch_val();
        let register_index = Interpreter::unpack_register(reg);
        let val = &self.regs[register_index];

        match val {
            Value::Number(v) => {
                println!("{}", v);
            },
            Value::StringLiteral(v) => {
                println!("{}", v);
            },
            _ => { panic!("Type not implemented in print: {:#?}", val); },
        }
    }

    /// Add instruction
    fn add(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());

        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 + v2;

            self.register_insert(res, Value::Number(result));
        } else if Interpreter::check_num(&self.regs[r1]) &&
            Interpreter::check_str(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: &str = Interpreter::unpack_string(&self.regs[r2]);
            let result: String = v1.to_string() + &v2;

            self.register_insert(res, Value::StringLiteral(result));
        } else if Interpreter::check_str(&self.regs[r1]) &&
            Interpreter::check_num(&self.regs[r2]) {
            let v1: &str = Interpreter::unpack_string(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result: String = v1.to_string() + &v2.to_string();

            self.register_insert(res, Value::StringLiteral(result));
            }
    }

    /// Sub instruction
    fn sub(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 - v2;

            self.register_insert(res, Value::Number(result));
        }
    }

    /// Mul instruction
    fn mul(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 * v2;

            self.register_insert(res, Value::Number(result));
        }
    }

    /// Div instruction
    fn div(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 / v2;

            self.register_insert(res, Value::Number(result));
        }
    }

    /// Less instruction
    fn cmp_less_than(&mut self) {
        let res  = Interpreter::unpack_register(self.fetch_val());
        let r1   = Interpreter::unpack_register(self.fetch_val());
        let r2   = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 < v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("Both values for 'less' operator need to be numbers");
        }
    }

    /// LessEq instruction
    fn cmp_less_equal(&mut self) {
        let res  = Interpreter::unpack_register(self.fetch_val());
        let r1   = Interpreter::unpack_register(self.fetch_val());
        let r2   = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 <= v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("Both values for 'less_eq' operator need to be numbers");
        }
    }

    /// Greater instruction
    fn cmp_greater_than(&mut self) {
        let res  = Interpreter::unpack_register(self.fetch_val());
        let r1   = Interpreter::unpack_register(self.fetch_val());
        let r2   = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 > v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("Both values for 'greater' operator need to be numbers");
        }
    }

    /// GreaterEq instruction
    fn cmp_greater_equal(&mut self) {
        let res  = Interpreter::unpack_register(self.fetch_val());
        let r1   = Interpreter::unpack_register(self.fetch_val());
        let r2   = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 >= v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("Both values for 'greater_eq' operator need to be numbers");
        }
    }

    /// Equals instruction
    fn cmp_equals(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 == v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("'equals' not yet implemented for non-numbers");
        }
    }
}
