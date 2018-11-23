use crate::{account::Account, salt::SaltController};

use data_encoding::HEXUPPER;
use ring::{digest, pbkdf2};

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
type Credential = [u8; CREDENTIAL_LEN];

impl Account {
    pub fn verify_password(&self, current_pw: &str) -> bool {
        let pw_salt = SaltController
            .get_salt(self.email.as_ref().unwrap())
            .unwrap();
        let mut actual: Credential = [0u8; CREDENTIAL_LEN];

        pbkdf2::derive(
            DIGEST_ALG,
            100_000,
            &pw_salt,
            current_pw.to_owned().as_bytes(),
            &mut actual,
        );
        let actual_hash = HEXUPPER.encode(&actual);

        Some(&actual_hash) == self.password.as_ref()
    }
}
