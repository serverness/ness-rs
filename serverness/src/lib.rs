use thiserror::Error;

mod client;
mod generated_sdk;

pub use client::*;
pub use generated_sdk::*;

#[derive(Error, Debug)]
pub enum ServernessError {
    #[error("no credentials provided")]
    NoCredentials,
}
