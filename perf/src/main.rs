use goose::prelude::*;

async fn load_example_badge(user: &mut GooseUser) -> GooseTaskResult {
  let _ = user.get("/ngryman/example/main/README.md").await;
  Ok(())
}

async fn load_vant_badge(user: &mut GooseUser) -> GooseTaskResult {
  let _ = user.get("/https:/unpkg.com/vant@3/lib/vant.min.js?label=gzip%20size&compression=gzip&color=&style=flat-square").await;
  Ok(())
}

async fn load_twbs_css_gzip(user: &mut GooseUser) -> GooseTaskResult {
  let _ = user.get("/twbs/bootstrap/main/dist/css/bootstrap.min.css?compression=gzip&label=CSS%20gzip%20size").await;
  Ok(())
}

async fn load_twbs_css_brotli(user: &mut GooseUser) -> GooseTaskResult {
  let _ = user.get("
  /twbs/bootstrap/main/dist/css/bootstrap.min.css?compression=brotli&label=CSS%20Brotli%20size").await;
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
  let _goose_metrics = GooseAttack::initialize()?
    .register_taskset(
      taskset!("example_badge")
        .register_task(task!(load_example_badge).set_weight(10)?),
    )
    .register_taskset(
      taskset!("popular_badges")
        .register_task(task!(load_vant_badge).set_weight(2)?)
        .register_task(task!(load_twbs_css_gzip).set_weight(1)?)
        .register_task(task!(load_twbs_css_brotli).set_weight(1)?)
    )
    // You could also set a default host here, for example:
    .set_default(GooseDefault::Host, "https://img.runbots.io")?
    // .set_default(GooseDefault::Host, "http://img.badgesize.io")?
    // We set a default run time so this test runs to completion.
    .set_default(GooseDefault::RunTime, 1)?
    .execute()
    .await?
    .print();

  Ok(())
}
