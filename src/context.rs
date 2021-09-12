use async_std::io::Read;
use std::pin::Pin;

use crate::{options::Color, Options};

pub struct Context {
  pub color: Color,
  pub options: Options,
  pub original_size: usize,
  pub pretty_size: String,
  pub size: usize,
  pub source_reader: Option<Pin<Box<dyn Read + Send>>>,
}

impl Context {
  pub fn new(options: Options) -> Self {
    Self {
      color: options.color.clone().unwrap_or_default(),
      options,
      ..Default::default()
    }
  }
}

impl Default for Context {
  fn default() -> Self {
    Self {
      color: Default::default(),
      options: Default::default(),
      original_size: Default::default(),
      pretty_size: "0 B".to_string(),
      size: Default::default(),
      source_reader: None,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::Options;

  use super::Context;

  #[test]
  fn set_defaults() {
    // Arrange
    let options = Options::default();

    // Act
    let context = Context::new(options);
    let default = Context::default();

    // Assert
    assert_eq!(context.options, default.options);
    assert_eq!(context.original_size, default.original_size);
    assert_eq!(context.pretty_size, default.pretty_size);
    assert_eq!(context.size, default.size);
  }
}
