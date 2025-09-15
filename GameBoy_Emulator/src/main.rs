

const MEMORY_SIZE: usize = 0xFFFF; // 64KB Memory

fn main() {
    let test_rom: Vec<u8> = vec![
        0x3E, 0x0A, // LDA, 0x01
        0x06, 0x05, // LDB, 0x02
        0x80, // ADD
        0x80, // ADD
        0xC3, 0x04, 0x00,
    ];

    let mut cpu = CPU::new();
    cpu.load_rom(&test_rom);
    cpu.run();
}

struct CPU {
    program_counter: u16,
    memory: [u8; MEMORY_SIZE], // 64KB Memory
}

impl CPU {
    fn new() -> Self {
        CPU {
            program_counter: 0,
            memory: [0; MEMORY_SIZE],
        }
    }

    fn load_rom(&mut self, rom: &[u8]) {
        self.memory[..rom.len()].copy_from_slice(rom);
    }

    fn run(&mut self) {
        loop {
            let opcode = self.memory[self.program_counter as usize];
            match opcode {
                0x3E => { // LDA
                    let value = self.memory[(self.program_counter + 1) as usize];
                    println!("LDA {}", value);
                    self.program_counter += 2;
                }
                0x06 => { // LDB
                    let value = self.memory[(self.program_counter + 1) as usize];
                    println!("LDB {}", value);
                    self.program_counter += 2;
                }
                0x80 => { // ADD
                    println!("ADD");
                    self.program_counter += 1;
                }
                0xC3 => { // JMP
                    let low = self.memory[(self.program_counter + 1) as usize] as u16;
                    let high = self.memory[(self.program_counter + 2) as usize] as u16;
                    let address = (high << 8) | low;
                    println!("JMP to {:04X}", address);
                    self.program_counter = address;
                }
                _ => {
                    println!("Unknown opcode {:02X} at {:04X}", opcode, self.program_counter);
                    break;
                }
            }
        }
    }
}