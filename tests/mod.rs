use badge_size::{options::Extension, App, BadgeInfo};
use surf::{Client, Result, StatusCode};

#[async_std::test]
async fn serve_github_file() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn serve_absolute_url_file() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/https://raw.githubusercontent.com/ngryman/example/main/README.md")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn follow_redirects() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/https://unpkg.com/constate/LICENSE")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn serve_svg_extension() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md.svg")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn serve_json_extension() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md.json")
    .header("user-agent", "ngryman")
    .await?;
  let badge_info: BadgeInfo = res.body_json().await?;

  assert_eq!(res.status(), StatusCode::Ok);
  assert_eq!(
    badge_info,
    BadgeInfo {
      extension: Extension::Json,
      label: "size".to_string(),
      original_size: 116,
      pretty_size: "116 B".to_string(),
      size: 116,
      ..Default::default()
    }
  );

  Ok(())
}

#[async_std::test]
async fn display_custom_label() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md.svg?label=foo")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn display_custom_color() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md.svg?color=green")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn display_gzip_size() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md?compression=gzip")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn display_brotli_size() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md?compression=brotli")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn display_custom_style() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md.svg?style=plastic")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn set_color_to_lightgreen_when_size_is_lte_softmax() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md?softmax=116")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn keep_custom_color_when_size_is_lte_softmax() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md?color=blue&softmax=116")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn set_color_to_yellow_when_size_exceeds_softmax() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md?softmax=100")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn set_color_to_red_when_size_exceeds_max() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md?max=100")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn set_color_to_yellow_when_size_exceeds_softmax_with_max_specified(
) -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md?softmax=100&max=200")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn reject_denied_user_agent() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let res = client
    .get("http://🚀/ngryman/example/main/README.md")
    .header(
      "user-agent",
      "Baiduspider ( http://www.baidu.com/search/spider.htm)",
    )
    .await?;

  assert_eq!(res.status(), StatusCode::NotFound);

  Ok(())
}

#[async_std::test]
async fn reject_denied_url() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let res = client
    .get("http://🚀/vxe-table/vxe-table/master/README.md.svg")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::NotFound);

  Ok(())
}

#[async_std::test]
async fn reject_empty_path() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let res = client
    .get("http://🚀")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::NotFound);

  Ok(())
}

#[async_std::test]
async fn display_error_for_invalid_path() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀//non-sense/query")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}

#[async_std::test]
async fn display_error_for_invalid_compression() -> Result<()> {
  let app = App::new();
  let client = Client::with_http_client(app.server);

  let mut res = client
    .get("http://🚀/ngryman/example/main/README.md?compression=lzma")
    .header("user-agent", "ngryman")
    .await?;

  assert_eq!(res.status(), StatusCode::Ok);
  insta::assert_debug_snapshot!(res.body_string().await?);

  Ok(())
}
