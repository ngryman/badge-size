use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Style {
  Flat,
  FlatSquare,
  Plastic,
}

impl Default for Style {
  fn default() -> Self {
    Self::Flat
  }
}

impl FromStr for Style {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "flat" | "" => Ok(Self::Flat),
      "flat-square" => Ok(Self::FlatSquare),
      "plastic" => Ok(Self::Plastic),
      s => Err(anyhow!(
        "flat, flat-square, or plastic styles are supported, got {}",
        s
      )),
    }
  }
}

impl ToString for Style {
  fn to_string(&self) -> String {
    match self {
      &Self::Flat => "flat".to_string(),
      &Self::FlatSquare => "flat-square".to_string(),
      &Self::Plastic => "plastic".to_string(),
    }
  }
}
