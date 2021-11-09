use crate::{
  options::{Color, Style},
  BadgeInfo,
};

use super::kerning::KERNING;

const PADDING: u16 = 5;

pub enum Gradient {
  Glossy,
  Subtle,
  None,
}

pub struct VisualInfo {
  pub color: Color,
  pub gradient: Gradient,
  pub height: u16,
  pub label: String,
  pub label_x: u16,
  pub label_width: u16,
  pub message: String,
  pub message_x: u16,
  pub message_width: u16,
  pub radius: u8,
  pub text_shadow: bool,
  pub text_y: u16,
  pub total_width: u16,
}

impl From<&BadgeInfo> for VisualInfo {
  #[must_use]
  fn from(badge_info: &BadgeInfo) -> Self {
    let label_x = PADDING;
    let label_width = compute_width(&badge_info.label);

    let message_x = label_x + label_width + 2 * PADDING;
    let message_width = compute_width(&badge_info.pretty_size);

    let total_width = (message_x + message_width + PADDING) * 10;

    Self {
      color: badge_info.color.clone(),
      gradient: match badge_info.style {
        Style::Flat => Gradient::Subtle,
        Style::FlatSquare => Gradient::None,
        Style::Plastic => Gradient::Glossy,
      },
      height: match badge_info.style {
        Style::Flat | Style::FlatSquare => 20,
        Style::Plastic => 18,
      },
      label: badge_info.label.clone(),
      label_x,
      label_width,
      message: badge_info.pretty_size.clone(),
      message_x,
      message_width,
      radius: match badge_info.style {
        Style::Flat => 3,
        Style::FlatSquare => 0,
        Style::Plastic => 4,
      },
      text_shadow: match badge_info.style {
        Style::Flat | Style::Plastic => true,
        Style::FlatSquare => false,
      },
      text_y: match badge_info.style {
        Style::Flat | Style::FlatSquare => 14,
        Style::Plastic => 13,
      },
      total_width,
    }
  }
}

fn compute_width(text: &str) -> u16 {
  let width = text
    .chars()
    .fold(0u16, |width, c| width + KERNING[c as usize] as u16);

  let width = width / 10;

  if width % 2 == 0 {
    width + 1
  } else {
    width
  }
}
