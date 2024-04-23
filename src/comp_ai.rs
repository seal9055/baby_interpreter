use crate::{
    codegen::{BcArr, Program, Cfg, Block},
    vm::Interpreter,
    Instr,
};

use rustc_hash::FxHashMap;

/*
#[derive(Clone, Debug, Default)]
struct Reg(usize);

#[derive(Clone, Debug, Default)]
struct Pool(usize);

#[derive(Clone, Debug, Default)]
struct IntervalDomain {
    memory_reg:  Vec<(Reg, Interval)>,
    memory_pool: Vec<(Pool, Interval)>
}

impl IntervalDomain {
    // 
    fn default() -> Self {
        let memory_reg  = vec![(Reg(0), Interval::new(0, 0))];
        let memory_pool = Vec::new();
        Self {
            memory_reg,
            memory_pool
        }
    }
}

#[derive(Clone, Debug, Default)]
struct BoolDomain {
    memory: FxHashMap<usize, BoolState>,
}
*/

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


#[derive(Clone, Debug, Default)]
enum BoolState {
    #[default] Unknown,
    T,
    F,
    Either,
}

#[derive(Clone, Debug)]
enum Mem {
    I(Interval),
    B(BoolState),
}

/// Used to index memory-map, indicating if this is reg or pool-indexed memory
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum MemIdx {
    R(usize),
    P(usize),
}

#[derive(Clone, Debug)]
pub struct AbstractInterpreter {
    /// Holds bytecode that is used to retrieve instructions and operands
    bytecode: Vec<BcArr>,

    /// Holds program counter
    ip: usize,

    /// Interpreter has 2 different types of memory locations (Reg & Pool), both of which can 
    memory: FxHashMap<MemIdx, Mem>,
}

impl AbstractInterpreter {
    pub fn new(program: &Program) -> Self {
        Self {
            bytecode: program.bytecode.clone(),
            ip: program.entry_point,
            memory: FxHashMap::default(),
        }
    }

    pub fn run(&mut self, cfg: &Cfg) {
        let mut block_worklist = vec![0];
        let mut handled_blocks: FxHashMap<usize, usize> = FxHashMap::default();

        while !block_worklist.is_empty() {
            let block_id = block_worklist.remove(0);

            // Don't repeat same block twice
            // This is bad, can only handle 1 loop iteration
            if handled_blocks.get(&block_id).is_some() {
                continue;
            }

            let block = cfg.blocks.get(&block_id).expect("CFG references non-existing block");
            handled_blocks.insert(block_id, 0);
            self.handle_block(&block);
            block.edges.iter().for_each(|e| block_worklist.push(*e));
        }
        for var in &self.memory {
            println!("{:?}", var);
        }
    }

    pub fn handle_block(&mut self, block: &Block) {
        for instr in &block.instrs {
            self.ip = instr.0;
            self.handle_label(instr.0);
        }
    }

    /// Retrieves the next value from the bytecode vector
    fn fetch_val(&mut self) -> BcArr {
        self.ip += 1;
        self.bytecode[self.ip - 1].clone()
    }

    /// Retrieves the next value from the bytecode vector
    fn fetch_val_at(&mut self, ip: usize) -> BcArr {
        self.ip += 1;
        self.bytecode[ip].clone()
    }

    fn advance_ip(&mut self) {
        self.ip += 1;
    }
    fn handle_label(&mut self, ip: usize) {
        let op = self.fetch_val_at(ip);
        println!("Handling: {:?}", op);

        match op {
            BcArr::I(Instr::LoadI) => {
                self.loadi();
            }
            BcArr::I(Instr::PushP) => {
                self.pushp();
            }
            BcArr::I(Instr::LoadP) => {
                self.loadp();
            }
            BcArr::I(Instr::CmpGT) => {
                self.cmpgt();
            }
            BcArr::I(Instr::JmpIf) => {
                self.jmpif();
            }
            BcArr::I(Instr::Jmp) => {
                self.jmp();
            }
            BcArr::I(Instr::Add) => {
                self.add();
            }
            BcArr::I(Instr::Print) => {
            }
            _ => unreachable!(),
        }

        //println!("{:?}", instr);

    }

    /// Loadi instruction - Loads an immediate value into a register
    fn loadi(&mut self) {
        let reg = self.fetch_val();
        let v = self.fetch_val();

        let register_index = MemIdx::R(Interpreter::unpack_register(reg));
        let val = Interpreter::unpack_number(&Interpreter::unpack_value(v)) as usize;

        self.memory.insert(register_index, Mem::I(Interval::new(val, val)));
        //self.add_new_reg_var_int(register_index, Interval::new(val, val));
    }

    /// Handle exact same way as Loadi
    fn pushp(&mut self) {
        let pool = self.fetch_val();
        let reg  = self.fetch_val();

        //println!("{:#?}", self.domain_int);

        let register_index = MemIdx::R(Interpreter::unpack_register(reg));
        let pool_index = MemIdx::P(Interpreter::unpack_pool(pool));
        let val = self.memory.get(&register_index).unwrap().clone();

        // TODO - this could also be an update I think, not just new

        self.memory.insert(pool_index, val);
        //self.add_new_pool_var_int(pool_index, val);
    }

    fn loadp(&mut self) {
        let reg = self.fetch_val();
        let pool = self.fetch_val();

        let register_index = MemIdx::R(Interpreter::unpack_register(reg));
        let pool_index = MemIdx::P(Interpreter::unpack_pool(pool));
        let val = self.memory.get(&pool_index).unwrap().clone();

        self.memory.insert(register_index, val);
        //self.add_new_reg_var_int(register_index, val);
    }

    // Doesn't have to do anything for now
    fn cmpgt(&mut self) {
        let res = self.fetch_val();
        let _r1  = self.fetch_val();
        let _r2  = self.fetch_val();

        let register_index = MemIdx::R(Interpreter::unpack_register(res));
        //self.add_new_var_bool(register_index, BoolState::Unknown);
        self.memory.insert(register_index, Mem::B(BoolState::Unknown));
    }

    // Doesn't have to do anything for now
    fn jmpif(&mut self) {
        let _offset  = self.fetch_val();
    }

    // Doesn't have to do anything for now
    fn jmp(&mut self) {
        let _offset  = self.fetch_val();
    }

    // Doesn't have to do anything for now
    fn add(&mut self) {
        let _r1  = self.fetch_val();
        let _r2  = self.fetch_val();
        let _r3  = self.fetch_val();
    }
}
