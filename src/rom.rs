const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    VERTICAL,
    HORIZONTAL,
    FOUR_SCREEN,
}

// so the ROM dump contains 4 things
//  1. header -> mapper and screen Mirroring
//  2. PRG ROM
//  3. CHR ROM
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(raw: &[u8]) -> Result<Rom, String> {
        // check if the file is iNES
        if &raw[0..=3] != NES_TAG {
            return Err("File isn't in iNES format".to_string());
        }

        // decode the mapper
        let mapper = (raw[7] & 0b1111_0000) | (raw[6] >> 4);

        // check iNES version -> reject version 2.0
        let ines_ver = (raw[7] >> 2) & 0b11;
        if ines_ver != 0 {
            return Err("NES2.0 format isn't supported".to_string());
        }

        let four_screen = raw[6] & 0b1000 != 0; // bit 3
        let vertical_mirroring = raw[6] & 0b1 != 0; // bit 0
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FOUR_SCREEN,
            (false, true) => Mirroring::VERTICAL,
            (false, false) => Mirroring::HORIZONTAL,
        };

        let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw[5] as usize & CHR_ROM_PAGE_SIZE;

        let trainer = raw[6] & 0b100 != 0;

        let prg_rom_start = 16 + if trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Rom {
            mapper,
            prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            screen_mirroring,
        })
    }
}
