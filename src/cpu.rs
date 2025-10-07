use crate::{
    addressing_mode::{self, AddressingMode},
    bus::Bus,
    flags::Flags,
    mem::Mem,
    opcodes::OPS_CODES_MAP,
};

pub struct CPU {
    program_counter: u16, // track the current position
    register_a: u8,       // accumulator
    register_x: u8,
    register_y: u8,
    status: Flags, // C Z I D B V
    stack_pointer: u8,
    memory: [u8; 0xFFFF], // 65536
    bus: Bus,
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data);
    }

    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        self.bus.mem_read_u16(addr)
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        self.bus.mem_write_u16(addr, data);
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
            stack_pointer: 0xfd,
            memory: [0; 0xFFFF],
            bus: Bus::new(),
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

    pub fn mem_write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = Flags::empty();

        // NES stores the 2 bytes starting memory addr at 0xFFFC
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: &[u8]) {
        // why 0x8000 you ask?
        // because PRG-ROM, the address where the NES program is going to be mapped, starts from
        // 0x8000 to 0xFFFF. the other addresses are reserved for other things
        self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x0600);
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

        let result = self.register_a.wrapping_sub(value);
        self.update_zero_and_negative_flag(result);
    }

    fn cpx(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr);

        if self.register_x >= value {
            self.status.insert(Flags::CARRY);
        } else {
            self.status.remove(Flags::CARRY);
        }

        let result = self.register_x.wrapping_sub(value);
        self.update_zero_and_negative_flag(result);
    }

    fn cpy(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr);

        if self.register_y >= value {
            self.status.insert(Flags::CARRY);
        } else {
            self.status.remove(Flags::CARRY);
        }

        let result = self.register_y.wrapping_sub(value);
        self.update_zero_and_negative_flag(result);
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

    fn rol_accumulator(&mut self) {
        let old_carry = if self.status.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        let new_carry = (self.register_a >> 7) & 1;
        self.status.set(Flags::CARRY, new_carry == 1);

        self.register_a <<= 1;
        self.register_a |= old_carry;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn rol(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let mut value = self.mem_read(addr);

        let old_carry = if self.status.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        let new_carry = (value >> 7) & 1;
        self.status.set(Flags::CARRY, new_carry == 1);

        value <<= 1;
        value |= old_carry;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn ror_accumulator(&mut self) {
        let old_carry = if self.status.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        let new_carry = self.register_a & 1;
        self.status.set(Flags::CARRY, new_carry == 1);

        self.register_a >>= 1;
        self.register_a |= old_carry << 7;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn ror(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let mut value = self.mem_read(addr);

        let old_carry = if self.status.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        let new_carry = value & 1;
        self.status.set(Flags::CARRY, new_carry == 1);

        value >>= 1;
        value |= old_carry << 7;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn jmp_absolute(&mut self) {
        let addr = self.mem_read_u16(self.program_counter);
        self.program_counter = addr;
    }

    fn jmp_indirect(&mut self) {
        let addr = self.mem_read_u16(self.program_counter);

        // 6502 has a bug that we have to mimic
        let indirect_mem = if addr & 0x00FF == 0x00FF {
            // so the idea is, if the low byte equals to 0xFF, which is at the page boundary,
            // a carry should be added to the high byte, right? e.g 9 + 7 -> carry = 1
            // but we don't want that, instead we use the original high byte, hence the bit masking
            let low = self.mem_read(addr);
            let high = self.mem_read(addr & 0xFF00); // get original high byte
            (high as u16) << 8 | low as u16
        } else {
            self.mem_read_u16(addr)
        };

        self.program_counter = indirect_mem;
    }

    fn jsr(&mut self) {
        let return_addr = self.program_counter + 2 - 1; // as stated in the 6502 instructions

        let high = (return_addr >> 8) as u8;
        let low = (return_addr & 0xff) as u8;

        self.stack_push(high);
        self.stack_push(low);

        let target_addr = self.mem_read_u16(self.program_counter);

        self.program_counter = target_addr;
    }

    fn rts(&mut self) {
        let low = self.stack_pop() as u16;
        let high = self.stack_pop() as u16;

        let return_addr = high << 8 | low;

        self.program_counter = return_addr.wrapping_add(1);
    }

    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }

    fn pla(&mut self) {
        let value = self.stack_pop();
        self.register_a = value;
        self.update_zero_and_negative_flag(self.register_a);
    }

    fn php(&mut self) {
        let mut status = self.status.bits();
        status |= 0b0001_1000; // turn on bit 4 and 5 locally. don't modify the global status flag
        self.stack_push(status);
    }

    fn plp(&mut self) {
        self.restore_status_from_stack();
    }

    fn clc(&mut self) {
        self.status.remove(Flags::CARRY);
    }

    fn sec(&mut self) {
        self.status.insert(Flags::CARRY);
    }

    fn cli(&mut self) {
        self.status.remove(Flags::INTERRUPT_DISABLE);
    }

    fn sei(&mut self) {
        self.status.insert(Flags::INTERRUPT_DISABLE);
    }

    fn clv(&mut self) {
        self.status.remove(Flags::OVERFLOW);
    }

    fn cld(&mut self) {
        self.status.remove(Flags::DECIMAL_MODE);
    }

    fn sed(&mut self) {
        self.status.insert(Flags::DECIMAL_MODE);
    }

    fn dec(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr).wrapping_sub(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn inc(&mut self, addressing_mode: AddressingMode) {
        let addr = self.get_effective_addr(addressing_mode);
        let value = self.mem_read(addr).wrapping_add(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn rti(&mut self) {
        self.restore_status_from_stack();

        let low = self.stack_pop() as u16;
        let high = self.stack_pop() as u16;
        self.program_counter = high << 8 | low;
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    fn restore_status_from_stack(&mut self) {
        let mut value = self.stack_pop();
        // make sure bit 5 stays 1
        value |= 0b0010_0000;
        // set bit 4 back to 0
        value &= !0b0001_0000;
        self.status = Flags::from_bits_truncate(value);
    }

    // the stack in NES works the other way around,
    // it doesn't start at 0x0100, but instead, it starts at the boundary,
    // which is 0x0100 + 0xFD => 509. why not 511, you ask? since the reserved spot for this is
    // from 0x0100 to 0x01FF (256 - 511) because the last 2 spots are already reserved for sth else
    // weird ik
    fn stack_push(&mut self, data: u8) {
        // 0x0100 is the starting point of the stack in the NES CPU memory map
        self.mem_write(0x0100 + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8 {
        // the pointer points to the next empty position, so that's why we decrement it first
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(0x0100 + self.stack_pointer as u16) as u8
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

    pub fn run<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);

            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let opscode = OPS_CODES_MAP.get(&code).expect("opscode not found");

            // store old program counter to differentiate jumping instructions
            let old_program_counter = self.program_counter;

            match opscode.code {
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                    self.lda(opscode.addr_mode)
                }
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => self.ldy(opscode.addr_mode),
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => self.ldx(opscode.addr_mode),
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(opscode.addr_mode),
                0x86 | 0x96 | 0x8e => self.stx(opscode.addr_mode),
                0x84 | 0x94 | 0x8c => self.sty(opscode.addr_mode),
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                    self.and(opscode.addr_mode)
                }
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                    self.eor(opscode.addr_mode)
                }
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {
                    self.ora(opscode.addr_mode)
                }
                0x24 | 0x2c => self.bit(opscode.addr_mode),
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                    self.cmp(opscode.addr_mode)
                }
                0xc0 | 0xc4 | 0xcc => self.cpy(opscode.addr_mode),
                0xe0 | 0xe4 | 0xec => self.cpx(opscode.addr_mode),
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                    self.adc(opscode.addr_mode);
                }
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    self.sbc(opscode.addr_mode);
                }
                0x90 => self.branch(!self.status.contains(Flags::CARRY)),
                0xb0 => self.branch(self.status.contains(Flags::CARRY)),
                0xf0 => self.branch(self.status.contains(Flags::ZERO)),
                0xd0 => self.branch(!self.status.contains(Flags::ZERO)),
                0x70 => self.branch(self.status.contains(Flags::OVERFLOW)),
                0x50 => self.branch(!self.status.contains(Flags::OVERFLOW)),
                0x10 => self.branch(!self.status.contains(Flags::NEGATIVE)),
                0x30 => self.branch(self.status.contains(Flags::NEGATIVE)),
                0x0a => self.asl_accumulator(),
                0x06 | 0x16 | 0x0e | 0x1e => self.asl(opscode.addr_mode),
                0x2a => self.rol_accumulator(),
                0x26 | 0x36 | 0x2e | 0x3e => self.rol(opscode.addr_mode),
                0x6a => self.ror_accumulator(),
                0x66 | 0x76 | 0x6e | 0x7e => self.ror(opscode.addr_mode),
                0xc6 | 0xd6 | 0xce | 0xde => self.dec(opscode.addr_mode),
                0xe6 | 0xf6 | 0xee | 0xfe => self.inc(opscode.addr_mode),
                0x4a => self.lsr_accumulator(),
                0x46 | 0x56 | 0x4e | 0x5e => self.lsr(opscode.addr_mode),
                0x68 => self.pla(),
                0x08 => self.php(),
                0x28 => self.plp(),
                0xd8 => self.cld(),
                0x58 => self.cli(),
                0xb8 => self.clv(),
                0x18 => self.clc(),
                0x38 => self.sec(),
                0x78 => self.sei(),
                0xf8 => self.sed(),
                0x48 => self.pha(),
                0x4c => self.jmp_absolute(),
                0x6c => self.jmp_indirect(),
                0x20 => self.jsr(),
                0x60 => self.rts(),
                0xaa => self.tax(),
                0x8a => self.txa(),
                0xa8 => self.tay(),
                0x98 => self.tya(),
                0xe8 => self.inx(),
                0xc8 => self.iny(),
                0xca => self.dex(),
                0x88 => self.dey(),
                0x40 => self.rti(),
                0xba => self.tsx(),
                0x9a => self.txs(),
                0xea => {}
                0x00 => return,
                _ => todo!(),
            }

            if old_program_counter == self.program_counter {
                self.program_counter += (opscode.len - 1) as u16;
            }
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_0xa5_lda_zeropage_loads() {
//         let mut cpu = CPU::new();
//         cpu.mem_write(0x0010, 0xAB); // [$0010] = AB
//         cpu.load_n_run(&[0xA5, 0x10, 0x00]); // LDA $10 ; BRK
//         assert_eq!(cpu.register_a, 0xAB);
//         assert_eq!(cpu.status & 0b0000_0010, 0); // Z clear
//         assert_eq!(cpu.status & 0b1000_0000, 0); // N clear
//     }
//
//     #[test]
//     fn test_0xb5_lda_zeropage_x_wraps() {
//         let mut cpu = CPU::new();
//         // X = 0x0F, base = 0xF8 → effective = (0xF8 + 0x0F) & 0xFF = 0x07
//         cpu.mem_write(0x0007, 0xCD);
//         cpu.load_n_run(&[
//             0xA2, 0x0F, // LDX #$0F
//             0xB5, 0xF8, // LDA $F8,X
//             0x00, // BRK
//         ]);
//         assert_eq!(cpu.register_a, 0xCD);
//     }
//
//     #[test]
//     fn test_0xad_lda_absolute_loads() {
//         let mut cpu = CPU::new();
//         cpu.mem_write(0x8000, 0x77);
//         cpu.load_n_run(&[0xAD, 0x00, 0x80, 0x00]); // LDA $8000 ; BRK
//         assert_eq!(cpu.register_a, 0x77);
//     }
//
//     #[test]
//     fn test_0xbd_lda_absolute_x() {
//         let mut cpu = CPU::new();
//         cpu.mem_write(0x8005, 0x44);
//         cpu.load_n_run(&[
//             0xA2, 0x05, // LDX #$05
//             0xBD, 0x00, 0x80, // LDA $8000,X  -> $8005
//             0x00,
//         ]);
//         assert_eq!(cpu.register_a, 0x44);
//     }
//
//     #[test]
//     fn test_0xb9_lda_absolute_y() {
//         let mut cpu = CPU::new();
//         cpu.mem_write(0x8003, 0x99);
//         cpu.load_n_run(&[
//             0xA0, 0x03, // LDY #$03
//             0xB9, 0x00, 0x80, // LDA $8000,Y  -> $8003
//             0x00,
//         ]);
//         assert_eq!(cpu.register_a, 0x99);
//     }
//
//     #[test]
//     fn test_0xa1_lda_indirect_x() {
//         let mut cpu = CPU::new();
//         // X = 4; operand = $20 → pointer in ZP at $24/$25 → $8000
//         cpu.mem_write(0x0024, 0x00); // low
//         cpu.mem_write(0x0025, 0x80); // high
//         cpu.mem_write(0x8000, 0x66); // target
//         cpu.load_n_run(&[
//             0xA2, 0x04, // LDX #$04
//             0xA1, 0x20, // LDA ($20,X)
//             0x00,
//         ]);
//         assert_eq!(cpu.register_a, 0x66);
//     }
//
//     #[test]
//     fn test_0xb1_lda_indirect_y() {
//         let mut cpu = CPU::new();
//         // ZP pointer at $20/$21 -> $8000 ; Y=5 → effective $8005
//         cpu.mem_write(0x0020, 0x00); // low
//         cpu.mem_write(0x0021, 0x80); // high
//         cpu.mem_write(0x8005, 0x42);
//         cpu.load_n_run(&[
//             0xA0, 0x05, // LDY #$05
//             0xB1, 0x20, // LDA ($20),Y
//             0x00,
//         ]);
//         assert_eq!(cpu.register_a, 0x42);
//     }
//
//     #[test]
//     fn test_lda_sets_negative_flag() {
//         let mut cpu = CPU::new();
//         cpu.load_n_run(&[0xA9, 0x80, 0x00]); // LDA #$80 ; BRK
//         assert_ne!(cpu.status & 0b1000_0000, 0); // N set
//         assert_eq!(cpu.status & 0b0000_0010, 0); // Z clear
//     }
//
//     #[test]
//     fn test_lda_zeropage_x_wrap_example_edge_ff() {
//         let mut cpu = CPU::new();
//         // base=$FF, X=2 → (0xFF+2)&0xFF = 0x01
//         cpu.mem_write(0x0001, 0x5A);
//         cpu.load_n_run(&[
//             0xA2, 0x02, // LDX #$02
//             0xB5, 0xFF, // LDA $FF,X  -> $01
//             0x00,
//         ]);
//         assert_eq!(cpu.register_a, 0x5A);
//     }
// }
