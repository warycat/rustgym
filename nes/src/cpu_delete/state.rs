#[derive(Debug, Default)]
pub struct State {
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    ps: u8,
    irq_flag: u32,
    cycle_count: u64,
    nmi_flag: bool,
    debug_pc: u16,
    prev_debug_pc: u16,
}

impl State {
    pub fn new() -> Self {
        State {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            ps: 0,
            irq_flag: 0,
            cycle_count: 0,
            nmi_flag: false,
            debug_pc: 0,
            prev_debug_pc: 0,
        }
    }
}
