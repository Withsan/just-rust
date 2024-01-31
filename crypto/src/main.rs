use ring::error;

mod aead_test;
mod agreement_test;
mod hmac_test;
fn main() -> Result<(), error::Unspecified> {
    hmac_test::hamc_test()?;
    hmac_test::multi_part()?;
    aead_test::aead_test()?;
    agreement_test::agreement_test()
}
