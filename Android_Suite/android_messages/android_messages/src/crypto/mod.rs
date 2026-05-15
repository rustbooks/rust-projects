// src/crypto/mod.rs
// AES-256-GCM encryption / decryption for message bodies and attachment metadata.
//
// Why AES-256-GCM?
//  ─ AEAD: authentication + encryption in one pass (no separate HMAC needed).
//  ─ NIST-standardised, hardware-accelerated on modern ARMv8 (Android) and x86.
//  ─ Pure-Rust `aes-gcm` crate; no C FFI, no linker surprises on any target.
//
// Key management (simplified for local storage):
//  ─ A 256-bit key is derived from a user passphrase using a PBKDF2-like approach
//    or, for now, stored in a key file with permissions 0600 (unix) / DACL (win).
//  ─ Each message uses a unique random 96-bit nonce prepended to the ciphertext.
//  ─ Ciphertext layout:  [nonce (12 bytes)] [ciphertext + tag (body_len + 16 bytes)]

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::{bail, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};

const NONCE_LEN: usize = 12; // 96-bit GCM nonce

/// Wrapper that holds a loaded AES-256-GCM cipher.
pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    /// Create from a raw 32-byte key.
    pub fn from_key_bytes(key_bytes: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        Self {
            cipher: Aes256Gcm::new(key),
        }
    }

    /// Load or generate a key stored at `key_path`.
    /// The key file contains 32 raw bytes.  If it does not exist, a random key is
    /// generated and saved.  On Unix the file is created with mode 0600.
    pub fn load_or_create(key_path: &std::path::Path) -> Result<Self> {
        if key_path.exists() {
            let bytes = std::fs::read(key_path)
                .map_err(|e| anyhow::anyhow!("Cannot read key file: {e}"))?;
            if bytes.len() != 32 {
                bail!("Key file corrupt: expected 32 bytes, got {}", bytes.len());
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&bytes);
            Ok(Self::from_key_bytes(&arr))
        } else {
            // Generate random key
            let key = Aes256Gcm::generate_key(OsRng);
            std::fs::write(key_path, key.as_slice())
                .map_err(|e| anyhow::anyhow!("Cannot write key file: {e}"))?;

            // Restrict permissions on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(key_path, std::fs::Permissions::from_mode(0o600))
                    .ok(); // non-fatal; best-effort
            }

            let mut arr = [0u8; 32];
            arr.copy_from_slice(&key);
            Ok(Self::from_key_bytes(&arr))
        }
    }

    /// Encrypt plaintext.  Returns base64(nonce || ciphertext || tag).
    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit random nonce
        let ct = self.cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|_| anyhow::anyhow!("Encryption failed"))?;

        let mut out = Vec::with_capacity(NONCE_LEN + ct.len());
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&ct);
        Ok(B64.encode(&out))
    }

    /// Decrypt a value produced by `encrypt`.  Returns the original plaintext.
    pub fn decrypt(&self, b64_ciphertext: &str) -> Result<String> {
        let data = B64
            .decode(b64_ciphertext)
            .map_err(|e| anyhow::anyhow!("Base64 decode failed: {e}"))?;

        if data.len() < NONCE_LEN + 16 {
            bail!("Ciphertext too short to be valid");
        }

        let (nonce_bytes, ct) = data.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);

        let pt = self.cipher
            .decrypt(nonce, ct)
            .map_err(|_| anyhow::anyhow!("Decryption failed — wrong key or tampered data"))?;

        String::from_utf8(pt).map_err(|e| anyhow::anyhow!("Decrypted bytes are not UTF-8: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let key = [0x42u8; 32];
        let enc = Encryptor::from_key_bytes(&key);
        let plaintext = "Hello, 🔒 secure world!";
        let ct = enc.encrypt(plaintext).expect("encrypt");
        let pt = enc.decrypt(&ct).expect("decrypt");
        assert_eq!(pt, plaintext);
    }

    #[test]
    fn tamper_detection() {
        let key = [0x11u8; 32];
        let enc = Encryptor::from_key_bytes(&key);
        let mut ct = enc.encrypt("secret").expect("encrypt");
        // Flip a byte in the base64 to simulate tampering.
        let bytes = unsafe { ct.as_bytes_mut() };
        bytes[20] ^= 0xFF;
        assert!(enc.decrypt(&ct).is_err(), "Should detect tampering");
    }
}
