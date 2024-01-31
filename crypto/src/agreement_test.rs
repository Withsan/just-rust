use ring::{agreement, error::Unspecified, rand};

pub fn agreement_test() -> Result<(), Unspecified> {
    let rng = rand::SystemRandom::new();
    let my_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
    let _my_public_key = my_private_key.compute_public_key()?;
    let peer_public_key_bytes = {
        let peer_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
        peer_private_key.compute_public_key()?
    };
    let peer_public_key =
        agreement::UnparsedPublicKey::new(&agreement::X25519, peer_public_key_bytes);
    agreement::agree_ephemeral(my_private_key, &peer_public_key, |key_material| {
        println!("{:?}", key_material)
    })?;
    Ok(())
}
