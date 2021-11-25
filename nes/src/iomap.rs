pub trait IoMap {
    fn peek8(&self, address: u16) -> u8;
    fn poke8(&mut self, address: u16, byte: u8);
    fn peek16(&mut self, address: u16) -> u16 {
        let b0 = self.peek8(address);
        let b1 = self.peek8(address + 1);
        b0 as u16 | (b1 as u16) << 8
    }
}
