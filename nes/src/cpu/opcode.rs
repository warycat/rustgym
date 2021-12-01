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

    fn x69(&mut self);
    fn x65(&mut self);
    fn x75(&mut self);
    fn x6d(&mut self);
    fn x7d(&mut self);
    fn x79(&mut self);
    fn x61(&mut self);
    fn x71(&mut self);

    fn xe9(&mut self);
    fn xeb(&mut self);
    fn xe5(&mut self);
    fn xf5(&mut self);
    fn xed(&mut self);
    fn xfd(&mut self);
    fn xf9(&mut self);
    fn xe1(&mut self);
    fn xf1(&mut self);

    fn x29(&mut self);
    fn x25(&mut self);
    fn x35(&mut self);
    fn x2d(&mut self);
    fn x3d(&mut self);
    fn x39(&mut self);
    fn x21(&mut self);
    fn x31(&mut self);

    fn x09(&mut self);
    fn x05(&mut self);
    fn x15(&mut self);
    fn x0d(&mut self);
    fn x1d(&mut self);
    fn x19(&mut self);
    fn x01(&mut self);
    fn x11(&mut self);

    fn x49(&mut self);
    fn x45(&mut self);
    fn x55(&mut self);
    fn x4d(&mut self);
    fn x5d(&mut self);
    fn x59(&mut self);
    fn x41(&mut self);
    fn x51(&mut self);

    fn x24(&mut self);
    fn x2c(&mut self);
    fn xc9(&mut self);
    fn xc5(&mut self);
    fn xd5(&mut self);
    fn xcd(&mut self);
    fn xdd(&mut self);
    fn xd9(&mut self);
    fn xc1(&mut self);
    fn xd1(&mut self);
    fn xe0(&mut self);
    fn xe4(&mut self);
    fn xec(&mut self);
    fn xc0(&mut self);
    fn xc4(&mut self);
    fn xcc(&mut self);

    fn x18(&mut self);
    fn x38(&mut self);
    fn x58(&mut self);
    fn x78(&mut self);
    fn xb8(&mut self);
    fn xd8(&mut self);
    fn xf8(&mut self);
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

    instruction_read!(x69, adc);
    instruction_read!(x65, adc, zero_page);
    instruction_read!(x75, adc, zero_page_indexed, x);
    instruction_read!(x6d, adc, absolute);
    instruction_read!(x7d, adc, absolute_indexed, x);
    instruction_read!(x79, adc, absolute_indexed, y);
    instruction_read!(x61, adc, indirect_indexed, x);
    instruction_read!(x71, adc, indirect_indexed, y);

    instruction_read!(xe9, sbc);
    instruction_read!(xeb, sbc);
    instruction_read!(xe5, sbc, zero_page);
    instruction_read!(xf5, sbc, zero_page_indexed, x);
    instruction_read!(xed, sbc, absolute);
    instruction_read!(xfd, sbc, absolute_indexed, x);
    instruction_read!(xf9, sbc, absolute_indexed, y);
    instruction_read!(xe1, sbc, indirect_indexed, x);
    instruction_read!(xf1, sbc, indirect_indexed, y);

    instruction_read!(x29, and);
    instruction_read!(x25, and, zero_page);
    instruction_read!(x35, and, zero_page_indexed, x);
    instruction_read!(x2d, and, absolute);
    instruction_read!(x3d, and, absolute_indexed, x);
    instruction_read!(x39, and, absolute_indexed, y);
    instruction_read!(x21, and, indirect_indexed, x);
    instruction_read!(x31, and, indirect_indexed, y);

    instruction_read!(x09, ora);
    instruction_read!(x05, ora, zero_page);
    instruction_read!(x15, ora, zero_page_indexed, x);
    instruction_read!(x0d, ora, absolute);
    instruction_read!(x1d, ora, absolute_indexed, x);
    instruction_read!(x19, ora, absolute_indexed, y);
    instruction_read!(x01, ora, indirect_indexed, x);
    instruction_read!(x11, ora, indirect_indexed, y);

    instruction_read!(x49, eor);
    instruction_read!(x45, eor, zero_page);
    instruction_read!(x55, eor, zero_page_indexed, x);
    instruction_read!(x4d, eor, absolute);
    instruction_read!(x5d, eor, absolute_indexed, x);
    instruction_read!(x59, eor, absolute_indexed, y);
    instruction_read!(x41, eor, indirect_indexed, x);
    instruction_read!(x51, eor, indirect_indexed, y);

    instruction_read!(x24, bit, zero_page);
    instruction_read!(x2c, bit, absolute);

    instruction_read!(xc9, cmp);
    instruction_read!(xc5, cmp, zero_page);
    instruction_read!(xd5, cmp, zero_page_indexed, x);
    instruction_read!(xcd, cmp, absolute);
    instruction_read!(xdd, cmp, absolute_indexed, x);
    instruction_read!(xd9, cmp, absolute_indexed, y);
    instruction_read!(xc1, cmp, indirect_indexed, x);
    instruction_read!(xd1, cmp, indirect_indexed, y);
    instruction_read!(xe0, cpx);
    instruction_read!(xe4, cpx, zero_page);
    instruction_read!(xec, cpx, absolute);
    instruction_read!(xc0, cpy);
    instruction_read!(xc4, cpy, zero_page);
    instruction_read!(xcc, cpy, absolute);

    instruction!(x18, clc);
    instruction!(x38, sec);
    instruction!(x58, cli);
    instruction!(x78, sei);
    instruction!(xb8, clv);
    instruction!(xd8, cld);
    instruction!(xf8, sed);
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
