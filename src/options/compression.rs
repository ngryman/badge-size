use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
  None,
  Brotli,
  Gzip,
}

impl FromStr for Compression {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "brotli" => Ok(Self::Brotli),
      "gzip" => Ok(Self::Gzip),
      "" => Ok(Self::None),
      s => Err(anyhow!(
        "gzip, brotli, or no compression are supported, got {}",
        s
      )),
    }
  }
}

impl ToString for Compression {
  fn to_string(&self) -> String {
    match self {
      &Self::Brotli => "brotli".to_string(),
      &Self::Gzip => "gzip".to_string(),
      &Self::None => "".to_string(),
    }
  }
}
