use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};
use aes::Aes256;
use block_modes::{block_padding, BlockMode, Cbc};
use crypto_hash::{hex_digest, Algorithm};

type Aes256Cbc = Cbc<Aes256, block_padding::Pkcs7>;

#[derive(QObject, Default)]
pub struct Encipher {
    base: qt_base_class!(trait QObject),

    encrypt: qt_method!(fn(&mut self, password: QString, text: QString) -> QString),
    decrypt: qt_method!(fn(&mut self, password: QString, text: QString) -> QString),
    verify: qt_method!(fn(&self, password: QString, text: QString) -> bool),
}

impl Encipher {
    pub fn init_from_engine(engine: &mut QmlEngine, encipher: QObjectPinned<Encipher>) {
        engine.set_object_property("encipher".into(), encipher);
    }

    fn key_iv(password: &str) -> (Vec<u8>, Vec<u8>) {
        let key = hex_digest(Algorithm::SHA256, password.as_bytes());
        let key = hex::decode(key).expect("Decoding key failed");

        let iv = hex_digest(Algorithm::MD5, password.as_bytes());
        let iv = hex::decode(iv).expect("Decoding iv failed");
        return (key, iv);
    }

    fn encrypt(&self, password: QString, text: QString) -> QString {
        let text = text.to_string();
        let (key, iv) = Encipher::key_iv(&password.to_string());
        let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

        let pos = text.len();
        if pos > 4096 {
            return "buffer is too small, can not filled with all text."
                .to_string()
                .into();
        }
        let mut buffer = [0u8; 4096];
        buffer[..pos].copy_from_slice(text.as_bytes());
        let text = cipher.encrypt(&mut buffer, pos).unwrap();

        return hex::encode(text).into();
    }

    fn decrypt(&self, password: QString, text: QString) -> QString {
        let text = text.to_string();
        let (key, iv) = Encipher::key_iv(&password.to_string());

        let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
        let buf = hex::decode(text.as_bytes());
        if buf.is_err() {
            return "Decoding text failed".to_string().into();
        }
        let mut buf = buf.unwrap().to_vec();
        let text = cipher.decrypt(&mut buf).unwrap();

        return String::from_utf8_lossy(text).to_string().into();
    }

    fn verify(&self, password: QString, text: QString) -> bool {
        let password = password.to_string();
        let text = text.to_string();
        let hex_text = self.encrypt(password.clone().into(), text.clone().into());
        let dec_text = self.decrypt(password.clone().into(), hex_text);
        return text == dec_text.to_string();
    }
}
