use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize};
use serverness::ClientConfig;

pub struct Context {
    client_config: ClientConfig,
}

impl Context {
    pub fn new(client_config: ClientConfig) -> Result<Self> {
        Ok(Self { client_config })
    }

    pub fn client_config(&self) -> &ClientConfig {
        &self.client_config
    }
}
