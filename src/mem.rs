pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    // reads a 16-bit memory in little endian order
    // ex:
    //  LDA $8000 <=> A9 00 80
    //  since NES uses little endian, the CPU will read 0x00 (least significant) first then 0x80 (most significant)
    //  since people write numbers from the most significant part first, we get 0x8000
    fn mem_read_u16(&self, addr: u16) -> u16 {
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
}
