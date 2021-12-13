use crate::{
    codegen::{Value, Instr, Vars, BcArr},
};

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

    // Holds variables currently in scope
    //constant_pool: Vec<Vars>, 
}

impl Interpreter {

    /// Returns new interpreter object
    pub fn new(bytecode: Vec<BcArr>) -> Self {
        Self {
            bytecode: bytecode,
            ip: 0,
            regs: Vec::new(),
            local_pool: Vec::new(),
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
        match instr { 
            BcArr::I(Instr::LoadI) => {
                self.loadi();
            }, BcArr::I(Instr::PushP) => {
                self.pushp();
            },
            BcArr::I(Instr::LoadP) => {
                self.loadp();
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
            _ => { panic!("Instruction not implemented in vm: {:?}", instr); },
        }
    }

    /// Loadi instruction
    fn loadi(&mut self) {
        let _reg = self.fetch_val();
        let val = self.fetch_val();

        //let register_index = Interpreter::unpack_register(reg);
        let value = Interpreter::unpack_value(val);

        self.regs.push(value);
    }

    /// PushP instruction
    fn pushp(&mut self) {
        let pool = self.fetch_val();
        let reg  = self.fetch_val();

        let register_index = Interpreter::unpack_register(reg);
        let pool_index = Interpreter::unpack_pool(pool);
        let val = &self.regs[register_index];

        if self.local_pool.len() > pool_index {
            self.local_pool[pool_index] = val.clone();
        } else {
            self.local_pool.push(val.clone());
        }
    }

    /// LoadP instruction
    fn loadp(&mut self) {
        let reg  = self.fetch_val();
        let pool = self.fetch_val();

        let pool_index = Interpreter::unpack_pool(pool);
        let val = &self.local_pool[pool_index];

        self.regs.push(val.clone());
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
            _ => { panic!("Type not implemented in print"); },
        }
    }

    /// Add instruction
    fn add(&mut self) {
        let _res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 + v2;

            self.regs.push(Value::Number(result));
        } else if Interpreter::check_num(&self.regs[r1]) &&
            Interpreter::check_str(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: &str = Interpreter::unpack_string(&self.regs[r2]);
            let result: String = v1.to_string() + &v2;

            self.regs.push(Value::StringLiteral(result));
            } else if Interpreter::check_str(&self.regs[r1]) &&
            Interpreter::check_num(&self.regs[r2]) {
            let v1: &str = Interpreter::unpack_string(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result: String = v1.to_string() + &v2.to_string();

            self.regs.push(Value::StringLiteral(result));
            }
    }

    /// Sub instruction
    fn sub(&mut self) {
        let _res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 - v2;

            self.regs.push(Value::Number(result));
        }
    }

    /// Mul instruction
    fn mul(&mut self) {
        let _res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 * v2;

            self.regs.push(Value::Number(result));
        }
    }

    /// Div instruction
    fn div(&mut self) {
        let _res = Interpreter::unpack_register(self.fetch_val());
        let r1  = Interpreter::unpack_register(self.fetch_val());
        let r2  = Interpreter::unpack_register(self.fetch_val());
        
        // if both r1 and r2 hold numbers
        if Interpreter::check_num(&self.regs[r1]) && 
            Interpreter::check_num(&self.regs[r2]) {
            let v1: f64 = Interpreter::unpack_number(&self.regs[r1]);
            let v2: f64 = Interpreter::unpack_number(&self.regs[r2]);
            let result = v1 / v2;

            self.regs.push(Value::Number(result));
        }
    }
}
