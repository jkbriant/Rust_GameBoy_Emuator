fn main() {
    println!("This is a test.");
    let test_rom = [0x07, 0x01];
    println!("{}", test_rom[0]);

    let mut cpu = CPU::new();
    println!(
        "Register A: {}, Register B: {}",
        cpu.register_a, cpu.register_b
    );

    cpu.execute(Instruction::load_a(0x05));
    cpu.execute(Instruction::load_b(0x09));

    println!("->");
    println!(
        "Register A: {}, Register B: {}",
        cpu.register_a, cpu.register_b
    );
}

enum Instruction {
    load_a(u8),
    load_b(u8),
}

fn decode_instruction(opcode: &u8) -> Instruction {
    return Instruction::load_b(0x01);
}

struct CPU {
    program_counter: usize,
    register_a: u8,
    register_b: u8,
}

impl CPU {
    fn new() -> Self {
        CPU {
            program_counter: 0,
            register_a: 0,
            register_b: 0,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
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
