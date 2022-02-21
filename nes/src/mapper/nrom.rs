use crate::*;
use log::info;

#[derive(Default)]
pub struct Nrom {
    id: u8,
    base_mapper: BaseMapper,
}

impl Nrom {
    pub fn new(rom_data: &RomData) -> Self {
        let mut this = Nrom::default();
        this.base_mapper = BaseMapper::new(rom_data);
        this.select_prg_page(0, 0, PrgMemoryType::PrgRom);
        this.select_prg_page(1, 1, PrgMemoryType::PrgRom);
        this.select_chr_page(0, 0, ChrMemoryType::ChrRom);
        info!(
            "prg_page 0x{:x}x{}",
            this.prg_page_size(),
            this.prg_page_count()
        );
        info!(
            "chr_page 0x{:x}x{}",
            this.chr_page_size(),
            this.chr_page_count()
        );
        this
    }
}

impl Mapper for Nrom {
    fn id(&self) -> u8 {
        self.id
    }
    fn base_mapper(&mut self) -> &mut BaseMapper {
        &mut self.base_mapper
    }
    fn prg_page_size(&mut self) -> u16 {
        0x4000
    }
    fn chr_page_size(&mut self) -> u16 {
        0x2000
    }
}
