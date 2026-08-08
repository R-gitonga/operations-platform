use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|error| error.to_string())?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<bool, String> {
    let parsed_hash =
        PasswordHash::new(password_hash)
            .map_err(|error| error.to_string())?;

    Ok(
        Argon2::default()
            .verify_password(
                password.as_bytes(),
                &parsed_hash,
            )
            .is_ok()
    )
}