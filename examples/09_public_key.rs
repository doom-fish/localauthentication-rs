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
    let right = LARight::new()?;
    let identifier = unique_identifier("public-key");

    match store.save_right(&right, &identifier) {
        Ok(persisted) => {
            let public_key = persisted.public_key()?;
            let sign = SecKeyAlgorithm::ecdsa_signature_message_x962_sha256();
            let encrypt =
                SecKeyAlgorithm::ecies_encryption_cofactor_variable_iv_x963_sha256_aes_gcm();

            println!("public key bytes: {}", public_key.export_bytes()?.len());
            println!("can verify: {}", public_key.can_verify_using(&sign)?);
            println!("can encrypt: {}", public_key.can_encrypt_using(&encrypt)?);
            println!(
                "private key can sign: {}",
                persisted.key()?.can_sign_using(&sign)?
            );
            store.remove_right(&persisted)?;
        }
        Err(error) => {
            println!("public-key APIs need entitlements on many systems: {error}");
        }
    }

    println!("✅ public-key smoke OK");
    Ok(())
}
