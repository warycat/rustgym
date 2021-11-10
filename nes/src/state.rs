pub trait Saver {
    fn begin(&mut self, chunk: u32) -> &mut Self;
    fn write8(&mut self, data: u8) -> &mut Self;
    fn write16(&mut self, data: u16) -> &mut Self;
    fn write32(&mut self, data: u32) -> &mut Self;
    fn write64(&mut self, data: u64) -> &mut Self;
    fn write(&mut self, data: *const u8, length: usize) -> &mut Self;
    fn compress(&mut self, data: *const u8, length: usize) -> &mut Self;
    fn end(&mut self) -> &mut Self;
}

pub trait Loader {
    fn begin(&mut self) -> u32;
    fn check(&mut self) -> u32;
    fn length(&mut self) -> u32;
    fn read8(&mut self) -> u8;
    fn read16(&mut self) -> u16;
    fn read32(&mut self) -> u32;
    fn read64(&mut self) -> u64;
    fn read(&mut self, data: &mut u8, length: usize);
    fn uncompress(&mut self, data: &mut u8, length: usize);
    fn end(&mut self);
}

pub trait SaveLoad {
    fn save_state(&self, saver: &mut impl Saver, base_chunk: u32);
    fn load_state(&self, loader: &mut impl Loader);
}
