mod config;

use config::Config;
use futures::future::join_all;
use lambda_extension::{
  service_fn,
  tracing::{debug, trace},
  Error, LambdaEvent, NextEvent,
};
use reqwest::Method;
use std::{env, time::Duration};
use tokio::{process::Command, time::sleep};
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
    .unwrap_or_else(|_| "/var/task/aws-lambda-sidecar.yaml".into());
  debug!("AWS_LAMBDA_SIDECAR_MANAGER_CONFIG={}", path);

  let config = Config::from_yaml(&path).await.unwrap();
  debug!("config={:?}", config);

  // spawn targets
  for target in config.targets {
    debug!("spawning target: {:?}", target);
    Command::new(target.shell.as_deref().unwrap_or("sh"))
      .arg("-c")
      .arg(&target.command)
      .spawn()
      .unwrap_or_else(|_| panic!("failed to start target: {:?}", target));
  }

  // wait for readiness
  if let Some(readiness) = config.readiness {
    join_all(readiness.into_iter().map(|r| async move {
      let interval = r.interval.unwrap_or(1);

      if let Some(http) = r.http {
        let client = reqwest::Client::new();
        let status = http.status.unwrap_or(200);

        let method = match http
          .method
          .as_ref()
          .map(|s| s.to_uppercase())
          .as_deref()
          .unwrap_or("GET")
        {
          "GET" => Method::GET,
          "POST" => Method::POST,
          "PUT" => Method::PUT,
          "DELETE" => Method::DELETE,
          "PATCH" => Method::PATCH,
          "HEAD" => Method::HEAD,
          "OPTIONS" => Method::OPTIONS,
          _ => panic!("unsupported HTTP method: {:?}", http.method),
        };

        loop {
          match client.request(method.clone(), &http.url).send().await {
            Ok(body) => {
              if body.status().as_u16() == status {
                debug!("target is ready, probe={:?}", http);
                break;
              } else {
                trace!(
                  "http status mismatch: {}, probe={:?}",
                  body.status().as_u16(),
                  http,
                );
              }
            }
            Err(e) => {
              trace!("http request failed, probe={:?}, error={:?}", http, e);
            }
          }
          trace!("sleeping for {}ms, probe={:?}", interval, http);
          sleep(Duration::from_millis(interval)).await;
        }
      }
      if let Some(exec) = r.exec {
        let shell = exec.shell.as_deref().unwrap_or("sh");
        let status = exec.status.unwrap_or(0);

        loop {
          match Command::new(shell).arg("-c").arg(&exec.command).spawn() {
            Ok(mut child) => match child.wait().await {
              Ok(exit) => {
                if exit.code().unwrap() == status {
                  debug!("target is ready, probe={:?}", exec);
                  break;
                } else {
                  trace!(
                    "exec status mismatch: {}, probe={:?}",
                    exit.code().unwrap(),
                    exec,
                  );
                }
              }
              Err(e) => {
                trace!("exec command failed, probe={:?}, err={:?}", exec, e);
              }
            },
            Err(e) => {
              trace!("exec command failed, probe={:?}, error={:?}", exec, e);
            }
          }

          trace!("sleeping for {}ms, probe={:?}", interval, exec);
          sleep(Duration::from_millis(interval)).await;
        }
      }
    }))
    .await;
  }

  let func = service_fn(my_extension);
  lambda_extension::run(func).await
}
