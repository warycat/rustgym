#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use md5;
use rustgym_consts::*;
use std::fs::*;
#[allow(non_snake_case)]
mod rom;
#[allow(non_snake_case)]
mod schema;
use rom::RomData;
use rustgym_schema::*;
use std::path::PathBuf;

pub fn rom_md5(header_size: usize, data: &[u8]) -> String {
    let digest = md5::compute(&data[header_size..]);
    format!("{:X}", digest)
}

pub fn get_nes_roms() -> Vec<NesRom> {
    let mut res = vec![];
    let dir = read_dir("./data/nes").unwrap();
    let conn = SqliteConnection::establish(OPENVGDB_URL).unwrap();
    for path in dir {
        let path: PathBuf = path.unwrap().path();
        let rom_data = read(&path).unwrap();
        let hash = rom_md5(16, &rom_data);
        use schema::ROMs::dsl::*;
        use schema::RELEASES::dsl::*;
        let rom_data: RomData = ROMs
            .inner_join(schema::RELEASES::dsl::RELEASES)
            .filter(romHashMD5.eq(hash))
            .select((
                schema::ROMs::dsl::romID,
                releaseTitleName,
                releaseDescription,
                romHashMD5,
                romFileName,
                releaseCoverFront,
                romSize,
            ))
            .first(&conn)
            .unwrap();

        let id: i32 = rom_data.romID.unwrap();
        let title: String = rom_data.releaseTitleName.unwrap();
        let description: String = rom_data.releaseDescription.unwrap();
        let filename: String = path.file_name().unwrap().to_str().unwrap().into();
        let image: String = rom_data.releaseCoverFront.unwrap();
        let size: i32 = rom_data.romSize.unwrap();
        let md5: String = rom_data.romHashMD5.unwrap();

        let nes_rom = NesRom::new(id, title, description, filename, image, size, md5);
        res.push(nes_rom)
    }
    res
}
