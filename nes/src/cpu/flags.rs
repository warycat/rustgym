pub const C: u8 = 0x01; // carry
pub const Z: u8 = 0x02; // zero
pub const I: u8 = 0x04; // interrupt
pub const D: u8 = 0x08; // decimal
pub const B: u8 = 0x10; // brk
pub const R: u8 = 0x20; // unsused
pub const V: u8 = 0x40; // overflow
pub const N: u8 = 0x80; // negative

pub struct Flags {
    pub c: u8,
    pub z: u8,
    pub i: u8,
    pub d: u8,
    b: u8,
    r: u8,
    pub v: u8,
    pub n: u8,
}

pub fn n(val: u8) -> u8 {
    val & N
}

pub fn z(val: u8) -> u8 {
    if val == 0 {
        Z
    } else {
        0
    }
}

pub fn v(a: u8, val: u8, res: u8) -> u8 {
    if (res ^ val) & (a ^ res) & 0x80 != 0 {
        V
    } else {
        0
    }
}

#[test]
fn overflow() {
    assert_eq!(v(80, 16, 96), 0);
    assert_eq!(v(80, 80, 160), V);
    assert_eq!(v(80, 144, 224), 0);
    assert_eq!(v(80, 208, 32), 0);
    assert_eq!(v(208, 16, 224), 0);
    assert_eq!(v(208, 80, 32), 0);
    assert_eq!(v(208, 144, 96), V);
    assert_eq!(v(208, 208, 160), 0);
}
