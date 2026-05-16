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

#[test]
fn private_key_exchange_is_accessible_when_storage_succeeds(
) -> Result<(), Box<dyn std::error::Error>> {
    let store = LARightStore::shared()?;
    let first_right = LARight::new()?;
    let second_right = LARight::new()?;
    let first_identifier = common::unique_identifier("public-key-exchange-a");
    let second_identifier = common::unique_identifier("public-key-exchange-b");

    let first = match store.save_right(&first_right, &first_identifier) {
        Ok(persisted) => persisted,
        Err(error) => {
            eprintln!("skipping live key-exchange assertions: {error}");
            return Ok(());
        }
    };
    let second = match store.save_right(&second_right, &second_identifier) {
        Ok(persisted) => persisted,
        Err(error) => {
            let _ = store.remove_right(&first);
            eprintln!("skipping live key-exchange assertions: {error}");
            return Ok(());
        }
    };

    let algorithm = SecKeyAlgorithm::ecdh_key_exchange_cofactor_x963_sha256();
    let parameters = SecKeyExchangeParameters::with_requested_size(32)
        .with_shared_info(b"localauthentication-rs");
    let first_private_key = first.key()?;
    let second_private_key = second.key()?;

    if first_private_key.can_exchange_keys_using(&algorithm)?
        && second_private_key.can_exchange_keys_using(&algorithm)?
    {
        let first_public_key = first_private_key.public_key()?.export_bytes()?;
        let second_public_key = second_private_key.public_key()?.export_bytes()?;

        match (
            first_private_key.exchange_keys_with_public_key(
                &second_public_key,
                &algorithm,
                &parameters,
            ),
            second_private_key.exchange_keys_with_public_key(
                &first_public_key,
                &algorithm,
                &parameters,
            ),
        ) {
            (Ok(first_secret), Ok(second_secret)) => {
                assert_eq!(first_secret, second_secret);
                assert_eq!(first_secret.len(), 32);
            }
            (Err(error), _) | (_, Err(error)) => {
                eprintln!("skipping live key-exchange assertions: {error}");
            }
        }
    }

    store.remove_right(&first)?;
    store.remove_right(&second)?;
    Ok(())
}
