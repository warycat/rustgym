use crate::*;
use log::{debug, error, info};

macro_rules! iif {
    ($cond:expr, $iftrue:expr, $iffalse:expr) => {
        if $cond {
            $iftrue
        } else {
            $iffalse
        }
    };
}

use AddrMode::*;

type Func = fn(&mut Console);

fn and(console: &mut Console) {
    let byte = console.cpu.a & Cpu::get_operand_value(console);
    console.cpu.set_a(byte);
}

fn eor(console: &mut Console) {
    let byte = console.cpu.a ^ Cpu::get_operand_value(console);
    console.cpu.set_a(byte);
}

fn ora(console: &mut Console) {
    let byte = console.cpu.a | Cpu::get_operand_value(console);
    console.cpu.set_a(byte);
}

fn adc(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.add(byte);
}

fn sbc(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.add(byte ^ 0xFF);
}

fn cpa(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.cmp(console.cpu.a, byte)
}

fn cpx(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.cmp(console.cpu.x, byte)
}

fn cpy(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.cmp(console.cpu.y, byte)
}

fn inc(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    console.cpu.clear_flags(PsFlags::Negative | PsFlags::Zero);
    let byte = Cpu::memory_read(console, addr);
    Cpu::memory_write(console, addr, byte);
    let byte = byte.wrapping_add(1);
    console.cpu.set_zero_negative_flags(byte);
    Cpu::memory_write(console, addr, byte);
}

fn dec(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    console.cpu.clear_flags(PsFlags::Negative | PsFlags::Zero);
    let byte = Cpu::memory_read(console, addr);
    Cpu::memory_write(console, addr, byte);
    let byte = byte.wrapping_sub(1);
    console.cpu.set_zero_negative_flags(byte);
    Cpu::memory_write(console, addr, byte);
}

fn bit(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console
        .cpu
        .clear_flags(PsFlags::Zero | PsFlags::Overflow | PsFlags::Negative);
    if console.cpu.a & byte == 0x00 {
        console.cpu.set_flags(PsFlags::Zero);
    }
    if byte & 0x40 == 0x40 {
        console.cpu.set_flags(PsFlags::Overflow);
    }
    if byte & 0x80 == 0x80 {
        console.cpu.set_flags(PsFlags::Negative);
    }
}

fn lda(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.set_a(byte);
}

fn ldx(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.set_x(byte);
}

fn ldy(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.set_y(byte);
}

fn sta(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    Cpu::memory_write(console, addr, console.cpu.a);
}

fn stx(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    Cpu::memory_write(console, addr, console.cpu.x);
}

fn sty(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    Cpu::memory_write(console, addr, console.cpu.y);
}

fn tax(console: &mut Console) {
    let byte = console.cpu.a;
    console.cpu.set_x(byte);
}

fn tay(console: &mut Console) {
    let byte = console.cpu.a;
    console.cpu.set_y(byte);
}

fn tsx(console: &mut Console) {
    let byte = console.cpu.sp;
    console.cpu.set_x(byte);
}

fn txa(console: &mut Console) {
    let byte = console.cpu.x;
    console.cpu.set_a(byte);
}

fn txs(console: &mut Console) {
    let byte = console.cpu.x;
    console.cpu.set_sp(byte);
}

fn tya(console: &mut Console) {
    let byte = console.cpu.y;
    console.cpu.set_a(byte);
}

fn pha(console: &mut Console) {
    let byte = console.cpu.a;
    Cpu::push_byte(console, byte);
}

fn php(console: &mut Console) {
    let byte = console.cpu.ps | PsFlags::Break | PsFlags::Reserved;
    Cpu::push_byte(console, byte.bits());
}

fn pla(console: &mut Console) {
    Cpu::dummy_read(console);
    let byte = Cpu::pop_byte(console);
    console.cpu.set_a(byte);
}

fn plp(console: &mut Console) {
    Cpu::dummy_read(console);
    let byte = Cpu::pop_byte(console);
    console.cpu.set_ps(byte);
}

fn inx(console: &mut Console) {
    let byte = console.cpu.x.wrapping_add(1);
    console.cpu.set_x(byte);
}

fn iny(console: &mut Console) {
    let byte = console.cpu.y.wrapping_add(1);
    console.cpu.set_y(byte);
}

fn dex(console: &mut Console) {
    let byte = console.cpu.x.wrapping_sub(1);
    console.cpu.set_x(byte);
}

fn dey(console: &mut Console) {
    let byte = console.cpu.y.wrapping_sub(1);
    console.cpu.set_y(byte);
}

fn asl(console: &mut Console) {
    use Operand::*;
    match console.cpu.operand {
        Accumulator => {
            let byte = console.cpu.a;
            let byte = console.cpu.asl(byte);
            console.cpu.set_a(byte);
        }
        Address(addr) => {
            let byte = Cpu::memory_read(console, addr);
            Cpu::memory_write(console, addr, byte);
            let byte = console.cpu.asl(byte);
            Cpu::memory_write(console, addr, byte);
        }
        _ => panic!(),
    }
}

fn lsr(console: &mut Console) {
    use Operand::*;
    match console.cpu.operand {
        Accumulator => {
            let byte = console.cpu.a;
            let byte = console.cpu.lsr(byte);
            console.cpu.set_a(byte);
        }
        Address(addr) => {
            let byte = Cpu::memory_read(console, addr);
            Cpu::memory_write(console, addr, byte);
            let byte = console.cpu.lsr(byte);
            Cpu::memory_write(console, addr, byte);
        }
        _ => panic!(),
    }
}

fn rol(console: &mut Console) {
    use Operand::*;
    match console.cpu.operand {
        Accumulator => {
            let byte = console.cpu.a;
            let byte = console.cpu.rol(byte);
            console.cpu.set_a(byte);
        }
        Address(addr) => {
            let byte = Cpu::memory_read(console, addr);
            Cpu::memory_write(console, addr, byte);
            let byte = console.cpu.rol(byte);
            Cpu::memory_write(console, addr, byte);
        }
        _ => panic!(),
    }
}

fn ror(console: &mut Console) {
    use Operand::*;
    match console.cpu.operand {
        Accumulator => {
            let byte = console.cpu.a;
            let byte = console.cpu.ror(byte);
            console.cpu.set_a(byte);
        }
        Address(addr) => {
            let byte = Cpu::memory_read(console, addr);
            Cpu::memory_write(console, addr, byte);
            let byte = console.cpu.ror(byte);
            Cpu::memory_write(console, addr, byte);
        }
        _ => panic!(),
    }
}

fn jmp(console: &mut Console) {
    let byte = console.cpu.get_operand_addr();
    console.cpu.set_pc(byte);
}

fn jsr(console: &mut Console) {
    Cpu::dummy_read(console);
    Cpu::push_word(console, console.cpu.pc - 1);
    jmp(console);
}

fn rts(console: &mut Console) {
    let addr = Cpu::pop_word(console);
    Cpu::dummy_read(console);
    Cpu::dummy_read(console);
    console.cpu.set_pc(addr + 1);
}

fn bpl(console: &mut Console) {
    Cpu::branch_relative(console, !console.cpu.check_flag(PsFlags::Negative));
}

fn bmi(console: &mut Console) {
    Cpu::branch_relative(console, console.cpu.check_flag(PsFlags::Negative));
}

fn bvc(console: &mut Console) {
    Cpu::branch_relative(console, !console.cpu.check_flag(PsFlags::Overflow));
}

fn bvs(console: &mut Console) {
    Cpu::branch_relative(console, console.cpu.check_flag(PsFlags::Overflow));
}

fn bcc(console: &mut Console) {
    Cpu::branch_relative(console, !console.cpu.check_flag(PsFlags::Carry));
}

fn bcs(console: &mut Console) {
    Cpu::branch_relative(console, console.cpu.check_flag(PsFlags::Carry));
}

fn bne(console: &mut Console) {
    Cpu::branch_relative(console, !console.cpu.check_flag(PsFlags::Zero));
}

fn beq(console: &mut Console) {
    Cpu::branch_relative(console, console.cpu.check_flag(PsFlags::Zero));
}

fn clc(console: &mut Console) {
    console.cpu.clear_flags(PsFlags::Carry);
}

fn sec(console: &mut Console) {
    console.cpu.set_flags(PsFlags::Carry);
}

fn cli(console: &mut Console) {
    console.cpu.clear_flags(PsFlags::Interrupt);
}

fn sei(console: &mut Console) {
    console.cpu.set_flags(PsFlags::Interrupt);
}

fn clv(console: &mut Console) {
    console.cpu.clear_flags(PsFlags::Overflow);
}

fn cld(console: &mut Console) {
    console.cpu.clear_flags(PsFlags::Decimal);
}

fn sed(console: &mut Console) {
    console.cpu.set_flags(PsFlags::Decimal);
}

fn brk(console: &mut Console) {
    let word = console.cpu.pc + 1;
    Cpu::push_word(console, word);
    let flags = console.cpu.ps | PsFlags::Break | PsFlags::Reserved;
    if console.cpu.need_nmi {
        console.cpu.need_nmi = false;
        Cpu::push_byte(console, flags.bits());
        console.cpu.set_flags(PsFlags::Interrupt);
        let word = Cpu::memory_read_word(console, NMI_VECTOR);
        console.cpu.set_pc(word);
    } else {
        Cpu::push_byte(console, flags.bits());
        console.cpu.set_flags(PsFlags::Interrupt);
        let word = Cpu::memory_read_word(console, IRQ_VECTOR);
        console.cpu.set_pc(word);
    }
    console.cpu.prev_need_nmi = false;
}

fn rti(console: &mut Console) {
    Cpu::dummy_read(console);
    let byte = Cpu::pop_byte(console);
    console.cpu.set_ps(byte);
    let word = Cpu::pop_word(console);
    console.cpu.set_pc(word);
}

fn nop(console: &mut Console) {
    Cpu::get_operand_value(console);
}

fn slo(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    let addr = console.cpu.get_operand_addr();
    Cpu::memory_write(console, addr, byte);
    let byte = console.cpu.asl(byte);
    console.cpu.set_a(console.cpu.a | byte);
    Cpu::memory_write(console, addr, byte);
}

fn sre(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    let addr = console.cpu.get_operand_addr();
    Cpu::memory_write(console, addr, byte);
    let byte = console.cpu.lsr(byte);
    console.cpu.set_a(console.cpu.a ^ byte);
    Cpu::memory_write(console, addr, byte);
}

fn rla(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    let addr = console.cpu.get_operand_addr();
    Cpu::memory_write(console, addr, byte);
    let byte = console.cpu.rol(byte);
    console.cpu.set_a(console.cpu.a & byte);
    Cpu::memory_write(console, addr, byte);
}

fn rra(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    let addr = console.cpu.get_operand_addr();
    Cpu::memory_write(console, addr, byte);
    let byte = console.cpu.ror(byte);
    console.cpu.add(byte);
    Cpu::memory_write(console, addr, byte);
}

fn sax(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    let byte = console.cpu.a & console.cpu.x;
    Cpu::memory_write(console, addr, byte);
}

fn lax(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.set_x(byte);
    console.cpu.set_a(byte);
}

fn dcp(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    let addr = console.cpu.get_operand_addr();
    Cpu::memory_write(console, addr, byte);
    let byte = byte.wrapping_sub(1);
    console.cpu.cmp(console.cpu.a, byte);
    Cpu::memory_write(console, addr, byte);
}

fn isb(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    let addr = console.cpu.get_operand_addr();
    Cpu::memory_write(console, addr, byte);
    let byte = byte.wrapping_add(1);
    console.cpu.add(byte ^ 0xFF);
    Cpu::memory_write(console, addr, byte);
}

fn aac(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.set_a(console.cpu.a & byte);
    console.cpu.clear_flags(PsFlags::Carry);
    if console.cpu.check_flag(PsFlags::Negative) {
        console.cpu.set_flags(PsFlags::Carry);
    }
    todo!()
}

fn asr(console: &mut Console) {
    console.cpu.clear_flags(PsFlags::Carry);
    let byte = Cpu::get_operand_value(console);
    console.cpu.set_a(console.cpu.a & byte);
    if console.cpu.a & 0x01 == 0x01 {
        console.cpu.set_flags(PsFlags::Carry);
    }
    console.cpu.set_a(console.cpu.a >> 1);
    todo!()
}

fn arr(console: &mut Console) {
    let byte = (console.cpu.a & Cpu::get_operand_value(console)) >> 1
        | iif!(console.cpu.check_flag(PsFlags::Carry), 0x80, 0x00);
    console.cpu.set_a(byte);
    console.cpu.clear_flags(PsFlags::Carry | PsFlags::Overflow);
    if console.cpu.a & 0x40 == 0x40 {
        console.cpu.set_flags(PsFlags::Carry);
    }
    if iif!(console.cpu.check_flag(PsFlags::Carry), 0x01, 0x00) ^ (console.cpu.a >> 5 & 0x01) != 0 {
        console.cpu.set_flags(PsFlags::Overflow);
    }
    todo!()
}

fn atx(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.set_a(byte);
    console.cpu.set_x(byte);
    todo!()
}

fn axs(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    let res = (console.cpu.a & console.cpu.x) - byte;
    console.cpu.clear_flags(PsFlags::Carry);
    if console.cpu.a & console.cpu.x >= byte {
        console.cpu.set_flags(PsFlags::Carry);
    }
    console.cpu.set_x(res);
    todo!()
}

fn sya(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    let hi = addr >> 8;
    let lo = addr & 0xFF;
    let byte = console.cpu.y & hi.wrapping_add(1) as u8;
    Cpu::memory_write(console, (byte as u16) << 8 | lo, byte);
    todo!()
}

fn sxa(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    let hi = addr >> 8;
    let lo = addr & 0xFF;
    let byte = console.cpu.x & hi.wrapping_add(1) as u8;
    Cpu::memory_write(console, (byte as u16) << 8 | lo, byte);
    todo!()
}

fn hlt(console: &mut Console) {
    Cpu::get_operand_value(console);
    todo!()
}

fn unk(console: &mut Console) {
    Cpu::get_operand_value(console);
    todo!()
}

fn axa(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    let byte = ((addr >> 8) + 1) as u8 & console.cpu.a & console.cpu.x;
    Cpu::memory_write(console, addr, byte);
    todo!()
}

fn tas(console: &mut Console) {
    let addr = console.cpu.get_operand_addr();
    console.cpu.set_sp(console.cpu.x & console.cpu.a);
    Cpu::memory_write(console, addr, console.cpu.sp & ((addr >> 8) + 1) as u8);
    todo!()
}

fn las(console: &mut Console) {
    let byte = Cpu::get_operand_value(console);
    console.cpu.set_a(byte & console.cpu.sp);
    console.cpu.set_x(console.cpu.a);
    console.cpu.set_sp(console.cpu.a);
    todo!()
}

const OP_CODES: [&str; 0x100] = [
    "brk", "ora", "hlt", "slo", "nop", "ora", "asl", "slo", "php", "ora", "asl", "aac", "nop",
    "ora", "asl", "slo", // 00
    "bpl", "ora", "hlt", "slo", "nop", "ora", "asl", "slo", "clc", "ora", "nop", "slo", "nop",
    "ora", "asl", "slo", // 10
    "jsr", "and", "hlt", "rla", "bit", "and", "rol", "rla", "plp", "and", "rol", "aac", "bit",
    "and", "rol", "rla", // 20
    "bmi", "and", "hlt", "rla", "nop", "and", "rol", "rla", "sec", "and", "nop", "rla", "nop",
    "and", "rol", "rla", // 30
    "rti", "eor", "hlt", "sre", "nop", "eor", "lsr", "sre", "pha", "eor", "lsr", "asr", "jmp",
    "eor", "lsr", "sre", // 40
    "bvc", "eor", "hlt", "sre", "nop", "eor", "lsr", "sre", "cli", "eor", "nop", "sre", "nop",
    "eor", "lsr", "sre", // 50
    "rts", "adc", "hlt", "rra", "nop", "adc", "ror", "rra", "pla", "adc", "ror", "arr", "jmp",
    "adc", "ror", "rra", // 60
    "bvs", "adc", "hlt", "rra", "nop", "adc", "ror", "rra", "sei", "adc", "nop", "rra", "nop",
    "adc", "ror", "rra", // 70
    "nop", "sta", "nop", "sax", "sty", "sta", "stx", "sax", "dey", "nop", "txa", "unk", "sty",
    "sta", "stx", "sax", // 80
    "bcc", "sta", "hlt", "axa", "sty", "sta", "stx", "sax", "tya", "sta", "txs", "tas", "sya",
    "sta", "sxa", "axa", // 90
    "ldy", "lda", "ldx", "lax", "ldy", "lda", "ldx", "lax", "tay", "lda", "tax", "atx", "ldy",
    "lda", "ldx", "lax", // A0
    "bcs", "lda", "hlt", "lax", "ldy", "lda", "ldx", "lax", "clv", "lda", "tsx", "las", "ldy",
    "lda", "ldx", "lax", // B0
    "cpy", "cpa", "nop", "dcp", "cpy", "cpa", "dec", "dcp", "iny", "cpa", "dex", "axs", "cpy",
    "cpa", "dec", "dcp", // C0
    "bne", "cpa", "hlt", "dcp", "nop", "cpa", "dec", "dcp", "cld", "cpa", "nop", "dcp", "nop",
    "cpa", "dec", "dcp", // D0
    "cpx", "sbc", "nop", "isb", "cpx", "sbc", "inc", "isb", "inx", "sbc", "nop", "sbc", "cpx",
    "sbc", "inc", "isb", // E0
    "beq", "sbc", "hlt", "isb", "nop", "sbc", "inc", "isb", "sed", "sbc", "nop", "isb", "nop",
    "sbc", "inc", "isb", // F0
];

//  +00  +01  +02  +03  +04  +05  +06  +07  +08  +09  +0A  +0B  +0C  +0D  +0E  +0F
const OP_TABLE: [Func; 0x100] = [
    brk, ora, hlt, slo, nop, ora, asl, slo, php, ora, asl, aac, nop, ora, asl, slo, // 00
    bpl, ora, hlt, slo, nop, ora, asl, slo, clc, ora, nop, slo, nop, ora, asl, slo, // 10
    jsr, and, hlt, rla, bit, and, rol, rla, plp, and, rol, aac, bit, and, rol, rla, // 20
    bmi, and, hlt, rla, nop, and, rol, rla, sec, and, nop, rla, nop, and, rol, rla, // 30
    rti, eor, hlt, sre, nop, eor, lsr, sre, pha, eor, lsr, asr, jmp, eor, lsr, sre, // 40
    bvc, eor, hlt, sre, nop, eor, lsr, sre, cli, eor, nop, sre, nop, eor, lsr, sre, // 50
    rts, adc, hlt, rra, nop, adc, ror, rra, pla, adc, ror, arr, jmp, adc, ror, rra, // 60
    bvs, adc, hlt, rra, nop, adc, ror, rra, sei, adc, nop, rra, nop, adc, ror, rra, // 70
    nop, sta, nop, sax, sty, sta, stx, sax, dey, nop, txa, unk, sty, sta, stx, sax, // 80
    bcc, sta, hlt, axa, sty, sta, stx, sax, tya, sta, txs, tas, sya, sta, sxa, axa, // 90
    ldy, lda, ldx, lax, ldy, lda, ldx, lax, tay, lda, tax, atx, ldy, lda, ldx, lax, // A0
    bcs, lda, hlt, lax, ldy, lda, ldx, lax, clv, lda, tsx, las, ldy, lda, ldx, lax, // B0
    cpy, cpa, nop, dcp, cpy, cpa, dec, dcp, iny, cpa, dex, axs, cpy, cpa, dec, dcp, // C0
    bne, cpa, hlt, dcp, nop, cpa, dec, dcp, cld, cpa, nop, dcp, nop, cpa, dec, dcp, // D0
    cpx, sbc, nop, isb, cpx, sbc, inc, isb, inx, sbc, nop, sbc, cpx, sbc, inc, isb, // E0
    beq, sbc, hlt, isb, nop, sbc, inc, isb, sed, sbc, nop, isb, nop, sbc, inc, isb, // F0
];

//  +00  +01  +02  +03  +04  +05  +06  +07  +08  +09  +0A  +0B  +0C  +0D  +0E  +0F
const ADDR_MODE: [AddrMode; 0x100] = [
    Imp, InX, Non, InX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Acc, Imm, Abs, Abs, Abs, Abs, // 00
    Rel, InY, Non, IwY, ZpX, ZpX, ZpX, ZpX, Imp, AbY, Imp, AwY, AbX, AbX, AwX, AwX, // 10
    Abs, InX, Non, InX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Acc, Imm, Abs, Abs, Abs, Abs, // 20
    Rel, InY, Non, IwY, ZpX, ZpX, ZpX, ZpX, Imp, AbY, Imp, AwY, AbX, AbX, AwX, AwX, // 30
    Imp, InX, Non, InX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Acc, Imm, Abs, Abs, Abs, Abs, // 40
    Rel, InY, Non, IwY, ZpX, ZpX, ZpX, ZpX, Imp, AbY, Imp, AwY, AbX, AbX, AwX, AwX, // 50
    Imp, InX, Non, InX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Acc, Imm, Ind, Abs, Abs, Abs, // 60
    Rel, InY, Non, IwY, ZpX, ZpX, ZpX, ZpX, Imp, AbY, Imp, AwY, AbX, AbX, AwX, AwX, // 70
    Imm, InX, Imm, InX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Imp, Imm, Abs, Abs, Abs, Abs, // 80
    Rel, IwY, Non, IwY, ZpX, ZpX, ZpY, ZpY, Imp, AwY, Imp, AwY, AwX, AwX, AwY, AwY, // 90
    Imm, InX, Imm, InX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Imp, Imm, Abs, Abs, Abs, Abs, // A0
    Rel, InY, Non, InY, ZpX, ZpX, ZpY, ZpY, Imp, AbY, Imp, AbY, AbX, AbX, AbY, AbY, // B0
    Imm, InX, Imm, InX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Imp, Imm, Abs, Abs, Abs, Abs, // C0
    Rel, InY, Non, IwY, ZpX, ZpX, ZpX, ZpX, Imp, AbY, Imp, AwY, AbX, AbX, AwX, AwX, // D0
    Imm, InX, Imm, InX, Zpg, Zpg, Zpg, Zpg, Imp, Imm, Imp, Imm, Abs, Abs, Abs, Abs, // E0
    Rel, InY, Non, IwY, ZpX, ZpX, ZpX, ZpX, Imp, AbY, Imp, AwY, AbX, AbX, AwX, AwX, // F0
];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum AddrMode {
    Non,
    Acc,
    Imp,
    Imm,
    Rel,
    Zpg,
    Abs,
    ZpX,
    ZpY,
    Ind,
    AbX,
    AbY,
    InX,
    InY,
    IwY,
    AwX,
    AwY,
}

impl Default for AddrMode {
    fn default() -> Self {
        Non
    }
}

#[derive(Debug)]
enum Operand {
    None,
    Accumulator,
    Value(u8),
    Relative(i8),
    Address(u16),
}

impl Default for Operand {
    fn default() -> Self {
        Operand::Value(0)
    }
}

bitflags! {
    #[derive(Default)]
    pub struct IrqFlags: u8 {
        const None = 0x00;
        const External = 0x01;
        const FrameCounter = 0x02;
        const Dmc = 0x04;
        const FdsDisk = 0x08;
    }
}

bitflags! {
    struct PsFlags: u8{
        const Carry = 0x01;
        const Zero = 0x02;
        const Interrupt = 0x04;
        const Decimal = 0x08;
        const Break = 0x10;
        const Reserved = 0x20;
        const Overflow = 0x40;
        const Negative = 0x80;
    }
}

impl Default for PsFlags {
    fn default() -> Self {
        PsFlags::Reserved
    }
}

#[derive(Debug, Default)]
pub struct Cpu {
    master_clock: u64,
    ppu_offset: u64,
    start_clock_count: u64,
    end_clock_count: u64,
    inst_addr_mode: AddrMode,
    need_halt: bool,
    sprite_dma_transfer: bool,
    dmc_dma_running: bool,
    need_dummy_read: bool,
    sprite_dma_offset: u16,
    state: CpuState,
    prev_run_irq: bool,
    run_irq: bool,
    prev_nmi_flag: bool,
    prev_need_nmi: bool,
    need_nmi: bool,
    last_crash_warning: u64,
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    ps: PsFlags,
    operand: Operand,
    pub pc: u16,
    pub cycle_count: i64,
    pub nmi_flag: bool,
    pub irq_mask: u8,
    pub irq_flags: IrqFlags,
    pub cpu_write: bool,
    pub debug_pc: u16,
    pub previous_debug_pc: u16,
}

impl Cpu {
    fn set_a(&mut self, byte: u8) {
        self.clear_flags(PsFlags::Zero | PsFlags::Negative);
        self.set_zero_negative_flags(byte);
        self.a = byte;
    }

    fn set_x(&mut self, byte: u8) {
        self.clear_flags(PsFlags::Zero | PsFlags::Negative);
        self.set_zero_negative_flags(byte);
        self.x = byte;
    }

    fn set_y(&mut self, byte: u8) {
        self.clear_flags(PsFlags::Zero | PsFlags::Negative);
        self.set_zero_negative_flags(byte);
        self.y = byte;
    }

    fn set_sp(&mut self, byte: u8) {
        self.sp = byte;
    }

    fn set_ps(&mut self, byte: u8) {
        self.ps = PsFlags::from_bits(byte & 0xCF | 0x20).unwrap();
    }

    fn set_pc(&mut self, byte: u16) {
        self.pc = byte;
    }

    fn process_pending_dma(console: &mut Console, read_addr: u16) {
        if !console.cpu.need_halt {
            return;
        }
        Cpu::start_cpu_cycle(console, true);
        MemoryManager::read_byte(console, read_addr);
        Cpu::end_cpu_cycle(console, true);
        console.cpu.need_halt = false;
        let mut sprite_dma_counter = 0;
        let mut sprite_read_addr = 0;
        let mut read_value = 0;
        let skip_dummy_reads = read_addr == 0x4016 || read_addr == 0x4017;
        while console.cpu.dmc_dma_running || console.cpu.sprite_dma_transfer {
            let cpu = &mut console.cpu;
            let get_cycle = cpu.cycle_count & 0x01 == 0;
            if get_cycle {
                let cpu = &mut console.cpu;
                if cpu.dmc_dma_running && !cpu.need_halt && !cpu.need_dummy_read {
                    Cpu::process_cycle(console);
                    // read_value = MemoryManager::read_byte(console.apu, addr: u16)
                    Cpu::end_cpu_cycle(console, true);
                    console.cpu.dmc_dma_running = false;
                    todo!();
                } else if cpu.sprite_dma_transfer {
                    Cpu::process_cycle(console);
                    read_value = MemoryManager::read_byte(
                        console,
                        console.cpu.sprite_dma_offset * 0x100 + sprite_read_addr,
                    );
                    Cpu::end_cpu_cycle(console, true);
                    sprite_read_addr += 1;
                    sprite_dma_counter += 1;
                } else {
                    assert!(cpu.need_halt || cpu.need_dummy_read);
                    Cpu::process_cycle(console);
                    if skip_dummy_reads {
                        MemoryManager::read_byte(console, read_addr);
                    }
                    Cpu::end_cpu_cycle(console, true);
                }
            } else {
                if cpu.sprite_dma_transfer && sprite_dma_counter & 0x01 == 0x01 {
                    Cpu::process_cycle(console);
                    MemoryManager::write_byte(console, 0x2004, read_value);
                    Cpu::end_cpu_cycle(console, true);
                    let cpu = &mut console.cpu;
                    sprite_dma_counter += 1;
                    if sprite_dma_counter == 0x200 {
                        cpu.sprite_dma_transfer = false;
                    }
                } else {
                    Cpu::process_cycle(console);
                    if skip_dummy_reads {
                        MemoryManager::read_byte(console, read_addr);
                    }
                    Cpu::end_cpu_cycle(console, true);
                }
            }
        }
    }

    fn process_cycle(console: &mut Console) {
        if console.cpu.need_halt {
            console.cpu.need_halt = false;
        } else if console.cpu.need_dummy_read {
            console.cpu.need_dummy_read = false;
        }
        Cpu::start_cpu_cycle(console, true);
    }

    fn fetch_operand(console: &mut Console) -> Operand {
        use AddrMode::*;
        match console.cpu.inst_addr_mode {
            Non => {
                panic!()
            }
            Acc => Cpu::get_accumulator(console),
            Imp => Cpu::get_implied(console),
            // Acc => {
            //     Cpu::dummy_read(console);
            //     Operand::Accumulator
            // }
            // Imp => {
            //     Cpu::dummy_read(console);
            //     Operand::None
            // }
            Imm => Cpu::get_immediate(console),
            Rel => Cpu::get_relative(console),
            Zpg => Cpu::get_zero_page_addr(console),
            ZpX => Cpu::get_zero_page_x_addr(console),
            ZpY => Cpu::get_zero_page_y_addr(console),
            Ind => Cpu::get_indirect_addr(console),
            InX => Cpu::get_indexed_indirect_x_addr(console),
            InY => Cpu::get_indirect_indexed_y_addr(console, false),
            IwY => Cpu::get_indirect_indexed_y_addr(console, true),
            Abs => Cpu::get_absolute_addr(console),
            AbX => Cpu::get_absolute_x_addr(console, false),
            AwX => Cpu::get_absolute_x_addr(console, true),
            AbY => Cpu::get_absolute_y_addr(console, false),
            AwY => Cpu::get_absolute_y_addr(console, true),
        }
    }

    fn get_op_code(console: &mut Console) -> u8 {
        let op_code = Cpu::memory_read(console, console.cpu.pc);
        console.cpu.pc += 1;
        op_code
    }

    fn dummy_read(console: &mut Console) {
        Cpu::memory_read(console, console.cpu.pc);
    }

    fn read_byte(console: &mut Console) -> u8 {
        let byte = Cpu::memory_read(console, console.cpu.pc);
        console.cpu.pc += 1;
        byte
    }

    fn read_word(console: &mut Console) -> u16 {
        let word = Cpu::memory_read_word(console, console.cpu.pc);
        console.cpu.pc += 2;
        word
    }

    fn cmp(&mut self, reg: u8, byte: u8) {
        self.clear_flags(PsFlags::Carry | PsFlags::Negative | PsFlags::Zero);
        let result = reg.wrapping_sub(byte);
        if reg >= byte {
            self.set_flags(PsFlags::Carry);
        }
        if reg == byte {
            self.set_flags(PsFlags::Zero);
        }
        if result & 0x80 == 0x80 {
            self.set_flags(PsFlags::Negative);
        }
    }

    fn add(&mut self, byte: u8) {
        let word = self.a as u16 + byte as u16 + (self.ps.bits() as u16 & 0x01);
        let res = word as u8;
        let flags = PsFlags::Carry | PsFlags::Negative | PsFlags::Overflow | PsFlags::Zero;
        self.clear_flags(flags);
        self.set_zero_negative_flags(res);
        if !(self.a ^ byte) & ((self.a ^ res) & 0x80) != 0x00 {
            self.set_flags(PsFlags::Overflow);
        }
        if word > 0xFF {
            self.set_flags(PsFlags::Carry);
        }
        self.set_a(res);
    }

    fn branch_relative(console: &mut Console, branch: bool) {
        let offset = console.cpu.get_operand_relative();
        if branch {
            if console.cpu.run_irq && !console.cpu.prev_run_irq {
                console.cpu.run_irq = false;
            }
            Cpu::dummy_read(console);
            if Cpu::check_page_crossed_i8(console.cpu.pc, offset) {
                Cpu::dummy_read(console);
            }
            let addr = console.cpu.pc.wrapping_add(offset as u16);
            console.cpu.set_pc(addr);
        }
    }

    fn lsr(&mut self, byte: u8) -> u8 {
        self.clear_flags(PsFlags::Carry | PsFlags::Negative | PsFlags::Zero);
        if byte & 0x01 == 0x01 {
            self.set_flags(PsFlags::Carry);
        }
        let res = byte >> 1;
        self.set_zero_negative_flags(res);
        res
    }

    fn asl(&mut self, byte: u8) -> u8 {
        self.clear_flags(PsFlags::Carry | PsFlags::Negative | PsFlags::Zero);
        if byte & 0x80 == 0x80 {
            self.set_flags(PsFlags::Carry);
        }
        let res = byte << 1;
        self.set_zero_negative_flags(res);
        res
    }

    fn ror(&mut self, byte: u8) -> u8 {
        let carry = self.check_flag(PsFlags::Carry);
        self.clear_flags(PsFlags::Carry | PsFlags::Negative | PsFlags::Zero);
        if byte & 0x01 == 0x01 {
            self.set_flags(PsFlags::Carry);
        }
        let res = byte >> 1 | if carry { 0x80 } else { 0x00 };
        self.set_zero_negative_flags(res);
        res
    }

    fn rol(&mut self, byte: u8) -> u8 {
        let carry = self.check_flag(PsFlags::Carry);
        self.clear_flags(PsFlags::Carry | PsFlags::Negative | PsFlags::Zero);
        if byte & 0x80 == 0x80 {
            self.set_flags(PsFlags::Carry);
        }
        let res = byte << 1 | if carry { 0x01 } else { 0x00 };
        self.set_zero_negative_flags(res);
        res
    }

    fn clear_flags(&mut self, flags: PsFlags) {
        self.ps &= !flags;
    }

    fn set_flags(&mut self, flags: PsFlags) {
        self.ps |= flags;
    }

    fn check_flag(&self, flag: PsFlags) -> bool {
        self.ps & flag == flag
    }

    fn set_zero_negative_flags(&mut self, value: u8) {
        if value == 0x00 {
            self.set_flags(PsFlags::Zero);
        } else if value & 0x80 == 0x80 {
            self.set_flags(PsFlags::Negative);
        }
    }

    fn check_page_crossed_u8(val_a: u16, val_b: u8) -> bool {
        val_a.wrapping_add(val_b as u16) & 0xFF00 != val_a & 0xFF00
    }

    fn check_page_crossed_i8(val_a: u16, val_b: i8) -> bool {
        val_a.wrapping_add(val_b as u16) & 0xFF00 != val_a & 0xFF00
    }

    fn memory_write(console: &mut Console, addr: u16, byte: u8) {
        console.cpu.cpu_write = true;
        Cpu::start_cpu_cycle(console, false);
        MemoryManager::write_byte(console, addr, byte);
        Cpu::end_cpu_cycle(console, false);
    }

    fn memory_read(console: &mut Console, addr: u16) -> u8 {
        Cpu::process_pending_dma(console, addr);
        Cpu::start_cpu_cycle(console, true);
        let value = MemoryManager::read_byte(console, addr);
        Cpu::end_cpu_cycle(console, true);
        value
    }

    fn memory_read_word(console: &mut Console, addr: u16) -> u16 {
        let lo = Cpu::memory_read(console, addr) as u16;
        let hi = Cpu::memory_read(console, addr + 1) as u16;
        lo | hi << 8
    }

    fn get_operand_value(console: &mut Console) -> u8 {
        use Operand::*;
        match console.cpu.operand {
            Address(addr) => Cpu::memory_read(console, addr),
            Value(byte) => byte,
            Accumulator => panic!(),
            Relative(_offset) => panic!(),
            None => 0,
        }
    }

    fn get_operand_addr(&mut self) -> u16 {
        if let Operand::Address(addr) = self.operand {
            addr
        } else {
            panic!()
        }
    }

    fn get_operand_relative(&mut self) -> i8 {
        if let Operand::Relative(offset) = self.operand {
            offset
        } else {
            panic!()
        }
    }

    fn get_indirect_addr(console: &mut Console) -> Operand {
        let addr = Cpu::read_word(console);
        if addr & 0xFF == 0xFF {
            let lo = Cpu::memory_read(console, addr) as u16;
            let hi = Cpu::memory_read(console, addr - 0xFF) as u16;
            Operand::Address(lo | hi << 8)
        } else {
            let word = Cpu::memory_read_word(console, addr);
            Operand::Address(word)
        }
    }

    fn get_accumulator(console: &mut Console) -> Operand {
        Cpu::dummy_read(console);
        Operand::Accumulator
    }

    fn get_implied(console: &mut Console) -> Operand {
        Cpu::dummy_read(console);
        Operand::None
    }

    fn get_immediate(console: &mut Console) -> Operand {
        let byte = Cpu::read_byte(console);
        Operand::Value(byte)
    }

    fn get_relative(console: &mut Console) -> Operand {
        let byte = Cpu::read_byte(console);
        Operand::Relative(byte as i8)
    }

    fn get_zero_page_addr(console: &mut Console) -> Operand {
        let byte = Cpu::read_byte(console);
        Operand::Address(byte as u16)
    }

    fn get_zero_page_x_addr(console: &mut Console) -> Operand {
        let byte = Cpu::read_byte(console);
        Cpu::memory_read(console, byte as u16);
        let byte = byte.wrapping_add(console.cpu.x);
        Operand::Address(byte as u16)
    }

    fn get_zero_page_y_addr(console: &mut Console) -> Operand {
        let byte = Cpu::read_byte(console);
        Cpu::memory_read(console, byte as u16);
        let byte = byte.wrapping_add(console.cpu.y);
        Operand::Address(byte as u16)
    }

    fn get_absolute_addr(console: &mut Console) -> Operand {
        let word = Cpu::read_word(console);
        Operand::Address(word)
    }

    fn get_absolute_x_addr(console: &mut Console, dummy_read: bool) -> Operand {
        let base_addr = Cpu::read_word(console);
        let page_crossed = Cpu::check_page_crossed_u8(base_addr, console.cpu.x);
        let addr = base_addr.wrapping_add(console.cpu.x as u16);
        if page_crossed || dummy_read {
            let addr = addr.wrapping_sub(if page_crossed { 0x100 } else { 0x00 });
            Cpu::memory_read(console, addr);
        }
        Operand::Address(addr)
    }

    fn get_absolute_y_addr(console: &mut Console, dummy_read: bool) -> Operand {
        let base_addr = Cpu::read_word(console);
        let page_crossed = Cpu::check_page_crossed_u8(base_addr, console.cpu.y);
        let addr = base_addr.wrapping_add(console.cpu.y as u16);
        if page_crossed || dummy_read {
            let addr = addr.wrapping_sub(if page_crossed { 0x100 } else { 0x00 });
            Cpu::memory_read(console, addr);
        }
        Operand::Address(addr)
    }

    fn get_indexed_indirect_x_addr(console: &mut Console) -> Operand {
        let mut zero = Cpu::read_byte(console);
        Cpu::memory_read(console, zero as u16);
        zero = zero.wrapping_add(console.cpu.x);
        let addr = if zero == 0xFF {
            let lo = Cpu::memory_read(console, 0x00FF) as u16;
            let hi = Cpu::memory_read(console, 0x0000) as u16;
            lo | hi << 8
        } else {
            let word = Cpu::memory_read_word(console, zero as u16);
            word
        };
        Operand::Address(addr)
    }

    fn get_indirect_indexed_y_addr(console: &mut Console, dummy_read: bool) -> Operand {
        let zero = Cpu::read_byte(console);
        let addr = if zero == 0xFF {
            let lo = Cpu::memory_read(console, 0x00FF) as u16;
            let hi = Cpu::memory_read(console, 0x0000) as u16;
            lo | hi << 8
        } else {
            let word = Cpu::memory_read_word(console, zero as u16);
            word
        };

        let page_crossed = Cpu::check_page_crossed_u8(addr, console.cpu.y);
        if page_crossed || dummy_read {
            let addr = addr.wrapping_add(console.cpu.y as u16);
            let addr = addr.wrapping_sub(if page_crossed { 0x100 } else { 0x00 });
            Cpu::memory_read(console, addr);
        }
        let addr = addr.wrapping_add(console.cpu.y as u16);
        Operand::Address(addr)
    }

    pub fn run_dma_transfer(&mut self, offset: u8) {
        todo!()
    }

    pub fn start_dmc_transfer(&mut self) {
        todo!()
    }

    pub fn set_debug_pc(value: u16) {
        todo!()
    }

    pub fn reset(console: &mut Console, soft_reset: bool) {
        console.cpu.nmi_flag = false;
        console.cpu.irq_flags = IrqFlags::None;
        console.cpu.sprite_dma_transfer = false;
        console.cpu.sprite_dma_offset = 0;
        console.cpu.need_halt = false;
        console.cpu.dmc_dma_running = false;
        console.cpu.last_crash_warning = 0;
        console.cpu.irq_mask = 0xFF;
        console.cpu.pc = MemoryManager::read_word(console, RESET_VECTOR);
        console.cpu.debug_pc = console.cpu.pc;
        if soft_reset {
            console.cpu.ps |= PsFlags::Interrupt;
            console.cpu.sp -= 0x03;
        } else {
            console.cpu.a = 0;
            console.cpu.sp = 0xfd;
            console.cpu.x = 0;
            console.cpu.y = 0;
            console.cpu.ps = PsFlags::Reserved | PsFlags::Interrupt;
            console.cpu.run_irq = false;
        }

        console.cpu.cycle_count = -1;
        console.cpu.master_clock = 0;
        // why 5?
        console.cpu.ppu_offset = 5;
        for _ in 0..8 {
            Cpu::start_cpu_cycle(console, true);
            Cpu::end_cpu_cycle(console, true);
        }
    }

    pub fn exec(console: &mut Console) {
        let op_code = Cpu::get_op_code(console) as usize;
        console.cpu.inst_addr_mode = ADDR_MODE[op_code];
        console.cpu.operand = Cpu::fetch_operand(console);
        OP_TABLE[op_code](console);
        if console.cpu.prev_run_irq || console.cpu.prev_need_nmi {
            Cpu::irq(console);
        }
    }

    pub fn irq(console: &mut Console) {
        todo!()
    }

    pub fn get_state(&mut self) -> CpuState {
        todo!()
    }

    pub fn push_byte(console: &mut Console, byte: u8) {
        let addr = 0x100 + console.cpu.sp as u16;
        Cpu::memory_write(console, addr, byte);
        console.cpu.set_sp(console.cpu.sp - 1);
    }

    pub fn push_word(console: &mut Console, word: u16) {
        let bytes = word.to_le_bytes();
        Cpu::push_byte(console, bytes[1]);
        Cpu::push_byte(console, bytes[0]);
    }

    pub fn pop_byte(console: &mut Console) -> u8 {
        console.cpu.set_sp(console.cpu.sp + 1);
        let addr = 0x100 + console.cpu.sp as u16;
        Cpu::memory_read(console, addr)
    }

    pub fn pop_word(console: &mut Console) -> u16 {
        let bytes = [Cpu::pop_byte(console), Cpu::pop_byte(console)];
        u16::from_le_bytes(bytes)
    }

    pub fn start_cpu_cycle(console: &mut Console, for_read: bool) {
        console.cpu.master_clock += if for_read {
            console.cpu.start_clock_count - 1
        } else {
            console.cpu.start_clock_count + 1
        };
        console.cpu.cycle_count += 1;
        Ppu::run(console, console.cpu.master_clock - console.cpu.ppu_offset);
        console.process_cpu_clock();
    }

    pub fn end_cpu_cycle(console: &mut Console, for_read: bool) {
        console.cpu.master_clock += if for_read {
            console.cpu.end_clock_count + 1
        } else {
            console.cpu.end_clock_count - 1
        };
        Ppu::run(console, console.cpu.master_clock - console.cpu.ppu_offset);
        console.cpu.prev_need_nmi = console.cpu.need_nmi;
        if console.cpu.prev_nmi_flag && console.cpu.nmi_flag {
            console.cpu.need_nmi = true;
        }
        console.cpu.prev_nmi_flag = console.cpu.nmi_flag;
        console.cpu.prev_run_irq = console.cpu.run_irq;
    }
    pub fn new() -> Self {
        let mut this = Cpu::default();
        this.start_clock_count = 6;
        this.end_clock_count = 6;
        this.nmi_flag = false;
        this.irq_flags = IrqFlags::None;
        this.sprite_dma_transfer = false;
        this.sprite_dma_offset = 0;
        this.need_halt = false;
        this.dmc_dma_running = false;
        this.last_crash_warning = 0;
        this.irq_mask = 0xFF;
        this.debug_pc = this.pc;
        this.a = 0;
        this.sp = 0xfd;
        this.x = 0;
        this.y = 0;
        this.ps = PsFlags::Reserved | PsFlags::Interrupt;
        this.run_irq = false;
        this.cycle_count = -1;
        this.master_clock = 0;
        let cpu_offset = 0;
        this.ppu_offset = 1;
        this.master_clock = CPU_DIVIDER + cpu_offset;
        this
    }
}

#[test]
fn test() {
    logger_init();
    let rom = VirtualFile::new("Nes Test", NES_TEST);
    let mut console = Console::new(&rom, Box::new(BaseRenderer::default()));
    let testlog = include_str!("../test/nestest.log");
    debug!("{:02x?}", console.mapper.base_mapper().prg_rom);
    console.cpu.pc = 0xC000;
    for line in testlog.lines() {
        let pc = &line[0..4];
        let a = &line[48..48 + 4];
        let x = &line[53..53 + 4];
        let y = &line[58..58 + 4];
        let p = &line[63..63 + 4];
        let s = &line[68..68 + 5];
        let ppu_xy = &line[74..74 + 11];
        let ppu_x = &ppu_xy[4..7];
        let ppu_y = &ppu_xy[8..11];
        let cyc = &line[86..];
        let ppu = &mut console.ppu;
        let cpu = &mut console.cpu;
        assert_eq!(pc, format!("{:04X}", cpu.pc));
        assert_eq!(a, format!("A:{:02X}", cpu.a));
        assert_eq!(x, format!("X:{:02X}", cpu.x));
        assert_eq!(y, format!("Y:{:02X}", cpu.y));
        assert_eq!(p, format!("P:{:02X}", cpu.ps));
        assert_eq!(s, format!("SP:{:02X}", cpu.sp));
        assert_eq!(ppu_x, format!("{:3}", ppu.scanline));
        assert_eq!(ppu_y, format!("{:3}", ppu.cycle));
        assert_eq!(cyc, format!("CYC:{}", cpu.cycle_count));
        Cpu::exec(&mut console);
    }
}
