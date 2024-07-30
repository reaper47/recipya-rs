use hmac::{Hmac, Mac};
use sha2::Sha512;

pub use self::error::{Error, Result};

mod error;
pub mod password;

pub struct EncryptContent {
    pub content: String,
    pub salt: String,
}

pub fn encrypt_into_b64u(key: &[u8], enc_content: &EncryptContent) -> Result<String> {
    let EncryptContent { content, salt } = enc_content;

    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    let hmac_result = hmac_sha512.finalize().into_bytes();
    let result = base64_url::encode(&hmac_result);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rand::RngCore;

    use super::*;

    #[test]
    fn test_encrypt_into_b64u_ok() -> Result<()> {
        let mut fx_key = [0u8; 64];
        rand::thread_rng().fill_bytes(&mut fx_key);
        let fx_enc_content = EncryptContent {
            content: "hello world".to_string(),
            salt: "some pepper".to_string(),
        };

        let fx_res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

        let res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;
        assert_eq!(res, fx_res);
        Ok(())
    }
}
