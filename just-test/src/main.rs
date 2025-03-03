use std::env;

use anyhow::{Ok, Result};
use openssl::{cipher::Cipher, cipher_ctx::CipherCtx};
use tracing::Level;
fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let subscribe = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscribe).expect("error");
    let data = b"fuck";
    let key = b"rust".repeat(8);
    let iv = &b"rust".repeat(8)[..];
    let iv = Some(iv);
    let cipher = Cipher::fetch(None, "SM4-GCM", None)?;
    // let cipher = Cipher::aes_256_ofb();
    let mut cipher_ctx = CipherCtx::new()?;
    cipher_ctx.encrypt_init(Some(&cipher), Some(&key), iv)?;
    let mut cipher_text = vec![];
    cipher_ctx.cipher_update_vec(data, &mut cipher_text)?;
    cipher_ctx.cipher_final(&mut cipher_text)?;
    let tag_length = cipher_ctx.tag_length();
    let mut tag = vec![0; tag_length];
    cipher_ctx.tag(&mut tag)?;

    let mut cipher_ctx = CipherCtx::new()?;
    cipher_ctx.decrypt_init(Some(&cipher), Some(&key), iv)?;
    let mut plain_text = vec![];
    cipher_ctx.set_tag(&tag)?;
    cipher_ctx.cipher_update_vec(&cipher_text, &mut plain_text)?;
    cipher_ctx.cipher_final(&mut plain_text)?;
    assert_eq!(data, &plain_text[..]);
    Ok(())
}
