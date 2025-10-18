use std::{collections::HashMap, ops::Add};

use crate::addressing_mode::AddressingMode;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
pub struct OpCode {
    pub name: &'static str,
    pub code: u8,
    pub len: u8,
    pub addr_mode: AddressingMode,
}

impl OpCode {
    pub fn new(code: u8, name: &'static str, len: u8, addr_mode: AddressingMode) -> Self {
        Self {
            code,
            name,
            len,
            addr_mode,
        }
    }
}

lazy_static! {
    pub static ref OPS_CODES: Vec<OpCode> = vec![
        // LDA
        OpCode::new(0xa9, "LDA", 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, AddressingMode::Absolute_X),
        OpCode::new(0xb9, "LDA", 3, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 2, AddressingMode::Indirect_Y),

        // LDX
        OpCode::new(0xa2, "LDX", 2, AddressingMode::Immediate),
        OpCode::new(0xa6, "LDX", 2, AddressingMode::ZeroPage),
        OpCode::new(0xb6, "LDX", 2, AddressingMode::ZeroPage_Y),
        OpCode::new(0xae, "LDX", 3, AddressingMode::Absolute),
        OpCode::new(0xbe, "LDX", 3, AddressingMode::Absolute_Y),

        // LDY
        OpCode::new(0xa0, "LDY", 2, AddressingMode::Immediate),
        OpCode::new(0xa4, "LDY", 2, AddressingMode::ZeroPage),
        OpCode::new(0xb4, "LDY", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xac, "LDY", 3, AddressingMode::Absolute),
        OpCode::new(0xbc, "LDY", 3, AddressingMode::Absolute_X),

        // STA
        OpCode::new(0x85, "STA", 2, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x8d, "STA", 3, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 3, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, AddressingMode::Indirect_Y),

        // STX
        OpCode::new(0x86, "STX", 2, AddressingMode::ZeroPage),
        OpCode::new(0x96, "STX", 2, AddressingMode::ZeroPage_Y),
        OpCode::new(0x8e, "STX", 3, AddressingMode::Absolute),

        // STY
        OpCode::new(0x84, "STY", 2, AddressingMode::ZeroPage),
        OpCode::new(0x94, "STY", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x8c, "STY", 3, AddressingMode::Absolute),

        // AND
        OpCode::new(0x29, "AND", 2, AddressingMode::Immediate),
        OpCode::new(0x25, "AND", 2, AddressingMode::ZeroPage),
        OpCode::new(0x35, "AND", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x2d, "AND", 3, AddressingMode::Absolute),
        OpCode::new(0x3d, "AND", 3, AddressingMode::Absolute_X),
        OpCode::new(0x39, "AND", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x21, "AND", 2, AddressingMode::Indirect_X),
        OpCode::new(0x31, "AND", 2, AddressingMode::Indirect_Y),

        // ORA
        OpCode::new(0x09, "ORA", 2, AddressingMode::Immediate),
        OpCode::new(0x05, "ORA", 2, AddressingMode::ZeroPage),
        OpCode::new(0x15, "ORA", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x0d, "ORA", 3, AddressingMode::Absolute),
        OpCode::new(0x1d, "ORA", 3, AddressingMode::Absolute_X),
        OpCode::new(0x19, "ORA", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x01, "ORA", 2, AddressingMode::Indirect_X),
        OpCode::new(0x11, "ORA", 2, AddressingMode::Indirect_Y),

        // EOR
        OpCode::new(0x49, "EOR", 2, AddressingMode::Immediate),
        OpCode::new(0x45, "EOR", 2, AddressingMode::ZeroPage),
        OpCode::new(0x55, "EOR", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x4d, "EOR", 3, AddressingMode::Absolute),
        OpCode::new(0x5d, "EOR", 3, AddressingMode::Absolute_X),
        OpCode::new(0x59, "EOR", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x41, "EOR", 2, AddressingMode::Indirect_X),
        OpCode::new(0x51, "EOR", 2, AddressingMode::Indirect_Y),

        // BIT
        OpCode::new(0x24, "BIT", 2, AddressingMode::ZeroPage),
        OpCode::new(0x2c, "BIT", 3, AddressingMode::Absolute),

        // CMP
        OpCode::new(0xc9, "CMP", 2, AddressingMode::Immediate),
        OpCode::new(0xc5, "CMP", 2, AddressingMode::ZeroPage),
        OpCode::new(0xd5, "CMP", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xcd, "CMP", 3, AddressingMode::Absolute),
        OpCode::new(0xdd, "CMP", 3, AddressingMode::Absolute_X),
        OpCode::new(0xd9, "CMP", 3, AddressingMode::Absolute_Y),
        OpCode::new(0xc1, "CMP", 2, AddressingMode::Indirect_X),
        OpCode::new(0xd1, "CMP", 2, AddressingMode::Indirect_Y),

        // CPY
        OpCode::new(0xc0, "CPY", 2, AddressingMode::Immediate),
        OpCode::new(0xc4, "CPY", 2, AddressingMode::ZeroPage),
        OpCode::new(0xcc, "CPY", 3, AddressingMode::Absolute),

        // CPX
        OpCode::new(0xe0, "CPX", 2, AddressingMode::Immediate),
        OpCode::new(0xe4, "CPX", 2, AddressingMode::ZeroPage),
        OpCode::new(0xec, "CPX", 3, AddressingMode::Absolute),

        // ADC
        OpCode::new(0x69, "ADC", 2, AddressingMode::Immediate),
        OpCode::new(0x65, "ADC", 2, AddressingMode::ZeroPage),
        OpCode::new(0x75, "ADC", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x6d, "ADC", 3, AddressingMode::Absolute),
        OpCode::new(0x7d, "ADC", 3, AddressingMode::Absolute_X),
        OpCode::new(0x79, "ADC", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x61, "ADC", 2, AddressingMode::Indirect_X),
        OpCode::new(0x71, "ADC", 2, AddressingMode::Indirect_Y),

        // SBC
        OpCode::new(0xe9, "SBC", 2, AddressingMode::Immediate),
        OpCode::new(0xe5, "SBC", 2, AddressingMode::ZeroPage),
        OpCode::new(0xf5, "SBC", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xed, "SBC", 3, AddressingMode::Absolute),
        OpCode::new(0xfd, "SBC", 3, AddressingMode::Absolute_X),
        OpCode::new(0xf9, "SBC", 3, AddressingMode::Absolute_Y),
        OpCode::new(0xe1, "SBC", 2, AddressingMode::Indirect_X),
        OpCode::new(0xf1, "SBC", 2, AddressingMode::Indirect_Y),

        // branching
        OpCode::new(0xd0, "BNE", 2, AddressingMode::Implied),
        OpCode::new(0x70, "BVS", 2, AddressingMode::Implied),
        OpCode::new(0x50, "BVC", 2, AddressingMode::Implied),
        OpCode::new(0x30, "BMI", 2, AddressingMode::Implied),
        OpCode::new(0xf0, "BEQ", 2, AddressingMode::Implied),
        OpCode::new(0xb0, "BCS", 2, AddressingMode::Implied),
        OpCode::new(0x90, "BCC", 2, AddressingMode::Implied),
        OpCode::new(0x10, "BPL", 2, AddressingMode::Implied),

        // ASL
        OpCode::new(0x0a, "ASL", 1, AddressingMode::Implied),
        OpCode::new(0x06, "ASL", 2, AddressingMode::ZeroPage),
        OpCode::new(0x16, "ASL", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x0e, "ASL", 3, AddressingMode::Absolute),
        OpCode::new(0x1e, "ASL", 3, AddressingMode::Absolute_X),

        // ROL
        OpCode::new(0x2a, "ROL", 1, AddressingMode::Implied),
        OpCode::new(0x26, "ROL", 2, AddressingMode::ZeroPage),
        OpCode::new(0x36, "ROL", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x2e, "ROL", 3, AddressingMode::Absolute),
        OpCode::new(0x3e, "ROL", 3, AddressingMode::Absolute_X),

        // ROR
        OpCode::new(0x6a, "ROR", 1, AddressingMode::Implied),
        OpCode::new(0x66, "ROR", 2, AddressingMode::ZeroPage),
        OpCode::new(0x76, "ROR", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x6e, "ROR", 3, AddressingMode::Absolute),
        OpCode::new(0x7e, "ROR", 3, AddressingMode::Absolute_X),

        // JMP
        OpCode::new(0x4c, "JMP", 3, AddressingMode::Implied),
        OpCode::new(0x6c, "JMP", 3, AddressingMode::Implied),

        // JSR
        OpCode::new(0x20, "JSR", 3, AddressingMode::Absolute),
        // RTS
        OpCode::new(0x60, "RTS", 1, AddressingMode::Implied),

        // Implied
        OpCode::new(0xaa, "TAX", 1, AddressingMode::Implied),
        OpCode::new(0xa8, "TAY", 1, AddressingMode::Implied),
        OpCode::new(0xba, "TSX", 1, AddressingMode::Implied),
        OpCode::new(0x8a, "TXA", 1, AddressingMode::Implied),
        OpCode::new(0x9a, "TXS", 1, AddressingMode::Implied),
        OpCode::new(0x98, "TYA", 1, AddressingMode::Implied),
        OpCode::new(0xe8, "INX", 1, AddressingMode::Implied),
        OpCode::new(0xc8, "INY", 1, AddressingMode::Implied),
        OpCode::new(0xca, "DEX", 1, AddressingMode::Implied),
        OpCode::new(0x88, "DEY", 1, AddressingMode::Implied),

        OpCode::new(0xea, "NOP", 1, AddressingMode::Implied),

        // Stack
        OpCode::new(0x48, "PHA", 1, AddressingMode::Implied),
        OpCode::new(0x68, "PLA", 1, AddressingMode::Implied),
        OpCode::new(0x08, "PHP", 1, AddressingMode::Implied),
        OpCode::new(0x28, "PLP", 1, AddressingMode::Implied),

        // Clear flags
        OpCode::new(0xD8, "CLD", 1, AddressingMode::Implied),
        OpCode::new(0x58, "CLI", 1, AddressingMode::Implied),
        OpCode::new(0xb8, "CLV", 1, AddressingMode::Implied),
        OpCode::new(0x18, "CLC", 1, AddressingMode::Implied),
        OpCode::new(0x38, "SEC", 1, AddressingMode::Implied),
        OpCode::new(0x78, "SEI", 1, AddressingMode::Implied),
        OpCode::new(0xf8, "SED", 1, AddressingMode::Implied),

        OpCode::new(0x40, "RTI", 1, AddressingMode::Implied),

        // DEC
        OpCode::new(0xc6, "DEC", 2, AddressingMode::ZeroPage),
        OpCode::new(0xd6, "DEC", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xce, "DEC", 3, AddressingMode::Absolute),
        OpCode::new(0xde, "DEC", 3, AddressingMode::Absolute_X),

        // INC
        OpCode::new(0xe6, "INC", 2, AddressingMode::ZeroPage),
        OpCode::new(0xf6, "INC", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xee, "INC", 3, AddressingMode::Absolute),
        OpCode::new(0xfe, "INC", 3, AddressingMode::Absolute_X),

        OpCode::new(0x4a, "LSR", 1, AddressingMode::Implied),
        OpCode::new(0x46, "LSR", 2, AddressingMode::ZeroPage),
        OpCode::new(0x56, "LSR", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x4e, "LSR", 3, AddressingMode::Absolute),
        OpCode::new(0x5e, "LSR", 3, AddressingMode::Absolute_X),

        // BRK
        OpCode::new(0x00, "BRK", 1, AddressingMode::Implied),

        // Unofficial codes
        OpCode::new(0xc7, "*DCP", 2, AddressingMode::ZeroPage),
        OpCode::new(0xd7, "*DCP", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xCF, "*DCP", 3, AddressingMode::Absolute),
        OpCode::new(0xdF, "*DCP", 3, AddressingMode::Absolute_X),
        OpCode::new(0xdb, "*DCP", 3, AddressingMode::Absolute_Y),
        OpCode::new(0xd3, "*DCP", 2, AddressingMode::Indirect_Y),
        OpCode::new(0xc3, "*DCP", 2, AddressingMode::Indirect_X),


        OpCode::new(0x27, "*RLA", 2, AddressingMode::ZeroPage),
        OpCode::new(0x37, "*RLA", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x2F, "*RLA", 3, AddressingMode::Absolute),
        OpCode::new(0x3F, "*RLA", 3, AddressingMode::Absolute_X),
        OpCode::new(0x3b, "*RLA", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x33, "*RLA", 2, AddressingMode::Indirect_Y),
        OpCode::new(0x23, "*RLA", 2, AddressingMode::Indirect_X),

        OpCode::new(0x07, "*SLO", 2, AddressingMode::ZeroPage),
        OpCode::new(0x17, "*SLO", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x0F, "*SLO", 3, AddressingMode::Absolute),
        OpCode::new(0x1f, "*SLO", 3, AddressingMode::Absolute_X),
        OpCode::new(0x1b, "*SLO", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x03, "*SLO", 2, AddressingMode::Indirect_X),
        OpCode::new(0x13, "*SLO", 2, AddressingMode::Indirect_Y),

        OpCode::new(0x47, "*SRE", 2, AddressingMode::ZeroPage),
        OpCode::new(0x57, "*SRE", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x4F, "*SRE", 3, AddressingMode::Absolute),
        OpCode::new(0x5f, "*SRE", 3, AddressingMode::Absolute_X),
        OpCode::new(0x5b, "*SRE", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x43, "*SRE", 2, AddressingMode::Indirect_X),
        OpCode::new(0x53, "*SRE", 2, AddressingMode::Indirect_Y),


        OpCode::new(0x80, "*NOP", 2, AddressingMode::Immediate),
        OpCode::new(0x82, "*NOP", 2, AddressingMode::Immediate),
        OpCode::new(0x89, "*NOP", 2, AddressingMode::Immediate),
        OpCode::new(0xc2, "*NOP", 2, AddressingMode::Immediate),
        OpCode::new(0xe2, "*NOP", 2, AddressingMode::Immediate),


        OpCode::new(0xCB, "*AXS", 2, AddressingMode::Immediate),

        OpCode::new(0x6B, "*ARR", 2, AddressingMode::Immediate),

        OpCode::new(0xeb, "*SBC", 2, AddressingMode::Immediate),

        OpCode::new(0x0b, "*ANC", 2, AddressingMode::Immediate),
        OpCode::new(0x2b, "*ANC", 2, AddressingMode::Immediate),

        OpCode::new(0x4b, "*ALR", 2, AddressingMode::Immediate),

        OpCode::new(0x04, "*NOP", 2, AddressingMode::ZeroPage),
        OpCode::new(0x44, "*NOP", 2, AddressingMode::ZeroPage),
        OpCode::new(0x64, "*NOP", 2, AddressingMode::ZeroPage),
        OpCode::new(0x14, "*NOP", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x34, "*NOP", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x54, "*NOP", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x74, "*NOP", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xd4, "*NOP", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0xf4, "*NOP", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x0c, "*NOP", 3, AddressingMode::Absolute),
        OpCode::new(0x1c, "*NOP", 3, AddressingMode::Absolute_X),
        OpCode::new(0x3c, "*NOP", 3, AddressingMode::Absolute_X),
        OpCode::new(0x5c, "*NOP", 3, AddressingMode::Absolute_X),
        OpCode::new(0x7c, "*NOP", 3, AddressingMode::Absolute_X),
        OpCode::new(0xdc, "*NOP", 3, AddressingMode::Absolute_X),
        OpCode::new(0xfc, "*NOP", 3, AddressingMode::Absolute_X),

        OpCode::new(0x67, "*RRA", 2, AddressingMode::ZeroPage),
        OpCode::new(0x77, "*RRA", 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x6f, "*RRA", 3, AddressingMode::Absolute),
        OpCode::new(0x7f, "*RRA", 3, AddressingMode::Absolute_X),
        OpCode::new(0x7b, "*RRA", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x63, "*RRA", 2, AddressingMode::Indirect_X),
        OpCode::new(0x73, "*RRA", 2, AddressingMode::Indirect_Y),


        OpCode::new(0xe7, "*ISB", 2,AddressingMode::ZeroPage),
        OpCode::new(0xf7, "*ISB", 2,AddressingMode::ZeroPage_X),
        OpCode::new(0xef, "*ISB", 3,AddressingMode::Absolute),
        OpCode::new(0xff, "*ISB", 3,AddressingMode::Absolute_X),
        OpCode::new(0xfb, "*ISB", 3,AddressingMode::Absolute_Y),
        OpCode::new(0xe3, "*ISB", 2,AddressingMode::Indirect_X),
        OpCode::new(0xf3, "*ISB", 2,AddressingMode::Indirect_Y),

        OpCode::new(0x02, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x12, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x22, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x32, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x42, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x52, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x62, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x72, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x92, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0xb2, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0xd2, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0xf2, "*NOP", 1,AddressingMode::Implied),

        OpCode::new(0x1a, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x3a, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x5a, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0x7a, "*NOP", 1,AddressingMode::Implied),
        OpCode::new(0xda, "*NOP", 1,AddressingMode::Implied),
        // OpCode::new(0xea, "NOP", , AddressingMode::NoneAddressing),
        OpCode::new(0xfa, "*NOP", 1,AddressingMode::Implied),

        OpCode::new(0xab, "*LXA", 2, AddressingMode::Immediate),
        OpCode::new(0x8b, "*XAA", 2, AddressingMode::Immediate),
        OpCode::new(0xbb, "*LAS", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x9b, "*TAS", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x93, "*AHX", 2, AddressingMode::Indirect_Y),
        OpCode::new(0x9f, "*AHX", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x9e, "*SHX", 3, AddressingMode::Absolute_Y),
        OpCode::new(0x9c, "*SHY", 3, AddressingMode::Absolute_X),

        OpCode::new(0xa7, "*LAX", 2, AddressingMode::ZeroPage),
        OpCode::new(0xb7, "*LAX", 2, AddressingMode::ZeroPage_Y),
        OpCode::new(0xaf, "*LAX", 3, AddressingMode::Absolute),
        OpCode::new(0xbf, "*LAX", 3, AddressingMode::Absolute_Y),
        OpCode::new(0xa3, "*LAX", 2, AddressingMode::Indirect_X),
        OpCode::new(0xb3, "*LAX", 2, AddressingMode::Indirect_Y),

        OpCode::new(0x87, "*SAX", 2, AddressingMode::ZeroPage),
        OpCode::new(0x97, "*SAX", 2, AddressingMode::ZeroPage_Y),
        OpCode::new(0x8f, "*SAX", 3, AddressingMode::Absolute),
        OpCode::new(0x83, "*SAX", 2, AddressingMode::Indirect_X),
    ];

    pub static ref OPS_CODES_MAP: HashMap<u8, OpCode> = {
        OPS_CODES
            .iter()
            .map(|x| (x.code, *x))
            .collect()
    };
}
