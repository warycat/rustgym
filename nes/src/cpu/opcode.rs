use crate::cpu::addressing::Addressing;
use crate::cpu::instruction::Instruction;
use crate::cpu::Cpu;
use crate::iomap::IoMap;

macro_rules! instruction {
    ($name:ident, $instruction:tt) => {
        fn $name(cpu: &mut Cpu) {
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
    ($name:ident, $instruction:tt) => {
        fn $name(cpu: &mut Cpu) {
            let byte = cpu.$instruction();
            cpu.a = byte;
        }
    };
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
    ($name:ident, $instruction:tt) => {
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
instruction_read!(xa1, lda, indirect_indexed, x);
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

instruction!(x4c, jmp, absolute);
instruction!(x6c, jmp, indirect);
instruction!(x20, jsr, absolute);
instruction!(x60, rts);
instruction!(x00, brk);
instruction!(x40, rti);

instruction_read!(x69, adc, immediate);
instruction_read!(x65, adc, zero_page);
instruction_read!(x75, adc, zero_page_indexed, x);
instruction_read!(x6d, adc, absolute);
instruction_read!(x7d, adc, absolute_indexed, x);
instruction_read!(x79, adc, absolute_indexed, y);
instruction_read!(x61, adc, indirect_indexed, x);
instruction_read!(x71, adc, indirect_indexed, y);

instruction_read!(xe9, sbc, immediate);
instruction_read!(xeb, sbc, immediate);
instruction_read!(xe5, sbc, zero_page);
instruction_read!(xf5, sbc, zero_page_indexed, x);
instruction_read!(xed, sbc, absolute);
instruction_read!(xfd, sbc, absolute_indexed, x);
instruction_read!(xf9, sbc, absolute_indexed, y);
instruction_read!(xe1, sbc, indirect_indexed, x);
instruction_read!(xf1, sbc, indirect_indexed, y);

instruction_read!(x29, and, immediate);
instruction_read!(x25, and, zero_page);
instruction_read!(x35, and, zero_page_indexed, x);
instruction_read!(x2d, and, absolute);
instruction_read!(x3d, and, absolute_indexed, x);
instruction_read!(x39, and, absolute_indexed, y);
instruction_read!(x21, and, indirect_indexed, x);
instruction_read!(x31, and, indirect_indexed, y);

instruction_read!(x09, ora, immediate);
instruction_read!(x05, ora, zero_page);
instruction_read!(x15, ora, zero_page_indexed, x);
instruction_read!(x0d, ora, absolute);
instruction_read!(x1d, ora, absolute_indexed, x);
instruction_read!(x19, ora, absolute_indexed, y);
instruction_read!(x01, ora, indirect_indexed, x);
instruction_read!(x11, ora, indirect_indexed, y);

instruction_read!(x49, eor, immediate);
instruction_read!(x45, eor, zero_page);
instruction_read!(x55, eor, zero_page_indexed, x);
instruction_read!(x4d, eor, absolute);
instruction_read!(x5d, eor, absolute_indexed, x);
instruction_read!(x59, eor, absolute_indexed, y);
instruction_read!(x41, eor, indirect_indexed, x);
instruction_read!(x51, eor, indirect_indexed, y);

instruction_read!(x24, bit, zero_page);
instruction_read!(x2c, bit, absolute);

instruction_read!(xc9, cmp, immediate);
instruction_read!(xc5, cmp, zero_page);
instruction_read!(xd5, cmp, zero_page_indexed, x);
instruction_read!(xcd, cmp, absolute);
instruction_read!(xdd, cmp, absolute_indexed, x);
instruction_read!(xd9, cmp, absolute_indexed, y);
instruction_read!(xc1, cmp, indirect_indexed, x);
instruction_read!(xd1, cmp, indirect_indexed, y);
instruction_read!(xe0, cpx, immediate);
instruction_read!(xe4, cpx, zero_page);
instruction_read!(xec, cpx, absolute);
instruction_read!(xc0, cpy, immediate);
instruction_read!(xc4, cpy, zero_page);
instruction_read!(xcc, cpy, absolute);

instruction!(x18, clc);
instruction!(x38, sec);
instruction!(x58, cli);
instruction!(x78, sei);
instruction!(xb8, clv);
instruction!(xd8, cld);
instruction!(xf8, sed);

instruction_read!(x10, bpl, relative);
instruction_read!(x30, bmi, relative);
instruction_read!(x50, bvc, relative);
instruction_read!(x70, bvs, relative);
instruction_read!(x90, bcc, relative);
instruction_read!(xb0, bcs, relative);
instruction_read!(xd0, bne, relative);
instruction_read!(xf0, beq, relative);

instruction_read_write!(x0a, asl);
instruction_read_write!(x06, asl, zero_page);
instruction_read_write!(x16, asl, zero_page_indexed, x);
instruction_read_write!(x0e, asl, absolute);
instruction_read_write!(x1e, asl, absolute_indexed, x);

instruction_read_write!(x4a, lsr);
instruction_read_write!(x46, lsr, zero_page);
instruction_read_write!(x56, lsr, zero_page_indexed, x);
instruction_read_write!(x4e, lsr, absolute);
instruction_read_write!(x5e, lsr, absolute_indexed, x);

instruction_read_write!(x2a, rol);
instruction_read_write!(x26, rol, zero_page);
instruction_read_write!(x36, rol, zero_page_indexed, x);
instruction_read_write!(x2e, rol, absolute);
instruction_read_write!(x3e, rol, absolute_indexed, x);

instruction_read_write!(x6a, ror);
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

instruction!(xca, dex);
instruction!(x88, dey);
instruction!(xe8, inx);
instruction!(xc8, iny);

instruction!(x48, pha);
instruction!(x08, php);
instruction!(x68, pla);
instruction!(x28, plp);

instruction!(x1a, nop);
instruction!(x3a, nop);
instruction!(x5a, nop);
instruction!(x7a, nop);
instruction!(xda, nop);
instruction!(xea, nop);
instruction!(xfa, nop);

instruction!(x02, jam);
instruction!(x12, jam);
instruction!(x22, jam);
instruction!(x32, jam);
instruction!(x42, jam);
instruction!(x52, jam);
instruction!(x62, jam);
instruction!(x72, jam);
instruction!(x92, jam);
instruction!(xb2, jam);
instruction!(xd2, jam);
instruction!(xf2, jam);

instruction!(x0b, jam);
instruction!(x2b, jam);
instruction!(x8b, jam);
instruction!(x6b, jam);
instruction!(x4b, jam);
instruction!(xc7, jam);
instruction!(xd7, jam);
instruction!(xc3, jam);
instruction!(xd3, jam);
instruction!(xcf, jam);
instruction!(xdf, jam);
instruction!(xdb, jam);
instruction!(x80, jam);
instruction!(x82, jam);
instruction!(x89, jam);
instruction!(xc2, jam);
instruction!(xe2, jam);
instruction!(x04, jam);
instruction!(x44, jam);
instruction!(x64, jam);
instruction!(x14, jam);
instruction!(x34, jam);
instruction!(x54, jam);
instruction!(x74, jam);
instruction!(xd4, jam);
instruction!(xf4, jam);
instruction!(xe7, jam);
instruction!(xf7, jam);
instruction!(xef, jam);
instruction!(xff, jam);
instruction!(xfb, jam);
instruction!(xe3, jam);
instruction!(xf3, jam);
instruction!(xbb, jam);
instruction!(xa7, jam);
instruction!(xb7, jam);
instruction!(xaf, jam);
instruction!(xbf, jam);
instruction!(xa3, jam);
instruction!(xb3, jam);
instruction!(xab, jam);
instruction!(x27, jam);
instruction!(x37, jam);
instruction!(x2f, jam);
instruction!(x3f, jam);
instruction!(x3b, jam);
instruction!(x23, jam);
instruction!(x33, jam);
instruction!(x67, jam);
instruction!(x77, jam);
instruction!(x6f, jam);
instruction!(x7f, jam);
instruction!(x7b, jam);
instruction!(x63, jam);
instruction!(x73, jam);
instruction!(x87, jam);
instruction!(x97, jam);
instruction!(x8f, jam);
instruction!(x83, jam);
instruction!(xcb, jam);
instruction!(x9f, jam);
instruction!(x93, jam);
instruction!(x9b, jam);
instruction!(x9e, jam);
instruction!(x9c, jam);
instruction!(x07, jam);
instruction!(x17, jam);
instruction!(x0f, jam);
instruction!(x1f, jam);
instruction!(x1b, jam);
instruction!(x03, jam);
instruction!(x13, jam);
instruction!(x47, jam);
instruction!(x57, jam);
instruction!(x4f, jam);
instruction!(x5f, jam);
instruction!(x5b, jam);
instruction!(x43, jam);
instruction!(x53, jam);
instruction!(x0c, jam);
instruction!(x1c, jam);
instruction!(x3c, jam);
instruction!(x5c, jam);
instruction!(x7c, jam);
instruction!(xdc, jam);
instruction!(xfc, jam);

pub const TABLE: [(fn(&mut Cpu), &str); 0x100] = [
    (x00, "BRK"),
    (x01, "ORA"),
    (x02, "HLT"),
    (x03, "ASO"),
    (x04, "SKB"),
    (x05, "ORA"),
    (x06, "ASL"),
    (x07, "ASO"),
    (x08, "PHP"),
    (x09, "ORA"),
    (x0a, "ASL"),
    (x0b, "ANC"),
    (x0c, "SKW"),
    (x0d, "ORA"),
    (x0e, "ASL"),
    (x0f, "ASO"),
    (x10, "BPL"),
    (x11, "ORA"),
    (x12, "HLT"),
    (x13, "ASO"),
    (x14, "SKB"),
    (x15, "ORA"),
    (x16, "ASL"),
    (x17, "ASO"),
    (x18, "CLC"),
    (x19, "ORA"),
    (x1a, "NOP"),
    (x1b, "ASO"),
    (x1c, "SKW"),
    (x1d, "ORA"),
    (x1e, "ASL"),
    (x1f, "ASO"),
    (x20, "JSR"),
    (x21, "AND"),
    (x22, "HLT"),
    (x23, "RLA"),
    (x24, "BIT"),
    (x25, "AND"),
    (x26, "ROL"),
    (x27, "RLA"),
    (x28, "PLP"),
    (x29, "AND"),
    (x2a, "ROL"),
    (x2b, "ANC"),
    (x2c, "BIT"),
    (x2d, "AND"),
    (x2e, "ROL"),
    (x2f, "RLA"),
    (x30, "BMI"),
    (x31, "AND"),
    (x32, "HLT"),
    (x33, "RLA"),
    (x34, "SKB"),
    (x35, "AND"),
    (x36, "ROL"),
    (x37, "RLA"),
    (x38, "SEC"),
    (x39, "AND"),
    (x3a, "NOP"),
    (x3b, "RLA"),
    (x3c, "SKW"),
    (x3d, "AND"),
    (x3e, "ROL"),
    (x3f, "RLA"),
    (x40, "RTI"),
    (x41, "EOR"),
    (x42, "HLT"),
    (x43, "LSE"),
    (x44, "SKB"),
    (x45, "EOR"),
    (x46, "LSR"),
    (x47, "LSE"),
    (x48, "PHA"),
    (x49, "EOR"),
    (x4a, "LSR"),
    (x4b, "ALR"),
    (x4c, "JMP"),
    (x4d, "EOR"),
    (x4e, "LSR"),
    (x4f, "LSE"),
    (x50, "BVC"),
    (x51, "EOR"),
    (x52, "HLT"),
    (x53, "LSE"),
    (x54, "SKB"),
    (x55, "EOR"),
    (x56, "LSR"),
    (x57, "LSE"),
    (x58, "CLI"),
    (x59, "EOR"),
    (x5a, "NOP"),
    (x5b, "LSE"),
    (x5c, "SKW"),
    (x5d, "EOR"),
    (x5e, "LSR"),
    (x5f, "LSE"),
    (x60, "RTS"),
    (x61, "ADC"),
    (x62, "HLT"),
    (x63, "RRA"),
    (x64, "SKB"),
    (x65, "ADC"),
    (x66, "ROR"),
    (x67, "RRA"),
    (x68, "PLA"),
    (x69, "ADC"),
    (x6a, "ROR"),
    (x6b, "ARR"),
    (x6c, "JMP"),
    (x6d, "ADC"),
    (x6e, "ROR"),
    (x6f, "RRA"),
    (x70, "BVS"),
    (x71, "ADC"),
    (x72, "HLT"),
    (x73, "RRA"),
    (x74, "SKB"),
    (x75, "ADC"),
    (x76, "ROR"),
    (x77, "RRA"),
    (x78, "SEI"),
    (x79, "ADC"),
    (x7a, "NOP"),
    (x7b, "RRA"),
    (x7c, "SKW"),
    (x7d, "ADC"),
    (x7e, "ROR"),
    (x7f, "RRA"),
    (x80, "SKB"),
    (x81, "STA"),
    (x82, "SKB"),
    (x83, "SAX"),
    (x84, "STY"),
    (x85, "STA"),
    (x86, "STX"),
    (x87, "SAX"),
    (x88, "DEY"),
    (x89, "SKB"),
    (x8a, "TXA"),
    (x8b, "ANE"),
    (x8c, "STY"),
    (x8d, "STA"),
    (x8e, "STX"),
    (x8f, "SAX"),
    (x90, "BCC"),
    (x91, "STA"),
    (x92, "HLT"),
    (x93, "SHA"),
    (x94, "STY"),
    (x95, "STA"),
    (x96, "STX"),
    (x97, "SAX"),
    (x98, "TYA"),
    (x99, "STA"),
    (x9a, "TXS"),
    (x9b, "SHS"),
    (x9c, "SHY"),
    (x9d, "STA"),
    (x9e, "SHX"),
    (x9f, "SHA"),
    (xa0, "LDY"),
    (xa1, "LDA"),
    (xa2, "LDX"),
    (xa3, "LAX"),
    (xa4, "LDY"),
    (xa5, "LDA"),
    (xa6, "LDX"),
    (xa7, "LAX"),
    (xa8, "TAY"),
    (xa9, "LDA"),
    (xaa, "TAX"),
    (xab, "ANX"),
    (xac, "LDY"),
    (xad, "LDA"),
    (xae, "LDX"),
    (xaf, "LAX"),
    (xb0, "BCS"),
    (xb1, "LDA"),
    (xb2, "HLT"),
    (xb3, "LAX"),
    (xb4, "LDY"),
    (xb5, "LDA"),
    (xb6, "LDX"),
    (xb7, "LAX"),
    (xb8, "CLV"),
    (xb9, "LDA"),
    (xba, "TSX"),
    (xbb, "LAS"),
    (xbc, "LDY"),
    (xbd, "LDA"),
    (xbe, "LDX"),
    (xbf, "LAX"),
    (xc0, "CPY"),
    (xc1, "CMP"),
    (xc2, "SKB"),
    (xc3, "DCM"),
    (xc4, "CPY"),
    (xc5, "CMP"),
    (xc6, "DEC"),
    (xc7, "DCM"),
    (xc8, "INY"),
    (xc9, "CMP"),
    (xca, "DEX"),
    (xcb, "SBX"),
    (xcc, "CPY"),
    (xcd, "CMP"),
    (xce, "DEC"),
    (xcf, "DCM"),
    (xd0, "BNE"),
    (xd1, "CMP"),
    (xd2, "HLT"),
    (xd3, "DCM"),
    (xd4, "SKB"),
    (xd5, "CMP"),
    (xd6, "DEC"),
    (xd7, "DCM"),
    (xd8, "CLD"),
    (xd9, "CMP"),
    (xda, "NOP"),
    (xdb, "DCM"),
    (xdc, "SKW"),
    (xdd, "CMP"),
    (xde, "DEC"),
    (xdf, "DCM"),
    (xe0, "CPX"),
    (xe1, "SBC"),
    (xe2, "SKB"),
    (xe3, "INS"),
    (xe4, "CPX"),
    (xe5, "SBC"),
    (xe6, "INC"),
    (xe7, "INS"),
    (xe8, "INX"),
    (xe9, "SBC"),
    (xea, "NOP"),
    (xeb, "SBC"),
    (xec, "CPX"),
    (xed, "SBC"),
    (xee, "INC"),
    (xef, "INS"),
    (xf0, "BEQ"),
    (xf1, "SBC"),
    (xf2, "HLT"),
    (xf3, "INS"),
    (xf4, "SKB"),
    (xf5, "SBC"),
    (xf6, "INC"),
    (xf7, "INS"),
    (xf8, "SED"),
    (xf9, "SBC"),
    (xfa, "NOP"),
    (xfb, "INS"),
    (xfc, "SKW"),
    (xfd, "SBC"),
    (xfe, "INC"),
    (xff, "INS"),
];

pub struct Opcode(pub u8);

impl Opcode {
    pub fn exec(&self, cpu: &mut Cpu) {
        let index = self.0 as usize;
        TABLE[index].0(cpu)
    }
    pub fn name(&self) -> &str {
        let index = self.0 as usize;
        TABLE[index].1
    }
}
