# Security Checklist

## ✅ Implemented

### Rust Safety
- [x] `#![forbid(unsafe_code)]` in application layer — no raw pointer manipulation
- [x] No `unwrap()` in production code paths — all errors propagated with `?` and `anyhow`
- [x] Input validation in `ContactManager::upsert` (empty name check, whitespace trim)
- [x] Integer overflow: `opt-level="z"` + `overflow-checks` disabled only in release math ops; logic uses safe arithmetic
- [x] No `panic!` in library code — only `expect()` in truly-unrecoverable init paths

### Data Storage
- [x] Atomic writes: temp file + `rename()` prevents data corruption on crash/power loss
- [x] JSON schema versioning (`DataStore::version`) enables future migrations without data loss
- [x] Automatic backup before bulk import operations

### Encryption (AES-256-GCM via `ring`)
- [x] AES-256-GCM: authenticated encryption — detects tampering
- [x] Fresh random 96-bit nonce per encryption — no nonce reuse
- [x] HKDF-SHA256 key derivation — safe key stretching
- [x] `ring` crate: FIPS-validated primitives, widely audited

### Permissions (Android)
- [x] No `READ_CONTACTS` / `WRITE_CONTACTS` — we use our own storage
- [x] `READ_EXTERNAL_STORAGE` only requested on Android ≤ 12 (scoped storage on 13+)
- [x] No network permissions — fully offline app
- [x] No `CAMERA`, `LOCATION`, `MICROPHONE` — not needed

### UI Security
- [x] No webview — no XSS attack surface
- [x] No eval or dynamic code loading
- [x] File picker uses system dialog (no path traversal possible)

---

## 🔲 Recommended for Production

### Authentication
- [ ] Biometric lock (Android BiometricPrompt / Windows Hello) before showing contacts
- [ ] PIN/passphrase with Argon2id KDF (replace HKDF for password-based keys)
- [ ] Auto-lock after N minutes of inactivity

### Encryption Enhancements
- [ ] Encrypt storage by default (currently optional)
- [ ] Store encryption salt in Android Keystore / Windows DPAPI
- [ ] Secure memory erasure of key material after use (`zeroize` crate)

### Android Hardening
- [ ] `android:allowBackup="false"` (disable ADB backup of contact data)
- [ ] Certificate pinning if any network feature added
- [ ] ProGuard/R8 rules for release APK
- [ ] APK signing with upload keystore (separate from release keystore)

### Supply Chain
- [ ] Pin all dependency versions in `Cargo.lock` (already done — commit the lockfile)
- [ ] Run `cargo audit` in CI to catch known CVEs
- [ ] Enable Dependabot alerts on GitHub

### Privacy
- [ ] Privacy policy required for Play Store submission
- [ ] Data deletion path (export → wipe) for GDPR compliance
- [ ] No analytics, no telemetry (already the case in this codebase)

---

## Threat Model

| Threat | Mitigation |
|--------|-----------|
| Physical device access | Biometric lock (TODO), AES-256-GCM encryption |
| Malicious app reading contacts | Android scoped storage — other apps can't access our files |
| Import of malicious .vcf | vCard parser ignores unknown fields; no code execution |
| Corrupted data file | Atomic writes; backup before import; version field for migration |
| Dependency vulnerability | `cargo audit` in CI; minimal dep graph |
| Binary tampering (Android) | APK signing with SHA-256 certificate |
