use crate::{
    controller::ResourceController,
    salt::{controller::SaltController, model::Salt},
};

use data_encoding;
use ring::rand::{SecureRandom, SystemRandom};
use ring::{digest, pbkdf2};

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
type Credential = [u8; CREDENTIAL_LEN];

impl SaltController {
    pub fn create_salt(&self) {
        let mut v = [0u8; CREDENTIAL_LEN];
        let _ = SystemRandom.fill(&mut v);
        self.create(&Salt {
            salt: data_encoding::HEXUPPER.encode(&v[..]),
        });
    }
}
