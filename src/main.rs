use crate::cpu::CPU;

mod addressing_mode;
mod cpu;
mod flags;
mod mem;
mod opcodes;

fn main() {
    let mut cpu = CPU::new();
    cpu.load_n_run(&[
        0xA9, 0x01, // LDA #$01   (A = 1)
        0xE9, 0x01, // SBC #$01   (A = 1 - 1 - 1 = -1 if C=0 by default)
        0x00, // BRK
    ]);
}
