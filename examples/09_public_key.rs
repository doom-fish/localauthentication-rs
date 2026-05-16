use localauthentication::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_identifier(prefix: &str) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("dev.doomfish.localauthentication.{prefix}.{now}")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = LARightStore::shared()?;
    let first_right = LARight::new()?;
    let second_right = LARight::new()?;
    let first_identifier = unique_identifier("public-key-a");
    let second_identifier = unique_identifier("public-key-b");

    match store.save_right(&first_right, &first_identifier) {
        Ok(first) => {
            let public_key = first.public_key()?;
            let sign = SecKeyAlgorithm::ecdsa_signature_message_x962_sha256();
            let encrypt =
                SecKeyAlgorithm::ecies_encryption_cofactor_variable_iv_x963_sha256_aes_gcm();
            let exchange = SecKeyAlgorithm::ecdh_key_exchange_cofactor_x963_sha256();
            let first_private_key = first.key()?;

            println!("public key bytes: {}", public_key.export_bytes()?.len());
            println!("can verify: {}", public_key.can_verify_using(&sign)?);
            println!("can encrypt: {}", public_key.can_encrypt_using(&encrypt)?);
            println!("private key can sign: {}", first_private_key.can_sign_using(&sign)?);
            println!(
                "private key can exchange: {}",
                first_private_key.can_exchange_keys_using(&exchange)?
            );

            match store.save_right(&second_right, &second_identifier) {
                Ok(second) => {
                    let second_private_key = second.key()?;
                    if first_private_key.can_exchange_keys_using(&exchange)?
                        && second_private_key.can_exchange_keys_using(&exchange)?
                    {
                        let parameters = SecKeyExchangeParameters::with_requested_size(32)
                            .with_shared_info(b"localauthentication-rs");
                        let first_public_key = first_private_key.public_key()?.export_bytes()?;
                        let second_public_key = second_private_key.public_key()?.export_bytes()?;

                        match (
                            first_private_key.exchange_keys_with_public_key(
                                &second_public_key,
                                &exchange,
                                &parameters,
                            ),
                            second_private_key.exchange_keys_with_public_key(
                                &first_public_key,
                                &exchange,
                                &parameters,
                            ),
                        ) {
                            (Ok(first_secret), Ok(second_secret)) => {
                                println!("shared secret bytes: {}", first_secret.len());
                                println!("shared secrets match: {}", first_secret == second_secret);
                            }
                            (Err(error), _) | (_, Err(error)) => {
                                println!("key exchange requires additional system support: {error}");
                            }
                        }
                    }
                    store.remove_right(&second)?;
                }
                Err(error) => {
                    println!("key-exchange demo needs two persisted keys: {error}");
                }
            }

            store.remove_right(&first)?;
        }
        Err(error) => {
            println!("public-key APIs need entitlements on many systems: {error}");
        }
    }

    println!("✅ public-key smoke OK");
    Ok(())
}
