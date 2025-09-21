#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn _get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn _set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn _get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn _set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn _get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn _set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }
}

pub struct CPU {
    pub reg: Registers,
    pub halted: bool,
}

impl CPU {
    pub fn _new() -> Self {
        CPU {
            reg: Registers::default(),
            halted: false,
        }
    }

    pub fn _execute_instruction(&mut self) -> u32 {
        if self.halted {
            return 4;
        }

        // let opcode =

        return 0;
    }
}
