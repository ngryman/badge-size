use serde::{Deserialize, Serialize};

use crate::{
  options::{Color, Extension, Style},
  renderer::render_badge,
  Context,
};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct BadgeInfo {
  pub color: Color,
  pub extension: Extension,
  pub label: String,
  pub original_size: usize,
  pub pretty_size: String,
  pub size: usize,
  pub style: Style,
}

impl BadgeInfo {
  pub fn to_svg(self) -> String {
    render_badge(self)
  }
}

impl Default for BadgeInfo {
  fn default() -> Self {
    Self {
      color: Default::default(),
      extension: Default::default(),
      label: Default::default(),
      original_size: Default::default(),
      pretty_size: "0 B".to_string(),
      size: Default::default(),
      style: Default::default(),
    }
  }
}

impl From<Context> for BadgeInfo {
  fn from(context: Context) -> Self {
    Self {
      color: context.color,
      extension: context.options.extension,
      label: context.options.label,
      original_size: context.original_size,
      pretty_size: context.pretty_size,
      size: context.size,
      style: context.options.style,
    }
  }
}

impl From<tide::Error> for BadgeInfo {
  fn from(err: tide::Error) -> Self {
    Self {
      color: Color::LightGrey,
      extension: Extension::Svg,
      label: "error".to_string(),
      pretty_size: err.to_string().to_lowercase(),
      ..Default::default()
    }
  }
}
