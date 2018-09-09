use rocket_contrib::{Json, Value, UUID};

pub enum Granter {
    Password,
    RefreshToken,
}

impl Granter {
    pub fn from_str(string: String) -> Result<Self, Json> {
        match &string[..] {
            "password" => Ok(Granter::Password),
            "refresh_token" => Ok(Granter::RefreshToken),
            _ => Err(Json(json!("invalid grant_type"))),
        }
    }
}
