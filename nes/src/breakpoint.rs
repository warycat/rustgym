#[derive(Default)]
pub struct Breakpoint {}

impl Breakpoint {}

pub enum BreakpointType {
    Global,
    Execute,
    ReadRam,
    WriteRam,
    ReadVram,
    WriteVram,
    DummyReadRam,
    DummyWriteRam,
}

impl BreakpointType {
    pub const COUNT: usize = 8;
}
