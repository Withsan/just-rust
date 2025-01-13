use aead::BoundKey;
use ring::{
    aead::{
        self, Aad, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM, NONCE_LEN,
    },
    error,
    rand::{self, SecureRandom},
};
struct CounterNonceSequence(u32);

impl NonceSequence for CounterNonceSequence {
    fn advance(&mut self) -> Result<aead::Nonce, error::Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];
        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);
        self.0 += 1;
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}
pub fn aead_test() -> Result<(), error::Unspecified> {
    let data = b"fuck you";
    let rand = rand::SystemRandom::new();
    let mut key_bytes = vec![0; AES_256_GCM.key_len()];
    let _ = rand.fill(&mut key_bytes);
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)?;
    let nonce_sequence = CounterNonceSequence(1);
    let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);
    let associated_data = Aad::from(b"fuck");
    let mut in_out = data.clone();
    let tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut in_out)?;
    let mut cipher_with_tag = [&in_out, tag.as_ref()].concat();
    let mut opening_key = OpeningKey::new(
        UnboundKey::new(&AES_256_GCM, &key_bytes)?,
        CounterNonceSequence(1),
    );
    let decrypted_data = opening_key.open_in_place(associated_data, &mut cipher_with_tag)?;
    println!(
        "decrypted_data:{}",
        String::from_utf8(decrypted_data.to_vec()).unwrap()
    );
    Ok(())
}
