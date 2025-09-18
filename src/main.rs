mod cpu;
use cpu::*;

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

fn decode_instruction(opcode: &u8) -> Instruction {
    return Instruction::load_b(0x01);
}
