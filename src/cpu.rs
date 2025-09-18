pub enum Instruction {
    NOP,
}

const MEMORY_SIZE: usize = 0x10000;

pub struct CPU {
    pub register_a: u8,
    pub register_b: u8,
    pub register_c: u8,
    pub register_d: u8,
    pub register_e: u8,
    pub register_h: u8,
    pub register_l: u8,

    pub program_counter: u16,
    pub stack_pointer: u16,

    pub memory: [u8; MEMORY_SIZE],

    pub cycle_counter: u64,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            register_d: 0,
            register_e: 0,
            register_h: 0,
            register_l: 0,

            program_counter: 0,
            stack_pointer: 0,

            memory: [0; MEMORY_SIZE],

            cycle_counter: 0,
        }
    }

    // performs the provided intruction on itself and moves the program_counter
    pub fn execute(&mut self, instruction: Instruction) {
        match (instruction) {
            Instruction::NOP => return,
        }
    }
}
