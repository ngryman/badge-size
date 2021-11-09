use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Color {
  Brightgreen,
  Green,
  YellowGreen,
  Yellow,
  Orange,
  Red,
  LightGrey,
  Blue,
  Hex(String),
}

impl Default for Color {
  fn default() -> Self {
    Self::Brightgreen
  }
}

impl FromStr for Color {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "brightgreen" | "" => Ok(Self::Brightgreen),
      "green" => Ok(Self::Green),
      "yellowgreen" => Ok(Self::YellowGreen),
      "yellow" => Ok(Self::Yellow),
      "orange" => Ok(Self::Orange),
      "red" => Ok(Self::Red),
      "lightgrey" => Ok(Self::LightGrey),
      "blue" => Ok(Self::Blue),
      hex => Ok(Self::Hex(hex.to_string())),
    }
  }
}

impl ToString for Color {
  fn to_string(&self) -> String {
    match self {
      &Self::Brightgreen => "44cc11".to_string(),
      &Self::Green => "97ca00".to_string(),
      &Self::YellowGreen => "a4a61d".to_string(),
      &Self::Yellow => "dfb317".to_string(),
      &Self::Orange => "fe7d37".to_string(),
      &Self::Red => "e05d44".to_string(),
      &Self::LightGrey => "9f9f9f".to_string(),
      &Self::Blue => "007ec6".to_string(),
      &Self::Hex(ref hex) => hex.to_owned(),
    }
  }
}
