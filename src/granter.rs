use rocket_contrib::{Json, Value, UUID};

// password: there are credentials in the body of the request
pub enum Granter {
    Password,
}

impl Granter {
    pub fn from_str(string: String) -> Result<Self, Json> {
        match &string[..] {
            "password" => Ok(Granter::Password),
            _ => Err(Json(json!("invalid grant_type"))),
        }
    }
}
