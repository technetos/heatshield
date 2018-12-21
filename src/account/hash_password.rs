use crate::{account::Account, salt::SaltController};

use data_encoding::HEXUPPER;
use ring::{digest, pbkdf2};

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
type Credential = [u8; CREDENTIAL_LEN];

impl Account {
    pub fn hash_password(&mut self) {
        let salt = SaltController
            .get_salt(&self.email.as_ref().unwrap().clone())
            .unwrap();
        let mut hash_result: Credential = [0u8; CREDENTIAL_LEN];

        pbkdf2::derive(
            DIGEST_ALG,
            100_000,
            &salt,
            self.password.as_ref().unwrap().clone().as_bytes(),
            &mut hash_result,
        );

        self.password = Some(HEXUPPER.encode(&hash_result));
    }
}
