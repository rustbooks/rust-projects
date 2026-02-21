# Troubleshooting Guide

## Common Build Issues

### 1. OpenSSL Error During Compilation

**Error Message:**
```
error: failed to run custom build command for `openssl-sys`
Could not find directory of OpenSSL installation
```

**Solution A: Install System OpenSSL (Fastest - Recommended)**

Ubuntu/Debian:
```bash
sudo apt-get update
sudo apt-get install libssl-dev pkg-config
```

Fedora/RHEL/CentOS:
```bash
sudo dnf install openssl-devel pkg-config
```

Arch Linux:
```bash
sudo pacman -S openssl pkg-config
```

Then rebuild:
```bash
cd exam_paper_automation
cargo clean
cargo build
```

**Solution B: The project already uses vendored OpenSSL**

The Cargo.toml has been configured with `openssl = { version = "0.10", features = ["vendored"] }` which compiles OpenSSL from source. You just need a C compiler:

Ubuntu/Debian:
```bash
sudo apt-get install build-essential
```

Fedora:
```bash
sudo dnf groupinstall "Development Tools"
```

Then:
```bash
cargo clean
cargo build --release
```

### 2. Database Connection Issues

**Solutions:**
1. Verify PostgreSQL is running:
   ```bash
   sudo systemctl status postgresql
   ```

2. Check DATABASE_URL in `.env`

3. Test connection:
   ```bash
   psql -d exam_automation
   ```

### 3. SMTP Email Issues

For Gmail:
- Enable 2FA
- Generate App Password
- Use App Password in SMTP_PASSWORD

## Quick Fix Commands

```bash
# Install all dependencies (Ubuntu/Debian)
sudo apt-get install build-essential libssl-dev pkg-config postgresql-client

# Rebuild project
cd exam_paper_automation
cargo clean
cargo build --release
```
