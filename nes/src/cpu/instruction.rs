use crate::cpu::flags::*;
use crate::cpu::Cpu;

pub trait Instruction {
    fn lda(&mut self, byte: u8);
    fn ldx(&mut self) {}
    // load index register y from memory
    fn ldy(&mut self) {}

    // store accumulator to memory
    fn sta(&mut self) {}
    // store index register x to memroy
    fn stx(&mut self) {}
    // store index register y to memory
    fn sty(&mut self) {}

    // transfer accumulator to index register x
    fn tax(&mut self) {}
    // transfer accumulator to index register y
    fn tay(&mut self) {}
    // transfer index register x to accumulator
    fn txa(&mut self) {}
    // transfer index register y to accumulator
    fn tya(&mut self) {}

    // jump
    fn jmp_abs(&mut self) {}
    fn jmp_ind(&mut self) {}
    // jump to subroutine
    fn jsr(&mut self) {}
    // return from subroutine
    fn rts(&mut self) {}
    // return from interrupt
    fn rti(&mut self) {}

    // branch if not equal
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

    // add memory and carry to accumulator
    fn adc(&mut self) {}
    // subtract memory with borrow from accumulator
    fn sbc(&mut self) {}
    // add accumulator with memory
    fn and(&mut self) {}
    // or accumulator with memory
    fn ora(&mut self) {}
    // exclusive-or accumulator with memory
    fn eor(&mut self) {}
    // test memory bits against accumulator
    fn bit(&mut self) {}
    // compare accumulator with memory
    fn cmp(&mut self) {}
    // compare index register x with memory
    fn cpx(&mut self) {}
    // compare index register y with memory
    fn cpy(&mut self) {}

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

    // clear carry flag
    fn clc(&mut self) {}
    // set carry flag
    fn sec(&mut self) {}
    // clear decimal mode flag
    fn cld(&mut self) {}
    // set decimal mode flag
    fn sed(&mut self) {}
    // clear overflow flag
    fn clv(&mut self) {}
    // set interrupt-disable flag
    fn sei(&mut self) {}
    // clear interrupt-disable flag
    fn cli(&mut self) {}

    // push accumulator onto stack
    fn pha(&mut self) {}
    // push status flags onto stack
    fn php(&mut self) {}
    // pull accumulator from stack
    fn pla(&mut self) {}
    // pull status flags from stack
    fn plp(&mut self) {}
    // transfer stack pointer to index register x
    fn tsx(&mut self) {}
    // transfer index register x to stack pointer
    fn txs(&mut self) {}

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

    // software break (interrupt)
    fn brk(&mut self) {}
    fn jam(&mut self) {}

    // no operation
    fn nop(&mut self) {}
}

impl Instruction for Cpu {
    fn lda(&mut self, val: u8) {
        self.a = val;
        self.flags.z = z(val);
        self.flags.n = z(val);
    }
    // load index register x from memory
    fn ldx(&mut self) {}
    // load index register y from memory
    fn ldy(&mut self) {}

    // store accumulator to memory
    fn sta(&mut self) {}
    // store index register x to memroy
    fn stx(&mut self) {}
    // store index register y to memory
    fn sty(&mut self) {}

    // transfer accumulator to index register x
    fn tax(&mut self) {
        self.x = self.a;
    }
    // transfer accumulator to index register y
    fn tay(&mut self) {}
    // transfer index register x to accumulator
    fn txa(&mut self) {}
    // transfer index register y to accumulator
    fn tya(&mut self) {}

    // jump
    fn jmp_abs(&mut self) {}
    fn jmp_ind(&mut self) {}
    // jump to subroutine
    fn jsr(&mut self) {}
    // return from subroutine
    fn rts(&mut self) {}
    // return from interrupt
    fn rti(&mut self) {}

    // branch if not equal
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

    // add memory and carry to accumulator
    fn adc(&mut self) {}
    // subtract memory with borrow from accumulator
    fn sbc(&mut self) {}
    // add accumulator with memory
    fn and(&mut self) {}
    // or accumulator with memory
    fn ora(&mut self) {}
    // exclusive-or accumulator with memory
    fn eor(&mut self) {}
    // test memory bits against accumulator
    fn bit(&mut self) {}
    // compare accumulator with memory
    fn cmp(&mut self) {}
    // compare index register x with memory
    fn cpx(&mut self) {}
    // compare index register y with memory
    fn cpy(&mut self) {}

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

    // clear carry flag
    fn clc(&mut self) {}
    // set carry flag
    fn sec(&mut self) {}
    // clear decimal mode flag
    fn cld(&mut self) {}
    // set decimal mode flag
    fn sed(&mut self) {}
    // clear overflow flag
    fn clv(&mut self) {}
    // set interrupt-disable flag
    fn sei(&mut self) {}
    // clear interrupt-disable flag
    fn cli(&mut self) {}

    // push accumulator onto stack
    fn pha(&mut self) {}
    // push status flags onto stack
    fn php(&mut self) {}
    // pull accumulator from stack
    fn pla(&mut self) {}
    // pull status flags from stack
    fn plp(&mut self) {}
    // transfer stack pointer to index register x
    fn tsx(&mut self) {}
    // transfer index register x to stack pointer
    fn txs(&mut self) {}

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

    // software break (interrupt)
    fn brk(&mut self) {}
    fn jam(&mut self) {}

    // no operation
    fn nop(&mut self) {}
}
