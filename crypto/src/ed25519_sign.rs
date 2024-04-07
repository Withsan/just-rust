use ring::{
    error::Unspecified,
    rand,
    signature::{self, KeyPair},
};

pub fn ed25519_sign_test() -> Result<(), Unspecified> {
    let rng = rand::SystemRandom::new();
    let pkcs8_key_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_key_bytes.as_ref())?;
    const MESSAGE: &[u8] = b"hello world";
    let signature = key_pair.sign(MESSAGE);
    let public_key_bytes = key_pair.public_key().as_ref();
    let public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key_bytes);
    public_key.verify(MESSAGE, signature.as_ref())?;
    Ok(())
}
