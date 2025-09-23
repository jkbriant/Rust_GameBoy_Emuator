use core::panic;

const MEMORY_SIZE: usize = 0x2000;

pub struct Memory {
    pub work_ram: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new(&self) -> Self {
        Memory {
            work_ram: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            // 0x0000..=0x7FFF => self.cartridge.read_rom(address),
            0x8000..=0x9FFF => 0, // VRAM (handled by PPU)
            // 0xA000..=0xBFFF => self.cartridge.read_ram(address),
            0xC000..=0xDFFF => self.work_ram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.work_ram[(address - 0xE000) as usize], // Echo RAM
            // 0xFE00..=0xFE9F => 0, // OAM (handled by PPU)
            // 0xFF00 => 0, // Joypad (handled by joypad module)
            // 0xFF01..=0xFF02 => 0, // Serial
            // 0xFF04..=0xFF07 => 0, // Timer (handled by timer module)
            // 0xFF0F => 0, // IF
            // 0xFF10..=0xFF3F => 0, // Audio (handled by APU)
            // 0xFF40..=0xFF4B => 0, // LCD (handled by PPU)
            // 0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            // 0xFFFF => 0, // IE
            // _ => 0
            _ => panic!("Ram address not implemented at {}", address),
        }
    }
}
