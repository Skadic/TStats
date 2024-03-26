use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use rand::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};


use self::crypt::EncryptedToken;

pub mod consts;
pub mod crypt;
pub mod cache;
mod log_status;
pub use cache::Cacheable;
pub use log_status::LogStatus;

#[derive(Serialize, Deserialize)]
pub struct Session {
    session_id: String,
    osu_user_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct OsuApiTokens {
    user_id: usize,
    access_token: EncryptedToken,
    refresh_token: EncryptedToken,
}

impl Cacheable for Session {
    type KeyType = str;

    fn type_key() -> &'static str {
        "session"
    }

    fn key(&self) -> &Self::KeyType {
        self.session_id.as_str()
    }
}

impl Session {
    pub fn generate_session_id() -> String {
        let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
        let mut buf = [0u8; 16];
        rng.fill_bytes(&mut buf);
        BASE64_STANDARD.encode(buf)
    }
}

