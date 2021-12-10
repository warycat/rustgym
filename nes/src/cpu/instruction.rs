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
    fn jmp(&mut self, word: u16);
    // Jump to SubRoutine
    fn jsr(&mut self, word: u16);
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

    // Branch on PLus
    fn bpl(&mut self, byte: i8);
    // Branch on MInus
    fn bmi(&mut self, byte: i8);
    // Branch on oVerflow Clear
    fn bvc(&mut self, byte: i8);
    // Branch on oVerflow Set
    fn bvs(&mut self, byte: i8);
    // Branch on Carry Clear
    fn bcc(&mut self, byte: i8);
    // Branch on Carry Set
    fn bcs(&mut self, byte: i8);
    // Branch on EQual
    fn beq(&mut self, byte: i8);
    // Branch on Not Equal
    fn bne(&mut self, byte: i8);

    // Arithmetic Shift Left
    fn asl(&mut self, byte: u8) -> u8;
    // Logical Shift Right
    fn lsr(&mut self, byte: u8) -> u8;
    // ROtate Left
    fn rol(&mut self, byte: u8) -> u8;
    // ROtate Right
    fn ror(&mut self, byte: u8) -> u8;
    // DECrement memory
    fn dec(&mut self, byte: u8) -> u8;
    // INCrement memory
    fn inc(&mut self, byte: u8) -> u8;

    // DEcrement X
    fn dex(&mut self);
    // INcrement X
    fn inx(&mut self);
    // DEcrement Y
    fn dey(&mut self);
    // INcrement Y
    fn iny(&mut self);

    // PusH Accumulator
    fn pha(&mut self);
    // PuLl Accumulator
    fn pla(&mut self);
    // push status flags onto stack
    fn php(&mut self);
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
    fn jmp(&mut self, word: u16) {
        self.pc = word;
    }
    fn jsr(&mut self, word: u16) {
        self.push16(word - 1);
        self.pc = word;
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

    fn bpl(&mut self, byte: i8) {
        if self.flags.n == 0 {
            self.pc = self.pc.wrapping_add(byte as u16);
        }
    }
    fn bmi(&mut self, byte: i8) {
        if self.flags.n == N {
            self.pc = self.pc.wrapping_add(byte as u16);
        }
    }
    fn bvc(&mut self, byte: i8) {
        if self.flags.v == 0 {
            self.pc = self.pc.wrapping_add(byte as u16);
        }
    }
    fn bvs(&mut self, byte: i8) {
        if self.flags.v == V {
            self.pc = self.pc.wrapping_add(byte as u16);
        }
    }
    fn bcc(&mut self, byte: i8) {
        if self.flags.c == 0 {
            self.pc = self.pc.wrapping_add(byte as u16);
        }
    }
    fn bcs(&mut self, byte: i8) {
        if self.flags.c == C {
            self.pc = self.pc.wrapping_add(byte as u16);
        }
    }
    fn bne(&mut self, byte: i8) {
        if self.flags.z == 0 {
            self.pc = self.pc.wrapping_add(byte as u16);
        }
    }
    fn beq(&mut self, byte: i8) {
        if self.flags.z == Z {
            self.pc = self.pc.wrapping_add(byte as u16);
        }
    }
    fn asl(&mut self, byte: u8) -> u8 {
        self.flags.c = carry8(byte, 0x80);
        self.flags.set_zn(byte << 1)
    }
    fn lsr(&mut self, byte: u8) -> u8 {
        self.flags.c = carry8(byte, 0x01);
        self.flags.set_zn(byte >> 1)
    }
    fn rol(&mut self, byte: u8) -> u8 {
        let c = self.flags.c;
        self.flags.c = carry8(byte, 0x80);
        self.flags.set_zn(byte << 1 | c)
    }
    fn ror(&mut self, byte: u8) -> u8 {
        let c = self.flags.c;
        self.flags.c = carry8(byte, 0x01);
        self.flags.set_zn(byte >> 1 | c << 7)
    }
    fn dec(&mut self, byte: u8) -> u8 {
        let byte = byte.wrapping_sub(1);
        self.flags.set_zn(byte)
    }
    fn inc(&mut self, byte: u8) -> u8 {
        let byte = byte.wrapping_add(1);
        self.flags.set_zn(byte)
    }
    fn dex(&mut self) {
        let byte = self.x.wrapping_sub(1);
        self.x = self.flags.set_zn(byte);
    }
    fn dey(&mut self) {
        let byte = self.y.wrapping_sub(1);
        self.y = self.flags.set_zn(byte);
    }
    fn inx(&mut self) {
        let byte = self.x.wrapping_add(1);
        self.x = self.flags.set_zn(byte);
    }
    fn iny(&mut self) {
        let byte = self.y.wrapping_add(1);
        self.y = self.flags.set_zn(byte);
    }

    fn pha(&mut self) {
        let byte = self.a;
        self.push8(byte);
    }
    fn pla(&mut self) {
        let byte = self.pop8();
        self.a = self.flags.set_zn(byte);
    }
    fn php(&mut self) {
        let byte = self.flags.pack();
        self.push8(byte);
    }
    fn plp(&mut self) {
        let byte = self.pop8();
        self.flags.unpack(byte);
    }

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
