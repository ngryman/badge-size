use anyhow::{anyhow, Result};
use std::collections::HashMap;
use tide::http::{url::ParseError, Url};

use super::{Color, Compression, Extension, Style};

pub const GITHUB_URL: &str = "https://raw.githubusercontent.com";

pub const MAX_SIZE: usize = 1024 * 1024 * 5;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Options {
  pub compression: Compression,
  pub color: Option<Color>,
  pub extension: Extension,
  pub label: String,
  pub max: usize,
  pub softmax: usize,
  pub source_url: Url,
  pub style: Style,
}

impl Options {
  #[must_use]
  pub fn from_url(url: &Url) -> Result<Self> {
    let mut path = url.path();
    let query_map: HashMap<_, _> = url.query_pairs().into_owned().collect();

    // Parse compression type
    let compression: Compression =
      get_or(&query_map, "compression", "").parse()?;

    // Get the extension from the URL pathname
    let extension = match path.rfind('.') {
      Some(pos) => match path[pos + 1..].parse() {
        Ok(ext) => {
          path = &path[..pos];
          ext
        }
        Err(_) => Extension::Svg,
      },
      None => Extension::Svg,
    };

    // Get the label
    let label = if let Some(query_label) = query_map.get("label") {
      query_label.to_owned()
    } else {
      if compression != Compression::None {
        format!("{} size", compression.to_string())
      } else {
        "size".to_string()
      }
    };

    // Get source URL
    let source_url = match Url::parse(&path[1..]) {
      Ok(url) => url,
      Err(ParseError::RelativeUrlWithoutBase) => {
        Url::parse(&format!("{}{}", GITHUB_URL, path))?
      }
      Err(_) => return Err(anyhow!("invalid url")),
    };

    Ok(Self {
      color: query_map.get("color").map(|c| c.parse().unwrap()),
      compression,
      extension,
      label,
      max: get_or(&query_map, "max", &MAX_SIZE.to_string()).parse()?,
      softmax: get_or(&query_map, "softmax", &MAX_SIZE.to_string()).parse()?,
      source_url,
      style: get_or(&query_map, "style", "").parse()?,
    })
  }
}

impl Default for Options {
  fn default() -> Self {
    Self {
      compression: Compression::None,
      color: None,
      extension: Extension::Svg,
      label: "size".to_string(),
      max: MAX_SIZE,
      softmax: MAX_SIZE,
      source_url: Url::parse(GITHUB_URL).unwrap(),
      style: Style::Flat,
    }
  }
}

fn get_or<'a>(
  map: &'a HashMap<String, String>,
  key: &str,
  default: &'a str,
) -> &'a str {
  match map.get(key) {
    Some(value) => value,
    None => default,
  }
}

#[cfg(test)]
mod tests {
  use tide::http::Url;

  use super::{Color, Compression, Extension, Options, Style};

  #[test]
  fn set_defaults() {
    // Arrange
    let url = Url::parse("http://🚀").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options, Default::default());
  }

  #[test]
  fn set_label_to_size_prefixed_with_compression() {
    // Arrange
    let url = Url::parse("http://🚀?compression=gzip").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options.label, "gzip size");
  }

  #[test]
  fn set_specified_label_despite_compression() {
    // Arrange
    let url = Url::parse("http://🚀?label=blob&compression=gzip").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options.label, "blob");
  }

  #[test]
  fn parse_a_valid_compression() {
    // Arrange
    let url = Url::parse("http://🚀?compression=gzip").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options.compression, Compression::Gzip)
  }

  #[test]
  fn return_error_for_an_invalid_compression() {
    // Arrange
    let url = Url::parse("http://🚀?compression=blob").unwrap();
    // Act
    let err = Options::from_url(&url).err().unwrap();
    // Assert
    assert_eq!(
      err.to_string(),
      "gzip, brotli, or no compression are supported, got blob"
    )
  }

  #[test]
  fn parse_a_valid_extension() {
    // Arrange
    let url = Url::parse("http://🚀/foo.png").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options.extension, Extension::Png)
  }

  #[test]
  fn set_default_to_svg_for_an_invalid_extension() {
    // Arrange
    let url = Url::parse("http://🚀/foo.blob").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options.extension, Extension::Svg)
  }

  #[test]
  fn strip_the_badge_extension_from_source_url() {
    // Arrange
    let url = Url::parse("http://🚀/foo.md.svg").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options.source_url.path(), "/foo.md")
  }

  #[test]
  fn keep_not_supported_extension_untouched() {
    // Arrange
    let url = Url::parse("http://🚀/foo.md").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options.source_url.path(), "/foo.md")
  }

  #[test]
  fn parse_a_valid_style() {
    // Arrange
    let url = Url::parse("http://🚀?style=flat-square").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options.style, Style::FlatSquare)
  }

  #[test]
  fn return_error_for_an_invalid_style() {
    // Arrange
    let url = Url::parse("http://🚀?style=blob").unwrap();
    // Act
    let err = Options::from_url(&url).err().unwrap();
    // Assert
    assert_eq!(
      err.to_string(),
      "flat, flat-square, or plastic styles are supported, got blob",
    )
  }

  #[test]
  fn parse_a_valid_color() {
    // Arrange
    let url = Url::parse("http://🚀?color=green").unwrap();
    // Act
    let options = Options::from_url(&url).unwrap();
    // Assert
    assert_eq!(options.color, Some(Color::Green))
  }
}
