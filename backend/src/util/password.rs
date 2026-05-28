const BCRYPT_COST: u32 = 10;

pub fn hash_password(password: &str) -> Result<String, ()> {
    bcrypt::hash(password, BCRYPT_COST).map_err(|_| ())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, ()> {
    bcrypt::verify(password, hash).map_err(|_| ())
}
