use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;

pub fn hash(password: &str) -> String {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let argon2: Argon2 = Argon2::default();
    let password_hash: String = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap_or_else(|_| panic!("error hashing password"))
        .to_string();
    password_hash
}

pub fn verify(password: &str, hash: &String) -> bool {
    let parsed_hash: PasswordHash =
        PasswordHash::new(hash).unwrap_or_else(|_| panic!("error parsing hash"));
    // assert hash
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
