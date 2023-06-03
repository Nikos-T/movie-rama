use argonautica::{Hasher, Verifier};

#[test]
fn test_argonautica() {

    let hash_secret = "hash_secret".to_string();
    let mut hasher = Hasher::default();
    let password_hash = hasher
        .with_password("pass".to_string())
        .with_secret_key(hash_secret.clone())
        .hash()
        .unwrap();

    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(&password_hash)
        .with_password("pass".to_string())
        .with_secret_key(hash_secret)
        .verify();

    println!("{is_valid:?}");

    // assert_eq!(is_valid, true);
}
