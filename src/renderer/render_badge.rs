use crate::BadgeInfo;

use super::{templates, Gradient, VisualInfo};

const PADDING: u16 = 5;

pub fn render_badge(badge_info: BadgeInfo) -> String {
  let visual_info = VisualInfo::from(badge_info);
  render_container(&visual_info)
}

fn render_container(visual_info: &VisualInfo) -> String {
  let title = format!("{}: {}", visual_info.label, visual_info.message);
  templates::CONTAINER_TEMPLATE
    .replace("{width}", &(visual_info.total_width / 10).to_string())
    .replace("{title}", &title)
    .replace("{background}", &render_background(&visual_info))
    .replace("{text}", &render_text(&visual_info))
}

fn render_background(visual_info: &VisualInfo) -> String {
  let template = match visual_info.gradient {
    Gradient::Glossy => templates::BACKGROUND_GLOSSY_TEMPLATE,
    Gradient::Subtle => templates::BACKGROUND_SUBTLE_TEMPLATE,
    Gradient::None => templates::BACKGROUND_NONE_TEMPLATE,
  };

  let width_label_bg = visual_info.message_x - PADDING;
  let width_message_bg = visual_info.message_width + 2 * PADDING;

  template
    .replace("{clip_path}", &render_clip_path(&visual_info))
    .replace("{color}", &visual_info.color.to_string())
    .replace("{width_label_bg}", &(width_label_bg).to_string())
    .replace("{width_message_bg}", &(width_message_bg).to_string())
    .replace("{width}", &(visual_info.total_width / 10).to_string())
    .replace("{height}", &visual_info.height.to_string())
}

fn render_clip_path(visual_info: &VisualInfo) -> String {
  templates::CLIP_PATH_TEMPLATE
    .replace("{color}", &visual_info.color.to_string())
    .replace("{radius}", &visual_info.radius.to_string())
}

fn render_text(visual_info: &VisualInfo) -> String {
  let VisualInfo {
    label,
    label_x,
    label_width,
    message,
    message_x,
    message_width,
    text_y,
    text_shadow,
    ..
  } = visual_info;

  let label_nodes =
    render_text_part(&label, *label_x + 1, *text_y, *label_width, *text_shadow);
  let message_nodes = render_text_part(
    &message,
    *message_x - 1,
    *text_y,
    *message_width,
    *text_shadow,
  );

  label_nodes + &message_nodes
}

fn render_text_part(
  text: &str,
  x: u16,
  y: u16,
  text_length: u16,
  shadow: bool,
) -> String {
  let template = if shadow {
    templates::TEXT_WITH_SHADOW
  } else {
    templates::TEXT_WITHOUT_SHADOW
  };

  template
    .replace("{x}", &x.to_string())
    .replace("{y}", &y.to_string())
    .replace("{y_offset}", &(y + 1).to_string())
    .replace("{text}", text)
    .replace("{text_length}", &text_length.to_string())
}
