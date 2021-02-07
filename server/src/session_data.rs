use actix_session::Session;
use actix_web::Error;
use rand::thread_rng;
use rand::Rng;
use rustgym_characters::CHARACTERS;
use uuid::Uuid;

#[derive(Debug)]
pub struct SessionData {
    pub count: i32,
    pub name: String,
    pub uuid: Uuid,
}

pub fn update_session(session: Session) -> Result<SessionData, Error> {
    let count = if let Some(count) = session.get::<i32>("counter")? {
        session.set("counter", count + 1)?;
        count + 1
    } else {
        session.set("counter", 1)?;
        1
    };

    let name = if let Some(name) = session.get::<String>("name")? {
        name
    } else {
        let n = CHARACTERS.len();
        let k = thread_rng().gen::<usize>();
        let name = CHARACTERS[k % n];
        session.set("name", name.to_string())?;
        name.to_string()
    };

    let uuid = if let Some(uuid) = session.get::<Uuid>("uuid")? {
        uuid
    } else {
        Uuid::new_v4()
    };

    let session_data = SessionData { count, name, uuid };
    Ok(session_data)
}
