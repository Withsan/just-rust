use std::path::Path;

use ring::{error::Unspecified, rand, rsa, signature};

pub fn rsa_sign_test() -> Result<(), Unspecified> {
    let private_key_der_bytes = read_file(Path::new("private_key.der")).unwrap();
    tracing::info!("{:?}", private_key_der_bytes);
    let key_pair = rsa::KeyPair::from_der(&private_key_der_bytes)?;
    tracing::info!("{:#?}", key_pair);
    const MESSAGE: &[u8] = b"hello world";
    let mut signature = vec![0; key_pair.public().modulus_len()];
    let rng = rand::SystemRandom::new();
    key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature)?;
    let public_key = signature::UnparsedPublicKey::new(
        &signature::RSA_PKCS1_2048_8192_SHA256,
        read_file(Path::new("public_key.der")).unwrap(),
    );
    public_key.verify(MESSAGE, &signature)
}
#[derive(Debug)]
enum MyError {
    IO(std::io::Error),
}
fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
    use std::io::Read;
    let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .map_err(|e| MyError::IO(e))?;
    Ok(contents)
}
