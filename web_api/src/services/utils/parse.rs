use uuid::Uuid;

// parse_uuid! {

pub fn parse_uuid(uuid: String) -> Result<Uuid, String> {
    match uuid.len() {
        36 => {
            match Uuid::parse_str(uuid.as_str()) {
                Ok(uuid) => Ok(uuid),
                Err(_) => Err("Invalid uuid".to_owned()),
            }
        }
        32 => {
            match Uuid::parse_str(uuid.as_str()) {
                Ok(uuid) => Ok(uuid),
                Err(_) => Err("Invalid uuid".to_owned()),
            }
        }
        _ => Err("Invalid uuid".to_owned()),
    }
}