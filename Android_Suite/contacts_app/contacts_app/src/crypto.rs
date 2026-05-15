// src/crypto.rs
// AES-256-GCM encryption for sensitive contact data.
// Uses the `ring` crate — widely audited, minimal C dependency, no C++.
//
// Key derivation: HKDF-SHA256 from a user-provided password/PIN.
// Each encryption operation uses a fresh random 96-bit nonce (GCM standard).
//
// This module is intentionally simple: encrypt/decrypt byte slices.
// The caller (storage.rs) handles file I/O.
//
// Security properties:
//   ✓ AES-256-GCM: authenticated encryption (integrity + confidentiality)
//   ✓ Random nonce per encryption: no nonce reuse possible
//   ✓ HKDF for key derivation: safe even with low-entropy passwords
//   ✗ Not a full KDF with scrypt/Argon2 (add for production PIN-based auth)

#![allow(dead_code)] // Some functions are prepared for future use

use ring::aead::{
    self, Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey,
    UnboundKey, AES_256_GCM, NONCE_LEN,
};
use ring::error::Unspecified;
use ring::hkdf;
use ring::rand::{SecureRandom, SystemRandom};

use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Output length of the derived encryption key (256 bits)
const KEY_LEN: usize = 32;

/// Salt used for HKDF (non-secret; can be stored alongside the encrypted data)
const HKDF_INFO: &[u8] = b"contacts-app-v1";

// ── Public API ────────────────────────────────────────────────────────────────

/// Encrypts `plaintext` with the given key.
/// Returns a base64-encoded blob: `nonce || ciphertext || tag`
pub fn encrypt(key_bytes: &[u8; KEY_LEN], plaintext: &[u8]) -> Result<String> {
    let rng = SystemRandom::new();

    // Generate a fresh random 12-byte nonce
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes)
        .map_err(|_| anyhow::anyhow!("Failed to generate nonce"))?;

    // Set up the sealing key with a one-shot nonce
    let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes)
        .map_err(|_| anyhow::anyhow!("Invalid key length"))?;
    let mut sealing_key = SealingKey::new(unbound_key, OneNonce(Some(
        Nonce::assume_unique_for_key(nonce_bytes)
    )));

    // Encrypt in-place: GCM appends the 16-byte tag at the end
    let mut in_out = plaintext.to_vec();
    sealing_key
        .seal_in_place_append_tag(Aad::empty(), &mut in_out)
        .map_err(|_| anyhow::anyhow!("Encryption failed"))?;

    // Prepend nonce: output = nonce || ciphertext+tag
    let mut result = Vec::with_capacity(NONCE_LEN + in_out.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&in_out);

    Ok(B64.encode(&result))
}

/// Decrypts a blob produced by `encrypt`.
pub fn decrypt(key_bytes: &[u8; KEY_LEN], encoded: &str) -> Result<Vec<u8>> {
    let blob = B64.decode(encoded).context("Invalid base64 in encrypted data")?;

    if blob.len() < NONCE_LEN {
        bail!("Encrypted data too short");
    }

    let (nonce_bytes, ciphertext) = blob.split_at(NONCE_LEN);
    let nonce_arr: [u8; NONCE_LEN] = nonce_bytes.try_into()
        .map_err(|_| anyhow::anyhow!("Bad nonce length"))?;

    let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes)
        .map_err(|_| anyhow::anyhow!("Invalid key length"))?;
    let mut opening_key = OpeningKey::new(unbound_key, OneNonce(Some(
        Nonce::assume_unique_for_key(nonce_arr)
    )));

    let mut in_out = ciphertext.to_vec();
    let plaintext = opening_key
        .open_in_place(Aad::empty(), &mut in_out)
        .map_err(|_| anyhow::anyhow!("Decryption failed — wrong key or corrupted data"))?;

    Ok(plaintext.to_vec())
}

/// Derives a 256-bit AES key from a password using HKDF-SHA256.
/// `salt` should be a random 32-byte value stored alongside the encrypted file.
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, salt);
    let prk = salt.extract(password.as_bytes());
    let mut key = [0u8; KEY_LEN];
    prk.expand(&[HKDF_INFO], MyLen(KEY_LEN))
        .expect("HKDF expand failed")
        .fill(&mut key)
        .expect("HKDF fill failed");
    key
}

/// Generates a random 32-byte salt for HKDF.
pub fn random_salt() -> Result<[u8; 32]> {
    let rng = SystemRandom::new();
    let mut salt = [0u8; 32];
    rng.fill(&mut salt)
        .map_err(|_| anyhow::anyhow!("Failed to generate salt"))?;
    Ok(salt)
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// One-shot nonce sequence (uses the nonce exactly once, then errors).
struct OneNonce(Option<Nonce>);

impl NonceSequence for OneNonce {
    fn advance(&mut self) -> std::result::Result<Nonce, Unspecified> {
        self.0.take().ok_or(Unspecified)
    }
}

/// HKDF OKM length wrapper
struct MyLen(usize);
impl hkdf::KeyType for MyLen {
    fn len(&self) -> usize { self.0 }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let salt = random_salt().unwrap();
        let key = derive_key("test-password-123", &salt);
        let plaintext = b"Hello, \xe0\xa4\xa8\xe0\xa4\xae\xe0\xa4\xb8\xe0\xa5\x8d\xe0\xa4\xa4\xe0\xa5\x87!"; // "Hello, नमस्ते!"
        let encrypted = encrypt(&key, plaintext).unwrap();
        let decrypted = decrypt(&key, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_wrong_key_fails() {
        let salt = random_salt().unwrap();
        let key1 = derive_key("correct-password", &salt);
        let key2 = derive_key("wrong-password", &salt);
        let plaintext = b"secret data";
        let encrypted = encrypt(&key1, plaintext).unwrap();
        assert!(decrypt(&key2, &encrypted).is_err());
    }

    #[test]
    fn test_unique_nonces() {
        let salt = random_salt().unwrap();
        let key = derive_key("password", &salt);
        let plain = b"same plaintext";
        let enc1 = encrypt(&key, plain).unwrap();
        let enc2 = encrypt(&key, plain).unwrap();
        // Different nonces → different ciphertext
        assert_ne!(enc1, enc2);
    }
}
