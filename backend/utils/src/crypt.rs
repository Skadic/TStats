use std::sync::OnceLock;

use aes_gcm::{
    aead::Aead,
    aes::{cipher::ArrayLength, Aes256},
    AeadCore, Aes256Gcm, KeyInit, KeySizeUser, Nonce,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use miette::{Context, IntoDiagnostic};
use rand::rngs::OsRng;
use scrypt::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Params, Scrypt,
};
use serde::{Deserialize, Serialize};

use crate::consts::AES_256_KEY;

static SCRYPT_PARAMS: OnceLock<Params> = OnceLock::new();

// There is probably an easier way to do this. This is the size the byte array containing
// the nonce is supposed to be.
const NONCE_SIZE: usize =
    std::mem::size_of::<<<Aes256Gcm as AeadCore>::NonceSize as ArrayLength<u8>>::ArrayType>();
const KEY_SIZE: usize =
    std::mem::size_of::<<<Aes256 as KeySizeUser>::KeySize as ArrayLength<u8>>::ArrayType>();

#[derive(Debug, thiserror::Error)]
pub enum EnvError {
    #[error("{0} is not set")]
    DoesNotExist(&'static str),
    #[error("error decoding Base64")]
    Base64(#[from] base64::DecodeError),
    #[error("invalid aes key length: length is {0} bytes but must be {KEY_SIZE}")]
    InvalidAesKeyLength(usize),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct HashedToken {
    token: String,
}

impl HashedToken {
    pub fn new(s: impl AsRef<[u8]>) -> scrypt::password_hash::Result<HashedToken> {
        let salt = SaltString::generate(OsRng);
        let params = SCRYPT_PARAMS.get_or_init(|| {
            Params::new(
                14,
                Params::RECOMMENDED_R,
                Params::RECOMMENDED_P,
                Params::RECOMMENDED_LEN,
            )
            .unwrap()
        });
        let token = Scrypt
            .hash_password_customized(s.as_ref(), None, None, *params, &salt)?
            .to_string();
        Ok(HashedToken { token })
    }

    /// Verifies the hashed token. This returns Ok(()) if verification is successful.
    ///
    /// # Arguments
    ///
    /// * `token` - The plaintext token.
    pub fn verify(&self, token: impl AsRef<[u8]>) -> scrypt::password_hash::Result<()> {
        PasswordHash::new(&self.token)
            .and_then(|parsed_hash| Scrypt.verify_password(token.as_ref(), &parsed_hash))
    }
}

pub struct EncryptedToken {
    nonce: Nonce<<Aes256Gcm as AeadCore>::NonceSize>,
    token: Vec<u8>,
}

impl EncryptedToken {
    pub fn new(s: impl AsRef<[u8]>) -> miette::Result<Self> {
        let cipher = get_aes_cipher()?;
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let token = cipher
            .encrypt(&nonce, s.as_ref())
            .into_diagnostic()
            .wrap_err("could not encrypt token")?;
        Ok(EncryptedToken { nonce, token })
    }

    pub fn decrypt(&self) -> miette::Result<Vec<u8>> {
        let cipher = get_aes_cipher()?;
        cipher
            .decrypt(&self.nonce, self.token.as_slice())
            .into_diagnostic()
            .wrap_err("could not decrypt token")
    }

    pub fn decrypt_string(&self) -> miette::Result<String> {
        let cipher = get_aes_cipher()?;
        let bytes = cipher
            .decrypt(&self.nonce, self.token.as_slice())
            .into_diagnostic()
            .wrap_err("could not decrypt token")?;
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }
}

impl Serialize for EncryptedToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct RawEncryptedToken {
            nonce: String,
            token: String,
        }
        RawEncryptedToken {
            nonce: BASE64_STANDARD.encode(self.nonce.as_slice()),
            token: BASE64_STANDARD.encode(self.token.as_slice()),
        }
        .serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for EncryptedToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        #[derive(Deserialize)]
        struct RawEncryptedToken {
            nonce: String,
            token: String,
        }
        let RawEncryptedToken { nonce, token } = RawEncryptedToken::deserialize(deserializer)?;
        let decoded_nonce = BASE64_STANDARD
            .decode(nonce)
            .map_err(serde::de::Error::custom)?;

        // In this case the nonce has an invalid size and deserialization fails
        if decoded_nonce.len() != NONCE_SIZE {
            return Err(miette::miette!(
                "nonce of invalid size: size is {} but expected {NONCE_SIZE}",
                decoded_nonce.len()
            ))
            .map_err(serde::de::Error::custom);
        }

        Ok(Self {
            nonce: Nonce::clone_from_slice(&decoded_nonce),
            token: BASE64_STANDARD
                .decode(token)
                .map_err(serde::de::Error::custom)?,
        })
    }
}

/// Verifies that the AES_256_KEY environment variable exists and is a valid AES-256 encryption
/// key.
#[tracing::instrument]
pub fn verify_aes_key() -> Result<(), EnvError> {
    let aes_key_base64 =
        std::env::var(AES_256_KEY).map_err(|_| EnvError::DoesNotExist(AES_256_KEY))?;
    println!("{}", &aes_key_base64[43..]);
    let aes_key = BASE64_STANDARD.decode(&aes_key_base64)?;

    if aes_key.len() != 32 {
        return Err(EnvError::InvalidAesKeyLength(
            aes_key_base64.as_bytes().len(),
        ));
    }
    Ok(())
}

fn get_aes_cipher() -> miette::Result<Aes256Gcm> {
    let aes_key_base64 = std::env::var(AES_256_KEY)
        .into_diagnostic()
        .wrap_err("AES_256_KEY not set. did you call verify_aes_key?")?;
    let aes_key = BASE64_STANDARD
        .decode(aes_key_base64)
        .into_diagnostic()
        .wrap_err("AES_256_KEY not base64 decodeable. did you call verify_aes_key?")?;
    let buf: [u8; 32] = aes_key.try_into().map_err(|v: Vec<u8>| {
        miette::miette!(
            "AES_256_KEY of incorrect length ({}, expected {KEY_SIZE}). did you call verify_aes_key?",
            v.len()
        )
    })?;
    Ok(Aes256Gcm::new(&buf.into()))
}

#[cfg(test)]
mod test {
    use aes_gcm::{Aes256Gcm, KeyInit};
    use rand::rngs::OsRng;

    use crate::consts::AES_256_KEY;

    use super::{verify_aes_key, EncryptedToken, HashedToken};
    use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};

    #[ctor::ctor]
    fn init_env() {
        let key_bytes = Aes256Gcm::generate_key(OsRng);
        std::env::set_var(AES_256_KEY, BASE64_STANDARD.encode(key_bytes.as_slice()));
        verify_aes_key().expect("invalid AES key");
        println!("{}", std::env::var(AES_256_KEY).unwrap())
    }

    #[test]
    fn hash_verify_test() {
        let hashed = HashedToken::new("hello world").expect("could not hash string");

        assert_eq!(
            Ok(()),
            hashed.verify("hello world"),
            "hashed string verification failed"
        );
        assert_eq!(
            Err(scrypt::password_hash::Error::Password),
            hashed.verify("hello"),
            "hashed string should have failed"
        )
    }

    #[test]
    fn hash_serialize_test() {
        let hashed = HashedToken::new("hello world").expect("could not hash string");
        let s = serde_json::to_string(&hashed).expect("could not serialize hashed token");
        println!("{s}");

        let deserialized: HashedToken =
            serde_json::from_str(&s).expect("could not deserialize hashed token");
        assert_eq!(
            hashed, deserialized,
            "hashed tokens not equal after deserialization"
        );
    }

    #[test]
    fn encrypt_decrypt_test() {
        let value = "hello world";
        let encrypted = {
            let e = EncryptedToken::new(value);
            assert!(e.is_ok(), "could not encrypt string");
            e.unwrap()
        };

        let e_bytes = encrypted.decrypt();
        assert!(e_bytes.is_ok(), "could not decrypt ciphertext");
        assert_eq!(
            value.as_bytes(),
            e_bytes.unwrap(),
            "decrypted bytes not equal to original",
        );

        let e_str = encrypted.decrypt_string();
        assert!(e_str.is_ok(), "could not decrypt ciphertext to string");
        assert_eq!(
            value,
            e_str.unwrap(),
            "decrypted string not equal to original",
        );
    }
}
