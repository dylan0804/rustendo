use crate::mem::Mem;

const RAM_START: u16 = 0x0000;
const RAM_MIRROR_END: u16 = 0x1FFF; // 1 decimal less than 0x2000
const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_MIRROR_END: u16 = 0x3FFF;

const RAM_MIRROR_MASK: u16 = 0x07FF; // keep low 11 bits
const PPU_REG_MASK: u16 = 0x2007;

pub struct Bus {
    cpu_vram: [u8; 2048], // RAM only uses 2KB of space
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_vram: [0; 2048],
        }
    }
}

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_MIRROR_END => {
                // we are only keeping the lowest 11 bits here, aka masking the highest 2 bits. why?
                // so, the NES CPU reserves 0x0000 to 0x1FFF for the RAM, but the actual RAM only
                // uses 2 KB, the rest (2048 - 8191) is actually used for mirroring, let's just say
                // it's empty for this sake. that's why only 11 bits are used not 13 bits, because 0 - 2047
                // only has 11 bits, the other 2, we hide it
                let mirrored = addr & RAM_MIRROR_MASK;
                self.cpu_vram[mirrored as usize]
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRROR_END => {
                // works exactly like RAM, only difference is where it starts and ends, and
                // which bits to hide -> 0x2000 - 0x2007
                let mirrored = addr & PPU_REG_MASK;
                todo!();
            }
            _ => 0,
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_START..=RAM_MIRROR_END => {
                let mirrored = addr & RAM_MIRROR_MASK;
                self.cpu_vram[mirrored as usize] = data;
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRROR_END => {
                let mirrored = addr & PPU_REG_MASK;
                todo!();
            }
            _ => {}
        }
    }
}
