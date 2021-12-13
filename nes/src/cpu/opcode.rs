use crate::cpu::addressing::*;
use crate::cpu::instruction::Instruction;
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

use Addressing::*;

pub const TABLE: [(fn(&mut Cpu), &str, Addressing); 0x100] = [
    (x00, "BRK", IMM),
    (x01, "ORA", IZX),
    (x02, "HLT", IMP),
    (x03, "SLO", IMP),
    (x04, "NOP", IMP),
    (x05, "ORA", ZP0),
    (x06, "ASL", ZP0),
    (x07, "SLO", IMP),
    (x08, "PHP", IMP),
    (x09, "ORA", IMM),
    (x0a, "ASL", IMP),
    (x0b, "ANC", IMP),
    (x0c, "NOP", IMP),
    (x0d, "ORA", ABS),
    (x0e, "ASL", ABS),
    (x0f, "SLO", IMP),
    (x10, "BPL", REL),
    (x11, "ORA", IZY),
    (x12, "HLT", IMP),
    (x13, "SLO", IMP),
    (x14, "NOP", IMP),
    (x15, "ORA", ZPX),
    (x16, "ASL", ZPX),
    (x17, "SLO", IMP),
    (x18, "CLC", IMP),
    (x19, "ORA", ABY),
    (x1a, "NOP", IMP),
    (x1b, "SLO", IMP),
    (x1c, "NOP", IMP),
    (x1d, "ORA", ABX),
    (x1e, "ASL", ABX),
    (x1f, "SLO", IMP),
    (x20, "JSR", ABS),
    (x21, "AND", IZX),
    (x22, "HLT", IMP),
    (x23, "RLA", IMP),
    (x24, "BIT", ZP0),
    (x25, "AND", ZP0),
    (x26, "ROL", ZP0),
    (x27, "RLA", IMP),
    (x28, "PLP", IMP),
    (x29, "AND", IMM),
    (x2a, "ROL", IMP),
    (x2b, "ANC", IMP),
    (x2c, "BIT", ABS),
    (x2d, "AND", ABS),
    (x2e, "ROL", ABS),
    (x2f, "RLA", IMP),
    (x30, "BMI", REL),
    (x31, "AND", IZY),
    (x32, "HLT", IMP),
    (x33, "RLA", IMP),
    (x34, "NOP", IMP),
    (x35, "AND", ZPX),
    (x36, "ROL", ZPX),
    (x37, "RLA", IMP),
    (x38, "SEC", IMP),
    (x39, "AND", ABY),
    (x3a, "NOP", IMP),
    (x3b, "RLA", IMP),
    (x3c, "NOP", IMP),
    (x3d, "AND", ABX),
    (x3e, "ROL", ABX),
    (x3f, "RLA", IMP),
    (x40, "RTI", IMP),
    (x41, "EOR", IZX),
    (x42, "HLT", IMP),
    (x43, "SRE", IMP),
    (x44, "NOP", IMP),
    (x45, "EOR", ZP0),
    (x46, "LSR", ZP0),
    (x47, "SRE", IMP),
    (x48, "PHA", IMP),
    (x49, "EOR", IMM),
    (x4a, "LSR", IMP),
    (x4b, "ALR", IMP),
    (x4c, "JMP", ABS),
    (x4d, "EOR", ABS),
    (x4e, "LSR", ABS),
    (x4f, "SRE", IMP),
    (x50, "BVC", REL),
    (x51, "EOR", IZY),
    (x52, "HLT", IMP),
    (x53, "SRE", IMP),
    (x54, "NOP", IMP),
    (x55, "EOR", ZPX),
    (x56, "LSR", ZPX),
    (x57, "SRE", IMP),
    (x58, "CLI", IMP),
    (x59, "EOR", ABY),
    (x5a, "NOP", IMP),
    (x5b, "SRE", IMP),
    (x5c, "NOP", IMP),
    (x5d, "EOR", ABX),
    (x5e, "LSR", ABX),
    (x5f, "SRE", IMP),
    (x60, "RTS", IMP),
    (x61, "ADC", IZX),
    (x62, "HLT", IMP),
    (x63, "RRA", IMP),
    (x64, "NOP", IMP),
    (x65, "ADC", ZP0),
    (x66, "ROR", ZP0),
    (x67, "RRA", IMP),
    (x68, "PLA", IMP),
    (x69, "ADC", IMM),
    (x6a, "ROR", IMP),
    (x6b, "ARR", IMP),
    (x6c, "JMP", IND),
    (x6d, "ADC", ABS),
    (x6e, "ROR", ABS),
    (x6f, "RRA", IMP),
    (x70, "BVS", REL),
    (x71, "ADC", IZY),
    (x72, "HLT", IMP),
    (x73, "RRA", IMP),
    (x74, "NOP", IMP),
    (x75, "ADC", ZPX),
    (x76, "ROR", ZPX),
    (x77, "RRA", IMP),
    (x78, "SEI", IMP),
    (x79, "ADC", ABY),
    (x7a, "NOP", IMP),
    (x7b, "RRA", IMP),
    (x7c, "NOP", IMP),
    (x7d, "ADC", ABX),
    (x7e, "ROR", ABX),
    (x7f, "RRA", IMP),
    (x80, "NOP", IMP),
    (x81, "STA", IZX),
    (x82, "NOP", IMP),
    (x83, "SAX", IMP),
    (x84, "STY", ZP0),
    (x85, "STA", ZP0),
    (x86, "STX", ZP0),
    (x87, "SAX", IMP),
    (x88, "DEY", IMP),
    (x89, "NOP", IMP),
    (x8a, "TXA", IMP),
    (x8b, "ANE", IMP),
    (x8c, "STY", ABS),
    (x8d, "STA", ABS),
    (x8e, "STX", ABS),
    (x8f, "SAX", IMP),
    (x90, "BCC", REL),
    (x91, "STA", IZY),
    (x92, "HLT", IMP),
    (x93, "SHA", IMP),
    (x94, "STY", ZPX),
    (x95, "STA", ZPX),
    (x96, "STX", ZPY),
    (x97, "SAX", IMP),
    (x98, "TYA", IMP),
    (x99, "STA", ABY),
    (x9a, "TXS", IMP),
    (x9b, "SHS", IMP),
    (x9c, "SHY", IMP),
    (x9d, "STA", ABX),
    (x9e, "SHX", IMP),
    (x9f, "SHA", IMP),
    (xa0, "LDY", IMM),
    (xa1, "LDA", IZX),
    (xa2, "LDX", IMM),
    (xa3, "LAX", IMP),
    (xa4, "LDY", ZP0),
    (xa5, "LDA", ZP0),
    (xa6, "LDX", ZP0),
    (xa7, "LAX", IMP),
    (xa8, "TAY", IMP),
    (xa9, "LDA", IMM),
    (xaa, "TAX", IMP),
    (xab, "ANX", IMP),
    (xac, "LDY", ABS),
    (xad, "LDA", ABS),
    (xae, "LDX", ABS),
    (xaf, "LAX", IMP),
    (xb0, "BCS", REL),
    (xb1, "LDA", IZY),
    (xb2, "HLT", IMP),
    (xb3, "LAX", IMP),
    (xb4, "LDY", ZPX),
    (xb5, "LDA", ZPX),
    (xb6, "LDX", ZPY),
    (xb7, "LAX", IMP),
    (xb8, "CLV", IMP),
    (xb9, "LDA", ABY),
    (xba, "TSX", IMP),
    (xbb, "LAS", IMP),
    (xbc, "LDY", ABX),
    (xbd, "LDA", ABX),
    (xbe, "LDX", ABY),
    (xbf, "LAX", IMP),
    (xc0, "CPY", IMM),
    (xc1, "CMP", IZX),
    (xc2, "NOP", IMP),
    (xc3, "DCP", IMP),
    (xc4, "CPY", ZP0),
    (xc5, "CMP", ZP0),
    (xc6, "DEC", ZP0),
    (xc7, "DCP", IMP),
    (xc8, "INY", IMP),
    (xc9, "CMP", IMM),
    (xca, "DEX", IMP),
    (xcb, "SBX", IMP),
    (xcc, "CPY", ABS),
    (xcd, "CMP", ABS),
    (xce, "DEC", ABS),
    (xcf, "DCP", IMP),
    (xd0, "BNE", REL),
    (xd1, "CMP", IZY),
    (xd2, "HLT", IMP),
    (xd3, "DCP", IMP),
    (xd4, "NOP", IMP),
    (xd5, "CMP", ZPX),
    (xd6, "DEC", ZPX),
    (xd7, "DCP", IMP),
    (xd8, "CLD", IMP),
    (xd9, "CMP", ABY),
    (xda, "NOP", IMP),
    (xdb, "DCP", IMP),
    (xdc, "NOP", IMP),
    (xdd, "CMP", ABX),
    (xde, "DEC", ABX),
    (xdf, "DCP", IMP),
    (xe0, "CPX", IMM),
    (xe1, "SBC", IZX),
    (xe2, "NOP", IMP),
    (xe3, "ISB", IMP),
    (xe4, "CPX", ZP0),
    (xe5, "SBC", ZP0),
    (xe6, "INC", ZP0),
    (xe7, "ISB", IMP),
    (xe8, "INX", IMP),
    (xe9, "SBC", IMM),
    (xea, "NOP", IMP),
    (xeb, "SBC", IMP),
    (xec, "CPX", ABS),
    (xed, "SBC", ABS),
    (xee, "INC", ABS),
    (xef, "ISB", IMP),
    (xf0, "BEQ", REL),
    (xf1, "SBC", IZY),
    (xf2, "HLT", IMP),
    (xf3, "ISB", IMP),
    (xf4, "NOP", IMP),
    (xf5, "SBC", ZPX),
    (xf6, "INC", ZPX),
    (xf7, "ISB", IMP),
    (xf8, "SED", IMP),
    (xf9, "SBC", ABY),
    (xfa, "NOP", IMP),
    (xfb, "ISB", IMP),
    (xfc, "NOP", IMP),
    (xfd, "SBC", ABX),
    (xfe, "INC", ABX),
    (xff, "ISB", IMP),
];

pub struct Opcode(pub u8);

impl Opcode {
    pub fn exec(&self, cpu: &mut Cpu) {
        let index = self.0 as usize;
        let t = &TABLE[index];
        match t.2 {
            IZX => {
                //println!("{} (${:02X},X)", t.1, cpu.peek8(cpu.pc));
            }
            IZY => {
                // println!("{} (${:02X}),Y", t.1, cpu.peek8(cpu.pc));
            }
            _ => {}
        }
        t.0(cpu)
    }
    pub fn name(&self) -> &str {
        let index = self.0 as usize;
        TABLE[index].1
    }
}
