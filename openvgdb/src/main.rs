use diesel::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use openvgdb::get_nes_roms;
use rustgym_consts::*;
use rustgym_schema::*;

fn main() {
    let conn = SqliteConnection::establish(DATABASE_URL).unwrap();
    use schema::nes_rom::dsl::*;
    let roms = get_nes_roms();
    let n = insert_into(nes_rom).values(&roms).execute(&conn).unwrap();
    assert_eq!(n, roms.len());
}
