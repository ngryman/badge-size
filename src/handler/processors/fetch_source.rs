use surf::middleware::Redirect;
use tide::{Error, Result};

use crate::Context;

pub async fn fetch_source(mut context: Context) -> Result<Context> {
  let res = surf::client()
    .with(Redirect::default())
    .get(&context.options.source_url)
    .send()
    .await?;

  if res.status().is_success() {
    context.source_reader = Some(Box::pin(res));
    Ok(context)
  } else {
    Err(Error::from_str(
      res.status(),
      res.status().canonical_reason().to_lowercase(),
    ))
  }
}
