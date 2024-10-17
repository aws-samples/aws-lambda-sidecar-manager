use lambda_extension::{service_fn, Error, LambdaEvent, NextEvent};
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

  let func = service_fn(my_extension);
  lambda_extension::run(func).await
}
