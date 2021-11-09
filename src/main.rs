use anyhow::Result;
use badge_size::App;
use flexi_logger::{FileSpec, Logger};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct Settings {
  #[structopt(long = "port", env = "PORT", default_value = "3000")]
  pub port: u32,
  #[structopt(long = "logpath", env = "LOGPATH")]
  pub logpath: Option<String>,
}

async fn init_logger(logpath: Option<String>) -> Result<()> {
  let mut logger = Logger::try_with_env_or_str("info,surf=error,tide=error")?;

  logger = if let Some(logpath) = logpath {
    logger.log_to_file(
      FileSpec::default()
        .directory(logpath)
        .basename("badgesize")
        .use_timestamp(false),
    )
  } else {
    logger.log_to_stdout()
  };

  logger.start()?;

  Ok(())
}

#[async_std::main]
#[paw::main]
async fn main(settings: Settings) -> Result<()> {
  init_logger(settings.logpath).await?;
  App::new().listen(settings.port).await?;
  Ok(())
}
