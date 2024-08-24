use aes::Aes256;
use cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

#[derive(Debug, Clone)]
pub struct Aes {
    encryptor: Aes256CbcEnc,
    decryptor: Aes256CbcDec,
}

impl Aes {
    pub fn new(key: String, iv: String) -> Result<Self, super::Error> {
        let encryptor =
            Aes256CbcEnc::new_from_slices(key.as_bytes(), iv.as_bytes()).map_err(|err| {
                super::Error {
                    cause: err.to_string(),
                    message: String::from("failed initialized aes key and iv"),
                    status: 500,
                }
            })?;
        let decryptor =
            Aes256CbcDec::new_from_slices(key.as_bytes(), iv.as_bytes()).map_err(|err| {
                super::Error {
                    cause: err.to_string(),
                    message: String::from("failed initialized aes key and iv"),
                    status: 500,
                }
            })?;
        Ok(Self {
            encryptor,
            decryptor,
        })
    }

    pub fn encrypt(self, plain_text: String) -> Result<String, super::Error> {
        let mut buf = [0; 4096];
        let msg_len = plain_text.len();
        buf[..msg_len].copy_from_slice(&plain_text.as_bytes());
        let cipher_text = self
            .encryptor
            .encrypt_padded_mut::<Pkcs7>(&mut buf, msg_len)
            .map_err(|err| super::Error {
                cause: err.to_string(),
                message: String::from("failed to encrypt plain text"),
                status: 500,
            })?;
        return Ok(hex::encode(cipher_text));
    }

    pub fn decrypt(self, cipher_text: String) -> Result<String, super::Error> {
        let mut hex_byte = hex::decode(cipher_text).map_err(|err| super::Error {
            cause: err.to_string(),
            status: 500,
            message: String::from("failed to decode hex"),
        })?;

        let plain_text_byte = self
            .decryptor
            .decrypt_padded_mut::<Pkcs7>(&mut hex_byte)
            .map_err(|err| super::Error {
                cause: err.to_string(),
                status: 500,
                message: String::from("failed to decode hex"),
            })?;
        return String::from_utf8(plain_text_byte.to_vec()).map_err(|err| super::Error {
            cause: err.to_string(),
            status: 500,
            message: String::from("failed to decode hex"),
        });
    }
}
