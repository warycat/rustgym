use crate::cpu::addr_mode::*;
// use crate::cpu::instruction::Instruction;
use crate::cpu::Cpu;
use crate::iomap::IoMap;

macro_rules! instruction {
    ($name:ident, $instruction:tt, implied) => {
        fn $name(cpu: &mut Cpu) {
            cpu.implied();
            cpu.$instruction();
        }
    };
    ($name:ident, $instruction:tt,$addressing:tt) => {
        fn $name(cpu: &mut Cpu) {
            let addr = cpu.$addressing();
            cpu.$instruction(addr);
        }
    };
}

macro_rules! instruction_read {
    ($name:ident, $instruction:tt, immediate) => {
        fn $name(cpu: &mut Cpu) {
            let byte = cpu.immediate();
            cpu.$instruction(byte);
        }
    };
    ($name:ident, $instruction:tt, relative) => {
        fn $name(cpu: &mut Cpu) {
            let byte = cpu.relative();
            cpu.$instruction(byte);
        }
    };
    ($name:ident, $instruction:tt, $addressing:tt) => {
        fn $name(cpu: &mut Cpu) {
            let addr = cpu.$addressing();
            let byte = cpu.peek8(addr);
            cpu.$instruction(byte);
        }
    };
    ($name:ident, $instruction:tt, $addressing:tt, $indexed:tt) => {
        fn $name(cpu: &mut Cpu) {
            let addr = cpu.$addressing(cpu.$indexed);
            let byte = cpu.peek8(addr);
            cpu.$instruction(byte);
        }
    };
}

macro_rules! instruction_write {
    ($name:ident, $instruction:tt, $addressing:tt) => {
        fn $name(cpu: &mut Cpu) {
            let addr = cpu.$addressing();
            let byte = cpu.$instruction();
            cpu.poke8(addr, byte);
        }
    };
    ($name:ident, $instruction:tt, $addressing:tt, $indexed:tt) => {
        fn $name(cpu: &mut Cpu) {
            let addr = cpu.$addressing(cpu.$indexed);
            let byte = cpu.$instruction();
            cpu.poke8(addr, byte);
        }
    };
}

macro_rules! instruction_read_write {
    ($name:ident, $instruction:tt, accumulator) => {
        fn $name(cpu: &mut Cpu) {
            let byte = cpu.$instruction(cpu.a);
            cpu.a = byte;
        }
    };
    ($name:ident, $instruction:tt, $addressing:tt) => {
        fn $name(cpu: &mut Cpu) {
            let addr = cpu.$addressing();
            let byte = cpu.peek8(addr);
            let byte = cpu.$instruction(byte);
            cpu.poke8(addr, byte);
        }
    };
    ($name:ident, $instruction:tt, $addressing:tt, $indexed:tt) => {
        fn $name(cpu: &mut Cpu) {
            let addr = cpu.$addressing(cpu.$indexed);
            let byte = cpu.peek8(addr);
            let byte = cpu.$instruction(byte);
            cpu.poke8(addr, byte);
        }
    };
}

instruction_read!(xa9, lda, immediate);
instruction_read!(xa5, lda, zero_page);
instruction_read!(xb5, lda, zero_page_indexed, x);
instruction_read!(xad, lda, absolute);
instruction_read!(xbd, lda, absolute_indexed, x);
instruction_read!(xb9, lda, absolute_indexed, y);
instruction_read!(xa1, lda, indexed_indirect, x);
instruction_read!(xb1, lda, indirect_indexed, y);

instruction_read!(xa2, ldx, immediate);
instruction_read!(xa6, ldx, zero_page);
instruction_read!(xb6, ldx, zero_page_indexed, y);
instruction_read!(xae, ldx, absolute);
instruction_read!(xbe, ldx, absolute_indexed, y);

instruction_read!(xa0, ldy, immediate);
instruction_read!(xa4, ldy, zero_page);
instruction_read!(xb4, ldy, zero_page_indexed, x);
instruction_read!(xac, ldy, absolute);
instruction_read!(xbc, ldy, absolute_indexed, x);

instruction_write!(x85, sta, zero_page);
instruction_write!(x95, sta, zero_page_indexed, x);
instruction_write!(x8d, sta, absolute);
instruction_write!(x9d, sta, absolute_indexed, x);
instruction_write!(x99, sta, absolute_indexed, y);
instruction_write!(x81, sta, indexed_indirect, x);
instruction_write!(x91, sta, indirect_indexed, y);

instruction_write!(x86, stx, zero_page);
instruction_write!(x96, stx, zero_page_indexed, y);
instruction_write!(x8e, stx, absolute);

instruction_write!(x84, sty, zero_page);
instruction_write!(x94, sty, zero_page_indexed, x);
instruction_write!(x8c, sty, absolute);

instruction!(xaa, tax, implied);
instruction!(xa8, tay, implied);
instruction!(xba, tsx, implied);
instruction!(x8a, txa, implied);
instruction!(x9a, txs, implied);
instruction!(x98, tya, implied);

instruction!(x4c, jmp, absolute);
instruction!(x6c, jmp, indirect);
instruction!(x20, jsr, absolute);
instruction!(x60, rts, implied);
instruction!(x00, brk, implied);
instruction!(x40, rti, implied);

instruction_read!(x69, adc, immediate);
instruction_read!(x65, adc, zero_page);
instruction_read!(x75, adc, zero_page_indexed, x);
instruction_read!(x6d, adc, absolute);
instruction_read!(x7d, adc, absolute_indexed, x);
instruction_read!(x79, adc, absolute_indexed, y);
instruction_read!(x61, adc, indexed_indirect, x);
instruction_read!(x71, adc, indirect_indexed, y);

instruction_read!(xe9, sbc, immediate);
instruction_read!(xeb, sbc, immediate);
instruction_read!(xe5, sbc, zero_page);
instruction_read!(xf5, sbc, zero_page_indexed, x);
instruction_read!(xed, sbc, absolute);
instruction_read!(xfd, sbc, absolute_indexed, x);
instruction_read!(xf9, sbc, absolute_indexed, y);
instruction_read!(xe1, sbc, indexed_indirect, x);
instruction_read!(xf1, sbc, indirect_indexed, y);

instruction_read!(x29, and, immediate);
instruction_read!(x25, and, zero_page);
instruction_read!(x35, and, zero_page_indexed, x);
instruction_read!(x2d, and, absolute);
instruction_read!(x3d, and, absolute_indexed, x);
instruction_read!(x39, and, absolute_indexed, y);
instruction_read!(x21, and, indexed_indirect, x);
instruction_read!(x31, and, indirect_indexed, y);

instruction_read!(x09, ora, immediate);
instruction_read!(x05, ora, zero_page);
instruction_read!(x15, ora, zero_page_indexed, x);
instruction_read!(x0d, ora, absolute);
instruction_read!(x1d, ora, absolute_indexed, x);
instruction_read!(x19, ora, absolute_indexed, y);
instruction_read!(x01, ora, indexed_indirect, x);
instruction_read!(x11, ora, indirect_indexed, y);

instruction_read!(x49, eor, immediate);
instruction_read!(x45, eor, zero_page);
instruction_read!(x55, eor, zero_page_indexed, x);
instruction_read!(x4d, eor, absolute);
instruction_read!(x5d, eor, absolute_indexed, x);
instruction_read!(x59, eor, absolute_indexed, y);
instruction_read!(x41, eor, indexed_indirect, x);
instruction_read!(x51, eor, indirect_indexed, y);

instruction_read!(x24, bit, zero_page);
instruction_read!(x2c, bit, absolute);

instruction_read!(xc9, cmp, immediate);
instruction_read!(xc5, cmp, zero_page);
instruction_read!(xd5, cmp, zero_page_indexed, x);
instruction_read!(xcd, cmp, absolute);
instruction_read!(xdd, cmp, absolute_indexed, x);
instruction_read!(xd9, cmp, absolute_indexed, y);
instruction_read!(xc1, cmp, indexed_indirect, x);
instruction_read!(xd1, cmp, indirect_indexed, y);
instruction_read!(xe0, cpx, immediate);
instruction_read!(xe4, cpx, zero_page);
instruction_read!(xec, cpx, absolute);
instruction_read!(xc0, cpy, immediate);
instruction_read!(xc4, cpy, zero_page);
instruction_read!(xcc, cpy, absolute);

instruction!(x18, clc, implied);
instruction!(x38, sec, implied);
instruction!(x58, cli, implied);
instruction!(x78, sei, implied);
instruction!(xb8, clv, implied);
instruction!(xd8, cld, implied);
instruction!(xf8, sed, implied);

instruction_read!(x10, bpl, relative);
instruction_read!(x30, bmi, relative);
instruction_read!(x50, bvc, relative);
instruction_read!(x70, bvs, relative);
instruction_read!(x90, bcc, relative);
instruction_read!(xb0, bcs, relative);
instruction_read!(xd0, bne, relative);
instruction_read!(xf0, beq, relative);

instruction_read_write!(x0a, asl, accumulator);
instruction_read_write!(x06, asl, zero_page);
instruction_read_write!(x16, asl, zero_page_indexed, x);
instruction_read_write!(x0e, asl, absolute);
instruction_read_write!(x1e, asl, absolute_indexed, x);

instruction_read_write!(x4a, lsr, accumulator);
instruction_read_write!(x46, lsr, zero_page);
instruction_read_write!(x56, lsr, zero_page_indexed, x);
instruction_read_write!(x4e, lsr, absolute);
instruction_read_write!(x5e, lsr, absolute_indexed, x);

instruction_read_write!(x2a, rol, accumulator);
instruction_read_write!(x26, rol, zero_page);
instruction_read_write!(x36, rol, zero_page_indexed, x);
instruction_read_write!(x2e, rol, absolute);
instruction_read_write!(x3e, rol, absolute_indexed, x);

instruction_read_write!(x6a, ror, accumulator);
instruction_read_write!(x66, ror, zero_page);
instruction_read_write!(x76, ror, zero_page_indexed, x);
instruction_read_write!(x6e, ror, absolute);
instruction_read_write!(x7e, ror, absolute_indexed, x);

instruction_read_write!(xc6, dec, zero_page);
instruction_read_write!(xd6, dec, zero_page_indexed, x);
instruction_read_write!(xce, dec, absolute);
instruction_read_write!(xde, dec, absolute_indexed, x);

instruction_read_write!(xe6, inc, zero_page);
instruction_read_write!(xf6, inc, zero_page_indexed, x);
instruction_read_write!(xee, inc, absolute);
instruction_read_write!(xfe, inc, absolute_indexed, x);

instruction!(xca, dex, implied);
instruction!(x88, dey, implied);
instruction!(xe8, inx, implied);
instruction!(xc8, iny, implied);

instruction!(x48, pha, implied);
instruction!(x08, php, implied);
instruction!(x68, pla, implied);
instruction!(x28, plp, implied);

instruction!(x1a, nop, implied);
instruction!(x3a, nop, implied);
instruction!(x5a, nop, implied);
instruction!(x7a, nop, implied);
instruction!(xda, nop, implied);
instruction!(xea, nop, implied);
instruction!(xfa, nop, implied);

instruction!(x02, jam, implied);
instruction!(x12, jam, implied);
instruction!(x22, jam, implied);
instruction!(x32, jam, implied);
instruction!(x42, jam, implied);
instruction!(x52, jam, implied);
instruction!(x62, jam, implied);
instruction!(x72, jam, implied);
instruction!(x92, jam, implied);
instruction!(xb2, jam, implied);
instruction!(xd2, jam, implied);
instruction!(xf2, jam, implied);

instruction_read!(x0b, anc, immediate);
instruction_read!(x2b, anc, immediate);
instruction_read!(x8b, ane, immediate);
instruction_read!(x6b, arr, immediate);
instruction_read!(x4b, asr, immediate);

instruction_read_write!(xc7, dcp, zero_page);
instruction_read_write!(xd7, dcp, zero_page_indexed, x);
instruction_read_write!(xc3, dcp, indexed_indirect, x);
instruction_read_write!(xd3, dcp, indirect_indexed, y);
instruction_read_write!(xcf, dcp, absolute);
instruction_read_write!(xdf, dcp, absolute_indexed, x);
instruction_read_write!(xdb, dcp, absolute_indexed, y);

instruction_read!(x80, dop, immediate);
instruction_read!(x82, dop, immediate);
instruction_read!(x89, dop, immediate);
instruction_read!(xc2, dop, immediate);
instruction_read!(xe2, dop, immediate);
instruction_read!(x04, dop, zero_page);
instruction_read!(x44, dop, zero_page);
instruction_read!(x64, dop, zero_page);
instruction_read!(x14, dop, zero_page_indexed, x);
instruction_read!(x34, dop, zero_page_indexed, x);
instruction_read!(x54, dop, zero_page_indexed, x);
instruction_read!(x74, dop, zero_page_indexed, x);
instruction_read!(xd4, dop, zero_page_indexed, x);
instruction_read!(xf4, dop, zero_page_indexed, x);

instruction_read_write!(xe7, isb, zero_page);
instruction_read_write!(xf7, isb, zero_page_indexed, x);
instruction_read_write!(xef, isb, absolute);
instruction_read_write!(xff, isb, absolute_indexed, x);
instruction_read_write!(xfb, isb, absolute_indexed, y);
instruction_read_write!(xe3, isb, indexed_indirect, x);
instruction_read_write!(xf3, isb, indirect_indexed, y);

instruction_read!(xbb, las, absolute_indexed, y);

instruction_read!(xa7, lax, zero_page);
instruction_read!(xb7, lax, zero_page_indexed, y);
instruction_read!(xaf, lax, absolute);
instruction_read!(xbf, lax, absolute_indexed, y);
instruction_read!(xa3, lax, indexed_indirect, x);
instruction_read!(xb3, lax, indirect_indexed, y);

instruction_read!(xab, lxa, immediate);

instruction_read_write!(x27, rla, zero_page);
instruction_read_write!(x37, rla, zero_page_indexed, x);
instruction_read_write!(x2f, rla, absolute);
instruction_read_write!(x3f, rla, absolute_indexed, x);
instruction_read_write!(x3b, rla, absolute_indexed, y);
instruction_read_write!(x23, rla, indexed_indirect, x);
instruction_read_write!(x33, rla, indirect_indexed, y);

instruction_read_write!(x67, rra, zero_page);
instruction_read_write!(x77, rra, zero_page_indexed, x);
instruction_read_write!(x6f, rra, absolute);
instruction_read_write!(x7f, rra, absolute_indexed, x);
instruction_read_write!(x7b, rra, absolute_indexed, y);
instruction_read_write!(x63, rra, indexed_indirect, x);
instruction_read_write!(x73, rra, indirect_indexed, y);

instruction_write!(x87, sax, zero_page);
instruction_write!(x97, sax, zero_page_indexed, y);
instruction_write!(x8f, sax, absolute);
instruction_write!(x83, sax, indexed_indirect, x);

instruction_read!(xcb, sbx, immediate);
instruction_write!(x9f, sha, absolute_indexed, y);
instruction_write!(x93, sha, indirect_indexed, y);
instruction_write!(x9b, shs, absolute_indexed, y);
instruction_write!(x9e, shx, absolute);
instruction_write!(x9c, shy, absolute);

instruction_read_write!(x07, slo, zero_page);
instruction_read_write!(x17, slo, zero_page_indexed, x);
instruction_read_write!(x0f, slo, absolute);
instruction_read_write!(x1f, slo, absolute_indexed, x);
instruction_read_write!(x1b, slo, absolute_indexed, y);
instruction_read_write!(x03, slo, indexed_indirect, x);
instruction_read_write!(x13, slo, indirect_indexed, y);

instruction_read_write!(x47, sre, zero_page);
instruction_read_write!(x57, sre, zero_page_indexed, x);
instruction_read_write!(x4f, sre, absolute);
instruction_read_write!(x5f, sre, absolute_indexed, x);
instruction_read_write!(x5b, sre, absolute_indexed, y);
instruction_read_write!(x43, sre, indexed_indirect, x);
instruction_read_write!(x53, sre, indirect_indexed, y);

instruction_read!(x0c, dop, absolute);
instruction_read!(x1c, dop, absolute_indexed, x);
instruction_read!(x3c, dop, absolute_indexed, x);
instruction_read!(x5c, dop, absolute_indexed, x);
instruction_read!(x7c, dop, absolute_indexed, x);
instruction_read!(xdc, dop, absolute_indexed, x);
instruction_read!(xfc, dop, absolute_indexed, x);

pub const OP_TABLE: [fn(&mut Cpu); 0x100] = [
    x00, x01, x02, x03, x04, x05, x06, x07, x08, x09, x0a, x0b, x0c, x0d, x0e, x0f, //
    x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x1a, x1b, x1c, x1d, x1e, x1f, //
    x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x2a, x2b, x2c, x2d, x2e, x2f, //
    x30, x31, x32, x33, x34, x35, x36, x37, x38, x39, x3a, x3b, x3c, x3d, x3e, x3f, //
    x40, x41, x42, x43, x44, x45, x46, x47, x48, x49, x4a, x4b, x4c, x4d, x4e, x4f, //
    x50, x51, x52, x53, x54, x55, x56, x57, x58, x59, x5a, x5b, x5c, x5d, x5e, x5f, //
    x60, x61, x62, x63, x64, x65, x66, x67, x68, x69, x6a, x6b, x6c, x6d, x6e, x6f, //
    x70, x71, x72, x73, x74, x75, x76, x77, x78, x79, x7a, x7b, x7c, x7d, x7e, x7f, //
    x80, x81, x82, x83, x84, x85, x86, x87, x88, x89, x8a, x8b, x8c, x8d, x8e, x8f, //
    x90, x91, x92, x93, x94, x95, x96, x97, x98, x99, x9a, x9b, x9c, x9d, x9e, x9f, //
    xa0, xa1, xa2, xa3, xa4, xa5, xa6, xa7, xa8, xa9, xaa, xab, xac, xad, xae, xaf, //
    xb0, xb1, xb2, xb3, xb4, xb5, xb6, xb7, xb8, xb9, xba, xbb, xbc, xbd, xbe, xbf, //
    xc0, xc1, xc2, xc3, xc4, xc5, xc6, xc7, xc8, xc9, xca, xcb, xcc, xcd, xce, xcf, //
    xd0, xd1, xd2, xd3, xd4, xd5, xd6, xd7, xd8, xd9, xda, xdb, xdc, xdd, xde, xdf, //
    xe0, xe1, xe2, xe3, xe4, xe5, xe6, xe7, xe8, xe9, xea, xeb, xec, xed, xee, xef, //
    xf0, xf1, xf2, xf3, xf4, xf5, xf6, xf7, xf8, xf9, xfa, xfb, xfc, xfd, xfe, xff, //
];

pub struct Opcode(pub u8);

impl Opcode {
    pub fn exec(&self, cpu: &mut Cpu) {
        let index = self.0 as usize;
        let t = &OP_TABLE[index];
        t(cpu)
    }
    pub fn name(&self) -> &str {
        let index = self.0 as usize;
        ""
    }
}
