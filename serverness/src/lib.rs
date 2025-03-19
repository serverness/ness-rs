use thiserror::Error;

mod client;
mod generated_sdk;
#[cfg(feature = "nu")]
mod nu_ext;

pub use client::*;
pub use generated_sdk::*;

#[derive(Error, Debug)]
pub enum ServernessError {
    #[error("no credentials provided")]
    NoCredentials,
}
