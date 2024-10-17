use lambda_extension::{service_fn, tracing, Error, LambdaEvent, NextEvent};

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
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

  let func = service_fn(my_extension);
  lambda_extension::run(func).await
}
