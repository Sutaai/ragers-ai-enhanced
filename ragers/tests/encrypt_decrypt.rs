//! Integration tests for the encryption/decryption seam.
//!
//! These exercise `get_encrypted_content` / `get_decrypted_content` through a
//! round-trip, in both binary and armored formats.

use std::str::FromStr;

use age::x25519::Identity;
use ragers::cli::{get_decrypted_content, get_encrypted_content};
use ragers::config::AsArmorFormat;

/// A known-good age secret key (User 1 from `tests/definitions/`).
const SECRET_KEY: &str =
    "AGE-SECRET-KEY-1M3C33J4SRQJ0J9FJMPD2V8FTTPWZL7SQ9LTZQPHT9JSNRCLUG9DSR9M2EL";

#[test]
fn encrypts_and_decrypts_a_round_trip_in_binary_format() {
    let identity = Identity::from_str(SECRET_KEY).unwrap();
    let recipient: Box<dyn age::Recipient> = Box::new(identity.to_public());
    let plaintext = b"the quick brown fox jumps over the lazy dog";

    let encrypted =
        get_encrypted_content(plaintext, vec![recipient.as_ref()], false.as_armor_format())
            .unwrap();
    assert_ne!(encrypted, plaintext);

    let identities: Vec<&dyn age::Identity> = vec![&identity];
    let decrypted = get_decrypted_content(&encrypted, &identities).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypts_and_decrypts_a_round_trip_in_armored_format() {
    let identity = Identity::from_str(SECRET_KEY).unwrap();
    let recipient: Box<dyn age::Recipient> = Box::new(identity.to_public());
    let plaintext = b"armored content";

    let encrypted =
        get_encrypted_content(plaintext, vec![recipient.as_ref()], true.as_armor_format()).unwrap();
    assert!(encrypted.starts_with(b"-----BEGIN AGE ENCRYPTED FILE-----"));

    let identities: Vec<&dyn age::Identity> = vec![&identity];
    let decrypted = get_decrypted_content(&encrypted, &identities).unwrap();
    assert_eq!(decrypted, plaintext);
}
