use ntex::web;

mod error;
mod models;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      .configure(services::ping::ntex_config)
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 3000))?
  .run()
  .await?;
  Ok(())
}
