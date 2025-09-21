mod cartridge;
mod cpu;
mod memory;

const MAX_CYCLES: u64 = 20;

fn main() {
    println!("GameBoy Emulator");
    // let test_rom = [];

    // let mut cpu = CPU::new();
    // cpu.halted = false;

    // while cpu.halted == false && cpu.cycle_counter < MAX_CYCLES {
    //     println!("{:?}", test_rom[cpu.program_counter as usize]);
    //     cpu.execute(test_rom[cpu.program_counter as usize]);
    //     println!(
    //         "PC: {} | A: {}, B: {}, C: {}",
    //         cpu.program_counter, cpu.register_a, cpu.register_b, cpu.register_c
    //     );
    // }
}

// fn decode_instruction(opcode: &u8) -> Instruction {
//     match (opcode) {
//         _ => return Instruction::NOP,
//     }
// }
