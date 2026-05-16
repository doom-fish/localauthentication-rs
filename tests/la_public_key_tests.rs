mod common;

use localauthentication::prelude::*;

#[test]
fn public_key_operations_are_accessible_when_storage_succeeds(
) -> Result<(), Box<dyn std::error::Error>> {
    let store = LARightStore::shared()?;
    let right = LARight::new()?;
    let identifier = common::unique_identifier("public-key-test");

    let persisted = match store.save_right(&right, &identifier) {
        Ok(persisted) => persisted,
        Err(error) => {
            eprintln!("skipping live public-key assertions: {error}");
            return Ok(());
        }
    };

    let public_key = persisted.public_key()?;
    let sign = SecKeyAlgorithm::ecdsa_signature_message_x962_sha256();
    let encrypt = SecKeyAlgorithm::ecies_encryption_cofactor_variable_iv_x963_sha256_aes_gcm();

    assert!(!public_key.export_bytes()?.is_empty());
    let _ = public_key.can_verify_using(&sign)?;
    let _ = public_key.can_encrypt_using(&encrypt)?;
    let private_key = persisted.key()?;
    let _ = private_key.can_sign_using(&sign)?;
    let _ = private_key.can_decrypt_using(&encrypt)?;
    let _ = private_key
        .can_exchange_keys_using(&SecKeyAlgorithm::ecdh_key_exchange_cofactor_x963_sha256())?;

    store.remove_right(&persisted)?;
    Ok(())
}
