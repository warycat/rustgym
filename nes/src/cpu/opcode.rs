use crate::cpu::addressing::Addressing;
use crate::cpu::instruction::Instruction;
use crate::cpu::Cpu;
use crate::iomap::IoMap;

trait Opcode {
    fn xa9(&mut self);
    fn xa5(&mut self);
    fn xb5(&mut self);
    fn xad(&mut self);
    fn xbd(&mut self);
    fn xb9(&mut self);
    fn xa1(&mut self);
    fn xb1(&mut self);

    fn xa2(&mut self);
    fn xa6(&mut self);
    fn xb6(&mut self);
    fn xae(&mut self);
    fn xbe(&mut self);

    fn xa0(&mut self);
    fn xa4(&mut self);
    fn xb4(&mut self);
    fn xac(&mut self);
    fn xbc(&mut self);

    fn x85(&mut self);
    fn x95(&mut self);
    fn x8d(&mut self);
    fn x9d(&mut self);
    fn x99(&mut self);
    fn x81(&mut self);
    fn x91(&mut self);

    fn x86(&mut self);
    fn x96(&mut self);
    fn x8e(&mut self);
    fn x84(&mut self);
    fn x94(&mut self);
    fn x8c(&mut self);

    fn xaa(&mut self);
    fn xa8(&mut self);
    fn xba(&mut self);
    fn x8a(&mut self);
    fn x9a(&mut self);
    fn x98(&mut self);

    fn x4c(&mut self);
    fn x6c(&mut self);
    fn x20(&mut self);
    fn x60(&mut self);
    fn x00(&mut self);
    fn x40(&mut self);
}

macro_rules! instruction_read {
    ($name:ident, $instruction:tt) => {
        fn $name(&mut self) {
            let byte = self.immediate();
            self.$instruction(byte);
        }
    };
    ($name:ident, $instruction:tt, $addressing:tt) => {
        fn $name(&mut self) {
            let addr = self.$addressing();
            let byte = self.peek8(addr);
            self.$instruction(byte);
        }
    };
    ($name:ident, $instruction:tt, $addressing:tt, $indexed:tt) => {
        fn $name(&mut self) {
            let addr = self.$addressing(self.$indexed);
            let byte = self.peek8(addr);
            self.$instruction(byte);
        }
    };
}

macro_rules! instruction_write {
    ($name:ident, $instruction:tt, $addressing:tt) => {
        fn $name(&mut self) {
            let addr = self.$addressing();
            let byte = self.$instruction();
            self.poke8(addr, byte);
        }
    };
    ($name:ident, $instruction:tt, $addressing:tt, $indexed:tt) => {
        fn $name(&mut self) {
            let addr = self.$addressing(self.$indexed);
            let byte = self.$instruction();
            self.poke8(addr, byte);
        }
    };
}

macro_rules! instruction {
    ($name:ident, $instruction:tt) => {
        fn $name(&mut self) {
            self.$instruction();
        }
    };
}

impl Opcode for Cpu {
    instruction_read!(xa9, lda);
    instruction_read!(xa5, lda, zero_page);
    instruction_read!(xb5, lda, zero_page_indexed, x);
    instruction_read!(xad, lda, absolute);
    instruction_read!(xbd, lda, absolute_indexed, x);
    instruction_read!(xb9, lda, absolute_indexed, y);
    instruction_read!(xa1, lda, indirect_indexed, x);
    instruction_read!(xb1, lda, indirect_indexed, y);

    instruction_read!(xa2, ldx);
    instruction_read!(xa6, ldx, zero_page);
    instruction_read!(xb6, ldx, zero_page_indexed, y);
    instruction_read!(xae, ldx, absolute);
    instruction_read!(xbe, ldx, absolute_indexed, y);

    instruction_read!(xa0, ldy);
    instruction_read!(xa4, ldy, zero_page);
    instruction_read!(xb4, ldy, zero_page_indexed, x);
    instruction_read!(xac, ldy, absolute);
    instruction_read!(xbc, ldy, absolute_indexed, x);

    instruction_write!(x85, sta, zero_page);
    instruction_write!(x95, sta, zero_page_indexed, x);
    instruction_write!(x8d, sta, absolute);
    instruction_write!(x9d, sta, absolute_indexed, x);
    instruction_write!(x99, sta, absolute_indexed, y);
    instruction_write!(x81, sta, indirect_indexed, x);
    instruction_write!(x91, sta, indirect_indexed, y);

    instruction_write!(x86, stx, zero_page);
    instruction_write!(x96, stx, zero_page_indexed, y);
    instruction_write!(x8e, stx, absolute);

    instruction_write!(x84, sty, zero_page);
    instruction_write!(x94, sty, zero_page_indexed, x);
    instruction_write!(x8c, sty, absolute);

    instruction!(xaa, tax);
    instruction!(xa8, tay);
    instruction!(xba, tsx);
    instruction!(x8a, txa);
    instruction!(x9a, txs);
    instruction!(x98, tya);

    instruction!(x4c, jmpa);
    instruction!(x6c, jmpi);
    instruction!(x20, jsr);
    instruction!(x60, rts);
    instruction!(x00, brk);
    instruction!(x40, rti);
}

#[test]
fn test_xa9() {
    let mut cpu = Cpu::new();
    cpu.bus.poke8(0, b'a');
    cpu.xa9();
    assert_eq!(cpu.a, b'a');
}

#[test]
fn test_xa5() {
    let mut cpu = Cpu::new();
    cpu.bus.poke8(0, b'a');
    cpu.bus.poke8(b'a' as u16, b'b');
    cpu.xa5();
    assert_eq!(cpu.a, b'b');
}

#[test]
fn test_x85() {
    let mut cpu = Cpu::new();
    cpu.a = b'a';
    cpu.bus.poke8(0, b'b');
    cpu.x85();
    assert_eq!(cpu.peek8(b'b' as u16), b'a');
}
