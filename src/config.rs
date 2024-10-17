use serde::Deserialize;
use tokio::fs;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Target {
  pub command: String,
  pub shell: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct HttpReadiness {
  pub url: String,
  pub method: Option<String>,
  pub status: Option<u16>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ExecReadiness {
  pub command: String,
  pub shell: Option<String>,
  pub status: Option<i32>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Readiness {
  pub http: Option<HttpReadiness>,
  pub exec: Option<ExecReadiness>,
  pub interval: Option<u64>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
  pub targets: Vec<Target>,
  pub readiness: Option<Vec<Readiness>>,
}

impl Config {
  pub async fn from_yaml(path: &str) -> anyhow::Result<Self> {
    Ok(serde_yaml::from_str(&fs::read_to_string(path).await?)?)
  }
}
