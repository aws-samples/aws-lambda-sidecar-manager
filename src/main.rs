mod config;

use config::Config;
use lambda_extension::{service_fn, tracing::debug, Error, LambdaEvent, NextEvent};
use std::env;
use tokio::process::Command;
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

async fn my_extension(event: LambdaEvent) -> Result<(), Error> {
  match event.next {
    NextEvent::Shutdown(_e) => {
      // do something with the shutdown event
    }
    NextEvent::Invoke(_e) => {
      // do something with the invoke event
    }
  }
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy(),
    )
    .without_time()
    .init();

  let path = env::var("AWS_LAMBDA_SIDECAR_MANAGER_CONFIG")
    .unwrap_or_else(|_| "aws-lambda-sidecar.yaml".into());
  debug!("AWS_LAMBDA_SIDECAR_MANAGER_CONFIG={}", path);

  let config = Config::from_yaml(&path).await.unwrap();
  debug!("config={:?}", config);

  for target in config.targets {
    debug!("starting target: {:?}", target);
    Command::new(target.shell.as_deref().unwrap_or("sh"))
      .arg("-c")
      .arg(&target.command)
      .spawn()
      .unwrap_or_else(|_| panic!("failed to start target: {:?}", target));
  }

  let func = service_fn(my_extension);
  lambda_extension::run(func).await
}
