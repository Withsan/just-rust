use ring::{
    digest, error,
    hmac::{self},
    rand,
};

pub fn hamc_test() -> Result<(), error::Unspecified> {
    let rng = rand::SystemRandom::new();
    let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng)?;
    let msg = "hello world";
    let tag = hmac::sign(&key, msg.as_bytes());
    hmac::verify(&key, msg.as_bytes(), tag.as_ref())?;
    Ok(())
}
pub fn multi_part() -> Result<(), error::Unspecified> {
    let parts = vec!["hello", "mothor", "fucker"];
    let rng = rand::SystemRandom::new();
    let key_value: [u8; digest::SHA384_OUTPUT_LEN] = rand::generate(&rng)?.expose();
    let s_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
    let mut s_ctx = hmac::Context::with_key(&s_key);
    for part in &parts {
        s_ctx.update(part.as_bytes());
    }
    let tag = s_ctx.sign();
    let v_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
    let mut msg = Vec::<u8>::new();
    for part in &parts {
        msg.extend(part.as_bytes());
    }
    hmac::verify(&v_key, &msg.as_ref(), tag.as_ref())?;
    Ok(())
}
