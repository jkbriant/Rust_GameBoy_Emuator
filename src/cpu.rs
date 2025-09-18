#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    // System Control
    NOP,
    HALT,
    // INVALID,
    // Load
    // LOAD_A(u8),
    // LOAD_B(u8),
    // LOAD_C(u8),
    // LOAD_D(u8),
    // LOAD_A_B, // load B to A
    // LOAD_B_A, // load A to B

    // Arithmetic
    // INC_A,
    // DEC_A,
    // DEC_B,
    // ADD_A_B,
    // ADD_A(u8),

    // Jumps
    JUMP(u16),
}

const MEMORY_SIZE: usize = 0x10000;

pub struct CPU {
    pub enabled: bool,

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
            enabled: false,

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
        self.cycle_counter += 1;
        match (instruction) {
            Instruction::NOP => self.program_counter += 1,
            Instruction::JUMP(address) => self.program_counter = address,
            Instruction::HALT => self.enabled = false,
        }
    }
}
