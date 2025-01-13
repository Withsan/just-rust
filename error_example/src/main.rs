use anyhow::{Ok, Result};
use std::{fmt::Debug, fs, io};
use thiserror::Error;

fn main() -> Result<()> {
    // read()?;
    bad_padding()?;
    Ok(())
}
struct AppError {
    code: usize,
    message: String,
}
impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AppError{{code:{},message:{}}}",
            &self.code, &self.message
        )
    }
}
impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError {
            code: 1,
            message: value.to_string(),
        }
    }
}
fn read() -> Result<String> {
    Ok(fs::read_to_string("")?)
}
fn bad_padding() -> Result<()> {
    Err(LibError::BadPadding("haha".into()).into())
}
#[derive(Error, Debug)]
enum LibError {
    #[error("invalid padding {0}")]
    BadPadding(String),
    #[error("badmode :{code}")]
    BadMode { code: i32 },
}
