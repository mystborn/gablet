pub fn generate_password_hash(password: &str) -> Option<String> {
    match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
        Ok(result) => Some(result),
        Err(_) => None
    }
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    match bcrypt::verify(password, &hashed_password) {
        Ok(result) => result,
        Err(_) => false
    }
}