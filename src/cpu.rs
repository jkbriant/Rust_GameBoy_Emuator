use std::any::Any;

#[derive(Copy, Clone)]
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
}

#[derive(Copy, Clone)]
enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum Operand8 {
    /// Directly access a register e.g. LD A, B
    Reg(Register8),
    /// Immediate 8bit value e.g. LD A, 0x02
    Immediate(u8),
    /// Memory access via a 16bit address e.g. LD A, (HL)
    Indirect(Register16),
}

pub enum Operand16 {
    /// Directly access a register e.g. LD A, B
    Reg(Register8),
    /// Immediate 8bit value e.g. LD A, 0x02
    Immediate(u8),
}

pub enum Condition {
    NZ,
    Z,
    NC,
    C,
}

pub enum Instruction {
    // 8bit loading
    Load8 {
        to: Operand8,
        from: Operand8,
    },

    // 16bit loading

    // 8bit arithmetic
    Add {
        src: Operand8,
    },
    AddCarry {
        src: Operand8,
    },
    Sub {
        src: Operand8,
    },
    SubCarry {
        src: Operand8,
    },
    And {
        src: Operand8,
    },
    Xor {
        src: Operand8,
    },
    Or {
        src: Operand8,
    },
    Cp {
        src: Operand8,
    },
    Inc {
        src: Operand8,
    },
    Dec {
        src: Operand8,
    },
    // 16bit arithmetic

    // Jumps and calls
    Jump {
        target: Operand16,
        condition: Option<Condition>,
    },
}

#[derive(Debug)]
pub struct CPU {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,

    pub halted: bool,
    pub memory: [u8; 0x10000],
}

impl CPU {
    pub fn _new() -> Self {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,

            halted: false,
            memory: [0; 0x10000],
        }
    }

    fn read_8(&self, register: Register8) -> u8 {
        match register {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::F => self.f,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    fn write_8(&mut self, register: Register8, value: u8) {
        match register {
            Register8::A => self.a = value,
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::F => self.f = value,
            Register8::H => self.h = value,
            Register8::L => self.l = value,
        }
    }

    fn read_16(&self, register: Register16) -> u16 {
        match register {
            Register16::AF => u16::from_le_bytes([self.a, self.f]),
            Register16::BC => u16::from_le_bytes([self.b, self.c]),
            Register16::DE => u16::from_le_bytes([self.d, self.e]),
            Register16::HL => u16::from_le_bytes([self.h, self.l]),
            Register16::PC => self.pc,
            Register16::SP => self.sp,
        }
    }

    fn write_16(&mut self, register: Register16, value: u16) {
        let [a, b] = value.to_le_bytes();

        match register {
            Register16::AF => {
                self.a = a;
                self.f = b;
            }
            Register16::BC => {
                self.b = a;
                self.c = b;
            }
            Register16::DE => {
                self.d = a;
                self.e = b;
            }
            Register16::HL => {
                self.h = a;
                self.l = b;
            }
            Register16::PC => self.pc = value,
            Register16::SP => self.sp = value,
        }
    }

    pub fn fetch_and_execute(&mut self) -> u32 {
        if self.halted {
            return 4;
        }

        let opcode = self.memory[self.pc as usize];
        let parameter_one = self.memory[(self.pc + 1) as usize];
        let parameter_two = self.memory[(self.pc + 2) as usize];

        let parameter_16 = u16::from_le_bytes([parameter_one, parameter_two]);

        match opcode {
            0x00 => {}
            0x01 => {
                self.write_16(Register16::BC, parameter_16); // LD BC, u16
            }
            0x02 => {
                // write to memory not implemented yet
            }
            0x03 => {
                // INC BC
                let value = self.read_16(Register16::BC) + 1;
                self.write_16(Register16::BC, value);
            }
            0x04 => {
                // INC B
                let value = self.read_8(Register8::B) + 1;
                self.write_8(Register8::B, value);
            }

            0x05 => {
                //
                let value = self.read_8(Register8::B) + 1;
                self.write_8(Register8::B, value);
            }

            _ => {
                panic!("Unimplemented opcode {}", opcode);
            }
        }

        return 0;
    }

    // LIST OF REQUIRED FUNCTIONS
    // fn load_
}
