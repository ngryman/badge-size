use tide::{
  http::{headers, mime},
  Body, Response, Result, StatusCode,
};

use crate::BadgeInfo;

const CACHE_CONTROL: &str = "public, max-age=300, stale-while-revalidate=86400";

pub async fn send_json(badge_info: BadgeInfo) -> Result<Response> {
  let body = Body::from_json(&badge_info)?;

  let mut res = Response::builder(StatusCode::Ok)
    .body(body)
    .header(headers::CACHE_CONTROL, CACHE_CONTROL)
    .content_type(mime::JSON)
    .build();
  res.insert_ext(badge_info);

  Ok(res)
}

pub async fn send_svg(badge_info: BadgeInfo) -> Result<Response> {
  let svg = badge_info.as_svg();
  let body = tide::Body::from_string(svg);

  let mut res = Response::builder(StatusCode::Ok)
    .body(body)
    .header(headers::CACHE_CONTROL, CACHE_CONTROL)
    .content_type(mime::SVG)
    .build();
  res.insert_ext(badge_info);

  Ok(res)
}
