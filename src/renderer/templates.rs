pub const CONTAINER_TEMPLATE: &str = r##"
  <svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="20" role="img" aria-label="{title}">
    <title>{title}</title>
    {background}
    <g fill="#fff" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="11">
      {text}
    </g>
  </svg>
"##;

pub const CLIP_PATH_TEMPLATE: &str = r##"
  <clipPath id="r">
    <rect width="{width}" height="{height}" rx="{radius}" fill="#fff"/>
  </clipPath>
  <g clip-path="url(#r)">
    <rect width="{width_label_bg}" height="{height}" fill="#555"/>
    <rect x="{width_label_bg}" width="{width_message_bg}" height="{height}" fill="#{color}"/>
    <rect width="{width}" height="{height}" fill="url(#s)"/>
  </g>
"##;

pub const BACKGROUND_GLOSSY_TEMPLATE: &str = r##"
  <linearGradient id="s" x2="0" y2="100%">
    <stop offset="0" stop-color="#fff" stop-opacity=".7"/>
    <stop offset=".1" stop-color="#aaa" stop-opacity=".1"/>
    <stop offset=".9" stop-color="#000" stop-opacity=".3"/>
    <stop offset="1" stop-color="#000" stop-opacity=".5"/>
  </linearGradient>
  {clip_path}
"##;

pub const BACKGROUND_SUBTLE_TEMPLATE: &str = r##"
  <linearGradient id="s" x2="0" y2="100%">
    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
    <stop offset="1" stop-opacity=".1"/>
  </linearGradient>
  {clip_path}
"##;

pub const BACKGROUND_NONE_TEMPLATE: &str = r##"
  <g shape-rendering="crispEdges">
    <rect width="{width_label_bg}" height="20" fill="#555"/>
    <rect x="{width_label_bg}" width="{width_message_bg}" height="{height}" fill="#{color}"/>
  </g>
"##;

pub const TEXT_WITH_SHADOW: &str = r##"
  <text x="{x}" y="{y_offset}" aria-hidden="true" fill="#010101" fill-opacity=".3" textLength="{text_length}">{text}</text>
  <text x="{x}" y="{y}" fill="#fff" textLength="{text_length}">{text}</text>
"##;

pub const TEXT_WITHOUT_SHADOW: &str = r##"
  <text x="{x}" y="{y}" fill="#fff" textLength="{text_length}">{text}</text>
"##;
