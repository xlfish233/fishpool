use rand::distributions::Alphanumeric;
use rand::Rng;
use std::iter;

pub trait SaltGenerator {
    fn gen_salt() -> String;
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
    use super::*;
    use super::DefaultSaltGenerator;

    #[test]
    fn test_default_salt_generator() {
        let salt = DefaultSaltGenerator::gen_salt();
        println!("{}", salt);
        assert_eq!(salt.len(), 16);
    }
}
