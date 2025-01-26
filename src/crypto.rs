use ring::{
    aead::{self, LessSafeKey, Nonce, UnboundKey, NONCE_LEN},
    agreement, hkdf,
    rand::{SecureRandom, SystemRandom},
};
use anyhow::Result;
use crate::error::ChatError;

const KEY_LENGTH: usize = 32;

pub struct Crypto {
    rng: SystemRandom,
}

impl Crypto {
    pub fn new() -> Self {
        Crypto {
            rng: SystemRandom::new(),
        }
    }

    pub fn generate_key_pair(&self) -> Result<(Vec<u8>, Vec<u8>), ChatError> {
        let private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &self.rng)?;
        let public_key = private_key.compute_public_key()?.as_ref().to_vec();
        Ok((public_key, private_key.into()))
    }

    pub fn encrypt(&self, key: &LessSafeKey, message: &[u8]) -> Result<(Vec<u8>, Vec<u8>), ChatError> {
        let mut nonce = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce)?;
        
        let mut in_out = message.to_vec();
        key.seal_in_place_append_tag(Nonce::assume_unique_for_key(nonce), aead::Aad::empty(), &mut in_out)
            .map_err(|_| ChatError::CryptoError("Encryption failed".into()))?;

        Ok((nonce.to_vec(), in_out))
    }

    pub fn decrypt(&self, key: &LessSafeKey, nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, ChatError> {
        let nonce = Nonce::try_assume_unique_for_key(nonce)
            .map_err(|_| ChatError::CryptoError("Invalid nonce".into()))?;
        
        let mut in_out = ciphertext.to_vec();
        key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|_| ChatError::CryptoError("Decryption failed".into()))?;
        
        in_out.truncate(in_out.len() - key.algorithm().tag_len());
        Ok(in_out)
    }

    pub fn perform_key_exchange(
        &self,
        private_key: &[u8],
        peer_public_key: &[u8],
    ) -> Result<(LessSafeKey, LessSafeKey), ChatError> {
        let shared_secret = agreement::agree_ephemeral(
            agreement::UnparsedPublicKey::new(&agreement::X25519, peer_public_key),
            &agreement::X25519,
            private_key,
            |key_material| Ok(key_material.to_vec()),
        )?;

        let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, &[]);
        let prk = salt.extract(&shared_secret);
        
        let encrypt_key = self.derive_key(&prk, b"encrypt")?;
        let decrypt_key = self.derive_key(&prk, b"decrypt")?;

        Ok((encrypt_key, decrypt_key))
    }

    fn derive_key(&self, prk: &hkdf::Prk, info: &[u8]) -> Result<LessSafeKey, ChatError> {
        let mut okm = vec![0u8; KEY_LENGTH];
        prk.expand(&[info], &aead::CHACHA20_POLY1305)?
            .fill(&mut okm)
            .map_err(|_| ChatError::CryptoError("Key derivation failed".into()))?;

        UnboundKey::new(&aead::CHACHA20_POLY1305, &okm)
            .map(LessSafeKey::new)
            .map_err(|_| ChatError::CryptoError("Key creation failed".into()))
    }
}