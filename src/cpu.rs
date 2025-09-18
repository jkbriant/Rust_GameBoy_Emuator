pub enum Instruction {
    load_a(u8),
    load_b(u8),
}

pub struct CPU {
    program_counter: u8,
    pub register_a: u8,
    pub register_b: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            register_a: 0,
            register_b: 0,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match (instruction) {
            Instruction::load_a(value) => {
                self.register_a = value;
            }
            Instruction::load_b(value) => {
                self.register_b = value;
            }
        }
    }
}
