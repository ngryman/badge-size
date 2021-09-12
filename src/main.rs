use badge_size::App;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
  App::new().listen().await
}
