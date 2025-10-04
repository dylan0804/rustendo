use std::ops::Add;

use bitflags::Flag;

use crate::{
    addressing_mode::{self, AddressingMode},
    flags::Flags,
    mem::Mem,
    opcodes::OPS_CODES_MAP,
};

pub struct CPU {
    pub program_counter: u16, // track the current position
    pub register_a: u8,       // accumulator
    pub register_x: u8,
    pub register_y: u8,
    pub status: Flags,    // C Z I D B V
    memory: [u8; 0xFFFF], // 65536
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            status: Flags::empty(),
            register_a: 0,
            register_x: 0,
            register_y: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn get_effective_addr(&mut self, addressing_mode: AddressingMode) -> u16 {
        match addressing_mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPage_X => {
                let base = self.mem_read(self.program_counter);
                base.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let base = self.mem_read(self.program_counter);
                base.wrapping_add(self.register_y) as u16
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                let pointer = base.wrapping_add(self.register_x);

                self.read_zp_16(pointer as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
                let pointer = self.read_zp_16(base as u16);

                pointer.wrapping_add(self.register_y as u16)
            }
            _ => 0,
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = Flags::empty();

        // NES stores the 2 bytes starting memory addr at 0xFFFC. idk why
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    fn load(&mut self, program: &[u8]) {
        // why 0x8000 you ask?
        // because PRG-ROM, the address where the NES program is going to be mapped, starts from
        // 0x8000 to 0xFFFF. the other addresses are reserved for other things
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_n_run(&mut self, program: &[u8]) {
        self.load(&program);
        self.reset();
        self.run();
    }

    fn lda(&mut self, addresing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addresing_mode);
        let value = self.mem_read(addr);
        self.register_a = value;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn ldy(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr);
        self.register_y = value;
        self.update_zero_and_negative_flag(value);
    }

    fn ldx(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr);
        self.register_x = value;
        self.update_zero_and_negative_flag(value);
    }

    fn sta(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        self.mem_write(addr, self.register_a);
    }

    fn stx(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        self.mem_write(addr, self.register_y);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn and(&mut self, addresing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addresing_mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a & value;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn ora(&mut self, addresing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addresing_mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a | value;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn eor(&mut self, addresing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addresing_mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a ^ value;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn bit(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr);
        let and = value & self.register_a;

        if and == 0 {
            self.status.insert(Flags::ZERO);
        } else {
            self.status.remove(Flags::ZERO);
        }

        self.status.set(Flags::OVERFLOW, value & 0b0100_0000 != 0);
        self.status.set(Flags::NEGATIVE, value & 0b1000_0000 != 0);
    }

    fn cmp(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr);

        if self.register_a >= value {
            self.status.insert(Flags::CARRY);
        } else {
            self.status.remove(Flags::CARRY);
        }

        self.update_zero_and_negative_flag(self.register_a.wrapping_sub(1));
    }

    fn cpx(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr);

        if self.register_x >= value {
            self.status.insert(Flags::CARRY);
        } else {
            self.status.remove(Flags::CARRY);
        }

        self.update_zero_and_negative_flag(self.register_x.wrapping_sub(1));
    }

    fn cpy(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr);

        if self.register_y >= value {
            self.status.insert(Flags::CARRY);
        } else {
            self.status.remove(Flags::CARRY);
        }

        self.update_zero_and_negative_flag(self.register_y.wrapping_sub(1));
    }

    fn adc(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr);
        self.add_to_register_a(value);
    }

    fn sbc(&mut self, addresing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addresing_mode);
        let value = self.mem_read(addr);
        self.add_to_register_a(!value);
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let value = self.mem_read(self.program_counter) as i8; // branch expects a signed byte
            self.program_counter += 1; // consume operand
            let jump_addr = self.program_counter.wrapping_add(value as i16 as u16);
            self.program_counter = jump_addr;
        }
    }

    fn asl_accumulator(&mut self) {
        let carry = (self.register_a >> 7) & 1; // get the carry flag (bit 7)
        self.register_a <<= 1;

        self.status.set(Flags::CARRY, carry == 1);
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn asl(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let mut value = self.mem_read(addr);

        // get bit 7
        let carry = (value >> 7) & 1;
        value <<= 1;

        self.status.set(Flags::CARRY, carry == 1);

        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn lsr_accumulator(&mut self) {
        let carry = self.register_a & 0x01;

        self.register_a >>= 1;
        self.status.set(Flags::CARRY, carry == 1);
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn lsr(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let mut value = self.mem_read(addr);

        // get bit 0
        let carry = value & 0x01;
        self.status.set(Flags::CARRY, carry == 1);

        value >>= 1;

        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn add_to_register_a(&mut self, value: u8) {
        let a = self.register_a as u16;
        let carry = if self.status.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        let sum = a + value as u16 + carry as u16;

        if sum > 0xFF {
            self.status.insert(Flags::CARRY);
        } else {
            self.status.remove(Flags::CARRY);
        }

        let result = sum as u8;

        if ((self.register_a ^ result) & (value ^ result) & 0x80) != 0 {
            self.status.insert(Flags::OVERFLOW);
        } else {
            self.status.remove(Flags::OVERFLOW);
        }

        self.register_a = result;
        self.update_zero_and_negative_flag(result);
    }

    fn update_zero_and_negative_flag(&mut self, value: u8) {
        // turn on the Z bit -> can only be 0 or 1
        self.status.set(Flags::ZERO, value == 0);
        // turn on the N bit
        self.status.set(Flags::NEGATIVE, (value & 0x80) != 0);
    }

    // reads a 16-bit memory in little endian order
    // ex:
    //  LDA $8000 <=> A9 00 80
    //  since NES uses little endian, the CPU will read 0x00 (least significant) first then 0x80 (most significant)
    //  since people write numbers from the most significant part first, we get 0x8000
    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        let low = self.mem_read(addr) as u16;
        let high = self.mem_read(addr + 1) as u16;
        (high << 8) | low
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let low = (data & 0xff) as u8; // mask everything except the low part
        let high = (data >> 8) as u8;

        self.mem_write(addr, low);
        self.mem_write(addr + 1, high);
    }

    fn read_zp_16(&mut self, addr: u16) -> u16 {
        let low = self.mem_read(addr) as u16;
        let high = self.mem_read(addr.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let opscode = OPS_CODES_MAP.get(&code).expect("opscode not found");

            match opscode.code {
                // lda
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                    self.lda(opscode.addr_mode);
                }
                // LDY {
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => {
                    self.ldy(opscode.addr_mode);
                }
                // LDX
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => {
                    self.ldx(opscode.addr_mode);
                }
                // STA
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    self.sta(opscode.addr_mode);
                }
                // STX
                0x86 | 0x96 | 0x8e => {
                    self.stx(opscode.addr_mode);
                }
                // STY
                0x84 | 0x94 | 0x8c => {
                    self.sty(opscode.addr_mode);
                }
                // AND
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                    self.and(opscode.addr_mode);
                }
                // EOR
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                    self.eor(opscode.addr_mode);
                }
                // ORA
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {
                    self.ora(opscode.addr_mode);
                }
                // BIT
                0x24 | 0x2c => {
                    self.bit(opscode.addr_mode);
                }
                // CMP
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                    self.cmp(opscode.addr_mode)
                }
                // CPY
                0xc0 | 0xc4 | 0xcc => {
                    self.cpy(opscode.addr_mode);
                }
                // CPX
                0xe0 | 0xe4 | 0xec => self.cpx(opscode.addr_mode),
                // ADC
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                    self.adc(opscode.addr_mode);
                }
                // SBC
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    self.sbc(opscode.addr_mode);
                }
                // BCC
                0x90 => {
                    self.branch(!self.status.contains(Flags::CARRY));
                }
                // BCS {
                0xb0 => {
                    self.branch(self.status.contains(Flags::CARRY));
                }
                // BEQ
                0xf0 => {
                    self.branch(self.status.contains(Flags::ZERO));
                }
                // BNE
                0xd0 => {
                    self.branch(!self.status.contains(Flags::ZERO));
                }
                // BVS
                0x70 => {
                    self.branch(self.status.contains(Flags::OVERFLOW));
                }
                // BVC
                0x50 => {
                    self.branch(!self.status.contains(Flags::OVERFLOW));
                }
                // BPL
                0x10 => {
                    self.branch(!self.status.contains(Flags::NEGATIVE));
                }
                // BMI
                0x30 => {
                    self.branch(self.status.contains(Flags::NEGATIVE));
                }
                // ASL accumulator
                0x0a => self.asl_accumulator(),
                // other ASL
                0x06 | 0x16 | 0x0e | 0x1e => {
                    self.asl(opscode.addr_mode);
                }
                // TAX
                0xaa => self.tax(),
                // TXA
                0x8a => self.txa(),
                // TAY
                0xa8 => self.tay(),
                // TYA
                0x98 => self.tya(),
                // INX
                0xe8 => self.inx(),
                // INY
                0xc8 => self.iny(),
                // DEX
                0xca => self.dex(),
                // DEY
                0x88 => self.dey(),
                // NOP
                0xea => {}
                // BRK
                0x00 => return,
                _ => todo!(),
            }

            if opscode.len - 1 >= 1 {
                self.program_counter += (opscode.len - 1) as u16;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa5_lda_zeropage_loads() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x0010, 0xAB); // [$0010] = AB
        cpu.load_n_run(&[0xA5, 0x10, 0x00]); // LDA $10 ; BRK
        assert_eq!(cpu.register_a, 0xAB);
        assert_eq!(cpu.status & 0b0000_0010, 0); // Z clear
        assert_eq!(cpu.status & 0b1000_0000, 0); // N clear
    }

    #[test]
    fn test_0xb5_lda_zeropage_x_wraps() {
        let mut cpu = CPU::new();
        // X = 0x0F, base = 0xF8 → effective = (0xF8 + 0x0F) & 0xFF = 0x07
        cpu.mem_write(0x0007, 0xCD);
        cpu.load_n_run(&[
            0xA2, 0x0F, // LDX #$0F
            0xB5, 0xF8, // LDA $F8,X
            0x00, // BRK
        ]);
        assert_eq!(cpu.register_a, 0xCD);
    }

    #[test]
    fn test_0xad_lda_absolute_loads() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x8000, 0x77);
        cpu.load_n_run(&[0xAD, 0x00, 0x80, 0x00]); // LDA $8000 ; BRK
        assert_eq!(cpu.register_a, 0x77);
    }

    #[test]
    fn test_0xbd_lda_absolute_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x8005, 0x44);
        cpu.load_n_run(&[
            0xA2, 0x05, // LDX #$05
            0xBD, 0x00, 0x80, // LDA $8000,X  -> $8005
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0x44);
    }

    #[test]
    fn test_0xb9_lda_absolute_y() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x8003, 0x99);
        cpu.load_n_run(&[
            0xA0, 0x03, // LDY #$03
            0xB9, 0x00, 0x80, // LDA $8000,Y  -> $8003
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0x99);
    }

    #[test]
    fn test_0xa1_lda_indirect_x() {
        let mut cpu = CPU::new();
        // X = 4; operand = $20 → pointer in ZP at $24/$25 → $8000
        cpu.mem_write(0x0024, 0x00); // low
        cpu.mem_write(0x0025, 0x80); // high
        cpu.mem_write(0x8000, 0x66); // target
        cpu.load_n_run(&[
            0xA2, 0x04, // LDX #$04
            0xA1, 0x20, // LDA ($20,X)
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0x66);
    }

    #[test]
    fn test_0xb1_lda_indirect_y() {
        let mut cpu = CPU::new();
        // ZP pointer at $20/$21 -> $8000 ; Y=5 → effective $8005
        cpu.mem_write(0x0020, 0x00); // low
        cpu.mem_write(0x0021, 0x80); // high
        cpu.mem_write(0x8005, 0x42);
        cpu.load_n_run(&[
            0xA0, 0x05, // LDY #$05
            0xB1, 0x20, // LDA ($20),Y
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0x42);
    }

    #[test]
    fn test_lda_sets_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_n_run(&[0xA9, 0x80, 0x00]); // LDA #$80 ; BRK
        assert_ne!(cpu.status & 0b1000_0000, 0); // N set
        assert_eq!(cpu.status & 0b0000_0010, 0); // Z clear
    }

    #[test]
    fn test_lda_zeropage_x_wrap_example_edge_ff() {
        let mut cpu = CPU::new();
        // base=$FF, X=2 → (0xFF+2)&0xFF = 0x01
        cpu.mem_write(0x0001, 0x5A);
        cpu.load_n_run(&[
            0xA2, 0x02, // LDX #$02
            0xB5, 0xFF, // LDA $FF,X  -> $01
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0x5A);
    }
}
