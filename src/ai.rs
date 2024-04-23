use crate::{
    codegen::{BcArr, Program},
    vm::Interpreter,
    Value, Instr,
};

use rustc_hash::FxHashMap;

/// Memory can be taken from either a pool or register
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum MemVal {
    R(u16),
    P(u16),
}

#[derive(Clone, Debug, Default)]
struct Interval {
    bottom: usize,
    top: usize
}

impl Interval {
    pub fn new(bottom: usize, top: usize) -> Self {
        Self {
            bottom,
            top,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Label(usize);

#[derive(Clone, Debug, Default)]
struct IntervalDomain {
    /// Used to describe the variables at a certain label
    /// Vec<Label, Map<Value, Interval>>
    memory: FxHashMap<Label, FxHashMap<MemVal, Interval>>,
}

impl IntervalDomain {
    fn add(&mut self, label: Label, val: Value, interval: Interval) {
        let mut map = FxHashMap::default();
        let v = match val {
            Value::Reg(v) => {
                MemVal::R(v)
            },
            Value::Pool(v) => {
                MemVal::P(v)
            }
            _ => unreachable!(),
        };
        map.insert(v, interval);
        self.memory.insert(label, map);
    }
}

#[derive(Clone, Debug)]
pub struct AbstractInterpreter {
    /// Holds bytecode that is used to retrieve instructions and operands
    bytecode: Vec<BcArr>,

    /// Holds program counter
    ip: usize,

    domain: IntervalDomain,
}

impl AbstractInterpreter {
    pub fn new(program: &Program) -> Self {
        Self {
            bytecode: program.bytecode.clone(),
            ip: program.entry_point,
            domain: IntervalDomain::default(),
        }
    }

    pub fn run(&mut self) {
        let len = self.bytecode.len();
        while self.ip < len {
            self.handle_label();
        }
    }

    /// Retrieves the next value from the bytecode vector
    fn fetch_val(&mut self) -> BcArr {
        self.ip += 1;
        self.bytecode[self.ip - 1].clone()
    }

    /*
    fn add_new_reg_var(&mut self, register_index: Reg, val: Interval) {
        self.domain.push((register_index, val));
    }
    */

    fn handle_label(&mut self) {
        let label_idx = Label(self.ip);
        let op = self.fetch_val();

        match op {
            BcArr::I(Instr::LoadI) => {
                self.loadi(label_idx);
            }
            /*
            BcArr::I(Instr::PushP) => {
                self.pushp();
            }
            BcArr::I(Instr::LoadP) => {
                self.loadp();
            }
            */
            _ => unreachable!(),
        }

        //println!("{:?}", instr);

    }

    /// Loadi instruction - Loads an immediate value into a register
    fn loadi(&mut self, label: Label) {
        let reg = self.fetch_val();
        let v = self.fetch_val();

        //let register_index = Reg(Interpreter::unpack_register(reg));
        //let val = Interpreter::unpack_number(&Interpreter::unpack_value(v)) as usize;

        //self.add_new_reg_var(register_index, Interval::new(val, val));
    }

    /*
    /// Handle exact same way as Loadi
    fn pushp(&mut self) {
        let pool = self.fetch_val();
        let reg  = self.fetch_val();

        let register_index = Reg(Interpreter::unpack_register(reg));
        let pool_index = Pool(Interpreter::unpack_pool(pool));
        let val = self.domain.memory_reg[register_index.0].1.clone();

        // TODO - this could also be an update I think, not just new
        self.add_new_pool_var(pool_index, val);
    }

    fn loadp(&mut self) {
        let reg = self.fetch_val();
        let pool = self.fetch_val();

        let register_index = Reg(Interpreter::unpack_register(reg));
        let pool_index = Interpreter::unpack_pool(pool);
        let val = self.domain.memory_pool[pool_index].1.clone();

        self.add_new_reg_var(register_index, val);
    }
    */
}
