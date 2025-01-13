use std::any::type_name_of_val;

use ring::error;
use rsa_sign::rsa_sign_test;
use tracing::Level;

mod aead_test;
mod agreement_test;
mod ed25519_sign;
mod hmac_test;
mod rsa_sign;
fn main() -> Result<(), error::Unspecified> {
    let file_appender = tracing_appender::rolling::hourly("~", "fuck.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_writer(non_blocking)
        .init();
    hmac_test::hamc_test()
    // hmac_test::multi_part()?;
    // aead_test::aead_test()?;
    // agreement_test::agreement_test()?;
    // ed25519_sign::ed25519_sign_test()?;
    // rsa_sign_test()
}
