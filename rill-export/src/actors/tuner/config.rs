use anyhow::Error;
use async_trait::async_trait;
use meio::prelude::{LiteTask, TaskError};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Serialize, Deserialize)]
pub struct Export {
    // TODO: Deserialize paths here directly using `FromStr`
    pub paths: Option<Vec<String>>,
    pub prometheus: Option<()>,
    pub graphite: Option<()>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub export: Export,
}

pub struct ReadConfigFile;

#[async_trait]
impl LiteTask for ReadConfigFile {
    type Output = Config;

    async fn interruptable_routine(mut self) -> Result<Self::Output, Error> {
        let mut file = File::open(crate::env::config()).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;
        let config: Config = toml::from_slice(&contents)?;
        Ok(config)
    }
}
