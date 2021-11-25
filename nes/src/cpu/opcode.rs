use crate::cpu::addressing::Addressing;
use crate::cpu::instruction::Instruction;
use crate::cpu::Cpu;

trait Opcode {
    fn op0xA9(&mut self);
    fn op0xA5(&mut self);
    fn op0xB5(&mut self);
    fn op0xAD(&mut self);
    fn op0xBD(&mut self);
    fn op0xB9(&mut self);
    // fn op0xA1(&mut self);
}

impl Opcode for Cpu {
    fn op0xA9(&mut self) {
        let byte = self.immediate_read();
        self.lda(byte);
    }
    fn op0xA5(&mut self) {
        let byte = self.zero_page_read();
        self.lda(byte);
    }
    fn op0xB5(&mut self) {
        let byte = self.zero_page_indexed_read(self.x);
        self.lda(byte)
    }
    fn op0xAD(&mut self) {
        let byte = self.absolute_read();
        self.lda(byte)
    }
    fn op0xBD(&mut self) {
        let byte = self.absolute_indexed_read(self.x);
        self.lda(byte)
    }
    fn op0xB9(&mut self) {
        let byte = self.absolute_indexed_read(self.y);
        self.lda(byte)
    }
    // fn op0xA1(&mut self) {
    //     let byte = self.indirect_indexed_read(self.x);
    //     self.lda(byte)
    // }
}
