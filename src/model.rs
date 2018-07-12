use chrono::NaiveDateTime;

struct AccessToken {
  id: u64,
  client: Client,
  enabled: bool,
}

/// # Client
///
/// ### name
/// The name of the client
///
/// ### email
/// The contact email for the client
///
/// ### enabled
/// The status of the client
struct Client {
  id: u64,
  kind: String,
  name: String,
  email: String,
  enabled: bool,
}

struct Account {
  id: u64,
  username: String,
  password: String,
  email: String,
  enabled: bool,
  verification: Verification,
}

struct Verification {
  id: u64,
  required: bool,
  date: NaiveDateTime,
  ip_address: String,
}
