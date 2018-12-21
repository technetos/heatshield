use chrono::{FixedOffset, Utc};
use jsonwebtoken;

#[derive(Serialize, Deserialize, Debug)]
pub struct JWT {
    exp: i64,
}

impl JWT {
    pub fn new() -> Self {
        // 3600 = 60*60 = 1 hour in seconds
        Self {
            exp: (Utc::now() + FixedOffset::east(3600)).timestamp(),
        }
    }
}
