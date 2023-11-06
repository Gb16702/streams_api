use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Argon2};

pub fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    return Ok(password_hash);
}