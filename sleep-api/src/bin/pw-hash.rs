use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use std::io::{self, Read};

fn main() {
    // Read password from stdin (echoed). To avoid echo, consider using the `rpassword` crate.
    eprintln!(
        "Enter password on stdin. Input will be echoed. Press Ctrl+D (Unix) or Ctrl+Z then Enter (Windows) to end:"
    );
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("failed to read stdin");
    let password = buf.trim_end_matches(&['\n', '\r'][..]).as_bytes();

    let salt = SaltString::generate(rand::rngs::OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password, &salt)
        .expect("hashing failed");
    println!("{hash}");
}
