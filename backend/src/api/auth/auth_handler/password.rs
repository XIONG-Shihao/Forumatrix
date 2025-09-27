use anyhow::{bail, Result};
use bcrypt::{hash, verify, DEFAULT_COST};

/// Hash a password with bcrypt. DEFAULT_COST ~ 12. You can raise if desired.
pub fn hash_password(plain: &str) -> Result<String> {
    let hashed = hash(plain, DEFAULT_COST)?;
    Ok(hashed)
}

/// Return Ok(()) if the password matches, Err otherwise (so callers can map to 401).
pub fn verify_password(plain: &str, hashed: &str) -> Result<()> {
    // Optional format guard so we fail clearly if old Argon2 hashes remain in DB.
    if !hashed.starts_with("$2") {
        bail!("unsupported password hash format (expected bcrypt)");
    }
    if verify(plain, hashed)? {
        Ok(())
    } else {
        bail!("password mismatch");
    }
}
