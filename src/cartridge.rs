use std::io::Result;

pub struct Cartridge {
    rom: [u8; 0x4000],
}

impl Cartridge {
    fn load_cartridge(&mut self, rom: [u8; 0x4000]) -> Result {
        self.rom = rom;
    }
    fn read_byte(&self, address: u16) -> u8 {
        return self.rom[address as usize];
    }
}
