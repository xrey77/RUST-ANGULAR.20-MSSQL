use bcrypt::{verify};
use bcrypt::{hash, BcryptResult, DEFAULT_COST};


pub fn hash_password(password: &str) -> BcryptResult<String> {
   hash(password, DEFAULT_COST)
}

// #[allow(dead_code)]
pub fn verify_password(password_attempt: &str, stored_hash: &str) -> BcryptResult<bool> {
    verify(password_attempt, stored_hash)
}


