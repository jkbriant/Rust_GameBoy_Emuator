use core::panic;

const WORK_RAM_SIZE: usize = 0x2000;
const VIDEO_RAM_SIZE: usize = 0x2000;

pub struct MemoryManager {
    pub work_ram: [u8; WORK_RAM_SIZE],
    pub video_ram: [u8; VIDEO_RAM_SIZE],
    // pub cartridge: Option<Cartridge>,
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {
            work_ram: [0; WORK_RAM_SIZE],
            video_ram: [0; VIDEO_RAM_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xC000..=0xDFFF => self.work_ram[(address - 0xC000) as usize],
            _ => panic!("Ram address not implemented at {}", address),
        }
    }
}
