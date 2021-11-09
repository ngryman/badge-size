use tide::{Request, Response, Result, StatusCode};

use crate::{options::Extension, BadgeInfo, Context, Options};

use super::{acl, processors, senders};

pub async fn handle(req: Request<()>) -> Result<Response> {
  // Early bail out if the request is denied
  if !acl::is_allowed(&req.as_ref()) {
    return Ok(Response::new(StatusCode::NotFound));
  }

  let badge_info = match pipeline(&req).await {
    Ok(badge_info) => badge_info,
    Err(err) => BadgeInfo::from(err),
  };

  match badge_info.extension {
    Extension::Json => senders::send_json(badge_info).await,
    Extension::Png | Extension::Svg => senders::send_svg(badge_info).await,
  }
}

async fn pipeline(req: &Request<()>) -> Result<BadgeInfo> {
  let url = req.url();

  let options = Options::from_url(url)?;
  let context = Context::new(options);

  let context = processors::fetch_source(context).await?;
  let context = processors::count_size(context).await?;
  let context = processors::apply_constraints(context).await?;

  let badge_info = BadgeInfo::from(context);

  Ok(badge_info)
}
