use rand::Rng;
use rand::distributions::Alphanumeric;
use std::iter;

pub trait SaltGenerator {
    fn gen_salt() -> String;
    /// Generate a salt and return salt and hashed.
    fn gen_salt_pair(password: &str) -> (String, String) {
        let salt = Self::gen_salt();
        let salted_password = format!("{}{}", salt, password);
        let hash = bcrypt::hash(salted_password, bcrypt::DEFAULT_COST).unwrap();
        (salt, hash)
    }

    /// Input salt password,and hash.Return true if password is correct.
    fn verify(salt: &str, password: &str, hash: &str) -> bool {
        let salted_password = format!("{}{}", salt, password);
        bcrypt::verify(salted_password, hash).unwrap()
    }
}

pub struct DefaultSaltGenerator;

impl SaltGenerator for DefaultSaltGenerator {
    fn gen_salt() -> String {
        let mut rng = rand::thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(16)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::DefaultSaltGenerator;
    use super::*;

    #[test]
    fn test_default_salt_generator() {
        let salt = DefaultSaltGenerator::gen_salt();
        println!("{}", salt);
        assert_eq!(salt.len(), 16);
    }
}
