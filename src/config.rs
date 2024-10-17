use serde::Deserialize;
use tokio::fs;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Target {
  pub command: String,
  pub shell: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
  pub targets: Vec<Target>,
}

impl Config {
  pub async fn from_yaml(path: &str) -> anyhow::Result<Self> {
    Ok(serde_yaml::from_str(&fs::read_to_string(path).await?)?)
  }
}
