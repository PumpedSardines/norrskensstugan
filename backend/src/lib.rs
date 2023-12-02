use sha3::Digest;

pub mod database;
pub mod routes;

pub fn hash_password(password: &str) -> String {
    format!("{:x}", sha3::Sha3_256::digest(password.as_bytes()))
}
