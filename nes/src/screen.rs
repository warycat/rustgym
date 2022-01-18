pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;
pub const PIXELS: usize = WIDTH * HEIGHT;
pub const PALETTE: usize = 64 * 8;

struct Screen {
    pixels: [u16; PIXELS],
    palette: [u32; PALETTE],
}
