// src/storage.rs — JSON + optional AES-256-GCM encrypted persistence

use std::{fs, io, path::{Path, PathBuf}};
use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Key, Nonce};
use rand::RngCore;
use crate::models::{AppSettings, CallRecord, Contact};

#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),
    Json(serde_json::Error),
    Crypto(String),
}
impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e)     => write!(f, "IO: {}", e),
            Self::Json(e)   => write!(f, "JSON: {}", e),
            Self::Crypto(e) => write!(f, "Crypto: {}", e),
        }
    }
}
impl From<io::Error>           for StorageError { fn from(e: io::Error) -> Self { Self::Io(e) } }
impl From<serde_json::Error>   for StorageError { fn from(e: serde_json::Error) -> Self { Self::Json(e) } }

pub struct Storage {
    data_dir: PathBuf,
    settings: AppSettings,
    cipher:   Option<Aes256Gcm>,
}

impl Storage {
    pub fn init() -> Result<Self, StorageError> {
        let data_dir = Self::resolve_data_dir();
        fs::create_dir_all(&data_dir)?;

        let settings_path = data_dir.join("settings.json");
        let settings = if settings_path.exists() {
            let raw = fs::read_to_string(&settings_path)?;
            serde_json::from_str(&raw).unwrap_or_default()
        } else {
            let s = AppSettings::default();
            fs::write(&settings_path, serde_json::to_string_pretty(&s)?)?;
            s
        };

        let cipher = if settings.encryption_enabled {
            if let Some(ref kb64) = settings.encrypted_key_b64 {
                let kb = Self::b64_decode(kb64)
                    .map_err(|e| StorageError::Crypto(e))?;
                if kb.len() == 32 {
                    let key = Key::<Aes256Gcm>::from_slice(&kb);
                    Some(Aes256Gcm::new(key))
                } else { None }
            } else { None }
        } else { None };

        Ok(Self { data_dir, settings, cipher })
    }

    pub fn load_contacts(&self) -> Result<Vec<Contact>, StorageError> {
        let p = self.contacts_path();
        if !p.exists() { return Ok(vec![]); }
        Ok(serde_json::from_slice(&self.read_file(&p)?)?)
    }

    pub fn save_contacts(&self, contacts: &[Contact]) -> Result<(), StorageError> {
        self.write_file(&self.contacts_path(), &serde_json::to_vec_pretty(contacts)?)
    }

    pub fn load_call_history(&self) -> Result<Vec<CallRecord>, StorageError> {
        let p = self.data_dir.join("call_history.json");
        if !p.exists() { return Ok(vec![]); }
        Ok(serde_json::from_slice(&self.read_file(&p)?)?)
    }

    pub fn save_call_history(&self, records: &[CallRecord]) -> Result<(), StorageError> {
        self.write_file(&self.data_dir.join("call_history.json"), &serde_json::to_vec_pretty(records)?)
    }

    pub fn settings(&self) -> &AppSettings { &self.settings }

    pub fn save_settings(&mut self, s: &AppSettings) -> Result<(), StorageError> {
        self.write_file(&self.data_dir.join("settings.json"), &serde_json::to_vec_pretty(s)?)?;
        self.settings = s.clone();
        Ok(())
    }

    // ── Private ───────────────────────────────────────────────────────────────

    fn resolve_data_dir() -> PathBuf {
        if let Ok(d) = std::env::var("ANDROID_DATA_DIR") { return PathBuf::from(d).join("rust-dialer"); }
        dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")).join("rust-dialer")
    }

    fn contacts_path(&self) -> PathBuf {
        if self.settings.encryption_enabled { self.data_dir.join("contacts.enc") }
        else                                { self.data_dir.join("contacts.json") }
    }

    fn read_file(&self, p: &Path) -> Result<Vec<u8>, StorageError> {
        let raw = fs::read(p)?;
        if let Some(ref c) = self.cipher { Self::decrypt(c, &raw) } else { Ok(raw) }
    }

    fn write_file(&self, p: &Path, data: &[u8]) -> Result<(), StorageError> {
        let payload = if let Some(ref c) = self.cipher { Self::encrypt(c, data)? } else { data.to_vec() };
        let tmp = p.with_extension("tmp");
        fs::write(&tmp, &payload)?;
        fs::rename(&tmp, p)?;
        Ok(())
    }

    fn encrypt(c: &Aes256Gcm, pt: &[u8]) -> Result<Vec<u8>, StorageError> {
        let mut nb = [0u8; 12]; OsRng.fill_bytes(&mut nb);
        let nonce = Nonce::from_slice(&nb);
        let mut ct = c.encrypt(nonce, pt).map_err(|e| StorageError::Crypto(e.to_string()))?;
        let mut out = nb.to_vec(); out.append(&mut ct); Ok(out)
    }

    fn decrypt(c: &Aes256Gcm, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        if data.len() < 12 { return Err(StorageError::Crypto("Too short".into())); }
        let (nb, ct) = data.split_at(12);
        c.decrypt(Nonce::from_slice(nb), ct).map_err(|e| StorageError::Crypto(e.to_string()))
    }

    pub fn b64_encode(b: &[u8]) -> String {
        const C: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut o = Vec::with_capacity((b.len()+2)/3*4);
        for ch in b.chunks(3) {
            let (b0,b1,b2) = (ch[0] as usize, if ch.len()>1{ch[1] as usize}else{0}, if ch.len()>2{ch[2] as usize}else{0});
            o.push(C[(b0>>2)&0x3F]); o.push(C[((b0<<4)|(b1>>4))&0x3F]);
            if ch.len()>1{o.push(C[((b1<<2)|(b2>>6))&0x3F]);}else{o.push(b'=');}
            if ch.len()>2{o.push(C[b2&0x3F]);}else{o.push(b'=');}
        }
        String::from_utf8(o).unwrap_or_default()
    }

    pub fn b64_decode(s: &str) -> Result<Vec<u8>, String> {
        let s = s.trim_end_matches('=');
        let chars: Vec<u8> = s.bytes().map(|b| match b {
            b'A'..=b'Z'=>b-b'A', b'a'..=b'z'=>b-b'a'+26, b'0'..=b'9'=>b-b'0'+52,
            b'+'=> 62, b'/'=> 63, _=> 64
        }).collect();
        if chars.iter().any(|&c|c==64) { return Err("Invalid base64".into()); }
        let mut o = Vec::new();
        for ch in chars.chunks(4) {
            let v = (ch[0] as usize)<<18 | (*ch.get(1).unwrap_or(&0) as usize)<<12
                  | (*ch.get(2).unwrap_or(&0) as usize)<<6 | (*ch.get(3).unwrap_or(&0) as usize);
            o.push((v>>16) as u8);
            if ch.len()>2{o.push((v>>8) as u8);}
            if ch.len()>3{o.push(v as u8);}
        }
        Ok(o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn b64_round_trip() {
        let d = b"Hello AES-256-GCM!";
        assert_eq!(Storage::b64_decode(&Storage::b64_encode(d)).unwrap(), d);
    }
    #[test] fn encrypt_decrypt() {
        let mut k=[0u8;32]; OsRng.fill_bytes(&mut k);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&k));
        let pt = b"secret";
        let enc = Storage::encrypt(&cipher, pt).unwrap();
        assert_eq!(Storage::decrypt(&cipher, &enc).unwrap(), pt);
    }
}
