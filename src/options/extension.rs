use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Extension {
  Json,
  Png,
  Svg,
}

impl Default for Extension {
  fn default() -> Self {
    Self::Svg
  }
}

impl FromStr for Extension {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "json" => Ok(Self::Json),
      "png" => Ok(Self::Png),
      "svg" | "" => Ok(Self::Svg),
      s => Err(anyhow!(
        "json, png, or svg extensions are supported, got {}",
        s
      )),
    }
  }
}

impl ToString for Extension {
  fn to_string(&self) -> String {
    match self {
      &Self::Json => "json".to_string(),
      &Self::Png => "png".to_string(),
      &Self::Svg => "svg".to_string(),
    }
  }
}
