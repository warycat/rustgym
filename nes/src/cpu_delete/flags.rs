use std::fmt;

// carry
pub const C: u8 = 0x01;

// zero
pub const Z: u8 = 0x02;

// interrupt
pub const I: u8 = 0x04;

// decimal
pub const D: u8 = 0x08;

// brk
pub const B: u8 = 0x10;

// reserved
pub const R: u8 = 0x20;

// overflow
pub const V: u8 = 0x40;

// negative
pub const N: u8 = 0x80;

#[derive(Default)]
pub struct Flags {
    pub c: u8,
    pub z: u8,
    pub i: u8,
    pub d: u8,
    pub b: u8,
    pub r: u8,
    pub v: u8,
    pub n: u8,
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        s.push(if self.n == 0 { '_' } else { 'N' });
        s.push(if self.v == 0 { '_' } else { 'V' });
        s.push('-');
        s.push(if self.b == 0 { '_' } else { 'B' });
        s.push(if self.d == 0 { '_' } else { 'D' });
        s.push(if self.i == 0 { '_' } else { 'I' });
        s.push(if self.z == 0 { '_' } else { 'Z' });
        s.push(if self.c == 0 { '_' } else { 'C' });
        write!(f, "{}", s)
    }
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            c: 0,
            z: 0,
            i: I,
            d: 0,
            b: 0,
            r: R,
            v: 0,
            n: 0,
        }
    }
    pub fn reset(&mut self) {
        self.c = 0;
        self.z = 0;
        self.i = 0;
        self.d = 0;
        self.b = 0;
        self.r = R;
        self.v = 0;
        self.n = 0;
    }
    pub fn set_zn(&mut self, byte: u8) -> u8 {
        self.z = z(byte);
        self.n = n(byte);
        byte
    }
    pub fn unpack(&mut self, byte: u8) {
        self.c = byte & C;
        self.z = byte & Z;
        self.i = byte & I;
        self.d = byte & D;
        self.b = 0;
        self.r = R;
        self.v = byte & V;
        self.n = byte & N;
    }
    pub fn pack(&self) -> u8 {
        self.c | self.z | self.i | self.d | self.b | self.r | self.v | self.n
    }
}

fn n(val: u8) -> u8 {
    val & N
}

fn z(val: u8) -> u8 {
    if val == 0 {
        Z
    } else {
        0
    }
}

pub fn overflow_adc(a: u8, val: u8, res: u8) -> u8 {
    if (res ^ val) & (a ^ res) & 0x80 != 0 {
        V
    } else {
        0
    }
}

pub fn overflow_arr(byte: u8) -> u8 {
    if (byte >> 6 ^ byte >> 5) & 0x01 != 0 {
        V
    } else {
        0
    }
}

pub fn carry16(word: u16, bit: u16) -> u8 {
    if word & bit != 0 {
        C
    } else {
        0
    }
}

pub fn carry8(byte: u8, bit: u8) -> u8 {
    if byte & bit != 0 {
        C
    } else {
        0
    }
}

#[test]
fn test_overflow() {
    assert_eq!(overflow_adc(80, 16, 96), 0);
    assert_eq!(overflow_adc(80, 80, 160), V);
    assert_eq!(overflow_adc(80, 144, 224), 0);
    assert_eq!(overflow_adc(80, 208, 32), 0);
    assert_eq!(overflow_adc(208, 16, 224), 0);
    assert_eq!(overflow_adc(208, 80, 32), 0);
    assert_eq!(overflow_adc(208, 144, 96), V);
    assert_eq!(overflow_adc(208, 208, 160), 0);
}
