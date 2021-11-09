use std::io;
use tide::Server;

use crate::{handle, TracingMiddleware};

pub struct App {
  pub server: Server<()>,
}

impl App {
  pub fn new() -> Self {
    let mut server = tide::new();
    server.with(TracingMiddleware::new());
    server.at("*").get(handle);
    Self { server }
  }

  pub async fn listen(self) -> io::Result<()> {
    let server = self.server.clone();
    server.listen("0.0.0.0:3000").await?;
    Ok(())
  }
}
