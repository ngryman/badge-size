use chrono::Utc;
use serde::Serialize;
use std::{fmt::Display, time::Instant};
use tide::{log, Middleware, Next, Request, Result, StatusCode};

use crate::BadgeInfo;

#[derive(Serialize)]
pub struct Trace<'a> {
  date: String,
  path: String,
  status: StatusCode,
  duration: u128,
  remote: Option<String>,
  badge: Option<&'a BadgeInfo>,
}

pub struct TracingMiddleware {}

// == impl Trace ==

impl<'a> Trace<'a> {}

impl<'a> Display for Trace<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", serde_json::to_string(self).unwrap())
  }
}

// == impl TracingMiddleware ==

impl TracingMiddleware {
  pub fn new() -> Self {
    Self {}
  }
}

impl Default for TracingMiddleware {
  fn default() -> Self {
    Self::new()
  }
}

#[tide::utils::async_trait]
impl<State> Middleware<State> for TracingMiddleware
where
  State: Clone + Send + Sync + 'static,
{
  async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
    let start = Instant::now();
    let path = req.url().path().to_string();
    let remote = req.remote().map(ToString::to_string);
    let res = next.run(req).await;

    let status = res.status();
    let badge = res.ext::<BadgeInfo>();
    let trace = Trace {
      date: Utc::now().to_string(),
      path,
      status,
      duration: start.elapsed().as_millis(),
      remote,
      badge,
    };

    log::info!("{}", trace);

    Ok(res)
  }
}
