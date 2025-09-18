mod cpu;
use cpu::*;

const MAX_CYCLES: u64 = 20;

fn main() {
    println!("GameBoy Emulator");
    let test_rom = [Instruction::NOP, Instruction::NOP, Instruction::JUMP(0)];

    let mut cpu = CPU::new();
    cpu.enabled = true;

    while cpu.enabled && cpu.cycle_counter < MAX_CYCLES {
        cpu.execute(test_rom[cpu.program_counter as usize]);
        println!(
            "({}) Register A: {}, Register B: {}",
            cpu.program_counter, cpu.register_a, cpu.register_b
        );
    }
}

// fn decode_instruction(opcode: &u8) -> Instruction {
//     match (opcode) {
//         _ => return Instruction::NOP,
//     }
// }
