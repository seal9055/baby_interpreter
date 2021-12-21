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
    pub fn new(program: Program) -> Self {
        Self {
            bytecode: program.bytecode,
            ip: program.entry_point,
            regs: Vec::new(),
            local_pool: Vec::new(),
            function_list: program.function_list,
            const_pool: program.const_pool,
            args: Vec::new(),
            call_stack: Vec::new(),
            flag: false,
        }
    }

    /// Convert ast into bytecodearray
    pub fn interpret(&mut self) -> () {
        let len = self.bytecode.len();
        // Initialize r0 since it is exclusively used as return value for 
        // functions so other operations do not attempt to use it.
        self.regs.push(Value::Number(0.0));

        while self.ip < len {
            self.execute_instr();
        }
    }

    /// Retrieves the next value from the bytecode vector
    fn fetch_val(&mut self) -> BcArr {
        self.ip += 1;
        self.bytecode[self.ip-1].clone()
    }

    /// Inserts value into specified register vector slot
    fn register_insert(&mut self, regid: usize, val: Value) {
        if self.regs.len() > regid {
            self.regs[regid] = val.clone();
        } else {
            self.regs.push(val.clone());
        }
    }

    /// Inserts value into specified local pool vector slot
    fn pool_insert(&mut self, index: usize, val: Value) {
        if self.local_pool.len() > index {
            self.local_pool[index] = val.clone();
        } else {
            self.local_pool.push(val.clone());
        }
    }

    /// Inserts value into specified argument vector slot
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

    /// Unpacks a local pool index from the BcArr enum
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

    /// Unpacks a constant pool index from the BcArr enum
    fn unpack_cpool(reg: BcArr) -> usize {
        extract_enum_value!(reg, BcArr::V(Value::CPool(c)) => c) as usize
    }

    /// Unpacks a number from the Value enum
    fn unpack_number(num: &Value) -> f64 {
        *extract_enum_value!(num, Value::Number(c) => c)
    }

    /// Unpacks a String from the Value enum
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

    /// Decode instruction and execute an appropriate function
    fn execute_instr(&mut self) {
        let instr = self.fetch_val();
        match instr { 
            BcArr::I(Instr::LoadI) => {
                self.loadi();
            },
            BcArr::I(Instr::LoadR) => {
                self.loadr();
            }, 
            BcArr::I(Instr::PushP) => {
                self.pushp();
            }, 
            BcArr::I(Instr::PushA) => {
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
            BcArr::I(Instr::Jmp)   => {
                self.jmp();
            },
            BcArr::I(Instr::Call)  => {
                self.function_call();
            },
            BcArr::I(Instr::JmpIf) => {
                self.jmp_if();
            },
            BcArr::I(Instr::Print) => {
                self.print();
            },
            BcArr::I(Instr::Add)   => {
                self.add();
            },
            BcArr::I(Instr::Sub)   => {
                self.sub();
            },
            BcArr::I(Instr::Mul)   => {
                self.mul();
            },
            BcArr::I(Instr::Div)   => {
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
            BcArr::I(Instr::Ret)   => {
                self.ret();
            },
            _ => { panic!("Runtime Error: Instruction not implemented in vm: \
                          {:?} at IP={}", instr, self.ip); },
        }
    }

    /// Loadi instruction - Loads an immediate value into a register
    fn loadi(&mut self) {
        let reg = self.fetch_val();
        let v   = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let val = Interpreter::unpack_value(v);

        self.register_insert(register_index, val);
    }

    /// Loadr instruction - Loads value from one register into another
    fn loadr(&mut self) {
        let dst = self.fetch_val();
        let src = self.fetch_val();

        let dst_index = Interpreter::unpack_register(dst);
        let src_index = Interpreter::unpack_register(src);
        let val = self.regs[src_index].clone();

        self.register_insert(dst_index, val);
    }

    /// PushP instruction - Push value from register into local pool
    fn pushp(&mut self) {
        let pool = self.fetch_val();
        let reg  = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let pool_index = Interpreter::unpack_pool(pool);
        let val = self.regs[register_index].clone();

        self.pool_insert(pool_index, val);
    }

    /// PushA instruction - Push value from register into argument register
    fn pusha(&mut self) {
        let arg = self.fetch_val();
        let reg = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let args_index = Interpreter::unpack_arg(arg);
        let val = self.regs[register_index].clone();

        self.args_insert(args_index, val);
    }

    /// LoadP instruction - Load value from local pool into a register
    fn loadp(&mut self) {
        let reg  = self.fetch_val();
        let pool = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let pool_index = Interpreter::unpack_pool(pool);
        let val = self.local_pool[pool_index].clone();

        self.register_insert(register_index, val);
    }

    /// LoadA instruction - Load value from an argument register into register
    fn loada(&mut self) {
        let pool  = self.fetch_val();
        let arg   = self.fetch_val();

        let pool_index = Interpreter::unpack_pool(pool);
        let arg_index  = Interpreter::unpack_arg(arg);
        let val = self.args[arg_index].clone();

        self.pool_insert(pool_index, val);
    }

    /// LoadC instruction - Load value from constant pool into a register
    fn loadc(&mut self) {
        let reg  = self.fetch_val();
        let cpool = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let cpool_index = Interpreter::unpack_cpool(cpool);
        let val = self.const_pool[cpool_index].clone();

        self.register_insert(register_index, val);
    }

    /// Jmp if flag is set - Adds VAddr offset to IP
    fn jmp_if(&mut self) {
        let offset: isize = (Interpreter::unpack_vaddr(self.fetch_val())) as isize;
        let mut fake_ip: isize = self.ip as isize;

        if self.flag {
            fake_ip += offset;
            self.ip = fake_ip as usize;
        }
    }

    /// Unconditional jmp - Adds VAddr offset to IP
    fn jmp(&mut self) {
        let offset: isize = (Interpreter::unpack_vaddr(self.fetch_val())) as isize;
        let mut fake_ip: isize = self.ip as isize;
        fake_ip += offset;
        self.ip = fake_ip as usize;
    }

    /// Function Call - set IP to specified VAddr
    fn function_call(&mut self) {
        self.call_stack.push(self.ip + 1);
        let ip: usize = Interpreter::unpack_vaddr(self.fetch_val());
        self.ip = ip;
    }

    /// Return from function by retrieving a value from callstack
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
            Value::Bool(v) => {
                println!("{}", v);
            },
            Value::Nil => {
                println!("NIL");
            },
            _ => { panic!("Runtime Error: Type not implemented in print: {:#?} \
                          at IP={}.", val, self.ip); },
        }
    }

    /// Add instruction
    fn add(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());

        if Interpreter::check_num(&self.regs[r1]) && // num & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 + v2;

            self.register_insert(res, Value::Number(result));
        } else if Interpreter::check_num(&self.regs[r1]) && // num & str
            Interpreter::check_str(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: &str = Interpreter::unpack_string(&self.regs[r2]);
            let result: String = v1.to_string() + &v2;

            self.register_insert(res, Value::StringLiteral(result));
        } else if Interpreter::check_str(&self.regs[r1]) && // str & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: &str = Interpreter::unpack_string(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result: String = v1.to_string() + &v2.to_string();

            self.register_insert(res, Value::StringLiteral(result));
        } else if Interpreter::check_str(&self.regs[r1]) && // str & str
            Interpreter::check_str(&self.regs[r2]) {
            let v1: &str = Interpreter::unpack_string(&self.regs[r1]);
            let v2: &str = Interpreter::unpack_string(&self.regs[r2]);
            let result: String = v1.to_string() + &v2.to_string();

            self.register_insert(res, Value::StringLiteral(result));
        } else {
            panic!("Runtime Error: Add operation not supported for the \
                specified operands at IP={}.", self.ip);
        }
    }

    /// Sub instruction
    fn sub(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        if Interpreter::check_num(&self.regs[r1]) && // num & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 - v2;

            self.register_insert(res, Value::Number(result));
        } else {
            panic!("Runtime Error: Sub operation not supported for the \
                specified operands at IP={}.", self.ip);
        }
    }

    /// Mul instruction
    fn mul(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        if Interpreter::check_num(&self.regs[r1]) && // num & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 * v2;

            self.register_insert(res, Value::Number(result));
        } else {
            panic!("Runtime Error: Mul operation not supported for the \
                specified operands at IP={}.", self.ip);
        }
    }

    /// Div instruction
    fn div(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        if Interpreter::check_num(&self.regs[r1]) && // num & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 / v2;

            self.register_insert(res, Value::Number(result));
        } else {
            panic!("Runtime Error: Div operation not supported for the \
                specified operands at IP={}.", self.ip);
        }
    }

    /// Less instruction
    fn cmp_less_than(&mut self) {
        let res  = Interpreter::unpack_register(self.fetch_val());
        let r1   = Interpreter::unpack_register(self.fetch_val());
        let r2   = Interpreter::unpack_register(self.fetch_val());
        
        if Interpreter::check_num(&self.regs[r1]) && // num & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 < v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("Runtime Error: Both values for 'less than' operation need \
                   to be numbers at IP={}.", self.ip);
        }
    }

    /// LessEq instruction
    fn cmp_less_equal(&mut self) {
        let res  = Interpreter::unpack_register(self.fetch_val());
        let r1   = Interpreter::unpack_register(self.fetch_val());
        let r2   = Interpreter::unpack_register(self.fetch_val());
        
        if Interpreter::check_num(&self.regs[r1]) && // num & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 <= v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("Runtime Error: Both values for 'less than equal' operation \
                    need to be numbers at IP={}.", self.ip);
        }
    }

    /// Greater instruction
    fn cmp_greater_than(&mut self) {
        let res  = Interpreter::unpack_register(self.fetch_val());
        let r1   = Interpreter::unpack_register(self.fetch_val());
        let r2   = Interpreter::unpack_register(self.fetch_val());
        
        if Interpreter::check_num(&self.regs[r1]) && // num & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 > v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("Runtime Error: Both values for 'greater than' operation \
                    need to be numbers at IP={}.", self.ip);
        }
    }

    /// GreaterEq instruction
    fn cmp_greater_equal(&mut self) {
        let res  = Interpreter::unpack_register(self.fetch_val());
        let r1   = Interpreter::unpack_register(self.fetch_val());
        let r2   = Interpreter::unpack_register(self.fetch_val());
        
        if Interpreter::check_num(&self.regs[r1]) && // num & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 >= v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("Runtime Error: Both values for 'greater than equal' \
                   operation need to be numbers at IP={}.", self.ip);
        }
    }

    /// Equals instruction
    fn cmp_equals(&mut self) {
        let res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) { // num & num
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 == v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else if Interpreter::check_num(&self.regs[r1]) && // num & str
            Interpreter::check_str(&self.regs[r2]) {
            let v1: f64  = Interpreter::unpack_number(&self.regs[r1]);
            let v2: &str = Interpreter::unpack_string(&self.regs[r2]);
            self.flag = v1.to_string() == v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else if Interpreter::check_str(&self.regs[r1]) && // str & num
            Interpreter::check_num(&self.regs[r2]) {
            let v1: &str = Interpreter::unpack_string(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            self.flag = v1 == v2.to_string();

            self.register_insert(res, Value::Bool(self.flag));
        } else if Interpreter::check_str(&self.regs[r1]) && // str & str
            Interpreter::check_str(&self.regs[r2]) {
            let v1: &str = Interpreter::unpack_string(&self.regs[r1]);
            let v2: &str = Interpreter::unpack_string(&self.regs[r2]);
            self.flag = v1 == v2;

            self.register_insert(res, Value::Bool(self.flag));
        } else {
            panic!("Runtime Error: Add operation not supported for the \
                specified operands at IP={}.", self.ip);
        }
    }
}
