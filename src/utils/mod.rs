pub(crate) mod jwt;

pub(crate) fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}
