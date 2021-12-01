use crate::base::*;
use crate::cpu::flags::*;
use crate::cpu::Cpu;
use crate::iomap::IoMap;
use std::num::Wrapping;

pub trait Instruction {
    // LoaD Accumulator
    fn lda(&mut self, byte: u8);
    // LoaD X register
    fn ldx(&mut self, byte: u8);
    // LoaD Y register
    fn ldy(&mut self, byte: u8);

    // STore Accumulator
    fn sta(&mut self) -> u8;
    // STore X register
    fn stx(&mut self) -> u8;
    // STore Y register
    fn sty(&mut self) -> u8;

    // Transfer A to X
    fn tax(&mut self);
    // Transfer X to A
    fn txa(&mut self);
    // Transfer A to Y
    fn tay(&mut self);
    // Transfer Y to A
    fn tya(&mut self);
    // Transfer Stack ptr to X
    fn tsx(&mut self);
    // Transfer X to Stack ptr
    fn txs(&mut self);

    // JuMP absolute
    fn jmpa(&mut self);
    // JuMP indirect
    fn jmpi(&mut self);
    // Jump to SubRoutine
    fn jsr(&mut self);
    // ReTurn from Subroutine
    fn rts(&mut self);
    // BReaK
    fn brk(&mut self);
    // ReTurn from Interrupt
    fn rti(&mut self);

    // ADd with Carry
    fn adc(&mut self, byte: u8);
    // SuBtract with Carry
    fn sbc(&mut self, byte: u8);
    // bitwise AND with accumulator
    fn and(&mut self, byte: u8);
    // bitwise OR with Accumulator
    fn ora(&mut self, byte: u8);
    // bitwise Exclusive OR
    fn eor(&mut self, byte: u8);
    // test BITs
    fn bit(&mut self, byte: u8);
    // CoMPare accumulator
    fn cmp(&mut self, byte: u8);
    // ComPare X register
    fn cpx(&mut self, byte: u8);
    // ComPare Y register
    fn cpy(&mut self, byte: u8);

    // CLear Carry
    fn clc(&mut self);
    // SEt Carry
    fn sec(&mut self);
    // SEt Interrupt
    fn sei(&mut self);
    // CLear Interrupt
    fn cli(&mut self);
    // CLear oVerflow
    fn clv(&mut self);
    // CLear Decimal
    fn cld(&mut self);
    // SEt Decimal
    fn sed(&mut self);

    // Branch on Not Equal
    fn bne(&mut self);
    // Branch on EQual
    fn beq(&mut self);
    // Branch on MInus
    fn bmi(&mut self);
    // Branch on PLus
    fn bpl(&mut self);
    // Branch on Carry Set
    fn bcs(&mut self);
    // Branch on Carry Clear
    fn bcc(&mut self);
    // Branch on oVerflow Set
    fn bvs(&mut self);
    // Branch on oVerflow Clear
    fn bvc(&mut self);

    // shift memory or accumulator left one bit
    fn asl(&mut self);
    // logical shift memory or accumulator right
    fn lsr(&mut self);
    // rotate memory or accumulator left one bit
    fn rol(&mut self);
    // rotate memory or accumulator right one bit
    fn ror(&mut self);
    // decrement
    fn dec(&mut self);
    // increment
    fn inc(&mut self);
    // decrement index register x
    fn dex(&mut self);
    // decrement index register y
    fn dey(&mut self);
    // increment index register x
    fn inx(&mut self);
    // increment index register y
    fn iny(&mut self);

    // push accumulator onto stack
    fn pha(&mut self);
    // push status flags onto stack
    fn php(&mut self);
    // pull accumulator from stack
    fn pla(&mut self);
    // pull status flags from stack
    fn plp(&mut self);

    fn anc(&mut self);
    fn ane(&mut self);
    fn arr(&mut self);
    fn asr(&mut self);
    fn dcp(&mut self);
    fn lsb(&mut self);
    fn las(&mut self);
    fn lax(&mut self);
    fn lxa(&mut self);
    fn rla(&mut self);
    fn rra(&mut self);
    fn sax(&mut self);
    fn sbx(&mut self);
    fn sha(&mut self);
    fn shs(&mut self);
    fn shx(&mut self);
    fn shy(&mut self);
    fn slo(&mut self);
    fn sre(&mut self);
    fn dop(&mut self);
    fn top(&mut self);

    fn jam(&mut self);

    // no operation
    fn nop(&mut self);
}

impl Instruction for Cpu {
    fn lda(&mut self, byte: u8) {
        self.a = self.flags.set_zn(byte);
    }
    fn ldx(&mut self, byte: u8) {
        self.x = self.flags.set_zn(byte);
    }
    fn ldy(&mut self, byte: u8) {
        self.y = self.flags.set_zn(byte);
    }
    fn sta(&mut self) -> u8 {
        self.a
    }
    fn stx(&mut self) -> u8 {
        self.x
    }
    fn sty(&mut self) -> u8 {
        self.y
    }
    fn tax(&mut self) {
        self.x = self.flags.set_zn(self.a);
    }
    fn tay(&mut self) {
        self.y = self.flags.set_zn(self.a);
    }
    fn txa(&mut self) {
        self.a = self.flags.set_zn(self.x);
    }
    fn tya(&mut self) {
        self.a = self.flags.set_zn(self.y);
    }
    fn tsx(&mut self) {
        self.x = self.flags.set_zn(self.s);
    }
    fn txs(&mut self) {
        self.s = self.x;
    }
    fn jmpa(&mut self) {
        self.pc = self.peek16(self.pc);
    }
    fn jmpi(&mut self) {
        let addr = self.peek16(self.pc);
        let lo = self.peek8(addr) as u16;
        let hi = self.peek8(addr & 0xFF00 | (addr + 1) & 0x00FF) as u16;
        self.pc = lo | hi << 8;
    }
    fn jsr(&mut self) {
        self.push16(self.pc + 1);
        self.pc = self.peek16(self.pc);
    }
    fn rts(&mut self) {
        self.pc = self.pop16() + 1;
    }
    fn brk(&mut self) {
        self.push16(self.pc + 1);
        self.push8(self.flags.pack() | B);
        self.flags.i = I;
        self.pc = self.peek16(IRQ_VECTOR);
    }
    fn rti(&mut self) {
        let packed = self.pop8();
        self.flags.unpack(packed);
        self.pc = self.pop16();
    }
    fn adc(&mut self, byte: u8) {
        let word = self.a as u16 + byte as u16 + self.flags.c as u16;
        let res = word as u8;
        self.flags.v = overflow(self.a, byte, res);
        self.flags.c = carry16(word, 0x100);
        self.a = self.flags.set_zn(res);
    }
    fn sbc(&mut self, byte: u8) {
        self.adc(byte ^ 0xFF)
    }
    fn and(&mut self, byte: u8) {
        let byte = self.a & byte;
        self.a = self.flags.set_zn(byte);
    }
    fn ora(&mut self, byte: u8) {
        let byte = self.a | byte;
        self.a = self.flags.set_zn(byte);
    }
    fn eor(&mut self, byte: u8) {
        let byte = self.a | byte;
        self.a = self.flags.set_zn(byte);
    }
    fn bit(&mut self, byte: u8) {
        let byte = self.a & byte;
        self.flags.set_zn(byte);
        self.flags.v = byte & V;
    }
    fn cmp(&mut self, byte: u8) {
        let word = (Wrapping(self.a as u16) - Wrapping(byte as u16)).0;
        self.flags.c = carry16(!word, 0x100);
        self.flags.set_zn(word as u8);
    }
    fn cpx(&mut self, byte: u8) {
        let word = (Wrapping(self.x as u16) - Wrapping(byte as u16)).0;
        self.flags.c = carry16(!word, 0x100);
        self.flags.set_zn(word as u8);
    }
    fn cpy(&mut self, byte: u8) {
        let word = (Wrapping(self.y as u16) - Wrapping(byte as u16)).0;
        self.flags.c = carry16(!word, 0x100);
        self.flags.set_zn(word as u8);
    }
    fn clc(&mut self) {
        self.flags.c = 0;
    }
    fn sec(&mut self) {
        self.flags.c = C;
    }
    fn cli(&mut self) {
        self.flags.i = 0;
    }
    fn sei(&mut self) {
        self.flags.i = I;
    }
    fn clv(&mut self) {
        self.flags.v = 0;
    }
    fn cld(&mut self) {
        self.flags.d = 0;
    }
    fn sed(&mut self) {
        self.flags.d = D;
    }

    fn bne(&mut self) {}
    // branch if equal
    fn beq(&mut self) {}
    // branch if negative
    fn bmi(&mut self) {}
    // branch if plus
    fn bpl(&mut self) {}
    // branch if carry set
    fn bcs(&mut self) {}
    // branch if carry clear
    fn bcc(&mut self) {}
    // branch if overflow set
    fn bvs(&mut self) {}
    // branch if overflow clear
    fn bvc(&mut self) {}

    // shift memory or accumulator left one bit
    fn asl(&mut self) {}
    // logical shift memory or accumulator right
    fn lsr(&mut self) {}
    // rotate memory or accumulator left one bit
    fn rol(&mut self) {}
    // rotate memory or accumulator right one bit
    fn ror(&mut self) {}
    // decrement
    fn dec(&mut self) {}
    // increment
    fn inc(&mut self) {}
    // decrement index register x
    fn dex(&mut self) {}
    // decrement index register y
    fn dey(&mut self) {}
    // increment index register x
    fn inx(&mut self) {}
    // increment index register y
    fn iny(&mut self) {}

    // push accumulator onto stack
    fn pha(&mut self) {}
    // push status flags onto stack
    fn php(&mut self) {}
    // pull accumulator from stack
    fn pla(&mut self) {}
    // pull status flags from stack
    fn plp(&mut self) {}
    // transfer stack pointer to index register x

    fn anc(&mut self) {}
    fn ane(&mut self) {}
    fn arr(&mut self) {}
    fn asr(&mut self) {}
    fn dcp(&mut self) {}
    fn lsb(&mut self) {}
    fn las(&mut self) {}
    fn lax(&mut self) {}
    fn lxa(&mut self) {}
    fn rla(&mut self) {}
    fn rra(&mut self) {}
    fn sax(&mut self) {}
    fn sbx(&mut self) {}
    fn sha(&mut self) {}
    fn shs(&mut self) {}
    fn shx(&mut self) {}
    fn shy(&mut self) {}
    fn slo(&mut self) {}
    fn sre(&mut self) {}
    fn dop(&mut self) {}
    fn top(&mut self) {}

    fn jam(&mut self) {}

    // no operation
    fn nop(&mut self) {}
}
